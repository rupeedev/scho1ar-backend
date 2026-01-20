pub mod config;
pub mod db;
pub mod error;
pub mod routes;

use db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: config::Config,
}

impl AppState {
    pub fn new(db: DbPool, config: config::Config) -> Self {
        Self { db, config }
    }
}
