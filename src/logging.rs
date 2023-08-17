use std::task::{Context, Poll};

use axum::body::{Body, BoxBody};
use chrono::Local;
use colored::Colorize;
use http::{Request, Response, Method, Uri};
use pin_project::pin_project;
use std::{future::Future, pin::Pin};
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
    S: Service<Request<Body>, Response = Response<BoxBody>>,
{
    type Response = Response<BoxBody>;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        let method = request.method().clone();
        let uri = request.uri().clone();

        let response_future = self.service.call(request);

        ResponseFuture { response_future, method, uri }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response_future: F,
    uri: Uri,
    method: Method,
}

impl<F, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<BoxBody>, Error>>,
{
    type Output = Result<Response<BoxBody>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                if let Ok(response) = result {
                    print!("{} - ", Local::now().format("%Y-%m-%d %H:%M:%S"));
                    print!("{}: {} - ", "Method".blue(), this.method);
                    print!("{}: '{}' ", "Route".green(), this.uri);
                    println!("{}: {} ", "Status".purple(), response.status());

                    return Poll::Ready(Ok(response));
                } else {
                    println!("Nope")
                }

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
