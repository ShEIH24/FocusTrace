pub mod goals;
pub mod models;
pub mod pomodoro;

pub use pomodoro::spawn as spawn_pomodoro;

use sqlx::SqlitePool;

use crate::db::repository::SettingsRepository;
use crate::error::AppError;
use crate::productivity::models::ProductivityConfig;

const CONFIG_KEY: &str = "productivity:config";

pub async fn load_config(pool: &SqlitePool) -> ProductivityConfig {
    SettingsRepository::new(pool)
        .get_json(CONFIG_KEY, ProductivityConfig::default())
        .await
}

pub async fn save_config(pool: &SqlitePool, cfg: &ProductivityConfig) -> Result<(), AppError> {
    SettingsRepository::new(pool)
        .set_json(CONFIG_KEY, cfg)
        .await
}
