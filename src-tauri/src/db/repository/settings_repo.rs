#![allow(dead_code)]

use chrono::Utc;
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::db::models::SettingRow;
use crate::error::AppError;

/// CRUD operations on the `settings` key-value table.
///
/// Values are stored as raw strings; callers are responsible for
/// serializing/deserializing (typically JSON).
pub struct SettingsRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SettingsRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Returns the value for `key`, or `None` if the key does not exist.
    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let value: Option<String> =
            sqlx::query_scalar("SELECT value FROM settings WHERE key = ?")
                .bind(key)
                .fetch_optional(self.pool)
                .await?;
        Ok(value)
    }

    /// Inserts or replaces a setting.
    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value,
                                            updated_at = excluded.updated_at",
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Returns all settings as a map.
    pub async fn get_all(&self) -> Result<HashMap<String, String>, AppError> {
        let rows = sqlx::query_as::<_, SettingRow>(
            "SELECT key, value, updated_at FROM settings ORDER BY key",
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| (r.key, r.value)).collect())
    }

    /// Removes a setting.  No-op if the key does not exist.
    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM settings WHERE key = ?")
            .bind(key)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Reads a JSON-encoded value and deserializes it, or returns `default`
    /// if the key is absent or deserialization fails.
    pub async fn get_json<T>(&self, key: &str, default: T) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        match self.get(key).await {
            Ok(Some(raw)) => serde_json::from_str(&raw).unwrap_or(default),
            _ => default,
        }
    }

    /// Serializes `value` to JSON and stores it.
    pub async fn set_json<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<(), AppError> {
        let raw = serde_json::to_string(value)?;
        self.set(key, &raw).await
    }

    /// Infallible `get` — returns `None` on error or absence.
    pub async fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).await.ok().flatten()
    }

    /// Alias for `set` with an infallible signature mirror.
    pub async fn set_string(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.set(key, value).await
    }
}
