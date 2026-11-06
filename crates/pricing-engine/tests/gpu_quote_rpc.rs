//! Future protobuf fields exercise the actual public gRPC server on the unchanged base.
use alloy_dyn_abi::TypedData;
use alloy_primitives::{Address, B256, Signature, U256, keccak256};
use blueprint_crypto::{BytesEncoding, k256::K256SigningKey};
use blueprint_pricing_engine_lib::{
    BenchmarkCache, BenchmarkProfile, JobPricingConfig, OperatorConfig, OperatorSigner,
    PricingEngineService, ResourcePricing,
    pow::{generate_challenge, generate_proof},
    pricing_engine::{self, pricing_engine_server::PricingEngineServer},
    signer::QuoteSigningDomain,
    types::ResourceUnit,
};
use prost::Message;
use rust_decimal::Decimal;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{
    Request,
    transport::{Channel, Server},
};

#[derive(Clone, PartialEq, Message)]
struct JobRequest {
    #[prost(uint64, tag = "1")]
    service_id: u64,
    #[prost(uint32, tag = "2")]
    job_index: u32,
    #[prost(bytes = "vec", tag = "3")]
    proof_of_work: Vec<u8>,
    #[prost(uint64, tag = "4")]
    challenge_timestamp: u64,
    #[prost(bool, tag = "5")]
    require_tee: bool,
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
    #[prost(uint32, optional, tag = "11")]
    confidentiality: Option<u32>,
}

#[derive(Clone, PartialEq, Message)]
struct JobResponse {
    #[prost(message, optional, tag = "1")]
    quote: Option<JobQuote>,
    #[prost(bytes = "vec", tag = "2")]
    signature: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct JobQuote {
    #[prost(uint64, tag = "1")]
    service_id: u64,
    #[prost(uint32, tag = "2")]
    job_index: u32,
    #[prost(uint64, tag = "4")]
    timestamp: u64,
    #[prost(uint64, tag = "5")]
    expiry: u64,
    #[prost(bytes = "vec", tag = "7")]
    requester: Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    inputs_hash: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    price: Vec<u8>,
    #[prost(uint32, tag = "6")]
    confidentiality: u32,
    #[prost(string, tag = "9")]
    gpu_class: String,
    #[prost(uint64, tag = "10")]
    gpu_count: u64,
    #[prost(uint64, tag = "11")]
    duration_secs: u64,
}

#[derive(Clone, PartialEq, Message)]
struct ServiceResponse {
    #[prost(message, optional, tag = "1")]
    quote: Option<ServiceQuote>,
}
#[derive(Clone, PartialEq, Message)]
struct ServiceQuote {
    #[prost(message, repeated, tag = "6")]
    resources: Vec<IntegerResource>,
}
#[derive(Clone, PartialEq, Message)]
struct IntegerResource {
    #[prost(string, tag = "1")]
    kind: String,
    #[prost(bytes = "vec", tag = "4")]
    price_per_unit: Vec<u8>,
}

struct Rpc {
    client: tonic::client::Grpc<Channel>,
    server: tokio::task::JoinHandle<()>,
    operator: Address,
}
impl Drop for Rpc {
    fn drop(&mut self) {
        self.server.abort();
    }
}

impl Rpc {
    async fn start() -> Self {
        Self::with_rates(Decimal::new(9_007_199_254_740_993, 18), "3").await
    }

    async fn with_rates(resource_rate: Decimal, gpu_rate: &str) -> Self {
        Self::configured(resource_rate, gpu_rate, "").await
    }

