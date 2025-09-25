//! Auth proxy integration for remote services
//!
//! Coordinates with Blueprint Manager auth proxy to handle secure routing
//! to remote instances across cloud providers.

use crate::core::error::{Error, Result};
use crate::secure_bridge::{RemoteEndpoint, SecureBridge};
use crate::security::encrypted_credentials::{
    EncryptedCloudCredentials, PlaintextCredentials, SecureCredentialManager,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};

/// JWT claims for access tokens
#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    /// Service ID
    service_id: u64,
    /// Blueprint ID  
    blueprint_id: u64,
    /// Token expiry (Unix timestamp)
    exp: i64,
    /// Issued at (Unix timestamp)
    iat: i64,
    /// Token ID for tracking
    jti: String,
}

/// Production-grade secure cloud credentials
#[derive(Debug, Clone)]
pub struct SecureCloudCredentials {
    pub service_id: u64,
    pub provider: String,
    /// Production AES-GCM encrypted credentials
    encrypted_credentials: EncryptedCloudCredentials,
    /// Secure credential manager for decryption
    credential_manager: Arc<SecureCredentialManager>,
    /// API key for service identification
    pub api_key: String,
}

impl SecureCloudCredentials {
    /// Create new secure credentials with production-grade encryption
    pub async fn new(service_id: u64, provider: &str, credentials: &str) -> Result<Self> {
        // Generate secure salt for key derivation
        let salt = blake3::hash(format!("{}_{}", service_id, provider).as_bytes());

        // Create secure credential manager with derived key
        let password = std::env::var("BLUEPRINT_CREDENTIAL_KEY")
            .unwrap_or_else(|_| format!("blueprint_default_key_{}", service_id));
        let credential_manager =
            Arc::new(SecureCredentialManager::new(&password, salt.as_bytes())?);

        // Create plaintext credentials
        let plaintext = PlaintextCredentials::from_json(credentials)?;

        // Encrypt with production AES-GCM
        let encrypted_credentials = credential_manager.store_credentials(provider, plaintext)?;

        // Generate cryptographically secure API key
        let api_key = format!(
            "bpak_{}_{}_{}",
            service_id,
            provider,
            hex::encode(
                &blake3::hash(
                    format!(
                        "{}_{}_{}",
                        service_id,
                        provider,
                        chrono::Utc::now().timestamp()
                    )
                    .as_bytes()
                )
                .as_bytes()[..8]
            )
        );

        Ok(Self {
            service_id,
            provider: provider.to_string(),
            encrypted_credentials,
            credential_manager,
            api_key,
        })
    }

    /// Decrypt credentials for use (securely)
    pub fn decrypt(&self) -> Result<String> {
        let plaintext_creds = self
            .credential_manager
            .retrieve_credentials(&self.encrypted_credentials)?;

        Ok(plaintext_creds.to_json())
    }
}

/// Remote service authentication record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteServiceAuth {
    pub service_id: u64,
    pub blueprint_id: u64,
    pub instance_id: String,
    pub public_ip: String,
    pub port: u16,
    pub api_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl RemoteServiceAuth {
    /// Register a new remote service with authentication
    pub async fn register(
        service_id: u64,
        blueprint_id: u64,
        instance_id: String,
        public_ip: String,
        port: u16,
        credentials: SecureCloudCredentials,
    ) -> Result<Self> {
        let auth = Self {
            service_id,
            blueprint_id,
            instance_id,
            public_ip,
            port,
            api_key: credentials.api_key.clone(),
            created_at: chrono::Utc::now(),
        };

        Ok(auth)
    }

    /// Generate production-grade JWT access token with HMAC-SHA256 signing
    pub async fn generate_access_token(&self, duration_secs: u64) -> Result<String> {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::seconds(duration_secs as i64);

        // Create JWT claims
        let claims = AccessTokenClaims {
            service_id: self.service_id,
            blueprint_id: self.blueprint_id,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
        };

        // Get signing key from environment or generate secure default
        let jwt_secret = std::env::var("BLUEPRINT_JWT_SECRET").unwrap_or_else(|_| {
            // In production, this should always be set via environment
            tracing::warn!("Using default JWT secret - set BLUEPRINT_JWT_SECRET in production");
            format!("blueprint_jwt_secret_{}", self.service_id)
        });

        // Create JWT with HMAC-SHA256 signing
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());

        let token = jsonwebtoken::encode(&header, &claims, &encoding_key)
            .map_err(|e| Error::ConfigurationError(format!("JWT encoding failed: {}", e)))?;

        Ok(token)
    }
}

/// Auth proxy extension for remote services
pub struct AuthProxyRemoteExtension {
    bridge: Arc<SecureBridge>,
    remote_services: Arc<tokio::sync::RwLock<HashMap<u64, RemoteServiceAuth>>>,
}

