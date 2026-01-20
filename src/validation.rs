//! Request validation utilities using the `validator` crate.
//!
//! This module provides a `ValidatedJson` extractor that combines JSON deserialization
//! with automatic validation of request payloads.
//!
//! # Usage
//!
//! ```ignore
//! use serde::Deserialize;
//! use validator::Validate;
//! use crate::validation::ValidatedJson;
//! use crate::error::AppResult;
//! use axum::Json;
//!
//! #[derive(Debug, Deserialize, Validate)]
//! pub struct CreateRequest {
//!     #[validate(length(min = 1, max = 100))]
//!     pub name: String,
//!
//!     #[validate(email)]
//!     pub email: Option<String>,
//! }
//!
//! pub async fn create_handler(
//!     ValidatedJson(payload): ValidatedJson<CreateRequest>,
//! ) -> AppResult<Json<CreateRequest>> {
//!     // payload is already validated
//!     Ok(Json(payload))
//! }
//! ```

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

/// A JSON extractor that automatically validates the payload.
///
/// This extractor combines `axum::Json` with `validator::Validate` to provide
/// automatic validation of incoming JSON request bodies.
///
/// # Error Handling
///
/// - If JSON parsing fails, returns a `400 Bad Request` with parse error details
/// - If validation fails, returns a `400 Bad Request` with validation error details
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // First, try to extract JSON
        let Json(value) =
            Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection: JsonRejection| {
                    AppError::BadRequest(format!("Invalid JSON: {}", rejection))
                })?;

        // Then validate the payload
        value.validate().map_err(|e| {
            let errors = format_validation_errors(&e);
            AppError::Validation(errors)
        })?;

        Ok(ValidatedJson(value))
    }
}

/// Formats validation errors into a human-readable string.
fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    let field_errors: Vec<String> = errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let messages: Vec<String> = errs
                .iter()
                .map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("invalid value for '{}'", e.code))
                })
                .collect();
            format!("{}: {}", field, messages.join(", "))
        })
        .collect();

    field_errors.join("; ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::ValidationErrors;

    #[test]
    fn test_format_validation_errors_empty() {
        let errors = ValidationErrors::new();
        let result = format_validation_errors(&errors);
        assert_eq!(result, "");
    }
}