    async fn configured(resource_rate: Decimal, gpu_rate: &str, tee: &str) -> Self {
        let file = tempfile::NamedTempFile::new().unwrap();
        let text = format!(
            "{}\n[gpu_pricing]\nnvidia-b200 = \"{gpu_rate}\"\n{tee}",
            toml::to_string(&OperatorConfig::default()).unwrap()
        );
        std::fs::write(file.path(), &text).unwrap();
        let config =
            Arc::new(blueprint_pricing_engine_lib::load_config_from_path(file.path()).unwrap());
        let key = K256SigningKey::from_bytes(&[1; 32]).unwrap();
        let signer = OperatorSigner::new(
            &config,
            key,
            QuoteSigningDomain {
                chain_id: 31_337,
                verifying_contract: Address::repeat_byte(4),
            },
        )
        .unwrap();
        let operator = signer.operator_id();
        let cache = Arc::new(BenchmarkCache::new("").unwrap());
        let profile: BenchmarkProfile = serde_json::from_value(json!({
            "job_id":"42","execution_mode":"native","duration_secs":1,
            "timestamp":chrono::Utc::now().timestamp(),"success":true,
            "cpu_details": {
                "num_cores_detected":4,"avg_cores_used":2.0,"avg_usage_percent":50.0,
                "peak_cores_used":2.0,"peak_usage_percent":75.0,"benchmark_duration_ms":10,
                "primes_found":128,"max_prime":1024,"primes_per_second":64.0,
                "cpu_model":"recorded-test-profile","cpu_frequency_mhz":3200.0
            },
            "io_details":null,"memory_details":null,"network_details":null,
            "gpu_details":null,"storage_details":{"storage_available_gb":1.0}
        }))
        .unwrap();
        cache.store_profile(42, &profile).unwrap();
        let resources = HashMap::from([(
            None,
            vec![
                ResourcePricing {
                    kind: ResourceUnit::CPU,
                    count: 2,
                    price_per_unit_rate: resource_rate,
                },
                ResourcePricing {
                    kind: ResourceUnit::StorageMB,
                    count: 1024,
                    price_per_unit_rate: Decimal::new(1, 6),
                },
            ],
        )]);
        let service = PricingEngineService::with_job_pricing(
            config,
            cache,
            Arc::new(Mutex::new(resources)),
            Arc::new(Mutex::new(JobPricingConfig::from([(
                (42, 0),
                U256::from(7),
            )]))),
            Arc::new(Mutex::new(signer)),
        )
        .with_pow_difficulty(1)
        .with_tee_pricing(blueprint_pricing_engine_lib::load_tee_pricing_from_toml(&text).unwrap());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(PricingEngineServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .unwrap();
        });
        let channel = tonic::transport::Endpoint::from_shared(format!("http://{address}"))
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            client: tonic::client::Grpc::new(channel),
            server,
            operator,
        }
    }

    async fn request() -> JobRequest {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        JobRequest {
            service_id: 42,
            job_index: 0,
            proof_of_work: generate_proof(&generate_challenge(42, timestamp), 1)
                .await
                .unwrap(),
            challenge_timestamp: timestamp,
            requester: Address::repeat_byte(2).to_vec(),
            inputs_hash: keccak256(b"lease").to_vec(),
            ..Default::default()
        }
    }

    async fn job(&mut self, request: JobRequest) -> Result<JobResponse, tonic::Status> {
        self.client.ready().await.unwrap();
        self.client
            .unary(
                Request::new(request),
                tonic::codegen::http::uri::PathAndQuery::from_static(
                    "/pricing_engine.PricingEngine/GetJobPrice",
                ),
                tonic::codec::ProstCodec::default(),
            )
            .await
            .map(tonic::Response::into_inner)
    }
    async fn service(&mut self) -> Result<ServiceResponse, tonic::Status> {
        let job = Self::request().await;
        let request = pricing_engine::GetPriceRequest {
            blueprint_id: 42,
            ttl_blocks: 1,
            proof_of_work: job.proof_of_work,
            challenge_timestamp: job.challenge_timestamp,
            requester: job.requester,
            security_requirements: Some(pricing_engine::AssetSecurityRequirements {
                asset: Some(pricing_engine::Asset {
                    asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![8; 20])),
                }),
                minimum_exposure_percent: 1,
                maximum_exposure_percent: 100,
            }),
            ..Default::default()
        };
        self.client.ready().await.unwrap();
        let response: tonic::Response<ServiceResponse> = self
            .client
            .unary(
                Request::new(request),
                tonic::codegen::http::uri::PathAndQuery::from_static(
                    "/pricing_engine.PricingEngine/GetPrice",
                ),
                tonic::codec::ProstCodec::default(),
            )
            .await?;
        Ok(response.into_inner())
    }
}

