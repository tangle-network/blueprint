//! Red-stage tests describing desired mTLS proxy workflow.

use std::collections::BTreeMap;
use std::error::Error;
use std::io::Cursor;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use hyper::service::service_fn;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder as AutoH2Builder,
};
use rustls::crypto::CryptoProvider;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::{RootCertStore, server::WebPkiClientVerifier};
use rustls_pemfile;

use axum::{Router, http::StatusCode};
use crc32fast::hash as crc32_hash;
use futures_util::stream::{self, Stream};
use k256::ecdsa::SigningKey;
use serde::Deserialize;
use serde_json::{Value, json};
use tempfile::tempdir;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    task::JoinHandle,
};
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity, Server};
use tonic::{Code, Request, Response, Status};

use crate::certificate_authority::CertificateAuthority;
use crate::db::RocksDb;
use crate::models::ServiceModel;
use crate::proxy::AuthenticatedProxy;
use crate::test_client::TestClient;
use crate::tls_envelope::{TlsEnvelope, init_tls_envelope_key};
use crate::tls_listener::ClientCertInfo;
use crate::types::{
    ChallengeRequest, ChallengeResponse, KeyType, ServiceId, VerifyChallengeRequest,
    VerifyChallengeResponse, headers,
};

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub mod proto {
    tonic::include_proto!("blueprint.auth.grpcproxytest");
}

use pem::{Pem, parse_many};
use proto::{
    EchoRequest, EchoResponse,
    echo_service_client::EchoServiceClient,
    echo_service_server::{EchoService, EchoServiceServer},
};
use tower::ServiceExt;
use tracing::{error, info};

#[derive(Default)]
struct TestEchoService;

#[tonic::async_trait]
impl EchoService for TestEchoService {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        Ok(Response::new(EchoResponse {
            message: request.into_inner().message,
        }))
    }

    type EchoStreamStream =
        Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + 'static>>;

    async fn echo_stream(
        &self,
        request: Request<tonic::Streaming<EchoRequest>>,
    ) -> Result<Response<Self::EchoStreamStream>, Status> {
        let stream = request.into_inner();
        let outbound = stream::unfold(stream, |mut stream| async {
            match stream.message().await {
                Ok(Some(req)) => Some((
                    Ok(EchoResponse {
                        message: req.message,
                    }),
                    stream,
                )),
                Ok(None) => None,
                Err(status) => Some((Err(status), stream)),
            }
        });

        Ok(Response::new(Box::pin(outbound)))
    }
}

struct BackendHandle {
    shutdown_tx: Option<oneshot::Sender<()>>,
    join: JoinHandle<()>,
}

impl BackendHandle {
    fn shutdown(mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        self.join.abort();
    }
}

struct MtlsTestHarness {
    _tmp_dir: tempfile::TempDir,
    db_path: PathBuf,
    db: RocksDb,
    tls_router: Router,
    proxy_client: TestClient,
    proxy_addr: String,
    service_id: ServiceId,
    api_key: String,
    backend: Option<BackendHandle>,
    tls_shutdown: Option<oneshot::Sender<()>>,
    tls_join: Option<JoinHandle<()>>,
    mtls_addr: Option<SocketAddr>,
    auto_start_tls_listener: bool,
}

#[derive(Deserialize)]
struct CertificateResponse {
    certificate_pem: String,
    private_key_pem: String,
    ca_bundle_pem: String,
    expires_at: u64,
    serial: String,
    revocation_url: String,
}

impl MtlsTestHarness {
    async fn setup() -> Self {
        Self::setup_with_options(true).await
    }

    async fn setup_without_tls_listener() -> Self {
        Self::setup_with_options(false).await
    }

