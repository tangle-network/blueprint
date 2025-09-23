//! TLS listener runtime for terminating inbound TLS/mTLS connections and
//! forwarding them into the authenticated proxy router.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::{Router, body::Body};
use blueprint_core::{debug, error, info, warn};
use hyper::service::service_fn;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder as AutoH2Builder,
};
use once_cell::sync::OnceCell;
use rustls::crypto::ring;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use rustls::{RootCertStore, ServerConfig};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use tokio_rustls::{LazyConfigAcceptor, server::TlsStream};
use tower::Service;
use tracing::{Instrument, info_span};
use x509_parser::prelude::*;

use crate::db::RocksDb;
use crate::models::TlsProfile;
use crate::tls_envelope::TlsEnvelope;
use crate::types::ServiceId;

/// Default bind port for the mutual TLS listener.
pub const DEFAULT_MTLS_PORT: u16 = 8277;

/// Listener configuration.
#[derive(Clone, Debug)]
pub struct TlsListenerConfig {
    /// Socket address to bind for the inbound TLS listener.
    pub bind_addr: SocketAddr,
}

impl Default for TlsListenerConfig {
    fn default() -> Self {
        Self {
            bind_addr: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                std::env::var("AUTH_PROXY_MTLS_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(DEFAULT_MTLS_PORT),
            ),
        }
    }
}

/// Runtime supervisor that owns the TLS listener and per-service TLS material.
#[derive(Clone)]
pub struct TlsListenerManager {
    inner: Arc<TlsListenerInner>,
}

struct TlsListenerInner {
    db: RocksDb,
    envelope: TlsEnvelope,
    config: TlsListenerConfig,
    router: OnceCell<Router<()>>,
    profiles: RwLock<HashMap<ServiceId, Arc<ServiceTlsConfig>>>,
    sni_index: RwLock<HashMap<String, ServiceId>>,
    lifecycle: Mutex<ListenerLifecycle>,
}

#[derive(Default)]
struct ListenerLifecycle {
    bound_addr: Option<SocketAddr>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    handle: Option<JoinHandle<()>>,
}

#[derive(Clone)]
struct ServiceTlsConfig {
    service_id: ServiceId,
    server_config: Arc<ServerConfig>,
    require_client_mtls: bool,
    sni_hostname: Option<String>,
}

impl TlsListenerManager {
    /// Create a new listener manager.
    pub fn new(db: RocksDb, envelope: TlsEnvelope, config: TlsListenerConfig) -> Self {
        Self {
            inner: Arc::new(TlsListenerInner {
                db,
                envelope,
                config,
                router: OnceCell::new(),
                profiles: RwLock::new(HashMap::new()),
                sni_index: RwLock::new(HashMap::new()),
                lifecycle: Mutex::new(ListenerLifecycle::default()),
            }),
        }
    }

