use std::task::{Context, Poll};

use axum::body::Body;
use chrono::Local;
use colored::Colorize;
use http::Request;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LogLayer;

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService { service }
    }
}

#[derive(Clone)]
pub struct LogService<S> {
    service: S,
}

impl<S> Service<Request<Body>> for LogService<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        // println!("request = {:?}", request);
        print!("{} - ", Local::now().format("%Y-%m-%d %H:%M:%S"));
        print!("{}: {} - ", "Method".blue(), request.method());
        println!("{}: {} ", "Route".green(), request.uri());
        self.service.call(request)
    }
}
