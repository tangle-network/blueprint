//! Red-stage tests describing desired mTLS proxy workflow.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::Duration;

use rustls::crypto::CryptoProvider;

use axum::http::StatusCode;
use futures_util::stream::{self, Stream};
use k256::ecdsa::SigningKey;
use serde::Deserialize;
use serde_json::{Value, json};
use tempfile::tempdir;
use tokio::{sync::oneshot, task::JoinHandle};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity, Server};
use tonic::{Code, Request, Response, Status};

use crate::models::ServiceModel;
use crate::proxy::AuthenticatedProxy;
use crate::test_client::TestClient;
use crate::types::{
    ChallengeRequest, ChallengeResponse, KeyType, ServiceId, VerifyChallengeRequest,
    VerifyChallengeResponse, headers,
};

pub mod proto {
    tonic::include_proto!("blueprint.auth.grpcproxytest");
}

use proto::{
    EchoRequest, EchoResponse,
    echo_service_client::EchoServiceClient,
    echo_service_server::{EchoService, EchoServiceServer},
};

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
    proxy_client: TestClient,
    proxy_addr: String,
    service_id: ServiceId,
    api_key: String,
    backend: Option<BackendHandle>,
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
        // Note: Crypto provider should be installed by the test runner
        
        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp_dir = tempdir().expect("tempdir");

        let (backend_addr, backend) = spawn_backend().await;

        let proxy = AuthenticatedProxy::new(tmp_dir.path()).expect("proxy init");
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
        let proxy_client = TestClient::new(router);
        let proxy_addr = format!("http://127.0.0.1:{}", proxy_client.server_port());

        let api_key =
            issue_api_key(&proxy_client, service_id, signing_key, public_key.to_vec()).await;

        MtlsTestHarness {
            _tmp_dir: tmp_dir,
            proxy_client,
            proxy_addr,
            service_id,
            api_key,
            backend: Some(backend),
        }
    }

    fn service_header(&self) -> String {
        self.service_id.to_string()
    }

    async fn put_tls_profile(&self, body: Value) -> Value {
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

    async fn issue_certificate(&self, body: Value) -> CertificateResponse {
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

        response.json().await
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

        let endpoint = Endpoint::from_shared(format!("https://{listener}"))?
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

    let harness = MtlsTestHarness::setup().await;

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

    let harness = MtlsTestHarness::setup().await;

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

    let harness = MtlsTestHarness::setup().await;

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

    let harness = MtlsTestHarness::setup().await;

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
        .to_string();

    let certificate = harness
        .issue_certificate(json!({
            "service_id": harness.service_id.id(),
            "common_name": "tenant-gamma",
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