#[tokio::test]
async fn gpu_allocation_crosses_the_public_rpc_without_becoming_a_cpu_quote() {
    let mut rpc = Rpc::start().await;
    let mut request = Rpc::request().await;
    request.gpu_class = "nvidia-b200".into();
    request.gpu_count = 8;
    request.duration_secs = 3600;
    let response = rpc.job(request).await.unwrap();
    let quote = response.quote.clone().unwrap();
    assert_eq!(
        quote.gpu_class, "nvidia-b200",
        "the RPC discarded the allocation class"
    );
    assert_eq!((quote.gpu_count, quote.duration_secs), (8, 3600));
    assert_eq!(
        U256::from_be_slice(&quote.price),
        U256::from(7 + 3 * 8 * 3600)
    );
    assert_eq!(quote.price.len(), 32);
    assert_eq!(response.signature.len(), 65);
    assert_eq!(recover(&response, &quote), rpc.operator);
    use blueprint_tangle_extra::job_quote::{
        GpuResourceCommitment, JobQuoteDetails, JobQuoteSigner, QuoteSigningDomain as JobDomain,
    };
    let mut sdk = JobQuoteSigner::new(
        K256SigningKey::from_bytes(&[1; 32]).unwrap(),
        JobDomain {
            chain_id: 31_337,
            verifying_contract: Address::repeat_byte(4),
        },
    )
    .unwrap();
    let signed = sdk
        .sign_with_resources(
            &JobQuoteDetails {
                requester: Address::from_slice(&quote.requester),
                service_id: quote.service_id,
                job_index: u8::try_from(quote.job_index).unwrap(),
                price: U256::from_be_slice(&quote.price),
                timestamp: quote.timestamp,
                expiry: quote.expiry,
                confidentiality: u8::try_from(quote.confidentiality).unwrap(),
                inputs_hash: B256::from_slice(&quote.inputs_hash),
            },
            GpuResourceCommitment::new(&quote.gpu_class, quote.gpu_count, quote.duration_secs)
                .unwrap(),
        )
        .unwrap();
    let mut sdk_response = response.clone();
    sdk_response.signature = signed.quote.signature.to_bytes();
    sdk_response.signature.push(27 + signed.quote.recovery_id);
    assert_eq!(recover(&sdk_response, &quote), rpc.operator);

    for field in 0..4 {
        let mut altered = quote.clone();
        match field {
            0 => altered.gpu_class = "nvidia-b300".into(),
            1 => altered.gpu_count = 1,
            2 => altered.duration_secs += 1,
            _ => altered.inputs_hash = keccak256(b"other inputs").to_vec(),
        }
        assert_ne!(recover(&response, &altered), rpc.operator);
    }
}

#[tokio::test]
async fn resource_prices_cross_the_public_rpc_as_exact_usd18_integers() {
    let mut rpc = Rpc::start().await;
    let response = rpc.service().await.unwrap();
    let quote = response.quote.unwrap();
    let cpu = quote
        .resources
        .iter()
        .find(|resource| resource.kind == "CPU")
        .unwrap();
    assert_eq!(
        cpu.price_per_unit.len(),
        32,
        "USD18 uses a canonical uint256 encoding"
    );
    assert_eq!(
        U256::from_be_slice(&cpu.price_per_unit),
        U256::from(9_007_199_254_740_993u64),
        "a floating wire price cannot preserve the requested integer"
    );
}

