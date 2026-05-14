#![allow(dead_code)]

use chrono::{Duration, NaiveDate, Utc};
use sqlx::SqlitePool;

use crate::db::models::{DailyMetrics, DailyMetricsInput};
use crate::error::AppError;

/// Read / write operations on `productivity_metrics`.
///
/// The table is a *write-through cache*: computed after each session insert
/// and read back when the frontend requests analytics.  Stale rows are
/// harmless — the `updated_at` field lets callers judge freshness.
pub struct MetricsRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> MetricsRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Inserts or replaces the daily metrics row for `date`.
    pub async fn upsert(&self, date: NaiveDate, m: &DailyMetricsInput) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let date_str = date.to_string();

        sqlx::query(
            "INSERT INTO productivity_metrics
                 (date, total_ms, productive_ms, distraction_ms, neutral_ms,
                  focus_score, session_count, switch_count, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(date) DO UPDATE SET
                 total_ms       = excluded.total_ms,
                 productive_ms  = excluded.productive_ms,
                 distraction_ms = excluded.distraction_ms,
                 neutral_ms     = excluded.neutral_ms,
                 focus_score    = excluded.focus_score,
                 session_count  = excluded.session_count,
                 switch_count   = excluded.switch_count,
                 updated_at     = excluded.updated_at",
        )
        .bind(&date_str)
        .bind(m.total_ms)
        .bind(m.productive_ms)
        .bind(m.distraction_ms)
        .bind(m.neutral_ms)
        .bind(m.focus_score)
        .bind(m.session_count)
        .bind(m.switch_count)
        .bind(&now)
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn get(&self, date: NaiveDate) -> Result<Option<DailyMetrics>, AppError> {
        let row = sqlx::query_as::<_, DailyMetrics>(
            "SELECT date, total_ms, productive_ms, distraction_ms, neutral_ms,
                    focus_score, session_count, switch_count, updated_at
             FROM productivity_metrics
             WHERE date = ?",
        )
        .bind(date.to_string())
        .fetch_optional(self.pool)
        .await?;

        Ok(row)
    }

    /// Returns metrics for every day in `[start_date, end_date]` (inclusive).
    /// Missing days are omitted — callers should treat absence as zero activity.
    pub async fn get_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailyMetrics>, AppError> {
        let rows = sqlx::query_as::<_, DailyMetrics>(
            "SELECT date, total_ms, productive_ms, distraction_ms, neutral_ms,
                    focus_score, session_count, switch_count, updated_at
             FROM productivity_metrics
             WHERE date >= ? AND date <= ?
             ORDER BY date",
        )
        .bind(start_date.to_string())
        .bind(end_date.to_string())
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Deletes metric rows older than `days` days.
    /// Returns the number of rows removed.
    pub async fn delete_older_than(&self, days: u32) -> Result<u64, AppError> {
        let cutoff = (Utc::now() - Duration::days(i64::from(days)))
            .date_naive()
            .to_string();

        let result = sqlx::query("DELETE FROM productivity_metrics WHERE date < ?")
            .bind(&cutoff)
            .execute(self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
