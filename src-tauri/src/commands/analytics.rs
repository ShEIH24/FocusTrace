use chrono::NaiveDate;
use tauri::State;

use crate::analytics::engine::AnalyticsEngine;
use crate::analytics::models::{DayReport, WeekReport};
use crate::db::models::WeeklySummary;
use crate::error::AppError;
use crate::state::AppState;

#[tauri::command]
pub async fn get_focus_score(state: State<'_, AppState>, date: String) -> Result<f32, AppError> {
    let date = parse_date(&date)?;
    AnalyticsEngine::new(state.pool.clone())
        .focus_score_for_date(date)
        .await
}

#[tauri::command]
pub async fn get_weekly_summary(state: State<'_, AppState>) -> Result<WeeklySummary, AppError> {
    AnalyticsEngine::new(state.pool.clone())
        .weekly_summary()
        .await
}

#[tauri::command]
pub async fn get_day_report(
    state: State<'_, AppState>,
    date: String,
) -> Result<DayReport, AppError> {
    let date = parse_date(&date)?;
    AnalyticsEngine::new(state.pool.clone())
        .day_report(date)
        .await
}

#[tauri::command]
pub async fn get_week_report(
    state: State<'_, AppState>,
    end_date: String,
) -> Result<WeekReport, AppError> {
    let end_date = parse_date(&end_date)?;
    AnalyticsEngine::new(state.pool.clone())
        .week_report(end_date)
        .await
}

fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| AppError::ConfigError(format!("invalid date '{s}': {e}")))
}
