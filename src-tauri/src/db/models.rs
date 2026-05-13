#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Legacy models (kept for backward compatibility with existing commands)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppUsageStat {
    pub exe: String,
    pub total_ms: i64,
    pub category: String,
    pub session_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklySummary {
    pub days: Vec<String>,
    pub scores: Vec<f32>,
    pub total_active_ms: i64,
    pub avg_score: f32,
}

// ---------------------------------------------------------------------------
// Normalized schema models
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub is_productive: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct App {
    pub id: i64,
    pub exe: String,
    pub display_name: String,
    pub exe_path: String,
    pub category_id: i64,
    pub first_seen: String,
    pub last_seen: String,
    pub total_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActivitySession {
    pub id: i64,
    pub app_id: i64,
    pub title: String,
    pub started_at: String,
    pub ended_at: String,
    pub duration_ms: i64,
}

/// Pre-computed aggregate for one calendar day (UTC).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DailyMetrics {
    pub date: String,
    pub total_ms: i64,
    pub productive_ms: i64,
    pub distraction_ms: i64,
    pub neutral_ms: i64,
    pub focus_score: f64,
    pub session_count: i64,
    pub switch_count: i64,
    pub updated_at: String,
}

/// Computed values passed to `MetricsRepository::upsert`.
#[derive(Debug, Clone, Default)]
pub struct DailyMetricsInput {
    pub total_ms: i64,
    pub productive_ms: i64,
    pub distraction_ms: i64,
    pub neutral_ms: i64,
    pub focus_score: f64,
    pub session_count: i64,
    pub switch_count: i64,
}

/// Hourly activity breakdown — used by the timeline view.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct HourlyStats {
    pub hour: i64,
    pub total_ms: i64,
    pub productive_ms: i64,
    pub session_count: i64,
}

/// Per-app aggregate — richer version of `AppUsageStat`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppUsageDetail {
    pub exe: String,
    pub display_name: String,
    pub category: String,
    pub total_ms: i64,
    pub session_count: i64,
    pub exe_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SettingRow {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}
