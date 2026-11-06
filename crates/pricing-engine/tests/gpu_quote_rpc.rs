use alloy_primitives::{Address, U256, keccak256};
use alloy_sol_types::SolValue;
use blueprint_crypto::{BytesEncoding, k256::K256SigningKey};
use blueprint_pricing_engine_lib::{
    BenchmarkCache, JobPricingConfig, OperatorConfig, OperatorSigner, PricingEngineService,
    pow::{generate_challenge, generate_proof},
    pricing_engine::pricing_engine_server::PricingEngineServer,
    signer::QuoteSigningDomain,
};
use prost::Message;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, oneshot};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Code, transport::Server};

#[allow(dead_code)]
mod utils;

#[derive(Clone, PartialEq, Message)]
struct Request {
    #[prost(uint64, tag = "1")]
    service_id: u64,
    #[prost(uint32, tag = "2")]
    job_index: u32,
    #[prost(bytes = "vec", tag = "3")]
    proof_of_work: Vec<u8>,
    #[prost(uint64, tag = "4")]
    challenge_timestamp: u64,
    #[prost(bytes = "vec", tag = "6")]
    requester: Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    inputs_hash: Vec<u8>,
    #[prost(string, tag = "8")]
    gpu_class: String,
    #[prost(uint64, tag = "9")]
    gpu_count: u64,
    #[prost(uint64, tag = "10")]
    duration_secs: u64,
}
#[derive(Clone, PartialEq, Message)]
struct Response {
    #[prost(message, optional, tag = "1")]
    quote: Option<Quote>,
    #[prost(bytes = "vec", tag = "2")]
    signature: Vec<u8>,
}
#[derive(Clone, PartialEq, Message)]
struct Quote {
    #[prost(uint64, tag = "1")]
    service_id: u64,
    #[prost(uint32, tag = "2")]
    job_index: u32,
    #[prost(bytes = "vec", tag = "3")]
    price: Vec<u8>,
    #[prost(uint64, tag = "4")]
    timestamp: u64,
    #[prost(uint64, tag = "5")]
    expiry: u64,
    #[prost(uint32, tag = "6")]
    confidentiality: u32,
    #[prost(bytes = "vec", tag = "7")]
    requester: Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    inputs_hash: Vec<u8>,
    #[prost(string, tag = "9")]
    gpu_class: String,
    #[prost(uint64, tag = "10")]
    gpu_count: u64,
    #[prost(uint64, tag = "11")]
    duration_secs: u64,
}

