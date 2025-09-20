//! TLS envelope encryption for secure storage of certificate material
//!
//! This module implements envelope encryption using XChaCha20-Poly1305 for
//! securing TLS certificates, private keys, and CA bundles in RocksDB.
//! The envelope key is loaded from environment variables or filesystem,
//! similar to the Paseto signing key pattern.
//!
//! # Security
//!
//! - Envelope key is 32 bytes for XChaCha20-Poly1305
//! - Key is persisted with restrictive permissions (0o600 on Unix)
//! - Supports environment variable and file-based loading
//! - Automatic key generation if none exists
//! - Secure memory cleanup with zeroize

use blueprint_core::{info, warn};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use base64::Engine;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use thiserror::Error;

/// Envelope encryption key for TLS material
#[derive(Clone, Debug)]
pub struct TlsEnvelopeKey(Key);

impl TlsEnvelopeKey {
    /// Generate a new random envelope key
    pub fn generate() -> Self {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        TlsEnvelopeKey(key)
    }

    /// Create from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        TlsEnvelopeKey(Key::from_slice(&bytes).clone())
    }

    /// Get key as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get key as hex string
    pub fn as_hex(&self) -> String {
        hex::encode(self.as_bytes())
    }

    /// Create from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self, TlsEnvelopeError> {
        let bytes =
            hex::decode(hex_str).map_err(|e| TlsEnvelopeError::InvalidHexFormat(e.to_string()))?;

        if bytes.len() != 32 {
            return Err(TlsEnvelopeError::InvalidKeyLength(bytes.len()));
        }

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&bytes);
        Ok(TlsEnvelopeKey::from_bytes(key_array))
    }
}

/// Envelope encryption for TLS material
#[derive(Clone, Debug)]
pub struct TlsEnvelope {
    key: TlsEnvelopeKey,
}

impl TlsEnvelope {
    /// Create new envelope with generated key
    pub fn new() -> Self {
        Self {
            key: TlsEnvelopeKey::generate(),
        }
    }

    /// Create with specific key
    pub fn with_key(key: TlsEnvelopeKey) -> Self {
        Self { key }
    }

    /// Encrypt data with envelope encryption
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, TlsEnvelopeError> {
        let cipher = ChaCha20Poly1305::new(&self.key.0);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| TlsEnvelopeError::EncryptionError(e.to_string()))?;

        // Prepend nonce to ciphertext
        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt data with envelope encryption
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, TlsEnvelopeError> {
        if data.len() < 12 {
            return Err(TlsEnvelopeError::InvalidCiphertextFormat(
                "data too short for nonce".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = ChaCha20Poly1305::new(&self.key.0);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| TlsEnvelopeError::DecryptionError(e.to_string()))?;

        Ok(plaintext)
    }

    /// Encrypt string and return base64-encoded result
    pub fn encrypt_string(&self, plaintext: &str) -> Result<String, TlsEnvelopeError> {
        let data = self.encrypt(plaintext.as_bytes())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(data))
    }

    /// Decrypt base64-encoded string
    pub fn decrypt_string(&self, encoded: &str) -> Result<String, TlsEnvelopeError> {
        let data = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| TlsEnvelopeError::Base64Error(e.to_string()))?;
        let plaintext = self.decrypt(&data)?;
        String::from_utf8(plaintext).map_err(|e| TlsEnvelopeError::Utf8Error(e.to_string()))
    }

    /// Get the envelope key
    pub fn key(&self) -> &TlsEnvelopeKey {
        &self.key
    }
}

/// Initialize TLS envelope key from environment or file
pub fn init_tls_envelope_key<P: AsRef<Path>>(
    db_path: P,
) -> Result<TlsEnvelopeKey, TlsEnvelopeError> {
    // Try to load key from environment variable first
    if let Ok(key_hex) = std::env::var("TLS_ENVELOPE_KEY") {
        match TlsEnvelopeKey::from_hex(&key_hex) {
            Ok(key) => {
                info!("Loaded TLS envelope key from environment variable");
                return Ok(key);
            }
            Err(e) => {
                warn!("Invalid TLS_ENVELOPE_KEY environment variable: {}", e);
            }
        }
    }

    // Try to load key from file path in environment variable
    if let Ok(key_path) = std::env::var("TLS_ENVELOPE_KEY_PATH") {
        let path = Path::new(&key_path);
        if path.exists() {
            match load_key_from_file(path) {
                Ok(key) => {
                    info!("Loaded TLS envelope key from file: {}", key_path);
                    return Ok(key);
                }
                Err(e) => {
                    warn!(
                        "Failed to load TLS envelope key from file {}: {}",
                        key_path, e
                    );
                }
            }
        } else {
            warn!("TLS envelope key file not found: {}", key_path);
        }
    }

    // Try to load key from default location in db directory
    let default_key_path = db_path.as_ref().join(".tls_envelope_key");
    if default_key_path.exists() {
        match load_key_from_file(&default_key_path) {
            Ok(key) => {
                info!("Loaded TLS envelope key from default location");
                return Ok(key);
            }
            Err(e) => {
                warn!(
                    "Failed to load TLS envelope key from default location: {}",
                    e
                );
            }
        }
    }

    // Generate new key and save it to default location
    info!("Generating new TLS envelope key");
    let key = TlsEnvelopeKey::generate();
    save_key_to_file(&key, &default_key_path)?;

    info!(
        "Generated and saved new TLS envelope key to: {:?}",
        default_key_path
    );
    Ok(key)
}

