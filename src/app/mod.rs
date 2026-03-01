use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::PgPool;

pub mod testing;
pub mod utils;

#[derive(Serialize)]
struct healthCheckBody {
    status: String,
    message: String
}

async fn health_check() -> impl Responder{
    let obj = healthCheckBody {
        status: "200".to_string(),
        message: "OK".to_string(),
    };
HttpResponse::Ok().json(obj)
}

pub fn configure(cfg: &mut web::ServiceConfig, pool: PgPool){
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_check))
            .configure(|c| testing::urls::configure(c, pool.clone()))
    );
}
