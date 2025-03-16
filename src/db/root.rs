use sqlx::{Error, PgPool, Pool, Postgres};
use tracing::{error, info};

pub async fn connect_db() -> Result<(), Error> {
    let url = "postgres://admin:admin@localhost:5432/axum_db";
    match PgPool::connect(url).await {
        Ok(pool) => {
            info!("✅ Connected to database");
            run_migrations(pool).await?;
        }
        Err(e) => {
            error!("Error connecting to {}: {}", url, e);
            return Err(e);
        }
    }

    Ok(())
}

pub async fn run_migrations(pool: Pool<Postgres>) -> Result<(), Error> {
    sqlx::migrate!("./src/db/migrations").run(&pool).await?;
    Ok(())
}