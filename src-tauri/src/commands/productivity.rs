use tauri::State;

use crate::error::AppError;
use crate::productivity::models::{
    GoalProgress, PomodoroTickPayload, ProductivityConfig, StreakInfo,
};
use crate::productivity::pomodoro::PomodoroCommand;
use crate::productivity::{goals, load_config, save_config};
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Pomodoro controls
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn pomodoro_start(state: State<'_, AppState>) -> Result<(), AppError> {
    state
        .pomodoro_tx
        .send(PomodoroCommand::Start)
        .await
        .map_err(|_| AppError::TrackingError("pomodoro worker unavailable".into()))
}

#[tauri::command]
pub async fn pomodoro_pause(state: State<'_, AppState>) -> Result<(), AppError> {
    state
        .pomodoro_tx
        .send(PomodoroCommand::Pause)
        .await
        .map_err(|_| AppError::TrackingError("pomodoro worker unavailable".into()))
}

#[tauri::command]
pub async fn pomodoro_resume(state: State<'_, AppState>) -> Result<(), AppError> {
    state
        .pomodoro_tx
        .send(PomodoroCommand::Resume)
        .await
        .map_err(|_| AppError::TrackingError("pomodoro worker unavailable".into()))
}

#[tauri::command]
pub async fn pomodoro_stop(state: State<'_, AppState>) -> Result<(), AppError> {
    state
        .pomodoro_tx
        .send(PomodoroCommand::Stop)
        .await
        .map_err(|_| AppError::TrackingError("pomodoro worker unavailable".into()))
}

#[tauri::command]
pub async fn pomodoro_skip(state: State<'_, AppState>) -> Result<(), AppError> {
    state
        .pomodoro_tx
        .send(PomodoroCommand::Skip)
        .await
        .map_err(|_| AppError::TrackingError("pomodoro worker unavailable".into()))
}

/// Returns the latest timer snapshot. The frontend polls this every second
/// as a reliable alternative to Tauri events.
#[tauri::command]
pub async fn get_pomodoro_state(
    state: State<'_, AppState>,
) -> Result<PomodoroTickPayload, AppError> {
    Ok(state.pomodoro_state.read().await.clone())
}

// ---------------------------------------------------------------------------
// Productivity config
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn get_productivity_config(
    state: State<'_, AppState>,
) -> Result<ProductivityConfig, AppError> {
    Ok(load_config(&state.pool).await)
}

#[tauri::command]
pub async fn update_productivity_config(
    state: State<'_, AppState>,
    config: ProductivityConfig,
) -> Result<(), AppError> {
    // Propagate new pomodoro config to the running worker.
    let _ = state
        .pomodoro_tx
        .send(PomodoroCommand::UpdateConfig(config.pomodoro.clone()))
        .await;

    // Update focus mode flag in cache.
    {
        let mut cache = state.cache.write().await;
        cache.focus_mode_enabled = config.focus_mode.enabled;
        cache.focus_alert_threshold_secs = config.focus_mode.alert_threshold_secs;
    }

    save_config(&state.pool, &config).await
}

// ---------------------------------------------------------------------------
// Goals + streak
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn get_daily_goals(state: State<'_, AppState>) -> Result<GoalProgress, AppError> {
    let cfg = load_config(&state.pool).await;
    goals::compute_goal_progress(&state.pool, &cfg.goals).await
}

#[tauri::command]
pub async fn get_streak(state: State<'_, AppState>) -> Result<StreakInfo, AppError> {
    let cfg = load_config(&state.pool).await;
    let target_ms = cfg.goals.productive_target_mins as i64 * 60_000;
    goals::compute_streak(&state.pool, target_ms).await
}

// ---------------------------------------------------------------------------
// Focus mode toggle (shortcut — avoids a full config round-trip)
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn toggle_focus_mode(state: State<'_, AppState>) -> Result<bool, AppError> {
    let new_state = {
        let mut cache = state.cache.write().await;
        cache.focus_mode_enabled = !cache.focus_mode_enabled;
        cache.distraction_count = 0; // reset counter on toggle
        cache.focus_mode_enabled
    };

    // Persist the new state.
    let mut cfg = load_config(&state.pool).await;
    cfg.focus_mode.enabled = new_state;
    save_config(&state.pool, &cfg).await?;

    Ok(new_state)
}

#[tauri::command]
pub async fn get_focus_mode_state(state: State<'_, AppState>) -> Result<bool, AppError> {
    Ok(state.cache.read().await.focus_mode_enabled)
}
