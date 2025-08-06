use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{Local, version4::V4};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::types::ServiceId;

/// Paseto key for symmetric encryption/decryption
#[derive(Clone, Debug)]
pub struct PasetoKey(SymmetricKey<V4>);

impl PasetoKey {
    /// Generate a new random Paseto key
    pub fn generate() -> Self {
        use blueprint_std::rand::RngCore;
        let mut rng = blueprint_std::BlueprintRng::new();
        let mut key_bytes = [0u8; 32];
        rng.fill_bytes(&mut key_bytes);

        let key = SymmetricKey::<V4>::from(&key_bytes).expect("Valid 32-byte key");
        PasetoKey(key)
    }

    /// Create from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        let key = SymmetricKey::<V4>::from(&bytes).expect("Valid 32-byte key");
        PasetoKey(key)
    }

    /// Get key as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

/// Claims embedded in Paseto access tokens
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccessTokenClaims {
    /// Service ID this token is for
    pub service_id: ServiceId,
    /// Optional tenant identifier (hashed user ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Additional headers to forward to upstream
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub additional_headers: BTreeMap<String, String>,
    /// Token expiration timestamp (seconds since epoch)
    pub expires_at: u64,
    /// Token issued at timestamp (seconds since epoch)
    pub issued_at: u64,
    /// API key ID that was used to generate this token
    pub key_id: String,
    /// Unique token identifier for logging/debugging
    pub jti: String,
}

impl AccessTokenClaims {
    /// Create new claims with current timestamp
    pub fn new(
        service_id: ServiceId,
        key_id: String,
        ttl: Duration,
        tenant_id: Option<String>,
        additional_headers: BTreeMap<String, String>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            service_id,
            tenant_id,
            additional_headers,
            expires_at: now + ttl.as_secs(),
            issued_at: now,
            key_id,
            jti: Uuid::new_v4().to_string(),
        }
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now >= self.expires_at
    }

    /// Get time until expiration
    pub fn time_to_expiry(&self) -> Option<Duration> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if now >= self.expires_at {
            None
        } else {
            Some(Duration::from_secs(self.expires_at - now))
        }
    }
}

/// Paseto token generator and validator
#[derive(Clone, Debug)]
pub struct PasetoTokenManager {
    key: PasetoKey,
    default_ttl: Duration,
}

