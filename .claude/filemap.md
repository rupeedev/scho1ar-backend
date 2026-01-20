# File Map

Project structure and file reference for Scho1ar Backend.

---

## Directory Structure

```
scho1ar-backend/
├── .claude/                    # Claude Code documentation
│   ├── commands/               # Slash commands
│   │   ├── add-dependency.md   # /add-dependency - Add and document crate
│   │   ├── add-lesson.md       # /add-lesson - Document a lesson learned
│   │   ├── new-route.md        # /new-route - Scaffold route module
│   │   └── update-docs.md      # /update-docs - Update all documentation
│   ├── coding-guidelines.md    # Rust coding standards
│   ├── filemap.md              # This file
│   ├── lessons-learned.md      # Critical incidents & solutions
│   └── techstack.md            # Technology choices
├── migrations/                 # SQLx database migrations
│   └── YYYYMMDDHHMMSS_*.sql    # Timestamped migration files
├── src/                        # Source code
│   ├── auth/                   # Authentication & authorization
│   │   ├── mod.rs              # Auth module exports
│   │   ├── claims.rs           # JWT claims structure (Clerk-specific)
│   │   ├── jwks.rs             # JWKS fetching and caching
│   │   └── middleware.rs       # require_auth middleware & Claims extractor
│   ├── routes/                 # HTTP route handlers
│   │   ├── mod.rs              # Router configuration
│   │   └── health.rs           # Health check endpoints
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library root, AppState
│   ├── config.rs               # Environment configuration (includes ClerkConfig)
│   ├── db.rs                   # Database connection pool
│   └── error.rs                # Error types
├── target/                     # Build artifacts (git-ignored)
├── .env                        # Local environment (git-ignored)
├── .env.example                # Environment template
├── .gitignore                  # Git ignore rules
├── Cargo.toml                  # Rust dependencies
├── Cargo.lock                  # Locked dependency versions
├── CLAUDE.md                   # Claude Code instructions
└── README.md                   # Project documentation
```

---

## Source Files

### Entry Points

| File | Description |
|------|-------------|
| `src/main.rs` | Application entry point. Initializes tracing, loads config, connects to DB, configures CORS, starts Axum server. |
| `src/lib.rs` | Library crate root. Exports `AppState` struct and public modules. |

### Core Modules

| File | Description | Key Exports |
|------|-------------|-------------|
| `src/config.rs` | Environment variable parsing. Loads from `.env` via dotenvy. Includes Clerk JWT config. | `Config`, `ClerkConfig`, `ConfigError` |
| `src/db.rs` | PostgreSQL connection pool setup with SQLx. | `DbPool`, `create_pool()` |
| `src/error.rs` | Unified error handling. Implements `IntoResponse` for Axum. | `AppError`, `AppResult<T>` |

### Authentication

| File | Description | Key Exports |
|------|-------------|-------------|
| `src/auth/mod.rs` | Auth module root. Re-exports public types. | `Claims`, `require_auth`, `AuthenticatedUser` |
| `src/auth/claims.rs` | JWT claims structure with Clerk-specific fields (user ID, org ID, roles). | `Claims` |
| `src/auth/jwks.rs` | JWKS fetching and caching. Fetches Clerk public keys with 1-hour TTL. | `JwksCache`, `SharedJwksCache`, `create_jwks_cache()` |
| `src/auth/middleware.rs` | Auth middleware and Claims extractor. Validates JWTs against Clerk JWKS. | `require_auth`, `AuthError`, `AuthenticatedUser` |

### Routes

| File | Description | Endpoints |
|------|-------------|-----------|
| `src/routes/mod.rs` | Router builder. Mounts all route modules under `/api`. Protected routes use `require_auth` middleware. | `create_router()`, `GET /api/`, `GET /api/me` (protected) |
| `src/routes/health.rs` | Health and readiness probes. | `GET /health`, `GET /ready` |

---

## Configuration Files

| File | Description |
|------|-------------|
| `Cargo.toml` | Rust package manifest. Dependencies, features, metadata. |
| `Cargo.lock` | Locked dependency tree. Commit to ensure reproducible builds. |
| `.env.example` | Template for environment variables. Copy to `.env` for local dev. |
| `.env` | Local environment variables (git-ignored). |
| `.gitignore` | Files excluded from version control. |

---

## Documentation

| File | Description |
|------|-------------|
| `README.md` | Project overview, quick start, API endpoints, roadmap. |
| `CLAUDE.md` | Instructions for Claude Code. Commands, architecture, patterns. |
| `.claude/coding-guidelines.md` | Rust coding standards and Axum/SQLx patterns. |
| `.claude/techstack.md` | Technology choices and rationale. |
| `.claude/lessons-learned.md` | Critical bugs and their solutions. |

---

## Slash Commands

Available commands in `.claude/commands/`:

| Command | File | Purpose |
|---------|------|---------|
| `/update-docs` | `update-docs.md` | Update all `.claude/` documentation to reflect current project state |
| `/add-lesson` | `add-lesson.md` | Document a new lesson learned from a bug or issue |
| `/new-route` | `new-route.md` | Scaffold a new route module with CRUD handlers |
| `/add-dependency` | `add-dependency.md` | Add a crate and update techstack documentation |

---

## Planned Directories

Future modules to be added (single crate, multiple binaries):

