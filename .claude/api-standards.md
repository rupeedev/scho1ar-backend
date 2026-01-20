# API Standards

This document defines the API standards and conventions for the Scho1ar backend built with Rust, Axum, and SQLx.

## API Architecture Overview

### Base Configuration

```
Base URL: http://localhost:3001 (development)
Authentication: Bearer token (JWT)
Authorization: Organization-scoped access
Content-Type: application/json for all request/response bodies
```

### Request Flow

```
HTTP Client → Axum Router → Middleware (Auth/CORS) → Handler → Service → SQLx → PostgreSQL
      ↓                                                                              ↓
  Response  ←────────────────────────────────────────────────────────────────────────┘
```

---

## Response Standards

### Success Response Format

All successful responses MUST follow this structure:

```json
{
  "data": {},
  "message": "Operation completed successfully",
  "timestamp": "2024-01-01T00:00:00.000Z"
}
```

**Rust Implementation:**

```rust
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            message: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            message: Some(message.into()),
            timestamp: Utc::now(),
        }
    }
}
```

### Error Response Format

All error responses MUST follow this structure:

```json
{
  "statusCode": 400,
  "error": "Bad Request",
  "message": "Detailed error description",
  "path": "/api/endpoint",
  "timestamp": "2024-01-01T00:00:00.000Z"
}
```

**Rust Implementation:**

```rust
use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub timestamp: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
        };

        let body = ErrorResponse {
            status_code: status.as_u16(),
            error: status.canonical_reason().unwrap_or("Error").to_string(),
            message: error_message,
            path: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        (status, Json(body)).into_response()
    }
}
```

### Pagination Response Format

Paginated endpoints MUST return this structure:

```json
{
  "data": [],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 100,
    "totalPages": 10,
    "hasNext": true,
    "hasPrevious": false
  }
}
```

**Rust Implementation:**

```rust
#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "hasNext")]
    pub has_next: bool,
    #[serde(rename = "hasPrevious")]
    pub has_previous: bool,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

impl PaginationMeta {
    pub fn new(page: u32, limit: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        Self {
            page,
            limit,
            total,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }
}
```

---

## HTTP Status Codes

### Success Codes

| Code | Name | Usage |
|------|------|-------|
| 200 | OK | Successful GET, PUT, PATCH, DELETE |
| 201 | Created | Successful POST creating a resource |
| 204 | No Content | Successful operation with no response body |

### Client Error Codes

| Code | Name | Usage |
|------|------|-------|
| 400 | Bad Request | Invalid request parameters or body |
| 401 | Unauthorized | Missing or invalid authentication |
| 403 | Forbidden | Valid auth but insufficient permissions |
| 404 | Not Found | Resource does not exist |
| 409 | Conflict | Resource conflict (e.g., duplicate) |
| 422 | Unprocessable Entity | Validation errors |

### Server Error Codes

| Code | Name | Usage |
|------|------|-------|
| 500 | Internal Server Error | Generic server error |
| 502 | Bad Gateway | Upstream service error |
| 503 | Service Unavailable | Temporary service unavailable |

---

## Endpoint Naming Conventions

### URL Structure

```
/api/{resource}                    # Collection
/api/{resource}/{id}               # Individual resource
/api/{resource}/{id}/{sub-resource} # Nested resource
/api/organizations/{orgId}/{resource} # Organization-scoped
```

### HTTP Methods

| Method | Purpose | Example |
|--------|---------|---------|
| GET | Read resource(s) | `GET /api/resources` |
| POST | Create resource | `POST /api/resources` |
| PUT | Full update | `PUT /api/resources/{id}` |
| PATCH | Partial update | `PATCH /api/resources/{id}` |
| DELETE | Remove resource | `DELETE /api/resources/{id}` |

### Action Endpoints

For non-CRUD operations, use verb-based sub-paths:

```
POST /api/resources/{id}/sync      # Trigger sync
POST /api/resources/{id}/enable    # Enable resource
POST /api/resources/{id}/disable   # Disable resource
```

---

## Request Validation

### Query Parameters

Use `serde` with validation for query parameters:

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ListQuery {
    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    pub limit: u32,

    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default)]
    pub search: Option<String>,

    #[serde(default)]
    pub sort_by: Option<String>,

    #[serde(default = "default_sort_order")]
    pub sort_order: SortOrder,
}

fn default_limit() -> u32 { 20 }
fn default_page() -> u32 { 1 }
fn default_sort_order() -> SortOrder { SortOrder::Desc }
```

### Request Body Validation

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(max = 500))]
    #[serde(default)]
    pub description: Option<String>,
}

// In handler:
pub async fn create_organization(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrganizationRequest>,
) -> AppResult<Json<Organization>> {
    payload.validate()
        .map_err(|e| AppError::BadRequest(format!("Validation failed: {}", e)))?;

    // ... create logic
}
```

