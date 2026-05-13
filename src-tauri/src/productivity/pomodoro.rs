use std::sync::Arc;

use chrono::Utc;
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{interval, Duration};
use tracing::{debug, info};

use crate::productivity::models::{
    PomodoroConfig, PomodoroPhase, PomodoroPhaseChangedPayload, PomodoroTickPayload,
};

// ---------------------------------------------------------------------------
// Commands sent to the worker from Tauri commands
// ---------------------------------------------------------------------------

pub enum PomodoroCommand {
    Start,
    Pause,
    Resume,
    Stop,
    Skip,
    UpdateConfig(PomodoroConfig),
}

// ---------------------------------------------------------------------------
// Internal worker state
// ---------------------------------------------------------------------------

struct WorkerState {
    phase: PomodoroPhase,
    is_paused: bool,
    remaining_secs: u64,
    total_secs: u64,
    session_count: u32,
    config: PomodoroConfig,
    /// DB session row id for the in-progress pomodoro (None when idle).
    current_db_id: Option<i64>,
}

impl WorkerState {
    fn new(config: PomodoroConfig) -> Self {
        Self {
            phase: PomodoroPhase::Idle,
            is_paused: false,
            remaining_secs: 0,
            total_secs: 0,
            session_count: 0,
            config,
            current_db_id: None,
        }
    }

    fn tick_payload(&self) -> PomodoroTickPayload {
        PomodoroTickPayload {
            phase: self.phase.label().into(),
            remaining_secs: self.remaining_secs,
            total_secs: self.total_secs,
            session_count: self.session_count,
            is_paused: self.is_paused,
        }
    }

    fn phase_changed_payload(&self) -> PomodoroPhaseChangedPayload {
        PomodoroPhaseChangedPayload {
            phase: self.phase.label().into(),
            session_count: self.session_count,
            total_secs: self.total_secs,
        }
    }

    fn transition_to(&mut self, phase: PomodoroPhase) {
        let secs = match &phase {
            PomodoroPhase::Working      => self.config.work_mins as u64 * 60,
            PomodoroPhase::ShortBreak   => self.config.short_break_mins as u64 * 60,
            PomodoroPhase::LongBreak    => self.config.long_break_mins as u64 * 60,
            PomodoroPhase::Idle         => 0,
        };
        self.phase = phase;
        self.remaining_secs = secs;
        self.total_secs = secs;
        self.is_paused = false;
    }

    fn is_running(&self) -> bool {
        self.phase != PomodoroPhase::Idle && !self.is_paused
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Spawns the pomodoro worker as a tokio task.
/// Returns the command sender used by Tauri commands to control the timer.
pub fn spawn(
    app: AppHandle,
    pool: SqlitePool,
    config: PomodoroConfig,
    snapshot: Arc<RwLock<PomodoroTickPayload>>,
) -> mpsc::Sender<PomodoroCommand> {
    let (tx, rx) = mpsc::channel::<PomodoroCommand>(32);
    tauri::async_runtime::spawn(run(app, pool, config, rx, snapshot));
    tx
}

// ---------------------------------------------------------------------------
// Worker loop
// ---------------------------------------------------------------------------

async fn run(
    app: AppHandle,
    pool: SqlitePool,
    initial_config: PomodoroConfig,
    mut cmd_rx: mpsc::Receiver<PomodoroCommand>,
    snapshot: Arc<RwLock<PomodoroTickPayload>>,
) {
    let mut state = WorkerState::new(initial_config);
    let mut ticker = interval(Duration::from_secs(1));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    // Pre-load today's completed work session count from DB.
    state.session_count = count_today_sessions(&pool).await;

    info!("Pomodoro worker started");

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if !state.is_running() { continue; }
                tick(&mut state, &app, &pool, &snapshot).await;
            }
            Some(cmd) = cmd_rx.recv() => {
                match handle_command(cmd, &mut state, &app, &pool).await {
                    ControlFlow::Continue => {
                        *snapshot.write().await = state.tick_payload();
                    }
                    ControlFlow::Break => break,
                }
            }
            else => break,
        }
    }
    info!("Pomodoro worker stopped");
}

