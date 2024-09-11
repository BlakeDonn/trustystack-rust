use actix_web::{guard, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool}; // Imports for managing database connections.
use diesel::PgConnection; // PostgreSQL connection type from Diesel.
use dotenv::dotenv; // Dotenv to load environment variables from a .env file.
use env_logger; // Logger initialization for logging.
use log::{error, info}; // Macros for logging informational and error messages.
use rust_backend::graphql_schema::context::Context;
use rust_backend::{graphql_handler, graphql_schema}; // Importing the graphql handler and schema.
use std::env; // Standard library's environment handling.
use std::sync::Arc; // Arc for shared ownership of the GraphQL schema.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initial print for Docker logs
    println!("Starting the Rust backend server...");

    // Initialize the logger
    env_logger::init();
    info!("Logger initialized");

    // Load environment variables from .env file
    dotenv().ok();
    info!("Loaded environment variables from .env");

    // Retrieve and log the DATABASE_URL
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => {
            println!("DATABASE_URL found: {}", url);
            info!("Database URL: {}", url);
            url
        }
        Err(_) => {
            error!("DATABASE_URL must be set in .env");
            println!("Error: DATABASE_URL must be set in .env");
            std::process::exit(1);
        }
    };

    // Create the database connection pool
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = match Pool::builder().build(manager) {
        Ok(pool) => {
            info!("Database connection pool created successfully.");
            println!("Database connection pool created successfully.");
            pool
        }
        Err(e) => {
            error!("Failed to create pool: {}", e);
            println!("Error: Failed to create pool: {}", e);
            std::process::exit(1);
        }
    };

    // Set up the GraphQL schema and Actix context
    let schema = Arc::new(graphql_schema::create_schema());
    let context = web::Data::new(Context::new(pool));

    // Start the Actix-Web server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .app_data(context.clone())
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
    })
    .bind("0.0.0.0:8080");

    match server {
        Ok(server) => {
            info!("Server started successfully at http://0.0.0.0:8080");
            println!("Server started successfully at http://0.0.0.0:8080"); // Print to stdout for Docker logs
            server.run().await
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            println!("Error: Failed to start server: {}", e); // Print to stdout for Docker logs
            std::process::exit(1);
        }
    }
}