---

## Authentication & Authorization

### JWT Token Structure

```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "org_id": "organization-uuid",
  "role": "admin",
  "exp": 1234567890,
  "iat": 1234567890
}
```

### Authorization Header

```
Authorization: Bearer <jwt-token>
```

### Organization Context

Organization-scoped endpoints receive org context via:
1. URL path parameter: `/organizations/{orgId}/resources`
2. JWT claims (preferred for security)

```rust
// Extract from JWT claims in middleware
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub org_id: Option<Uuid>,
    pub role: UserRole,
}

// Use in handler
pub async fn list_resources(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(org_id): Path<Uuid>,
) -> AppResult<Json<Vec<Resource>>> {
    // Verify user has access to org_id
    if auth.org_id != Some(org_id) {
        return Err(AppError::Forbidden("Access denied".into()));
    }
    // ... list logic
}
```

---

## Handler Pattern

### Standard Handler Structure

```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

/// List resources with pagination and filtering
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> AppResult<Json<PaginatedResponse<Resource>>> {
    let resources = sqlx::query_as!(
        Resource,
        r#"
        SELECT * FROM resources
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        params.limit as i64,
        ((params.page - 1) * params.limit) as i64,
    )
    .fetch_all(&state.db)
    .await?;

    let total = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM resources"
    )
    .fetch_one(&state.db)
    .await?
    .unwrap_or(0) as u64;

    Ok(Json(PaginatedResponse {
        data: resources,
        pagination: PaginationMeta::new(params.page, params.limit, total),
    }))
}

/// Get single resource by ID
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Resource>> {
    let resource = sqlx::query_as!(
        Resource,
        "SELECT * FROM resources WHERE id = $1",
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Resource {} not found", id)))?;

    Ok(Json(resource))
}

/// Create new resource
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateResourceRequest>,
) -> AppResult<(StatusCode, Json<Resource>)> {
    payload.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let resource = sqlx::query_as!(
        Resource,
        r#"
        INSERT INTO resources (name, description)
        VALUES ($1, $2)
        RETURNING *
        "#,
        payload.name,
        payload.description,
    )
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(resource)))
}

/// Update existing resource
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateResourceRequest>,
) -> AppResult<Json<Resource>> {
    let resource = sqlx::query_as!(
        Resource,
        r#"
        UPDATE resources
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
        id,
        payload.name,
        payload.description,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Resource {} not found", id)))?;

    Ok(Json(resource))
}

/// Delete resource
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let result = sqlx::query!(
        "DELETE FROM resources WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Resource {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
```

---

## Router Organization

### Module Router Pattern

```rust
// src/routes/organizations.rs
use axum::{routing::{get, post, put, delete}, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(get_by_id).put(update).delete(delete))
        .route("/:id/members", get(list_members).post(add_member))
}

// src/routes/mod.rs
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/organizations", organizations::router())
        .nest("/api/resources", resources::router())
        .nest("/api/cloud-accounts", cloud_accounts::router())
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

---

## Background Jobs / Async Operations

For long-running operations, return immediately with job status:

```json
{
  "success": true,
  "message": "Sync job created and queued for processing",
  "jobId": "uuid",
  "status": "pending",
  "progress": 0,
  "progressMessage": "Job queued"
}
```

**Handler Pattern:**

```rust
pub async fn trigger_sync(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
) -> AppResult<(StatusCode, Json<SyncJobResponse>)> {
    let job = sqlx::query_as!(
        SyncJob,
        r#"
        INSERT INTO sync_jobs (cloud_account_id, status, progress)
        VALUES ($1, 'pending', 0)
        RETURNING *
        "#,
        account_id,
    )
    .fetch_one(&state.db)
    .await?;

    // Spawn background task
    tokio::spawn(process_sync_job(state.clone(), job.id));

    Ok((StatusCode::ACCEPTED, Json(SyncJobResponse {
        success: true,
        message: "Sync job created and queued for processing".into(),
        job_id: job.id,
        status: "pending".into(),
        progress: 0,
        progress_message: "Job queued".into(),
    })))
}
```

---

## Best Practices Summary

### DO:
1. Use consistent response formats across all endpoints
2. Validate all inputs with proper error messages
3. Use organization scoping for multi-tenant data
4. Return appropriate HTTP status codes
5. Include timestamps in all responses
6. Use UUIDs for resource identifiers
7. Implement proper pagination for list endpoints
8. Log all errors with context (but sanitize sensitive data)

### DON'T:
1. Expose internal error details to clients
2. Use sequential IDs (security risk)
3. Return unbounded lists (always paginate)
4. Mix authentication concerns in handlers
5. Skip validation for "trusted" inputs
6. Return 200 for errors
7. Include sensitive data in error messages
