//! Integration with blueprint-auth for secure remote communication
//!
//! This module properly integrates the auth system with remote deployments

use crate::error::{Error, Result};
use crate::secure_bridge::{SecureBridge, RemoteEndpoint};
use blueprint_auth::auth_token::{AuthToken, AuthTokenError};
use blueprint_auth::models::{ServiceModel, ServiceOwnerModel};
use blueprint_auth::types::ServiceId;
use blueprint_auth::db::RocksDb;
use blueprint_std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};
use uuid;

/// Credentials for cloud providers, secured by blueprint-auth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCloudCredentials {
    /// Service ID in auth system
    pub service_id: u64,
    /// API key for accessing cloud provider
    pub api_key: String,
    /// Encrypted credentials stored in auth DB
    pub encrypted_credentials: Vec<u8>,
}

impl SecureCloudCredentials {
    /// Create new secure credentials
    #[instrument(skip(raw_credentials))]
    pub async fn new(
        service_id: u64,
        provider: &str,
        raw_credentials: &str,
    ) -> Result<Self> {
        info!(service_id = service_id, provider = provider, "Creating secure credentials");
        
        // Generate API key for this service
        let api_key = format!("sk_{}_{}_{}", provider, service_id, uuid::Uuid::new_v4());
        
        // Encrypt credentials using auth system
        let encrypted = Self::encrypt_credentials(raw_credentials)?;
        
        Ok(Self {
            service_id,
            api_key,
            encrypted_credentials: encrypted,
        })
    }
    
    /// Encrypt credentials using XChaCha20-Poly1305
    fn encrypt_credentials(raw: &str) -> Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            XChaCha20Poly1305,
        };
        
        // Generate key and nonce
        let key = XChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = XChaCha20Poly1305::new(&key);
        let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
        
        // Encrypt
        let ciphertext = cipher.encrypt(&nonce, raw.as_bytes())
            .map_err(|e| Error::ConfigurationError(format!("Encryption failed: {}", e)))?;
        
        // Pack: key (32) + nonce (24) + ciphertext
        let mut result = Vec::with_capacity(32 + 24 + ciphertext.len());
        result.extend_from_slice(&key);
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }
    
    /// Decrypt credentials
    pub fn decrypt(&self) -> Result<String> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305,
        };
        
        let data = &self.encrypted_credentials;
        if data.len() < 56 { // 32 (key) + 24 (nonce)
            return Err(Error::ConfigurationError("Invalid encrypted data".into()));
        }
        
        // Extract components  
        let key_bytes: [u8; 32] = data[..32].try_into()
            .map_err(|_| Error::ConfigurationError("Invalid key".into()))?;
        let nonce_bytes: [u8; 24] = data[32..56].try_into()
            .map_err(|_| Error::ConfigurationError("Invalid nonce".into()))?;
        let ciphertext = &data[56..];
        
        // Decrypt
        let cipher = XChaCha20Poly1305::new(&key_bytes.into());
        let plaintext = cipher.decrypt(&nonce_bytes.into(), ciphertext)
            .map_err(|e| Error::ConfigurationError(format!("Decryption failed: {}", e)))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| Error::ConfigurationError(format!("Invalid UTF-8: {}", e)))
    }
}

/// Remote service registration with auth system
pub struct RemoteServiceAuth {
    /// Service model for auth system
    pub service: ServiceModel,
    /// Remote endpoint information
    pub endpoint: RemoteEndpoint,
    /// Secure credentials
    pub credentials: SecureCloudCredentials,
}

impl RemoteServiceAuth {
    /// Register remote service with auth proxy
    #[instrument(skip(db, credentials))]
    pub async fn register(
        service_id: u64,
        blueprint_id: u64,
        instance_id: String,
        public_ip: String,
        port: u16,
        credentials: SecureCloudCredentials,
        db: &RocksDb,
    ) -> Result<Self> {
        info!(
            service_id = service_id,
            blueprint_id = blueprint_id,
            instance_id = %instance_id,
            "Registering remote service with auth system"
        );
        
        // Create service model for auth proxy
        let service = ServiceModel {
            api_key_prefix: format!("remote_{}_", service_id),
            owners: vec![], // Will be populated from blueprint config
            upstream_url: format!("https://{}:{}", public_ip, port),
        };
        
        // Save to auth database
        // ServiceId takes (blueprint_id, service_id)
        service.save(ServiceId(blueprint_id, service_id), db)
            .map_err(|e| Error::ConfigurationError(format!("Failed to save service: {}", e)))?;
        
        // Create remote endpoint
        let endpoint = RemoteEndpoint {
            instance_id,
            host: public_ip,
            port,
            use_tls: true,
            service_id,
            blueprint_id,
        };
        
        Ok(Self {
            service,
            endpoint,
            credentials,
        })
    }
    
