use diesel::result::Error as DieselError;
use diesel_migrations::MigrationError as DieselMigrationError;
use std::env::VarError;
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] diesel::ConnectionError),

    #[error("Query error: {0}")]
    QueryError(#[from] DieselError),

    #[error("Migration error: {0}")]
    MigrationError(#[from] DieselMigrationError),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] VarError),

    #[error("Unexpected error: {0}")]
    BoxedError(#[from] Box<dyn Error + Send + Sync>),
}

