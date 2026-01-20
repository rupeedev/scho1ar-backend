# Security Guidelines

This document provides comprehensive security guidelines for the Scho1ar backend built with Rust, Axum, and SQLx.

---

## Critical Security Requirements

### Security-First Mindset

- Security is NOT a feature — it's a fundamental requirement
- When in doubt, choose the more secure option
- Always test security measures thoroughly before deployment
- Never bypass security checks for "convenience"

---

## 1. Input Validation & Sanitization

### Validation at Multiple Layers

All user inputs MUST be validated at multiple layers:

1. **Request parsing** (serde deserialization)
2. **Struct validation** (validator crate)
3. **Business logic validation** (service layer)
4. **Database constraints** (PostgreSQL)

### Rust Validation Pattern

```rust
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateResourceRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,

    #[validate(length(max = 500))]
    #[serde(default)]
    pub description: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    #[serde(default)]
    pub contact_email: Option<String>,

    #[validate(custom = "validate_no_dangerous_chars")]
    #[serde(default)]
    pub external_id: Option<String>,
}

fn validate_no_dangerous_chars(value: &str) -> Result<(), validator::ValidationError> {
    let dangerous_chars = ['\'', '"', ';', '<', '>', '`', '(', ')', '{', '}', '[', ']', '|', '&', '$', '\\'];

    if value.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err(validator::ValidationError::new("dangerous_characters"));
    }
    Ok(())
}
```

### Input Sanitization Helper

```rust
/// Sanitize user input by removing potentially dangerous characters
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| !matches!(c, '\'' | '"' | ';' | '<' | '>' | '`' | '(' | ')' | '{' | '}' | '[' | ']' | '|' | '&' | '$' | '\\'))
        .take(500) // Length limit
        .collect::<String>()
        .trim()
        .to_string()
}

/// Sanitize search input more strictly
pub fn sanitize_search_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '-' || *c == '_')
        .take(100)
        .collect::<String>()
        .trim()
        .to_string()
}
```

### Handler Validation Pattern

```rust
pub async fn create_resource(
    State(state): State<AppState>,
    Json(payload): Json<CreateResourceRequest>,
) -> AppResult<Json<Resource>> {
    // Validate struct
    payload.validate()
        .map_err(|e| AppError::BadRequest(format!("Validation failed: {}", e)))?;

    // Additional business validation
    if payload.name.to_lowercase().contains("admin") {
        return Err(AppError::BadRequest("Reserved name".into()));
    }

    // Safe to proceed with validated data
    // ...
}
```

---

## 2. SQL Injection Prevention

### CRITICAL: Use Parameterized Queries ONLY

SQLx provides compile-time checked queries. ALWAYS use them.

```rust
// ✅ CORRECT - Parameterized query (safe)
let resource = sqlx::query_as!(
    Resource,
    "SELECT * FROM resources WHERE id = $1 AND organization_id = $2",
    resource_id,
    org_id
)
.fetch_optional(&state.db)
.await?;

// ✅ CORRECT - Dynamic query with bind parameters (safe)
let mut query = sqlx::QueryBuilder::new("SELECT * FROM resources WHERE 1=1");

if let Some(name) = &params.name {
    query.push(" AND name ILIKE ");
    query.push_bind(format!("%{}%", name));
}

if let Some(status) = &params.status {
    query.push(" AND status = ");
    query.push_bind(status);
}

let resources = query
    .build_query_as::<Resource>()
    .fetch_all(&state.db)
    .await?;
```

```rust
// ❌ WRONG - String concatenation (VULNERABLE)
let query = format!(
    "SELECT * FROM resources WHERE name = '{}'",
    user_input  // SQL INJECTION VULNERABILITY!
);

// ❌ WRONG - Using format! for dynamic queries
let column = user_input;
let query = format!("SELECT {} FROM resources", column);  // VULNERABLE!
```

### Safe Dynamic Column/Table Names

If you must use dynamic identifiers (rare), whitelist them:

```rust
fn validate_sort_column(column: &str) -> Result<&'static str, AppError> {
    match column {
        "name" => Ok("name"),
        "created_at" => Ok("created_at"),
        "updated_at" => Ok("updated_at"),
        "status" => Ok("status"),
        _ => Err(AppError::BadRequest("Invalid sort column".into())),
    }
}

