use chrono::NaiveDate;
use sqlx::SqlitePool;

use crate::analytics::aggregator::{
    aggregate_by_app, aggregate_by_category, build_day_summary, format_duration,
};
use crate::analytics::config::AnalyticsConfig;
use crate::analytics::focus::{detect_deep_work, detect_focus_sessions};
use crate::analytics::models::{AppBreakdown, DayReport, WeekReport};
use crate::analytics::scorer::{FocusScore, Scorer};
use crate::analytics::switch_detector::analyze_context_switching;
use crate::db::models::WeeklySummary;
use crate::db::queries::get_sessions_for_date;
use crate::error::AppError;
use crate::events::types::SessionInfo;

pub struct AnalyticsEngine {
    pool: SqlitePool,
    config: AnalyticsConfig,
}

impl AnalyticsEngine {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            config: AnalyticsConfig::default(),
        }
    }

    #[allow(dead_code)]
    pub fn with_config(pool: SqlitePool, config: AnalyticsConfig) -> Self {
        Self { pool, config }
    }

    // -----------------------------------------------------------------------
    // Backward-compatible API (existing Tauri commands continue to work)
    // -----------------------------------------------------------------------

    pub async fn focus_score_for_date(&self, date: NaiveDate) -> Result<f32, AppError> {
        let sessions = get_sessions_for_date(&self.pool, date).await?;
        Ok(FocusScore::calculate(&sessions))
    }

    pub async fn weekly_summary(&self) -> Result<WeeklySummary, AppError> {
        let today = chrono::Utc::now().date_naive();
        let mut days = Vec::with_capacity(7);
        let mut scores = Vec::with_capacity(7);
        let mut total_active_ms: i64 = 0;

        for i in 0..7i64 {
            let date = today - chrono::Duration::days(i);
            let sessions = get_sessions_for_date(&self.pool, date).await?;
            let score = FocusScore::calculate(&sessions);
            let day_ms: i64 = sessions.iter().map(|s| s.duration_ms).sum();
            days.push(date.to_string());
            scores.push(score);
            total_active_ms += day_ms;
        }

        let avg_score = if scores.is_empty() {
            0.0
        } else {
            scores.iter().sum::<f32>() / scores.len() as f32
        };

        Ok(WeeklySummary {
            days,
            scores,
            total_active_ms,
            avg_score,
        })
    }

    // -----------------------------------------------------------------------
    // Rich API
    // -----------------------------------------------------------------------

    /// Full per-day analytics report.  CPU-bound work runs off the async executor.
    pub async fn day_report(&self, date: NaiveDate) -> Result<DayReport, AppError> {
        let sessions = get_sessions_for_date(&self.pool, date).await?;
        let config = self.config.clone();
        tokio::task::spawn_blocking(move || build_day_report(date, sessions, &config))
            .await
            .map_err(|e| AppError::TrackingError(format!("analytics join error: {e}")))
    }

    /// Parallel per-day reports for a 7-day window ending on `end_date`.
    pub async fn week_report(&self, end_date: NaiveDate) -> Result<WeekReport, AppError> {
        let start_date = end_date - chrono::Duration::days(6);

        // Sequential I/O — DB reads are cheap, avoid connection thrashing
        let mut day_data: Vec<(NaiveDate, Vec<SessionInfo>)> = Vec::with_capacity(7);
        let mut d = start_date;
        while d <= end_date {
            let sessions = get_sessions_for_date(&self.pool, d).await?;
            day_data.push((d, sessions));
            d += chrono::Duration::days(1);
        }

        let config = self.config.clone();

        // CPU-bound analytics — rayon parallel across 7 days
        let reports: Vec<DayReport> = tokio::task::spawn_blocking(move || {
            use rayon::prelude::*;
            day_data
                .into_par_iter()
                .map(|(date, sessions)| build_day_report(date, sessions, &config))
                .collect()
        })
        .await
        .map_err(|e| AppError::TrackingError(format!("analytics join error: {e}")))?;

        let total_ms: i64 = reports.iter().map(|r| r.total_ms).sum();
        let productive_ms: i64 = reports.iter().map(|r| r.productive_ms).sum();
        let avg_score = if reports.is_empty() {
            0.0
        } else {
            reports.iter().map(|r| r.score.total).sum::<f64>() / reports.len() as f64
        };

        let top_apps = merge_top_apps(&reports, total_ms);
        let summary = build_week_summary(total_ms, productive_ms, &top_apps, avg_score);

        Ok(WeekReport {
            start_date,
            end_date,
            days: reports,
            avg_score,
            total_ms,
            productive_ms,
            top_apps,
            summary,
        })
    }
}

