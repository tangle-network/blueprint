use base64::Engine;
use blueprint_std::rand::{CryptoRng, RngCore};
use core::fmt::Display;

use crate::types::ServiceId;

/// A custom base64 engine that uses URL-safe encoding and no padding.
pub const CUSTOM_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::URL_SAFE,
    base64::engine::general_purpose::NO_PAD,
);

/// API Token Generator That is responsible for generating API tokens.
pub struct ApiTokenGenerator {
    /// The prefix to be used for the generated tokens.
    prefix: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedApiToken {
    /// The plaintext token that can be used for authentication through the API.
    plaintext: String,
    /// The hashed token that is stored in the database.
    pub(crate) token: String,
    /// The ID of the service that the token is associated with.
    pub(crate) service_id: ServiceId,
    /// The expiration time of the token in seconds since the epoch.
    /// If `None`, the token does not expire.
    expires_at: Option<u64>,
}

impl Display for GeneratedApiToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl Default for ApiTokenGenerator {
    fn default() -> Self {
        ApiTokenGenerator::new()
    }
}

impl ApiTokenGenerator {
    /// Creates a new instance of the API token generator with an empty prefix.
    ///
    /// See [`with_prefix`](Self::with_prefix) for more details.
    pub fn new() -> Self {
        ApiTokenGenerator {
            prefix: String::new(),
        }
    }

    /// Creates a new instance of the API token generator with the specified prefix.
    ///
    /// The prefix is used to identify the token type and can be useful for security purposes.
    pub fn with_prefix(prefix: &str) -> Self {
        ApiTokenGenerator {
            prefix: prefix.to_string(),
        }
    }

    /// Generates a new API token without an expiration time for the given service ID.
    ///
    /// This is a convenience method that calls [`generate_token_with_expiration`](Self::generate_token_with_expiration) with an expiration time of 0.
    pub fn generate_token<R: RngCore + CryptoRng>(
        &self,
        service_id: ServiceId,
        rng: &mut R,
    ) -> GeneratedApiToken {
        self.generate_token_with_expiration(service_id, 0, rng)
    }

    /// Generates a new API token with the specified expiration time.
    pub fn generate_token_with_expiration<R: RngCore + CryptoRng>(
        &self,
        service_id: ServiceId,
        expires_at: u64,
        rng: &mut R,
    ) -> GeneratedApiToken {
        use tiny_keccak::Hasher;
        let mut token = vec![0u8; 40];
        rng.fill_bytes(&mut token);
        let checksum = crc32fast::hash(&token);
        // Append the checksum to the token
        token.extend_from_slice(&checksum.to_be_bytes());

        let token_str = CUSTOM_ENGINE.encode(&token);
        let final_token = format!("{}{}", self.prefix, token_str);
        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(final_token.as_bytes());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);

        GeneratedApiToken {
            plaintext: final_token,
            token: CUSTOM_ENGINE.encode(output),
            service_id,
            expires_at: if expires_at != 0 {
                Some(expires_at)
            } else {
                None
            },
        }
    }
}

impl GeneratedApiToken {
    /// Get the plaintext token to be shared with the client with the given ID.
    ///
    /// The ID could be an incremental number to identify the token in the database, should be unique.
    pub fn plaintext(&self, id: u64) -> String {
        format!("{}|{}", id, self.plaintext)
    }

    /// Get the hashed token to be stored in the database.
    ///
    /// Store this token in the database, and use the returned ID with [`plaintext`](Self::plaintext) to identify the token to be shared with the client.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Get the expiration time of the token.
    pub fn expires_at(&self) -> Option<u64> {
        self.expires_at
    }
}

/// ApiToken that stores a token string
#[derive(Debug, Clone)]
pub struct ApiToken(pub u64, pub String);

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseApiTokenError {
    /// The token is malformed and cannot be parsed.
    ///
    /// The correct format for the token is `id|token`.
    #[error("Malformed token; expected format is `id|token`")]
    MalformedToken,
    /// The token ID is not a valid number.
    #[error("Invalid token ID; expected a number")]
    InvalidTokenId,
}

impl ApiToken {
    /// Creates a new `ApiToken` from the given id and token string
    fn new(id: u64, token: impl Into<String>) -> Self {
        ApiToken(id, token.into())
    }

    /// Parses a string into an `ApiToken`.
    pub(crate) fn from_str(s: &str) -> Result<ApiToken, ParseApiTokenError> {
        let mut parts = s.splitn(3, '|');

        let id_part = parts.next().ok_or(ParseApiTokenError::MalformedToken)?;
        let id = id_part
            .parse::<u64>()
            .map_err(|_| ParseApiTokenError::InvalidTokenId)?;

        let token_part = parts.next().ok_or(ParseApiTokenError::MalformedToken)?;

        if CUSTOM_ENGINE.decode(token_part).is_err() {
            return Err(ParseApiTokenError::MalformedToken);
        }

        // Check if there are more than 2 parts (meaning more than one separator)
        if parts.next().is_some() {
            return Err(ParseApiTokenError::MalformedToken);
        }

        Ok(ApiToken::new(id, token_part))
    }
}

