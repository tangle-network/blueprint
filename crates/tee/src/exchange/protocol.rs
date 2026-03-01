//! Key exchange protocol types.
//!
//! Implements the two-phase key exchange flow:
//! 1. TEE generates an ephemeral X25519 keypair and produces attestation binding the public key.
//! 2. Client verifies attestation, generates their own ephemeral X25519 keypair, computes the
//!    shared secret via Diffie-Hellman, encrypts secrets with ChaCha20Poly1305, and sends the
//!    ciphertext along with their ephemeral public key.
//! 3. TEE computes the same shared secret from the client's public key and decrypts.

use crate::attestation::report::AttestationReport;
use crate::errors::TeeError;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::ChaCha20Poly1305;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::Zeroize;

/// An ephemeral session for key exchange.
///
/// Each session generates a random X25519 keypair. The private key is held
/// in memory and zeroed on drop via [`Zeroize`]. Sessions are one-time
/// use: once consumed, the session cannot be reused.
#[derive(Debug)]
pub struct KeyExchangeSession {
    /// Unique hex-encoded session identifier (16 random bytes).
    pub session_id: String,
    /// The ephemeral X25519 public key (32 bytes).
    pub public_key: Vec<u8>,
    /// The ephemeral X25519 private key (raw bytes), held only in TEE memory.
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
    /// Generates a random X25519 keypair via `x25519-dalek`. The public key is the
    /// Curve25519 point derived from the private key via scalar multiplication.
    pub fn new(ttl_secs: u64) -> Self {
        let secret = StaticSecret::random_from_rng(rand::rngs::OsRng);
        let public = PublicKey::from(&secret);
        let public_key = public.as_bytes().to_vec();
        let private_key = secret.to_bytes().to_vec();

        // Generate session ID
        let mut session_id_bytes = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut session_id_bytes);
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

    /// Decrypt a sealed secret payload using this session's private key.
    ///
    /// Reconstructs the X25519 shared secret from the client's ephemeral public key,
    /// derives a symmetric key via SHA-256, and decrypts with ChaCha20Poly1305.
    pub fn open(&self, payload: &SealedSecretPayload) -> Result<Vec<u8>, TeeError> {
        let client_public_bytes: [u8; 32] = payload
            .ephemeral_public_key
            .as_ref()
            .ok_or_else(|| TeeError::SealedSecret("missing ephemeral public key".into()))?
            .as_slice()
            .try_into()
            .map_err(|_| TeeError::SealedSecret("ephemeral public key must be 32 bytes".into()))?;

        let nonce_bytes = payload
            .nonce
            .as_ref()
            .ok_or_else(|| TeeError::SealedSecret("missing nonce".into()))?;
        if nonce_bytes.len() != 12 {
            return Err(TeeError::SealedSecret("nonce must be 12 bytes".into()));
        }

        let secret_bytes: [u8; 32] = self
            .private_key
            .as_slice()
            .try_into()
            .map_err(|_| TeeError::SealedSecret("invalid private key length".into()))?;

        let secret = StaticSecret::from(secret_bytes);
        let client_public = PublicKey::from(client_public_bytes);
        let shared = secret.diffie_hellman(&client_public);

        let enc_key = Sha256::digest(shared.as_bytes());
        let cipher = ChaCha20Poly1305::new_from_slice(&enc_key)
            .map_err(|e| TeeError::SealedSecret(format!("cipher init failed: {e}")))?;

        let nonce = chacha20poly1305::aead::generic_array::GenericArray::from_slice(nonce_bytes);
        cipher
            .decrypt(nonce, payload.ciphertext.as_ref())
            .map_err(|_| {
                TeeError::SealedSecret("decryption failed: invalid ciphertext or key".into())
            })
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
/// The ciphertext is encrypted using ChaCha20Poly1305 with a symmetric key derived
/// from the X25519 shared secret, and can only be decrypted inside the TEE that
/// holds the corresponding private key.
///
/// Use [`SealedSecretPayload::seal`] to construct and [`KeyExchangeSession::open`]
/// to decrypt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedSecretPayload {
    /// The session ID this payload is sealed for.
    pub session_id: String,
    /// The encrypted secret bytes (ChaCha20Poly1305 ciphertext with appended tag).
    pub ciphertext: Vec<u8>,
    /// The 12-byte encryption nonce for ChaCha20Poly1305.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Vec<u8>>,
    /// The client's ephemeral X25519 public key (32 bytes) for DH key agreement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_public_key: Option<Vec<u8>>,
}

impl SealedSecretPayload {
    /// Encrypt a secret to the TEE's ephemeral public key.
    ///
    /// Generates a client-side ephemeral X25519 keypair, computes the shared
    /// secret via DH with the TEE's public key, derives a symmetric key via
    /// SHA-256, and encrypts with ChaCha20Poly1305.
    pub fn seal(
        session_id: String,
        plaintext: &[u8],
        tee_public_key: &[u8],
    ) -> Result<Self, TeeError> {
        let tee_pk_bytes: [u8; 32] = tee_public_key
            .try_into()
            .map_err(|_| TeeError::SealedSecret("TEE public key must be 32 bytes".into()))?;

        let client_secret = StaticSecret::random_from_rng(rand::rngs::OsRng);
        let client_public = PublicKey::from(&client_secret);
        let tee_public = PublicKey::from(tee_pk_bytes);

        let shared = client_secret.diffie_hellman(&tee_public);
        let enc_key = Sha256::digest(shared.as_bytes());
        let cipher = ChaCha20Poly1305::new_from_slice(&enc_key)
            .map_err(|e| TeeError::SealedSecret(format!("cipher init failed: {e}")))?;

        let mut nonce_bytes = [0u8; 12];
        rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = chacha20poly1305::aead::generic_array::GenericArray::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| TeeError::SealedSecret("encryption failed".into()))?;

        Ok(Self {
            session_id,
            ciphertext,
            nonce: Some(nonce_bytes.to_vec()),
            ephemeral_public_key: Some(client_public.as_bytes().to_vec()),
        })
    }
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
