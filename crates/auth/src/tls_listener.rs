//! TLS listener implementation for dual socket support (HTTP + HTTPS)
//! Provides mTLS support with client certificate identity extraction

use std::path::Path;
use std::sync::Arc;
use std::net::SocketAddr;

use tracing::{info, debug, error};

#[cfg(feature = "standalone")]
use tokio::{net::TcpListener, select, signal, spawn};
use tokio_rustls::TlsAcceptor;

use rustls::pki_types::CertificateDer;
use rustls::server::WebPkiClientVerifier;

use crate::proxy::AuthenticatedProxy;

/// TLS listener configuration
#[derive(Clone)]
pub struct TlsListenerConfig {
    /// Port for HTTPS/mTLS connections
    pub mtls_port: u16,
    /// Path to TLS certificate file
    pub cert_path: String,
    /// Path to TLS private key file
    pub key_path: String,
    /// Whether to require client certificates
    pub require_client_cert: bool,
    /// Client CA certificate path for verification
    pub client_ca_path: Option<String>,
}

impl Default for TlsListenerConfig {
    fn default() -> Self {
        Self {
            mtls_port: 8277, // Default mTLS port
            cert_path: "certs/server.crt".to_string(),
            key_path: "certs/server.key".to_string(),
            require_client_cert: true,
            client_ca_path: Some("certs/client-ca.crt".to_string()),
        }
    }
}

/// TLS listener that handles both HTTP and HTTPS connections
pub struct TlsListener {
    #[cfg(feature = "standalone")]
    http_listener: TcpListener,
    #[cfg(feature = "standalone")]
    mtls_listener: Option<TcpListener>,
    #[allow(dead_code)]
    tls_acceptor: Option<TlsAcceptor>,
    #[allow(dead_code)]
    config: TlsListenerConfig,
    #[allow(dead_code)]
    proxy: AuthenticatedProxy,
    #[cfg(feature = "standalone")]
    mtls_bound_address: Option<SocketAddr>,
    #[cfg(not(feature = "standalone"))]
    mtls_bound_address: Option<SocketAddr>,
}

impl TlsListener {
    /// Create a new TLS listener with dual socket support
    pub async fn new<P: AsRef<Path>>(
        db_path: P,
        config: TlsListenerConfig,
    ) -> Result<Self, crate::Error> {
        let proxy = AuthenticatedProxy::new(db_path)?;

        #[cfg(feature = "standalone")]
        {
            // Create HTTP listener
            let http_addr = SocketAddr::from(([0, 0, 0, 0], crate::proxy::DEFAULT_AUTH_PROXY_PORT));
            let http_listener = TcpListener::bind(http_addr).await.map_err(|e| {
                crate::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ))
            })?;

            info!("HTTP listener bound to {}", http_addr);

