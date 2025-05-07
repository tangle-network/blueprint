use core::fmt::Display;

use rand::{CryptoRng, RngCore};

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
    /// The expiration time of the token in milliseconds since the epoch.
    /// If `None`, the token does not expire.
    expires_at: Option<i64>,
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

    /// Generates a new API token without an expiration time.
    ///
    /// This is a convenience method that calls [`generate_token_with_expiration`](Self::generate_token_with_expiration) with an expiration time of 0.
    pub fn generate_token<R: RngCore + CryptoRng>(&self, rng: &mut R) -> GeneratedApiToken {
        self.generate_token_with_expiration(rng, 0)
    }

    /// Generates a new API token with the specified expiration time.
    pub fn generate_token_with_expiration<R: RngCore + CryptoRng>(
        &self,
        rng: &mut R,
        expires_at: i64,
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
    pub fn plaintext<I: AsRef<str>>(&self, id: I) -> String {
        format!("{}|{}", self.plaintext, id.as_ref())
    }

    /// Get the hashed token to be stored in the database.
    ///
    /// Store this token in the database, and use the returned ID with [`plaintext`](Self::plaintext) to identify the token to be shared with the client.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Get the expiration time of the token.
    pub fn expires_at(&self) -> Option<i64> {
        self.expires_at
    }
}
