use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub description: String,
}

impl FromRow<'_, PgRow> for Note {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Note {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
        })
    }
}

impl Note {
    pub async fn save_note(
        pool: &PgPool,
        title: &str,
        description: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut txn = pool.begin().await?;
        let query = "INSERT INTO notes (title, description) VALUES ($1, $2)";
        sqlx::query(query)
            .bind(title)
            .bind(description)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let query = "SELECT id, title, description FROM notes";

        let notes = sqlx::query_as::<_, Note>(query).fetch_all(pool).await?;

        Ok(notes)
    }

    pub async fn update(
        pool: &PgPool,
        id: i32,
        title: &str,
        description: &str,
    ) -> Result<u64, Box<dyn Error>> {
        let mut txn = pool.begin().await?;

        let query = "UPDATE notes SET title = $1, description = $2 WHERE id = $3";
        let result = sqlx::query(query)
            .bind(title)
            .bind(description)
            .bind(id)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, Box<dyn Error>> {
        let mut txn = pool.begin().await?;

        let query = "DELETE FROM notes WHERE id = $1";
        let result = sqlx::query(query).bind(id).execute(&mut *txn).await?;

        txn.commit().await?;

        Ok(result.rows_affected())
    }
}
