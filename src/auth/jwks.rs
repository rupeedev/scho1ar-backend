//! JWKS (JSON Web Key Set) fetching and caching for Clerk

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use jsonwebtoken::{Algorithm, DecodingKey};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::config::ClerkConfig;

/// JWKS response from Clerk
#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<JwkKey>,
}

/// Individual JWK key from the JWKS endpoint
#[derive(Debug, Deserialize)]
struct JwkKey {
    /// Key type (e.g., "RSA")
    kty: String,
    /// Key ID
    kid: String,
    /// Algorithm (e.g., "RS256")
    #[serde(default)]
    alg: Option<String>,
    /// RSA modulus (base64url encoded)
    #[serde(default)]
    n: Option<String>,
    /// RSA exponent (base64url encoded)
    #[serde(default)]
    e: Option<String>,
    /// Key use (e.g., "sig" for signature)
    #[serde(rename = "use", default)]
    #[allow(dead_code)]
    key_use: Option<String>,
}

/// Cached decoding key with metadata
struct CachedKey {
    decoding_key: DecodingKey,
    algorithm: Algorithm,
}

/// JWKS cache for storing fetched keys
pub struct JwksCache {
    keys: RwLock<HashMap<String, CachedKey>>,
    jwks_url: String,
    last_fetch: RwLock<Option<Instant>>,
    cache_duration: Duration,
    http_client: reqwest::Client,
}

impl JwksCache {
    /// Create a new JWKS cache from Clerk configuration
    pub fn new(config: &ClerkConfig) -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
            jwks_url: config.jwks_url.clone(),
            last_fetch: RwLock::new(None),
            cache_duration: Duration::from_secs(3600), // 1 hour cache
            http_client: reqwest::Client::new(),
        }
    }

    /// Get a decoding key by key ID, fetching from JWKS if needed
    pub async fn get_key(&self, kid: &str) -> Result<(DecodingKey, Algorithm), JwksError> {
        // Check if we need to refresh the cache
        let should_refresh = {
            let last_fetch = self.last_fetch.read().await;
            match *last_fetch {
                Some(instant) => instant.elapsed() > self.cache_duration,
                None => true,
            }
        };

        // Try to get the key from cache first
        {
            let keys = self.keys.read().await;
            if let Some(cached) = keys.get(kid) {
                if !should_refresh {
                    return Ok((cached.decoding_key.clone(), cached.algorithm));
                }
            }
        }

        // Key not found or cache expired, fetch new keys
        self.refresh_keys().await?;

        // Try again after refresh
        let keys = self.keys.read().await;
        keys.get(kid)
            .map(|cached| (cached.decoding_key.clone(), cached.algorithm))
            .ok_or_else(|| JwksError::KeyNotFound(kid.to_string()))
    }

    /// Refresh the JWKS cache
    async fn refresh_keys(&self) -> Result<(), JwksError> {
        tracing::debug!("Fetching JWKS from {}", self.jwks_url);

        let response = self
            .http_client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| JwksError::FetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(JwksError::FetchError(format!(
                "JWKS endpoint returned status {}",
                response.status()
            )));
        }

        let jwks: JwksResponse = response
            .json()
            .await
            .map_err(|e| JwksError::ParseError(e.to_string()))?;

        let mut new_keys = HashMap::new();

        for key in jwks.keys {
            if key.kty != "RSA" {
                tracing::debug!("Skipping non-RSA key: {}", key.kid);
                continue;
            }

            let Some(n) = key.n else {
                tracing::warn!("RSA key {} missing modulus", key.kid);
                continue;
            };

            let Some(e) = key.e else {
                tracing::warn!("RSA key {} missing exponent", key.kid);
                continue;
            };

            let algorithm = match key.alg.as_deref() {
                Some("RS256") | None => Algorithm::RS256,
                Some("RS384") => Algorithm::RS384,
                Some("RS512") => Algorithm::RS512,
                Some(alg) => {
                    tracing::warn!("Unsupported algorithm {} for key {}", alg, key.kid);
                    continue;
                }
            };

            match DecodingKey::from_rsa_components(&n, &e) {
                Ok(decoding_key) => {
                    new_keys.insert(
                        key.kid.clone(),
                        CachedKey {
                            decoding_key,
                            algorithm,
                        },
                    );
                }
                Err(e) => {
                    tracing::warn!("Failed to parse RSA key {}: {}", key.kid, e);
                }
            }
        }

        // Update the cache
        {
            let mut keys = self.keys.write().await;
            *keys = new_keys;
        }

        {
            let mut last_fetch = self.last_fetch.write().await;
            *last_fetch = Some(Instant::now());
        }

        tracing::debug!("JWKS cache refreshed successfully");
        Ok(())
    }
}

/// Thread-safe shared JWKS cache
pub type SharedJwksCache = Arc<JwksCache>;

/// Create a new shared JWKS cache
pub fn create_jwks_cache(config: &ClerkConfig) -> SharedJwksCache {
    Arc::new(JwksCache::new(config))
}

#[derive(Debug, thiserror::Error)]
pub enum JwksError {
    #[error("Failed to fetch JWKS: {0}")]
    FetchError(String),

    #[error("Failed to parse JWKS: {0}")]
    ParseError(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),
}