impl Display for ApiToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}|{}", self.0, self.1)
    }
}

impl<S> axum::extract::FromRequestParts<S> for ApiToken
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        use axum::http::StatusCode;
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
            Ok(anything) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!(
                        "Invalid Authorization header; expected Bearer <api_token>, got {}",
                        anything
                    ),
                )
                    .into_response());
            }
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid Authorization header; not a valid UTF-8 string",
                )
                    .into_response());
            }
        };

        match ApiToken::from_str(header_str) {
            Ok(token) => Ok(token),
            Err(e) => {
                Err((StatusCode::BAD_REQUEST, format!("Invalid API Token: {e}",)).into_response())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ServiceId;
    use axum::extract::FromRequestParts;
    use axum::http::{Request, header::AUTHORIZATION};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_api_token_generator_new() {
        let generator = ApiTokenGenerator::new();
        let token =
            generator.generate_token(ServiceId::new(1), &mut blueprint_std::BlueprintRng::new());
        assert!(!token.token.is_empty());
    }

    #[test]
    fn test_api_token_generator_with_prefix() {
        let prefix = "test-prefix-";
        let generator = ApiTokenGenerator::with_prefix(prefix);
        let mut rng = blueprint_std::BlueprintRng::new();

        // Generate token with prefix
        let token1 = generator.generate_token(ServiceId::new(1), &mut rng);

        // Generate token without prefix for comparison
        let plain_generator = ApiTokenGenerator::new();
        let token2 = plain_generator.generate_token(ServiceId::new(1), &mut rng);

        // Tokens should be different due to prefix
        assert_ne!(token1.token, token2.token);
    }

    #[test]
    fn test_token_expiration() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expiry = now + 3600; // 1 hour from now

        let generator = ApiTokenGenerator::new();
        let mut rng = blueprint_std::BlueprintRng::new();

        // Token with expiration
        let token_with_expiry =
            generator.generate_token_with_expiration(ServiceId::new(1), expiry, &mut rng);

        // Token without expiration
        let token_without_expiry = generator.generate_token(ServiceId::new(1), &mut rng);

        // Check expiration times
        assert_eq!(token_with_expiry.expires_at(), Some(expiry));
        assert_eq!(token_without_expiry.expires_at(), None);
    }

    #[test]
    fn test_plaintext_token() {
        let generator = ApiTokenGenerator::new();
        let mut rng = blueprint_std::BlueprintRng::new();
        let token = generator.generate_token(ServiceId::new(1), &mut rng);

        let id = 42;
        let plaintext = token.plaintext(id);

        // Plaintext should be formatted as "id|token"
        assert!(plaintext.starts_with(&format!("{}|", id)));
        assert!(plaintext.len() > 3); // At least "id|t"
    }

    #[test]
    fn test_api_token_display() {
        let token = ApiToken(123, "test-token".to_string());
        assert_eq!(token.to_string(), "123|test-token");
    }

    #[tokio::test]
    async fn test_api_token_from_request() {
        // Create a request and extract parts
        let req = Request::builder()
            .header(
                AUTHORIZATION,
                "Bearer 123|RmFrZVRva2VuVGhhdElzQmFzZTY0RW5jb2RlZA",
            )
            .body(())
            .unwrap();
        let (mut parts, _) = req.into_parts();

        // Test successful extraction
        let result: Result<ApiToken, _> = ApiToken::from_request_parts(&mut parts, &()).await;
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.0, 123);
        assert_eq!(token.1, "RmFrZVRva2VuVGhhdElzQmFzZTY0RW5jb2RlZA");

        // Test missing Authorization header
        let req = Request::builder().body(()).unwrap();
        let (mut parts, _) = req.into_parts();
        let result: Result<ApiToken, _> = ApiToken::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());

        // Test invalid Authorization format
        let req = Request::builder()
            .header(AUTHORIZATION, "Basic 123:password")
            .body(())
            .unwrap();
        let (mut parts, _) = req.into_parts();
        let result: Result<ApiToken, _> = ApiToken::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_custom_engine() {
        let input = b"This is a test string for base64 encoding";
        let encoded = base64::Engine::encode(&CUSTOM_ENGINE, input);
        let decoded = base64::Engine::decode(&CUSTOM_ENGINE, &encoded).unwrap();
        assert_eq!(decoded, input);

        // The encoding should be URL-safe with no padding
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
        assert!(!encoded.contains('='));
    }
}
