// tests/migration_tests.rs

use diesel::pg::PgConnection;
use diesel_migrations::MigrationHarness;
use dotenv::dotenv;
use rust_backend::migration::{generate_lock_key, run, AdvisoryLock, MIGRATIONS};
use rust_backend::types::errors::MigrationError;
use serial_test::serial;
use std::env;
use std::sync::{Arc, Barrier};
use std::thread;

/// Tests that migrations and data import run successfully using the test database.
#[test]
fn test_successful_migration_and_data_import() {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    env::set_var("DATABASE_URL", database_url);

    let result = run();
    assert!(
        result.is_ok(),
        "Migrations and data import should complete successfully: {:?}",
        result
    );
}

/// Tests the advisory lock mechanism to prevent concurrent migrations.
#[test]
fn test_advisory_lock() {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let lock_key = generate_lock_key("rust_backend_test");

    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = Arc::clone(&barrier);
    let database_url_clone = database_url.clone();

    let handle = thread::spawn(move || {
        let _lock = AdvisoryLock::new(&database_url_clone, lock_key)
            .expect("Thread 1: Failed to acquire lock");
        barrier_clone.wait(); // Signal that the lock has been acquired
        std::thread::sleep(std::time::Duration::from_secs(2));
        // The lock is released when `_lock` goes out of scope
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

/// Tests error handling when the `DATABASE_URL` environment variable is missing.
#[test]
#[serial]
fn test_missing_database_url() {
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

/// Tests error handling with an invalid `DATABASE_URL`.
#[test]
#[serial]
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
