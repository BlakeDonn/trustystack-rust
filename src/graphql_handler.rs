// src/graphql_handler.rs

use std::env;

use crate::graphql_schema::context::Context;
use crate::graphql_schema::{self, schema::Schema};
use crate::models::auth::user::User;
use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use juniper::http::GraphQLRequest;
use log::{error, info};
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

/// Handler for user login. Validates credentials and returns a JWT.
pub async fn login_handler(
    data: web::Json<LoginRequest>,
    context: web::Data<Context>,
) -> HttpResponse {
    let username = data.username.clone();
    let password = data.password.clone();

    // Placeholder: Replace with actual user fetching and password verification
    // For example purposes, we'll assume any username/password is valid and assign "user" role
    let user = User {
        id: 1,
        username: username.clone(),
        role: "user".to_string(),
    };

    // TODO: Verify the password using bcrypt or another hashing algorithm
    // Example:
    // let is_valid = bcrypt::verify(password, &user.hashed_password).unwrap_or(false);
    // if !is_valid {
    //     return HttpResponse::Unauthorized().json("Invalid credentials");
    // }

    // Create JWT claims
    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        role: user.role.clone(),
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize,
    };

    // Encode the JWT
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env");
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to generate JWT: {}", e);
            return HttpResponse::InternalServerError().json("Failed to generate token");
        }
    };

    info!("User '{}' logged in successfully.", user.username);

    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
}

use actix_web::HttpMessage;
/// Handles GraphQL requests by executing the query and returning the response as JSON.
use actix_web::{Error, HttpRequest};

pub async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    req: HttpRequest,
    data: web::Json<GraphQLRequest>,
    context_data: web::Data<Context>,
) -> Result<HttpResponse, Error> {
    // Extract the authenticated user from the request extensions
    let user = req.extensions().get::<User>().cloned();

    // Create a new context with the user included
    let ctx = Context::new(context_data.db.clone(), user);

    // Execute the GraphQL request
    let res = data.execute(&schema, &ctx).await;

    // Return the response as JSON
    Ok(HttpResponse::Ok().json(res))
}
