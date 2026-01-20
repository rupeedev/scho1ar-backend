pub mod health;

use axum::{routing::get, Router};

use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check endpoints
        .route("/health", get(health::health_check))
        .route("/ready", get(health::ready_check))
        // API routes will be added here
        .nest("/api", api_routes())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        // Placeholder for future API routes
        .route("/", get(api_root))
}

async fn api_root() -> &'static str {
    "Scho1ar API v0.1.0"
}
