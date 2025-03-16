use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use crate::db::models::note::Note;

#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub description: String,
}

pub async fn create_note(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(req): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    match Note::save_note(&pool.clone(), &req.title, &req.description)
        .await
    {
        Ok(_) => {
            (StatusCode::CREATED).into_response()
        }
        Err(_e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create note").into_response()
        }
    }
}