# Lessons Learned

Critical incidents and solutions to avoid repeating mistakes.

---

## 2026-01-20: CORS Configuration with Credentials

### Incident

Server crashed on startup with panic:

```
Invalid CORS configuration: Cannot combine `Access-Control-Allow-Credentials: true`
with `Access-Control-Allow-Headers: *`
```

### Root Cause

Used `tower_http::cors::Any` for allowed headers while also enabling `allow_credentials(true)`:

```rust
// WRONG - causes runtime panic
let cors = CorsLayer::new()
    .allow_headers(tower_http::cors::Any)  // This is the problem
    .allow_credentials(true);
```

### Why It Fails

The CORS specification (W3C) explicitly prohibits wildcard `*` values for:
- `Access-Control-Allow-Headers`
- `Access-Control-Allow-Methods`
- `Access-Control-Allow-Origin`

...when `Access-Control-Allow-Credentials: true` is set.

This is a security measure to prevent credential leakage to unintended origins.

### Solution

Use explicit header list instead of `Any`:

```rust
// CORRECT
use axum::http::header;

let cors = CorsLayer::new()
    .allow_headers([
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::ACCEPT,
        header::ORIGIN,
    ])
    .allow_credentials(true);
```

### Prevention

- Never use `Any` or wildcard with `allow_credentials(true)`
- Always specify explicit origins, methods, and headers when credentials are needed
- Test CORS configuration before deploying

---

## 2026-01-20: Non-Existent Header Constant

### Incident

Compilation failed:

```
error[E0425]: cannot find value `X_REQUESTED_WITH` in module `header`
```

### Root Cause

Attempted to use `header::X_REQUESTED_WITH` which doesn't exist in `axum::http::header`:

```rust
// WRONG - X_REQUESTED_WITH is not a standard constant
.allow_headers([
    header::AUTHORIZATION,
    header::CONTENT_TYPE,
    header::X_REQUESTED_WITH,  // Does not exist
])
```

### Why It Fails

`X-Requested-With` is a non-standard header (commonly used by jQuery/AJAX). It's not included in the standard HTTP header constants defined in `axum::http::header`.

### Solution

Either omit it (if not needed) or create a custom header:

```rust
// Option 1: Omit if not needed
.allow_headers([
    header::AUTHORIZATION,
    header::CONTENT_TYPE,
    header::ACCEPT,
    header::ORIGIN,
])

// Option 2: Create custom header if needed
use axum::http::HeaderName;

.allow_headers([
    header::AUTHORIZATION,
    header::CONTENT_TYPE,
    HeaderName::from_static("x-requested-with"),
])
```

### Prevention

- Check `axum::http::header` docs for available constants
- Use `HeaderName::from_static()` for non-standard headers
- Run `cargo check` before running the server

---

## Quick Reference: Available Standard Headers

Common headers in `axum::http::header`:

| Constant | Header Name |
|----------|-------------|
| `AUTHORIZATION` | Authorization |
| `CONTENT_TYPE` | Content-Type |
| `ACCEPT` | Accept |
| `ORIGIN` | Origin |
| `COOKIE` | Cookie |
| `SET_COOKIE` | Set-Cookie |
| `CACHE_CONTROL` | Cache-Control |
| `CONTENT_LENGTH` | Content-Length |
| `HOST` | Host |
| `USER_AGENT` | User-Agent |

For non-standard headers, use:
```rust
HeaderName::from_static("x-custom-header")
```
