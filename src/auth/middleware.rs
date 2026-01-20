//! Authentication middleware for protected routes

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, decode_header, Validation};
use serde_json::json;

use super::claims::Claims;
use super::jwks::SharedJwksCache;
use crate::config::ClerkConfig;

/// Extension key for storing authenticated claims
#[derive(Clone)]
pub struct AuthenticatedUser(pub Claims);

/// Authentication error responses
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken(String),
    ExpiredToken,
    InvalidIssuer,
    JwksError(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header".to_string(),
            ),
            AuthError::InvalidToken(msg) => {
                (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", msg))
            }
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token has expired".to_string()),
            AuthError::InvalidIssuer => {
                (StatusCode::UNAUTHORIZED, "Invalid token issuer".to_string())
            }
            AuthError::JwksError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Authentication service error: {}", msg),
            ),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

/// Extract bearer token from Authorization header
fn extract_bearer_token(auth_header: &str) -> Option<&str> {
    auth_header
        .strip_prefix("Bearer ")
        .or_else(|| auth_header.strip_prefix("bearer "))
}

/// Authentication middleware that validates Clerk JWTs
///
/// This middleware:
/// 1. Extracts the JWT from the Authorization header
/// 2. Decodes the header to get the key ID (kid)
/// 3. Fetches the public key from Clerk's JWKS endpoint
/// 4. Validates the token signature and claims
/// 5. Stores the claims in request extensions for handlers to access
///
/// Usage with axum:
/// ```rust,ignore
/// use axum::{Router, middleware};
/// use crate::auth::require_auth;
///
/// let protected_routes = Router::new()
///     .route("/protected", get(handler))
///     .layer(middleware::from_fn_with_state(
///         (jwks_cache, clerk_config),
///         require_auth,
///     ));
/// ```
pub async fn require_auth(
    State((jwks_cache, clerk_config)): State<(SharedJwksCache, ClerkConfig)>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    // Extract bearer token
    let token = extract_bearer_token(auth_header).ok_or(AuthError::MissingToken)?;

    // Decode header to get the key ID
    let header = decode_header(token)
        .map_err(|e| AuthError::InvalidToken(format!("Invalid JWT header: {}", e)))?;

    let kid = header
        .kid
        .ok_or_else(|| AuthError::InvalidToken("Missing key ID in token header".to_string()))?;

    // Get the decoding key from JWKS cache
    let (decoding_key, algorithm) = jwks_cache
        .get_key(&kid)
        .await
        .map_err(|e| AuthError::JwksError(e.to_string()))?;

    // Configure validation
    let mut validation = Validation::new(algorithm);
    validation.set_issuer(&[&clerk_config.issuer]);

    // Set audience if configured
    if let Some(ref aud) = clerk_config.audience {
        validation.set_audience(&[aud]);
    } else {
        // Clerk tokens may not have a standard audience
        validation.validate_aud = false;
    }

    // Decode and validate the token
    let token_data =
        decode::<Claims>(token, &decoding_key, &validation).map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => AuthError::InvalidIssuer,
            _ => AuthError::InvalidToken(e.to_string()),
        })?;

    // Store claims in request extensions
    request
        .extensions_mut()
        .insert(AuthenticatedUser(token_data.claims));

    // Continue to the next handler
    Ok(next.run(request).await)
}

/// Extractor for getting authenticated user claims in handlers
///
/// Usage:
/// ```rust,ignore
/// use crate::auth::Claims;
///
/// async fn protected_handler(
///     claims: Claims,
/// ) -> impl IntoResponse {
///     format!("Hello, user {}", claims.user_id())
/// }
/// ```
impl<S> axum::extract::FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut axum::http::request::Parts,
        _state: &'life1 S,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            parts
                .extensions
                .get::<AuthenticatedUser>()
                .map(|user| user.0.clone())
                .ok_or(AuthError::MissingToken)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bearer_token() {
        assert_eq!(extract_bearer_token("Bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("bearer abc123"), Some("abc123"));
        assert_eq!(extract_bearer_token("Basic abc123"), None);
        assert_eq!(extract_bearer_token("abc123"), None);
    }
}
