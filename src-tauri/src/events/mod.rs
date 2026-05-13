pub mod types;

use tauri::{AppHandle, Emitter};

use crate::events::types::{DistractionAlertPayload, MetricsUpdatedPayload, SessionEndedPayload, WindowInfo};

pub fn emit_activity_updated(app: &AppHandle, info: &WindowInfo) {
    if let Err(e) = app.emit("activity-updated", info) {
        tracing::error!("emit activity-updated: {e}");
    }
}

pub fn emit_idle_changed(app: &AppHandle, payload: serde_json::Value) {
    if let Err(e) = app.emit("idle-changed", payload) {
        tracing::error!("emit idle-changed: {e}");
    }
}

pub fn emit_session_ended(app: &AppHandle, payload: &SessionEndedPayload) {
    if let Err(e) = app.emit("session-ended", payload) {
        tracing::error!("emit session-ended: {e}");
    }
}

pub fn emit_metrics_updated(app: &AppHandle, payload: &MetricsUpdatedPayload) {
    if let Err(e) = app.emit("metrics-updated", payload) {
        tracing::error!("emit metrics-updated: {e}");
    }
}

pub fn emit_distraction_alert(app: &AppHandle, payload: &DistractionAlertPayload) {
    if let Err(e) = app.emit("distraction-alert", payload) {
        tracing::error!("emit distraction-alert: {e}");
    }
}
