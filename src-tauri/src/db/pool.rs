use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use crate::error::AppError;

/// Opens (or creates) a SQLite connection pool, applies performance pragmas,
/// then runs all pending migrations.
///
/// Pragma choices:
/// - WAL journal  — concurrent readers + writer, no reader/writer blocking
/// - Normal sync  — safe with WAL (OS crash-safe), faster than FULL
/// - 20 MB cache  — reduces disk I/O for repeated range scans
/// - MEMORY temp  — temp tables/indexes live in RAM
/// - Foreign keys — enforced at runtime (disabled by SQLite default)
pub async fn create_pool(db_path: &Path) -> Result<SqlitePool, AppError> {
    let url = format!("sqlite:{}", db_path.display());

    let options = SqliteConnectOptions::from_str(&url)?
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5))
        .pragma("cache_size", "-20000") // 20 MB page cache
        .pragma("temp_store", "MEMORY")
        .pragma("foreign_keys", "ON")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
