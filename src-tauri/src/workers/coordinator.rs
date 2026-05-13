use std::sync::Arc;

use chrono::Utc;
use sqlx::SqlitePool;
use tauri::AppHandle;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};

use crate::analytics::classifier::RulesClassifier;
use crate::db::repository::{AppRepository, MetricsRepository, NewSession, SessionRepository};
use crate::events::{
    emit_activity_updated, emit_distraction_alert, emit_idle_changed,
    emit_metrics_updated, emit_session_ended,
};
use crate::events::types::{
    AppEvent, DistractionAlertPayload, MetricsUpdatedPayload, SessionEndedPayload, SessionInfo,
};
use crate::state::{ActivityCache, AppState};
use crate::tracker::idle_detector::IdleDetector;
use crate::tracker::session_manager::SessionManager;

pub async fn start(
    state: AppState,
    app: AppHandle,
    mut event_rx: mpsc::Receiver<AppEvent>,
) {
    let poll_ms = state.config.read().await.poll_interval_ms;

    tauri::async_runtime::spawn(crate::tracker::window_tracker::run(
        state.event_tx.clone(),
        poll_ms,
    ));
    tauri::async_runtime::spawn(
        IdleDetector::new(state.config.clone(), state.event_tx.clone()).run(),
    );

    let classifier = {
        let cfg = state.config.read().await;
        RulesClassifier::with_config(&cfg.productive_apps, &cfg.distraction_apps)
    };
    let mut session_mgr = SessionManager::new(classifier);

    info!("Coordinator started");

    while let Some(event) = event_rx.recv().await {
        match event {
            AppEvent::Shutdown => {
                info!("Shutdown — finalizing session");
                if let Some(mut session) = session_mgr.finalize() {
                    let cfg = state.config.read().await;
                    let classifier = RulesClassifier::with_config(
                        &cfg.productive_apps,
                        &cfg.distraction_apps,
                    );
                    session.category = classifier
                        .classify(&session.exe, &session.exe_path)
                        .to_string();
                    drop(cfg);
                    let pool = state.pool.clone();
                    let cache = state.cache.clone();
                    let handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        persist_and_notify(pool, cache, session, handle).await;
                    });
                }
                break;
            }

            AppEvent::WindowChanged(info) => {
                let is_distraction = {
                    let mut cache = state.cache.write().await;
                    cache.current_window = Some(info.clone());
                    cache.is_idle = false;

                    // Focus mode: check if the new window is a distraction.
                    if cache.focus_mode_enabled {
                        let lower = info.exe.to_lowercase();
                        let config = state.config.read().await;
                        let is_dist = config
                            .distraction_apps
                            .iter()
                            .any(|d| lower.contains(d.as_str()));
                        if is_dist {
                            cache.distraction_count += 1;
                            cache.distraction_active_secs = 0;
                        } else {
                            cache.distraction_active_secs = 0;
                        }
                        is_dist
                    } else {
                        false
                    }
                };

                if is_distraction {
                    let (count, threshold) = {
                        let cache = state.cache.read().await;
                        (cache.distraction_count, cache.focus_alert_threshold_secs)
                    };
                    // Alert immediately on first switch, then every `threshold` seconds.
                    if count == 1 || count % threshold.max(1) == 0 {
                        emit_distraction_alert(
                            &app,
                            &DistractionAlertPayload {
                                exe: info.exe.clone(),
                                distraction_count: count,
                                active_secs: 0,
                            },
                        );
                    }
                }

                if let Some(mut session) = session_mgr.on_window_changed(info.clone()) {
                    // Re-classify with current config so user changes take effect immediately.
                    let cfg = state.config.read().await;
                    let classifier = RulesClassifier::with_config(
                        &cfg.productive_apps,
                        &cfg.distraction_apps,
                    );
                    session.category = classifier
                        .classify(&session.exe, &session.exe_path)
                        .to_string();
                    drop(cfg);
                    let _ = state.event_tx.send(AppEvent::SessionEnded(session)).await;
                }

                emit_activity_updated(&app, &info);
            }

            AppEvent::IdleStarted { at } => {
                {
                    let mut cache = state.cache.write().await;
                    cache.is_idle = true;
                    cache.idle_since = Some(at);
                }
                emit_idle_changed(
                    &app,
                    serde_json::json!({ "isIdle": true, "at": at.to_rfc3339() }),
                );
            }

            AppEvent::IdleEnded { duration_secs } => {
                {
                    let mut cache = state.cache.write().await;
                    cache.is_idle = false;
                    cache.idle_since = None;
                }
                emit_idle_changed(
                    &app,
                    serde_json::json!({ "isIdle": false, "durationSecs": duration_secs }),
                );
            }

            AppEvent::SessionEnded(session) => {
                emit_session_ended(&app, &SessionEndedPayload::from(&session));
                let pool = state.pool.clone();
                let cache = state.cache.clone();
                let handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    persist_and_notify(pool, cache, session, handle).await;
                });
            }
        }
    }
}

async fn persist_and_notify(
    pool: SqlitePool,
    cache: Arc<RwLock<ActivityCache>>,
    session: SessionInfo,
    app: AppHandle,
) {
    let category_id = category_name_to_id(&session.category);

    let app_repo = AppRepository::new(&pool);
    let app_id = match app_repo
        .upsert(&session.exe, &session.exe_path, category_id, session.duration_ms)
        .await
    {
        Ok(id) => id,
        Err(e) => { error!(exe = %session.exe, "upsert app: {e}"); return; }
    };

    let sess_repo = SessionRepository::new(&pool);
    let new_sess = NewSession {
        app_id,
        title: session.title.clone(),
        started_at: session.started_at,
        duration_ms: session.duration_ms,
    };
    if let Err(e) = sess_repo.insert(&new_sess).await {
        error!(exe = %session.exe, "insert session: {e}"); return;
    }

    let date = session.started_at.date_naive();
    let metrics = match sess_repo.compute_metrics(date).await {
        Ok(m) => m,
        Err(e) => { error!(date = %date, "compute_metrics: {e}"); return; }
    };

    let metrics_repo = MetricsRepository::new(&pool);
    if let Err(e) = metrics_repo.upsert(date, &metrics).await {
        error!(date = %date, "upsert metrics: {e}");
    }

    let today = Utc::now().date_naive();
    if date == today {
        let mut c = cache.write().await;
        c.today_score = metrics.focus_score as f32;
        c.today_total_ms = metrics.total_ms;
        c.today_productive_ms = metrics.productive_ms;
    }

    emit_metrics_updated(&app, &MetricsUpdatedPayload {
        date: date.to_string(),
        focus_score: metrics.focus_score,
        total_ms: metrics.total_ms,
        productive_ms: metrics.productive_ms,
        distraction_ms: metrics.distraction_ms,
        neutral_ms: metrics.neutral_ms,
        session_count: metrics.session_count,
    });

    update_tray_tooltip(&app, metrics.focus_score as u32, metrics.total_ms);
}

fn update_tray_tooltip(app: &AppHandle, score: u32, total_ms: i64) {
    let h = total_ms / 3_600_000;
    let m = (total_ms % 3_600_000) / 60_000;
    let tip = if h > 0 {
        format!("FocusTrace  •  Focus {score}  |  {h}h {m}m today")
    } else {
        format!("FocusTrace  •  Focus {score}  |  {m}m today")
    };
    if let Some(tray) = app.tray_by_id("main-tray") {
        let _ = tray.set_tooltip(Some(tip));
    }
}

fn category_name_to_id(name: &str) -> i64 {
    match name {
        "Productive" => 1,
        "Distraction" => 3,
        _ => 2,
    }
}