// ---------------------------------------------------------------------------
// Tick handler
// ---------------------------------------------------------------------------

async fn tick(
    state: &mut WorkerState,
    app: &AppHandle,
    pool: &SqlitePool,
    snapshot: &Arc<RwLock<PomodoroTickPayload>>,
) {
    // Decrement and emit tick.
    if state.remaining_secs > 0 {
        state.remaining_secs -= 1;
    }
    let payload = state.tick_payload();
    emit_tick(app, &payload);
    *snapshot.write().await = payload;

    if state.remaining_secs > 0 {
        return;
    }

    // Phase complete — determine next phase.
    let completed_phase = state.phase.clone();

    match completed_phase {
        PomodoroPhase::Working => {
            state.session_count += 1;
            record_session(pool, "work", state.config.work_mins as u64 * 60_000, true, state.current_db_id.take()).await;

            let use_long = state.session_count % state.config.sessions_until_long == 0;
            let next = if use_long { PomodoroPhase::LongBreak } else { PomodoroPhase::ShortBreak };
            let label = if use_long { "Long break time! You earned it." } else { "Short break time!" };

            notify(app, "Pomodoro complete", label);
            state.transition_to(next.clone());
            emit_phase_changed(app, &state.phase_changed_payload());

            if !state.config.auto_start_breaks {
                state.is_paused = true;
            } else {
                state.current_db_id = open_db_session(pool, next.label()).await;
            }
        }

        PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
            record_session(
                pool,
                state.phase.label(),
                (state.total_secs) * 1000,
                true,
                state.current_db_id.take(),
            ).await;

            notify(app, "Break over", "Time to focus!");
            state.transition_to(PomodoroPhase::Working);
            emit_phase_changed(app, &state.phase_changed_payload());

            if !state.config.auto_start_work {
                state.is_paused = true;
            } else {
                state.current_db_id = open_db_session(pool, "work").await;
            }
        }

        PomodoroPhase::Idle => {}
    }

    *snapshot.write().await = state.tick_payload();
}

// ---------------------------------------------------------------------------
// Command handler
// ---------------------------------------------------------------------------

#[allow(dead_code)]
enum ControlFlow { Continue, Break }

async fn handle_command(
    cmd: PomodoroCommand,
    state: &mut WorkerState,
    app: &AppHandle,
    pool: &SqlitePool,
) -> ControlFlow {
    match cmd {
        PomodoroCommand::Start => {
            if state.phase != PomodoroPhase::Idle { return ControlFlow::Continue; }
            state.transition_to(PomodoroPhase::Working);
            state.current_db_id = open_db_session(pool, "work").await;
            emit_phase_changed(app, &state.phase_changed_payload());
            info!("Pomodoro started — {}s work", state.total_secs);
        }

        PomodoroCommand::Pause => {
            if state.is_running() {
                state.is_paused = true;
                emit_tick(app, &state.tick_payload());
                debug!("Pomodoro paused at {}s remaining", state.remaining_secs);
            }
        }

        PomodoroCommand::Resume => {
            if state.phase != PomodoroPhase::Idle && state.is_paused {
                state.is_paused = false;
                emit_tick(app, &state.tick_payload());
                debug!("Pomodoro resumed");
            }
        }

        PomodoroCommand::Stop => {
            if state.phase != PomodoroPhase::Idle {
                let elapsed = (state.total_secs - state.remaining_secs) * 1000;
                record_session(pool, state.phase.label(), elapsed as u64, false, state.current_db_id.take()).await;
                state.transition_to(PomodoroPhase::Idle);
                emit_phase_changed(app, &state.phase_changed_payload());
                info!("Pomodoro stopped");
            }
        }

        PomodoroCommand::Skip => {
            if state.phase == PomodoroPhase::Idle { return ControlFlow::Continue; }
            let elapsed = (state.total_secs - state.remaining_secs) * 1000;
            record_session(pool, state.phase.label(), elapsed as u64, false, state.current_db_id.take()).await;

            let next = match state.phase {
                PomodoroPhase::Working => {
                    let use_long = (state.session_count + 1) % state.config.sessions_until_long == 0;
                    if use_long { PomodoroPhase::LongBreak } else { PomodoroPhase::ShortBreak }
                }
                _ => PomodoroPhase::Working,
            };
            state.transition_to(next.clone());
            state.is_paused = true;
            state.current_db_id = open_db_session(pool, next.label()).await;
            emit_phase_changed(app, &state.phase_changed_payload());
        }

        PomodoroCommand::UpdateConfig(cfg) => {
            state.config = cfg;
            debug!("Pomodoro config updated");
        }
    }
    ControlFlow::Continue
}

