//! Auth proxy integration for remote services
//!
//! Coordinates with Blueprint Manager auth proxy to handle secure routing
//! to remote instances across cloud providers.

use crate::core::error::{Error, Result};
use crate::secure_bridge::{SecureBridge, RemoteEndpoint};
use blueprint_std::collections::HashMap;
use blueprint_std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Secure cloud credentials with encryption
#[derive(Debug, Clone)]
pub struct SecureCloudCredentials {
    pub service_id: u64,
    pub provider: String,
    pub encrypted_credentials: Vec<u8>,
    pub api_key: String,
}

impl SecureCloudCredentials {
    /// Create new secure credentials with encryption
    pub async fn new(service_id: u64, provider: &str, credentials: &str) -> Result<Self> {
        // Simple encryption for demo - in production use proper crypto
        let encrypted = credentials.as_bytes().iter()
            .map(|b| b.wrapping_add(42))
            .collect();

        // Generate API key for external access
        let api_key = format!("bpak_{}_{}_{}", 
                             service_id, 
                             provider,
                             uuid::Uuid::new_v4().to_string()[..8].to_string());

        Ok(Self {
            service_id,
            provider: provider.to_string(),
            encrypted_credentials: encrypted,
            api_key,
        })
    }

    /// Decrypt credentials for use
    pub fn decrypt(&self) -> Result<String> {
        let decrypted: Vec<u8> = self.encrypted_credentials.iter()
            .map(|b| b.wrapping_sub(42))
            .collect();

        String::from_utf8(decrypted)
            .map_err(|e| Error::ConfigurationError(format!("Decryption failed: {}", e)))
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

    /// Generate access token for external requests
    pub async fn generate_access_token(&self, duration_secs: u64) -> Result<String> {
        // Simple token generation - in production use JWT with proper signing
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(duration_secs as i64);
        let token = format!("bpat_{}_{}_{}_{}", 
                           self.service_id,
                           self.blueprint_id,
                           expires_at.timestamp(),
                           uuid::Uuid::new_v4().to_string()[..12].to_string());

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

        self.bridge.register_endpoint(service_id, endpoint).await;

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
        let _auth = services.get(&service_id)
            .ok_or_else(|| Error::ConfigurationError(format!("Service {} not registered", service_id)))?;
        drop(services);

        // Validate access token format and expiry
        if access_token.is_empty() {
            return Err(Error::ConfigurationError("Access token required".into()));
        }
        
        // Validate Blueprint access token format
        if !access_token.starts_with("bpat_") {
            return Err(Error::ConfigurationError("Invalid token format".into()));
        }
        
        // Parse token components: bpat_{service_id}_{blueprint_id}_{timestamp}_{uuid}
        let token_parts: Vec<&str> = access_token.split('_').collect();
        if token_parts.len() != 5 {
            return Err(Error::ConfigurationError("Malformed access token".into()));
        }
        
        // Validate service ID matches
        if let Ok(token_service_id) = token_parts[1].parse::<u64>() {
            if token_service_id != service_id {
                return Err(Error::ConfigurationError("Token service mismatch".into()));
            }
        } else {
            return Err(Error::ConfigurationError("Invalid token service ID".into()));
        }
        
        // Check token expiry
        if let Ok(timestamp) = token_parts[3].parse::<i64>() {
            let expires_at = chrono::DateTime::from_timestamp(timestamp, 0)
                .ok_or_else(|| Error::ConfigurationError("Invalid token timestamp".into()))?;
            
            if chrono::Utc::now() > expires_at {
                return Err(Error::ConfigurationError("Access token expired".into()));
            }
        } else {
            return Err(Error::ConfigurationError("Invalid token timestamp".into()));
        }

        // Add authentication headers
        let mut auth_headers = headers;
        auth_headers.insert("Authorization".to_string(), format!("Bearer {}", access_token));
        auth_headers.insert("X-Blueprint-Service".to_string(), service_id.to_string());

        // Forward request through secure bridge
        self.bridge.forward_request(service_id, method, path, auth_headers, body).await
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
        let creds = SecureCloudCredentials::new(1, "aws", "secret_data").await.unwrap();
        assert_eq!(creds.service_id, 1);
        assert_eq!(creds.provider, "aws");
        assert!(!creds.api_key.is_empty());
        
        let decrypted = creds.decrypt().unwrap();
        assert_eq!(decrypted, "secret_data");
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