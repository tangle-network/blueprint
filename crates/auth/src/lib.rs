//! Authentication module for the Blueprint SDK.

use rand::{CryptoRng, Rng};

/// Generates API Tokens for the authentication process.
pub mod api_tokens;
/// The database module for the authentication process.
pub mod db;
/// Database models
pub mod models;
/// Authenticated Proxy Server built on top of Axum.
pub mod proxy;
/// Holds the authentication-related types.
pub mod types;

#[cfg(test)]
mod test_client;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error related to the ECDSA signature verification.
    #[error(transparent)]
    K256(#[from] k256::ecdsa::Error),
    /// Error related to the SR25519 signature verification.
    #[error("Schnorrkel error: {0}")]
    Schnorrkel(schnorrkel::SignatureError),

    #[error(transparent)]
    RocksDB(#[from] rocksdb::Error),

    #[error("Invalid database compaction style: {0}")]
    InvalidDBCompactionStyle(String),

    #[error("Invalid database compression type: {0}")]
    InvalidDBCompressionType(String),

    #[error("unknown database column family: {0}")]
    UnknownColumnFamily(&'static str),

    #[error(transparent)]
    ProtobufDecode(#[from] prost::DecodeError),

    #[error("Unknown key type")]
    UnknownKeyType,

    #[error(transparent)]
    Uri(#[from] axum::http::uri::InvalidUri),
}

/// Generates a random challenge string to be used in the authentication process.
///
/// This should be sent to the client to be signed with the user's private key.
pub fn generate_challenge<R: Rng + CryptoRng>(rng: &mut R) -> [u8; 32] {
    let mut challenge = [0u8; 32];
    rng.fill(&mut challenge);
    challenge
}

/// Verifies the challenge solution sent from the client.
pub fn verify_challenge(
    challenge: &[u8; 32],
    signature: &[u8],
    pub_key: &[u8],
    key_type: types::KeyType,
) -> Result<bool, Error> {
    match key_type {
        types::KeyType::Unknown => Err(Error::UnknownKeyType),
        types::KeyType::Ecdsa => verify_challenge_ecdsa(challenge, signature, pub_key),
        types::KeyType::Sr25519 => verify_challenge_sr25519(challenge, signature, pub_key),
    }
}

/// Verifies the challenge solution using ECDSA.
fn verify_challenge_ecdsa(
    challenge: &[u8; 32],
    signature: &[u8],
    pub_key: &[u8],
) -> Result<bool, Error> {
    use k256::ecdsa::signature::hazmat::PrehashVerifier;
    let pub_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(pub_key)?;
    let signature = k256::ecdsa::Signature::try_from(signature)?;
    Ok(pub_key.verify_prehash(challenge, &signature).is_ok())
}

/// Verifies the challenge solution using SR25519.
///
/// Note: the signing context is `tangle` and the challenge is passed as bytes, not hashed.
fn verify_challenge_sr25519(
    challenge: &[u8; 32],
    signature: &[u8],
    pub_key: &[u8],
) -> Result<bool, Error> {
    let pub_key = schnorrkel::PublicKey::from_bytes(pub_key).map_err(Error::Schnorrkel)?;
    let signature = schnorrkel::Signature::from_bytes(signature).map_err(Error::Schnorrkel)?;
    let mut ctx = schnorrkel::signing_context(b"tangle").bytes(challenge);
    pub_key
        .verify(&mut ctx, &signature)
        .map_err(Error::Schnorrkel)?;
    Ok(true)
}
