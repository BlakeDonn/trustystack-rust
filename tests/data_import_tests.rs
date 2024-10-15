// tests/data_import_tests.rs

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::MigrationHarness;
use dotenv::dotenv;
use rust_backend::data_import::{import_manufacturers_with_path, run_data_import};
use rust_backend::migration::MIGRATIONS;
use rust_backend::types::errors::DataImportError;
use std::env;
use tempfile::tempdir;

/// Tests that data import runs successfully using the test database.
#[test]
fn test_successful_data_import() {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let mut conn =
        PgConnection::establish(&database_url).expect("Failed to connect to test database");

    // Run migrations
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    // Run data import
    let result = run_data_import(&mut conn);
    assert!(
        result.is_ok(),
        "Data import should complete successfully: {:?}",
        result
    );
}

/// Tests error handling when CSV files are missing.
#[test]
fn test_missing_csv_files() {
    dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");

    let mut conn =
        PgConnection::establish(&database_url).expect("Failed to connect to test database");

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let temp_csv_path = temp_dir.path().join("manufacturers.csv");

    // Copy the CSV file to the temp directory
    std::fs::copy("./data/csv/manufacturers.csv", &temp_csv_path).expect("Failed to copy CSV file");

    // Remove the temp CSV file to simulate missing file
    std::fs::remove_file(&temp_csv_path).expect("Failed to remove CSV file");

    // Adjust the import function to accept a path parameter
    let result = import_manufacturers_with_path(&mut conn, temp_csv_path.to_str().unwrap());

    assert!(
        matches!(result, Err(DataImportError::CSVError(_))),
        "Should return CSVError when CSV file is missing"
    );

    // The temporary directory and its contents are automatically deleted
}