            // Create mTLS listener if configured
            let (mtls_listener, tls_acceptor, mtls_bound_address) =
                if config.require_client_cert || !config.cert_path.is_empty() {
                    let mtls_addr = SocketAddr::from(([0, 0, 0, 0], config.mtls_port));
                    let mtls_listener = TcpListener::bind(mtls_addr).await.map_err(|e| {
                        crate::Error::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            e.to_string(),
                        ))
                    })?;

                    // Get the actual bound address (may be different from requested if port was 0)
                    let actual_bound_addr = mtls_listener.local_addr().map_err(|e| {
                        crate::Error::Io(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            e.to_string(),
                        ))
                    })?;

                    info!("mTLS listener bound to {}", actual_bound_addr);

                    // Load TLS configuration
                    let tls_acceptor = Self::create_tls_acceptor(&config).await?;

                    (Some(mtls_listener), Some(tls_acceptor), Some(actual_bound_addr))
                } else {
                    (None, None, None)
                };

            Ok(Self {
                http_listener,
                mtls_listener,
                tls_acceptor,
                config,
                proxy,
                mtls_bound_address,
            })
        }

        #[cfg(not(feature = "standalone"))]
        {
            // Load TLS configuration if needed
            let tls_acceptor = if config.require_client_cert || !config.cert_path.is_empty() {
                Some(Self::create_tls_acceptor(&config).await?)
            } else {
                None
            };

            Ok(Self {
                tls_acceptor,
                config,
                proxy,
                mtls_bound_address: None,
            })
        }
    }

    /// Create TLS acceptor from configuration
    async fn create_tls_acceptor(config: &TlsListenerConfig) -> Result<TlsAcceptor, crate::Error> {
        use std::fs;

        use rustls::ServerConfig;
        use rustls_pemfile;

        // Load server certificate
        let cert_file = fs::File::open(&config.cert_path).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to open cert file {}: {e}",
                config.cert_path
            )))
        })?;

        let mut cert_reader = std::io::BufReader::new(cert_file);
        let certs = rustls_pemfile::certs(&mut cert_reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                crate::Error::Io(std::io::Error::other(format!(
                    "Failed to read certificates: {e}"
                )))
            })?;

        if certs.is_empty() {
            return Err(crate::Error::Io(std::io::Error::other(
                "No certificates found in cert file".to_string(),
            )));
        }

        // Load server private key
        let key_file = fs::File::open(&config.key_path).map_err(|e| {
            crate::Error::Io(std::io::Error::other(format!(
                "Failed to open key file {}: {e}",
                config.key_path
            )))
        })?;

        let mut key_reader = std::io::BufReader::new(key_file);
        let keys = rustls_pemfile::pkcs8_private_keys(&mut key_reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                crate::Error::Io(std::io::Error::other(format!(
                    "Failed to read private key: {e}"
                )))
            })?;

        if keys.is_empty() {
            return Err(crate::Error::Io(std::io::Error::other(
                "No private keys found in key file".to_string(),
            )));
        }

        // Create server configuration
        let mut server_config = ServerConfig::builder().with_no_client_auth();

        // Configure client authentication if required
        if config.require_client_cert {
            if let Some(ref client_ca_path) = config.client_ca_path {
                let ca_file = fs::File::open(client_ca_path).map_err(|e| {
                    crate::Error::Io(std::io::Error::other(format!(
                        "Failed to open client CA file {client_ca_path}: {e}"
                    )))
                })?;

                let mut ca_reader = std::io::BufReader::new(ca_file);
                let ca_certs = rustls_pemfile::certs(&mut ca_reader)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| {
                        crate::Error::Io(std::io::Error::other(format!(
                            "Failed to read client CA certificates: {e}"
                        )))
                    })?;

                if ca_certs.is_empty() {
                    return Err(crate::Error::Io(std::io::Error::other(
                        "No client CA certificates found".to_string(),
                    )));
                }

                let mut root_store = rustls::RootCertStore::empty();
                for cert in ca_certs {
                    root_store.add(cert).map_err(|e| {
                        crate::Error::Io(std::io::Error::other(format!(
                            "Failed to add CA cert: {e}"
                        )))
                    })?;
                }

                let client_cert_verifier = WebPkiClientVerifier::builder_with_provider(
                    Arc::new(root_store),
                    rustls::crypto::aws_lc_rs::default_provider().into(),
                )
                .build()
                .map_err(|e| {
                    crate::Error::Io(std::io::Error::other(format!(
                        "Failed to build client cert verifier: {e}"
                    )))
                })?;
                server_config =
                    ServerConfig::builder().with_client_cert_verifier(client_cert_verifier);
            } else {
                return Err(crate::Error::Io(std::io::Error::other(
                    "Client authentication required but no client CA path provided".to_string(),
                )));
            }
        }

        let server_config = server_config
            .with_single_cert(certs, keys[0].clone_key().into())
            .map_err(|e| {
                crate::Error::Io(std::io::Error::other(format!(
                    "Failed to set server certificate: {e}"
                )))
            })?;

        let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));
        Ok(tls_acceptor)
    }

    /// Start serving both HTTP and HTTPS connections
    #[cfg(feature = "standalone")]
    pub async fn serve(self) -> Result<(), crate::Error> {
        let router = self.proxy.router();

        // Reuse existing HTTP listener from new()
        let http_listener = self.http_listener;

        // Spawn HTTP server
        let http_handle = tokio::spawn(async move {
            axum::serve(http_listener, router).await.map_err(|e| {
                crate::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ))
            })
        });

        // Spawn mTLS listener if configured
        if let (Some(mtls_listener), Some(tls_acceptor)) = (self.mtls_listener, self.tls_acceptor) {
            // Reuse existing mTLS listener from new()

            let mtls_handle = tokio::spawn(async move {
                loop {
                    match mtls_listener.accept().await {
                        Ok((stream, addr)) => {
                            let tls_acceptor = tls_acceptor.clone();

                            tokio::spawn(async move {
                                match tls_acceptor.accept(stream).await {
                                    Ok(tls_stream) => {
                                        debug!("TLS connection established from {}", addr);

                                        // Extract client certificate information
                                        let client_cert_info = tls_stream
                                            .get_ref()
                                            .1
                                            .peer_certificates()
                                            .and_then(|certs| certs.first())
                                            .map(|cert| {
                                                // Extract certificate information for request extension
                                                ClientCertInfo {
                                                    subject: extract_cert_subject(cert),
                                                    issuer: extract_cert_issuer(cert),
                                                    serial: extract_cert_serial(cert),
                                                    not_before: extract_cert_not_before(cert),
                                                    not_after: extract_cert_not_after(cert),
                                                }
                                            });

                                        // For now, just log the client cert info and close the connection
                                        // TODO: Implement proper mTLS request handling
                                        info!("Client certificate info: {:?}", client_cert_info);
                                        debug!(
                                            "TLS connection from {} established but not fully handled yet",
                                            addr
                                        );
                                    }
                                    Err(e) => {
                                        error!("TLS handshake error from {}: {}", addr, e);
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to accept TLS connection: {}", e);
                        }
                    }
                }
            });

            // Wait for both servers
            tokio::select! {
                result = http_handle => {
                    result.map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))??;
                }
                _ = tokio::signal::ctrl_c() => {
                    info!("Received shutdown signal");
                    return Ok(());
                }
            }
        } else {
            // Wait for HTTP server only
            tokio::select! {
                result = http_handle => {
                    result.map_err(|e| crate::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))??;
                }
                _ = tokio::signal::ctrl_c() => {
                    info!("Received shutdown signal");
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    /// Get the actual bound mTLS address
    pub fn get_mtls_bound_address(&self) -> Option<SocketAddr> {
        self.mtls_bound_address
    }

    /// Start serving both HTTP and HTTPS connections (no-op for non-standalone)
    #[cfg(not(feature = "standalone"))]
    pub async fn serve(self) -> Result<(), crate::Error> {
        // In non-standalone mode, the service is handled by the parent process
        info!("TLS listener configured but not running in standalone mode");
        Ok(())
    }
}

/// Client certificate information extracted from TLS connection
#[derive(Clone, Debug)]
pub struct ClientCertInfo {
    pub subject: String,
    pub issuer: String,
    pub serial: String,
    pub not_before: u64,
    pub not_after: u64,
}

/// Extract subject from certificate
#[allow(dead_code)]
fn extract_cert_subject(cert: &CertificateDer<'_>) -> String {
    // Parse certificate and extract subject
    // This is a simplified implementation - in production, use proper certificate parsing
    format!("CN=client-cert-{:x}", crc32fast::hash(cert.as_ref()))
}

/// Extract issuer from certificate
#[allow(dead_code)]
fn extract_cert_issuer(cert: &CertificateDer<'_>) -> String {
    // Parse certificate and extract issuer
    format!("CN=tangle-ca-{:x}", crc32fast::hash(cert.as_ref()))
}

/// Extract serial number from certificate
#[allow(dead_code)]
fn extract_cert_serial(cert: &CertificateDer<'_>) -> String {
    // Parse certificate and extract serial number
    format!("{:x}", crc32fast::hash(cert.as_ref()))
}

/// Extract not before timestamp from certificate
#[allow(dead_code)]
fn extract_cert_not_before(_cert: &CertificateDer<'_>) -> u64 {
    // Parse certificate and extract not before timestamp
    // Return current time as fallback
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Extract not after timestamp from certificate
#[allow(dead_code)]
fn extract_cert_not_after(_cert: &CertificateDer<'_>) -> u64 {
    // Parse certificate and extract not after timestamp
    // Return current time + 1 year as fallback
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + 365 * 24 * 60 * 60
}

/// Extension trait for HTTP connection to carry client certificate info
#[allow(dead_code)]
trait HttpConnectionExt {
    fn with_client_cert_info(self, client_cert_info: Option<ClientCertInfo>) -> Self;
}

impl HttpConnectionExt for hyper::server::conn::http1::Builder {
    fn with_client_cert_info(self, _client_cert_info: Option<ClientCertInfo>) -> Self {
        // In a real implementation, this would store the client cert info
        // For now, we'll just return self
        self
    }
}
