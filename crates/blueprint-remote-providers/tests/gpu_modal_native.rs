//! Native HTTP/2 protocol proof through public SDK exports; no GPU resources.
use blueprint_remote_providers::infra::CloudProviderAdapter;
use blueprint_remote_providers::providers::modal::{ModalAdapter, ModalRateKeys};
use blueprint_remote_providers::{CloudProvider, CloudProvisioner, Error, ResourceSpec};
use prost::bytes::{Buf, BufMut};
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use tonic::{
    Request, Response, Status,
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    codegen::{BoxFuture, Service, http},
    server::NamedService,
};

fn varint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    while value >= 128 {
        bytes.push(value as u8 | 128);
        value >>= 7;
    }
    bytes.push(value as u8);
    bytes
}
fn read_varint(bytes: &mut &[u8]) -> u64 {
    let mut value = 0;
    for shift in (0..64).step_by(7) {
        let byte = bytes[0];
        *bytes = &bytes[1..];
        value |= u64::from(byte & 127) << shift;
        if byte < 128 {
            return value;
        }
    }
    panic!("invalid boundary protobuf varint")
}
fn message(tag: u32, body: &[u8]) -> Vec<u8> {
    [
        varint(u64::from(tag) * 8 + 2),
        varint(body.len() as u64),
        body.to_vec(),
    ]
    .concat()
}
fn text(tag: u32, value: &str) -> Vec<u8> {
    message(tag, value.as_bytes())
}
fn integer(tag: u32, value: u64) -> Vec<u8> {
    [varint(u64::from(tag) * 8), varint(value)].concat()
}
fn double(tag: u32, value: f64) -> Vec<u8> {
    [varint(u64::from(tag) * 8 + 1), value.to_le_bytes().to_vec()].concat()
}
fn fields(mut bytes: &[u8], wanted: u32) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    while !bytes.is_empty() {
        let key = read_varint(&mut bytes);
        let field = match key & 7 {
            0 => varint(read_varint(&mut bytes)),
            1 => {
                let value = bytes[..8].to_vec();
                bytes = &bytes[8..];
                value
            }
            2 => {
                let size = read_varint(&mut bytes) as usize;
                let value = bytes[..size].to_vec();
                bytes = &bytes[size..];
                value
            }
            5 => {
                let value = bytes[..4].to_vec();
                bytes = &bytes[4..];
                value
            }
            _ => panic!("unexpected boundary protobuf field"),
        };
        if key >> 3 == u64::from(wanted) {
            result.push(field);
        }
    }
    result
}
fn first(bytes: &[u8], tag: u32) -> Vec<u8> {
    fields(bytes, tag).into_iter().next().unwrap_or_default()
}
fn string(bytes: &[u8], tag: u32) -> String {
    String::from_utf8(first(bytes, tag)).unwrap()
}
fn number(bytes: &[u8], tag: u32) -> u64 {
    let bytes = first(bytes, tag);
    if bytes.is_empty() {
        0
    } else {
        read_varint(&mut bytes.as_slice())
    }
}
fn mapping(bytes: &[u8], tag: u32) -> HashMap<String, String> {
    fields(bytes, tag)
        .into_iter()
        .map(|entry| (string(&entry, 1), string(&entry, 2)))
        .collect()
}

#[derive(Clone, Default)]
struct RawCodec;
struct RawEncoder;
struct RawDecoder;
impl Encoder for RawEncoder {
    type Item = Vec<u8>;
    type Error = Status;
    fn encode(&mut self, item: Self::Item, output: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        output.put_slice(&item);
        Ok(())
    }
}
impl Decoder for RawDecoder {
    type Item = Vec<u8>;
    type Error = Status;
    fn decode(&mut self, input: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(input.copy_to_bytes(input.remaining()).to_vec()))
    }
}
impl Codec for RawCodec {
    type Encode = Vec<u8>;
    type Decode = Vec<u8>;
    type Encoder = RawEncoder;
    type Decoder = RawDecoder;
    fn encoder(&mut self) -> Self::Encoder {
        RawEncoder
    }
    fn decoder(&mut self) -> Self::Decoder {
        RawDecoder
    }
}