```
scho1ar-backend/
├── src/
│   ├── bin/                        # Multiple binaries
│   │   ├── api.rs                  # Axum API server (cargo run --bin api)
│   │   └── worker.rs               # Temporal worker (cargo run --bin worker)
│   │
│   ├── auth/                       # ✅ IMPLEMENTED - Authentication & authorization
│   │   ├── mod.rs                  # Auth module exports
│   │   ├── claims.rs               # JWT claims (Clerk-specific)
│   │   ├── jwks.rs                 # JWKS fetching and caching
│   │   └── middleware.rs           # require_auth middleware + Claims extractor
│   │
│   ├── models/                     # Database models (shared)
│   │   ├── mod.rs
│   │   ├── organization.rs
│   │   ├── cloud_account.rs
│   │   ├── resource.rs
│   │   ├── cost_entry.rs
│   │   └── schedule.rs
│   │
│   ├── routes/                     # API handlers
│   │   ├── mod.rs
│   │   ├── health.rs               # /health, /ready
│   │   ├── organizations.rs        # /api/organizations
│   │   ├── cloud_accounts.rs       # /api/cloud-accounts
│   │   ├── resources.rs            # /api/resources
│   │   ├── costs.rs                # /api/costs
│   │   └── schedules.rs            # /api/schedules
│   │
│   ├── aws/                        # AWS SDK services (shared by API + Worker)
│   │   ├── mod.rs
│   │   ├── client.rs               # AWS client factory
│   │   ├── ec2.rs                  # EC2 operations
│   │   ├── rds.rs                  # RDS operations
│   │   ├── s3.rs                   # S3 operations
│   │   ├── lambda.rs               # Lambda operations
│   │   └── cost_explorer.rs        # Cost Explorer queries
│   │
│   ├── temporal/                   # Temporal integration
│   │   ├── mod.rs
│   │   ├── client.rs               # Temporal client (API uses to start workflows)
│   │   ├── workflows/              # Workflow definitions (Worker runs these)
│   │   │   ├── mod.rs
│   │   │   ├── sync.rs             # CloudAccountSyncWorkflow
│   │   │   └── schedule.rs         # ResourceScheduleWorkflow
│   │   └── activities/             # Activity implementations
│   │       ├── mod.rs
│   │       ├── aws.rs              # AWS SDK activities
│   │       └── db.rs               # Database activities
│   │
│   ├── services/                   # Business logic (shared)
│   │   ├── mod.rs
│   │   ├── cost_sync.rs            # Cost data sync
│   │   ├── cost_analytics.rs       # Aggregations, trends
│   │   ├── health_score.rs         # Cloud health calculation
│   │   └── optimization.rs         # Recommendations
│   │
│   ├── vault/                      # Credential encryption
│   │   ├── mod.rs
│   │   └── encryption.rs
│   │
│   ├── scheduler/                  # Schedule execution (before Temporal)
│   │   ├── mod.rs
│   │   ├── cron.rs
│   │   └── executor.rs
│   │
│   ├── main.rs                     # Default binary (API)
│   ├── lib.rs                      # Shared library
│   ├── config.rs
│   ├── db.rs
│   ├── error.rs
│   └── pagination.rs               # Pagination utilities
│
├── migrations/                     # SQLx migrations
│   ├── YYYYMMDD_create_organizations.sql
│   ├── YYYYMMDD_create_cloud_accounts.sql
│   ├── YYYYMMDD_create_resources.sql
│   ├── YYYYMMDD_create_cost_entries.sql
│   └── YYYYMMDD_create_schedules.sql
│
├── Cargo.toml                      # [[bin]] entries for api + worker
└── docker-compose.yml              # Temporal + PostgreSQL for local dev
```

**Binary configuration in Cargo.toml:**
```toml
[[bin]]
name = "api"
path = "src/bin/api.rs"

[[bin]]
name = "worker"
path = "src/bin/worker.rs"

[lib]
name = "scho1ar_backend"
path = "src/lib.rs"
```

**Run commands:**
```bash
cargo run                # Default: API server on :3001
cargo run --bin api      # Explicit: API server
cargo run --bin worker   # Temporal worker
```

---

## Quick Reference

### Where to find...

| Looking for... | Location |
|----------------|----------|
| Server startup | `src/main.rs:8` (`#[tokio::main]`) |
| CORS configuration | `src/main.rs:32-54` |
| Route registration | `src/routes/mod.rs:8` (`create_router`) |
| Protected routes setup | `src/routes/mod.rs:18` (`api_routes` with middleware) |
| Database pool config | `src/db.rs:7` (`create_pool`) |
| Environment loading | `src/config.rs:23` (`Config::from_env`) |
| Clerk JWT config | `src/config.rs:13` (`ClerkConfig` struct) |
| Error → HTTP response | `src/error.rs:26` (`impl IntoResponse`) |
| App shared state | `src/lib.rs:11` (`AppState` struct with JWKS cache) |
| JWT validation middleware | `src/auth/middleware.rs:79` (`require_auth`) |
| Claims extractor | `src/auth/middleware.rs:150` (`impl FromRequestParts`) |
| JWKS caching | `src/auth/jwks.rs:50` (`JwksCache`) |

### Adding new features

| Task | Files to modify |
|------|-----------------|
| New API route | Create `src/routes/<name>.rs`, add to `src/routes/mod.rs` |
| New protected route | Add to `protected_routes` in `src/routes/mod.rs`, use `Claims` extractor |
| New error type | Add variant to `src/error.rs` |
| New config option | Add field to `src/config.rs`, update `.env.example` |
| New database table | Add migration in `migrations/`, create model in `src/models/` |
| New AWS service | Create `src/aws/<service>.rs`, add to `src/aws/mod.rs` |
| New Temporal workflow | Create `src/temporal/workflows/<name>.rs`, register in worker |
| New Temporal activity | Create in `src/temporal/activities/`, add to activity impl |
