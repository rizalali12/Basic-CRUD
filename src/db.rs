use sqlx::{postgres::PgPoolOptions, PgPool };

pub async fn create_database_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect(database_url)
        .await
}