use chrono::NaiveDate;
use serde::Serialize;
use tauri::State;

use crate::db::models::{AppUsageStat, HourlyStats};
use crate::db::queries::{get_app_usage_stats, get_sessions_for_date};
use crate::db::repository::SessionRepository;
use crate::error::AppError;
use crate::events::types::{SessionInfo, WindowInfo};
use crate::state::AppState;

#[tauri::command]
pub async fn get_current_activity(
    state: State<'_, AppState>,
) -> Result<Option<WindowInfo>, AppError> {
    let cache = state.cache.read().await;
    Ok(cache.current_window.clone())
}

#[tauri::command]
pub async fn get_activity_log(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<SessionInfo>, AppError> {
    let date = parse_date(&date)?;
    get_sessions_for_date(&state.pool, date).await
}

#[tauri::command]
pub async fn get_app_usage(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<AppUsageStat>, AppError> {
    let date = parse_date(&date)?;
    get_app_usage_stats(&state.pool, date).await
}

/// Hourly activity breakdown for `date` — each entry covers one clock hour.
/// Hours with no activity are omitted (sparse result).
#[tauri::command]
pub async fn get_hourly_stats(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<HourlyStats>, AppError> {
    let date = parse_date(&date)?;
    SessionRepository::new(&state.pool)
        .hourly_breakdown(date)
        .await
}

/// Returns a snapshot of the in-memory activity cache.
/// Cheaper than a DB round-trip; used by the frontend on startup to
/// restore UI state before the first real-time event arrives.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveCacheSnapshot {
    pub current_window: Option<WindowInfo>,
    pub is_idle: bool,
    pub idle_since: Option<String>,
    pub today_score: f32,
    pub today_total_ms: i64,
    pub today_productive_ms: i64,
}

#[tauri::command]
pub async fn get_live_cache(state: State<'_, AppState>) -> Result<LiveCacheSnapshot, AppError> {
    let c = state.cache.read().await;
    Ok(LiveCacheSnapshot {
        current_window: c.current_window.clone(),
        is_idle: c.is_idle,
        idle_since: c.idle_since.map(|t| t.to_rfc3339()),
        today_score: c.today_score,
        today_total_ms: c.today_total_ms,
        today_productive_ms: c.today_productive_ms,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| AppError::ConfigError(format!("invalid date '{s}': {e}")))
}