    /// Install the axum router that should handle decrypted traffic.
    pub fn install_router(&self, router: Router<()>) {
        if self.inner.router.set(router).is_err() {
            warn!("TLS router already installed; skipping reinstallation");
            return;
        }

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let manager = self.clone();
            handle.spawn(async move {
                if let Err(err) = manager.ensure_listener().await {
                    error!(
                        "failed to start TLS listener after router installation: {}",
                        err
                    );
                }
            });
        } else {
            warn!("No Tokio runtime available; TLS listener start deferred");
        }
    }

    /// Returns the bound listener address if the TLS listener is running.
    pub async fn mtls_addr(&self) -> Option<SocketAddr> {
        let lifecycle = self.inner.lifecycle.lock().await;
        lifecycle.bound_addr
    }

    /// Upsert a service TLS profile and ensure the listener is running with the new configuration.
    pub async fn upsert_service_profile(
        &self,
        service_id: ServiceId,
        profile: &TlsProfile,
    ) -> Result<SocketAddr, crate::Error> {
        let config = self.build_service_config(service_id, profile)?;
        self.store_service_config(config).await;

        self.ensure_listener().await?;
        self.mtls_addr()
            .await
            .ok_or_else(|| crate::Error::Tls("TLS listener failed to initialise".into()))
    }

    pub async fn load_service_profile(
        &self,
        service_id: ServiceId,
        profile: &TlsProfile,
    ) -> Result<(), crate::Error> {
        let config = self.build_service_config(service_id, profile)?;
        self.store_service_config(config).await;
        Ok(())
    }

    async fn store_service_config(&self, config: ServiceTlsConfig) {
        let service_id = config.service_id;
        let sni_hostname = config.sni_hostname.clone();
        let config = Arc::new(config);
        {
            let mut profiles = self.inner.profiles.write().await;
            profiles.insert(service_id, config.clone());
        }
        if let Some(hostname) = sni_hostname {
            let mut index = self.inner.sni_index.write().await;
            index.insert(hostname, service_id);
        }
    }

    fn build_service_config(
        &self,
        service_id: ServiceId,
        profile: &TlsProfile,
    ) -> Result<ServiceTlsConfig, crate::Error> {
        if profile.encrypted_server_cert.is_empty() || profile.encrypted_server_key.is_empty() {
            return Err(crate::Error::Tls(
                "TLS profile missing server certificate material".into(),
            ));
        }
        if profile.encrypted_client_ca_bundle.is_empty() {
            return Err(crate::Error::Tls(
                "TLS profile missing client CA bundle".into(),
            ));
        }

        let server_cert_pem = self.decrypt_utf8(&profile.encrypted_server_cert)?;
        let server_key_pem = self.decrypt_utf8(&profile.encrypted_server_key)?;
        let ca_bundle_pem = self.decrypt_utf8(&profile.encrypted_client_ca_bundle)?;

        let server_chain = load_cert_chain(&server_cert_pem)?;
        let private_key = load_private_key(&server_key_pem)?;
        let client_roots = load_ca_store(&ca_bundle_pem)?;

        let verifier = if profile.require_client_mtls {
            Some(
                WebPkiClientVerifier::builder_with_provider(
                    Arc::new(client_roots),
                    ring::default_provider().into(),
                )
                .build()
                .map_err(|err| {
                    crate::Error::Tls(format!("failed to construct client verifier: {err}"))
                })?,
            )
        } else {
            None
        };

        let builder = if let Some(verifier) = verifier {
            ServerConfig::builder().with_client_cert_verifier(verifier)
        } else {
            ServerConfig::builder().with_no_client_auth()
        };

        let mut server_config = builder
            .with_single_cert(server_chain, private_key)
            .map_err(|err| {
                crate::Error::Tls(format!("failed to configure server certificate: {err}"))
            })?;
        server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        Ok(ServiceTlsConfig {
            service_id,
            server_config: Arc::new(server_config),
            require_client_mtls: profile.require_client_mtls,
            sni_hostname: profile.sni.clone(),
        })
    }

    async fn ensure_listener(&self) -> Result<(), crate::Error> {
        let mut lifecycle = self.inner.lifecycle.lock().await;
        if lifecycle.handle.is_some() {
            return Ok(());
        }

        if self.inner.router.get().is_none() {
            debug!("TLS router not yet installed; deferring listener startup");
            return Ok(());
        }

        let bind_addr = self.inner.config.bind_addr;
        let listener = match TcpListener::bind(bind_addr).await {
            Ok(listener) => listener,
            Err(err) if err.kind() == std::io::ErrorKind::AddrInUse => {
                warn!(
                    "mTLS listener port {} in use, falling back to ephemeral",
                    bind_addr
                );
                let fallback_addr = SocketAddr::new(bind_addr.ip(), 0);
                TcpListener::bind(fallback_addr)
                    .await
                    .map_err(crate::Error::Io)?
            }
            Err(err) => return Err(crate::Error::Io(err)),
        };
        let local_addr = listener.local_addr().map_err(crate::Error::Io)?;
        info!("mTLS listener bound to {}", local_addr);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let manager = self.clone();
        let span = info_span!("mtls_accept_loop", %local_addr);
        let handle = tokio::spawn(async move {
            manager
                .run_accept_loop(listener, shutdown_rx)
                .instrument(span)
                .await;
        });

        lifecycle.bound_addr = Some(local_addr);
        lifecycle.shutdown_tx = Some(shutdown_tx);
        lifecycle.handle = Some(handle);
        Ok(())
    }

    async fn run_accept_loop(self, listener: TcpListener, mut shutdown: oneshot::Receiver<()>) {
        loop {
            tokio::select! {
                biased;
                _ = &mut shutdown => {
                    info!("mTLS listener shutting down");
                    break;
                }
                accept = listener.accept() => {
                    match accept {
                        Ok((stream, addr)) => {
                            let this = self.clone();
                            tokio::spawn(async move {
                                if let Err(err) = this.handle_connection(stream, addr).await {
                                    error!("TLS connection error from {}: {}", addr, err);
                                }
                            });
                        }
                        Err(err) => {
                            error!("Failed to accept TLS connection: {}", err);
                            break;
                        }
                    }
                }
            }
        }
    }

    async fn handle_connection(
        &self,
        stream: TcpStream,
        peer_addr: SocketAddr,
    ) -> Result<(), crate::Error> {
        let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), stream);
        let start = acceptor
            .await
            .map_err(|err| crate::Error::Tls(format!("TLS client hello error: {err}")))?;
        let client_hello = start.client_hello();
        let server_name = client_hello.server_name().map(str::to_owned);
        let service_config = self.select_service_config(server_name.as_deref()).await?;

        let tls_stream = start
            .into_stream(service_config.server_config.clone())
            .await
            .map_err(|err| crate::Error::Tls(format!("TLS handshake error: {err}")))?;

        debug!(
            service_id = %service_config.service_id,
            sni = server_name.as_deref().unwrap_or("<none>"),
            require_client_mtls = service_config.require_client_mtls,
            peer = %peer_addr,
            "TLS handshake completed"
        );

        let cert_info = client_cert_info_from_stream(&tls_stream);
        let router = self.inner.router.get().expect("router installed").clone();

        let service_id = service_config.service_id;
        let svc_cert = cert_info.clone();
        let service = service_fn(move |mut req| {
            let svc = router.clone();
            let cert_info = svc_cert.clone();
            async move {
                if let Some(info) = cert_info {
                    req.extensions_mut().insert(info);
                }
                req.extensions_mut().insert(service_id);

                let req = req.map(Body::new);

                let mut router_service = svc.clone().into_service();
                let response = Service::call(&mut router_service, req)
                    .await
                    .expect("router service should be infallible");

                Ok::<_, hyper::Error>(response)
            }
        });

        let builder = AutoH2Builder::new(TokioExecutor::new());
        builder
            .serve_connection(TokioIo::new(tls_stream), service)
            .await
            .map_err(|err| crate::Error::Tls(format!("TLS proxy connection error: {err}")))
    }

    async fn select_service_config(
        &self,
        sni: Option<&str>,
    ) -> Result<Arc<ServiceTlsConfig>, crate::Error> {
        let profiles = self.inner.profiles.read().await;

        if let Some(server_name) = sni {
            let maybe_service_id = {
                let index = self.inner.sni_index.read().await;
                index.get(server_name).copied()
            };

            if let Some(service_id) = maybe_service_id {
                if let Some(config) = profiles.get(&service_id) {
                    return Ok(config.clone());
                }
            }

            if profiles.len() == 1 {
                // Guarded fallback to support the brief window while SNI index updates.
                return Ok(profiles.values().next().cloned().expect("length checked"));
            }

            return Err(crate::Error::Tls(format!(
                "unrecognized TLS server name: {server_name}"
            )));
        }

        match profiles.len() {
            0 => Err(crate::Error::Tls("no TLS profiles configured".into())),
            1 => Ok(profiles.values().next().cloned().expect("length checked")),
            _ => Err(crate::Error::Tls(
                "TLS client did not provide SNI and multiple profiles are configured".into(),
            )),
        }
    }

    fn decrypt_utf8(&self, data: &[u8]) -> Result<String, crate::Error> {
        let bytes = self.inner.envelope.decrypt(data)?;
        String::from_utf8(bytes)
            .map_err(|err| crate::Error::Tls(format!("invalid UTF-8 in TLS material: {err}")))
    }

    pub fn envelope(&self) -> &TlsEnvelope {
        &self.inner.envelope
    }

    pub fn db(&self) -> &RocksDb {
        &self.inner.db
    }
}