#[tokio::test]
async fn gpu_quote_crosses_grpc_with_exact_allocation_and_unknown_classes_fail_closed() {
    let mut config = OperatorConfig::default();
    config.gpu_pricing = HashMap::from([
        ("nvidia-b200".into(), U256::from(3)),
        ("nvidia-b300".into(), U256::from(5)),
    ]);
    let config_file = tempfile::NamedTempFile::new().unwrap();
    let rates = config.gpu_pricing.clone();
    for (class, rate) in [("unknown-gpu", U256::from(3)), ("nvidia-b200", U256::ZERO)] {
        config.gpu_pricing.insert(class.into(), rate);
        std::fs::write(config_file.path(), toml::to_string(&config).unwrap()).unwrap();
        assert!(blueprint_pricing_engine_lib::load_config_from_path(config_file.path()).is_err());
        config.gpu_pricing = rates.clone();
    }
    std::fs::write(config_file.path(), toml::to_string(&config).unwrap()).unwrap();
    let config =
        Arc::new(blueprint_pricing_engine_lib::load_config_from_path(config_file.path()).unwrap());
    let key = K256SigningKey::from_bytes(&[1; 32]).unwrap();
    let verifying_key = key.verifying_key();
    let domain = QuoteSigningDomain {
        chain_id: 31337,
        verifying_contract: Address::repeat_byte(4),
    };
    let signer = OperatorSigner::new(&config, key, domain).unwrap();
    let benchmark_cache = Arc::new(BenchmarkCache::new("").unwrap());
    benchmark_cache
        .store_profile(42, &utils::sample_benchmark_profile(42))
        .unwrap();
    let exact_rate = 9_007_199_254_740_993u64;
    let pricing = HashMap::from([(
        None,
        vec![blueprint_pricing_engine_lib::ResourcePricing {
            kind: blueprint_pricing_engine_lib::types::ResourceUnit::CPU,
            count: 2,
            price_per_unit_rate: rust_decimal::Decimal::new(exact_rate as i64, 9),
        }],
    )]);
    let service = PricingEngineService::with_job_pricing(
        config,
        benchmark_cache,
        Arc::new(Mutex::new(pricing)),
        Arc::new(Mutex::new(JobPricingConfig::from([(
            (42, 0),
            U256::from(7),
        )]))),
        Arc::new(Mutex::new(signer)),
    )
    .with_pow_difficulty(1);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (stop_tx, stop_rx) = oneshot::channel();
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(PricingEngineServer::new(service))
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                let _ = stop_rx.await;
            })
            .await
            .unwrap();
    });
    let channel = tonic::transport::Endpoint::from_shared(format!("http://{addr}"))
        .unwrap()
        .connect()
        .await
        .unwrap();
    let mut client = tonic::client::Grpc::new(channel);
    let timestamp = chrono::Utc::now().timestamp() as u64;
    let mut request = Request {
        service_id: 42,
        job_index: 0,
        proof_of_work: generate_proof(&generate_challenge(42, timestamp), 1)
            .await
            .unwrap(),
        challenge_timestamp: timestamp,
        requester: Address::repeat_byte(2).to_vec(),
        inputs_hash: keccak256(b"lease").to_vec(),
        gpu_class: "nvidia-b200".into(),
        gpu_count: 8,
        duration_secs: 3600,
    };
    client.ready().await.unwrap();
    let response: Response = client
        .unary(
            tonic::Request::new(request.clone()),
            tonic::codegen::http::uri::PathAndQuery::from_static(
                "/pricing_engine.PricingEngine/GetJobPrice",
            ),
            tonic::codec::ProstCodec::default(),
        )
        .await
        .unwrap()
        .into_inner();
    let quote = response.quote.unwrap();
    assert_eq!(
        quote.gpu_class, "nvidia-b200",
        "GPU class must survive the public RPC"
    );
    assert_eq!((quote.gpu_count, quote.duration_secs), (8, 3600));
    assert_eq!(
        U256::from_be_slice(&quote.price),
        U256::from(7 + 3 * 8 * 3600)
    );
    use k256::ecdsa::signature::hazmat::PrehashVerifier;
    let signature = k256::ecdsa::Signature::from_slice(&response.signature[..64]).unwrap();
    verifying_key
        .0
        .verify_prehash(&gpu_digest(&quote, domain), &signature)
        .unwrap();
    let mut altered = quote.clone();
    altered.gpu_class = "nvidia-b300".into();
    assert!(
        verifying_key
            .0
            .verify_prehash(&gpu_digest(&altered, domain), &signature)
            .is_err()
    );
    for (class, count, duration, code) in [
        ("unknown-gpu", 8, 3600, Code::InvalidArgument),
        ("nvidia-h200", 1, 3600, Code::NotFound),
        ("nvidia-b200", 9, 3600, Code::InvalidArgument),
        ("nvidia-b200", 0, 3600, Code::InvalidArgument),
        ("nvidia-b200", 8, 0, Code::InvalidArgument),
        ("", 8, 3600, Code::InvalidArgument),
    ] {
        request.gpu_class = class.into();
        request.gpu_count = count;
        request.duration_secs = duration;
        client.ready().await.unwrap();
        let result: Result<tonic::Response<Response>, _> = client
            .unary(
                tonic::Request::new(request.clone()),
                tonic::codegen::http::uri::PathAndQuery::from_static(
                    "/pricing_engine.PricingEngine/GetJobPrice",
                ),
                tonic::codec::ProstCodec::default(),
            )
            .await;
        assert_eq!(
            result.unwrap_err().code(),
            code,
            "{class}, {count}, {duration}"
        );
    }
    request.gpu_class = "nvidia-b300".into();
    request.gpu_count = 8;
    request.duration_secs = 3600;
    client.ready().await.unwrap();
    let response: Response = client
        .unary(
            tonic::Request::new(request.clone()),
            tonic::codegen::http::uri::PathAndQuery::from_static(
                "/pricing_engine.PricingEngine/GetJobPrice",
            ),
            tonic::codec::ProstCodec::default(),
        )
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        U256::from_be_slice(&response.quote.unwrap().price),
        U256::from(7 + 5 * 8 * 3600)
    );

    request.gpu_class.clear();
    request.gpu_count = 0;
    request.duration_secs = 0;
    client.ready().await.unwrap();
    let response: Response = client
        .unary(
            tonic::Request::new(request.clone()),
            tonic::codegen::http::uri::PathAndQuery::from_static(
                "/pricing_engine.PricingEngine/GetJobPrice",
            ),
            tonic::codec::ProstCodec::default(),
        )
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        U256::from_be_slice(&response.quote.unwrap().price),
        U256::from(7)
    );

    let service_request = blueprint_pricing_engine_lib::pricing_engine::GetPriceRequest {
        blueprint_id: 42,
        ttl_blocks: 1,
        proof_of_work: request.proof_of_work,
        challenge_timestamp: timestamp,
        requester: request.requester,
        security_requirements: Some(
            blueprint_pricing_engine_lib::pricing_engine::AssetSecurityRequirements {
                asset: Some(blueprint_pricing_engine_lib::pricing_engine::Asset {
                    asset_type: Some(
                        blueprint_pricing_engine_lib::pricing_engine::asset::AssetType::Erc20(
                            vec![0; 20],
                        ),
                    ),
                }),
                minimum_exposure_percent: 0,
                maximum_exposure_percent: 100,
            },
        ),
        ..Default::default()
    };
    client.ready().await.unwrap();
    let response: blueprint_pricing_engine_lib::pricing_engine::GetPriceResponse = client
        .unary(
            tonic::Request::new(service_request),
            tonic::codegen::http::uri::PathAndQuery::from_static(
                "/pricing_engine.PricingEngine/GetPrice",
            ),
            tonic::codec::ProstCodec::default(),
        )
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        U256::from_be_slice(&response.quote_details.unwrap().resources[0].price_per_unit),
        U256::from(exact_rate)
    );
    stop_tx.send(()).unwrap();
    server.await.unwrap();
}

