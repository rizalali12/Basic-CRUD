use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestingError {
    #[error("Testing not found")]
    NotFound(String),

    #[error("Validation error")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorBody {
    success: bool,
    error: String,
    message: String,
}

impl ResponseError for TestingError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_code) = match self {
            TestingError::NotFound(_) => (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND"),
            TestingError::ValidationError(_) => {
                (actix_web::http::StatusCode::BAD_REQUEST, "BAD_REQUEST")
            }
            TestingError::DatabaseError(_) => {
                (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR")
            }
        };

        HttpResponse::build(status).json(ErrorBody {
            success: false,
            error: error_code.to_string(),
            message: self.to_string(),
        })
    }
}