impl Drop for TlsListenerManager {
    fn drop(&mut self) {
        if Arc::strong_count(&self.inner) > 1 {
            return;
        }
        if let Ok(mut lifecycle) = self.inner.lifecycle.try_lock() {
            if let Some(tx) = lifecycle.shutdown_tx.take() {
                let _ = tx.send(());
            }
            if let Some(handle) = lifecycle.handle.take() {
                handle.abort();
            }
        }
    }
}

impl std::fmt::Debug for TlsListenerManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TlsListenerManager").finish()
    }
}

/// Metadata extracted from a validated client certificate.
#[derive(Clone, Debug)]
pub struct ClientCertInfo {
    pub subject: String,
    pub issuer: String,
    pub serial: String,
    pub not_before: u64,
    pub not_after: u64,
}

fn load_cert_chain(pem: &str) -> Result<Vec<CertificateDer<'static>>, crate::Error> {
    let mut reader = std::io::Cursor::new(pem.as_bytes());
    let certs = rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| crate::Error::Tls(format!("failed to parse certificate chain: {err}")))?;
    if certs.is_empty() {
        return Err(crate::Error::Tls(
            "server certificate chain is empty".into(),
        ));
    }
    Ok(certs)
}

fn load_private_key(pem: &str) -> Result<PrivateKeyDer<'static>, crate::Error> {
    let mut reader = std::io::Cursor::new(pem.as_bytes());
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader);
    match keys.next() {
        Some(Ok(key)) => Ok(PrivateKeyDer::Pkcs8(key)),
        Some(Err(err)) => Err(crate::Error::Tls(format!(
            "failed to parse private key: {err}"
        ))),
        None => Err(crate::Error::Tls("server private key not found".into())),
    }
}