// ---------------------------------------------------------------------------
// Pure computation helpers (no async, called from spawn_blocking / rayon)
// ---------------------------------------------------------------------------

fn build_day_report(
    date: NaiveDate,
    sessions: Vec<SessionInfo>,
    config: &AnalyticsConfig,
) -> DayReport {
    let total_ms: i64 = sessions.iter().map(|s| s.duration_ms).sum();
    let productive_ms: i64 = sessions
        .iter()
        .filter(|s| s.category == "Productive")
        .map(|s| s.duration_ms)
        .sum();
    let distraction_ms: i64 = sessions
        .iter()
        .filter(|s| s.category == "Distraction")
        .map(|s| s.duration_ms)
        .sum();
    let neutral_ms = total_ms
        .saturating_sub(productive_ms)
        .saturating_sub(distraction_ms);

    let score = Scorer::new(config).score(&sessions);
    let focus_sessions = detect_focus_sessions(&sessions, &config.focus);
    let deep_work = detect_deep_work(&sessions, &config.focus);
    let context_switches = analyze_context_switching(&sessions, total_ms, &config.switching);
    let app_breakdown = aggregate_by_app(&sessions, total_ms);
    let category_breakdown = aggregate_by_category(&app_breakdown, total_ms);
    let summary = build_day_summary(
        total_ms,
        productive_ms,
        &app_breakdown,
        &focus_sessions,
        score.total,
    );

    DayReport {
        date,
        total_ms,
        productive_ms,
        distraction_ms,
        neutral_ms,
        session_count: sessions.len(),
        score,
        focus_sessions,
        deep_work,
        context_switches,
        app_breakdown,
        category_breakdown,
        summary,
    }
}

fn merge_top_apps(reports: &[DayReport], total_ms: i64) -> Vec<AppBreakdown> {
    use std::collections::HashMap;
    let mut map: HashMap<String, AppBreakdown> = HashMap::new();
    for r in reports {
        for app in &r.app_breakdown {
            let e = map.entry(app.exe.clone()).or_insert_with(|| AppBreakdown {
                exe: app.exe.clone(),
                display_name: app.display_name.clone(),
                category: app.category.clone(),
                total_ms: 0,
                session_count: 0,
                percentage: 0.0,
                formatted: String::new(),
            });
            e.total_ms += app.total_ms;
            e.session_count += app.session_count;
        }
    }

    let mut result: Vec<AppBreakdown> = map
        .into_values()
        .map(|mut a| {
            a.percentage = if total_ms > 0 {
                a.total_ms as f64 / total_ms as f64 * 100.0
            } else {
                0.0
            };
            a.formatted = format_duration(a.total_ms);
            a
        })
        .collect();

    result.sort_by_key(|b| std::cmp::Reverse(b.total_ms));
    result.truncate(10);
    result
}

fn build_week_summary(
    total_ms: i64,
    productive_ms: i64,
    top_apps: &[AppBreakdown],
    avg_score: f64,
) -> String {
    if total_ms == 0 {
        return "Нет данных за эту неделю".into();
    }

    let mut parts: Vec<String> = Vec::new();
    parts.push(format!("Всего активности: {}", format_duration(total_ms)));
    if productive_ms > 0 {
        parts.push(format!("Продуктивно: {}", format_duration(productive_ms)));
    }
    if let Some(top) = top_apps.first() {
        parts.push(format!(
            "Топ приложение: {} ({})",
            top.display_name, top.formatted
        ));
    }
    parts.push(format!("Средняя оценка: {avg_score:.0}"));
    parts.join(". ")
}
