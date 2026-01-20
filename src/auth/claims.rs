//! JWT Claims structure for Clerk tokens

use serde::{Deserialize, Serialize};

/// JWT claims extracted from a validated Clerk token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - Clerk user ID (e.g., "user_2abc123...")
    pub sub: String,
    /// Issuer - Clerk issuer URL
    pub iss: String,
    /// Audience - typically the frontend app URL
    #[serde(default)]
    pub aud: Option<String>,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
    /// Not before time (Unix timestamp)
    #[serde(default)]
    pub nbf: Option<i64>,
    /// JWT ID - unique identifier for the token
    #[serde(default)]
    pub jti: Option<String>,
    /// Authorized party - the client that was issued the token
    #[serde(default)]
    pub azp: Option<String>,
    /// Session ID from Clerk
    #[serde(default)]
    pub sid: Option<String>,
    /// Organization ID if user is part of an organization
    #[serde(default)]
    pub org_id: Option<String>,
    /// Organization role if user is part of an organization
    #[serde(default)]
    pub org_role: Option<String>,
    /// Organization slug
    #[serde(default)]
    pub org_slug: Option<String>,
}

impl Claims {
    /// Get the Clerk user ID
    pub fn user_id(&self) -> &str {
        &self.sub
    }

    /// Get the organization ID if present
    pub fn organization_id(&self) -> Option<&str> {
        self.org_id.as_deref()
    }

    /// Check if the user belongs to an organization
    pub fn has_organization(&self) -> bool {
        self.org_id.is_some()
    }

    /// Get the organization role if present
    pub fn organization_role(&self) -> Option<&str> {
        self.org_role.as_deref()
    }
}
