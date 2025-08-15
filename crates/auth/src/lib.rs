//! Authentication module for the Blueprint SDK.
//!
//! This module provides a three-tier token authentication system:
//!
//! 1. **API Keys** (`ak_xxxxx.yyyyy`) - Long-lived credentials for service authentication
//! 2. **Access Tokens** (`v4.local.xxxxx`) - Short-lived Paseto tokens for authorization
//! 3. **Legacy Tokens** (`id|token`) - Deprecated format for backward compatibility
//!
//! # Architecture
//!
//! The authentication flow follows these steps:
//! 1. Client authenticates with API key
//! 2. API key is exchanged for a short-lived access token
//! 3. Access token is used for subsequent requests
//! 4. Token refresh happens automatically before expiration
//!
//! # Security Features
//!
//! - Cryptographic tenant binding prevents impersonation
//! - Header re-validation prevents injection attacks  
//! - Persistent key storage with secure permissions
//! - Automatic token rotation and refresh
//!
//! # Example
//!
//! ```no_run
//! use blueprint_auth::proxy::AuthenticatedProxy;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize proxy with persistent storage
//!     let proxy = AuthenticatedProxy::new("/var/lib/auth/db")?;
//!
//!     // Start the proxy server
//!     let router = proxy.router();
//!     Ok(())
//! }
//! ```

use blueprint_std::rand::{CryptoRng, Rng};

/// Long-lived API key management
pub mod api_keys;
/// Generates API Tokens for the authentication process.
pub mod api_tokens;
/// Unified authentication token types
pub mod auth_token;
/// The database module for the authentication process.
pub mod db;
/// Database models
pub mod models;
/// Paseto token generation and validation
pub mod paseto_tokens;
/// Authenticated Proxy Server built on top of Axum.
pub mod proxy;
/// Holds the authentication-related types.
pub mod types;
/// Header validation utilities
pub mod validation;

#[cfg(test)]
mod test_client;