    async fn setup_with_options(auto_start_tls_listener: bool) -> Self {
        static INSTALL_PROVIDER: std::sync::Once = std::sync::Once::new();
        INSTALL_PROVIDER.call_once(|| {
            CryptoProvider::install_default(rustls::crypto::ring::default_provider())
                .expect("install ring provider");
        });

        // Note: Crypto provider should be installed by the test runner

        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp_dir = tempdir().expect("tempdir");
        let db_path = tmp_dir.path().to_path_buf();

        let (backend_addr, backend) = spawn_backend().await;

        let proxy = AuthenticatedProxy::new(tmp_dir.path()).expect("proxy init");
        let db = proxy.db();
        let service_id = ServiceId::new(9001);
        let signing_key = SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        let mut service = ServiceModel {
            api_key_prefix: "mtls_".to_string(),
            owners: vec![],
            upstream_url: format!("http://{backend_addr}"),
            tls_profile: None,
        };
        service.add_owner(KeyType::Ecdsa, public_key.to_vec());
        service.save(service_id, &proxy.db()).expect("save service");

        let router = proxy.router();
        let tls_router = router.clone();
        let proxy_client = TestClient::new(router);
        let proxy_addr = format!("http://127.0.0.1:{}", proxy_client.server_port());

        let api_key =
            issue_api_key(&proxy_client, service_id, signing_key, public_key.to_vec()).await;

        MtlsTestHarness {
            _tmp_dir: tmp_dir,
            db_path,
            db,
            tls_router,
            proxy_client,
            proxy_addr,
            service_id,
            api_key,
            backend: Some(backend),
            tls_shutdown: None,
            tls_join: None,
            mtls_addr: None,
            auto_start_tls_listener,
        }
    }

    fn service_header(&self) -> String {
        self.service_id.to_string()
    }

    async fn put_tls_profile(&mut self, body: Value) -> Value {
        let response = self
            .proxy_client
            .put(&format!(
                "/v1/admin/services/{}/tls-profile",
                self.service_header()
            ))
            .header(headers::X_SERVICE_ID, self.service_header())
            .json(&body)
            .await;

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "expected tls profile endpoint to acknowledge configuration, got status {}: {}",
            response.status(),
            response.text().await
        );

        response.json().await
    }

    async fn issue_certificate(&mut self, body: Value) -> CertificateResponse {
        let response = self
            .proxy_client
            .post("/v1/auth/certificates")
            .header(headers::X_SERVICE_ID, self.service_header())
            .header(headers::AUTHORIZATION, format!("Bearer {}", self.api_key))
            .json(&body)
            .await;

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "expected certificate issuance to succeed, got status {}: {}",
            response.status(),
            response.text().await
        );

        let cert: CertificateResponse = response.json().await;

        if self.auto_start_tls_listener && cert.ca_bundle_pem.contains("BEGIN CERTIFICATE") {
            self.ensure_tls_listener()
                .await
                .expect("start mtls listener");
        }

        cert
    }

    async fn ensure_tls_listener(&mut self) -> Result<SocketAddr, Box<dyn Error + Send + Sync>> {
        if let Some(addr) = self.mtls_addr {
            return Ok(addr);
        }

        let (addr, shutdown_tx, join) = spawn_tls_listener(
            self.db_path.as_path(),
            self.db.clone(),
            self.tls_router.clone(),
            self.service_id,
        )
        .await?;

        self.mtls_addr = Some(addr);
        self.tls_shutdown = Some(shutdown_tx);
        self.tls_join = Some(join);

        Ok(addr)
    }

    fn http_endpoint(&self) -> &str {
        &self.proxy_addr
    }

    async fn connect_grpc_over_http(&self) -> EchoServiceClient<Channel> {
        let channel = Channel::from_shared(self.http_endpoint().to_string())
            .expect("valid endpoint")
            .connect()
            .await
            .expect("connect over http");
        EchoServiceClient::new(channel)
    }

    async fn connect_grpc_with_tls(
        &self,
        listener: &str,
        client_identity: Identity,
        ca: Certificate,
    ) -> Result<EchoServiceClient<Channel>, tonic::transport::Error> {
        let tls = ClientTlsConfig::new()
            .ca_certificate(ca)
            .identity(client_identity)
            .domain_name("localhost");

        let target = self
            .mtls_addr
            .map(|addr| format!("https://{addr}"))
            .unwrap_or_else(|| format!("https://{listener}"));

        let endpoint = Endpoint::from_shared(target)?
            .timeout(Duration::from_secs(3))
            .tls_config(tls)?;

        endpoint.connect().await.map(EchoServiceClient::new)
    }
}