#[derive(Default)]
struct State {
    calls: Vec<String>,
    tags: Vec<Vec<u8>>,
    create: Option<Vec<u8>>,
    secret: Option<Vec<u8>>,
    alive: bool,
    unknown_rate: bool,
    deprecated: bool,
    deny_auth: bool,
}
#[derive(Clone)]
struct Peer(Arc<Mutex<State>>);
struct Unary(Peer, String);
struct ImageStream(Peer);
impl NamedService for Peer {
    const NAME: &'static str = "modal.client.ModalClient";
}
impl Service<http::Request<tonic::body::Body>> for Peer {
    type Response = http::Response<tonic::body::Body>;
    type Error = Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: http::Request<tonic::body::Body>) -> Self::Future {
        assert_eq!(request.version(), http::Version::HTTP_2);
        assert_eq!(request.headers()["x-modal-token-id"], "test-token-id");
        assert_eq!(
            request.headers()["x-modal-token-secret"],
            "test-token-secret"
        );
        let path = request.uri().path().to_string();
        if !path.ends_with("/AuthTokenGet") {
            assert_eq!(
                request.headers()["x-modal-auth-token"],
                "e30.eyJleHAiOjQxMDI0NDQ4MDB9.signature"
            );
        }
        let peer = self.clone();
        Box::pin(async move {
            let mut grpc = tonic::server::Grpc::new(RawCodec);
            if path.ends_with("/ImageJoinStreaming") {
                Ok(grpc.server_streaming(ImageStream(peer), request).await)
            } else {
                Ok(grpc.unary(Unary(peer, path), request).await)
            }
        })
    }
}
impl tonic::server::ServerStreamingService<Vec<u8>> for ImageStream {
    type Response = Vec<u8>;
    type ResponseStream = tokio_stream::Iter<std::vec::IntoIter<Result<Vec<u8>, Status>>>;
    type Future = BoxFuture<Response<Self::ResponseStream>, Status>;
    fn call(&mut self, _: Request<Vec<u8>>) -> Self::Future {
        self.0
            .0
            .lock()
            .unwrap()
            .calls
            .push("ImageJoinStreaming".into());
        Box::pin(async {
            Ok(Response::new(tokio_stream::iter(vec![Ok(message(
                1,
                &integer(1, 1),
            ))])))
        })
    }
}
impl tonic::server::UnaryService<Vec<u8>> for Unary {
    type Response = Vec<u8>;
    type Future = BoxFuture<Response<Self::Response>, Status>;
    fn call(&mut self, request: Request<Vec<u8>>) -> Self::Future {
        let mut state = self.0.0.lock().unwrap();
        let method = self.1.rsplit('/').next().unwrap();
        state.calls.push(method.into());
        let request = request.into_inner();
        let response = match method {
            "AuthTokenGet" if state.deny_auth => Err(Status::unauthenticated("test-token-secret")),
            "AuthTokenGet" => Ok(text(1, "e30.eyJleHAiOjQxMDI0NDQ4MDB9.signature")),
            "WorkspaceBillingRates" => {
                let mut values = vec![
                    message(
                        1,
                        &[text(1, "account.cpu-core-hour"), text(2, "0.1")].concat(),
                    ),
                    message(
                        1,
                        &[text(1, "account.ram-gib-hour"), text(2, "0.01")].concat(),
                    ),
                ];
                if !state.unknown_rate {
                    values.push(message(
                        1,
                        &[text(1, "account.B200-device-hour"), text(2, "2")].concat(),
                    ));
                }
                if state.deprecated {
                    values.push(message(
                        3,
                        &[text(1, "account.B200-device-hour"), text(2, "deprecated")].concat(),
                    ));
                }
                Ok(values.concat())
            }
            "AppGetOrCreate" => {
                assert_eq!(string(&request, 1), "tangle-gpu-leases");
                Ok(text(1, "ap-native"))
            }
            "EnvironmentGetOrCreate" => Ok(message(
                2,
                &message(2, &text(1, "provider-builder-version")),
            )),
            "ImageGetOrCreate" => {
                assert_eq!(string(&request, 4), "ap-native");
                assert_eq!(string(&first(&request, 2), 6), "FROM worker:explicit");
                assert_eq!(string(&request, 9), "provider-builder-version");
                Ok(text(1, "im-native"))
            }
            "SecretGetOrCreate" => {
                assert_eq!(number(&request, 4), 5, "workload env must be ephemeral");
                assert_eq!(
                    mapping(&request, 5)["TANGLE_GPU_WORKER_TOKEN"],
                    "test-worker-secret"
                );
                state.secret = Some(request);
                Ok(text(1, "st-ephemeral"))
            }
            "SandboxCreate" => {
                assert!(state.create.is_none(), "duplicate paid create");
                let definition = first(&request, 2);
                let resources = first(&definition, 5);
                let gpu = first(&resources, 4);
                assert_eq!(number(&gpu, 2), 2);
                assert_eq!(string(&gpu, 4), "B200");
                assert_eq!(number(&resources, 2), 16 * 1024);
                assert_eq!(number(&resources, 5), 16 * 1024);
                assert_eq!(number(&resources, 3), 4000);
                assert_eq!(number(&resources, 7), 4000);
                assert_eq!(number(&definition, 7), 300);
                assert_eq!(string(&definition, 28), "gvisor");
                assert_eq!(
                    fields(&definition, 1),
                    vec![b"/worker".to_vec(), b"serve".to_vec()]
                );
                assert_eq!(string(&definition, 3), "im-native");
                assert_eq!(string(&definition, 4), "st-ephemeral");
                assert_eq!(string(&definition, 39), "203.0.113.8/32");
                let port = first(&first(&definition, 20), 1);
                assert_eq!(number(&port, 1), 8787);
                assert_eq!(number(&port, 2), 0);
                state.tags = fields(&request, 4);
                state.create = Some(request);
                state.alive = true;
                Ok(text(1, "sb-native"))
            }
            "SandboxList" => {
                if !state.alive || !first(&request, 2).is_empty() {
                    Ok(Vec::new())
                } else {
                    let gpu = [integer(2, 2), text(4, "B200")].concat();
                    let task =
                        [double(2, 1793923200.0), text(6, "B200"), message(9, &gpu)].concat();
                    let tunnel =
                        [text(1, "worker.example"), integer(2, 443), integer(5, 8787)].concat();
                    let mut info = vec![
                        text(1, "sb-native"),
                        double(3, 1793923200.0),
                        message(4, &task),
                        integer(11, 300),
                        double(13, 1793923200.0),
                        message(15, &tunnel),
                    ];
                    info.extend(state.tags.iter().map(|tag| message(6, tag)));
                    Ok(message(1, &info.concat()))
                }
            }
            "SandboxWait" => Ok(if state.alive {
                Vec::new()
            } else {
                message(1, &integer(1, 3))
            }),
            "SandboxTerminate" => {
                state.alive = false;
                Ok(Vec::new())
            }
            "SandboxGetTunnels" => Ok(message(
                2,
                &[text(1, "worker.example"), integer(2, 443), integer(5, 8787)].concat(),
            )),
            _ => panic!("unexpected provider RPC {method}"),
        };
        Box::pin(async move { response.map(Response::new) })
    }
}

