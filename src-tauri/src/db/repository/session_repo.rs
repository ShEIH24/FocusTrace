#![allow(dead_code)]

use chrono::{Duration, NaiveDate, Utc};
use sqlx::SqlitePool;

use crate::db::models::{AppUsageDetail, AppUsageStat, DailyMetricsInput, HourlyStats};

#[derive(sqlx::FromRow)]
struct MetricsRow {
    total_ms: i64,
    productive_ms: i64,
    distraction_ms: i64,
    neutral_ms: i64,
    session_count: i64,
}
use crate::error::AppError;
use crate::events::types::SessionInfo;

// ---------------------------------------------------------------------------
// Input type
// ---------------------------------------------------------------------------

/// Data required to persist one focus session.
/// The caller is responsible for resolving `app_id` via `AppRepository::upsert`.
#[derive(Debug, Clone)]
pub struct NewSession {
    pub app_id: i64,
    pub title: String,
    pub started_at: chrono::DateTime<Utc>,
    pub duration_ms: i64,
}

// ---------------------------------------------------------------------------
// Repository
// ---------------------------------------------------------------------------

/// Read / write operations on `activity_sessions`.
pub struct SessionRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SessionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    // -----------------------------------------------------------------------
    // Writes
    // -----------------------------------------------------------------------

    /// Inserts a single session.  `ended_at` is derived as
    /// `started_at + duration_ms`.
    pub async fn insert(&self, s: &NewSession) -> Result<(), AppError> {
        let ended_at = s.started_at + Duration::milliseconds(s.duration_ms);
        sqlx::query(
            "INSERT INTO activity_sessions (app_id, title, started_at, ended_at, duration_ms)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(s.app_id)
        .bind(&s.title)
        .bind(s.started_at.to_rfc3339())
        .bind(ended_at.to_rfc3339())
        .bind(s.duration_ms)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Inserts multiple sessions in a single transaction.
    /// Returns immediately (no-op) if the slice is empty.
    pub async fn insert_batch(&self, sessions: &[NewSession]) -> Result<(), AppError> {
        if sessions.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for s in sessions {
            let ended_at = s.started_at + Duration::milliseconds(s.duration_ms);
            sqlx::query(
                "INSERT INTO activity_sessions
                     (app_id, title, started_at, ended_at, duration_ms)
                 VALUES (?, ?, ?, ?, ?)",
            )
            .bind(s.app_id)
            .bind(&s.title)
            .bind(s.started_at.to_rfc3339())
            .bind(ended_at.to_rfc3339())
            .bind(s.duration_ms)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Reads — backward-compatible with existing commands
    // -----------------------------------------------------------------------

    /// Returns all sessions for `date` (UTC) mapped to `SessionInfo` so the
    /// existing analytics engine and Tauri commands work without changes.
    pub async fn find_by_date(&self, date: NaiveDate) -> Result<Vec<SessionInfo>, AppError> {
        let (start, end) = day_bounds(date);

        let rows = sqlx::query_as::<_, SessionInfo>(
            "SELECT a.exe, s.title, s.started_at, s.duration_ms,
                    c.name AS category, a.exe_path
             FROM activity_sessions s
             JOIN apps       a ON a.id = s.app_id
             JOIN categories c ON c.id = a.category_id
             WHERE s.started_at >= ? AND s.started_at < ?
             ORDER BY s.started_at",
        )
        .bind(&start)
        .bind(&end)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Per-app usage totals for a single day — same shape as the legacy
    /// `AppUsageStat` used by `commands::activity::get_app_usage`.
    pub async fn usage_by_app_for_date(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<AppUsageStat>, AppError> {
        let (start, end) = day_bounds(date);

        let rows = sqlx::query_as::<_, AppUsageStat>(
            "SELECT a.exe, SUM(s.duration_ms) AS total_ms,
                    c.name AS category, COUNT(*) AS session_count
             FROM activity_sessions s
             JOIN apps       a ON a.id = s.app_id
             JOIN categories c ON c.id = a.category_id
             WHERE s.started_at >= ? AND s.started_at < ?
             GROUP BY a.id
             ORDER BY total_ms DESC",
        )
        .bind(&start)
        .bind(&end)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    // -----------------------------------------------------------------------
    // Analytics queries
    // -----------------------------------------------------------------------

    /// Richer per-app breakdown including exe_path and display_name.
    pub async fn usage_detail_for_date(
        &self,
        date: NaiveDate,
    ) -> Result<Vec<AppUsageDetail>, AppError> {
        let (start, end) = day_bounds(date);

        let rows = sqlx::query_as::<_, AppUsageDetail>(
            "SELECT a.exe, a.display_name, c.name AS category,
                    SUM(s.duration_ms) AS total_ms,
                    COUNT(*) AS session_count, a.exe_path
             FROM activity_sessions s
             JOIN apps       a ON a.id = s.app_id
             JOIN categories c ON c.id = a.category_id
             WHERE s.started_at >= ? AND s.started_at < ?
             GROUP BY a.id
             ORDER BY total_ms DESC",
        )
        .bind(&start)
        .bind(&end)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Top `limit` apps over a date range — used by the weekly analytics view.
    pub async fn top_apps(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        limit: i64,
    ) -> Result<Vec<AppUsageDetail>, AppError> {
        let start = format!("{start_date}T00:00:00+00:00");
        let end = format!("{end_date}T00:00:00+00:00");

        let rows = sqlx::query_as::<_, AppUsageDetail>(
            "SELECT a.exe, a.display_name, c.name AS category,
                    SUM(s.duration_ms) AS total_ms,
                    COUNT(*) AS session_count, a.exe_path
             FROM activity_sessions s
             JOIN apps       a ON a.id = s.app_id
             JOIN categories c ON c.id = a.category_id
             WHERE s.started_at >= ? AND s.started_at < ?
             GROUP BY a.id
             ORDER BY total_ms DESC
             LIMIT ?",
        )
        .bind(&start)
        .bind(&end)
        .bind(limit)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Breaks a day into hourly buckets (0–23).
    /// Hours with no activity are omitted (sparse result set).
    pub async fn hourly_breakdown(&self, date: NaiveDate) -> Result<Vec<HourlyStats>, AppError> {
        let (start, end) = day_bounds(date);

        let rows = sqlx::query_as::<_, HourlyStats>(
            "SELECT
                 CAST(strftime('%H', started_at) AS INTEGER) AS hour,
                 SUM(duration_ms)  AS total_ms,
                 SUM(CASE WHEN c.is_productive = 1 THEN duration_ms ELSE 0 END) AS productive_ms,
                 COUNT(*)          AS session_count
             FROM activity_sessions s
             JOIN apps       a ON a.id = s.app_id
             JOIN categories c ON c.id = a.category_id
             WHERE s.started_at >= ? AND s.started_at < ?
             GROUP BY hour
             ORDER BY hour",
        )
        .bind(&start)
        .bind(&end)
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Computes aggregate productivity metrics for `date`.
    /// Used by `MetricsRepository::refresh` to update the cache.
    pub async fn compute_metrics(&self, date: NaiveDate) -> Result<DailyMetricsInput, AppError> {
        let (start, end) = day_bounds(date);

        // Aggregate totals in a single pass.
        let row = sqlx::query_as::<_, MetricsRow>(
            "SELECT
                 COALESCE(SUM(s.duration_ms), 0)                                          AS total_ms,
                 COALESCE(SUM(CASE WHEN c.is_productive = 1 THEN s.duration_ms ELSE 0 END), 0)
                                                                                          AS productive_ms,
                 COALESCE(SUM(CASE WHEN c.name = 'Distraction' THEN s.duration_ms ELSE 0 END), 0)
                                                                                          AS distraction_ms,
                 COALESCE(SUM(CASE WHEN c.name = 'Neutral' THEN s.duration_ms ELSE 0 END), 0)
                                                                                          AS neutral_ms,
                 COALESCE(COUNT(*), 0)                                                    AS session_count
             FROM activity_sessions s
             JOIN apps       a ON a.id = s.app_id
             JOIN categories c ON c.id = a.category_id
             WHERE s.started_at >= ? AND s.started_at < ?",
        )
        .bind(&start)
        .bind(&end)
        .fetch_one(self.pool)
        .await?;

        let total_ms = row.total_ms;
        let productive_ms = row.productive_ms;
        let distraction_ms = row.distraction_ms;
        let neutral_ms = row.neutral_ms;
        let session_count = row.session_count;

        let focus_score = compute_focus_score(total_ms, productive_ms, session_count);

        Ok(DailyMetricsInput {
            total_ms,
            productive_ms,
            distraction_ms,
            neutral_ms,
            focus_score,
            session_count,
            switch_count: session_count,
        })
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns `(start, exclusive_end)` RFC3339 strings for a UTC calendar day.
/// Using `[start, end)` avoids the midnight-rounding edge case in the legacy
/// `[00:00:00, 23:59:59]` approach.
pub(crate) fn day_bounds(date: NaiveDate) -> (String, String) {
    let next = date + Duration::days(1);
    (
        format!("{date}T00:00:00+00:00"),
        format!("{next}T00:00:00+00:00"),
    )
}

/// Focus score 0–100: ratio of productive time with a context-switch penalty.
fn compute_focus_score(total_ms: i64, productive_ms: i64, session_count: i64) -> f64 {
    if total_ms == 0 {
        return 0.0;
    }
    let mut score = productive_ms as f64 / total_ms as f64 * 100.0;
    let total_hours = total_ms as f64 / 3_600_000.0;
    if total_hours > 0.0 && session_count as f64 / total_hours > 30.0 {
        score *= 0.9;
    }
    score.clamp(0.0, 100.0)
}

// ---------------------------------------------------------------------------
// Unit tests (pure functions only — no DB needed)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_score_zero_for_no_activity() {
        assert_eq!(compute_focus_score(0, 0, 0), 0.0);
    }

    #[test]
    fn focus_score_full_productive() {
        let score = compute_focus_score(3_600_000, 3_600_000, 10);
        assert!((score - 100.0).abs() < 0.01, "expected ~100, got {score}");
    }

    #[test]
    fn focus_score_penalizes_high_switch_rate() {
        // 31 switches in 1 hour → penalty
        let penalized = compute_focus_score(3_600_000, 3_600_000, 31);
        let not_penalized = compute_focus_score(3_600_000, 3_600_000, 10);
        assert!(
            penalized < not_penalized,
            "high switch rate should reduce score"
        );
        assert!((penalized - 90.0).abs() < 0.01);
    }

    #[test]
    fn day_bounds_uses_exclusive_end() {
        let (start, end) = day_bounds(NaiveDate::from_ymd_opt(2024, 1, 31).expect("valid date"));
        assert_eq!(start, "2024-01-31T00:00:00+00:00");
        assert_eq!(end, "2024-02-01T00:00:00+00:00");
    }
}
