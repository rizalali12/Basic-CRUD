use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        })
    }

    pub fn created(message: &str) -> HttpResponse {
        HttpResponse::Created().json(Self {
            success: true,
            message: message.to_string(),
            data: None,
        })
    }
}

pub fn no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
