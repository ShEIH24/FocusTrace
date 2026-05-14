use std::collections::HashMap;

use crate::analytics::models::{AppBreakdown, CategoryBreakdown, FocusSession};
use crate::events::types::SessionInfo;

pub fn aggregate_by_app(sessions: &[SessionInfo], total_ms: i64) -> Vec<AppBreakdown> {
    let mut map: HashMap<&str, (i64, usize, &str)> = HashMap::new();

    for s in sessions {
        let e = map
            .entry(s.exe.as_str())
            .or_insert((0, 0, s.category.as_str()));
        e.0 += s.duration_ms;
        e.1 += 1;
    }

    let mut result: Vec<AppBreakdown> = map
        .into_iter()
        .map(|(exe, (ms, count, category))| {
            let pct = if total_ms > 0 {
                ms as f64 / total_ms as f64 * 100.0
            } else {
                0.0
            };
            AppBreakdown {
                display_name: exe.trim_end_matches(".exe").to_string(),
                formatted: format_duration(ms),
                exe: exe.to_string(),
                category: category.to_string(),
                total_ms: ms,
                session_count: count,
                percentage: pct,
            }
        })
        .collect();

    result.sort_by_key(|b| std::cmp::Reverse(b.total_ms));
    result
}

pub fn aggregate_by_category(apps: &[AppBreakdown], total_ms: i64) -> Vec<CategoryBreakdown> {
    let mut map: HashMap<&str, i64> = HashMap::new();
    for app in apps {
        *map.entry(app.category.as_str()).or_insert(0) += app.total_ms;
    }

    let mut result: Vec<CategoryBreakdown> = map
        .into_iter()
        .map(|(category, ms)| {
            let pct = if total_ms > 0 {
                ms as f64 / total_ms as f64 * 100.0
            } else {
                0.0
            };
            CategoryBreakdown {
                formatted: format_duration(ms),
                category: category.to_string(),
                total_ms: ms,
                percentage: pct,
            }
        })
        .collect();

    result.sort_by_key(|b| std::cmp::Reverse(b.total_ms));
    result
}

/// Formats milliseconds as a human-readable Russian string.
/// "3 ч 42 мин" / "42 мин" / "45 сек"
pub fn format_duration(ms: i64) -> String {
    let total_secs = ms / 1000;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        if minutes > 0 {
            format!("{hours} ч {minutes} мин")
        } else {
            format!("{hours} ч")
        }
    } else if minutes > 0 {
        format!("{minutes} мин")
    } else {
        format!("{seconds} сек")
    }
}

pub fn build_day_summary(
    total_ms: i64,
    productive_ms: i64,
    apps: &[AppBreakdown],
    focus_sessions: &[FocusSession],
    score: f64,
) -> String {
    if total_ms == 0 {
        return "Нет данных за этот день".into();
    }

    let mut parts: Vec<String> = Vec::new();

    if let Some(top) = apps.first() {
        parts.push(format!("{} в {}", top.formatted, top.display_name));
    }

    // Productive time line — only when it differs from the top app
    let top_ms = apps.first().map(|a| a.total_ms).unwrap_or(0);
    if productive_ms > 0 && productive_ms != top_ms {
        parts.push(format!(
            "{} продуктивной работы",
            format_duration(productive_ms)
        ));
    }

    if !focus_sessions.is_empty() {
        let best = focus_sessions.iter().max_by_key(|s| s.duration_ms);
        if let Some(b) = best {
            parts.push(format!(
                "лучший фокус-блок: {} ({})",
                format_duration(b.duration_ms),
                b.depth.label()
            ));
        }
    }

    let score_label = if score >= 75.0 {
        "отличный день"
    } else if score >= 50.0 {
        "продуктивный день"
    } else if score >= 25.0 {
        "средний день"
    } else {
        "низкая продуктивность"
    };
    parts.push(format!("Оценка: {:.0} — {score_label}", score));

    parts.join(". ")
}
