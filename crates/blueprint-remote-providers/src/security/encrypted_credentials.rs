//! Encrypted credential storage to replace plaintext CloudCredentials
//!
//! Provides secure storage for cloud provider credentials using AES-GCM encryption

use crate::core::error::{Error, Result};
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use serde::{Deserialize, Serialize};
use blueprint_std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Encrypted storage for cloud provider credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCloudCredentials {
    /// Provider identifier
    pub provider: String,
    /// Encrypted credential blob
    encrypted_data: Vec<u8>,
    /// Nonce for decryption
    nonce: Vec<u8>,
    /// Metadata (non-sensitive)
    pub metadata: HashMap<String, String>,
}

/// Plaintext credential data (only exists during encryption/decryption)
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, Default)]
pub struct PlaintextCredentials {
    // AWS
    pub aws_access_key: Option<String>,
    pub aws_secret_key: Option<String>,

    // GCP
    pub gcp_project_id: Option<String>,
    pub gcp_service_account_key: Option<String>,

    // Azure
    pub azure_subscription_id: Option<String>,
    pub azure_client_id: Option<String>,
    pub azure_client_secret: Option<String>,
    pub azure_tenant_id: Option<String>,

    // DigitalOcean
    pub do_api_token: Option<String>,

    // Vultr
    pub vultr_api_key: Option<String>,
}

impl EncryptedCloudCredentials {
    /// Create new encrypted credentials with provided key
    pub fn encrypt_with_key(
        provider: &str,
        credentials: PlaintextCredentials,
        key: &[u8; 32],
    ) -> Result<Self> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| Error::ConfigurationError(format!("Invalid key: {e}")))?;

        // Generate random nonce
        let nonce_bytes = Self::generate_nonce();
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Serialize and encrypt credentials
        let plaintext = serde_json::to_vec(&credentials)
            .map_err(|e| Error::ConfigurationError(format!("Serialization failed: {e}")))?;

        let encrypted_data = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| Error::ConfigurationError(format!("Encryption failed: {e}")))?;

        Ok(Self {
            provider: provider.to_string(),
            encrypted_data,
            nonce: nonce.to_vec(),
            metadata: HashMap::new(),
        })
    }

    /// Decrypt credentials (temporarily exposes plaintext)
    pub fn decrypt(&self, key: &[u8; 32]) -> Result<PlaintextCredentials> {
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| Error::ConfigurationError(format!("Invalid key: {e}")))?;

        let nonce = Nonce::from_slice(&self.nonce);

        let plaintext = cipher
            .decrypt(nonce, self.encrypted_data.as_ref())
            .map_err(|e| Error::ConfigurationError(format!("Decryption failed: {e}")))?;

        let credentials: PlaintextCredentials = serde_json::from_slice(&plaintext)
            .map_err(|e| Error::ConfigurationError(format!("Deserialization failed: {e}")))?;

        Ok(credentials)
    }

    /// Generate secure random nonce
    fn generate_nonce() -> [u8; 12] {
        use rand::RngCore;
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }

    /// Add non-sensitive metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get provider type
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Check if credentials are encrypted
    pub fn is_encrypted(&self) -> bool {
        !self.encrypted_data.is_empty()
    }
}

/// Secure credential manager with key derivation
#[derive(Debug)]
pub struct SecureCredentialManager {
    master_key: [u8; 32],
}

impl SecureCredentialManager {
    /// Create new credential manager with derived key
    pub fn new(password: &str, salt: &[u8]) -> Result<Self> {
        // In production, use proper key derivation (PBKDF2, Argon2, etc.)
        let mut key = [0u8; 32];
        let combined = format!("{}{}", password, hex::encode(salt));
        let hash = blake3::hash(combined.as_bytes());
        key.copy_from_slice(hash.as_bytes());

        Ok(Self { master_key: key })
    }

    /// Encrypt and store credentials
    pub fn store_credentials(
        &self,
        provider: &str,
        credentials: PlaintextCredentials,
    ) -> Result<EncryptedCloudCredentials> {
        let mut encrypted =
            EncryptedCloudCredentials::encrypt_with_key(provider, credentials, &self.master_key)?;
        encrypted.add_metadata("created_at".to_string(), chrono::Utc::now().to_rfc3339());
        encrypted.add_metadata("version".to_string(), "1.0".to_string());
        Ok(encrypted)
    }

