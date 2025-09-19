//! gRPC proxy integration tests (red stage for TDD)

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::pin::Pin;

use futures_util::stream::{self, Stream};
use tempfile::tempdir;
use tokio::{sync::oneshot, task::JoinHandle};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};

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

struct GrpcProxyTestContext {
    _tmp_dir: tempfile::TempDir,
    proxy_addr: String,
    api_key: String,
    service_id: ServiceId,
    shutdown_tx: oneshot::Sender<()>,
    backend_handle: JoinHandle<()>,
}

async fn spawn_echo_backend() -> (SocketAddr, oneshot::Sender<()>, JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let addr = listener.local_addr().expect("read local addr");
    let incoming = TcpListenerStream::new(listener);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let service = EchoServiceServer::new(TestEchoService::default());
    let handle = tokio::spawn(async move {
        let shutdown = async {
            let _ = shutdown_rx.await;
        };

        if let Err(err) = Server::builder()
            .add_service(service)
            .serve_with_incoming_shutdown(incoming, shutdown)
            .await
        {
            eprintln!("test backend server error: {err}");
        }
    });

    (addr, shutdown_tx, handle)
}

async fn setup_grpc_proxy_context() -> GrpcProxyTestContext {
    let mut rng = blueprint_std::BlueprintRng::new();
    let tmp_dir = tempdir().expect("tempdir");

    let (backend_addr, shutdown_tx, backend_handle) = spawn_echo_backend().await;

    let proxy = AuthenticatedProxy::new(tmp_dir.path()).expect("proxy init");
    let db = proxy.db();

    let service_id = ServiceId::new(4242);
    let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
    let public_key = signing_key.verifying_key().to_sec1_bytes();

    let mut service = ServiceModel {
        api_key_prefix: "grpc_".to_string(),
        owners: vec![],
        upstream_url: format!("http://{}", backend_addr),
    };
    service.add_owner(KeyType::Ecdsa, public_key.to_vec());
    service.save(service_id, &db).expect("save service");

    let router = proxy.router();
    let client = TestClient::new(router);
    let proxy_addr = format!("http://127.0.0.1:{}", client.server_port());

    let challenge_req = ChallengeRequest {
        pub_key: public_key.to_vec(),
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

    let api_key = match verify_res {
        VerifyChallengeResponse::Verified { api_key, .. } => api_key,
        other => panic!("expected verified response, got {other:?}"),
    };

    GrpcProxyTestContext {
        _tmp_dir: tmp_dir,
        proxy_addr,
        api_key,
        service_id,
        shutdown_tx,
        backend_handle,
    }
}

fn apply_auth_metadata<T>(request: &mut Request<T>, ctx: &GrpcProxyTestContext) {
    let token = format!("Bearer {}", ctx.api_key);
    let auth_metadata = MetadataValue::try_from(token.as_str()).expect("valid auth metadata");
    request
        .metadata_mut()
        .insert("authorization", auth_metadata);

    let service_header = ctx.service_id.to_string();
    let service_metadata =
        MetadataValue::try_from(service_header.as_str()).expect("valid service metadata");
    request
        .metadata_mut()
        .insert("x-service-id", service_metadata);
}

async fn connect_grpc_client(ctx: &GrpcProxyTestContext) -> EchoServiceClient<Channel> {
    let channel = Channel::from_shared(ctx.proxy_addr.clone())
        .expect("valid proxy endpoint")
        .connect()
        .await
        .expect("connect proxy channel");
    EchoServiceClient::new(channel)
}

#[tokio::test]
async fn grpc_unary_proxy_round_trip_is_forwarded() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let ctx = setup_grpc_proxy_context().await;
    let mut client = connect_grpc_client(&ctx).await;

    let mut request = Request::new(EchoRequest {
        message: "ping".to_string(),
    });
    apply_auth_metadata(&mut request, &ctx);

    let response = client
        .echo(request)
        .await
        .expect("proxy should forward unary call");
    assert_eq!(response.into_inner().message, "ping");

    // Shutdown backend server to avoid leak.
    let GrpcProxyTestContext {
        shutdown_tx,
        backend_handle,
        ..
    } = ctx;
    shutdown_tx.send(()).ok();
    backend_handle.await.ok();
}

#[tokio::test]
async fn grpc_streaming_proxy_round_trip_is_forwarded() {
    let _guard = tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_line_number(true)
            .with_file(true)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .with_test_writer()
            .finish(),
    );

    let ctx = setup_grpc_proxy_context().await;
    let mut client = connect_grpc_client(&ctx).await;

    let outbound = stream::iter([
        EchoRequest {
            message: "one".to_string(),
        },
        EchoRequest {
            message: "two".to_string(),
        },
    ]);

    let mut request = Request::new(outbound);
    apply_auth_metadata(&mut request, &ctx);

    let mut response_stream = client
        .echo_stream(request)
        .await
        .expect("proxy should forward streaming call")
        .into_inner();

    let mut received = Vec::new();
    while let Some(message) = response_stream.message().await.expect("stream result") {
        received.push(message.message);
    }

    assert_eq!(received, vec!["one", "two"]);

    let GrpcProxyTestContext {
        shutdown_tx,
        backend_handle,
        ..
    } = ctx;
    shutdown_tx.send(()).ok();
    backend_handle.await.ok();
}
