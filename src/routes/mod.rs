pub mod health;

use axum::{middleware, routing::get, Router};

use crate::auth::{require_auth, Claims};
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check endpoints (public)
        .route("/health", get(health::health_check))
        .route("/ready", get(health::ready_check))
        // API routes
        .nest("/api", api_routes(state.clone()))
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    // Public API routes
    let public_routes = Router::new().route("/", get(api_root));

    // Protected API routes (require authentication)
    let protected_routes =
        Router::new()
            .route("/me", get(get_current_user))
            .layer(middleware::from_fn_with_state(
                (state.jwks_cache.clone(), state.config.clerk.clone()),
                require_auth,
            ));

    // Merge public and protected routes
    public_routes.merge(protected_routes)
}

async fn api_root() -> &'static str {
    "Scho1ar API v0.1.0"
}

/// Example protected endpoint that returns the current user's claims
async fn get_current_user(claims: Claims) -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "userId": claims.user_id(),
        "organizationId": claims.organization_id(),
        "organizationRole": claims.organization_role(),
    }))
}
