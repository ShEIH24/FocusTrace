const tokenInput   = document.getElementById("tokenInput");
const showTokenBtn = document.getElementById("showToken");
const enabledToggle= document.getElementById("enabledToggle");
const saveBtn      = document.getElementById("saveBtn");
const statusDot    = document.getElementById("statusDot");
const statusText   = document.getElementById("statusText");
const currentTab   = document.getElementById("currentTab");
const tabUrl       = document.getElementById("tabUrl");
const tabCategory  = document.getElementById("tabCategory");

// ---------------------------------------------------------------------------
// Load saved state
// ---------------------------------------------------------------------------

chrome.storage.local.get(["ws_token", "tracking_enabled"], (result) => {
  tokenInput.value    = result.ws_token    ?? "";
  enabledToggle.checked = result.tracking_enabled !== false;
});

// ---------------------------------------------------------------------------
// Show current tab from the active page
// ---------------------------------------------------------------------------

chrome.tabs.query({ active: true, currentWindow: true }, ([tab]) => {
  if (!tab?.url || tab.url.startsWith("chrome://") || tab.url.startsWith("edge://") || tab.url.startsWith("about:")) {
    return;
  }
  const domain = extractDomain(tab.url);
  if (domain) {
    currentTab.style.display = "block";
    tabUrl.textContent = domain;
  }
});

// ---------------------------------------------------------------------------
// WS connection status (probe via a quick connect)
// ---------------------------------------------------------------------------

function probeConnection() {
  chrome.storage.local.get(["ws_token", "tracking_enabled"], (result) => {
    if (!result.tracking_enabled && result.tracking_enabled !== undefined) {
      setStatus("disabled", "Tracking disabled");
      return;
    }
    if (!result.ws_token) {
      setStatus("error", "No token configured");
      return;
    }

    setStatus("connecting", "Connecting…");
    const probe = new WebSocket("ws://127.0.0.1:9919");

    probe.onopen = () => {
      probe.send(JSON.stringify({ type: "auth", token: result.ws_token }));
    };

    probe.onmessage = (e) => {
      let msg;
      try { msg = JSON.parse(e.data); } catch { probe.close(); return; }
      if (msg.type === "auth_ok") {
        setStatus("connected", "Connected");
      } else {
        setStatus("error", "Invalid token");
      }
      probe.close();
    };

    probe.onerror = () => {
      setStatus("error", "App not running");
      probe.close();
    };

    probe.onclose = () => {};
  });
}

probeConnection();

// ---------------------------------------------------------------------------
// Status display
// ---------------------------------------------------------------------------

function setStatus(state, text) {
  statusDot.className = "dot " + state;
  statusText.textContent = text;
}

// ---------------------------------------------------------------------------
// Controls
// ---------------------------------------------------------------------------

showTokenBtn.addEventListener("click", () => {
  tokenInput.type = tokenInput.type === "password" ? "text" : "password";
});

saveBtn.addEventListener("click", async () => {
  const token   = tokenInput.value.trim();
  const enabled = enabledToggle.checked;
  await chrome.storage.local.set({ ws_token: token, tracking_enabled: enabled });
  saveBtn.textContent = "Saved!";
  setTimeout(() => {
    saveBtn.textContent = "Save";
    probeConnection();
  }, 1200);
});

enabledToggle.addEventListener("change", async () => {
  await chrome.storage.local.set({ tracking_enabled: enabledToggle.checked });
  probeConnection();
});

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function extractDomain(url) {
  try {
    return new URL(url).hostname.replace(/^www\./, "");
  } catch {
    return null;
  }
}
