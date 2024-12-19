// src/graphql_handler.rs

use crate::graphql_schema::{context::Context, schema::Schema};
use actix_web::{web, HttpResponse};
use juniper::http::GraphQLRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32, // Subject (user ID)
    username: String,
    role: String, // User role
    exp: usize,   // Expiration time as UNIX timestamp
}

/// Handles GraphQL requests by executing the query and returning the response as JSON.
pub async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    context_data: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    let ctx = Context::new(context_data.db.clone());

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}
