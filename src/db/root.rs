use sqlx::{Error, PgPool, Pool, Postgres};
use tracing::{error, info};
use crate::env::get_env;

pub async fn connect_db() -> Result<PgPool, Error> {
    let url = get_env("POSTGRES_URL");
    let pool = match PgPool::connect(&url).await {
        Ok(pool) => {
            info!("✅ Connected to database");
            run_migrations(pool.clone()).await?;
            pool
        }
        Err(e) => {
            error!("Error connecting to {}: {}", &url, e);
            return Err(e);
        }
    };

    Ok(pool)
}

pub async fn run_migrations(pool: Pool<Postgres>) -> Result<(), Error> {
    // Only “up” are migrated, “down” are present but will not be migrated unless explicitly stated.
    sqlx::migrate!("./src/db/migrations").run(&pool).await?;
    Ok(())
}