use std::collections::HashSet;

use chrono::{DateTime, Duration, Utc};

use crate::analytics::config::FocusThresholds;
use crate::analytics::models::{DeepWorkInterval, DepthLevel, FocusSession};
use crate::events::types::SessionInfo;

/// Merges consecutive sessions separated by at most `merge_gap_ms` into focus
/// blocks, then keeps only blocks meeting `min_focus_ms`.  O(n).
pub fn detect_focus_sessions(sessions: &[SessionInfo], cfg: &FocusThresholds) -> Vec<FocusSession> {
    if sessions.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    // Open block: (start, end, accumulated_ms, dominant_app, session_count)
    type Block = (DateTime<Utc>, DateTime<Utc>, i64, String, usize);
    let mut open: Option<Block> = None;

    for s in sessions {
        let s_end = s.started_at + Duration::milliseconds(s.duration_ms);

        open = match open.take() {
            None => Some((s.started_at, s_end, s.duration_ms, s.exe.clone(), 1)),
            Some((b_start, b_end, b_ms, b_app, b_count)) => {
                let gap = (s.started_at - b_end).num_milliseconds();
                if gap <= cfg.merge_gap_ms {
                    // Promote current session to dominant if it outweighs the block so far
                    let dominant = if s.duration_ms > b_ms {
                        s.exe.clone()
                    } else {
                        b_app
                    };
                    Some((
                        b_start,
                        s_end.max(b_end),
                        b_ms + s.duration_ms,
                        dominant,
                        b_count + 1,
                    ))
                } else {
                    flush_block(b_start, b_end, b_ms, b_app, b_count, cfg, &mut result);
                    Some((s.started_at, s_end, s.duration_ms, s.exe.clone(), 1))
                }
            }
        };
    }

    if let Some((b_start, b_end, b_ms, b_app, b_count)) = open {
        flush_block(b_start, b_end, b_ms, b_app, b_count, cfg, &mut result);
    }

    result
}

#[inline]
fn flush_block(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    ms: i64,
    app: String,
    count: usize,
    cfg: &FocusThresholds,
    out: &mut Vec<FocusSession>,
) {
    if ms >= cfg.min_focus_ms {
        out.push(FocusSession {
            started_at: start,
            ended_at: end,
            duration_ms: ms,
            depth: classify_depth(ms, cfg),
            primary_app: app,
            session_count: count,
        });
    }
}

/// Returns only the deep-work / flow blocks, annotated with the unique apps used.
pub fn detect_deep_work(sessions: &[SessionInfo], cfg: &FocusThresholds) -> Vec<DeepWorkInterval> {
    detect_focus_sessions(sessions, cfg)
        .into_iter()
        .filter(|s| s.depth >= DepthLevel::Deep)
        .map(|s| {
            let apps: Vec<String> = sessions
                .iter()
                .filter(|r| r.started_at >= s.started_at && r.started_at < s.ended_at)
                .map(|r| r.exe.clone())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            DeepWorkInterval {
                started_at: s.started_at,
                ended_at: s.ended_at,
                duration_ms: s.duration_ms,
                depth: s.depth,
                apps,
            }
        })
        .collect()
}

pub fn classify_depth(duration_ms: i64, cfg: &FocusThresholds) -> DepthLevel {
    if duration_ms >= cfg.flow_ms {
        DepthLevel::Flow
    } else if duration_ms >= cfg.deep_work_ms {
        DepthLevel::Deep
    } else if duration_ms >= cfg.min_focus_ms {
        DepthLevel::Focused
    } else {
        DepthLevel::Shallow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn make_session(start_offset_ms: i64, duration_ms: i64, exe: &str) -> SessionInfo {
        let base = Utc::now();
        SessionInfo {
            exe: exe.to_string(),
            title: String::new(),
            started_at: base + Duration::milliseconds(start_offset_ms),
            duration_ms,
            category: "Productive".to_string(),
            exe_path: String::new(),
        }
    }

    #[test]
    fn single_session_below_threshold_produces_no_block() {
        let cfg = FocusThresholds::default();
        let sessions = vec![make_session(0, 300_000, "code.exe")]; // 5 min < 10 min threshold
        assert!(detect_focus_sessions(&sessions, &cfg).is_empty());
    }

    #[test]
    fn single_long_session_produces_focused_block() {
        let cfg = FocusThresholds::default();
        let sessions = vec![make_session(0, 900_000, "code.exe")]; // 15 min
        let blocks = detect_focus_sessions(&sessions, &cfg);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].depth, DepthLevel::Focused);
    }

    #[test]
    fn sessions_within_gap_are_merged() {
        let cfg = FocusThresholds::default();
        // Two 5-min sessions with 1-min gap: merged to 11 min total → Focused
        let s1 = make_session(0, 300_000, "code.exe");
        let s2 = make_session(360_000, 300_000, "code.exe"); // gap = 60s < 120s
        let sessions = vec![s1, s2];
        let blocks = detect_focus_sessions(&sessions, &cfg);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].duration_ms >= 600_000);
    }

    #[test]
    fn sessions_beyond_gap_produce_separate_blocks() {
        let cfg = FocusThresholds::default();
        let s1 = make_session(0, 700_000, "code.exe"); // 11.6 min
        let s2 = make_session(1_000_000, 700_000, "code.exe"); // gap 300s > 120s
        let blocks = detect_focus_sessions(&vec![s1, s2], &cfg);
        assert_eq!(blocks.len(), 2);
    }
}
