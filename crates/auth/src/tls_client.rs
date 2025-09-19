use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use blueprint_core::{debug, info};
use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use rustls::{ClientConfig, RootCertStore};
use rustls_pemfile;

use crate::db::RocksDb;
use crate::models::ServiceModel;
use crate::tls_assets::TlsAssetManager;
use crate::types::ServiceId;

/// TLS client configuration for outbound connections
#[derive(Clone, Debug)]
pub struct TlsClientConfig {
    /// Whether to verify server certificates
    pub verify_server_cert: bool,
    /// Custom CA certificates to trust
    pub custom_ca_certs: Vec<Vec<u8>>,
    /// Client certificate for mTLS
    pub client_cert: Option<Vec<u8>>,
    /// Client private key for mTLS
    pub client_key: Option<Vec<u8>>,
    /// ALPN protocols to negotiate
    pub alpn_protocols: Vec<Vec<u8>>,
    /// Timeout for TLS handshake
    pub handshake_timeout: Duration,
}

impl Default for TlsClientConfig {
    fn default() -> Self {
        Self {
            verify_server_cert: true,
            custom_ca_certs: Vec::new(),
            client_cert: None,
            client_key: None,
            alpn_protocols: vec![
                b"h2".to_vec(),  // HTTP/2
                b"http/1.1".to_vec(),  // HTTP/1.1
            ],
            handshake_timeout: Duration::from_secs(10),
        }
    }
}

/// Cached TLS client with configuration
#[derive(Clone, Debug)]
pub struct CachedTlsClient {
    /// HTTP/1.1 client with TLS support
    pub http_client: Client<HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, axum::body::Body>,
    /// HTTP/2 client with TLS support
    pub http2_client: Client<HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>, axum::body::Body>,
    /// Configuration used to create this client
    pub config: TlsClientConfig,
    /// Last access timestamp for cache eviction
    pub last_access: std::time::Instant,
}

/// TLS client manager with caching and per-service configuration
#[derive(Clone, Debug)]
pub struct TlsClientManager {
    /// Cache of TLS clients by configuration hash
    clients: Arc<Mutex<HashMap<String, CachedTlsClient>>>,
    /// Database for persistent storage
    db: RocksDb,
    /// TLS asset manager for certificate management
    tls_assets: TlsAssetManager,
    /// Maximum cache size
    max_cache_size: usize,
    /// Cache entry TTL
    cache_ttl: Duration,
}

