# Create New Route Module

Scaffold a new route module following project conventions.

## Arguments

- `$ARGUMENTS` - Name of the route module (e.g., "organizations", "cloud_accounts")

## Instructions

### 1. Read Project Guidelines

First, read the coding guidelines to ensure compliance:
```
.claude/coding-guidelines.md
.claude/filemap.md
```

### 2. Create Route Handler File

Create `src/routes/{module_name}.rs` with this template:

```rust
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppResult, AppState};

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Serialize)]
pub struct {Entity} {
    pub id: Uuid,
    // Add fields
}

#[derive(Debug, Deserialize)]
pub struct Create{Entity}Request {
    // Add fields
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(get_one).put(update).delete(delete))
}

// ============================================================================
// Handlers
// ============================================================================

async fn list(State(state): State<AppState>) -> AppResult<Json<Vec<{Entity}>>> {
    todo!("Implement list")
}

async fn create(
    State(state): State<AppState>,
    Json(payload): Json<Create{Entity}Request>,
) -> AppResult<Json<{Entity}>> {
    todo!("Implement create")
}

async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<{Entity}>> {
    todo!("Implement get_one")
}

async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Create{Entity}Request>,
) -> AppResult<Json<{Entity}>> {
    todo!("Implement update")
}

async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    todo!("Implement delete")
}
```

### 3. Register in routes/mod.rs

Add to `src/routes/mod.rs`:

```rust
pub mod {module_name};
```

And in `api_routes()`:

```rust
.nest("/{url-path}", {module_name}::router())
```

### 4. Update Documentation

Update `.claude/filemap.md`:
- Add new file to "Routes" table
- Update directory structure

### 5. Summary

Output what was created:
```
Created route module: {module_name}
- src/routes/{module_name}.rs
- Updated src/routes/mod.rs
- Updated .claude/filemap.md

Endpoints:
- GET    /api/{url-path}      - List all
- POST   /api/{url-path}      - Create new
- GET    /api/{url-path}/:id  - Get one
- PUT    /api/{url-path}/:id  - Update
- DELETE /api/{url-path}/:id  - Delete
```
