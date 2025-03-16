use std::error::Error;
use sqlx::{PgPool};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub description: String,
}


impl Note {
    pub async fn save_note(pool: &PgPool, title: &str, description: &str) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO notes (title, description) VALUES ($1, $2)";
        sqlx::query(query).bind(title).bind(description).execute(pool).await?;

        Ok(())
    }
}