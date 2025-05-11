use core::fmt::Display;
use rand::{CryptoRng, RngCore};

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

        let token_str = base64::Engine::encode(&CUSTOM_ENGINE, &token);
        let final_token = format!("{}{}", self.prefix, token_str);
        let mut hasher = tiny_keccak::Keccak::v256();
        hasher.update(final_token.as_bytes());
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);

        GeneratedApiToken {
            plaintext: token_str,
            token: base64::Engine::encode(&CUSTOM_ENGINE, output),
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
    fn from_str(s: &str) -> Result<ApiToken, ParseApiTokenError> {
        // Use an iterator to avoid allocations
        let mut parts = s.splitn(3, '|');

        // Get the ID part (first segment)
        let id_part = parts.next().ok_or(ParseApiTokenError::MalformedToken)?;
        let id = id_part
            .parse::<u64>()
            .map_err(|_| ParseApiTokenError::InvalidTokenId)?;

        // Get the token part (second segment)
        let token_part = parts.next().ok_or(ParseApiTokenError::MalformedToken)?;

        // Check if it is a valid base64 token
        if base64::Engine::decode(&CUSTOM_ENGINE, token_part).is_err() {
            return Err(ParseApiTokenError::MalformedToken);
        }

        // Check if there are more than 2 parts (meaning more than one separator)
        if parts.next().is_some() {
            return Err(ParseApiTokenError::MalformedToken);
        }

        // Use Cow to avoid allocation when parsing temporary strings
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
