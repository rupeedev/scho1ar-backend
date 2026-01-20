use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub database: String,
}

pub async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    // Check database connectivity
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => "connected".to_string(),
        Err(_) => "disconnected".to_string(),
    };

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
    }))
}

pub async fn ready_check() -> StatusCode {
    StatusCode::OK
}