impl Drop for MtlsTestHarness {
    fn drop(&mut self) {
        if let Some(backend) = self.backend.take() {
            backend.shutdown();
        }
        if let Some(tx) = self.tls_shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.tls_join.take() {
            handle.abort();
        }
    }
}

async fn spawn_backend() -> (SocketAddr, BackendHandle) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind backend");
    let addr = listener.local_addr().expect("backend addr");
    let incoming = TcpListenerStream::new(listener);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let service = EchoServiceServer::new(TestEchoService);
    let join = tokio::spawn(async move {
        let shutdown = async {
            let _ = shutdown_rx.await;
        };

        if let Err(err) = Server::builder()
            .add_service(service)
            .serve_with_incoming_shutdown(incoming, shutdown)
            .await
        {
            eprintln!("backend error: {err}");
        }
    });

    (
        addr,
        BackendHandle {
            shutdown_tx: Some(shutdown_tx),
            join,
        },
    )
}

async fn issue_api_key(
    client: &TestClient,
    service_id: ServiceId,
    signing_key: SigningKey,
    public_key: Vec<u8>,
) -> String {
    let challenge_req = ChallengeRequest {
        pub_key: public_key.clone(),
        key_type: KeyType::Ecdsa,
    };

    let challenge_res: ChallengeResponse = client
        .post("/v1/auth/challenge")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&challenge_req)
        .await
        .json()
        .await;

    let (signature, _) = signing_key
        .sign_prehash_recoverable(&challenge_res.challenge)
        .expect("sign challenge");

    let verify_req = VerifyChallengeRequest {
        challenge: challenge_res.challenge,
        signature: signature.to_bytes().into(),
        challenge_request: challenge_req,
        expires_at: 0,
        additional_headers: BTreeMap::new(),
    };

    let verify_res: VerifyChallengeResponse = client
        .post("/v1/auth/verify")
        .header(headers::X_SERVICE_ID, service_id.to_string())
        .json(&verify_req)
        .await
        .json()
        .await;

    match verify_res {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        other => panic!("expected API key, got {other:?}"),
    }
}

async fn spawn_tls_listener(
    db_path: &Path,
    db: RocksDb,
    router: Router,
    service_id: ServiceId,
) -> Result<(SocketAddr, oneshot::Sender<()>, JoinHandle<()>), Box<dyn Error + Send + Sync>> {
    let envelope_key =
        init_tls_envelope_key(db_path).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    let envelope = TlsEnvelope::with_key(envelope_key);

    let ca = load_or_create_service_ca(&db, &envelope, service_id)?;
    let server_config = build_tls_server_config(&ca, service_id)?;
    let acceptor = TlsAcceptor::from(Arc::new(server_config));

    let router = router.clone();

    let bind_addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(bind_addr)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    let local_addr = listener
        .local_addr()
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    info!("test tls listener bound to {}", local_addr);

    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();
    let join = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    info!("shutting down test tls listener");
                    break;
                }
                accept = listener.accept() => {
                    match accept {
                        Ok((stream, peer_addr)) => {
                            let acceptor = acceptor.clone();
                            let svc = router.clone();
                            tokio::spawn(async move {
                                match acceptor.accept(stream).await {
                                    Ok(tls_stream) => {
                                        let cert_info = client_cert_info_from_stream(&tls_stream);
                                        let service = service_fn(move |mut req| {
                                            let svc = svc.clone();
                                            let cert_info = cert_info.clone();
                                            async move {
                                                if let Some(cert) = cert_info.clone() {
                                                    req.extensions_mut().insert(cert);
                                                }
                                                let response = svc.clone()
                                                    .oneshot(req)
                                                    .await
                                                    .expect("proxy router should be infallible");
                                                Ok::<_, hyper::Error>(response)
                                            }
                                        });

                                        let builder = AutoH2Builder::new(TokioExecutor::new());
                                        if let Err(err) = builder
                                            .serve_connection(TokioIo::new(tls_stream), service)
                                            .await
                                        {
                                            error!("TLS proxy connection error: {err}");
                                        }
                                    }
                                    Err(err) => {
                                        error!("TLS handshake error from {peer_addr}: {err}");
                                    }
                                }
                            });
                        }
                        Err(err) => {
                            error!("Failed to accept TLS connection: {err}");
                            break;
                        }
                    }
                }
            }
        }
    });

    Ok((local_addr, shutdown_tx, join))
}

