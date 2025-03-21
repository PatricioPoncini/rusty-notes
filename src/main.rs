mod db;
mod env;
mod logger;
mod routes;
mod test;

use crate::db::root::connect_db;
use crate::env::{get_env, init_env};
use crate::logger::root::{init_logger, logging_middleware};
use crate::routes::doc::{doc, openapi_yaml};
use crate::routes::note::{create_note, delete_note, get_all_notes, update_note};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router, middleware};
use std::sync::Arc;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    init_logger();

    if let Err(e) = init_env() {
        error!("Error loading environment variables: {}", e);
        std::process::exit(1);
    }

    let db_pool = match connect_db().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    let shared_pool = Arc::new(db_pool);

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/notes", post(create_note))
        .route("/notes", get(get_all_notes))
        .route("/notes/{id}", put(update_note))
        .route("/notes/{id}", delete(delete_note))
        .route("/doc", get(doc))
        .route("/openapi.yaml", get(openapi_yaml))
        .layer(middleware::from_fn(logging_middleware))
        .layer(Extension(shared_pool))
        .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", get_env("PORT")))
        .await
        .unwrap();
    info!(
        "{}",
        format!("🚀 Server running on port :{}", get_env("PORT"))
    );
    axum::serve(listener, app).await.unwrap();
}
