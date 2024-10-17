use actix_web::{guard, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use rust_backend::graphql_schema::context::Context;
use rust_backend::{graphql_handler, graphql_schema};
use std::env;
use std::sync::Arc;

/// Entry point of the Rust backend server, responsible for initializing the environment,
/// setting up the database connection, and starting the HTTP server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the Rust backend server...");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Logger initialized");

    dotenv().ok();
    info!("Loaded environment variables from .env");

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

    let schema = Arc::new(graphql_schema::create_schema());
    let context = web::Data::new(Context::new(pool));

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
            println!("Server started successfully at http://0.0.0.0:8080");
            server.run().await
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            println!("Error: Failed to start server: {}", e);
            std::process::exit(1);
        }
    }
}
