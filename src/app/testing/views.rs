use super::{
    errors::TestingError,
    models::{
        CreateTesting, GetTestingWithPayload, TestingPaginated, TestingPagination, UpdateTesting,
    },
};
use crate::app::testing::models::Testing;
use crate::app::utils::{ApiResponse, no_content};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

// GET /testing
pub async fn list(
    pool: web::Data<PgPool>,
    query: web::Query<TestingPagination>,
) -> Result<HttpResponse, TestingError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * per_page;
    let search = query.search.as_ref().map(|s| format!("%{}%", s));

    let data = sqlx::query_as!(
        Testing,
        r#"
            SELECT id, todo, message, is_active, created_at, updated_at FROM testing 
            WHERE ($1::text IS NULL OR todo ILIKE $1 OR message ILIKE $1)
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
        "#,
        search,
        per_page as i64,
        offset as i64
    )
    .fetch_all(pool.get_ref())
    .await?;

    let total = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "total!: i64" FROM testing WHERE ($1::text IS NULL OR todo ILIKE $1 OR message ILIKE $1)"#,
        search
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(TestingPaginated {
        data,
        total,
        page,
        per_page,
    }))
}

// GET /testing/{id}
pub async fn detail(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, TestingError> {
    let id = path.into_inner();

    let testing = sqlx::query_as!(
        Testing,
        "SELECT id, todo, message, is_active, created_at, updated_at FROM testing WHERE id = $1",
        id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| TestingError::NotFound(id.to_string()))?;

    Ok(HttpResponse::Ok().json(testing))
}

// GET /testing/detailwithpayload
pub async fn detail_with_payload(
    pool: web::Data<PgPool>,
    body: web::Json<GetTestingWithPayload>,
) -> Result<HttpResponse, TestingError> {
    body.validate()
        .map_err(|e: validator::ValidationErrors| TestingError::ValidationError(e.to_string()))?;

    let detail = sqlx::query_as!(
        Testing,
        "SELECT id, todo, message, is_active, created_at, updated_at FROM testing WHERE id = $1",
        body.id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| TestingError::NotFound(body.id.to_string()))?;

    Ok(HttpResponse::Ok().json(detail))
}

// POST /testing
pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateTesting>,
) -> Result<HttpResponse, TestingError> {
    body.validate()
        .map_err(|e: validator::ValidationErrors| TestingError::ValidationError(e.to_string()))?;

    sqlx::query!(
        r#"
            INSERT INTO testing (id, todo, message)
            VALUES (gen_random_uuid(), $1, $2)
        "#,
        body.todo,
        body.message
    )
    .execute(pool.get_ref())
    .await?;

    Ok(ApiResponse::<()>::created("Berhasil bikin todo list"))
}

// PUT /testing
pub async fn update(
    pool: web::Data<PgPool>,
    body: web::Json<UpdateTesting>,
) -> Result<HttpResponse, TestingError> {
    body.validate()
        .map_err(|e: validator::ValidationErrors| TestingError::ValidationError(e.to_string()))?; // ← tambah :

    let exist = sqlx::query_scalar!("SELECT id FROM testing WHERE id = $1", body.id)
        .fetch_optional(pool.get_ref())
        .await?;

    if exist.is_none() {
        return Err(TestingError::NotFound(body.id.to_string()));
    }

    sqlx::query!(
        r#"UPDATE testing SET todo = $1, message = $2 WHERE id = $3"#,
        body.todo,
        body.message,
        body.id
    )
    .execute(pool.get_ref())
    .await?;

    Ok(ApiResponse::<()>::success("Berhasil update", ()))
}

//DELETE /{id}
pub async fn delete(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, TestingError> {
    let id = path.into_inner();

    let exist = sqlx::query_scalar!("SELECT id FROM testing WHERE id = $1", id)
        .fetch_optional(pool.get_ref())
        .await?;
    if exist.is_none() {
        return Err(TestingError::NotFound(id.to_string()));
    }

    sqlx::query!("DELETE FROM testing WHERE id = $1", id)
        .execute(pool.get_ref())
        .await?;

    Ok(ApiResponse::<()>::success("Berhasil Delete", ()))
}
