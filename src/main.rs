mod logger;

use axum::{middleware, Router};
use axum::routing::get;
use tracing::info;
use crate::logger::root::{init_logger, logging_middleware};

#[tokio::main]
async fn main() {
    init_logger();

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .layer(middleware::from_fn(logging_middleware));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("Server running on port :8080");
    axum::serve(listener, app).await.unwrap();
}