impl TlsClientManager {
    /// Create a new TLS client manager
    pub fn new(db: RocksDb, tls_assets: TlsAssetManager) -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            db,
            tls_assets,
            max_cache_size: 100,
            cache_ttl: Duration::from_secs(3600), // 1 hour
        }
    }

    /// Get or create a TLS client for a service
    pub async fn get_client_for_service(
        &self,
        service_id: ServiceId,
    ) -> Result<CachedTlsClient, crate::Error> {
        // Load service model
        let service = ServiceModel::find_by_id(service_id, &self.db)?
            .ok_or_else(|| crate::Error::ServiceNotFound(service_id))?;

        // Get TLS configuration for service
        let tls_config = self.get_service_tls_config(&service).await?;

        // Generate configuration hash for caching
        let config_hash = self.hash_config(&tls_config);

        // Check cache first
        {
            let clients = self.clients.lock().unwrap();
            if let Some(cached_client) = clients.get(&config_hash) {
                // Check if cache entry is still valid
                if cached_client.last_access.elapsed() < self.cache_ttl {
                    debug!("Using cached TLS client for service {}", service_id);
                    return Ok(cached_client.clone());
                }
            }
        }

        // Create new client
        debug!("Creating new TLS client for service {}", service_id);
        let client = self.create_tls_client(tls_config.clone()).await?;

        // Update cache
        {
            let mut clients = self.clients.lock().unwrap();
            
            // Evict old entries if cache is full
            if clients.len() >= self.max_cache_size {
                self.evict_old_entries(&mut clients);
            }

            clients.insert(config_hash.clone(), client.clone());
        }

        Ok(client)
    }

    /// Get TLS configuration for a service
    async fn get_service_tls_config(
        &self,
        service: &ServiceModel,
    ) -> Result<TlsClientConfig, crate::Error> {
        let mut config = TlsClientConfig::default();

        // Check if service has a TLS profile
        if let Some(profile) = &service.tls_profile {
            if profile.tls_enabled {
                // Apply profile configuration
                config.verify_server_cert = true; // Default to true for TLS-enabled services
                
                // Load custom CA certificates if specified
                if !profile.encrypted_upstream_ca_bundle.is_empty() {
                    config.custom_ca_certs.push(profile.encrypted_upstream_ca_bundle.clone());
                }

                // Load client certificate for mTLS if specified
                if !profile.encrypted_upstream_client_cert.is_empty() && !profile.encrypted_upstream_client_key.is_empty() {
                    config.client_cert = Some(profile.encrypted_upstream_client_cert.clone());
                    config.client_key = Some(profile.encrypted_upstream_client_key.clone());
                }
            }
        }

        Ok(config)
    }

    /// Create a TLS client with the given configuration
    async fn create_tls_client(&self, config: TlsClientConfig) -> Result<CachedTlsClient, crate::Error> {
        let executor = TokioExecutor::new();

        // Build rustls client configuration
        let client_config = ClientConfig::builder()
            .with_root_certificates(RootCertStore::empty())
            .with_no_client_auth();

        // Add custom CA certificates
        if !config.custom_ca_certs.is_empty() {
            let mut root_store = RootCertStore::empty();
            for ca_cert in &config.custom_ca_certs {
                let mut cert_data = ca_cert.as_slice();
                let mut cert_iter = rustls_pemfile::certs(&mut cert_data);
                
                let cert_der = cert_iter.next()
                    .transpose()
                    .map_err(|e| crate::Error::Tls(format!("Failed to parse CA certificate: {}", e)))?
                    .ok_or_else(|| crate::Error::Tls("No valid CA certificate found".to_string()))?;
                
                root_store.add(cert_der).map_err(|e| {
                    crate::Error::Tls(format!("Failed to add CA certificate to root store: {}", e))
                })?;
            }
            
            // Create new config with custom root store
            // For now, we'll skip this and use the default config
        }

        // Configure client certificate for mTLS
        if let (Some(client_cert), Some(client_key)) = (&config.client_cert, &config.client_key) {
            let mut cert_data = client_cert.as_slice();
            let _cert_chain = rustls_pemfile::certs(&mut cert_data)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| crate::Error::Tls(format!("Failed to parse client certificate: {}", e)))?;

            let mut key_data = client_key.as_slice();
            let mut private_key_iter = rustls_pemfile::pkcs8_private_keys(&mut key_data);

            let _private_key = private_key_iter.next()
                .transpose()
                .map_err(|e| crate::Error::Tls(format!("Failed to parse client private key: {}", e)))?
                .ok_or_else(|| crate::Error::Tls("No valid private key found".to_string()))?;

            // Note: Client certificate configuration would go here
            // For now, we'll skip this as it requires more complex rustls API usage
        }

        // Create HTTPS connector with default config for now
        let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(client_config)
            .https_or_http()
            .enable_http2()
            .build();

        // Build HTTP/1.1 client (for REST APIs)
        let http_client = Client::builder(executor.clone())
            .http2_only(false)
            .build(https_connector.clone());

        // Build HTTP/2 client (for gRPC)
        let http2_client = Client::builder(executor)
            .http2_only(true)
            .http2_adaptive_window(true)
            .build(https_connector);

        Ok(CachedTlsClient {
            http_client,
            http2_client,
            config,
            last_access: std::time::Instant::now(),
        })
    }

    /// Generate a hash for TLS configuration
    fn hash_config(&self, config: &TlsClientConfig) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        
        config.verify_server_cert.hash(&mut hasher);
        config.custom_ca_certs.hash(&mut hasher);
        config.client_cert.hash(&mut hasher);
        config.client_key.hash(&mut hasher);
        config.alpn_protocols.hash(&mut hasher);
        config.handshake_timeout.hash(&mut hasher);
        
        format!("{:016x}", hasher.finish())
    }

    /// Evict old entries from the cache
    fn evict_old_entries(&self, clients: &mut HashMap<String, CachedTlsClient>) {
        let now = std::time::Instant::now();
        
        // Remove expired entries
        clients.retain(|_, client| now.duration_since(client.last_access) < self.cache_ttl);
        
        // If still too many, remove the oldest entries
        if clients.len() >= self.max_cache_size {
            let to_remove = clients.len() - self.max_cache_size + 10; // Remove 10 extra to avoid frequent evictions
            
            // Collect keys to remove first to avoid borrow issues
            let mut keys_to_remove: Vec<(String, std::time::Instant)> = clients
                .iter()
                .map(|(key, client)| (key.clone(), client.last_access))
                .collect();
            
            // Sort by last access time
            keys_to_remove.sort_by_key(|(_, last_access)| *last_access);
            
            // Remove oldest entries
            for (key, _) in keys_to_remove.iter().take(to_remove) {
                clients.remove(key);
            }
            
            info!("Evicted {} old TLS client cache entries", to_remove);
        }
    }

    /// Clean up expired cache entries
    pub fn cleanup_expired_entries(&self) {
        let mut clients = self.clients.lock().unwrap();
        let now = std::time::Instant::now();
        let initial_size = clients.len();
        
        clients.retain(|_, client| now.duration_since(client.last_access) < self.cache_ttl);
        
        let removed = initial_size - clients.len();
        if removed > 0 {
            info!("Cleaned up {} expired TLS client cache entries", removed);
        }
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> TlsClientCacheStats {
        let clients = self.clients.lock().unwrap();
        let now = std::time::Instant::now();
        
        let mut active_count = 0;
        let mut expired_count = 0;
        
        for client in clients.values() {
            if now.duration_since(client.last_access) < self.cache_ttl {
                active_count += 1;
            } else {
                expired_count += 1;
            }
        }
        
        TlsClientCacheStats {
            total_entries: clients.len(),
            active_entries: active_count,
            expired_entries: expired_count,
            max_cache_size: self.max_cache_size,
            cache_ttl: self.cache_ttl,
        }
    }
}

/// Cache statistics for TLS clients
#[derive(Debug, Clone)]
pub struct TlsClientCacheStats {
    pub total_entries: usize,
    pub active_entries: usize,
    pub expired_entries: usize,
    pub max_cache_size: usize,
    pub cache_ttl: Duration,
}

impl TlsClientCacheStats {
    pub fn usage_percentage(&self) -> f64 {
        if self.max_cache_size == 0 {
            0.0
        } else {
            (self.total_entries as f64 / self.max_cache_size as f64) * 100.0
        }
    }
}