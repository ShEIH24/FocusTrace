use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    pub weights: ScoringWeights,
    pub focus: FocusThresholds,
    pub switching: SwitchConfig,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            weights: ScoringWeights::default(),
            focus: FocusThresholds::default(),
            switching: SwitchConfig::default(),
        }
    }
}

/// Relative weights for the four scoring components (should sum to ~1.0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub productive_ratio: f64,
    pub focus_depth: f64,
    pub switch_penalty: f64,
    pub continuity: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            productive_ratio: 0.40,
            focus_depth: 0.30,
            switch_penalty: 0.20,
            continuity: 0.10,
        }
    }
}

/// Time thresholds used to classify focus session depth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusThresholds {
    /// Minimum continuous productive time to qualify as a focus session (ms).
    pub min_focus_ms: i64,
    /// Max gap between consecutive sessions to merge into one block (ms).
    pub merge_gap_ms: i64,
    /// Minimum duration for a deep work block — Pomodoro unit (ms).
    pub deep_work_ms: i64,
    /// Minimum duration for a flow state block (ms).
    pub flow_ms: i64,
}

impl Default for FocusThresholds {
    fn default() -> Self {
        Self {
            min_focus_ms: 600_000,   // 10 minutes
            merge_gap_ms: 120_000,   // 2 minutes
            deep_work_ms: 1_500_000, // 25 minutes
            flow_ms: 5_400_000,      // 90 minutes
        }
    }
}

/// Parameters for context-switch analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchConfig {
    /// Switches/hour above which the score is penalized.
    pub switch_threshold: f64,
    /// Two consecutive switches within this window count as "rapid" (ms).
    pub rapid_switch_ms: i64,
    /// Minimum number of consecutive rapid switches to form a spree.
    pub spree_min_count: usize,
}

impl Default for SwitchConfig {
    fn default() -> Self {
        Self {
            switch_threshold: 20.0,
            rapid_switch_ms: 60_000, // 1 minute
            spree_min_count: 5,
        }
    }
}
