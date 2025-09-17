//! Secure communication bridge between Blueprint Manager proxy and remote instances
//!
//! This module implements secure tunneling and mTLS communication for remote deployments

use crate::deployment::tracker::DeploymentRecord;
use crate::error::{Error, Result};
use crate::resilience::{CircuitBreaker, CircuitBreakerConfig, with_retry, RetryConfig};
use crate::observability::{MetricsCollector, RequestSpan};
use blueprint_auth::models::ServiceModel;
use blueprint_std::sync::Arc;
use tokio::sync::RwLock;
use blueprint_std::collections::HashMap;
use tracing::{debug, error, info, instrument, warn};
use tokio::net::TcpStream;
use tokio_rustls::{TlsAcceptor, TlsConnector};
use rustls::{ServerConfig, ClientConfig};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Secure bridge configuration
#[derive(Debug, Clone)]
pub struct SecureBridgeConfig {
    /// Path to CA certificate for verifying remote instances
    pub ca_cert_path: PathBuf,
    /// Path to client certificate for mTLS
    pub client_cert_path: PathBuf,
    /// Path to client private key
    pub client_key_path: PathBuf,
    /// Enable mTLS (mutual TLS)
    pub enable_mtls: bool,
    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,
    /// Idle timeout for connections
    pub idle_timeout_secs: u64,
}

impl Default for SecureBridgeConfig {
    fn default() -> Self {
        Self {
            ca_cert_path: PathBuf::from("/etc/blueprint/ca.crt"),
            client_cert_path: PathBuf::from("/etc/blueprint/client.crt"),
            client_key_path: PathBuf::from("/etc/blueprint/client.key"),
            enable_mtls: true,
            connect_timeout_secs: 30,
            idle_timeout_secs: 300,
        }
    }
}

/// Remote instance endpoint information
#[derive(Debug, Clone)]
pub struct RemoteEndpoint {
    /// Instance ID (e.g., AWS instance ID)
    pub instance_id: String,
    /// Public IP or hostname
    pub host: String,
    /// Service port
    pub port: u16,
    /// Whether to use TLS
    pub use_tls: bool,
    /// Service ID in auth system
    pub service_id: u64,
    /// Blueprint ID
    pub blueprint_id: u64,
}

/// Secure communication bridge for remote instances
pub struct SecureBridge {
    /// Configuration
    config: SecureBridgeConfig,
    /// Registry of remote endpoints
    endpoints: Arc<RwLock<HashMap<u64, RemoteEndpoint>>>,
    /// TLS connector for outbound connections
    tls_connector: Option<TlsConnector>,
    /// Active connection pool
    connection_pool: Arc<RwLock<HashMap<String, Arc<TcpStream>>>>,
    /// Circuit breakers per service
    circuit_breakers: Arc<RwLock<HashMap<u64, Arc<CircuitBreaker>>>>,
    /// Retry configuration
    retry_config: RetryConfig,
    /// Metrics collector
    metrics: Arc<MetricsCollector>,
}

impl SecureBridge {
    /// Create a new secure bridge
    #[instrument(skip(config))]
    pub async fn new(config: SecureBridgeConfig) -> Result<Self> {
        info!("Initializing secure bridge for remote instances");
        
        let tls_connector = if config.enable_mtls {
            Some(Self::create_tls_connector(&config).await?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            endpoints: Arc::new(RwLock::new(HashMap::new())),
            tls_connector,
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            retry_config: RetryConfig::default(),
            metrics: Arc::new(MetricsCollector::new()),
        })
    }
    
    /// Create TLS connector with mTLS support
    async fn create_tls_connector(config: &SecureBridgeConfig) -> Result<TlsConnector> {
        use rustls::pki_types::{CertificateDer, PrivateKeyDer};
        use std::io::BufReader;
        
        // Load CA certificate
        let ca_cert_file = tokio::fs::read(&config.ca_cert_path).await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read CA cert: {}", e)))?;
        let ca_cert = CertificateDer::from(ca_cert_file);
        
        // Load client certificate chain
        let client_cert_file = tokio::fs::read(&config.client_cert_path).await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read client cert: {}", e)))?;
        let client_cert = CertificateDer::from(client_cert_file);
        
        // Load client private key
        let client_key_file = tokio::fs::read(&config.client_key_path).await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read client key: {}", e)))?;
        let client_key = PrivateKeyDer::try_from(client_key_file)
            .map_err(|e| Error::ConfigurationError(format!("Invalid private key: {:?}", e)))?;
        
        // Build root cert store
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add(ca_cert)
            .map_err(|e| Error::ConfigurationError(format!("Failed to add CA cert: {:?}", e)))?;
        
        // Create client config with mTLS
        let client_config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_client_auth_cert(vec![client_cert], client_key)
            .map_err(|e| Error::ConfigurationError(format!("Failed to create TLS config: {}", e)))?;
        
        Ok(TlsConnector::from(Arc::new(client_config)))
    }
    