#[tokio::test]
async fn confidentiality_uses_the_same_four_policies_as_the_public_chain() {
    let mut rpc = Rpc::start().await;
    for policy in [2, 3] {
        let mut request = Rpc::request().await;
        request.confidentiality = Some(policy);
        let response = rpc.job(request).await.unwrap();
        let quote = response.quote.as_ref().unwrap();
        assert_eq!(recover(&response, quote), rpc.operator);
        assert_eq!(
            quote.confidentiality, policy,
            "the request policy was replaced by a boolean"
        );
    }
    for policy in [0, 2, 3] {
        let mut request = Rpc::request().await;
        request.require_tee = true;
        request.confidentiality = Some(policy);
        assert_eq!(
            rpc.job(request).await.unwrap_err().code(),
            tonic::Code::InvalidArgument,
            "an optional policy cannot downgrade a hard TEE request"
        );
    }
}

#[tokio::test]
async fn malformed_legacy_rpc_parameters_cannot_produce_a_quote() {
    let mut rpc = Rpc::start().await;
    let valid = Rpc::request().await;
    let response = rpc.job(valid.clone()).await.unwrap();
    assert_eq!(
        recover(&response, response.quote.as_ref().unwrap()),
        rpc.operator
    );
    assert_eq!(
        U256::from_be_slice(&response.quote.unwrap().price),
        U256::from(7)
    );
    for malformed in 0..4 {
        let mut request = valid.clone();
        match malformed {
            0 => request.requester = vec![0; 20],
            1 => request.inputs_hash.clear(),
            2 => request.challenge_timestamp = 0,
            _ => request.proof_of_work = vec![0; 7],
        }
        assert_eq!(
            rpc.job(request).await.unwrap_err().code(),
            tonic::Code::InvalidArgument
        );
    }
    let mut unknown_job = valid;
    unknown_job.job_index = 1;
    assert_eq!(
        rpc.job(unknown_job).await.unwrap_err().code(),
        tonic::Code::NotFound
    );
}

fn recover(response: &JobResponse, q: &JobQuote) -> Address {
    let mut fields = json!([
        {"name":"requester","type":"address"},{"name":"serviceId","type":"uint64"},
        {"name":"jobIndex","type":"uint8"},{"name":"price","type":"uint256"},
        {"name":"timestamp","type":"uint64"},{"name":"expiry","type":"uint64"},
        {"name":"confidentiality","type":"uint8"},{"name":"inputsHash","type":"bytes32"}
    ]);
    let gpu = !q.gpu_class.is_empty();
    let primary = if gpu {
        "GpuJobQuoteDetails"
    } else {
        "JobQuoteDetails"
    };
    if gpu {
        fields
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"resourceCommitment","type":"GpuResourceCommitment"}));
    }
    let mut types = json!({"EIP712Domain":[
        {"name":"name","type":"string"},{"name":"version","type":"string"},
        {"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}
    ]});
    types[primary] = fields;
    if gpu {
        types["GpuResourceCommitment"] = json!([
            {"name":"kind","type":"uint8"},{"name":"count","type":"uint64"},
            {"name":"gpuClass","type":"bytes32"},{"name":"durationSecs","type":"uint64"}
        ]);
    }
    let mut message = json!({
        "requester":Address::from_slice(&q.requester),"serviceId":q.service_id.to_string(),
        "jobIndex":q.job_index,"price":U256::from_be_slice(&q.price).to_string(),
        "timestamp":q.timestamp.to_string(),"expiry":q.expiry.to_string(),
        "confidentiality":q.confidentiality,"inputsHash":B256::from_slice(&q.inputs_hash)
    });
    if gpu {
        message["resourceCommitment"] = json!({
            "kind":5,"count":q.gpu_count.to_string(),
            "gpuClass":keccak256(q.gpu_class.as_bytes()),"durationSecs":q.duration_secs.to_string()
        });
    }
    let typed: TypedData = serde_json::from_value(json!({
        "types":types,"primaryType":primary,
        "domain":{"name":"TangleQuote","version":"1","chainId":31_337,"verifyingContract":Address::repeat_byte(4)},
        "message":message
    })).unwrap();
    Signature::from_raw(&response.signature)
        .unwrap()
        .recover_address_from_prehash(&typed.eip712_signing_hash().unwrap())
        .unwrap()
}

