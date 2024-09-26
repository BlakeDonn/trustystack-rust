// src/migration.rs

use crate::types::errors::MigrationError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn generate_lock_key(app_name: &str) -> i64 {
    let mut hasher = DefaultHasher::new();
    app_name.hash(&mut hasher);
    hasher.finish() as i64
}

pub struct AdvisoryLock {
    connection: PgConnection,
    lock_key: i64,
}

impl AdvisoryLock {
    pub fn new(database_url: &str, lock_key: i64) -> Result<Self, MigrationError> {
        let mut connection = PgConnection::establish(database_url)?;
        diesel::sql_query(format!("SELECT pg_advisory_lock({});", lock_key))
            .execute(&mut connection)?;
        Ok(AdvisoryLock {
            connection,
            lock_key,
        })
    }

    pub fn run_migrations(&mut self) -> Result<(), MigrationError> {
        self.connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }
}

impl Drop for AdvisoryLock {
    fn drop(&mut self) {
        if let Err(e) = diesel::sql_query(format!("SELECT pg_advisory_unlock({});", self.lock_key))
            .execute(&mut self.connection)
        {
            eprintln!("Failed to release advisory lock: {}", e);
        }
    }
}

pub fn run() -> Result<(), MigrationError> {
    let database_url = env::var("DATABASE_URL")?;

    let lock_key = generate_lock_key("rust_backend");

    let mut lock = AdvisoryLock::new(&database_url, lock_key)?;

    lock.run_migrations()?;

    println!("Database migrations completed successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_successful_migration() {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        env::set_var("DATABASE_URL", database_url);

        let result = run();

        assert!(result.is_ok(), "Migrations should complete successfully");
    }

    #[test]
    fn test_advisory_lock() {
        dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let lock_key = generate_lock_key("rust_backend_test");

        let barrier = Arc::new(Barrier::new(2));

        let barrier_clone = barrier.clone();
        let database_url_clone = database_url.clone();

        let handle = thread::spawn(move || {
            let _lock = AdvisoryLock::new(&database_url_clone, lock_key)
                .expect("Thread 1: Failed to acquire lock");
            barrier_clone.wait(); // Signal that the lock has been acquired
                                  // Hold the lock for a short duration
            std::thread::sleep(std::time::Duration::from_secs(2));
            // Lock is released when `_lock` goes out of scope
        });

        barrier.wait(); // Wait for the first thread to acquire the lock

        let start_time = std::time::Instant::now();
        let _lock =
            AdvisoryLock::new(&database_url, lock_key).expect("Thread 2: Failed to acquire lock");
        let elapsed = start_time.elapsed();

        assert!(
            elapsed >= std::time::Duration::from_secs(2),
            "Lock acquisition should be delayed until the first lock is released"
        );

        handle.join().expect("Failed to join thread");
    }

    #[test]
    fn test_missing_database_url() {
        // Temporarily unset DATABASE_URL
        dotenv().ok();
        let original_value = env::var("DATABASE_URL").ok();
        env::remove_var("DATABASE_URL");

        let result = run();

        assert!(
            matches!(result, Err(MigrationError::EnvVarError(_))),
            "Should return EnvVarError when DATABASE_URL is missing"
        );

        // Restore the original value
        if let Some(value) = original_value {
            env::set_var("DATABASE_URL", value);
        }
    }

    #[test]
    fn test_invalid_database_url() {
        let original_value = env::var("DATABASE_URL").ok();
        env::set_var("DATABASE_URL", "invalid_url");

        let result = run();

        assert!(
            matches!(result, Err(MigrationError::ConnectionError(_))),
            "Should return ConnectionError for invalid DATABASE_URL"
        );

        // Restore the original value
        if let Some(value) = original_value {
            env::set_var("DATABASE_URL", value);
        }
    }
}
