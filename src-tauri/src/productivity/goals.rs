use chrono::{Duration, Utc};
use sqlx::SqlitePool;

use crate::db::repository::MetricsRepository;
use crate::error::AppError;
use crate::productivity::models::{GoalConfig, GoalProgress, StreakInfo};

/// Computes today's goal progress by reading current metrics + pomodoro counts.
pub async fn compute_goal_progress(
    pool: &SqlitePool,
    goals: &GoalConfig,
) -> Result<GoalProgress, AppError> {
    let today = Utc::now().date_naive();
    let today_str = today.to_string();

    // Fetch today's productivity metrics.
    let metrics_repo = MetricsRepository::new(pool);
    let metrics = metrics_repo.get(today).await?;

    let productive_actual_ms = metrics.as_ref().map_or(0, |m| m.productive_ms);
    let score_actual = metrics.as_ref().map_or(0.0, |m| m.focus_score);

    // Count completed work pomodoros today.
    let start_of_day = format!("{today_str}T00:00:00+00:00");
    let pomodoro_actual: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM pomodoro_sessions
         WHERE phase = 'work' AND completed = 1 AND started_at >= ?",
    )
    .bind(&start_of_day)
    .fetch_one(pool)
    .await?;

    let productive_target_ms = goals.productive_target_mins as i64 * 60_000;
    let pomodoro_target = goals.pomodoro_target;

    let productive_pct = if productive_target_ms > 0 {
        (productive_actual_ms as f64 / productive_target_ms as f64 * 100.0).min(100.0)
    } else {
        0.0
    };

    let pomodoro_pct = if pomodoro_target > 0 {
        (pomodoro_actual as f64 / pomodoro_target as f64 * 100.0).min(100.0)
    } else {
        0.0
    };

    let score_pct = if goals.score_target > 0.0 {
        (score_actual / goals.score_target * 100.0).min(100.0)
    } else {
        0.0
    };

    // Goal is "met" if all enabled targets are >= 100%.
    let checks: Vec<f64> = [
        (productive_target_ms > 0).then_some(productive_pct),
        (pomodoro_target > 0).then_some(pomodoro_pct),
        (goals.score_target > 0.0).then_some(score_pct),
    ]
    .into_iter()
    .flatten()
    .collect();

    let overall_met = !checks.is_empty() && checks.iter().all(|&p| p >= 100.0);

    Ok(GoalProgress {
        date: today_str,
        productive_target_ms,
        productive_actual_ms,
        productive_pct,
        pomodoro_target,
        pomodoro_actual: pomodoro_actual as u32,
        pomodoro_pct,
        score_target: goals.score_target,
        score_actual,
        score_pct,
        overall_met,
    })
}

/// Computes the current and longest productivity streaks.
///
/// A day counts toward the streak when focus_score >= 40 (minimum baseline)
/// OR when productive_ms >= the goal (if a productive time goal is set).
pub async fn compute_streak(
    pool: &SqlitePool,
    productive_target_ms: i64,
) -> Result<StreakInfo, AppError> {
    let today = Utc::now().date_naive();

    // Fetch last 60 days of metrics to find the longest streak.
    let start = (today - Duration::days(59)).to_string();
    let rows = sqlx::query_as::<_, (String, f64, i64)>(
        "SELECT date, focus_score, productive_ms
         FROM productivity_metrics
         WHERE date >= ?
         ORDER BY date ASC",
    )
    .bind(&start)
    .fetch_all(pool)
    .await?;

    // Build a set of "productive" dates.
    let productive_dates: std::collections::HashSet<String> = rows
        .iter()
        .filter(|(_, score, prod_ms)| {
            *score >= 40.0 || (productive_target_ms > 0 && *prod_ms >= productive_target_ms)
        })
        .map(|(date, _, _)| date.clone())
        .collect();

    // Compute current streak (from today backwards).
    let mut current = 0u32;
    let mut d = today;
    loop {
        if productive_dates.contains(&d.to_string()) {
            current += 1;
            d = d - Duration::days(1);
        } else {
            break;
        }
    }

    // Compute longest streak across all 60 days.
    let mut longest = current;
    let mut run = 0u32;
    let mut day_iter = today - Duration::days(59);
    while day_iter <= today {
        if productive_dates.contains(&day_iter.to_string()) {
            run += 1;
            longest = longest.max(run);
        } else {
            run = 0;
        }
        day_iter = day_iter + Duration::days(1);
    }

    // Last 7 days (oldest first).
    let last_7_days: Vec<bool> = (0..7i64)
        .rev()
        .map(|i| {
            let date = (today - Duration::days(i)).to_string();
            productive_dates.contains(&date)
        })
        .collect();

    let last_productive_date = rows
        .iter()
        .rev()
        .find(|(_, score, prod_ms)| {
            *score >= 40.0 || (productive_target_ms > 0 && *prod_ms >= productive_target_ms)
        })
        .map(|(date, _, _)| date.clone());

    Ok(StreakInfo {
        current,
        longest,
        last_7_days,
        last_productive_date,
    })
}