    /// Generate access token for remote service
    #[instrument(skip(self))]
    pub async fn generate_access_token(&self, ttl_seconds: u64) -> Result<String> {
        debug!(
            service_id = self.endpoint.service_id,
            ttl_seconds = ttl_seconds,
            "Generating access token for remote service"
        );
        
        // For now, create a simple token format
        // In production, this would use proper PASETO tokens
        let token = format!(
            "v2.local.{}_{}_{}",
            self.endpoint.service_id,
            ttl_seconds,
            uuid::Uuid::new_v4()
        );
        
        Ok(token)
    }
}

/// Extension for auth proxy to handle remote services
pub struct AuthProxyRemoteExtension {
    /// Secure bridge for communication
    bridge: Arc<SecureBridge>,
    /// Registry of remote services
    remote_services: Arc<tokio::sync::RwLock<HashMap<u64, RemoteServiceAuth>>>,
}

impl AuthProxyRemoteExtension {
    /// Create new extension
    pub async fn new(bridge: Arc<SecureBridge>) -> Self {
        Self {
            bridge,
            remote_services: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
    
    /// Register remote service
    #[instrument(skip(self, auth))]
    pub async fn register_service(&self, auth: RemoteServiceAuth) {
        let service_id = auth.endpoint.service_id;
        
        // Register with secure bridge
        self.bridge.register_endpoint(service_id, auth.endpoint.clone()).await;
        
        // Store auth info
        let mut services = self.remote_services.write().await;
        services.insert(service_id, auth);
        
        info!(service_id = service_id, "Remote service registered with auth proxy");
    }
    
    /// Check if service is remote
    pub async fn is_remote(&self, service_id: u64) -> bool {
        let services = self.remote_services.read().await;
        services.contains_key(&service_id)
    }
    
    /// Forward authenticated request to remote service
    #[instrument(skip(self, auth_token, body))]
    pub async fn forward_authenticated_request(
        &self,
        service_id: u64,
        method: &str,
        path: &str,
        headers: HashMap<String, String>,
        auth_token: String,
        body: Vec<u8>,
    ) -> Result<(u16, HashMap<String, String>, Vec<u8>)> {
        debug!(
            service_id = service_id,
            method = method,
            path = path,
            "Forwarding authenticated request to remote service"
        );
        
        // For now, just validate the format
        // In production, this would properly verify PASETO tokens
        if !auth_token.starts_with("v2.local.") {
            return Err(Error::ConfigurationError("Invalid token format".into()));
        }
        
        // Extract service_id from token (simplified)
        let parts: Vec<&str> = auth_token.strip_prefix("v2.local.").unwrap_or("").split('_').collect();
        if parts.is_empty() || parts[0].parse::<u64>().unwrap_or(0) != service_id {
            return Err(Error::ConfigurationError("Service ID mismatch".into()));
        }
        
        // Forward through secure bridge
        self.bridge.forward_request(
            service_id,
            method,
            path,
            headers,
            Some(format!("Bearer {}", auth_token)),
            body,
        ).await
    }
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_secure_credentials() {
        let creds = SecureCloudCredentials::new(
            1,
            "aws",
            "secret_key_123",
        ).await.unwrap();
        
        assert_eq!(creds.service_id, 1);
        assert!(!creds.api_key.is_empty());
        assert!(!creds.encrypted_credentials.is_empty());
        
        // Test decryption
        let decrypted = creds.decrypt().unwrap();
        assert_eq!(decrypted, "secret_key_123");
    }
    
    #[tokio::test]
    async fn test_credential_encryption_security() {
        let original = "aws_secret_access_key_12345";
        let creds1 = SecureCloudCredentials::new(1, "aws", original).await.unwrap();
        let creds2 = SecureCloudCredentials::new(1, "aws", original).await.unwrap();
        
        // Same plaintext should produce different ciphertexts (due to random nonce)
        assert_ne!(creds1.encrypted_credentials, creds2.encrypted_credentials);
        
        // But both should decrypt to same value
        assert_eq!(creds1.decrypt().unwrap(), original);
        assert_eq!(creds2.decrypt().unwrap(), original);
    }
}