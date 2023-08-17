use axum::{routing::get, Router};
use logging::LogLayer;

mod logging;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(LogLayer {});

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
