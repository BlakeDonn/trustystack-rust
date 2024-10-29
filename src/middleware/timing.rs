// src/middleware/timing.rs

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::info;
use std::rc::Rc;
use std::task::{Context as TaskContext, Poll};
use std::time::Instant;

/// Middleware to measure and log the duration of each request.
pub struct Timing;

impl<S, B> Transform<S, ServiceRequest> for Timing
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TimingMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct TimingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TimingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            let res = srv.call(req).await?;
            let duration = start.elapsed();
            info!("Request processed in {:?}", duration);
            Ok(res)
        })
    }
}
