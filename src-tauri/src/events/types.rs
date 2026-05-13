use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Window / session types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub exe: String,
    pub title: String,
    pub pid: u32,
    pub exe_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SessionInfo {
    pub exe: String,
    pub title: String,
    pub started_at: DateTime<Utc>,
    pub duration_ms: i64,
    pub category: String,
    #[sqlx(default)]
    pub exe_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEndedPayload {
    pub exe: String,
    pub title: String,
    pub started_at: String,
    pub duration_ms: i64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsUpdatedPayload {
    pub date: String,
    pub focus_score: f64,
    pub total_ms: i64,
    pub productive_ms: i64,
    pub distraction_ms: i64,
    pub neutral_ms: i64,
    pub session_count: i64,
}

impl From<&SessionInfo> for SessionEndedPayload {
    fn from(s: &SessionInfo) -> Self {
        Self {
            exe: s.exe.clone(),
            title: s.title.clone(),
            started_at: s.started_at.to_rfc3339(),
            duration_ms: s.duration_ms,
            category: s.category.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Productivity event payloads (emitted directly by workers, not via channel)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistractionAlertPayload {
    pub exe: String,
    pub distraction_count: u32,
    pub active_secs: u32,
}

// ---------------------------------------------------------------------------
// Internal application event channel (coordinator ↔ workers)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum AppEvent {
    WindowChanged(WindowInfo),
    IdleStarted { at: DateTime<Utc> },
    IdleEnded { duration_secs: u64 },
    SessionEnded(SessionInfo),
    Shutdown,
}
