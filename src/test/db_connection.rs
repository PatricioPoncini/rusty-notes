#[cfg(test)]
mod db_connection_test {
    use crate::test::utils::create_pool_connection;
    use std::env;

    #[tokio::test]
    #[should_panic]
    async fn connection_failure() {
        let database_url = env::var("WRONG_DATABASE_URL").unwrap_or("".to_string());
        create_pool_connection(database_url).await;
    }

    #[tokio::test]
    async fn connection_success() {
        let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());
        create_pool_connection(database_url).await;
    }
}
