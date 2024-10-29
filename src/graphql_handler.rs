use crate::graphql_schema;
use crate::graphql_schema::context::Context;
use actix_web::{web, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

/// Function to print a greeting message to the console.
pub fn greet() {
    println!("Hello from the library!");
}

/// Handles GraphQL requests by executing the query and returning the response as JSON.
pub async fn graphql_handler(
    schema: web::Data<Arc<graphql_schema::schema::Schema>>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    log::info!("Received GraphQL request.");
    let res = data.execute(&schema, &context).await;
    log::info!("GraphQL query executed.");
    Ok(HttpResponse::Ok().json(res))
}
