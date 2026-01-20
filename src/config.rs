use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub environment: String,
    pub clerk: ClerkConfig,
}

#[derive(Debug, Clone)]
pub struct ClerkConfig {
    /// Clerk JWKS URL for fetching public keys
    pub jwks_url: String,
    /// Clerk issuer URL for JWT validation
    pub issuer: String,
    /// Expected audience (usually your frontend URL or Clerk app ID)
    pub audience: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::Missing("DATABASE_URL".to_string()))?;

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::Invalid("PORT must be a valid number".to_string()))?;

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let environment = env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string());

        // Clerk configuration
        let clerk_issuer = env::var("CLERK_ISSUER")
            .map_err(|_| ConfigError::Missing("CLERK_ISSUER".to_string()))?;

        let clerk_jwks_url = env::var("CLERK_JWKS_URL")
            .unwrap_or_else(|_| format!("{}/.well-known/jwks.json", clerk_issuer));

        let clerk_audience = env::var("CLERK_AUDIENCE").ok();

        let clerk = ClerkConfig {
            jwks_url: clerk_jwks_url,
            issuer: clerk_issuer,
            audience: clerk_audience,
        };

        Ok(Config {
            database_url,
            host,
            port,
            cors_origins,
            environment,
            clerk,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    Missing(String),
    #[error("Invalid configuration: {0}")]
    Invalid(String),
}
