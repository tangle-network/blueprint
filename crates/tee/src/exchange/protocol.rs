//! Key exchange protocol types.
//!
//! Implements the two-phase key exchange flow:
//! 1. TEE generates an ephemeral keypair and produces attestation binding the public key.
//! 2. Client verifies attestation and encrypts secrets to the ephemeral public key.

use crate::attestation::report::AttestationReport;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use zeroize::Zeroize;

/// An ephemeral session for key exchange.
///
/// Each session generates a random 32-byte keypair. The private key is held
/// in memory and zeroed on drop via `write_volatile`. Sessions are one-time
/// use: once consumed, the session cannot be reused.
///
/// # WARNING: Placeholder — not production crypto
///
/// The current public key derivation uses `SHA-256(private_key)` instead of
/// proper X25519 point multiplication. This is a structural placeholder.
/// A production implementation must use `x25519-dalek` or equivalent for
/// real Diffie-Hellman key exchange.
#[derive(Debug)]
pub struct KeyExchangeSession {
    /// Unique hex-encoded session identifier (16 random bytes).
    pub session_id: String,
    /// The ephemeral public key.
    ///
    /// WARNING: Currently `SHA-256(private_key)` — not a real X25519 public point.
    /// See struct-level docs for details.
    pub public_key: Vec<u8>,
    /// The ephemeral private key (raw bytes), held only in TEE memory.
    /// Zeroed on drop.
    private_key: Vec<u8>,
    /// Unix timestamp when this session was created.
    pub created_at: u64,
    /// Maximum lifetime in seconds from `created_at`.
    pub ttl_secs: u64,
}

impl KeyExchangeSession {
    /// Create a new ephemeral key exchange session.
    ///
    /// Generates a random 32-byte keypair. The public key is currently derived
    /// as `SHA-256(private_key)` — **not** proper X25519. See struct-level docs.
    pub fn new(ttl_secs: u64) -> Self {
        let mut rng = rand::thread_rng();

        // Generate random key material (32 bytes)
        let mut private_key = vec![0u8; 32];
        rng.fill_bytes(&mut private_key);

        // WARNING: placeholder key derivation — production must use x25519-dalek
        // or equivalent for real Diffie-Hellman key exchange.
        let public_key = Sha256::digest(&private_key).to_vec();

        tracing::warn!("key exchange uses placeholder SHA-256 derivation, not real X25519");

        // Generate session ID
        let mut session_id_bytes = [0u8; 16];
        rng.fill_bytes(&mut session_id_bytes);
        let session_id = hex::encode(session_id_bytes);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            session_id,
            public_key,
            private_key,
            created_at: now,
            ttl_secs,
        }
    }

    /// Check if this session has expired.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now.saturating_sub(self.created_at) > self.ttl_secs
    }

    /// Get the public key hash for binding in an attestation report.
    pub fn public_key_digest(&self) -> String {
        hex::encode(Sha256::digest(&self.public_key))
    }

    /// Get the remaining TTL as a Duration.
    pub fn remaining_ttl(&self) -> Duration {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let elapsed = now.saturating_sub(self.created_at);
        Duration::from_secs(self.ttl_secs.saturating_sub(elapsed))
    }
}

impl Drop for KeyExchangeSession {
    fn drop(&mut self) {
        self.private_key.zeroize();
    }
}

/// A request to retrieve an ephemeral public key for key exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeRequest {
    /// Optional nonce/challenge from the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// Response to a [`KeyExchangeRequest`], containing the ephemeral public key
/// and the attestation report that binds it to the TEE.
///
/// The client uses this to verify the TEE's identity (both locally and
/// optionally against the on-chain attestation hash) before encrypting
/// secrets to the ephemeral key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeResponse {
    /// The session identifier.
    pub session_id: String,
    /// The ephemeral public key (hex-encoded).
    pub public_key_hex: String,
    /// The attestation report binding this public key.
    pub attestation: AttestationReport,
}

/// A sealed secret payload encrypted to the TEE's ephemeral public key.
///
/// Clients construct this after verifying the [`KeyExchangeResponse`] attestation.
/// The ciphertext is encrypted to the session's ephemeral public key and can only
/// be decrypted inside the TEE that holds the corresponding private key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedSecretPayload {
    /// The session ID this payload is sealed for.
    pub session_id: String,
    /// The encrypted secret bytes.
    pub ciphertext: Vec<u8>,
    /// Encryption nonce (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Vec<u8>>,
}

/// Result of a sealed secret injection into the TEE.
///
/// Returned after the TEE decrypts and installs the sealed secret. Contains
/// the attestation digest and key fingerprint at the time of injection for
/// audit purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedSecretResult {
    /// Whether the injection was successful.
    pub success: bool,
    /// The attestation digest at the time of injection.
    pub attestation_digest: String,
    /// The public key fingerprint used.
    pub key_fingerprint: String,
}