pub async fn list_resources(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> AppResult<Json<Vec<Resource>>> {
    let sort_column = validate_sort_column(&params.sort_by.unwrap_or_default())?;

    // Now safe to use in query
    let query = format!(
        "SELECT * FROM resources ORDER BY {} {}",
        sort_column,
        if params.sort_order == SortOrder::Asc { "ASC" } else { "DESC" }
    );

    // ...
}
```

---

## 3. Authentication & Authorization

### JWT Token Validation

```rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: Uuid,           // User ID
    pub email: String,
    pub org_id: Option<Uuid>,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn validate_token(token: &str, secret: &[u8]) -> Result<Claims, AppError> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &validation
    )
    .map_err(|e| {
        tracing::warn!("JWT validation failed: {}", e);
        AppError::Unauthorized
    })?;

    Ok(token_data.claims)
}
```

### Auth Middleware

```rust
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = validate_token(token, &state.config.jwt_secret).await?;

    // Insert user info into request extensions
    request.extensions_mut().insert(AuthUser {
        id: claims.sub,
        email: claims.email,
        org_id: claims.org_id,
        role: claims.role.parse().unwrap_or(UserRole::Viewer),
    });

    Ok(next.run(request).await)
}
```

### Role-Based Access Control

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    Member,
    Viewer,
}

pub fn require_role(user: &AuthUser, required_roles: &[UserRole]) -> Result<(), AppError> {
    if !required_roles.contains(&user.role) {
        tracing::warn!(
            user_id = %user.id,
            role = ?user.role,
            required = ?required_roles,
            "Insufficient permissions"
        );
        return Err(AppError::Forbidden("Insufficient permissions".into()));
    }
    Ok(())
}

// Usage in handler
pub async fn delete_organization(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(org_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    require_role(&auth, &[UserRole::Admin])?;

    // ... delete logic
}
```

### Organization Scoping

```rust
/// Verify user has access to the specified organization
pub async fn verify_org_access(
    db: &PgPool,
    user_id: Uuid,
    org_id: Uuid,
) -> Result<(), AppError> {
    let has_access = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM organization_members
            WHERE user_id = $1 AND organization_id = $2
        ) as "exists!"
        "#,
        user_id,
        org_id
    )
    .fetch_one(db)
    .await?;

    if !has_access {
        tracing::warn!(
            user_id = %user_id,
            org_id = %org_id,
            "Unauthorized organization access attempt"
        );
        return Err(AppError::Forbidden("Access denied to this organization".into()));
    }

    Ok(())
}
```

---

## 4. Error Handling Security

### CRITICAL: Never Expose Internal Details

```rust
// ✅ CORRECT - Safe error handling
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, user_message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),

            // IMPORTANT: Don't expose internal error details
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);  // Log full details
                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);  // Log full details
                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected error occurred".to_string())
            }
        };

        let body = ErrorResponse {
            status_code: status.as_u16(),
            error: status.canonical_reason().unwrap_or("Error").to_string(),
            message: user_message,  // Sanitized message only
            timestamp: chrono::Utc::now().to_rfc3339(),
            path: None,
        };

        (status, Json(body)).into_response()
    }
}
```

```rust
// ❌ WRONG - Exposing internal details
AppError::Database(e) => {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))  // LEAKS INFO!
}
```

### Error Messages by Context

| Error Type | User-Facing Message | Logged Message |
|------------|---------------------|----------------|
| Invalid password | "Invalid credentials" | "Password mismatch for user X" |
| User not found | "Invalid credentials" | "User not found: email@example.com" |
| SQL error | "An unexpected error occurred" | Full SQL error with query details |
| Token expired | "Session expired" | "JWT expired for user X" |
| Rate limited | "Too many requests" | "Rate limit exceeded: IP X, endpoint Y" |

---

## 5. Security Logging

### Structured Security Logging

```rust
use tracing::{info, warn, error};

/// Log security-relevant events
pub fn log_security_event(event: SecurityEvent) {
    match event.severity {
        Severity::Info => info!(
            event_type = %event.event_type,
            user_id = ?event.user_id,
            ip = ?event.ip_address,
            details = ?event.details,
            "Security event"
        ),
        Severity::Warning => warn!(
            event_type = %event.event_type,
            user_id = ?event.user_id,
            ip = ?event.ip_address,
            details = ?event.details,
            "Security warning"
        ),
        Severity::Critical => error!(
            event_type = %event.event_type,
            user_id = ?event.user_id,
            ip = ?event.ip_address,
            details = ?event.details,
            "CRITICAL security event"
        ),
    }
}

#[derive(Debug)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub severity: Severity,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub details: serde_json::Value,
}

#[derive(Debug)]
pub enum SecurityEventType {
    LoginSuccess,
    LoginFailure,
    PasswordChange,
    PermissionDenied,
    SuspiciousInput,
    RateLimitExceeded,
    TokenInvalid,
    UnauthorizedAccess,
}
```

### Log Suspicious Input

```rust
pub fn validate_and_log_input(input: &str, field_name: &str, user_id: Option<Uuid>) -> String {
    let has_suspicious_chars = input.chars().any(|c|
        matches!(c, '\'' | '"' | ';' | '<' | '>' | '`' | '|' | '&')
    );

    if has_suspicious_chars {
        log_security_event(SecurityEvent {
            event_type: SecurityEventType::SuspiciousInput,
            severity: Severity::Warning,
            user_id,
            ip_address: None,
            details: serde_json::json!({
                "field": field_name,
                "input_length": input.len(),
                "contains_sql_chars": input.contains('\'') || input.contains(';'),
                "contains_html_chars": input.contains('<') || input.contains('>'),
            }),
        });
    }

    sanitize_input(input)
}
```

---

## 6. Rate Limiting

### Tower Rate Limiting

```rust
use tower::limit::RateLimitLayer;
use std::time::Duration;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api", api_routes())
        // Global rate limit: 100 requests per minute
        .layer(RateLimitLayer::new(100, Duration::from_secs(60)))
        .with_state(state)
}
```

### Per-Endpoint Rate Limiting

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<std::time::Instant>>>>,
    limit: usize,
    window: Duration,
}

impl RateLimiter {
    pub async fn check(&self, key: &str) -> Result<(), AppError> {
        let mut requests = self.requests.write().await;
        let now = std::time::Instant::now();

        let entry = requests.entry(key.to_string()).or_insert_with(Vec::new);

        // Remove old requests outside window
        entry.retain(|&t| now.duration_since(t) < self.window);

        if entry.len() >= self.limit {
            tracing::warn!(key = %key, "Rate limit exceeded");
            return Err(AppError::TooManyRequests);
        }

        entry.push(now);
        Ok(())
    }
}
```

