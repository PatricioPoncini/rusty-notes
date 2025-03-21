use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[allow(dead_code)] // used in test cases
pub async fn create_pool_connection(database_url: String) -> Pool<Postgres> {
    let pool_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    return match pool_connection {
        Ok(pool) => pool,
        Err(err) => panic!("Connection failure: {:?}", err),
    };
}