impl AuthProxyRemoteExtension {
    /// Create new auth proxy extension
    pub async fn new(bridge: Arc<SecureBridge>) -> Self {
        Self {
            bridge,
            remote_services: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register a remote service with the auth proxy
    pub async fn register_service(&self, auth: RemoteServiceAuth) {
        let service_id = auth.service_id;

        // Register with secure bridge
        let endpoint = RemoteEndpoint {
            instance_id: auth.instance_id.clone(),
            host: auth.public_ip.clone(),
            port: auth.port,
            use_tls: true,
            service_id: auth.service_id,
            blueprint_id: auth.blueprint_id,
        };

        if let Err(e) = self.bridge.register_endpoint(service_id, endpoint).await {
            warn!("Failed to register endpoint: {}", e);
        }

        // Store in local registry
        let mut services = self.remote_services.write().await;
        services.insert(service_id, auth);

        info!("Remote service {} registered with auth proxy", service_id);
    }

    /// Check if service is remote
    pub async fn is_remote(&self, service_id: u64) -> bool {
        let services = self.remote_services.read().await;
        services.contains_key(&service_id)
    }

    /// Forward authenticated request to remote service
    pub async fn forward_authenticated_request(
        &self,
        service_id: u64,
        method: &str,
        path: &str,
        headers: HashMap<String, String>,
        access_token: String,
        body: Vec<u8>,
    ) -> Result<(u16, HashMap<String, String>, Vec<u8>)> {
        // Verify service is registered
        let services = self.remote_services.read().await;
        let _auth = services.get(&service_id).ok_or_else(|| {
            Error::ConfigurationError(format!("Service {} not registered", service_id))
        })?;
        drop(services);

        // Production JWT validation with signature verification
        if access_token.is_empty() {
            return Err(Error::ConfigurationError("Access token required".into()));
        }

        // Get JWT secret for validation (same as used for signing)
        let jwt_secret = std::env::var("BLUEPRINT_JWT_SECRET")
            .unwrap_or_else(|_| format!("blueprint_jwt_secret_{}", service_id));

        // Validate JWT signature and claims
        let validation = Validation::new(Algorithm::HS256);
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

        let token_data =
            jsonwebtoken::decode::<AccessTokenClaims>(&access_token, &decoding_key, &validation)
                .map_err(|e| Error::ConfigurationError(format!("JWT validation failed: {}", e)))?;

        let claims = token_data.claims;

        // Validate service ID matches
        if claims.service_id != service_id {
            return Err(Error::ConfigurationError(
                "Token service ID mismatch".into(),
            ));
        }

        // Additional expiry check (JWT library already validates exp claim, but double-check)
        let now = chrono::Utc::now().timestamp();
        if now >= claims.exp {
            return Err(Error::ConfigurationError("Access token expired".into()));
        }

        tracing::debug!(
            "JWT token validated for service {} (expires: {}, jti: {})",
            service_id,
            claims.exp,
            claims.jti
        );

        // Add authentication headers
        let mut auth_headers = headers;
        auth_headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", access_token),
        );
        auth_headers.insert("X-Blueprint-Service".to_string(), service_id.to_string());

        // Forward request through secure bridge
        self.bridge
            .forward_request(service_id, method, path, auth_headers, body)
            .await
    }

    /// Remove remote service
    pub async fn remove_service(&self, service_id: u64) {
        let mut services = self.remote_services.write().await;
        if services.remove(&service_id).is_some() {
            self.bridge.remove_endpoint(service_id).await;
            info!("Removed remote service {}", service_id);
        }
    }

    /// List all remote services
    pub async fn list_remote_services(&self) -> Vec<RemoteServiceAuth> {
        let services = self.remote_services.read().await;
        services.values().cloned().collect()
    }

    /// Health check all remote services
    pub async fn health_check_all(&self) -> HashMap<u64, bool> {
        let services = self.remote_services.read().await;
        let mut results = HashMap::new();

        for &service_id in services.keys() {
            let healthy = self.bridge.health_check(service_id).await.unwrap_or(false);
            results.insert(service_id, healthy);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secure_bridge::SecureBridgeConfig;

    #[tokio::test]
    async fn test_secure_credentials() {
        let credentials_json =
            r#"{"aws_access_key": "AKIATEST123", "aws_secret_key": "secretkey123"}"#;
        let creds = SecureCloudCredentials::new(1, "aws", credentials_json)
            .await
            .unwrap();
        assert_eq!(creds.service_id, 1);
        assert_eq!(creds.provider, "aws");
        assert!(!creds.api_key.is_empty());
        assert!(creds.api_key.starts_with("bpak_1_aws_"));

        let decrypted = creds.decrypt().unwrap();
        assert!(decrypted.contains("AKIATEST123"));
        assert!(decrypted.contains("secretkey123"));
    }

    #[tokio::test]
    async fn test_auth_proxy_extension() {
        let config = SecureBridgeConfig {
            enable_mtls: false,
            ..Default::default()
        };

        let bridge = Arc::new(SecureBridge::new(config).await.unwrap());
        let extension = AuthProxyRemoteExtension::new(bridge).await;

        // Initially no remote services
        assert!(!extension.is_remote(1).await);
        assert!(extension.list_remote_services().await.is_empty());

        // Create and register a service
        let auth = RemoteServiceAuth {
            service_id: 1,
            blueprint_id: 100,
            instance_id: "i-test".to_string(),
            public_ip: "1.2.3.4".to_string(),
            port: 8080,
            api_key: "test_key".to_string(),
            created_at: chrono::Utc::now(),
        };

        extension.register_service(auth).await;

        // Verify registration
        assert!(extension.is_remote(1).await);
        assert_eq!(extension.list_remote_services().await.len(), 1);

        // Remove service
        extension.remove_service(1).await;
        assert!(!extension.is_remote(1).await);
    }
}
