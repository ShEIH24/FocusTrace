use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{debug, info, warn};

use crate::browser::category::extract_domain;
use crate::browser::{BrowserState, TabInfo};

// ---------------------------------------------------------------------------
// Wire protocol types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum InboundMsg {
    Auth {
        token: String,
    },
    TabUpdate {
        browser: String,
        url: String,
        title: String,
    },
    TabDeactivated {
        browser: String,
    },
    Ping,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum OutboundMsg {
    AuthOk { version: u32 },
    AuthErr { reason: &'static str },
    Pong,
}

// ---------------------------------------------------------------------------
// Server entry point
// ---------------------------------------------------------------------------

pub async fn run(app: AppHandle, pool: SqlitePool, browser_state: Arc<BrowserState>) {
    let addr: SocketAddr = "127.0.0.1:9919".parse().expect("valid address");

    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            warn!("Browser WS server failed to bind {addr}: {e}");
            return;
        }
    };

    info!("Browser WS server listening on {addr}");

    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                debug!("Browser extension connected from {peer}");
                let app2 = app.clone();
                let pool2 = pool.clone();
                let state2 = browser_state.clone();
                tokio::spawn(handle_connection(stream, app2, pool2, state2));
            }
            Err(e) => {
                warn!("Browser WS accept error: {e}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Per-connection handler
// ---------------------------------------------------------------------------

async fn handle_connection(
    stream: TcpStream,
    app: AppHandle,
    pool: SqlitePool,
    state: Arc<BrowserState>,
) {
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            warn!("WS handshake failed: {e}");
            return;
        }
    };

    let (mut tx, mut rx) = ws.split();

    // ── Auth handshake (5-second window) ────────────────────────────────────
    let first_msg = match timeout(Duration::from_secs(5), rx.next()).await {
        Ok(Some(Ok(msg))) => msg,
        _ => {
            warn!("Browser WS: auth timeout or error");
            return;
        }
    };

    let authed = match first_msg {
        Message::Text(text) => match serde_json::from_str::<InboundMsg>(&text) {
            Ok(InboundMsg::Auth { token }) if token == state.ws_token => true,
            _ => false,
        },
        _ => false,
    };

    if !authed {
        let resp = serde_json::to_string(&OutboundMsg::AuthErr {
            reason: "invalid token",
        })
        .unwrap_or_default();
        let _ = tx.send(Message::Text(resp.into())).await;
        return;
    }

    let resp = serde_json::to_string(&OutboundMsg::AuthOk { version: 1 }).unwrap_or_default();
    if tx.send(Message::Text(resp.into())).await.is_err() {
        return;
    }

    // ── Message loop ─────────────────────────────────────────────────────────
    let last_tab: Arc<Mutex<Option<(TabInfo, chrono::DateTime<Utc>)>>> = Arc::new(Mutex::new(None));

    while let Some(msg_result) = rx.next().await {
        let msg = match msg_result {
            Ok(m) => m,
            Err(_) => break,
        };

        match msg {
            Message::Text(text) => match serde_json::from_str::<InboundMsg>(&text) {
                Ok(InboundMsg::TabUpdate {
                    browser,
                    url,
                    title,
                }) => {
                    on_tab_update(&url, &title, &browser, &last_tab, &state, &app, &pool).await;
                }
                Ok(InboundMsg::TabDeactivated { browser }) => {
                    on_tab_deactivated(&browser, &last_tab, &state, &pool).await;
                }
                Ok(InboundMsg::Ping) => {
                    let pong = serde_json::to_string(&OutboundMsg::Pong).unwrap_or_default();
                    let _ = tx.send(Message::Text(pong.into())).await;
                }
                _ => {}
            },
            Message::Close(_) => break,
            Message::Ping(data) => {
                let _ = tx.send(Message::Pong(data)).await;
            }
            _ => {}
        }
    }

    // Persist the last in-progress tab visit on disconnect.
    on_tab_deactivated("", &last_tab, &state, &pool).await;
    debug!("Browser WS connection closed");
}

// ---------------------------------------------------------------------------
// Event handlers
// ---------------------------------------------------------------------------

async fn on_tab_update(
    url: &str,
    title: &str,
    browser: &str,
    last_tab: &Arc<Mutex<Option<(TabInfo, chrono::DateTime<Utc>)>>>,
    state: &Arc<BrowserState>,
    app: &AppHandle,
    pool: &SqlitePool,
) {
    let domain = match extract_domain(url) {
        Some(d) => d,
        None => return,
    };

    // Skip browser internal pages.
    if url.starts_with("chrome://")
        || url.starts_with("edge://")
        || url.starts_with("about:")
        || url.starts_with("moz-extension://")
        || url.starts_with("chrome-extension://")
    {
        return;
    }

    let cat = state.category_engine.lock().await.categorize(&domain);
    let now = Utc::now();

    let new_tab = TabInfo {
        browser: browser.to_string(),
        url: url.to_string(),
        domain: domain.clone(),
        title: title.to_string(),
        category: cat.category.clone(),
        subcategory: cat.subcategory.clone(),
        timestamp: now,
    };

    // Persist the previous tab visit.
    let mut lock = last_tab.lock().await;
    if let Some((prev, prev_ts)) = lock.take() {
        let duration_ms = (now - prev_ts).num_milliseconds().max(0) as i64;
        if duration_ms > 500 {
            persist_event(pool, &prev, prev_ts, now, duration_ms).await;
        }
    }
    *lock = Some((new_tab.clone(), now));
    drop(lock);

    // Update shared current tab.
    *state.current_tab.write().await = Some(new_tab.clone());

    // Emit frontend event.
    let _ = app.emit("browser-tab-updated", &new_tab);
}

async fn on_tab_deactivated(
    _browser: &str,
    last_tab: &Arc<Mutex<Option<(TabInfo, chrono::DateTime<Utc>)>>>,
    state: &Arc<BrowserState>,
    pool: &SqlitePool,
) {
    let mut lock = last_tab.lock().await;
    if let Some((tab, ts)) = lock.take() {
        let now = Utc::now();
        let duration_ms = (now - ts).num_milliseconds().max(0) as i64;
        if duration_ms > 500 {
            persist_event(pool, &tab, ts, now, duration_ms).await;
        }
    }
    drop(lock);

    // Clear shared state — browser is no longer the focused window.
    *state.current_tab.write().await = None;
}

// ---------------------------------------------------------------------------
// DB persistence
// ---------------------------------------------------------------------------

async fn persist_event(
    pool: &SqlitePool,
    tab: &TabInfo,
    started_at: chrono::DateTime<Utc>,
    ended_at: chrono::DateTime<Utc>,
    duration_ms: i64,
) {
    let _ = sqlx::query(
        "INSERT INTO browser_events
             (started_at, ended_at, duration_ms, browser, url, domain, title, category, subcategory)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(started_at.to_rfc3339())
    .bind(ended_at.to_rfc3339())
    .bind(duration_ms)
    .bind(&tab.browser)
    .bind(&tab.url)
    .bind(&tab.domain)
    .bind(&tab.title)
    .bind(&tab.category)
    .bind(&tab.subcategory)
    .execute(pool)
    .await;
}