impl PasetoTokenManager {
    /// Create new token manager with generated key
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            key: PasetoKey::generate(),
            default_ttl,
        }
    }

    /// Create with specific key (for persistence across restarts)
    pub fn with_key(key: PasetoKey, default_ttl: Duration) -> Self {
        Self { key, default_ttl }
    }

    /// Generate a new Paseto access token
    pub fn generate_token(
        &self,
        service_id: ServiceId,
        key_id: String,
        tenant_id: Option<String>,
        additional_headers: BTreeMap<String, String>,
        ttl: Option<Duration>,
    ) -> Result<String, PasetoError> {
        let claims = AccessTokenClaims::new(
            service_id,
            key_id,
            ttl.unwrap_or(self.default_ttl),
            tenant_id,
            additional_headers,
        );

        // Create proper PASETO claims with standard fields
        let mut paseto_claims =
            Claims::new().map_err(|e| PasetoError::SerializationError(e.to_string()))?;

        // Add our custom claims as JSON
        let custom_json = serde_json::to_string(&claims)
            .map_err(|e| PasetoError::SerializationError(e.to_string()))?;
        paseto_claims
            .add_additional("data", custom_json)
            .map_err(|e| PasetoError::SerializationError(e.to_string()))?;

        pasetors::local::encrypt(&self.key.0, &paseto_claims, None, None)
            .map_err(|e| PasetoError::EncryptionError(e.to_string()))
    }

    /// Validate and decode a Paseto access token
    pub fn validate_token(&self, token: &str) -> Result<AccessTokenClaims, PasetoError> {
        let untrusted_token = UntrustedToken::<Local, V4>::try_from(token)
            .map_err(|e| PasetoError::DecryptionError(e.to_string()))?;

        // Create basic validation rules (no strict validation for now)
        let validation_rules = ClaimsValidationRules::new();

        let trusted_token =
            pasetors::local::decrypt(&self.key.0, &untrusted_token, &validation_rules, None, None)
                .map_err(|e| PasetoError::DecryptionError(e.to_string()))?;

        // Extract our custom data from the "data" field
        let token_claims = trusted_token.payload_claims().ok_or_else(|| {
            PasetoError::DeserializationError("Failed to parse payload claims".to_string())
        })?;

        let custom_data = token_claims
            .get_claim("data")
            .ok_or_else(|| PasetoError::DeserializationError("Missing data claim".to_string()))?;

        // Get string value from the claim
        let custom_json_str = custom_data.as_str().ok_or_else(|| {
            PasetoError::DeserializationError("Data claim is not a string".to_string())
        })?;

        let claims: AccessTokenClaims = serde_json::from_str(custom_json_str)
            .map_err(|e| PasetoError::DeserializationError(e.to_string()))?;

        if claims.is_expired() {
            return Err(PasetoError::TokenExpired);
        }

        Ok(claims)
    }

    /// Get the encryption key for persistence
    pub fn get_key(&self) -> &PasetoKey {
        &self.key
    }

    /// Get the default TTL
    pub fn default_ttl(&self) -> Duration {
        self.default_ttl
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PasetoError {
    #[error("Token is expired")]
    TokenExpired,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Decryption error: {0}")]
    DecryptionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_paseto_key_generation() {
        let key1 = PasetoKey::generate();
        let key2 = PasetoKey::generate();

        // Keys should be different
        assert_ne!(key1.0, key2.0);

        // Should be 32 bytes
        assert_eq!(key1.as_bytes().len(), 32);
    }

    #[test]
    fn test_access_token_claims_creation() {
        let service_id = ServiceId::new(1);
        let key_id = "ak_test123".to_string();
        let ttl = Duration::from_secs(900); // 15 minutes
        let tenant_id = Some("tenant123".to_string());
        let mut headers = BTreeMap::new();
        headers.insert("X-Custom".to_string(), "value".to_string());

        let claims = AccessTokenClaims::new(
            service_id,
            key_id.clone(),
            ttl,
            tenant_id.clone(),
            headers.clone(),
        );

        assert_eq!(claims.service_id, service_id);
        assert_eq!(claims.key_id, key_id);
        assert_eq!(claims.tenant_id, tenant_id);
        assert_eq!(claims.additional_headers, headers);
        assert!(!claims.is_expired());
        assert!(claims.time_to_expiry().is_some());
        assert!(!claims.jti.is_empty());
    }

    #[test]
    fn test_token_expiration() {
        let claims = AccessTokenClaims {
            service_id: ServiceId::new(1),
            tenant_id: None,
            additional_headers: BTreeMap::new(),
            expires_at: 1, // Very old timestamp
            issued_at: 1,
            key_id: "ak_test".to_string(),
            jti: Uuid::new_v4().to_string(),
        };

        assert!(claims.is_expired());
        assert!(claims.time_to_expiry().is_none());
    }

    #[test]
    fn test_token_generation_and_validation() {
        let manager = PasetoTokenManager::new(Duration::from_secs(900));
        let service_id = ServiceId::new(1);
        let key_id = "ak_test123".to_string();
        let tenant_id = Some("tenant123".to_string());
        let mut headers = BTreeMap::new();
        headers.insert("X-Custom".to_string(), "value".to_string());

        // Generate token
        let token = manager
            .generate_token(
                service_id,
                key_id.clone(),
                tenant_id.clone(),
                headers.clone(),
                None,
            )
            .expect("Should generate token");

        // Token should start with v4.local.
        assert!(token.starts_with("v4.local."));

        // Validate token
        let claims = manager
            .validate_token(&token)
            .expect("Should validate token");

        assert_eq!(claims.service_id, service_id);
        assert_eq!(claims.key_id, key_id);
        assert_eq!(claims.tenant_id, tenant_id);
        assert_eq!(claims.additional_headers, headers);
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_token_validation_with_different_key() {
        let manager1 = PasetoTokenManager::new(Duration::from_secs(900));
        let manager2 = PasetoTokenManager::new(Duration::from_secs(900));

        let token = manager1
            .generate_token(
                ServiceId::new(1),
                "ak_test".to_string(),
                None,
                BTreeMap::new(),
                None,
            )
            .expect("Should generate token");

        // Should fail with different key
        let result = manager2.validate_token(&token);
        assert!(result.is_err());
        assert!(matches!(result, Err(PasetoError::DecryptionError(_))));
    }

    #[test]
    fn test_expired_token_validation() {
        let manager = PasetoTokenManager::new(Duration::from_millis(1));

        let token = manager
            .generate_token(
                ServiceId::new(1),
                "ak_test".to_string(),
                None,
                BTreeMap::new(),
                None,
            )
            .expect("Should generate token");

        // Wait for token to expire
        thread::sleep(Duration::from_millis(10));

        let result = manager.validate_token(&token);
        assert!(result.is_err());
        assert!(matches!(result, Err(PasetoError::TokenExpired)));
    }
}
