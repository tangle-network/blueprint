use serde::{Deserialize, Serialize};

/// Common headers used in the authentication process.
pub mod headers {
    pub const AUTHORIZATION: &str = "Authorization";
    pub const AUTHORIZATION_BEARER: &str = "Bearer";
    pub const X_SERVICE_ID: &str = "X-Service-Id";
}

/// Represents the different types of cryptographic keys used in the authentication process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KeyType {
    /// Ecdsa key type
    Ecdsa,
    /// Sr25519 key type
    Sr25519,
}

/// Represents the challenge request sent from the client to the server to request a challenge.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeRequest {
    /// The public key representing the user in hex format
    #[serde(with = "hex")]
    pub pub_key: Vec<u8>,
    /// The type of the public key
    pub key_type: KeyType,
}

/// Represents the challenge response sent from the server to the client after a successful challenge request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeResponse {
    /// The challenge string sent from the server to the client to be signed by the user
    #[serde(with = "hex")]
    pub challenge: [u8; 32],
    /// Expires at timestamp in milliseconds since epoch
    /// the time when the challenge will expire and should not be used anymore
    pub expires_at: i64,
}

/// Represents the challenge solution sent from the client to the server after signing the challenge string.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifyChallengeRequest {
    /// The signed challenge string sent from the client to the server
    #[serde(with = "hex")]
    pub signature: Vec<u8>,
}

/// Represents the response sent from the server to the client after verifying the challenge solution.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VerifyChallengeResponse {
    /// The challenge was verified successfully
    Verified {
        /// The access token to be used for authentication from now on
        access_token: String,
        /// A unix timestamp in milliseconds since epoch at which the access token will expire
        expires_at: i64,
    },
    /// The challenge was not verified because the challenge has expired
    Expired,
    /// The challenge was not verified because the signature is invalid
    InvalidSignature,
}
