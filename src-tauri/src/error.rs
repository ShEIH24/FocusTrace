use serde::ser::SerializeStruct;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[allow(dead_code)]
    #[error("tracking error: {0}")]
    TrackingError(String),
    #[error("config error: {0}")]
    ConfigError(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let code = match self {
            AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::MigrationError(_) => "MIGRATION_ERROR",
            AppError::TrackingError(_) => "TRACKING_ERROR",
            AppError::ConfigError(_) => "CONFIG_ERROR",
            AppError::IoError(_) => "IO_ERROR",
            AppError::SerdeError(_) => "SERDE_ERROR",
        };
        let mut s = serializer.serialize_struct("AppError", 2)?;
        s.serialize_field("code", code)?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}