#[tokio::test]
async fn resource_integer_boundaries_are_exact_or_refused_without_rounding() {
    for (rate, expected) in [
        (Decimal::ZERO, Some(U256::ZERO)),
        (Decimal::new(10, 19), Some(U256::from(1))),
        (Decimal::new(1, 19), None),
        (Decimal::new(-1, 18), None),
    ] {
        let mut rpc = Rpc::with_rates(rate, "3").await;
        let response = rpc.service().await;
        match expected {
            Some(expected) => {
                let quote = response.unwrap().quote.unwrap();
                let price = &quote
                    .resources
                    .iter()
                    .find(|r| r.kind == "CPU")
                    .unwrap()
                    .price_per_unit;
                assert_eq!(price.len(), 32);
                assert_eq!(U256::from_be_slice(price), expected);
            }
            None => assert_eq!(
                response.unwrap_err().code(),
                tonic::Code::FailedPrecondition
            ),
        }
    }
}

#[tokio::test]
async fn gpu_quote_price_overflow_and_unpriced_or_invalid_allocations_are_refused() {
    let mut rpc = Rpc::start().await;
    for (class, count, duration, expected) in [
        ("unknown-gpu", 8, 3600, tonic::Code::InvalidArgument),
        ("nvidia-h200", 1, 3600, tonic::Code::NotFound),
        ("nvidia-b200", 9, 3600, tonic::Code::InvalidArgument),
        ("nvidia-b200", 0, 3600, tonic::Code::InvalidArgument),
        ("nvidia-b200", 8, 0, tonic::Code::InvalidArgument),
        ("", 8, 3600, tonic::Code::InvalidArgument),
    ] {
        let mut request = Rpc::request().await;
        request.gpu_class = class.into();
        request.gpu_count = count;
        request.duration_secs = duration;
        assert_eq!(rpc.job(request).await.unwrap_err().code(), expected);
    }
    let mut rpc = Rpc::with_rates(Decimal::new(1, 6), &U256::MAX.to_string()).await;
    let mut request = Rpc::request().await;
    request.gpu_class = "nvidia-b200".into();
    request.gpu_count = 1;
    request.duration_secs = 1;
    assert_eq!(
        rpc.job(request).await.unwrap_err().code(),
        tonic::Code::OutOfRange
    );
}

#[tokio::test]
async fn configured_tee_policy_keeps_standard_execution_and_full_uint256_prices_exact() {
    let tee = "\n[tee]\navailable=true\nprovider=\"configured-operator\"\nmultiplier=1.5\n";
    let mut rpc = Rpc::configured(Decimal::new(1, 6), "3", tee).await;
    for (policy, price) in [(0, 7u64), (1, 10), (2, 7), (3, 10)] {
        let mut request = Rpc::request().await;
        request.confidentiality = Some(policy);
        let response = rpc.job(request).await.unwrap();
        let quote = response.quote.as_ref().unwrap();
        assert_eq!(quote.confidentiality, policy);
        assert_eq!(U256::from_be_slice(&quote.price), U256::from(price));
        assert_eq!(recover(&response, quote), rpc.operator);
    }
    let rate = (U256::MAX - U256::from(7)).to_string();
    let exact = "\n[tee]\navailable=true\nprovider=\"configured-operator\"\nmultiplier=1.0\n";
    let mut rpc = Rpc::configured(Decimal::new(1, 6), &rate, exact).await;
    let mut request = Rpc::request().await;
    request.confidentiality = Some(1);
    request.gpu_class = "nvidia-b200".into();
    request.gpu_count = 1;
    request.duration_secs = 1;
    let response = rpc.job(request.clone()).await.unwrap();
    let quote = response.quote.as_ref().unwrap();
    assert_eq!(quote.price.len(), 32);
    assert_eq!(U256::from_be_slice(&quote.price), U256::MAX);
    assert_eq!(recover(&response, quote), rpc.operator);
    let mut rpc = Rpc::configured(Decimal::new(1, 6), &rate, tee).await;
    assert_eq!(
        rpc.job(request).await.unwrap_err().code(),
        tonic::Code::OutOfRange
    );
}
