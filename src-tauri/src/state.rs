use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::browser::BrowserState;
use crate::config::app_config::AppConfig;
use crate::events::types::{AppEvent, WindowInfo};
use crate::productivity::models::PomodoroTickPayload;
use crate::productivity::pomodoro::PomodoroCommand;

#[derive(Debug, Default)]
pub struct ActivityCache {
    pub current_window: Option<WindowInfo>,
    pub is_idle: bool,
    pub idle_since: Option<DateTime<Utc>>,
    /// Focus score for today (0–100), updated after each session is persisted.
    pub today_score: f32,
    pub today_total_ms: i64,
    pub today_productive_ms: i64,
    /// Focus mode: detect and alert on distraction apps.
    pub focus_mode_enabled: bool,
    pub focus_alert_threshold_secs: u32,
    /// Counts distraction windows seen during the current focus session.
    pub distraction_count: u32,
    /// Seconds the current distraction app has been active.
    pub distraction_active_secs: u32,
}

/// Shared application state passed to Tauri commands and workers.
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Arc<RwLock<AppConfig>>,
    pub event_tx: mpsc::Sender<AppEvent>,
    pub cache: Arc<RwLock<ActivityCache>>,
    pub pomodoro_tx: mpsc::Sender<PomodoroCommand>,
    pub browser: Arc<BrowserState>,
    /// Latest pomodoro tick snapshot, updated every second by the worker.
    /// Used by the `get_pomodoro_state` command for frontend polling.
    pub pomodoro_state: Arc<RwLock<PomodoroTickPayload>>,
}
