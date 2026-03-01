use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;

mod db;
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = db::create_database_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database pool initialized");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(|cfg| app::configure(cfg, pool.clone()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
