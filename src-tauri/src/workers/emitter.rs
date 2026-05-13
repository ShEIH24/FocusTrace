use tauri::{AppHandle, Emitter};

use crate::events::types::WindowInfo;

/// Emits an `activity-updated` event to all frontend windows.
pub fn emit_activity_updated(app: &AppHandle, info: &WindowInfo) {
    if let Err(e) = app.emit("activity-updated", info) {
        tracing::error!("Failed to emit activity-updated: {e}");
    }
}

/// Emits an `idle-changed` event with the current idle state payload.
pub fn emit_idle_changed(app: &AppHandle, payload: serde_json::Value) {
    if let Err(e) = app.emit("idle-changed", payload) {
        tracing::error!("Failed to emit idle-changed: {e}");
    }
}