// ---------------------------------------------------------------------------
// Emit helpers — use AppHandle::emit() for global broadcast so that
// the JS listen() from @tauri-apps/api/event receives the events.
// ---------------------------------------------------------------------------

fn emit_tick(app: &AppHandle, payload: &PomodoroTickPayload) {
    if let Err(e) = app.emit("pomodoro-tick", payload) {
        tracing::error!("emit pomodoro-tick: {e}");
    }
}

fn emit_phase_changed(app: &AppHandle, payload: &PomodoroPhaseChangedPayload) {
    if let Err(e) = app.emit("pomodoro-phase-changed", payload) {
        tracing::error!("emit pomodoro-phase-changed: {e}");
    }
}

fn notify(app: &AppHandle, title: &str, body: &str) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.request_user_attention(Some(tauri::UserAttentionType::Informational));
    }
    if let Err(e) = app.emit("app-notification", serde_json::json!({ "title": title, "body": body })) {
        tracing::error!("emit app-notification: {e}");
    }
}

// ---------------------------------------------------------------------------
// DB helpers
// ---------------------------------------------------------------------------

async fn count_today_sessions(pool: &SqlitePool) -> u32 {
    let today = Utc::now().date_naive().to_string();
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pomodoro_sessions
         WHERE phase = 'work' AND completed = 1
         AND started_at >= ?",
    )
    .bind(format!("{today}T00:00:00+00:00"))
    .fetch_one(pool)
    .await
    .unwrap_or(0) as u32
}

/// Inserts a new in-progress pomodoro row and returns its id.
async fn open_db_session(pool: &SqlitePool, phase: &str) -> Option<i64> {
    let now = Utc::now().to_rfc3339();
    sqlx::query_scalar::<_, i64>(
        "INSERT INTO pomodoro_sessions (started_at, phase) VALUES (?, ?) RETURNING id",
    )
    .bind(&now)
    .bind(phase)
    .fetch_one(pool)
    .await
    .ok()
}

/// Closes (or inserts) a pomodoro row with the final stats.
async fn record_session(
    pool: &SqlitePool,
    phase: &str,
    duration_ms: u64,
    completed: bool,
    id: Option<i64>,
) {
    let now = Utc::now().to_rfc3339();
    if let Some(row_id) = id {
        let _ = sqlx::query(
            "UPDATE pomodoro_sessions
             SET ended_at = ?, duration_ms = ?, completed = ?, interrupted = ?
             WHERE id = ?",
        )
        .bind(&now)
        .bind(duration_ms as i64)
        .bind(completed as i64)
        .bind(!completed as i64)
        .bind(row_id)
        .execute(pool)
        .await;
    } else {
        let started = Utc::now()
            - chrono::Duration::milliseconds(duration_ms as i64);
        let _ = sqlx::query(
            "INSERT INTO pomodoro_sessions
                 (started_at, ended_at, duration_ms, phase, completed, interrupted)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(started.to_rfc3339())
        .bind(&now)
        .bind(duration_ms as i64)
        .bind(phase)
        .bind(completed as i64)
        .bind(!completed as i64)
        .execute(pool)
        .await;
    }
}
