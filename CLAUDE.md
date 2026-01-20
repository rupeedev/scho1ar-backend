# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## IMPORTANT: Required Reading Before Any Task

**Before starting any development task, you MUST read the following documentation files:**

| Document | Path | Purpose |
|----------|------|---------|
| **File Map** | `.claude/filemap.md` | Project structure, file locations, where to find things |
| **Coding Guidelines** | `.claude/coding-guidelines.md` | Rust patterns, Axum/SQLx conventions, best practices |
| **Tech Stack** | `.claude/techstack.md` | Technology choices, dependencies, rationale |
| **Lessons Learned** | `.claude/lessons-learned.md` | Critical bugs and solutions - avoid repeating mistakes |
| **API Standards** | `.claude/api-standards.md` | Response formats, HTTP codes, endpoint patterns, pagination |
| **Security Guidelines** | `.claude/security-guidelines.md` | Input validation, auth, SQL injection prevention, error handling |

Read these files to understand the codebase structure, coding standards, and known pitfalls before making any changes.

---

## Project Overview

Scho1ar Backend is a Rust API for cloud cost management built with Axum, SQLx, and Tokio. It handles organizations, cloud accounts, resources, costs, and schedules for AWS infrastructure.

## Common Commands

```bash
# Development
cargo run                    # Run dev server (port 3001)
cargo watch -x run           # Run with auto-reload (requires cargo-watch)
cargo check                  # Fast compilation check
cargo build --release        # Build optimized binary

# Testing & Quality
cargo test                   # Run all tests
cargo test <name>            # Run specific test
cargo clippy                 # Lint
cargo fmt                    # Format code

# Database (PostgreSQL must be running)
docker run -d --name scho1ar-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=scho1ar -p 5432:5432 postgres:16

# Background Server
nohup ./target/release/scho1ar-backend > server.log 2>&1 &
pkill scho1ar-backend        # Stop server

# Logging (set RUST_LOG env var)
RUST_LOG=debug cargo run     # Debug logging
RUST_LOG=scho1ar_backend=trace,tower_http=debug cargo run  # Granular control
```

## Architecture

### Application Flow
```
main.rs → Config::from_env() → db::create_pool() → AppState → routes::create_router() → axum::serve()
```

### Key Components

| File | Purpose |
|------|---------|
| `src/main.rs` | Entry point, CORS setup, server startup |
| `src/lib.rs` | `AppState` struct (holds DB pool + config) |
| `src/config.rs` | `Config` struct loaded from env vars |
| `src/db.rs` | SQLx PostgreSQL connection pool |
| `src/error.rs` | `AppError` enum with `IntoResponse` impl |
| `src/routes/mod.rs` | Router builder, nests `/api` routes |

### Adding New Routes

1. Create handler in `src/routes/` (e.g., `organizations.rs`)
2. Add `pub mod organizations;` to `src/routes/mod.rs`
3. Nest routes in `api_routes()` function:
```rust
fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/organizations", organizations::router())
}
```

### Handler Pattern
```rust
use axum::{extract::State, Json};
use crate::{AppState, error::AppResult};

pub async fn list(State(state): State<AppState>) -> AppResult<Json<Vec<Item>>> {
    let items = sqlx::query_as!(Item, "SELECT * FROM items")
        .fetch_all(&state.db)
        .await?;
    Ok(Json(items))
}
```

### Error Handling

Use `AppResult<T>` as return type. Available error variants in `src/error.rs`:
- `AppError::Database(sqlx::Error)` - auto-converts via `#[from]`
- `AppError::NotFound(String)` - 404 responses
- `AppError::BadRequest(String)` - 400 responses
- `AppError::Unauthorized` - 401 responses
- `AppError::Internal(String)` - 500 responses

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | PostgreSQL connection string |
| `HOST` | No | `0.0.0.0` | Server host |
| `PORT` | No | `3001` | Server port |
| `NODE_ENV` | No | `development` | Environment mode |
| `CORS_ORIGINS` | No | `http://localhost:3000,http://localhost:5173` | Comma-separated allowed origins |

## Critical: CORS with Credentials

Never use `tower_http::cors::Any` with `allow_credentials(true)`. Use explicit headers:

```rust
// CORRECT
.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT, header::ORIGIN])
.allow_credentials(true);
```

See `.claude/lessons-learned.md` for details.

## Task Tracking (iKanban MCP)

Use the iKanban MCP to log tasks for this project. **Team: SCO (Scho1ar)**

### Create Task
```bash
python3 /Users/rupeshpanwar/Documents/docs/common-mcp/ikanban.py create SCO "<title>" \
  --project "<epic-uuid>" \
  --status <todo|inprogress|done> \
  --priority <medium|high|urgent> \
  -d "<description>"
```

### Common Epics (Project UUIDs)
| Epic | UUID |
|------|------|
| env-setup | `e904deb9-e4f1-4a82-89ca-e49d75d1018a` |
| backend | `ec364e49-b620-48e1-9dd1-8744eaedb5e2` |
| frontend | `ff89ece5-eb49-4d8b-a349-4fc227773cbc` |
| infra | `80ff36dc-4367-4a3a-a9a8-343eb1c5865f` |

### Other Commands
```bash
python3 /Users/rupeshpanwar/Documents/docs/common-mcp/ikanban.py issues SCO           # List issues
python3 /Users/rupeshpanwar/Documents/docs/common-mcp/ikanban.py issues SCO -s inprogress  # Filter by status
python3 /Users/rupeshpanwar/Documents/docs/common-mcp/ikanban.py task SCO-<number>    # View task
python3 /Users/rupeshpanwar/Documents/docs/common-mcp/ikanban.py update <task-id> --status done  # Update
```

---

## Reminder

Always consult `.claude/` documentation before making changes:
- Check `filemap.md` to locate relevant files
- Follow patterns in `coding-guidelines.md`
- Review `lessons-learned.md` to avoid known issues
- Use `api-standards.md` for response formats and endpoint patterns
- Follow `security-guidelines.md` for input validation and auth