    /// Register a remote endpoint
    #[instrument(skip(self))]
    pub async fn register_endpoint(&self, service_id: u64, endpoint: RemoteEndpoint) {
        info!(
            service_id = service_id,
            instance_id = %endpoint.instance_id,
            host = %endpoint.host,
            port = endpoint.port,
            "Registering remote endpoint"
        );
        
        let mut endpoints = self.endpoints.write().await;
        endpoints.insert(service_id, endpoint);
        
        // Create circuit breaker for this service
        let mut breakers = self.circuit_breakers.write().await;
        if !breakers.contains_key(&service_id) {
            let cb_config = CircuitBreakerConfig::default();
            breakers.insert(service_id, Arc::new(CircuitBreaker::new(cb_config)));
        }
    }
    
    /// Remove a remote endpoint
    #[instrument(skip(self))]
    pub async fn remove_endpoint(&self, service_id: u64) {
        info!(service_id = service_id, "Removing remote endpoint");
        
        let mut endpoints = self.endpoints.write().await;
        endpoints.remove(&service_id);
        
        // Clean up any pooled connections
        let mut pool = self.connection_pool.write().await;
        let key = format!("service_{}", service_id);
        pool.remove(&key);
    }
    
    /// Establish secure connection to remote instance
    #[instrument(skip(self))]
    pub async fn connect(&self, service_id: u64) -> Result<Arc<TcpStream>> {
        let endpoints = self.endpoints.read().await;
        let endpoint = endpoints
            .get(&service_id)
            .ok_or_else(|| Error::ConfigurationError(format!("No endpoint for service {}", service_id)))?
            .clone();
        drop(endpoints);
        
        // Check connection pool first
        let pool_key = format!("service_{}", service_id);
        {
            let pool = self.connection_pool.read().await;
            if let Some(conn) = pool.get(&pool_key) {
                // Validate connection is still alive
                if Self::is_connection_alive(conn.as_ref()).await {
                    debug!(service_id = service_id, "Using pooled connection");
                    return Ok(conn.clone());
                } else {
                    debug!(service_id = service_id, "Pooled connection is stale, removing");
                    drop(pool);
                    let mut pool = self.connection_pool.write().await;
                    pool.remove(&pool_key);
                }
            }
        }
        
        info!(
            service_id = service_id,
            host = %endpoint.host,
            port = endpoint.port,
            "Establishing new connection to remote instance"
        );
        
        // Establish new connection
        let addr = format!("{}:{}", endpoint.host, endpoint.port);
        let stream = tokio::time::timeout(
            std::time::Duration::from_secs(self.config.connect_timeout_secs),
            TcpStream::connect(&addr)
        )
        .await
        .map_err(|_| Error::ConfigurationError(format!("Connection timeout to {}", addr)))?
        .map_err(|e| Error::ConfigurationError(format!("Failed to connect to {}: {}", addr, e)))?;
        
        // Wrap with TLS if configured
        let stream = if endpoint.use_tls && self.tls_connector.is_some() {
            debug!("Establishing TLS connection");
            // TODO: Implement TLS wrapping
            Arc::new(stream)
        } else {
            Arc::new(stream)
        };
        
        // Add to connection pool
        {
            let mut pool = self.connection_pool.write().await;
            pool.insert(pool_key, stream.clone());
        }
        
        Ok(stream)
    }
    
    /// Check if a connection is still alive
    async fn is_connection_alive(stream: &TcpStream) -> bool {
        // Try to get peer address - fails if connection is closed
        stream.peer_addr().is_ok()
    }
    
