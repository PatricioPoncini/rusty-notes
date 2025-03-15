use tracing_subscriber;
use axum::{
    body::Body,
    extract::MatchedPath,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::info;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
}

pub async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let path_str = if let Some(path) = req.extensions().get::<MatchedPath>() {
        path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };

    let method_str = req.method().to_string();

    let start = Instant::now();

    let res = next.run(req).await;

    let elapsed = start.elapsed();

    info!(
        method = %method_str,
        path = %path_str,
        time_ms = elapsed.as_millis(),
        "-->"
    );

    Ok(res)
}