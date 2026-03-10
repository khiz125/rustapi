use sqlx::{Pgpool, postgres::PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<Pgpool, sqlx::Error> {
    AnyPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
