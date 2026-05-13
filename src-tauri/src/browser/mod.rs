pub mod category;
pub mod ws_server;

use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::AppHandle;
use tokio::sync::RwLock;
use tracing::debug;

use tokio::sync::Mutex as AsyncMutex;

use crate::browser::category::CategoryEngine;
use crate::db::repository::SettingsRepository;

const TOKEN_KEY: &str = "browser:ws_token";

// ---------------------------------------------------------------------------
// Shared state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabInfo {
    pub browser: String,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub category: String,
    pub subcategory: String,
    #[serde(skip)]
    #[allow(dead_code)]
    pub timestamp: DateTime<Utc>,
}

pub struct BrowserState {
    pub current_tab: RwLock<Option<TabInfo>>,
    pub ws_token: String,
    pub category_engine: AsyncMutex<CategoryEngine>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Load (or generate) the WS token, build the category engine with user rules,
/// and return the shared BrowserState + start the WS server task.
pub async fn init(app: AppHandle, pool: SqlitePool) -> Arc<BrowserState> {
    let ws_token = get_or_create_token(&pool).await;

    let mut engine = CategoryEngine::new();
    load_user_rules(&pool, &mut engine).await;

    let state = Arc::new(BrowserState {
        current_tab: RwLock::new(None),
        ws_token,
        category_engine: AsyncMutex::new(engine),
    });

    let state2 = state.clone();
    tauri::async_runtime::spawn(ws_server::run(app, pool, state2));

    state
}

/// Return the current active browser tab, if any.
pub async fn get_current_tab(state: &BrowserState) -> Option<TabInfo> {
    state.current_tab.read().await.clone()
}

// ---------------------------------------------------------------------------
// DB helpers
// ---------------------------------------------------------------------------

async fn get_or_create_token(pool: &SqlitePool) -> String {
    let repo = SettingsRepository::new(pool);
    if let Some(token) = repo.get_string(TOKEN_KEY).await {
        if !token.is_empty() {
            return token;
        }
    }

    let token = generate_token();
    let _ = repo.set_string(TOKEN_KEY, &token).await;
    debug!("Generated new browser WS token");
    token
}

async fn load_user_rules(pool: &SqlitePool, engine: &mut CategoryEngine) {
    let rows = sqlx::query_as::<_, (String, String, String)>(
        "SELECT domain_pattern, category, subcategory FROM browser_category_rules",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    for (pattern, category, subcategory) in rows {
        engine.add_user_rule(pattern, category, subcategory);
    }
}

fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    let pid = std::process::id() as u64;
    // 48 hex chars — plenty of entropy for a local trust boundary.
    format!(
        "{:016x}{:016x}{:016x}",
        ns.wrapping_mul(0x9e3779b97f4a7c15),
        pid.wrapping_mul(0x6c62272e07bb0142),
        (ns ^ pid).wrapping_mul(0xbf58476d1ce4e5b9),
    )
}

// ---------------------------------------------------------------------------
// Payload types for commands
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserEventRow {
    pub id: i64,
    pub started_at: String,
    pub duration_ms: i64,
    pub browser: String,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub category: String,
    pub subcategory: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainStat {
    pub domain: String,
    pub category: String,
    pub subcategory: String,
    pub duration_ms: i64,
    pub visits: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCategoryRule {
    pub id: i64,
    pub domain_pattern: String,
    pub category: String,
    pub subcategory: String,
}