async fn setup() -> (
    Arc<Mutex<State>>,
    CloudProvisioner,
    tokio::task::JoinHandle<()>,
) {
    let state = Arc::new(Mutex::new(State::default()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}", listener.local_addr().unwrap());
    let peer = Peer(state.clone());
    let server = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(peer)
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
            .unwrap();
    });
    let adapter = ModalAdapter::with_endpoint(
        "test-token-id".into(),
        "test-token-secret".into(),
        &endpoint,
    )
    .unwrap()
    .with_rate_keys(ModalRateKeys {
        cpu_per_core_hour: "account.cpu-core-hour".into(),
        memory_per_gib_hour: "account.ram-gib-hour".into(),
        gpu_per_device_hour: HashMap::from([("B200".into(), "account.B200-device-hour".into())]),
    });
    let providers = HashMap::from([(
        CloudProvider::Modal,
        Box::new(adapter) as Box<dyn CloudProviderAdapter>,
    )]);
    (state, CloudProvisioner::with_providers(providers), server)
}
fn spec() -> ResourceSpec {
    ResourceSpec {
        provider: Some(CloudProvider::Modal),
        cpu: 4.0,
        memory_gb: 16.0,
        gpu_class: Some("nvidia-b200".into()),
        gpu_count: Some(2),
        gpu_vram_gb: Some(180),
        allow_spot: true,
        image: Some("worker:explicit".into()),
        command: Some(vec!["/worker".into(), "serve".into()]),
        ports: vec![8787],
        endpoint_port: Some(8787),
        source_network: Some("203.0.113.8/32".into()),
        env: std::collections::BTreeMap::from([(
            "TANGLE_GPU_WORKER_TOKEN".into(),
            "test-worker-secret".into(),
        )]),
        lease_duration_secs: Some(300),
        max_hourly_cost_usd: Some(5.0),
        ownership: Some(blueprint_core::resources::ManagedLeaseIdentity {
            namespace: "modal-http2-proof".into(),
            lease_id: uuid::Uuid::new_v4().to_string(),
        }),
        ..ResourceSpec::default()
    }
}