fn load_or_create_service_ca(
    db: &RocksDb,
    envelope: &TlsEnvelope,
    service_id: ServiceId,
) -> Result<CertificateAuthority, Box<dyn Error + Send + Sync>> {
    let mut service = ServiceModel::find_by_id(service_id, db)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
        .ok_or_else(|| {
            Box::new(std::io::Error::other("service not found")) as Box<dyn Error + Send + Sync>
        })?;

    let profile = service.tls_profile.as_mut().ok_or_else(|| {
        Box::new(std::io::Error::other("tls profile missing")) as Box<dyn Error + Send + Sync>
    })?;

    if profile.encrypted_client_ca_bundle.is_empty() {
        let ca = CertificateAuthority::new(envelope.clone())
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let mut bundle = ca.ca_certificate_pem();
        if !bundle.ends_with('\n') {
            bundle.push('\n');
        }
        bundle.push_str(&ca.ca_private_key_pem());

        profile.encrypted_client_ca_bundle = envelope
            .encrypt(bundle.as_bytes())
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        service
            .save(service_id, db)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        Ok(ca)
    } else {
        let decrypted = envelope
            .decrypt(&profile.encrypted_client_ca_bundle)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        let pem_str = String::from_utf8(decrypted)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        let (cert_pem, key_pem) = split_ca_bundle(&pem_str)?;
        CertificateAuthority::from_components(&cert_pem, &key_pem, envelope.clone())
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }
}

fn split_ca_bundle(pem_bundle: &str) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let blocks = parse_many(pem_bundle).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let mut cert_block: Option<Pem> = None;
    let mut key_block: Option<Pem> = None;

    for block in blocks {
        match block.tag.as_str() {
            "CERTIFICATE" if cert_block.is_none() => cert_block = Some(block),
            tag if tag.ends_with("PRIVATE KEY") && key_block.is_none() => key_block = Some(block),
            _ => {}
        }
    }

    let cert_pem = cert_block.map(|block| pem::encode(&block)).ok_or_else(|| {
        Box::new(std::io::Error::other("CA certificate missing")) as Box<dyn Error + Send + Sync>
    })?;
    let key_pem = key_block.map(|block| pem::encode(&block)).ok_or_else(|| {
        Box::new(std::io::Error::other("CA private key missing")) as Box<dyn Error + Send + Sync>
    })?;

    Ok((cert_pem, key_pem))
}

fn build_tls_server_config(
    ca: &CertificateAuthority,
    service_id: ServiceId,
) -> Result<rustls::ServerConfig, Box<dyn Error + Send + Sync>> {
    let (server_cert_pem, server_key_pem) = ca
        .generate_server_certificate(service_id, vec!["localhost".to_string()])
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let server_chain = parse_certificates(&server_cert_pem)?;
    let server_key = parse_private_key(&server_key_pem)?;

    let ca_certs = parse_certificates(&ca.ca_certificate_pem())?;
    let mut root_store = RootCertStore::empty();
    for cert in ca_certs {
        root_store
            .add(cert)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
    }

    let client_verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
        .build()
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let mut server_config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(server_chain, server_key)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Ok(server_config)
}

fn parse_certificates(
    pem_str: &str,
) -> Result<Vec<CertificateDer<'static>>, Box<dyn Error + Send + Sync>> {
    let mut reader = Cursor::new(pem_str.as_bytes());
    let mut certs = Vec::new();
    for result in rustls_pemfile::certs(&mut reader) {
        let cert = result.map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        certs.push(cert);
    }

    if certs.is_empty() {
        Err(Box::new(std::io::Error::other("no certificates found")))
    } else {
        Ok(certs)
    }
}

fn parse_private_key(
    pem_str: &str,
) -> Result<PrivateKeyDer<'static>, Box<dyn Error + Send + Sync>> {
    let mut reader = Cursor::new(pem_str.as_bytes());
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader);
    let key = keys
        .next()
        .transpose()
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
        .ok_or_else(|| {
            Box::new(std::io::Error::other("pkcs8 key not found")) as Box<dyn Error + Send + Sync>
        })?;
    Ok(key.into())
}

