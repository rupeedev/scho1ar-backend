pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod routes;
pub mod validation;

use auth::jwks::{create_jwks_cache, SharedJwksCache};
use db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: config::Config,
    pub jwks_cache: SharedJwksCache,
}

impl AppState {
    pub fn new(db: DbPool, config: config::Config) -> Self {
        let jwks_cache = create_jwks_cache(&config.clerk);
        Self {
            db,
            config,
            jwks_cache,
        }
    }
}
