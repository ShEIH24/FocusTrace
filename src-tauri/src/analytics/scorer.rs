use crate::analytics::config::AnalyticsConfig;
use crate::analytics::focus::detect_focus_sessions;
use crate::analytics::models::ScoreBreakdown;
use crate::events::types::SessionInfo;

/// Backward-compatible entry point — returns a single f32 score using defaults.
pub struct FocusScore;

impl FocusScore {
    pub fn calculate(sessions: &[SessionInfo]) -> f32 {
        Scorer::new(&AnalyticsConfig::default()).score(sessions).total as f32
    }
}

pub struct Scorer<'a> {
    config: &'a AnalyticsConfig,
}

impl<'a> Scorer<'a> {
    pub fn new(config: &'a AnalyticsConfig) -> Self {
        Self { config }
    }

    pub fn score(&self, sessions: &[SessionInfo]) -> ScoreBreakdown {
        let total_ms: i64 = sessions.iter().map(|s| s.duration_ms).sum();
        if total_ms == 0 {
            return ScoreBreakdown {
                total: 0.0,
                productive_ratio_component: 0.0,
                focus_depth_component: 0.0,
                switch_penalty_component: 0.0,
                continuity_component: 0.0,
            };
        }

        let w = &self.config.weights;

        // Component 1: productive time ratio (0–100)
        let productive_ms: i64 = sessions
            .iter()
            .filter(|s| s.category == "Productive")
            .map(|s| s.duration_ms)
            .sum();
        let c1 = (productive_ms as f64 / total_ms as f64 * 100.0).clamp(0.0, 100.0);

        // Component 2: average depth score across *productive* focus blocks (0–100)
        let productive_sessions: Vec<SessionInfo> = sessions
            .iter()
            .filter(|s| s.category == "Productive")
            .cloned()
            .collect();
        let focus_sessions = detect_focus_sessions(&productive_sessions, &self.config.focus);
        let c2 = if focus_sessions.is_empty() {
            0.0
        } else {
            focus_sessions.iter().map(|s| s.depth.score()).sum::<f64>()
                / focus_sessions.len() as f64
        };

        // Component 3: switch score — penalizes high churn (0–100)
        let total_hours = total_ms as f64 / 3_600_000.0;
        let switch_rate = if total_hours > 0.0 {
            sessions.len().saturating_sub(1) as f64 / total_hours
        } else {
            0.0
        };
        let c3 = (self.config.switching.switch_threshold / switch_rate.max(0.1) * 100.0)
            .clamp(0.0, 100.0);

        // Component 4: continuity — longest focus block relative to 2-hour target (0–100)
        const TARGET_MS: f64 = 7_200_000.0;
        let longest_ms = focus_sessions.iter().map(|s| s.duration_ms).max().unwrap_or(0);
        let c4 = (longest_ms as f64 / TARGET_MS * 100.0).clamp(0.0, 100.0);

        let total = (c1 * w.productive_ratio
            + c2 * w.focus_depth
            + c3 * w.switch_penalty
            + c4 * w.continuity)
            .clamp(0.0, 100.0);

        ScoreBreakdown {
            total,
            productive_ratio_component: c1,
            focus_depth_component: c2,
            switch_penalty_component: c3,
            continuity_component: c4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn productive(duration_ms: i64) -> SessionInfo {
        SessionInfo {
            exe: "code.exe".into(),
            title: String::new(),
            started_at: Utc::now(),
            duration_ms,
            category: "Productive".into(),
            exe_path: String::new(),
        }
    }

    fn distraction(duration_ms: i64) -> SessionInfo {
        SessionInfo {
            exe: "youtube.exe".into(),
            title: String::new(),
            started_at: Utc::now(),
            duration_ms,
            category: "Distraction".into(),
            exe_path: String::new(),
        }
    }

    #[test]
    fn empty_sessions_returns_zero() {
        assert_eq!(FocusScore::calculate(&[]), 0.0);
    }

    #[test]
    fn all_distraction_returns_low_score() {
        let sessions: Vec<_> = (0..5).map(|_| distraction(600_000)).collect();
        let score = FocusScore::calculate(&sessions);
        assert!(score < 30.0, "expected low score, got {score}");
    }

    #[test]
    fn scorer_components_sum_to_total() {
        let config = AnalyticsConfig::default();
        let scorer = Scorer::new(&config);
        let sessions = vec![productive(3_600_000)];
        let b = scorer.score(&sessions);
        let w = &config.weights;
        let expected = b.productive_ratio_component * w.productive_ratio
            + b.focus_depth_component * w.focus_depth
            + b.switch_penalty_component * w.switch_penalty
            + b.continuity_component * w.continuity;
        assert!((b.total - expected).abs() < 0.01);
    }
}
