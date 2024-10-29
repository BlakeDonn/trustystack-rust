// src/middleware/auth.rs

use crate::models::auth::user::User;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::env;
use std::rc::Rc;

/// Represents the claims contained within a JWT.
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    username: String,
    role: String,
    exp: usize,
}

/// Authentication middleware struct.
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner {
            service: Rc::new(service),
        })
    }
}

/// Inner middleware struct holding the next service in the chain.
pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract the Authorization header
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    // Check if the header starts with "Bearer "
                    match auth_str.starts_with("Bearer ") {
                        true => {
                            let token = &auth_str[7..]; // Remove "Bearer " prefix

                            // Decode the JWT
                            let secret =
                                env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env");
                            let decoding_key = DecodingKey::from_secret(secret.as_ref());

                            let validation = Validation::new(Algorithm::HS256);

                            match decode::<Claims>(token, &decoding_key, &validation) {
                                Ok(token_data) => {
                                    let claims = token_data.claims;
                                    info!("Authenticated user: {}", claims.username);

                                    // Create a User instance from claims
                                    let user = User {
                                        id: claims.sub,
                                        username: claims.username,
                                        role: claims.role,
                                    };

                                    // Insert the user into the request's extensions
                                    req.extensions_mut().insert(user);
                                }
                                Err(e) => {
                                    error!("Failed to decode JWT: {}", e);
                                    return Err(actix_web::error::ErrorUnauthorized(
                                        "Invalid Token",
                                    ));
                                }
                            }
                        }
                        false => (),
                    }
                }
            }

            // Call the next service in the chain
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
