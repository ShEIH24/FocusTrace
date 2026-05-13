// FocusTrace background service worker
// Connects to the local WebSocket server (ws://127.0.0.1:9919) and streams
// active tab updates.  Works in Chrome/Edge (MV3) and Firefox 109+ (MV3).

const WS_URL = "ws://127.0.0.1:9919";
const STORAGE_TOKEN_KEY = "ws_token";
const STORAGE_ENABLED_KEY = "tracking_enabled";
const PING_INTERVAL_SECS = 20;
const RECONNECT_DELAY_MS = 3000;
const ALARM_NAME = "focustrace-keepalive";

let ws = null;
let pingTimer = null;
let reconnectTimer = null;
let isAuthenticated = false;
let lastSentUrl = "";
let lastSentTitle = "";

// ---------------------------------------------------------------------------
// Storage helpers
// ---------------------------------------------------------------------------

async function getToken() {
  const result = await chrome.storage.local.get([STORAGE_TOKEN_KEY, STORAGE_ENABLED_KEY]);
  return {
    token: result[STORAGE_TOKEN_KEY] ?? "",
    enabled: result[STORAGE_ENABLED_KEY] !== false, // default on
  };
}

async function isTrackingEnabled() {
  const { enabled } = await getToken();
  return enabled;
}

// ---------------------------------------------------------------------------
// WebSocket connection
// ---------------------------------------------------------------------------

async function connect() {
  if (ws && (ws.readyState === WebSocket.OPEN || ws.readyState === WebSocket.CONNECTING)) {
    return;
  }

  const { token, enabled } = await getToken();
  if (!enabled || !token) {
    return;
  }

  ws = new WebSocket(WS_URL);

  ws.onopen = async () => {
    // Send auth as the very first message.
    ws.send(JSON.stringify({ type: "auth", token }));
  };

  ws.onmessage = async (event) => {
    let msg;
    try { msg = JSON.parse(event.data); } catch { return; }

    switch (msg.type) {
      case "auth_ok":
        isAuthenticated = true;
        startPing();
        // Send the current active tab immediately after auth.
        await sendCurrentTab();
        break;

      case "auth_err":
        console.warn("FocusTrace: auth rejected —", msg.reason);
        ws.close();
        break;

      case "pong":
        break;
    }
  };

  ws.onclose = () => {
    isAuthenticated = false;
    stopPing();
    scheduleReconnect();
  };

  ws.onerror = (err) => {
    console.debug("FocusTrace WS error:", err);
  };
}

function scheduleReconnect() {
  if (reconnectTimer) return;
  reconnectTimer = setTimeout(async () => {
    reconnectTimer = null;
    await connect();
  }, RECONNECT_DELAY_MS);
}

function disconnect() {
  stopPing();
  if (ws) {
    // Remove handlers before closing so onclose doesn't trigger reconnect.
    ws.onclose = null;
    ws.onerror = null;
    ws.close();
    ws = null;
  }
  isAuthenticated = false;
  lastSentUrl = "";
  lastSentTitle = "";
}

// ---------------------------------------------------------------------------
// Ping / keepalive
// ---------------------------------------------------------------------------

function startPing() {
  stopPing();
  pingTimer = setInterval(() => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify({ type: "ping" }));
    }
  }, PING_INTERVAL_SECS * 1000);
}

function stopPing() {
  if (pingTimer) {
    clearInterval(pingTimer);
    pingTimer = null;
  }
}

// ---------------------------------------------------------------------------
// Tab sending
// ---------------------------------------------------------------------------

async function sendCurrentTab() {
  if (!isAuthenticated || !ws || ws.readyState !== WebSocket.OPEN) return;

  try {
    const [activeTab] = await chrome.tabs.query({ active: true, currentWindow: true });
    if (!activeTab?.url) return;
    await sendTabUpdate(activeTab);
  } catch (_) {}
}

async function sendTabUpdate(tab) {
  if (!isAuthenticated || !ws || ws.readyState !== WebSocket.OPEN) return;

  const url   = tab.url   ?? "";
  const title = tab.title ?? "";

  // Deduplicate — don't flood on rapid redraws.
  if (url === lastSentUrl && title === lastSentTitle) return;

  // Skip internal browser pages.
  if (
    url.startsWith("chrome://")   ||
    url.startsWith("edge://")     ||
    url.startsWith("about:")      ||
    url.startsWith("chrome-extension://") ||
    url.startsWith("moz-extension://")    ||
    url === ""
  ) {
    return;
  }

  lastSentUrl   = url;
  lastSentTitle = title;

  const browser = detectBrowser();
  ws.send(JSON.stringify({ type: "tab_update", browser, url, title }));
}

function sendTabDeactivated() {
  if (!isAuthenticated || !ws || ws.readyState !== WebSocket.OPEN) return;
  lastSentUrl   = "";
  lastSentTitle = "";
  const browser = detectBrowser();
  ws.send(JSON.stringify({ type: "tab_deactivated", browser }));
}

function detectBrowser() {
  const ua = navigator.userAgent;
  if (ua.includes("Edg/"))    return "edge";
  if (ua.includes("Firefox")) return "firefox";
  return "chrome";
}

// ---------------------------------------------------------------------------
// Chrome events
// ---------------------------------------------------------------------------

chrome.tabs.onActivated.addListener(async (info) => {
  if (!await isTrackingEnabled()) return;
  if (!isAuthenticated) { await connect(); return; }
  try {
    const tab = await chrome.tabs.get(info.tabId);
    await sendTabUpdate(tab);
  } catch (_) {}
});

chrome.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
  if (!await isTrackingEnabled()) return;
  // Only fire when the URL or title actually committed (not on every paint).
  if (!changeInfo.url && !changeInfo.title) return;
  try {
    const [active] = await chrome.tabs.query({ active: true, currentWindow: true });
    if (active?.id !== tabId) return;
    await sendTabUpdate(tab);
  } catch (_) {}
});

chrome.windows.onFocusChanged.addListener(async (windowId) => {
  if (!await isTrackingEnabled()) return;

  if (windowId === chrome.windows.WINDOW_ID_NONE) {
    // Browser lost focus.
    sendTabDeactivated();
    return;
  }

  // A different browser window gained focus — send the active tab.
  if (!isAuthenticated) { await connect(); return; }
  await sendCurrentTab();
});

// ---------------------------------------------------------------------------
// Storage change — react to token updates or enable/disable.
// ---------------------------------------------------------------------------

chrome.storage.onChanged.addListener(async (changes) => {
  if (STORAGE_TOKEN_KEY in changes || STORAGE_ENABLED_KEY in changes) {
    const { enabled } = await getToken();
    if (enabled) {
      disconnect();
      await connect();
    } else {
      disconnect();
    }
  }
});

// ---------------------------------------------------------------------------
// Alarm keepalive (MV3 service workers may be suspended after 5 min)
// ---------------------------------------------------------------------------

chrome.alarms.create(ALARM_NAME, { periodInMinutes: 0.4 }); // every ~24 s

chrome.alarms.onAlarm.addListener(async (alarm) => {
  if (alarm.name !== ALARM_NAME) return;
  if (!ws || ws.readyState === WebSocket.CLOSED || ws.readyState === WebSocket.CLOSING) {
    await connect();
  }
});

// ---------------------------------------------------------------------------
// Startup
// ---------------------------------------------------------------------------

connect();
