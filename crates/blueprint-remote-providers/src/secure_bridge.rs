//! Secure bridge for Blueprint Manager <-> Remote Instance communication
//!
//! Provides secure, authenticated tunneling between the local Blueprint auth proxy
//! and remote instances across cloud providers.

use crate::core::error::{Error, Result};
use crate::deployment::tracker::DeploymentRecord;
use blueprint_std::collections::HashMap;
use blueprint_std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Configuration for secure bridge
#[derive(Debug, Clone)]
pub struct SecureBridgeConfig {
    /// Enable mTLS for production deployments
    pub enable_mtls: bool,
    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,
    /// Idle connection timeout in seconds
    pub idle_timeout_secs: u64,
    /// Maximum concurrent connections per endpoint
    pub max_connections_per_endpoint: usize,
}

impl Default for SecureBridgeConfig {
    fn default() -> Self {
        Self {
            enable_mtls: true,
            connect_timeout_secs: 30,
            idle_timeout_secs: 300,
            max_connections_per_endpoint: 10,
        }
    }
}

/// Remote endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteEndpoint {
    /// Cloud instance ID
    pub instance_id: String,
    /// Hostname or IP address
    pub host: String,
    /// Port for blueprint service
    pub port: u16,
    /// Use TLS for connection
    pub use_tls: bool,
    /// Service ID for routing
    pub service_id: u64,
    /// Blueprint ID for identification
    pub blueprint_id: u64,
}

/// Secure bridge for remote communication
pub struct SecureBridge {
    config: SecureBridgeConfig,
    endpoints: Arc<RwLock<HashMap<u64, RemoteEndpoint>>>,
    client: reqwest::Client,
}

