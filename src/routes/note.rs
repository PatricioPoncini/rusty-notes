use crate::db::models::note::Note;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;

#[derive(Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub description: String,
}

pub async fn create_note(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(req): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    match Note::save_note(&pool.clone(), &req.title, &req.description).await {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(e) => {
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create note").into_response()
        }
    }
}

pub async fn get_all_notes(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(_): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    match Note::get_all(&pool.clone()).await {
        Ok(notes) => (StatusCode::CREATED, Json(notes)).into_response(),
        Err(e) => {
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get all notes").into_response()
        }
    }
}

pub async fn update_note(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<i32>,
    Json(req): Json<CreateNoteRequest>,
) -> impl IntoResponse {
    match Note::update(&pool, id, &req.title, &req.description).await {
        Ok(rows) => {
            if rows == 0 {
                return (StatusCode::NOT_FOUND, "Note not found").into_response();
            }
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update note").into_response()
        }
    }
}

pub async fn delete_note(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match Note::delete(&pool, id).await {
        Ok(rows) => {
            if rows == 0 {
                return (StatusCode::NOT_FOUND, "Note not found").into_response();
            }
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => {
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete note").into_response()
        }
    }
}
