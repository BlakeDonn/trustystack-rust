pub mod diesel_schema;
pub mod graphql_schema;
pub mod migration;
pub mod models;
pub mod types;

use crate::graphql_schema::context::Context;
use actix_web::{web, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub fn greet() {
    println!("Hello from the library!");
}

pub async fn graphql_handler(
    schema: web::Data<Arc<graphql_schema::Schema>>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    log::info!("Received GraphQL request.");
    let res = data.execute(&schema, &context).await;
    log::info!("GraphQL query executed.");
    Ok(HttpResponse::Ok().json(res))
}
