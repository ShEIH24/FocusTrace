use chrono::{DateTime, Duration, Utc};

use crate::analytics::config::SwitchConfig;
use crate::analytics::models::{ContextSwitchStats, SwitchSpree};
use crate::events::types::SessionInfo;

pub fn analyze_context_switching(
    sessions: &[SessionInfo],
    total_ms: i64,
    cfg: &SwitchConfig,
) -> ContextSwitchStats {
    if sessions.len() < 2 {
        return ContextSwitchStats {
            total_switches: 0,
            switches_per_hour: 0.0,
            rapid_switches: 0,
            sprees: Vec::new(),
            insight: "Недостаточно данных".into(),
        };
    }

    let total_hours = total_ms as f64 / 3_600_000.0;
    let total_switches = sessions.len() - 1;
    let switches_per_hour = if total_hours > 0.0 {
        total_switches as f64 / total_hours
    } else {
        0.0
    };

    let (rapid_switches, sprees) = detect_rapid_sprees(sessions, cfg);

    let insight = build_insight(switches_per_hour, cfg.switch_threshold, &sprees);

    ContextSwitchStats {
        total_switches,
        switches_per_hour,
        rapid_switches,
        sprees,
        insight,
    }
}

/// Returns `(rapid_switch_count, sprees)`.
fn detect_rapid_sprees(
    sessions: &[SessionInfo],
    cfg: &SwitchConfig,
) -> (usize, Vec<SwitchSpree>) {
    let mut rapid_count = 0usize;
    let mut sprees: Vec<SwitchSpree> = Vec::new();

    let mut spree_start: Option<DateTime<Utc>> = None;
    let mut spree_len = 0usize;
    let mut spree_last_end: Option<DateTime<Utc>> = None;

    for pair in sessions.windows(2) {
        let prev = &pair[0];
        let curr = &pair[1];
        let gap_ms = (curr.started_at - prev.started_at).num_milliseconds();

        if gap_ms <= cfg.rapid_switch_ms {
            rapid_count += 1;
            spree_len += 1;
            if spree_start.is_none() {
                spree_start = Some(prev.started_at);
            }
            spree_last_end = Some(curr.started_at + Duration::milliseconds(curr.duration_ms));
        } else {
            maybe_flush_spree(spree_start, spree_last_end, spree_len, cfg, &mut sprees);
            spree_start = None;
            spree_last_end = None;
            spree_len = 0;
        }
    }
    maybe_flush_spree(spree_start, spree_last_end, spree_len, cfg, &mut sprees);

    (rapid_count, sprees)
}

#[inline]
fn maybe_flush_spree(
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    count: usize,
    cfg: &SwitchConfig,
    out: &mut Vec<SwitchSpree>,
) {
    if count >= cfg.spree_min_count {
        if let (Some(s), Some(e)) = (start, end) {
            out.push(SwitchSpree {
                started_at: s,
                ended_at: e,
                switch_count: count,
            });
        }
    }
}

fn build_insight(rate: f64, threshold: f64, sprees: &[SwitchSpree]) -> String {
    if rate == 0.0 {
        return "Нет переключений".into();
    }
    if !sprees.is_empty() {
        format!(
            "Частые переключения снижают фокус: {} серий быстрых переключений",
            sprees.len()
        )
    } else if rate > threshold {
        format!(
            "Высокая частота переключений: {:.0} раз/час (норма < {:.0})",
            rate, threshold
        )
    } else {
        format!("Хороший фокус: {:.0} переключений/час", rate)
    }
}
