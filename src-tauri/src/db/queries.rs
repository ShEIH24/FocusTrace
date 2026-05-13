use chrono::NaiveDate;
use sqlx::SqlitePool;

use crate::db::models::AppUsageStat;
use crate::db::repository::session_repo::day_bounds;
use crate::error::AppError;
use crate::events::types::SessionInfo;

/// Returns all sessions for `date` (UTC), joined with `apps` and `categories`.
///
/// Used by the analytics engine and `commands::activity::get_activity_log`.
pub async fn get_sessions_for_date(
    pool: &SqlitePool,
    date: NaiveDate,
) -> Result<Vec<SessionInfo>, AppError> {
    let (start, end) = day_bounds(date);

    let rows = sqlx::query_as::<_, SessionInfo>(
        "SELECT a.exe, s.title, s.started_at, s.duration_ms,
                c.name AS category, a.exe_path
         FROM activity_sessions s
         JOIN apps       a ON a.id = s.app_id
         JOIN categories c ON c.id = a.category_id
         WHERE s.started_at >= ? AND s.started_at < ?
         ORDER BY s.started_at DESC",
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Per-app usage aggregates for `date`.
///
/// Used by `commands::activity::get_app_usage`.
pub async fn get_app_usage_stats(
    pool: &SqlitePool,
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
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
