// src/graphql_handler.rs

use crate::graphql_schema::{context::Context, schema::Schema};
use crate::models::auth::User;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use juniper::http::GraphQLRequest;
use std::sync::Arc;

/// Handles GraphQL requests by executing the query and returning the response as JSON.
pub async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    req: HttpRequest,
    data: web::Json<GraphQLRequest>,
    context_data: web::Data<Context>,
) -> Result<HttpResponse, actix_web::Error> {
    // Extract the authenticated user from the request extensions
    let user = req.extensions().get::<User>().cloned();

    // Create a new context with the user included
    let ctx = Context::new(context_data.db.clone(), user);

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}
