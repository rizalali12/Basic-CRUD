use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Testing {
    pub id: Uuid,
    pub todo: String,
    pub message: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, FromRow, Validate)]
pub struct CreateTesting {
    #[validate(length(min = 5, max = 100))]
    pub todo: String,

    #[validate(length(min = 5, max = 100))]
    pub message: String,
}

#[derive(Debug, Deserialize, FromRow, Validate)]
pub struct UpdateTesting {
    #[validate(length(min = 5, max = 100))]
    pub todo: String,

    #[validate(length(min = 5, max = 100))]
    pub message: String,

    pub id: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GetTestingWithPayload {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TestingPagination {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TestingPaginated {
    pub data: Vec<Testing>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