    /// Decrypt and retrieve credentials
    pub fn retrieve_credentials(
        &self,
        encrypted: &EncryptedCloudCredentials,
    ) -> Result<PlaintextCredentials> {
        encrypted.decrypt(&self.master_key)
    }

    /// Validate encrypted credentials
    pub fn validate_credentials(&self, encrypted: &EncryptedCloudCredentials) -> Result<bool> {
        match self.retrieve_credentials(encrypted) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// Secure AWS credential extraction
impl PlaintextCredentials {
    /// Create from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| Error::ConfigurationError(format!("Invalid JSON: {e}")))
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn aws_credentials(&self) -> Option<(&str, &str)> {
        match (&self.aws_access_key, &self.aws_secret_key) {
            (Some(access), Some(secret)) => Some((access, secret)),
            _ => None,
        }
    }

    pub fn gcp_credentials(&self) -> Option<(&str, &str)> {
        match (&self.gcp_project_id, &self.gcp_service_account_key) {
            (Some(project), Some(key)) => Some((project, key)),
            _ => None,
        }
    }

    pub fn azure_credentials(&self) -> Option<(&str, &str, &str, &str)> {
        match (
            &self.azure_subscription_id,
            &self.azure_client_id,
            &self.azure_client_secret,
            &self.azure_tenant_id,
        ) {
            (Some(sub), Some(client), Some(secret), Some(tenant)) => {
                Some((sub, client, secret, tenant))
            }
            _ => None,
        }
    }

    pub fn digitalocean_token(&self) -> Option<&str> {
        self.do_api_token.as_deref()
    }

    pub fn vultr_api_key(&self) -> Option<&str> {
        self.vultr_api_key.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_encryption_decryption() {
        // Use proper encryption with known key (production pattern)
        let test_key: [u8; 32] = [0x42; 32]; // Test key

        let mut credentials = PlaintextCredentials::default();
        credentials.aws_access_key = Some("AKIATEST123".to_string());
        credentials.aws_secret_key = Some("secretkey123".to_string());
        credentials.gcp_project_id = Some("test-project".to_string());

        // Encrypt credentials with known key
        let encrypted =
            EncryptedCloudCredentials::encrypt_with_key("aws", credentials, &test_key).unwrap();
        assert!(encrypted.is_encrypted());
        assert_eq!(encrypted.provider(), "aws");

        // Successful decryption with correct key
        let decrypted = encrypted.decrypt(&test_key).unwrap();
        assert_eq!(decrypted.aws_access_key, Some("AKIATEST123".to_string()));
        assert_eq!(decrypted.aws_secret_key, Some("secretkey123".to_string()));
        assert_eq!(decrypted.gcp_project_id, Some("test-project".to_string()));

        // Decryption fails with wrong key
        let wrong_key = [0u8; 32];
        assert!(encrypted.decrypt(&wrong_key).is_err());
    }

    #[test]
    fn test_secure_credential_manager() {
        let manager = SecureCredentialManager::new("test_password", b"test_salt").unwrap();

        let mut credentials = PlaintextCredentials::default();
        credentials.aws_access_key = Some("AKIATEST123".to_string());
        credentials.aws_secret_key = Some("secretkey123".to_string());

        let encrypted = manager.store_credentials("aws", credentials).unwrap();
        assert!(encrypted.is_encrypted());

        let decrypted = manager.retrieve_credentials(&encrypted).unwrap();
        assert_eq!(decrypted.aws_access_key, Some("AKIATEST123".to_string()));

        assert!(manager.validate_credentials(&encrypted).unwrap());
    }

    #[test]
    fn test_credential_zeroization() {
        let mut credentials = PlaintextCredentials::default();
        credentials.aws_secret_key = Some("super_secret_key".to_string());

        // Zeroize should clear sensitive data
        credentials.zeroize();

        // After zeroization, values should be cleared
        assert!(
            credentials.aws_secret_key.is_none()
                || credentials.aws_secret_key.as_ref().unwrap().is_empty()
        );
    }
}
