// src/middleware/logging.rs

use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Bytes;
use actix_web::{Error, HttpMessage};
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::stream::{self, StreamExt};
use log::info;
use serde_json::Value;
use std::rc::Rc;
use std::task::{Context as TaskContext, Poll};

/// Middleware to log incoming GraphQL requests and their variables
pub struct GraphQLLogging;

impl<S, B> Transform<S, ServiceRequest> for GraphQLLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GraphQLLoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(GraphQLLoggingMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct GraphQLLoggingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for GraphQLLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: actix_web::body::MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    /// Check if the service is ready to accept a request
    fn poll_ready(&self, ctx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    /// Handle the incoming request
    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();

        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            if method == "POST" && path == "/graphql" {
                // Take the payload from the request
                let mut payload = req.take_payload();
                let mut body_bytes = Vec::new();

                // Collect the payload into bytes manually
                while let Some(chunk) = payload.next().await {
                    let chunk = chunk?;
                    body_bytes.extend_from_slice(&chunk);
                }

                // Convert the collected bytes into `Bytes`
                let body = Bytes::from(body_bytes.clone());

                // Attempt to parse the JSON body
                if let Ok(json) = serde_json::from_slice::<Value>(&body_bytes) {
                    let query = json.get("query").and_then(|q| q.as_str()).unwrap_or("");
                    let variables = json.get("variables").unwrap_or(&Value::Null);
                    info!("GraphQL Query: {}", query);
                    info!("GraphQL Variables: {}", variables);
                } else {
                    info!("GraphQL Request could not be parsed as JSON.");
                }

                // Create a stream for `Bytes` wrapped in `Ok`, boxed with `'static` lifetime
                let new_payload = Payload::from(
                    stream::once(async move { Ok(body) }).boxed_local(), // Boxes the stream to fit the expected trait with a 'static lifetime
                );
                req.set_payload(new_payload);
            }

            // Proceed with the request
            let res = srv.call(req).await?;
            Ok(res)
        })
    }
}
