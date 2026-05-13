#![allow(dead_code)]

use sqlx::SqlitePool;

use crate::db::repository::{AppRepository, NewSession, SessionRepository};
use crate::error::AppError;
use crate::events::types::SessionInfo;

/// High-level write interface used by external callers that already have a
/// `SessionInfo` and want a single call to handle the full upsert chain.
pub struct DbWriter {
    pool: SqlitePool,
}

impl DbWriter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Upserts the app record and inserts the session in one call.
    pub async fn write_session(&self, session: &SessionInfo) -> Result<(), AppError> {
        let category_id = match session.category.as_str() {
            "Productive" => 1,
            "Distraction" => 3,
            _ => 2,
        };

        let app_id = AppRepository::new(&self.pool)
            .upsert(
                &session.exe,
                &session.exe_path,
                category_id,
                session.duration_ms,
            )
            .await?;

        SessionRepository::new(&self.pool)
            .insert(&NewSession {
                app_id,
                title: session.title.clone(),
                started_at: session.started_at,
                duration_ms: session.duration_ms,
            })
            .await
    }
}