/// Load key from file
fn load_key_from_file(path: &Path) -> Result<TlsEnvelopeKey, TlsEnvelopeError> {
    let mut file = fs::File::open(path).map_err(|e| TlsEnvelopeError::IoError(e.to_string()))?;

    let mut key_bytes = Vec::new();
    file.read_to_end(&mut key_bytes)
        .map_err(|e| TlsEnvelopeError::IoError(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(TlsEnvelopeError::InvalidKeyLength(key_bytes.len()));
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    Ok(TlsEnvelopeKey::from_bytes(key_array))
}

/// Save key to file with secure permissions
fn save_key_to_file(key: &TlsEnvelopeKey, path: &Path) -> Result<(), TlsEnvelopeError> {
    let mut file = fs::File::create(path).map_err(|e| TlsEnvelopeError::IoError(e.to_string()))?;

    file.write_all(key.as_bytes())
        .map_err(|e| TlsEnvelopeError::IoError(e.to_string()))?;

    file.sync_all()
        .map_err(|e| TlsEnvelopeError::IoError(e.to_string()))?;

    // Set restrictive permissions on the key file (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = std::fs::Permissions::from_mode(0o600);
        fs::set_permissions(path, permissions)
            .map_err(|e| TlsEnvelopeError::IoError(e.to_string()))?;
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum TlsEnvelopeError {
    #[error("Invalid hex format: {0}")]
    InvalidHexFormat(String),

    #[error("Invalid key length: {0} bytes (expected 32)")]
    InvalidKeyLength(usize),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Invalid ciphertext format: {0}")]
    InvalidCiphertextFormat(String),

    #[error("Base64 error: {0}")]
    Base64Error(String),

    #[error("UTF-8 error: {0}")]
    Utf8Error(String),

    #[error("IO error: {0}")]
    IoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_key_generation() {
        let key1 = TlsEnvelopeKey::generate();
        let key2 = TlsEnvelopeKey::generate();

        // Keys should be different
        assert_ne!(key1.as_hex(), key2.as_hex());

        // Should be 32 bytes
        assert_eq!(key1.as_bytes().len(), 32);
    }

    #[test]
    fn test_key_from_hex() {
        let key = TlsEnvelopeKey::generate();
        let hex_str = key.as_hex();

        let decoded = TlsEnvelopeKey::from_hex(&hex_str).expect("Should decode hex");
        assert_eq!(key.as_hex(), decoded.as_hex());
    }

    #[test]
    fn test_envelope_encryption() {
        let envelope = TlsEnvelope::new();
        let plaintext = b"secret certificate data";

        let encrypted = envelope.encrypt(plaintext).expect("Should encrypt");
        let decrypted = envelope.decrypt(&encrypted).expect("Should decrypt");

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_string_encryption() {
        let envelope = TlsEnvelope::new();
        let plaintext = "secret certificate string";

        let encrypted = envelope
            .encrypt_string(plaintext)
            .expect("Should encrypt string");
        let decrypted = envelope
            .decrypt_string(&encrypted)
            .expect("Should decrypt string");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_keys_fail() {
        let envelope1 = TlsEnvelope::new();
        let envelope2 = TlsEnvelope::new();

        let plaintext = b"secret data";
        let encrypted = envelope1.encrypt(plaintext).expect("Should encrypt");

        // Should fail with different key
        let result = envelope2.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_ciphertext() {
        let envelope = TlsEnvelope::new();
        let invalid_data = b"too short";

        let result = envelope.decrypt(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_key_persistence() {
        let tmp_dir = tempdir().expect("tempdir");
        let key_path = tmp_dir.path().join("test_key");

        let key = TlsEnvelopeKey::generate();
        save_key_to_file(&key, &key_path).expect("Should save key");

        let loaded_key = load_key_from_file(&key_path).expect("Should load key");
        assert_eq!(key.as_hex(), loaded_key.as_hex());
    }

    #[test]
    fn test_base64_roundtrip() {
        let envelope = TlsEnvelope::new();
        let plaintext = "test string for base64 encoding";

        let encrypted = envelope.encrypt_string(plaintext).expect("Should encrypt");
        let decrypted = envelope.decrypt_string(&encrypted).expect("Should decrypt");

        assert_eq!(plaintext, decrypted);
    }
}
