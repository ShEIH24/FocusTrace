use tauri::{AppHandle, Manager};

use crate::error::AppError;

/// Shows and focuses the main application window.
#[tauri::command]
pub async fn show_main_window(app: AppHandle) -> Result<(), AppError> {
    if let Some(w) = app.get_webview_window("main") {
        w.show()
            .map_err(|e| AppError::TrackingError(e.to_string()))?;
        w.set_focus()
            .map_err(|e| AppError::TrackingError(e.to_string()))?;
    }
    Ok(())
}

/// Hides the main application window to the system tray.
#[tauri::command]
pub async fn hide_main_window(app: AppHandle) -> Result<(), AppError> {
    if let Some(w) = app.get_webview_window("main") {
        w.hide()
            .map_err(|e| AppError::TrackingError(e.to_string()))?;
    }
    Ok(())
}
