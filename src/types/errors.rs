use diesel::result::Error as DieselError;
use diesel_migrations::MigrationError as DieselMigrationError;
use std::env::VarError;
use std::error::Error;
use thiserror::Error;

/// Custom error enum for handling different types of migration-related errors.
#[derive(Debug, Error)]
pub enum MigrationError {
    /// Represents a connection error during migrations.
    #[error("Connection error: {0}")]
    ConnectionError(#[from] diesel::ConnectionError),

    /// Represents a query execution error.
    #[error("Query error: {0}")]
    QueryError(#[from] DieselError),

    /// Represents an error during a migration step.
    #[error("Migration error: {0}")]
    MigrationError(#[from] DieselMigrationError),

    /// Represents an error related to environment variables.
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] VarError),

    /// Represents an unexpected error during the migration process.
    #[error("Unexpected error: {0}")]
    BoxedError(#[from] Box<dyn Error + Send + Sync>),
}
