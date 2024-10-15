use crate::types::errors::MigrationError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};

/// Embeds the Diesel migrations into the binary.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Generates a unique lock key based on the application name.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
///
/// # Returns
///
/// * A unique i64 hash value used as the lock key.
pub fn generate_lock_key(app_name: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    app_name.hash(&mut hasher);
    hasher.finish() as i64
}

/// Manages the advisory lock for database migrations.
pub struct AdvisoryLock {
    connection: PgConnection,
    lock_key: i64,
}

impl AdvisoryLock {
    /// Acquires an advisory lock using the provided database URL and lock key.
    ///
    /// # Arguments
    ///
    /// * `database_url` - The database connection URL.
    /// * `lock_key` - A unique key for the advisory lock.
    ///
    /// # Returns
    ///
    /// * `Ok(AdvisoryLock)` if the lock is successfully acquired.
    /// * `Err(MigrationError)` if an error occurs.
    pub fn new(database_url: &str, lock_key: i64) -> Result<Self, MigrationError> {
        let mut connection = PgConnection::establish(database_url)?;
        diesel::sql_query(format!("SELECT pg_advisory_lock({});", lock_key))
            .execute(&mut connection)?;
        Ok(AdvisoryLock {
            connection,
            lock_key,
        })
    }

    /// Runs pending migrations using the embedded migration scripts.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if migrations run successfully.
    /// * `Err(MigrationError)` if an error occurs during migration.
    pub fn run_migrations(&mut self) -> Result<(), MigrationError> {
        self.connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    /// Runs data import after migrations.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if data import is successful.
    /// * `Err(MigrationError)` if an error occurs during data import.
    pub fn run_data_import(&mut self) -> Result<(), MigrationError> {
        crate::data_import::run_data_import(&mut self.connection)
            .map_err(MigrationError::DataImportError)?;
        Ok(())
    }
}

impl Drop for AdvisoryLock {
    /// Releases the advisory lock when the `AdvisoryLock` instance goes out of scope.
    fn drop(&mut self) {
        if let Err(e) = diesel::sql_query(format!("SELECT pg_advisory_unlock({});", self.lock_key))
            .execute(&mut self.connection)
        {
            eprintln!("Failed to release advisory lock: {}", e);
        }
    }
}

/// Executes the migration and data import process.
///
/// # Returns
///
/// * `Ok(())` if migrations and data import are successful.
/// * `Err(MigrationError)` if an error occurs.
pub fn run() -> Result<(), MigrationError> {
    let database_url = env::var("DATABASE_URL")?;

    let lock_key = generate_lock_key("rust_backend");

    let mut lock = AdvisoryLock::new(&database_url, lock_key)?;

    lock.run_migrations()?;
    lock.run_data_import()?;

    println!("Database migrations and data import completed successfully.");

    Ok(())
}
