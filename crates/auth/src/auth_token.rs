use axum::http::StatusCode;
use std::collections::BTreeMap;

use crate::{
    api_tokens::{ApiToken, ParseApiTokenError},
    paseto_tokens::{AccessTokenClaims, PasetoError},
    types::ServiceId,
};

/// Unified authentication token that handles both legacy and new token types
#[derive(Debug, Clone)]
pub enum AuthToken {
    /// Legacy API token format: "id|token"
    Legacy(ApiToken),
    /// Long-lived API key: "ak_xxxxx.yyyyy"
    ApiKey(String),
    /// Short-lived Paseto access token: "v4.local.xxxxx"
    AccessToken(AccessTokenClaims),
}

#[derive(Debug, thiserror::Error)]
pub enum AuthTokenError {
    #[error("Invalid token format")]
    InvalidFormat,

    #[error("Legacy token error: {0}")]
    LegacyToken(#[from] ParseApiTokenError),

    #[error("Paseto token error: {0}")]
    PasetoToken(#[from] PasetoError),

    #[error("Malformed API key")]
    MalformedApiKey,
}

impl AuthToken {
    /// Parse a token string into the appropriate AuthToken type
    pub fn parse(token_str: &str) -> Result<Self, AuthTokenError> {
        if token_str.starts_with("v4.local.") {
            // This is a Paseto access token - we'll validate it in the proxy handler
            // For now, we just store the string and validate later with the key
            return Err(AuthTokenError::InvalidFormat); // Will be handled by PasetoTokenManager
        } else if token_str.contains('|') {
            // This is a legacy token: "id|token"
            let legacy_token = ApiToken::from_str(token_str)?;
            return Ok(AuthToken::Legacy(legacy_token));
        } else if token_str.contains('.') {
            // This is an API key: "prefix_xxxxx.yyyyy"
            // We check for dot last to avoid confusion with Paseto tokens
            return Ok(AuthToken::ApiKey(token_str.to_string()));
        }

        Err(AuthTokenError::InvalidFormat)
    }

    /// Get the service ID if available (only for validated tokens)
    pub fn service_id(&self) -> Option<ServiceId> {
        match self {
            AuthToken::Legacy(_) => None, // Need to look up in database
            AuthToken::ApiKey(_) => None, // Need to look up in database
            AuthToken::AccessToken(claims) => Some(claims.service_id),
        }
    }

    /// Get additional headers if available (only for access tokens)
    pub fn additional_headers(&self) -> BTreeMap<String, String> {
        match self {
            AuthToken::AccessToken(claims) => claims.additional_headers.clone(),
            _ => BTreeMap::new(),
        }
    }

    /// Check if token is expired (only applicable to access tokens)
    pub fn is_expired(&self) -> bool {
        match self {
            AuthToken::AccessToken(claims) => claims.is_expired(),
            _ => false, // Legacy tokens and API keys have their own expiration handling
        }
    }
}

/// Extraction result from request parsing
#[derive(Debug)]
pub enum TokenExtractionResult {
    /// Legacy token that needs database validation
    Legacy(ApiToken),
    /// API key that needs database validation and exchange
    ApiKey(String),
    /// Validated Paseto access token ready to use
    ValidatedAccessToken(AccessTokenClaims),
}

impl<S> axum::extract::FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        use axum::response::IntoResponse;

        let header = match parts.headers.get(crate::types::headers::AUTHORIZATION) {
            Some(header) => header,
            None => {
                return Err(
                    (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response()
                );
            }
        };

        let header_str = match header.to_str() {
            Ok(header_str) if header_str.starts_with("Bearer ") => &header_str[7..],
            Ok(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid Authorization header; expected Bearer token",
                )
                    .into_response());
            }
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid Authorization header; not valid UTF-8",
                )
                    .into_response());
            }
        };

        match AuthToken::parse(header_str) {
            Ok(token) => Ok(token),
            Err(AuthTokenError::InvalidFormat) => {
                // Special handling for Paseto tokens - they need the manager to validate
                if header_str.starts_with("v4.local.") {
                    // We'll handle Paseto validation in the proxy layer
                    // For now, store the raw string for later processing
                    Err(
                        (StatusCode::BAD_REQUEST, "Paseto token validation required")
                            .into_response(),
                    )
                } else {
                    Err((StatusCode::BAD_REQUEST, "Invalid token format").into_response())
                }
            }
            Err(e) => {
                Err((StatusCode::BAD_REQUEST, format!("Invalid token: {e}")).into_response())
            }
        }
    }
}

/// Token exchange request for converting API keys to access tokens
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenExchangeRequest {
    /// Optional additional headers to include in the access token
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub additional_headers: BTreeMap<String, String>,
    /// Optional custom TTL in seconds (if not provided, uses default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
}

/// Token exchange response containing the new access token
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct TokenExchangeResponse {
    /// The new Paseto access token
    pub access_token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Expiration timestamp (seconds since epoch)
    pub expires_at: u64,
    /// Time to live in seconds
    pub expires_in: u64,
}

impl TokenExchangeResponse {
    pub fn new(access_token: String, expires_at: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            access_token,
            token_type: "Bearer".to_string(),
            expires_at,
            expires_in: expires_at.saturating_sub(now),
        }
    }
}
