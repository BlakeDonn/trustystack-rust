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

    /// Represents an error during data import.
    #[error("Data import error: {0}")]
    DataImportError(#[from] DataImportError),

    /// Represents an unexpected error during the migration process.
    #[error("Unexpected error: {0}")]
    BoxedError(#[from] Box<dyn Error + Send + Sync>),
}

/// Custom error enum for handling data import errors.
#[derive(Debug, Error)]
pub enum DataImportError {
    /// Represents an IO error during data import.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    /// Represents a CSV parsing error.
    #[error("CSV error: {0}")]
    CSVError(#[from] csv::Error),

    /// Represents a JSON parsing error.
    #[error("JSON error: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// Represents a Diesel ORM error during data import.
    #[error("Diesel error: {0}")]
    DieselError(#[from] DieselError),

    /// Represents a custom error message.
    #[error("Custom error: {0}")]
    CustomError(String),
}
