//! Authentication middleware for Clerk JWT validation
//!
//! This module provides JWT-based authentication using Clerk as the identity provider.
//! It validates JWTs against Clerk's JWKS endpoint and extracts user claims.

mod claims;
pub mod jwks;
mod middleware;

pub use claims::Claims;
pub use middleware::{require_auth, AuthenticatedUser};