fn client_cert_info_from_stream(stream: &TlsStream<TcpStream>) -> Option<ClientCertInfo> {
    let (_, connection) = stream.get_ref();
    connection
        .peer_certificates()
        .and_then(|certs| certs.first())
        .map(|cert| ClientCertInfo {
            subject: format!("CN=client-cert-{:x}", crc32_hash(cert.as_ref())),
            issuer: format!("CN=tangle-ca-{:x}", crc32_hash(cert.as_ref())),
            serial: format!("{:x}", crc32_hash(cert.as_ref())),
            not_before: current_unix_timestamp(),
            not_after: current_unix_timestamp() + 365 * 24 * 60 * 60,
        })
}

fn current_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[tokio::test]
async fn admin_can_enable_service_mtls_and_issue_certificates() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let mut harness = MtlsTestHarness::setup().await;

    let profile = harness
        .put_tls_profile(json!({
            "require_client_mtls": true,
            "client_cert_ttl_hours": 24,
            "subject_alt_name_template": "spiffe://tenant-{tenant}/default"
        }))
        .await;

    assert!(
        profile["mtls_listener"].as_str().is_some(),
        "profile response should expose mtls listener"
    );

    let certificate = harness
        .issue_certificate(json!({
            "service_id": harness.service_id.id(),
            "common_name": "tenant-alpha",
            "subject_alt_names": ["localhost", "spiffe://tenant-alpha/service"],
            "ttl_hours": 12
        }))
        .await;

    assert!(
        certificate.certificate_pem.contains("BEGIN CERTIFICATE"),
        "issued cert should be PEM"
    );
    assert!(
        certificate.private_key_pem.contains("BEGIN PRIVATE KEY"),
        "issued private key should be PEM"
    );
    assert!(
        certificate.ca_bundle_pem.contains("BEGIN CERTIFICATE"),
        "response should include CA bundle"
    );

    // Validate certificate metadata fields
    assert!(
        certificate.expires_at > now(),
        "certificate should have future expiration time"
    );
    assert!(
        !certificate.serial.is_empty(),
        "certificate should have a non-empty serial number"
    );
    assert!(
        certificate
            .revocation_url
            .starts_with("/v1/auth/certificates/")
            && certificate.revocation_url.ends_with("/revoke"),
        "certificate should have a valid revocation URL"
    );
}

#[tokio::test]
async fn certificate_ttl_longer_than_policy_is_rejected() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let mut harness = MtlsTestHarness::setup().await;

    let _profile = harness
        .put_tls_profile(json!({
            "require_client_mtls": true,
            "client_cert_ttl_hours": 2
        }))
        .await;

    let response = harness
        .proxy_client
        .post("/v1/auth/certificates")
        .header(headers::X_SERVICE_ID, harness.service_header())
        .header(
            headers::AUTHORIZATION,
            format!("Bearer {}", harness.api_key),
        )
        .json(&json!({
            "service_id": harness.service_id.id(),
            "common_name": "tenant-beta",
            "subject_alt_names": ["localhost"],
            "ttl_hours": 48
        }))
        .await;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "ttl beyond policy should be rejected"
    );
}

#[tokio::test]
async fn plaintext_grpc_requests_are_rejected_when_mtls_required() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let mut harness = MtlsTestHarness::setup().await;

    harness
        .put_tls_profile(json!({
            "require_client_mtls": true,
            "client_cert_ttl_hours": 24
        }))
        .await;

    let mut client = harness.connect_grpc_over_http().await;

    let mut request = Request::new(EchoRequest {
        message: "mtls-required".to_string(),
    });
    let token = format!("Bearer {}", harness.api_key);
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(token.as_str()).unwrap(),
    );
    request.metadata_mut().insert(
        "x-service-id",
        MetadataValue::try_from(harness.service_header().as_str()).unwrap(),
    );

    let result = client.echo(request).await;

    assert!(
        matches!(result, Err(status) if status.code() == Code::Unauthenticated),
        "plaintext request should be rejected with Unauthenticated"
    );
}

