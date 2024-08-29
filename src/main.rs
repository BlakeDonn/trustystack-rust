use actix_web::{guard, web, App, HttpServer};
use rust_backend::{graphql_handler, graphql_schema};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rust_backend::greet();

    // Create the schema with the correct context type
    let schema = Arc::new(graphql_schema::create_schema());

    // Start the Actix-Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone())) // Pass the schema
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

