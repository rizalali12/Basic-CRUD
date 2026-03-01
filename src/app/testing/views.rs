use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use super::{
    errors::TestingError,
    models::{
        CreateTesting, UpdateTesting, TestingPagination, TestingPaginated
    },
};

use crate::app::utils::{no_content, ApiResponse};

// GET /testing
pub async fn list(pool: web::Data<PgPool>, query: web::Query<TestingPagination>) -> Result<HttpResponse, TestingError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1,100);
    let offset = (page - 1) * per_page;
    let search = query.search.as_ref().map(|s| format!("%{}%", s));

    let data = sqlx::query_as!(
        crate::app::testing::models::Testing,
        r#"
            SELECT id, todo, message, is_active, created_at, updated_at FROM testing 
            WHERE ($1::text IS NULL OR todo ILIKE $1 OR message ILIKE $1)
        ORDER BY created_at DESC 
        LIMIT $2 OFFSET $3
        "#,
        search,
        per_page,
        offset
    )
    .fetch_all(pool.get_ref())
    .await?;

    let total = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "total!: i64" FROM testing WHERE ($1::text IS NULL OR todo ILIKE $1 OR message ILIKE $1)"#,
        search
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(TestingPaginated {data, total, page, per_page}))
}

//POST /testing
pub async fn create(pool: web::Data<PgPool>, body: web::Json<CreateTesting>) -> Result<HttpResponse, TestingError> {
    body.validate()
        .map_err(|e: validator::ValidationErrors| TestingError::ValidationError(e.to_string()))?;

   let addTestingToDb = sqlx::query_as!(
       crate::app::testing::models::Testing,
       r#"
        INSERT INTO testing (id, todo, message)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING id, todo, message, is_active, created_at, updated_at
       "#,
       body.todo,
       body.message
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(ApiResponse::<()>::created("Berhasil bikin todo list"))
}




