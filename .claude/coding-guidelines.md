# Rust Coding Guidelines

Guidelines for Rust development in the Scho1ar Backend project.

---

## Project Conventions

### Module Structure

```rust
// In mod.rs or lib.rs - order modules alphabetically
pub mod config;
pub mod db;
pub mod error;
pub mod routes;
```

### Import Ordering

```rust
// 1. Standard library
use std::net::SocketAddr;

// 2. External crates (blank line after std)
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// 3. Crate-local imports (blank line after external)
use crate::{AppState, error::AppResult};
```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Structs | PascalCase | `CloudAccount`, `HealthResponse` |
| Traits | PascalCase | `IntoResponse`, `FromRow` |
| Functions | snake_case | `create_pool`, `health_check` |
| Constants | SCREAMING_SNAKE | `MAX_CONNECTIONS`, `DEFAULT_PORT` |
| Type aliases | PascalCase | `DbPool`, `AppResult` |
| Modules | snake_case | `cloud_accounts`, `health` |

---

## Axum Patterns

### Handler Signature

```rust
// Use State extractor for AppState
// Return AppResult<Json<T>> for JSON responses
pub async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Item>> {
    // ...
}
```

### Extractor Order

Extractors must be ordered by how they consume the request:

```rust
// CORRECT: Path/Query before Body
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<QueryParams>,
    Json(payload): Json<UpdatePayload>,  // Body last
) -> AppResult<Json<Item>> { ... }
```

### Router Definition

```rust
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(get_one).put(update).delete(delete))
}
```

---

## SQLx Patterns

### Query Macros

Prefer `query_as!` for type-safe queries:

```rust
// Compile-time checked query with automatic mapping
let items = sqlx::query_as!(
    Item,
    r#"SELECT id, name, created_at FROM items WHERE org_id = $1"#,
    org_id
)
.fetch_all(&state.db)
.await?;
```

### Nullable Columns

Use `as "field?"` for nullable columns:

```rust
sqlx::query_as!(
    User,
    r#"SELECT id, name, email as "email?" FROM users"#
)
```

### Transactions

```rust
let mut tx = state.db.begin().await?;

sqlx::query!("INSERT INTO items (name) VALUES ($1)", name)
    .execute(&mut *tx)
    .await?;

sqlx::query!("UPDATE counters SET count = count + 1")
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

---

## Error Handling

### Use the ? Operator

```rust
// GOOD: Propagate errors with ?
pub async fn get_item(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<Item>> {
    let item = sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))?;

    Ok(Json(item))
}
```

### Custom Error Variants

Add new variants to `AppError` when needed:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // Add #[from] for automatic conversion
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    // String message for context
    #[error("Validation error: {0}")]
    Validation(String),
}
```

---

## Structs and Serialization

### Request/Response Types

```rust
// Derive order: Debug, Clone (if needed), Serialize/Deserialize, sqlx traits
#[derive(Debug, Serialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
}
```

### Serde Attributes

```rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]  // Accept camelCase from frontend
pub struct CreateRequest {
    pub org_id: Uuid,
    pub display_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]  // Return camelCase to frontend
pub struct Response {
    pub created_at: DateTime<Utc>,
}
```

---

## Async Best Practices

### Avoid Blocking in Async

```rust
// BAD: Blocking call in async context
let data = std::fs::read_to_string("file.txt")?;

// GOOD: Use tokio's async fs
let data = tokio::fs::read_to_string("file.txt").await?;
```

### Concurrent Operations

```rust
use tokio::try_join;

// Run independent queries concurrently
let (users, items) = try_join!(
    sqlx::query_as!(User, "SELECT * FROM users").fetch_all(&state.db),
    sqlx::query_as!(Item, "SELECT * FROM items").fetch_all(&state.db),
)?;
```

---

## Type Safety

### Newtype Pattern for IDs

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct OrganizationId(Uuid);

impl OrganizationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
```

### Builder Pattern for Complex Structs

```rust
#[derive(Default)]
pub struct QueryBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    filter: Option<String>,
}

impl QueryBuilder {
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn build(self) -> Query { ... }
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // Test pure functions
    }

    #[tokio::test]
    async fn test_async_function() {
        // Test async code
    }
}
```

### Integration Tests with Test Database

```rust
// tests/api_tests.rs
use sqlx::PgPool;

#[sqlx::test]
async fn test_create_item(pool: PgPool) {
    // Pool is automatically created and cleaned up
}
```

---

## Common Pitfalls

### 1. Forgetting `.await`

```rust
// WRONG: Returns Future, not result
let items = sqlx::query_as!(Item, "SELECT * FROM items").fetch_all(&pool);

// CORRECT
let items = sqlx::query_as!(Item, "SELECT * FROM items").fetch_all(&pool).await?;
```

### 2. Clone vs Reference

```rust
// Prefer references when possible
fn process(config: &Config) { ... }  // Borrow

// Clone only when ownership transfer is needed
let state = AppState::new(pool, config.clone());
```

### 3. String Allocation

```rust
// BAD: Unnecessary allocation
fn get_status() -> String {
    "ok".to_string()
}

// GOOD: Return static str when possible
fn get_status() -> &'static str {
    "ok"
}
```

### 4. Mutex in Async Code

```rust
// BAD: std::sync::Mutex blocks the async runtime
use std::sync::Mutex;

// GOOD: Use tokio's async mutex
use tokio::sync::Mutex;
```

---

## Clippy Lints

Run `cargo clippy` and address all warnings. Common lints to watch for:

- `clippy::unwrap_used` - Use `?` or `expect()` with context
- `clippy::clone_on_ref_ptr` - Avoid unnecessary Arc clones
- `clippy::large_enum_variant` - Box large variants
- `clippy::needless_pass_by_value` - Take references instead