fn gpu_digest(q: &Quote, domain: QuoteSigningDomain) -> [u8; 32] {
    let resource = keccak256(
        (
            keccak256(
                "ResourceCommitment(uint8 kind,uint64 count,bytes32 gpuClass,uint64 durationSecs)",
            ),
            U256::from(5),
            U256::from(q.gpu_count),
            keccak256(q.gpu_class.as_bytes()),
            U256::from(q.duration_secs),
        )
            .abi_encode(),
    );
    let structure = keccak256((
        keccak256("GpuJobQuoteDetails(address requester,uint64 serviceId,uint8 jobIndex,uint256 price,uint64 timestamp,uint64 expiry,uint8 confidentiality,bytes32 inputsHash,ResourceCommitment resourceCommitment)ResourceCommitment(uint8 kind,uint64 count,bytes32 gpuClass,uint64 durationSecs)"),
        Address::from_slice(&q.requester), U256::from(q.service_id), U256::from(q.job_index), U256::from_be_slice(&q.price),
        U256::from(q.timestamp), U256::from(q.expiry), U256::from(q.confidentiality), alloy_primitives::B256::from_slice(&q.inputs_hash), resource,
    ).abi_encode());
    let domain_hash = keccak256((
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
        keccak256("TangleQuote"), keccak256("1"), U256::from(domain.chain_id), domain.verifying_contract,
    ).abi_encode());
    keccak256(
        [
            b"\x19\x01".as_slice(),
            domain_hash.as_slice(),
            structure.as_slice(),
        ]
        .concat(),
    )
    .into()
}