---

## 7. CORS Security

### Secure CORS Configuration

```rust
use tower_http::cors::{CorsLayer, Any};
use axum::http::{header, Method};

pub fn cors_layer(allowed_origins: &[String]) -> CorsLayer {
    let origins: Vec<_> = allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // IMPORTANT: Explicit headers, never use Any with credentials
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}
```

```rust
// ❌ CRITICAL: Never do this
CorsLayer::new()
    .allow_origin(Any)
    .allow_headers(Any)          // INSECURE!
    .allow_credentials(true)      // Incompatible with Any!
```

---

## 8. Credential Security

### Environment Variables

```rust
pub struct Config {
    pub database_url: String,
    pub jwt_secret: Vec<u8>,
    pub aws_access_key: Option<String>,
    pub aws_secret_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| ConfigError::Missing("DATABASE_URL"))?,
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| ConfigError::Missing("JWT_SECRET"))?
                .into_bytes(),
            aws_access_key: std::env::var("AWS_ACCESS_KEY_ID").ok(),
            aws_secret_key: std::env::var("AWS_SECRET_ACCESS_KEY").ok(),
        })
    }
}
```

### Never Log Credentials

```rust
// ✅ CORRECT
tracing::info!(
    "Connecting to database at {}",
    database_url.split('@').last().unwrap_or("*****")  // Hide credentials
);

// ❌ WRONG
tracing::info!("Connecting with URL: {}", database_url);  // LEAKS PASSWORD!
```

---

## 9. Security Testing Checklist

### Manual Testing

- [ ] Test all inputs with SQL injection payloads (`' OR '1'='1`, `'; DROP TABLE`, etc.)
- [ ] Test all inputs with XSS payloads (`<script>`, `javascript:`, etc.)
- [ ] Test authentication bypass attempts
- [ ] Test authorization bypass (access other org's data)
- [ ] Test rate limiting with rapid requests
- [ ] Verify error messages don't leak internal details
- [ ] Test with expired/invalid/malformed JWT tokens

### Automated Testing

```rust
#[cfg(test)]
mod security_tests {
    use super::*;

    #[tokio::test]
    async fn test_sql_injection_prevention() {
        let app = create_test_app().await;

        let malicious_inputs = vec![
            "'; DROP TABLE users; --",
            "' OR '1'='1",
            "1; SELECT * FROM users",
            "admin'--",
        ];

        for input in malicious_inputs {
            let response = app
                .get(&format!("/api/resources?search={}", urlencoding::encode(input)))
                .await;

            // Should return empty results or bad request, not error
            assert!(response.status().is_success() || response.status() == 400);
        }
    }

    #[tokio::test]
    async fn test_unauthorized_access() {
        let app = create_test_app().await;

        // Try to access without token
        let response = app.get("/api/organizations").await;
        assert_eq!(response.status(), 401);

        // Try to access other org's data
        let response = app
            .with_auth(user_token)
            .get("/api/organizations/other-org-id/resources")
            .await;
        assert_eq!(response.status(), 403);
    }
}
```

---

## 10. Security Headers

### Recommended Response Headers

```rust
use axum::{middleware::Next, response::Response, extract::Request};

pub async fn security_headers_middleware(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin".parse().unwrap());
    headers.insert(
        "Content-Security-Policy",
        "default-src 'self'; frame-ancestors 'none'".parse().unwrap()
    );

    response
}
```

---

## Best Practices Summary

### DO:
1. Validate ALL inputs with explicit validation rules
2. Use parameterized queries exclusively
3. Log security events with context (but not sensitive data)
4. Return generic error messages to users
5. Implement rate limiting on all endpoints
6. Use HTTPS in production
7. Rotate credentials regularly
8. Audit access logs periodically

### DON'T:
1. Trust any user input, even from "internal" clients
2. Use string concatenation for SQL queries
3. Expose stack traces or internal errors
4. Log passwords, tokens, or API keys
5. Store credentials in code
6. Skip authentication checks for "convenience"
7. Use `Any` with CORS credentials
8. Assume validation at one layer is sufficient

---

## Incident Response

### Security Incident Steps

1. **Immediate**: Block the threat (revoke tokens, block IPs)
2. **Assess**: Determine scope and impact
3. **Contain**: Prevent further damage
4. **Investigate**: Analyze logs and evidence
5. **Remediate**: Fix vulnerabilities
6. **Document**: Record incident and lessons learned