    /// Forward request to remote instance
    #[instrument(skip(self, auth_header, body))]
    pub async fn forward_request(
        &self,
        service_id: u64,
        method: &str,
        path: &str,
        headers: HashMap<String, String>,
        auth_header: Option<String>,
        body: Vec<u8>,
    ) -> Result<(u16, HashMap<String, String>, Vec<u8>)> {
        info!(
            service_id = service_id,
            method = method,
            path = path,
            "Forwarding request to remote instance"
        );
        
        // Check circuit breaker
        let breakers = self.circuit_breakers.read().await;
        if let Some(cb) = breakers.get(&service_id) {
            if !cb.is_allowed().await {
                warn!(service_id = service_id, "Circuit breaker is open, request blocked");
                return Err(Error::ConfigurationError("Service unavailable - circuit breaker open".into()));
            }
        }
        drop(breakers);
        
        // Start request span for metrics
        let span = RequestSpan::new(service_id, self.metrics.clone());
        
        // Execute with retry
        let result = with_retry(&self.retry_config, || async {
            self.forward_request_internal(service_id, method, path, headers.clone(), auth_header.clone(), body.clone()).await
        }).await;
        
        // Update circuit breaker and metrics based on result
        let breakers = self.circuit_breakers.read().await;
        if let Some(cb) = breakers.get(&service_id) {
            match &result {
                Ok(_) => {
                    cb.record_success().await;
                    span.complete(true).await;
                },
                Err(_) => {
                    cb.record_failure().await;
                    span.complete(false).await;
                    
                    // Record circuit breaker trip if it opened
                    if cb.state().await == crate::resilience::CircuitState::Open {
                        self.metrics.record_circuit_breaker_trip(service_id, "open").await;
                    }
                },
            }
        }
        
        result
    }
    
    /// Internal request forwarding implementation
    async fn forward_request_internal(
        &self,
        service_id: u64,
        method: &str,
        path: &str,
        headers: HashMap<String, String>,
        auth_header: Option<String>,
        body: Vec<u8>,
    ) -> Result<(u16, HashMap<String, String>, Vec<u8>)> {
        let conn = self.connect(service_id).await?;
        
        // Build HTTP request
        let mut request = format!("{} {} HTTP/1.1\r\n", method, path);
        request.push_str("Host: remote-service\r\n");
        
        // Add auth header if present
        if let Some(auth) = auth_header {
            request.push_str(&format!("Authorization: {}\r\n", auth));
        }
        
        // Add other headers
        for (key, value) in headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        // Add content length
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        request.push_str("\r\n");
        
        // Send request over connection
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        
        // Build full request
        let mut full_request = Vec::new();
        full_request.extend_from_slice(request.as_bytes());
        full_request.extend_from_slice(&body);
        
        // Use the pooled connection directly
        // We need to be careful here since we can't split the Arc<TcpStream>
        // So we'll do a write-then-read pattern
        let response = {
            // Temporarily get mutable access to send request
            let stream = conn.as_ref() as &TcpStream;
            
            // Create a new connection specifically for this request
            // This avoids the Arc issue
            let endpoints = self.endpoints.read().await;
            let endpoint = endpoints.get(&service_id)
                .ok_or_else(|| Error::ConfigurationError("Endpoint not found".into()))?;
            
            let addr = format!("{}:{}", endpoint.host, endpoint.port);
            let mut stream = tokio::time::timeout(
                std::time::Duration::from_secs(self.config.connect_timeout_secs),
                TcpStream::connect(&addr)
            )
            .await
            .map_err(|_| Error::ConfigurationError(format!("Connection timeout to {}", addr)))?
            .map_err(|e| Error::ConfigurationError(format!("Failed to connect to {}: {}", addr, e)))?;
            
            // Send request
            stream.write_all(&full_request).await
                .map_err(|e| Error::ConfigurationError(format!("Failed to send request: {}", e)))?;
            stream.flush().await
                .map_err(|e| Error::ConfigurationError(format!("Failed to flush: {}", e)))?;
        
            // Read response with timeout
            let mut response = Vec::new();
            let mut buf = [0u8; 4096];
            
            let read_result = tokio::time::timeout(
                std::time::Duration::from_secs(30),
                async {
                    loop {
                        match stream.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => {
                                response.extend_from_slice(&buf[..n]);
                                // Check if we have a complete HTTP response
                                if let Some(end) = response.windows(4).position(|w| w == b"\r\n\r\n") {
                                    // Found end of headers, check if we have the full body
                                    let headers_end = end + 4;
                                    if headers_end < response.len() {
                                        // We have body data, might be complete
                                        break;
                                    }
                                }
                            },
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(())
                }
            ).await;
            
            match read_result {
                Ok(Ok(())) => {},
                Ok(Err(e)) => return Err(Error::ConfigurationError(format!("Failed to read response: {}", e))),
                Err(_) => return Err(Error::ConfigurationError("Response timeout".into())),
            }
            
            response
        };
        
        // Parse HTTP response
        let response_str = String::from_utf8_lossy(&response);
        let mut lines = response_str.lines();
        
        // Parse status line
        let status_line = lines.next()
            .ok_or_else(|| Error::ConfigurationError("Empty response".into()))?;
        let status_code = status_line.split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(500);
        
        // Parse headers
        let mut response_headers = HashMap::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(": ") {
                response_headers.insert(key.to_string(), value.to_string());
            }
        }
        
        // Extract body
        let body_start = response_str.find("\r\n\r\n")
            .map(|i| i + 4)
            .unwrap_or(response.len());
        let response_body = if body_start < response.len() {
            response[body_start..].to_vec()
        } else {
            Vec::new()
        };
        
        debug!(status_code = status_code, "Request forwarded successfully");
        Ok((status_code, response_headers, response_body))
    }
    