#[tokio::test]
async fn native_modal_quote_create_observe_adopt_and_retire_are_real_http2() {
    let (state, cloud, server) = setup().await;
    let spec = spec();
    let quotes = cloud.quotes(&spec).await.unwrap();
    assert_eq!(quotes.len(), 1);
    assert!((quotes[0].selection.estimated_hourly_cost.unwrap() - 4.56).abs() < 1e-9);
    assert_eq!(quotes[0].pricing_source, "live");
    assert_eq!(
        quotes[0].selection.gpu_class.as_deref(),
        Some("nvidia-b200")
    );
    let allocation = cloud.create_managed(&spec).await.unwrap();
    assert_eq!(allocation.instance.id, "sb-native");
    let adopted = cloud.create_managed(&spec).await.unwrap();
    assert_eq!(adopted.instance.id, "sb-native");
    let observed = cloud
        .status_managed(
            &CloudProvider::Modal,
            spec.ownership.as_ref().unwrap(),
            "sb-native",
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(observed.endpoint.as_deref(), Some("https://worker.example"));
    assert_eq!(observed.expiry_source.as_deref(), Some("derived"));
    assert_eq!(observed.expires_at.unwrap().timestamp(), 1793923500);
    assert!(observed.hourly_cost_usd.is_none());
    let mut changed = spec.clone();
    changed.env.insert(
        "TANGLE_GPU_WORKER_TOKEN".into(),
        "different-workload".into(),
    );
    assert!(cloud.create_managed(&changed).await.is_err());
    cloud
        .destroy_managed(&CloudProvider::Modal, spec.ownership.as_ref().unwrap())
        .await
        .unwrap();
    assert!(
        cloud
            .list_managed("modal-http2-proof")
            .await
            .unwrap()
            .is_empty()
    );
    let state = state.lock().unwrap();
    assert!(!state.alive);
    assert_eq!(
        state
            .calls
            .iter()
            .filter(|method| method.as_str() == "SandboxCreate")
            .count(),
        1
    );
    assert!(
        state
            .calls
            .iter()
            .any(|method| method == "ImageJoinStreaming")
    );
    server.abort();
}

#[tokio::test]
async fn unsupported_shapes_unknown_or_deprecated_rates_and_credentials_fail_closed() {
    let (state, cloud, server) = setup().await;
    let mut invalid = spec();
    invalid.command = None;
    assert!(cloud.quotes(&invalid).await.unwrap().is_empty());
    assert!(state.lock().unwrap().calls.is_empty());
    invalid = spec();
    invalid.allow_spot = false;
    assert!(cloud.quotes(&invalid).await.unwrap().is_empty());
    state.lock().unwrap().unknown_rate = true;
    assert!(cloud.quotes(&spec()).await.unwrap().is_empty());
    assert!(cloud.create_managed(&spec()).await.is_err());
    state.lock().unwrap().unknown_rate = false;
    state.lock().unwrap().deprecated = true;
    assert!(cloud.quotes(&spec()).await.unwrap().is_empty());
    assert!(state.lock().unwrap().create.is_none());
    server.abort();
    let (state, cloud, server) = setup().await;
    state.lock().unwrap().deny_auth = true;
    assert!(matches!(
        cloud.quotes(&spec()).await,
        Err(Error::AuthRejected)
    ));
    assert!(state.lock().unwrap().create.is_none());
    server.abort();
}
