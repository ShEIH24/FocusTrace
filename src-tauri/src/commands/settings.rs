use tauri::State;

use crate::config::app_config::AppConfig;
use crate::error::AppError;
use crate::state::AppState;

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, AppError> {
    let cfg = state.config.read().await;
    Ok(cfg.clone())
}

#[tauri::command]
pub async fn update_config(
    state: State<'_, AppState>,
    patch: serde_json::Value,
) -> Result<(), AppError> {
    let mut cfg = state.config.write().await;
    let data_dir = cfg.data_dir.clone();

    let mut merged = serde_json::to_value(&*cfg)?;
    if let (Some(obj), Some(patch_obj)) = (merged.as_object_mut(), patch.as_object()) {
        for (k, v) in patch_obj {
            obj.insert(k.clone(), v.clone());
        }
    }

    *cfg = serde_json::from_value(merged)?;
    cfg.save(&data_dir)
        .map_err(AppError::IoError)?;

    Ok(())
}
