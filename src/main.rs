use crate::graphql_schema::context::Context;
use actix_web::{guard, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool}; // Imports for managing database connections.
use diesel::PgConnection; // PostgreSQL connection type from Diesel.
use dotenv::dotenv; // Dotenv to load environment variables from a .env file.
use env_logger; // Logger initialization for logging.
use log::info; // Macro for logging informational messages.
use rust_backend::{graphql_handler, graphql_schema}; // Importing the graphql handler and schema.
use std::env; // Standard library's environment handling.
use std::sync::Arc; // Arc for shared ownership of the GraphQL schema.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    info!("Starting the Actix-Web server...");

    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the DATABASE_URL from the environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create the connection pool for PostgreSQL
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    info!("Database connection pool created successfully.");

    // Create the GraphQL schema and wrap it in Arc for thread safety
    let schema = Arc::new(graphql_schema::create_schema());
    let context = web::Data::new(Context::new(pool)); // Create the context with the database pool.

    // Start the Actix-Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone())) // Pass the schema to the application data.
            .app_data(context.clone()) // Pass the context to the application data.
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler), // Handle POST requests to /graphql with graphql_handler.
            )
    })
    .bind("127.0.0.1:8080")? // Bind the server to the specified address and port.
    .run()
    .await
}
