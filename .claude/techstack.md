# Tech Stack

Technology choices and rationale for Scho1ar Backend.

---

## Core Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| **Rust** | 1.75+ | Systems programming language |
| **Axum** | 0.7 | Web framework |
| **Tokio** | 1.x | Async runtime |
| **SQLx** | 0.8 | Database toolkit |
| **PostgreSQL** | 16 | Primary database |

---

## Dependencies

### Web Framework

| Crate | Purpose |
|-------|---------|
| `axum` | HTTP routing, extractors, middleware |
| `axum-extra` | Additional extractors (typed headers) |
| `tower` | Service abstraction, middleware traits |
| `tower-http` | HTTP-specific middleware (CORS, tracing) |

**Why Axum?**
- Built on `tower` ecosystem (composable middleware)
- Type-safe extractors with compile-time guarantees
- First-class async/await support
- Excellent performance characteristics
- Active development by Tokio team

### Database

| Crate | Purpose |
|-------|---------|
| `sqlx` | Async PostgreSQL driver with compile-time query checking |

**SQLx Features Enabled:**
- `runtime-tokio` - Tokio async runtime integration
- `postgres` - PostgreSQL driver
- `uuid` - UUID type support
- `chrono` - DateTime type support
- `json` - JSONB column support
- `migrate` - Database migration support

**Why SQLx?**
- Compile-time SQL verification (catches errors before runtime)
- No ORM overhead - write raw SQL
- Async-first design
- Built-in connection pooling
- Migration support

### Serialization

| Crate | Purpose |
|-------|---------|
| `serde` | Serialization framework |
| `serde_json` | JSON serialization |

### Authentication

| Crate | Purpose |
|-------|---------|
| `jsonwebtoken` | JWT encoding/decoding and validation (RS256, RS384, RS512) |
| `reqwest` | HTTP client for fetching JWKS from Clerk |

**Clerk JWT Integration:**
- JWKS-based key fetching with 1-hour cache TTL
- RSA public key validation
- Claims extraction (user ID, organization ID, roles)
- Middleware-based route protection

### Utilities

| Crate | Purpose |
|-------|---------|
| `uuid` | UUID v4 generation and parsing |
| `chrono` | Date/time handling |
| `dotenvy` | Environment variable loading from `.env` |
| `thiserror` | Derive macro for error types |
| `tracing` | Structured logging/diagnostics |
| `tracing-subscriber` | Log formatting and filtering |
| `once_cell` | Lazy static initialization |
| `validator` | Request payload validation with derive macros |

---

## Architecture Decisions

### Why Rust?

1. **Memory Safety** - No null pointers, no data races
2. **Performance** - Zero-cost abstractions, no GC pauses
3. **Type System** - Catch errors at compile time
4. **Async Model** - Efficient handling of concurrent connections
5. **Ecosystem** - Mature crates for web development

### Why PostgreSQL?

1. **JSONB Support** - Flexible schema for cloud resource metadata
2. **UUID Native Type** - First-class UUID support
3. **Concurrent Performance** - MVCC for high read/write throughput
4. **Extensions** - pg_cron for scheduled jobs, PostGIS if needed
5. **SQLx Integration** - Excellent compile-time query support

### Async Runtime: Tokio

- Industry standard for Rust async
- Work-stealing scheduler for efficient CPU utilization
- Full async I/O (network, filesystem)
- Rich ecosystem of async crates

---

## Planned Additions

| Technology | Purpose | Status |
|------------|---------|--------|
| `aws-sdk-rust` | AWS API integration | Planned |
| `redis` | Caching layer | Planned |

### Recently Implemented

| Technology | Purpose | Implemented |
|------------|---------|-------------|
| `validator` | Request payload validation with derive macro | 2026-01-20 |
| `jsonwebtoken` | JWT authentication (Clerk) | 2026-01-20 |
| `reqwest` | HTTP client (JWKS fetching) | 2026-01-20 |
| `axum-extra` | Typed header extractors | 2026-01-20 |

---

## Development Tools

| Tool | Purpose |
|------|---------|
| `cargo-watch` | Auto-reload on file changes |
| `cargo-clippy` | Linting |
| `rustfmt` | Code formatting |
| `sqlx-cli` | Database migrations |

### Install Development Tools

```bash
# Auto-reload
cargo install cargo-watch

# SQLx CLI for migrations
cargo install sqlx-cli --no-default-features --features postgres

# Run with auto-reload
cargo watch -x run
```

---

## Version Compatibility

```toml
# Minimum Supported Rust Version
rust-version = "1.75"

# Edition
edition = "2021"
```

### Dependency Version Policy

- **Major versions**: Pin to specific major (e.g., `axum = "0.7"`)
- **Security updates**: Allow patch updates via Cargo.lock
- **Breaking changes**: Review changelogs before major upgrades
