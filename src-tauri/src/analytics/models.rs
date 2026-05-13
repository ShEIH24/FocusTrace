#![allow(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayReport {
    pub date: NaiveDate,
    pub total_ms: i64,
    pub productive_ms: i64,
    pub distraction_ms: i64,
    pub neutral_ms: i64,
    pub session_count: usize,
    pub score: ScoreBreakdown,
    pub focus_sessions: Vec<FocusSession>,
    pub deep_work: Vec<DeepWorkInterval>,
    pub context_switches: ContextSwitchStats,
    pub app_breakdown: Vec<AppBreakdown>,
    pub category_breakdown: Vec<CategoryBreakdown>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekReport {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: Vec<DayReport>,
    pub avg_score: f64,
    pub total_ms: i64,
    pub productive_ms: i64,
    pub top_apps: Vec<AppBreakdown>,
    pub summary: String,
}

/// A merged block of continuous focused activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub duration_ms: i64,
    pub depth: DepthLevel,
    pub primary_app: String,
    pub session_count: usize,
}

/// Classification of a focus session by duration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DepthLevel {
    Shallow,  // < min_focus_ms
    Focused,  // min_focus_ms – deep_work_ms
    Deep,     // deep_work_ms – flow_ms
    Flow,     // >= flow_ms
}

impl DepthLevel {
    pub fn score(&self) -> f64 {
        match self {
            DepthLevel::Shallow => 25.0,
            DepthLevel::Focused => 50.0,
            DepthLevel::Deep => 75.0,
            DepthLevel::Flow => 100.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            DepthLevel::Shallow => "Поверхностная",
            DepthLevel::Focused => "Сфокусированная",
            DepthLevel::Deep => "Глубокая",
            DepthLevel::Flow => "Поток",
        }
    }
}

/// A focus block that meets the deep work threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepWorkInterval {
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub duration_ms: i64,
    pub depth: DepthLevel,
    pub apps: Vec<String>,
}

/// Aggregate context-switching statistics for a day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSwitchStats {
    pub total_switches: usize,
    pub switches_per_hour: f64,
    pub rapid_switches: usize,
    pub sprees: Vec<SwitchSpree>,
    pub insight: String,
}

/// A run of rapid consecutive application switches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchSpree {
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub switch_count: usize,
}

/// Per-component score breakdown (each component 0–100).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub total: f64,
    pub productive_ratio_component: f64,
    pub focus_depth_component: f64,
    pub switch_penalty_component: f64,
    pub continuity_component: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBreakdown {
    pub exe: String,
    pub display_name: String,
    pub category: String,
    pub total_ms: i64,
    pub session_count: usize,
    pub percentage: f64,
    pub formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    pub category: String,
    pub total_ms: i64,
    pub percentage: f64,
    pub formatted: String,
}
