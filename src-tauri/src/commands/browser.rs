use chrono::Utc;
use tauri::State;

use crate::browser::{BrowserEventRow, DomainStat, UserCategoryRule};
use crate::error::AppError;
use crate::state::AppState;

/// The WS token the user pastes into the browser extension popup.
#[tauri::command]
pub async fn get_browser_ws_token(state: State<'_, AppState>) -> Result<String, AppError> {
    Ok(state.browser.ws_token.clone())
}

/// Current active browser tab, if any.
#[tauri::command]
pub async fn get_current_browser_tab(
    state: State<'_, AppState>,
) -> Result<Option<crate::browser::TabInfo>, AppError> {
    Ok(crate::browser::get_current_tab(&state.browser).await)
}

/// All browser events for a given date (YYYY-MM-DD), newest first.
#[tauri::command]
pub async fn get_browser_history(
    date: String,
    state: State<'_, AppState>,
) -> Result<Vec<BrowserEventRow>, AppError> {
    let start = format!("{date}T00:00:00+00:00");
    let end = format!("{date}T23:59:59+00:00");

    let rows = sqlx::query_as::<_, (i64, String, i64, String, String, String, String, String, String)>(
        "SELECT id, started_at, duration_ms, browser, url, domain, title, category, subcategory
         FROM browser_events
         WHERE started_at >= ? AND started_at <= ?
         ORDER BY started_at DESC",
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(&state.pool)
    .await?;

    let result = rows
        .into_iter()
        .map(|(id, started_at, duration_ms, browser, url, domain, title, category, subcategory)| {
            BrowserEventRow { id, started_at, duration_ms, browser, url, domain, title, category, subcategory }
        })
        .collect();

    Ok(result)
}

/// Aggregate per-domain stats for a given date.
#[tauri::command]
pub async fn get_browser_domain_stats(
    date: String,
    state: State<'_, AppState>,
) -> Result<Vec<DomainStat>, AppError> {
    let start = format!("{date}T00:00:00+00:00");
    let end = format!("{date}T23:59:59+00:00");

    let rows = sqlx::query_as::<_, (String, String, String, i64, i64)>(
        "SELECT domain, category, subcategory,
                SUM(duration_ms) AS total_ms,
                COUNT(*)         AS visits
         FROM browser_events
         WHERE started_at >= ? AND started_at <= ?
         GROUP BY domain
         ORDER BY total_ms DESC",
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(&state.pool)
    .await?;

    let result = rows
        .into_iter()
        .map(|(domain, category, subcategory, duration_ms, visits)| DomainStat {
            domain,
            category,
            subcategory,
            duration_ms,
            visits,
        })
        .collect();

    Ok(result)
}

/// Add or update a user-defined domain → category rule.
#[tauri::command]
pub async fn add_browser_category_rule(
    domain_pattern: String,
    category: String,
    subcategory: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO browser_category_rules (domain_pattern, category, subcategory)
         VALUES (?, ?, ?)
         ON CONFLICT(domain_pattern) DO UPDATE
         SET category = excluded.category, subcategory = excluded.subcategory",
    )
    .bind(&domain_pattern)
    .bind(&category)
    .bind(&subcategory)
    .execute(&state.pool)
    .await?;

    // Update live engine.
    state
        .browser
        .category_engine
        .lock()
        .await
        .add_user_rule(domain_pattern, category, subcategory);

    Ok(())
}

/// Remove a user-defined rule by domain pattern.
#[tauri::command]
pub async fn remove_browser_category_rule(
    domain_pattern: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM browser_category_rules WHERE domain_pattern = ?")
        .bind(&domain_pattern)
        .execute(&state.pool)
        .await?;
    Ok(())
}

/// List all user-defined category rules.
#[tauri::command]
pub async fn get_browser_category_rules(
    state: State<'_, AppState>,
) -> Result<Vec<UserCategoryRule>, AppError> {
    let rows = sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, domain_pattern, category, subcategory
         FROM browser_category_rules ORDER BY domain_pattern",
    )
    .fetch_all(&state.pool)
    .await?;

    let result = rows
        .into_iter()
        .map(|(id, domain_pattern, category, subcategory)| UserCategoryRule {
            id,
            domain_pattern,
            category,
            subcategory,
        })
        .collect();

    Ok(result)
}

/// Quick URL categorization — useful for Settings preview.
#[tauri::command]
pub async fn categorize_url(
    url: String,
    state: State<'_, AppState>,
) -> Result<(String, String), AppError> {
    let domain = crate::browser::category::extract_domain(&url)
        .unwrap_or_else(|| url.clone());
    let cat = state.browser.category_engine.lock().await.categorize(&domain);
    Ok((cat.category, cat.subcategory))
}

/// Today's browser time aggregated by category.
#[tauri::command]
pub async fn get_browser_today_summary(
    state: State<'_, AppState>,
) -> Result<Vec<(String, i64)>, AppError> {
    let today = Utc::now().date_naive().to_string();
    let start = format!("{today}T00:00:00+00:00");

    let rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT category, SUM(duration_ms)
         FROM browser_events WHERE started_at >= ?
         GROUP BY category ORDER BY SUM(duration_ms) DESC",
    )
    .bind(&start)
    .fetch_all(&state.pool)
    .await?;

    Ok(rows)
}
