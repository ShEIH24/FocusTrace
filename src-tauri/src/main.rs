#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used)]

mod analytics;
mod browser;
mod commands;
mod config;
mod db;
mod error;
mod events;
mod productivity;
mod state;
mod tracker;
mod workers;

use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use tauri::{Emitter, Manager};
use tauri_plugin_updater::UpdaterExt;

use config::app_config::AppConfig;
use db::pool::create_pool;
use events::types::AppEvent;
use productivity::models::PomodoroTickPayload;
use state::{ActivityCache, AppState};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env().add_directive(
                "focus_trace=debug"
                    .parse()
                    .expect("valid tracing directive"),
            ),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--autostarted"]),
        ))
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let config = AppConfig::load(&app_data_dir);
            let db_path = app_data_dir.join("focustrace.db");
            let handle = app.handle().clone();

            let pool = tauri::async_runtime::block_on(create_pool(&db_path))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let (event_tx, event_rx) = mpsc::channel::<AppEvent>(512);

            // Load productivity config and restore focus mode state.
            let prod_cfg = tauri::async_runtime::block_on(productivity::load_config(&pool));
            let focus_mode_enabled = prod_cfg.focus_mode.enabled;
            let focus_alert_threshold = prod_cfg.focus_mode.alert_threshold_secs;

            // Shared snapshot updated by the pomodoro worker every second.
            let pomodoro_state = Arc::new(RwLock::new(PomodoroTickPayload::default()));

            // Spawn pomodoro worker.
            let pomodoro_tx = productivity::spawn_pomodoro(
                handle.clone(),
                pool.clone(),
                prod_cfg.pomodoro,
                pomodoro_state.clone(),
            );

            // Init browser tracking (WS server spawned inside).
            let browser =
                tauri::async_runtime::block_on(browser::init(handle.clone(), pool.clone()));

            // Capture the foreground window right now so the frontend sees an
            // active window immediately on startup (before the tracker polls).
            let initial_window = tracker::win32::get_foreground_window_info();

            let app_state = AppState {
                pool,
                config: Arc::new(RwLock::new(config)),
                event_tx,
                cache: Arc::new(RwLock::new(ActivityCache {
                    focus_mode_enabled,
                    focus_alert_threshold_secs: focus_alert_threshold,
                    current_window: initial_window,
                    ..ActivityCache::default()
                })),
                pomodoro_tx,
                browser,
                pomodoro_state,
            };

            app.manage(app_state.clone());

            tauri::async_runtime::spawn(async move {
                workers::coordinator::start(app_state, handle, event_rx).await;
            });

            workers::tray::setup_tray(app.handle())?;

            // Проверка обновлений при запуске — через 8 с, чтобы не замедлять старт.
            let update_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(8)).await;
                let Ok(updater) = update_handle.updater() else {
                    return;
                };
                if let Ok(Some(update)) = updater.check().await {
                    let _ = update_handle.emit("update-available", update.version.clone());
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Activity
            commands::activity::get_current_activity,
            commands::activity::get_activity_log,
            commands::activity::get_app_usage,
            commands::activity::get_hourly_stats,
            commands::activity::get_live_cache,
            // Analytics
            commands::analytics::get_focus_score,
            commands::analytics::get_weekly_summary,
            commands::analytics::get_day_report,
            commands::analytics::get_week_report,
            // Productivity
            commands::productivity::pomodoro_start,
            commands::productivity::pomodoro_pause,
            commands::productivity::pomodoro_resume,
            commands::productivity::pomodoro_stop,
            commands::productivity::pomodoro_skip,
            commands::productivity::get_pomodoro_state,
            commands::productivity::get_productivity_config,
            commands::productivity::update_productivity_config,
            commands::productivity::get_daily_goals,
            commands::productivity::get_streak,
            commands::productivity::toggle_focus_mode,
            commands::productivity::get_focus_mode_state,
            // Settings
            commands::settings::get_config,
            commands::settings::update_config,
            // Browser tracking
            commands::browser::get_browser_ws_token,
            commands::browser::get_current_browser_tab,
            commands::browser::get_browser_history,
            commands::browser::get_browser_domain_stats,
            commands::browser::get_browser_today_summary,
            commands::browser::add_browser_category_rule,
            commands::browser::remove_browser_category_rule,
            commands::browser::get_browser_category_rules,
            commands::browser::categorize_url,
            // Window / tray
            commands::tray::show_main_window,
            commands::tray::hide_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error running FocusTrace");
}
