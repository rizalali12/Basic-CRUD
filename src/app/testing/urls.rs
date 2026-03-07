use super::views;
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;

pub fn configure(cfg: &mut web::ServiceConfig, pool: PgPool) {
    let json_cfg = web::JsonConfig::default()
        .error_handler(|err, _req| {
            let response = HttpResponse::BadRequest().json(json!({
                "error": "BAD_REQUEST",
                "message": err.to_string()
            }));
            actix_web::error::InternalError::from_response(err, response).into()
        });

    cfg.service(
        web::scope("/testing")
            .app_data(web::Data::new(pool))
            .app_data(json_cfg)
            .route("", web::get().to(views::list))
            .route("", web::post().to(views::create))
            .route("", web::put().to(views::update))
            .route("/detailwithpayload", web::get().to(views::detail_with_payload))
            .route("/{id}", web::get().to(views::detail))
            .route("/{id}", web::delete().to(views::delete))
    );
}
