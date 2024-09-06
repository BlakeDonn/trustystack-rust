pub mod diesel_schema; // Module for Diesel schema.
pub mod graphql_schema; // Module for GraphQL schema.
pub mod models; // Module for application models.

use crate::graphql_schema::context::Context; // Importing the GraphQL context.
use actix_web::{web, HttpResponse}; // Actix-Web imports for handling web requests and responses.
use juniper::http::GraphQLRequest; // Import for handling GraphQL requests.
use std::sync::Arc; // Arc for shared ownership of the GraphQL schema.

pub fn greet() {
    println!("Hello from the library!"); // Simple function to print a greeting.
}

pub async fn graphql_handler(
    schema: web::Data<Arc<graphql_schema::Schema>>, // The GraphQL schema.
    data: web::Json<GraphQLRequest>,                // The incoming GraphQL request.
    context: web::Data<Context>,                    // The context which includes the database pool.
) -> Result<HttpResponse, actix_web::Error> {
    log::info!("Received GraphQL request."); // Log when a GraphQL request is received.
    let res = data.execute(&schema, &context).await; // Execute the GraphQL query with the schema and context.
    log::info!("GraphQL query executed."); // Log when the GraphQL query is executed.
    Ok(HttpResponse::Ok().json(res)) // Return the result as a JSON response.
}
