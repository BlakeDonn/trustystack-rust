pub mod diesel_schema;
pub mod graphql_schema;
pub mod models;

use actix_web::{web, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

pub fn greet() {
    println!("Hello from the library!");
}

pub async fn graphql_handler(
    schema: web::Data<Arc<graphql_schema::Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let res = data.execute(&schema, &()).await;
    Ok(HttpResponse::Ok().json(res))
}
