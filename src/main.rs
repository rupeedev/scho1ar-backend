use std::net::SocketAddr;

use axum::http::{header, HeaderValue, Method};
use scho1ar_backend::{config::Config, db, routes, AppState};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "scho1ar_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting Scho1ar Backend in {} mode", config.environment);

    // Connect to database
    tracing::info!("Connecting to database...");
    let pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Database connected successfully");

    // Create application state
    let state = AppState::new(pool, config.clone());

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origins
                .iter()
                .filter_map(|origin| origin.parse::<HeaderValue>().ok())
                .collect::<Vec<_>>(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
        ])
        .allow_credentials(true);

    // Build router
    let app = routes::create_router(state).layer(cors);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
