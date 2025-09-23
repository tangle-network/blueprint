//! Secure bridge for Blueprint Manager <-> Remote Instance communication
//!
//! Provides secure, authenticated tunneling between the local Blueprint auth proxy
//! and remote instances across cloud providers.

use crate::core::error::{Error, Result};
use crate::deployment::tracker::DeploymentRecord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
    /// Validate certificate format and basic security properties
    fn validate_certificate_format(cert_data: &[u8], cert_type: &str) -> Result<()> {
        let cert_str = String::from_utf8(cert_data.to_vec())
            .map_err(|_| Error::ConfigurationError(format!("{} must be valid UTF-8", cert_type)))?;

        // Basic PEM format validation
        if !cert_str.contains("-----BEGIN") || !cert_str.contains("-----END") {
            return Err(Error::ConfigurationError(format!(
                "{} must be in PEM format",
                cert_type
            )));
        }

        // Validate certificate is not obviously invalid
        if cert_data.len() < 100 {
            return Err(Error::ConfigurationError(format!(
                "{} appears to be too short to be valid",
                cert_type
            )));
        }

        // Check for common certificate types
        let valid_headers = [
            "-----BEGIN CERTIFICATE-----",
            "-----BEGIN PRIVATE KEY-----",
            "-----BEGIN RSA PRIVATE KEY-----",
            "-----BEGIN EC PRIVATE KEY-----",
        ];

        if !valid_headers.iter().any(|header| cert_str.contains(header)) {
            return Err(Error::ConfigurationError(format!(
                "{} does not contain recognized PEM headers",
                cert_type
            )));
        }

        // TODO: Add certificate expiry validation in production
        // TODO: Add certificate chain validation

        Ok(())
    }

    /// Create new secure bridge
    pub async fn new(config: SecureBridgeConfig) -> Result<Self> {
        let mut client_builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.connect_timeout_secs))
            .tcp_keepalive(std::time::Duration::from_secs(60));

        // Configure TLS settings with production-grade certificate validation
        if config.enable_mtls {
            // Production mTLS certificate configuration
            info!("mTLS enabled for secure bridge - strict certificate validation");

            // Load client certificate and private key for mTLS
            let cert_path = std::env::var("BLUEPRINT_CLIENT_CERT_PATH")
                .unwrap_or_else(|_| "/etc/blueprint/certs/client.crt".to_string());
            let key_path = std::env::var("BLUEPRINT_CLIENT_KEY_PATH")
                .unwrap_or_else(|_| "/etc/blueprint/certs/client.key".to_string());
            let ca_path = std::env::var("BLUEPRINT_CA_CERT_PATH")
                .unwrap_or_else(|_| "/etc/blueprint/certs/ca.crt".to_string());

            // PRODUCTION SECURITY: Enforce certificate presence in production
            let is_production = std::env::var("BLUEPRINT_ENV")
                .unwrap_or_else(|_| "development".to_string())
                == "production";

            if is_production
                && (!std::path::Path::new(&cert_path).exists()
                    || !std::path::Path::new(&key_path).exists()
                    || !std::path::Path::new(&ca_path).exists())
            {
                return Err(Error::ConfigurationError(
                    "Production deployment requires mTLS certificates at configured paths".into(),
                ));
            }

            if std::path::Path::new(&cert_path).exists()
                && std::path::Path::new(&key_path).exists()
                && std::path::Path::new(&ca_path).exists()
            {
                // Read certificate files
                let client_cert = std::fs::read(&cert_path).map_err(|e| {
                    Error::ConfigurationError(format!("Failed to read client cert: {}", e))
                })?;
                let client_key = std::fs::read(&key_path).map_err(|e| {
                    Error::ConfigurationError(format!("Failed to read client key: {}", e))
                })?;
                let ca_cert = std::fs::read(&ca_path).map_err(|e| {
                    Error::ConfigurationError(format!("Failed to read CA cert: {}", e))
                })?;

                // Validate certificate formats before use
                Self::validate_certificate_format(&client_cert, "client certificate")?;
                Self::validate_certificate_format(&client_key, "client private key")?;
                Self::validate_certificate_format(&ca_cert, "CA certificate")?;

                // Create identity by combining cert and key into single PEM buffer
                let mut combined_pem = Vec::new();
                combined_pem.extend_from_slice(&client_cert);
                combined_pem.extend_from_slice(b"\n");
                combined_pem.extend_from_slice(&client_key);

                let identity = reqwest::Identity::from_pem(&combined_pem).map_err(|e| {
                    Error::ConfigurationError(format!("Failed to create identity: {}", e))
                })?;
                let ca_cert = reqwest::Certificate::from_pem(&ca_cert).map_err(|e| {
                    Error::ConfigurationError(format!("Failed to parse CA cert: {}", e))
                })?;

                client_builder = client_builder
                    .identity(identity)
                    .add_root_certificate(ca_cert)
                    .use_rustls_tls()
                    .tls_built_in_root_certs(false); // Only trust our CA

                info!("mTLS certificates loaded and validated successfully");
            } else if is_production {
                return Err(Error::ConfigurationError(
                    "mTLS certificates required for production deployment".into(),
                ));
            } else {
                warn!("mTLS certificates not found - using system certs for development");
                client_builder = client_builder.use_rustls_tls();
            }
        } else {
            let is_production = std::env::var("BLUEPRINT_ENV")
                .unwrap_or_else(|_| "development".to_string())
                == "production";

            if is_production {
                return Err(Error::ConfigurationError(
                    "mTLS cannot be disabled in production environment".into(),
                ));
            }

            client_builder = client_builder.danger_accept_invalid_certs(true);
            warn!("mTLS disabled - DEVELOPMENT ONLY");
        }

        let client = client_builder.build().map_err(|e| {
            Error::ConfigurationError(format!("Failed to create HTTP client: {}", e))
        })?;

        Ok(Self {
            config,
            endpoints: Arc::new(RwLock::new(HashMap::new())),
            client,
        })
    }

    /// Validate endpoint for security - prevent SSRF attacks
    fn validate_endpoint_security(endpoint: &RemoteEndpoint) -> Result<()> {
        // SECURITY: Only allow localhost and private IP ranges for remote instances
        let host = &endpoint.host;

        // Parse IP address
        if let Ok(ip) = host.parse::<std::net::IpAddr>() {
            match ip {
                std::net::IpAddr::V4(ipv4) => {
                    if !ipv4.is_loopback() && !ipv4.is_private() {
                        return Err(Error::ConfigurationError(
                            "Remote endpoints must use localhost or private IP ranges only".into(),
                        ));
                    }
                }
                std::net::IpAddr::V6(ipv6) => {
                    if !ipv6.is_loopback() {
                        return Err(Error::ConfigurationError(
                            "Remote endpoints must use localhost for IPv6".into(),
                        ));
                    }
                }
            }
        } else {
            // If it's a hostname, only allow localhost variants
            if !host.starts_with("localhost") && host != "127.0.0.1" && host != "::1" {
                return Err(Error::ConfigurationError(
                    "Remote endpoints must use localhost hostname only".into(),
                ));
            }
        }

        // Validate port range (u16 max is 65535, so only check minimum)
        if endpoint.port < 1024 {
            return Err(Error::ConfigurationError(
                "Port must be >= 1024".into(),
            ));
        }

        Ok(())
    }

    /// Register a remote endpoint with security validation
    pub async fn register_endpoint(&self, service_id: u64, endpoint: RemoteEndpoint) -> Result<()> {
        // SECURITY: Validate endpoint before registration
        Self::validate_endpoint_security(&endpoint)?;

        if let Ok(mut endpoints) = self.endpoints.write() {
            endpoints.insert(service_id, endpoint.clone());
            info!(
                "Registered secure remote endpoint for service {}: {}:{}",
                service_id, endpoint.host, endpoint.port
            );
            Ok(())
        } else {
            Err(Error::ConfigurationError(
                "Failed to acquire endpoint lock".into(),
            ))
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
        let endpoints = self
            .endpoints
            .read()
            .map_err(|_| Error::ConfigurationError("Lock poisoned".to_string()))?;
        let endpoint = endpoints.get(&service_id).ok_or_else(|| {
            Error::ConfigurationError(format!("No endpoint for service {}", service_id))
        })?;

        let url = format!(
            "{}://{}:{}/health",
            if endpoint.use_tls { "https" } else { "http" },
            endpoint.host,
            endpoint.port
        );

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => {
                // SECURITY: Don't log detailed error information to prevent information disclosure
                warn!("Health check failed for service {}", service_id);
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
        let endpoints = self
            .endpoints
            .read()
            .map_err(|_| Error::ConfigurationError("Lock poisoned".to_string()))?;
        let endpoint = endpoints.get(&service_id).ok_or_else(|| {
            Error::ConfigurationError(format!("No endpoint for service {}", service_id))
        })?;

        let url = format!(
            "{}://{}:{}{}",
            if endpoint.use_tls { "https" } else { "http" },
            endpoint.host,
            endpoint.port,
            path
        );

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            "PATCH" => self.client.patch(&url),
            _ => {
                return Err(Error::ConfigurationError(format!(
                    "Unsupported method: {}",
                    method
                )));
            }
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
        let response = request
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Request failed: {}", e)))?;

        // Extract response
        let status = response.status().as_u16();
        let response_headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let response_body = response
            .bytes()
            .await
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

                let _ = self.register_endpoint(service_id, endpoint).await;
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
        bridge.register_endpoint(1, endpoint.clone()).await.unwrap();
        assert_eq!(bridge.list_endpoints().await.len(), 1);

        // Get endpoint
        let retrieved = bridge.get_endpoint(1).await.unwrap();
        assert_eq!(retrieved.instance_id, "i-test123");

        // Remove endpoint
        bridge.remove_endpoint(1).await;
        assert!(bridge.list_endpoints().await.is_empty());
    }
}
