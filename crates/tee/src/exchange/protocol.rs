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

/// An ephemeral session for key exchange.
#[derive(Debug)]
pub struct KeyExchangeSession {
    /// Unique session identifier.
    pub session_id: String,
    /// The ephemeral public key (raw bytes).
    pub public_key: Vec<u8>,
    /// The ephemeral private key (raw bytes), held only in TEE memory.
    private_key: Vec<u8>,
    /// When this session was created.
    pub created_at: u64,
    /// Time-to-live in seconds.
    pub ttl_secs: u64,
    /// Whether this session has been consumed (one-time use).
    pub consumed: bool,
}

impl KeyExchangeSession {
    /// Create a new ephemeral key exchange session.
    ///
    /// Generates a random 32-byte keypair (suitable as X25519 seed or
    /// similar key material).
    pub fn new(ttl_secs: u64) -> Self {
        let mut rng = rand::thread_rng();

        // Generate random key material (32 bytes)
        let mut private_key = vec![0u8; 32];
        rng.fill_bytes(&mut private_key);

        // Derive public key hash (in a real implementation, this would be
        // proper X25519 or similar key derivation)
        let public_key = Sha256::digest(&private_key).to_vec();

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
            consumed: false,
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

    /// Check if this session is still valid (not expired and not consumed).
    pub fn is_valid(&self) -> bool {
        !self.consumed && !self.is_expired()
    }

    /// Mark this session as consumed.
    pub fn consume(&mut self) {
        self.consumed = true;
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

/// A request to retrieve an ephemeral public key for key exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeRequest {
    /// Optional nonce/challenge from the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// Response containing the ephemeral public key and attestation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeResponse {
    /// The session identifier.
    pub session_id: String,
    /// The ephemeral public key (hex-encoded).
    pub public_key_hex: String,
    /// The attestation report binding this public key.
    pub attestation: AttestationReport,
}

/// A sealed secret payload encrypted to the ephemeral public key.
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

/// Result of a sealed secret injection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedSecretResult {
    /// Whether the injection was successful.
    pub success: bool,
    /// The attestation digest at the time of injection.
    pub attestation_digest: String,
    /// The public key fingerprint used.
    pub key_fingerprint: String,
}