#[tokio::test]
async fn grpc_request_with_signed_certificate_succeeds() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let mut harness = MtlsTestHarness::setup().await;

    let profile = harness
        .put_tls_profile(json!({
            "require_client_mtls": true,
            "client_cert_ttl_hours": 24,
            "allowed_dns_names": ["localhost"]
        }))
        .await;

    let listener = profile["mtls_listener"]
        .as_str()
        .expect("mtls listener address present")
        .trim_start_matches("https://")
        .to_string();

    let certificate = harness
        .issue_certificate(json!({
            "service_id": harness.service_id.id(),
            "common_name": "tenant-gamma",
            "subject_alt_names": ["localhost"],
            "ttl_hours": 1
        }))
        .await;

    // Validate certificate metadata fields
    assert!(
        certificate.expires_at > now(),
        "certificate should have future expiration time"
    );
    assert!(
        !certificate.serial.is_empty(),
        "certificate should have a non-empty serial number"
    );
    assert!(
        certificate
            .revocation_url
            .starts_with("/v1/auth/certificates/")
            && certificate.revocation_url.ends_with("/revoke"),
        "certificate should have a valid revocation URL"
    );

    let identity = Identity::from_pem(
        certificate.certificate_pem.clone(),
        certificate.private_key_pem.clone(),
    );
    let ca = Certificate::from_pem(certificate.ca_bundle_pem.clone());

    let mut client = harness
        .connect_grpc_with_tls(&listener, identity, ca)
        .await
        .expect("mTLS client should connect");

    let mut request = Request::new(EchoRequest {
        message: "mutual-tls".to_string(),
    });
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {}", harness.api_key).as_str()).unwrap(),
    );
    request.metadata_mut().insert(
        "x-service-id",
        MetadataValue::try_from(harness.service_header().as_str()).unwrap(),
    );

    let response = client
        .echo(request)
        .await
        .expect("mTLS call should succeed");
    assert_eq!(response.into_inner().message, "mutual-tls");
}

#[tokio::test]
async fn tls_profile_response_includes_ca_certificate() {
    let mut harness = MtlsTestHarness::setup_without_tls_listener().await;

    let profile = harness
        .put_tls_profile(json!({
            "require_client_mtls": true,
            "client_cert_ttl_hours": 24,
            "allowed_dns_names": ["localhost"]
        }))
        .await;

    assert!(
        profile["ca_certificate_pem"].as_str().is_some(),
        "expected TLS profile response to include service CA bundle"
    );
}

#[tokio::test]
async fn enabling_profile_starts_mtls_listener() {
    let mut harness = MtlsTestHarness::setup_without_tls_listener().await;

    let profile = harness
        .put_tls_profile(json!({
            "require_client_mtls": true,
            "client_cert_ttl_hours": 24,
            "allowed_dns_names": ["localhost"]
        }))
        .await;

    let listener = profile["mtls_listener"]
        .as_str()
        .expect("mtls listener address present")
        .trim_start_matches("https://")
        .to_string();

    let certificate = harness
        .issue_certificate(json!({
            "service_id": harness.service_id.id(),
            "common_name": "tenant-delta",
            "subject_alt_names": ["localhost"],
            "ttl_hours": 1
        }))
        .await;

    let identity = Identity::from_pem(
        certificate.certificate_pem.clone(),
        certificate.private_key_pem.clone(),
    );
    let ca = Certificate::from_pem(certificate.ca_bundle_pem.clone());

    let mut client = harness
        .connect_grpc_with_tls(&listener, identity, ca)
        .await
        .expect("mTLS client should connect after enabling profile");

    let mut request = Request::new(EchoRequest {
        message: "plan-specified-flow".to_string(),
    });
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {}", harness.api_key).as_str()).unwrap(),
    );
    request.metadata_mut().insert(
        "x-service-id",
        MetadataValue::try_from(harness.service_header().as_str()).unwrap(),
    );

    let response = client
        .echo(request)
        .await
        .expect("mTLS call should succeed once profile is enabled");
    assert_eq!(response.into_inner().message, "plan-specified-flow");
}