fn load_ca_store(pem_bundle: &str) -> Result<RootCertStore, crate::Error> {
    let mut store = RootCertStore::empty();
    let mut reader = std::io::Cursor::new(pem_bundle.as_bytes());
    let mut loaded_any = false;
    for item in rustls_pemfile::read_all(&mut reader) {
        match item.map_err(|err| crate::Error::Tls(format!("failed to parse CA bundle: {err}")))? {
            rustls_pemfile::Item::X509Certificate(cert) => {
                store.add(cert).map_err(|err| {
                    crate::Error::Tls(format!("failed to add CA certificate to root store: {err}"))
                })?;
                loaded_any = true;
            }
            _ => {
                // Ignore non-certificate PEM blocks such as private keys.
            }
        }
    }

    if !loaded_any {
        return Err(crate::Error::Tls(
            "client CA bundle does not contain any certificates".into(),
        ));
    }

    Ok(store)
}

fn client_cert_info_from_stream(stream: &TlsStream<TcpStream>) -> Option<ClientCertInfo> {
    let (_, connection) = stream.get_ref();
    let cert = connection.peer_certificates()?.first()?.clone();
    parse_client_certificate(&cert)
}

fn parse_client_certificate(cert: &CertificateDer<'_>) -> Option<ClientCertInfo> {
    let (_, parsed) = parse_x509_certificate(cert.as_ref()).ok()?;
    let subject = parsed.subject().to_string();
    let issuer = parsed.issuer().to_string();
    let serial = hex::encode(parsed.raw_serial()).to_ascii_lowercase();
    let not_before = parsed.validity().not_before.timestamp();
    let not_after = parsed.validity().not_after.timestamp();

    // Convert timestamps to u64, handling negative values (certificates before 1970)
    // by using wrapping arithmetic to preserve the actual timestamp information
    let not_before_u64 = if not_before < 0 {
        not_before.wrapping_neg() as u64 | (1 << 63)
    } else {
        not_before as u64
    };
    let not_after_u64 = if not_after < 0 {
        not_after.wrapping_neg() as u64 | (1 << 63)
    } else {
        not_after as u64
    };

    Some(ClientCertInfo {
        subject,
        issuer,
        serial,
        not_before: not_before_u64,
        not_after: not_after_u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_addr_is_unspecified() {
        let config = TlsListenerConfig::default();
        assert_eq!(config.bind_addr.port(), DEFAULT_MTLS_PORT);
        assert!(matches!(config.bind_addr.ip(), IpAddr::V4(_)));
    }
}
