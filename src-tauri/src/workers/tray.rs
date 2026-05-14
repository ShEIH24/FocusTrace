use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use tracing::error;

/// Builds the system tray icon with menu and event handlers.
/// Must be called once inside the Tauri `setup` closure.
/// The returned `TrayIcon` is managed by Tauri and kept alive for the app lifetime.
pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let show_item = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit FocusTrace", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_item, &sep, &quit_item])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .tooltip("FocusTrace")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            // Left-click toggles window visibility.
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                toggle_window(app);
            }
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show_window(app),
            "quit" => {
                // Send Shutdown through the event channel so the coordinator
                // can finalize the current session before the process exits.
                use crate::events::types::AppEvent;
                use crate::state::AppState;
                if let Some(state) = app.try_state::<AppState>() {
                    let _ = state.event_tx.try_send(AppEvent::Shutdown);
                }
                // Give the coordinator ~200 ms to flush, then exit.
                std::thread::spawn(|| {
                    std::thread::sleep(std::time::Duration::from_millis(250));
                    std::process::exit(0);
                });
            }
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;

    Ok(())
}

fn show_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        if let Err(e) = w.show().and_then(|_| w.set_focus()) {
            error!("show_window: {e}");
        }
    }
}

fn toggle_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        match w.is_visible() {
            Ok(true) => {
                let _ = w.hide();
            }
            _ => show_window(app),
        }
    }
}
