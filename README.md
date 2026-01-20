# Scho1ar Backend (Rust)

Cloud cost management API built with Rust.

## Tech Stack

- **Framework**: Axum
- **Runtime**: Tokio
- **Database**: PostgreSQL via SQLx
- **Serialization**: Serde

## Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- PostgreSQL 14+
- Docker (optional, for local database)

## Quick Start

### 1. Clone and setup

```bash
cd scho1ar-backend
cp .env.example .env
# Edit .env with your database credentials
```

### 2. Start PostgreSQL (if using Docker)

```bash
docker run -d \
  --name scho1ar-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=scho1ar \
  -p 5432:5432 \
  postgres:16
```

### 3. Run the server

```bash
# Development mode with auto-reload
cargo run

# Or with release optimizations
cargo run --release
```

Server will start at `http://localhost:3001`

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check with DB status |
| GET | `/ready` | Readiness probe |
| GET | `/api/` | API version info |

## Project Structure

```
src/
├── main.rs       # Entry point, server startup
├── lib.rs        # Library root, AppState
├── config.rs     # Environment configuration
├── db.rs         # Database connection pool
├── error.rs      # Error types and handling
└── routes/
    ├── mod.rs    # Router configuration
    └── health.rs # Health check endpoints
```

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | PostgreSQL connection string |
| `HOST` | No | `0.0.0.0` | Server host |
| `PORT` | No | `3001` | Server port |
| `NODE_ENV` | No | `development` | Environment mode |
| `CORS_ORIGINS` | No | `localhost:3000,5173` | Allowed origins |

## Roadmap

- [ ] Database migrations (SQLx migrate)
- [ ] Authentication (JWT/Supabase)
- [ ] Organizations CRUD
- [ ] Cloud Accounts CRUD
- [ ] AWS SDK integration
- [ ] Resource discovery
- [ ] Cost tracking
- [ ] Schedules