impl SecureBridge {
    /// Create new secure bridge
    pub async fn new(config: SecureBridgeConfig) -> Result<Self> {
        let mut client_builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.connect_timeout_secs))
            .tcp_keepalive(std::time::Duration::from_secs(60));

        // Configure TLS settings
        if config.enable_mtls {
            // Production mTLS certificate configuration
            info!("mTLS enabled for secure bridge");
            
            // Load client certificate and private key for mTLS
            let cert_path = blueprint_std::env::var("BLUEPRINT_CLIENT_CERT_PATH")
                .unwrap_or_else(|_| "/etc/blueprint/certs/client.crt".to_string());
            let key_path = blueprint_std::env::var("BLUEPRINT_CLIENT_KEY_PATH")
                .unwrap_or_else(|_| "/etc/blueprint/certs/client.key".to_string());
            let ca_path = blueprint_std::env::var("BLUEPRINT_CA_CERT_PATH")
                .unwrap_or_else(|_| "/etc/blueprint/certs/ca.crt".to_string());
            
            // In production, these certificate files should exist
            // For now, we configure the client to expect them but gracefully degrade
            if std::path::Path::new(&cert_path).exists() && 
               std::path::Path::new(&key_path).exists() &&
               std::path::Path::new(&ca_path).exists() {
                
                // Read certificate files
                let client_cert = std::fs::read(&cert_path)
                    .map_err(|e| Error::ConfigurationError(format!("Failed to read client cert: {}", e)))?;
                let client_key = std::fs::read(&key_path)
                    .map_err(|e| Error::ConfigurationError(format!("Failed to read client key: {}", e)))?;
                let ca_cert = std::fs::read(&ca_path)
                    .map_err(|e| Error::ConfigurationError(format!("Failed to read CA cert: {}", e)))?;
                
                // Create identity and CA certificate
                let identity = reqwest::Identity::from_pkcs8_pem(&client_cert, &client_key)
                    .map_err(|e| Error::ConfigurationError(format!("Failed to create identity: {}", e)))?;
                let ca_cert = reqwest::Certificate::from_pem(&ca_cert)
                    .map_err(|e| Error::ConfigurationError(format!("Failed to parse CA cert: {}", e)))?;
                
                client_builder = client_builder
                    .identity(identity)
                    .add_root_certificate(ca_cert)
                    .use_rustls_tls();
                    
                info!("mTLS certificates loaded successfully");
            } else {
                warn!("mTLS certificates not found at expected paths, using system certs");
                client_builder = client_builder.use_rustls_tls();
            }
        } else {
            client_builder = client_builder.danger_accept_invalid_certs(true);
            warn!("mTLS disabled - only for testing");
        }

        let client = client_builder.build()
            .map_err(|e| Error::ConfigurationError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            endpoints: Arc::new(RwLock::new(HashMap::new())),
            client,
        })
    }

    /// Register a remote endpoint
    pub async fn register_endpoint(&self, service_id: u64, endpoint: RemoteEndpoint) {
        if let Ok(mut endpoints) = self.endpoints.write() {
            endpoints.insert(service_id, endpoint.clone());
            info!("Registered remote endpoint for service {}: {}:{}", 
                  service_id, endpoint.host, endpoint.port);
        }
    }

    /// Remove an endpoint
    pub async fn remove_endpoint(&self, service_id: u64) {
        if let Ok(mut endpoints) = self.endpoints.write() {
            if endpoints.remove(&service_id).is_some() {
                info!("Removed remote endpoint for service {}", service_id);
            }
        }
    }

    /// Health check for remote endpoint
    pub async fn health_check(&self, service_id: u64) -> Result<bool> {
        let endpoints = self.endpoints.read().map_err(|_| Error::ConfigurationError("Lock poisoned".to_string()))?;
        let endpoint = endpoints.get(&service_id)
            .ok_or_else(|| Error::ConfigurationError(format!("No endpoint for service {}", service_id)))?;

        let url = format!("{}://{}:{}/health", 
                         if endpoint.use_tls { "https" } else { "http" },
                         endpoint.host,
                         endpoint.port);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                warn!("Health check failed for service {}: {}", service_id, e);
                Ok(false)
            }
        }
    }

    /// Forward authenticated request to remote endpoint
    pub async fn forward_request(
        &self,
        service_id: u64,
        method: &str,
        path: &str,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Result<(u16, HashMap<String, String>, Vec<u8>)> {
        let endpoints = self.endpoints.read().map_err(|_| Error::ConfigurationError("Lock poisoned".to_string()))?;
        let endpoint = endpoints.get(&service_id)
            .ok_or_else(|| Error::ConfigurationError(format!("No endpoint for service {}", service_id)))?;

        let url = format!("{}://{}:{}{}", 
                         if endpoint.use_tls { "https" } else { "http" },
                         endpoint.host,
                         endpoint.port,
                         path);

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            "PATCH" => self.client.patch(&url),
            _ => return Err(Error::ConfigurationError(format!("Unsupported method: {}", method))),
        };

        // Add headers
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        // Add body if provided
        if !body.is_empty() {
            request = request.body(body);
        }

        // Send request
        let response = request.send().await
            .map_err(|e| Error::ConfigurationError(format!("Request failed: {}", e)))?;

        // Extract response
        let status = response.status().as_u16();
        let response_headers: HashMap<String, String> = response.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let response_body = response.bytes().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read response: {}", e)))?
            .to_vec();

        Ok((status, response_headers, response_body))
    }

    /// Update bridge from deployment record
    pub async fn update_from_deployment(&self, record: &DeploymentRecord) {
        if let Some(instance_id) = record.resource_ids.get("instance_id") {
            if let Some(public_ip) = record.resource_ids.get("public_ip") {
                let service_id = record.blueprint_id.parse::<u64>().unwrap_or(0);
                
                let endpoint = RemoteEndpoint {
                    instance_id: instance_id.clone(),
                    host: public_ip.clone(),
                    port: 8080, // Default blueprint service port
                    use_tls: true,
                    service_id,
                    blueprint_id: service_id,
                };

                self.register_endpoint(service_id, endpoint).await;
            }
        }
    }

    /// Get endpoint information for service
    pub async fn get_endpoint(&self, service_id: u64) -> Option<RemoteEndpoint> {
        let endpoints = self.endpoints.read().ok()?;
        endpoints.get(&service_id).cloned()
    }

    /// List all registered endpoints
    pub async fn list_endpoints(&self) -> Vec<(u64, RemoteEndpoint)> {
        match self.endpoints.read() {
            Ok(endpoints) => endpoints.iter().map(|(id, ep)| (*id, ep.clone())).collect(),
            Err(_) => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bridge_creation() {
        let config = SecureBridgeConfig {
            enable_mtls: false,
            ..Default::default()
        };

        let bridge = SecureBridge::new(config).await.unwrap();
        assert!(bridge.list_endpoints().await.is_empty());
    }

    #[tokio::test]
    async fn test_endpoint_management() {
        let config = SecureBridgeConfig {
            enable_mtls: false,
            ..Default::default()
        };

        let bridge = SecureBridge::new(config).await.unwrap();

        let endpoint = RemoteEndpoint {
            instance_id: "i-test123".to_string(),
            host: "test.example.com".to_string(),
            port: 8080,
            use_tls: true,
            service_id: 1,
            blueprint_id: 100,
        };

        // Register endpoint
        bridge.register_endpoint(1, endpoint.clone()).await;
        assert_eq!(bridge.list_endpoints().await.len(), 1);

        // Get endpoint
        let retrieved = bridge.get_endpoint(1).await.unwrap();
        assert_eq!(retrieved.instance_id, "i-test123");

        // Remove endpoint
        bridge.remove_endpoint(1).await;
        assert!(bridge.list_endpoints().await.is_empty());
    }
}