#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error related to the ECDSA signature verification.
    #[error("k256 error: {0}")]
    K256(k256::ecdsa::Error),
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

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
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
    let pub_key = k256::ecdsa::VerifyingKey::from_sec1_bytes(pub_key).map_err(Error::K256)?;
    let signature = k256::ecdsa::Signature::try_from(signature).map_err(Error::K256)?;
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
    // We must make sure that this is the same as declared in the substrate source code.
    const CTX: &[u8] = b"substrate";
    let pub_key = schnorrkel::PublicKey::from_bytes(pub_key).map_err(Error::Schnorrkel)?;
    let signature = schnorrkel::Signature::from_bytes(signature).map_err(Error::Schnorrkel)?;
    Ok(pub_key.verify_simple(CTX, challenge, &signature).is_ok())
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    use crate::types::{KeyType, VerifyChallengeRequest};
    use k256::ecdsa::SigningKey;

    #[test]
    fn test_generate_challenge() {
        // Test with system RNG
        let mut rng = blueprint_std::BlueprintRng::new();
        let challenge1 = generate_challenge(&mut rng);

        // Generate another challenge with the same RNG
        let challenge2 = generate_challenge(&mut rng);

        // Challenges should be different
        assert_ne!(challenge1, challenge2);

        // Should produce a non-zero challenge
        assert_ne!(challenge1, [0u8; 32]);
    }

    #[test]
    fn test_verify_challenge_ecdsa_valid() {
        let mut rng = blueprint_std::BlueprintRng::new();

        // Generate a random challenge
        let challenge = generate_challenge(&mut rng);

        // Generate a key pair
        let signing_key = SigningKey::random(&mut rng);
        let verification_key = signing_key.verifying_key();
        let public_key = verification_key.to_sec1_bytes();

        // Sign the challenge
        let signature = &signing_key.sign_prehash_recoverable(&challenge).unwrap().0;

        // Verify the signature
        let result =
            verify_challenge_ecdsa(&challenge, signature.to_bytes().as_slice(), &public_key);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_challenge_ecdsa_invalid_signature() {
        let mut rng = blueprint_std::BlueprintRng::new();

        // Generate random challenges
        let challenge = generate_challenge(&mut rng);
        let different_challenge = generate_challenge(&mut rng);

        // Generate a key pair
        let signing_key = SigningKey::random(&mut rng);
        let verification_key = signing_key.verifying_key();
        let public_key = verification_key.to_sec1_bytes();

        // Sign a different challenge
        let signature = &signing_key
            .sign_prehash_recoverable(&different_challenge)
            .unwrap()
            .0;
        // Verification should fail because signature doesn't match the challenge
        let result =
            verify_challenge_ecdsa(&challenge, signature.to_bytes().as_slice(), &public_key);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_challenge_ecdsa_invalid_key() {
        let mut rng = blueprint_std::BlueprintRng::new();

        // Generate a random challenge
        let challenge = generate_challenge(&mut rng);

        // Generate a key pair
        let signing_key = SigningKey::random(&mut rng);

        // Generate a different key pair
        let different_signing_key = SigningKey::random(&mut rng);
        let different_verification_key = different_signing_key.verifying_key();
        let different_public_key = different_verification_key.to_sec1_bytes();

        // Sign with first key
        let signature = &signing_key.sign_prehash_recoverable(&challenge).unwrap().0;

        // Verify with second key - should fail
        let result = verify_challenge_ecdsa(
            &challenge,
            signature.to_bytes().as_slice(),
            &different_public_key,
        );
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_challenge_unknown_key_type() {
        let mut rng = blueprint_std::BlueprintRng::new();
        let challenge = generate_challenge(&mut rng);
        let result = verify_challenge(&challenge, &[0u8; 64], &[0u8; 33], KeyType::Unknown);
        assert!(matches!(result, Err(Error::UnknownKeyType)));
    }

    #[test]
    fn test_verify_challenge_integration() {
        let mut rng = blueprint_std::BlueprintRng::new();

        // Generate a random challenge
        let challenge = generate_challenge(&mut rng);

        // Generate an ECDSA key pair
        let signing_key = SigningKey::random(&mut rng);
        let verification_key = signing_key.verifying_key();
        let public_key = verification_key.to_sec1_bytes();

        // Sign the challenge with ECDSA
        let signature = &signing_key.sign_prehash_recoverable(&challenge).unwrap().0;

        // Verify via the main verify_challenge function
        let result = verify_challenge(
            &challenge,
            signature.to_bytes().as_slice(),
            &public_key,
            KeyType::Ecdsa,
        );
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_challenge_sr25519_error_handling() {
        let mut rng = blueprint_std::BlueprintRng::new();
        let challenge = generate_challenge(&mut rng);

        // Create invalid signature and public key data
        let invalid_signature = [0u8; 64];
        let invalid_pub_key = [0u8; 32];

        // This should return an error since the signature and public key are invalid
        let result = verify_challenge_sr25519(&challenge, &invalid_signature, &invalid_pub_key);
        assert!(result.is_err());

        // Verify the error is a Schnorrkel error
        match result {
            Err(Error::Schnorrkel(_)) => {}
            _ => panic!("Expected Schnorrkel error"),
        }
    }

    #[test]
    fn js_compat_ecdsa() {
        // Test ECDSA compatibility with JavaScript
        // See `fixtures/sign.ts` for the original JavaScript code that generates this data.
        let data = serde_json::json!({
            "pub_key": "020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1",
            "key_type": "Ecdsa",
            "challenge": "0000000000000000000000000000000000000000000000000000000000000000",
            "signature": "26138be19cfc76e800bdcbba5e3bbc5bd79168cd06ea6afd5be6860d23d5e0340c728508ca0b47b49627b5560fbca6cdd92cbf6ac402d0941bba7e42b9d7a20c",
            "expires_at": 0
        });

        let req: VerifyChallengeRequest = serde_json::from_value(data).unwrap();
        let result = verify_challenge(
            &req.challenge,
            &req.signature,
            &req.challenge_request.pub_key,
            req.challenge_request.key_type,
        );
        assert!(
            result.is_ok(),
            "Failed to verify ECDSA challenge: {}",
            result.err().unwrap()
        );
        assert!(result.is_ok(), "ECDSA verification failed");
    }

    #[test]
    fn js_compat_sr25519() {
        // Test Sr25519 compatibility with JavaScript
        // See `fixtures/sign_sr25519.ts` for the original JavaScript code that generates this data.
        let data = serde_json::json!({
            "pub_key": "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
            "key_type": "Sr25519",
            "challenge": "0000000000000000000000000000000000000000000000000000000000000000",
            "signature": "f05fa2a2074d5295a34aae0d5383792a6cc34304c9cb4f6a0c577df4b374fe7bab051bd7570415578ba2da67e056d8f89b420d2e5b82412dc0f0e02877b9e48c",
            "expires_at": 0
        });

        let req: VerifyChallengeRequest = serde_json::from_value(data).unwrap();
        let result = verify_challenge(
            &req.challenge,
            &req.signature,
            &req.challenge_request.pub_key,
            req.challenge_request.key_type,
        );
        assert!(
            result.is_ok(),
            "Failed to verify Sr25519 challenge: {}",
            result.err().unwrap()
        );
        assert!(result.is_ok(), "Sr25519 verification failed");
    }
}
