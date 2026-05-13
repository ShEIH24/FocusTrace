use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Pomodoro
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroConfig {
    pub work_mins: u32,
    pub short_break_mins: u32,
    pub long_break_mins: u32,
    pub sessions_until_long: u32,
    pub auto_start_breaks: bool,
    pub auto_start_work: bool,
}

impl Default for PomodoroConfig {
    fn default() -> Self {
        Self {
            work_mins: 25,
            short_break_mins: 5,
            long_break_mins: 15,
            sessions_until_long: 4,
            auto_start_breaks: false,
            auto_start_work: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PomodoroPhase {
    #[default]
    Idle,
    Working,
    ShortBreak,
    LongBreak,
}

impl PomodoroPhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Working => "working",
            Self::ShortBreak => "short_break",
            Self::LongBreak => "long_break",
        }
    }
}

/// Payload emitted every tick (1 Hz) while the timer is running.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroTickPayload {
    pub phase: String,
    pub remaining_secs: u64,
    pub total_secs: u64,
    pub session_count: u32,
    pub is_paused: bool,
}

impl Default for PomodoroTickPayload {
    fn default() -> Self {
        Self {
            phase: "idle".into(),
            remaining_secs: 0,
            total_secs: 0,
            session_count: 0,
            is_paused: false,
        }
    }
}

/// Payload emitted when the timer phase changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroPhaseChangedPayload {
    pub phase: String,
    pub session_count: u32,
    pub total_secs: u64,
}

// ---------------------------------------------------------------------------
// Daily goals
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalConfig {
    /// Target productive time per day (minutes). 0 = disabled.
    pub productive_target_mins: u32,
    /// Target number of completed pomodoros. 0 = disabled.
    pub pomodoro_target: u32,
    /// Target focus score (0–100). 0 = disabled.
    pub score_target: f64,
}

impl Default for GoalConfig {
    fn default() -> Self {
        Self {
            productive_target_mins: 240, // 4 hours
            pomodoro_target: 8,
            score_target: 70.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalProgress {
    pub date: String,
    pub productive_target_ms: i64,
    pub productive_actual_ms: i64,
    pub productive_pct: f64,
    pub pomodoro_target: u32,
    pub pomodoro_actual: u32,
    pub pomodoro_pct: f64,
    pub score_target: f64,
    pub score_actual: f64,
    pub score_pct: f64,
    pub overall_met: bool,
}

// ---------------------------------------------------------------------------
// Focus mode
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusModeConfig {
    pub enabled: bool,
    /// Alert after a distraction app has been active this many seconds.
    pub alert_threshold_secs: u32,
}

impl Default for FocusModeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            alert_threshold_secs: 30,
        }
    }
}

// ---------------------------------------------------------------------------
// Streaks
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreakInfo {
    pub current: u32,
    pub longest: u32,
    /// 7-element array (oldest → newest). true = productive day.
    pub last_7_days: Vec<bool>,
    pub last_productive_date: Option<String>,
}

// ---------------------------------------------------------------------------
// Full productivity config (stored as JSON in settings table)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityConfig {
    pub pomodoro: PomodoroConfig,
    pub goals: GoalConfig,
    pub focus_mode: FocusModeConfig,
}
