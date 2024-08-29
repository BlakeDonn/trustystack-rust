use crate::graphql_schema::context::Context;
use actix_web::{guard, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use env_logger;
use log::info;
use rust_backend::{graphql_handler, graphql_schema};
use std::env;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    info!("Starting the Actix-Web server...");

    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the DATABASE_URL from the environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create the connection pool
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    info!("Database connection pool created successfully.");

    // Create the schema with the correct context type
    let schema = Arc::new(graphql_schema::create_schema());
    let context = web::Data::new(Context::new(pool));

    // Start the Actix-Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone())) // Pass the schema
            .app_data(context.clone()) // Pass the context
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_handler),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
