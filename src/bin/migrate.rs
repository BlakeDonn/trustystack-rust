use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect("Error connecting to the database");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");

    println!("Database migrations completed successfully.");
}