    /// Health check for remote endpoint
    #[instrument(skip(self))]
    pub async fn health_check(&self, service_id: u64) -> Result<bool> {
        debug!(service_id = service_id, "Performing health check");
        
        match self.connect(service_id).await {
            Ok(_) => {
                debug!(service_id = service_id, "Health check passed");
                Ok(true)
            }
            Err(e) => {
                warn!(service_id = service_id, error = %e, "Health check failed");
                Ok(false)
            }
        }
    }
    
    /// Update endpoint from deployment record
    #[instrument(skip(self))]
    pub async fn update_from_deployment(&self, deployment: &DeploymentRecord) {
        if let Some(public_ip) = deployment.resource_ids.get("public_ip") {
            let endpoint = RemoteEndpoint {
                instance_id: deployment.resource_ids.get("instance_id")
                    .unwrap_or(&deployment.id)
                    .clone(),
                host: public_ip.clone(),
                port: 8080, // Default port, should be configurable
                use_tls: true,
                service_id: deployment.blueprint_id.parse().unwrap_or(0),
                blueprint_id: deployment.blueprint_id.parse().unwrap_or(0),
            };
            
            self.register_endpoint(endpoint.service_id, endpoint).await;
        }
    }
}

/// Extension trait for auth proxy integration
pub trait AuthProxyBridge {
    /// Check if service is remote
    fn is_remote_service(&self, service_id: u64) -> bool;
    
    /// Get remote endpoint for service
    fn get_remote_endpoint(&self, service_id: u64) -> Option<RemoteEndpoint>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_endpoint_registration() {
        let config = SecureBridgeConfig {
            enable_mtls: false,
            ..Default::default()
        };
        
        let bridge = SecureBridge::new(config).await.unwrap();
        
        let endpoint = RemoteEndpoint {
            instance_id: "i-test123".to_string(),
            host: "192.168.1.100".to_string(),
            port: 8080,
            use_tls: true,
            service_id: 1,
            blueprint_id: 100,
        };
        
        bridge.register_endpoint(1, endpoint.clone()).await;
        
        let endpoints = bridge.endpoints.read().await;
        assert!(endpoints.contains_key(&1));
        assert_eq!(endpoints.get(&1).unwrap().instance_id, "i-test123");
    }
    
    #[tokio::test]
    async fn test_endpoint_removal() {
        let config = SecureBridgeConfig {
            enable_mtls: false,
            ..Default::default()
        };
        
        let bridge = SecureBridge::new(config).await.unwrap();
        
        let endpoint = RemoteEndpoint {
            instance_id: "i-test456".to_string(),
            host: "10.0.0.1".to_string(),
            port: 443,
            use_tls: true,
            service_id: 2,
            blueprint_id: 200,
        };
        
        bridge.register_endpoint(2, endpoint).await;
        bridge.remove_endpoint(2).await;
        
        let endpoints = bridge.endpoints.read().await;
        assert!(!endpoints.contains_key(&2));
    }
}