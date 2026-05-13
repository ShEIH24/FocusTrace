#![allow(dead_code)]

use chrono::Utc;
use sqlx::SqlitePool;

use crate::db::models::App;
use crate::error::AppError;

/// CRUD + upsert operations on the `apps` table.
///
/// `AppRepository` is stateless: it borrows the pool and can be created
/// on-demand in any async context without locking overhead.
pub struct AppRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> AppRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    /// Inserts a new app row or, on `exe` conflict, updates `exe_path`
    /// (only if non-empty), `last_seen`, and increments `total_ms`.
    ///
    /// `category_id` is only applied on the *first* insert — subsequent
    /// upserts preserve any manual category the user may have assigned.
    ///
    /// Returns the row `id` of the inserted or existing app.
    pub async fn upsert(
        &self,
        exe: &str,
        exe_path: &str,
        category_id: i64,
        session_ms: i64,
    ) -> Result<i64, AppError> {
        let now = Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await?;

        // Insert if not present; ignore conflict so the row always exists after this.
        sqlx::query(
            "INSERT OR IGNORE INTO apps
                 (exe, display_name, exe_path, category_id, first_seen, last_seen, total_ms)
             VALUES (?, ?, ?, ?, ?, ?, 0)",
        )
        .bind(exe)
        .bind(exe)
        .bind(exe_path)
        .bind(category_id)
        .bind(&now)
        .bind(&now)
        .execute(&mut *tx)
        .await?;

        // Always update mutable stats; update category on every upsert so the
        // classifier improvements (e.g. Steam path detection) take effect on
        // existing DB rows without requiring a manual DB reset.
        sqlx::query(
            "UPDATE apps SET
                 exe_path    = CASE WHEN length(?) > 0 THEN ? ELSE exe_path END,
                 category_id = ?,
                 last_seen   = ?,
                 total_ms    = total_ms + ?
             WHERE exe = ?",
        )
        .bind(exe_path)
        .bind(exe_path)
        .bind(category_id)
        .bind(&now)
        .bind(session_ms)
        .bind(exe)
        .execute(&mut *tx)
        .await?;

        let id: i64 = sqlx::query_scalar("SELECT id FROM apps WHERE exe = ?")
            .bind(exe)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(id)
    }

    pub async fn find_by_exe(&self, exe: &str) -> Result<Option<App>, AppError> {
        let row = sqlx::query_as::<_, App>(
            "SELECT id, exe, display_name, exe_path, category_id,
                    first_seen, last_seen, total_ms
             FROM apps WHERE exe = ?",
        )
        .bind(exe)
        .fetch_optional(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn find_all(&self) -> Result<Vec<App>, AppError> {
        let rows = sqlx::query_as::<_, App>(
            "SELECT id, exe, display_name, exe_path, category_id,
                    first_seen, last_seen, total_ms
             FROM apps
             ORDER BY total_ms DESC",
        )
        .fetch_all(self.pool)
        .await?;

        Ok(rows)
    }

    /// Overrides the human-readable label shown in the UI.
    pub async fn set_display_name(&self, id: i64, name: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE apps SET display_name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Re-assigns an app to a different productivity category.
    pub async fn set_category(&self, id: i64, category_id: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE apps SET category_id = ? WHERE id = ?")
            .bind(category_id)
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
