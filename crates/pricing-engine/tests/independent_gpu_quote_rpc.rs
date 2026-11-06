//! Independent public gRPC policy and EIP-712 domain challenges.
use alloy_dyn_abi::TypedData;
use alloy_primitives::{Address, B256, Signature, U256, keccak256};
use blueprint_crypto::{BytesEncoding, k256::K256SigningKey};
use blueprint_pricing_engine_lib::{
    BenchmarkCache, JobPricingConfig, OperatorConfig, OperatorSigner, PricingEngineService,
    pow::{generate_challenge, generate_proof},
    pricing::TeePricing,
    pricing_engine::{
        GetJobPriceRequest, GetJobPriceResponse, JobQuoteDetails,
        pricing_engine_client::PricingEngineClient, pricing_engine_server::PricingEngineServer,
    },
    signer::QuoteSigningDomain,
};
use rust_decimal::Decimal;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Channel, Server};

struct Peer {
    client: PricingEngineClient<Channel>,
    task: tokio::task::JoinHandle<()>,
    operator: Address,
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Peer {
    async fn start(rate: U256, available: bool) -> Self {
        let config = Arc::new(OperatorConfig {
            gpu_pricing: HashMap::from([("nvidia-b200".into(), rate)]),
            ..Default::default()
        });
        let signer = OperatorSigner::new(
            &config,
            K256SigningKey::from_bytes(&[1; 32]).unwrap(),
            QuoteSigningDomain {
                chain_id: 31337,
                verifying_contract: Address::repeat_byte(4),
            },
        )
        .unwrap();
        let operator = signer.operator_id();
        let service = PricingEngineService::with_job_pricing(
            config,
            Arc::new(BenchmarkCache::new("").unwrap()),
            Arc::new(Mutex::new(HashMap::new())),
            Arc::new(Mutex::new(JobPricingConfig::from([(
                (42, 0),
                U256::from(7),
            )]))),
            Arc::new(Mutex::new(signer)),
        )
        .with_pow_difficulty(1)
        .with_tee_pricing(TeePricing {
            available,
            multiplier: Decimal::ONE,
            provider: "independent-peer".into(),
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            Server::builder()
                .add_service(PricingEngineServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .unwrap();
        });
        let client = PricingEngineClient::connect(format!("http://{address}"))
            .await
            .unwrap();
        Self {
            client,
            task,
            operator,
        }
    }
    async fn request() -> GetJobPriceRequest {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        GetJobPriceRequest {
            service_id: 42,
            job_index: 0,
            proof_of_work: generate_proof(&generate_challenge(42, timestamp), 1)
                .await
                .unwrap(),
            challenge_timestamp: timestamp,
            requester: Address::repeat_byte(2).to_vec(),
            inputs_hash: keccak256(b"independent lease").to_vec(),
            gpu_class: "nvidia-b200".into(),
            gpu_count: 1,
            duration_secs: 1,
            ..Default::default()
        }
    }
}

fn recover(
    response: &GetJobPriceResponse,
    q: &JobQuoteDetails,
    chain: u64,
    contract: Address,
    gpu: bool,
) -> Address {
    let primary = if gpu {
        "GpuJobQuoteDetails"
    } else {
        "JobQuoteDetails"
    };
    let mut fields = json!([
        {"name":"requester","type":"address"},{"name":"serviceId","type":"uint64"},
        {"name":"jobIndex","type":"uint8"},{"name":"price","type":"uint256"},
        {"name":"timestamp","type":"uint64"},{"name":"expiry","type":"uint64"},
        {"name":"confidentiality","type":"uint8"},{"name":"inputsHash","type":"bytes32"}]);
    let mut types = json!({"EIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},
        {"name":"chainId","type":"uint256"},{"name":"verifyingContract","type":"address"}]});
    if gpu {
        fields
            .as_array_mut()
            .unwrap()
            .push(json!({"name":"resourceCommitment","type":"GpuResourceCommitment"}));
        types["GpuResourceCommitment"] = json!([{"name":"kind","type":"uint8"},{"name":"count","type":"uint64"},
            {"name":"gpuClass","type":"bytes32"},{"name":"durationSecs","type":"uint64"}]);
    }
    types[primary] = fields;
    let mut message = json!({"requester":Address::from_slice(&q.requester),"serviceId":q.service_id.to_string(),
        "jobIndex":q.job_index,"price":U256::from_be_slice(&q.price).to_string(),"timestamp":q.timestamp.to_string(),
        "expiry":q.expiry.to_string(),"confidentiality":q.confidentiality,"inputsHash":B256::from_slice(&q.inputs_hash)});
    if gpu {
        message["resourceCommitment"] = json!({"kind":5,"count":q.gpu_count.to_string(),
        "gpuClass":keccak256(q.gpu_class.as_bytes()),"durationSecs":q.duration_secs.to_string()});
    }
    let typed:TypedData=serde_json::from_value(json!({"types":types,"primaryType":primary,
        "domain":{"name":"TangleQuote","version":"1","chainId":chain,"verifyingContract":contract},"message":message})).unwrap();
    Signature::from_raw(&response.signature)
        .unwrap()
        .recover_address_from_prehash(&typed.eip712_signing_hash().unwrap())
        .unwrap()
}

#[tokio::test]
async fn independent_rpc_signature_binds_requester_domain_policy_and_full_price() {
    let mut peer = Peer::start(U256::from(9_007_199_254_740_993u64), false).await;
    let mut request = Peer::request().await;
    request.confidentiality = Some(2);
    let response = peer
        .client
        .get_job_price(request)
        .await
        .unwrap()
        .into_inner();
    let quote = response.quote_details.as_ref().unwrap();
    let contract = Address::repeat_byte(4);
    assert_eq!(
        recover(&response, quote, 31337, contract, true),
        peer.operator
    );
    assert_ne!(
        recover(&response, quote, 31338, contract, true),
        peer.operator
    );
    assert_ne!(
        recover(&response, quote, 31337, Address::repeat_byte(5), true),
        peer.operator
    );
    assert_ne!(
        recover(&response, quote, 31337, contract, false),
        peer.operator
    );
    for mutation in 0..6 {
        let mut altered = quote.clone();
        match mutation {
            0 => altered.requester = Address::repeat_byte(3).to_vec(),
            1 => altered.confidentiality = 3,
            2 => {
                altered.price =
                    (U256::from_be_slice(&altered.price) - U256::from(1)).to_be_bytes_vec()
            }
            3 => altered.service_id += 1,
            4 => altered.inputs_hash = keccak256(b"foreign inputs").to_vec(),
            _ => altered.expiry += 1,
        }
        assert_ne!(
            recover(&response, &altered, 31337, contract, true),
            peer.operator
        );
    }
    let mut legacy = Peer::request().await;
    legacy.gpu_class.clear();
    legacy.gpu_count = 0;
    legacy.duration_secs = 0;
    let response = peer
        .client
        .get_job_price(legacy)
        .await
        .unwrap()
        .into_inner();
    let quote = response.quote_details.as_ref().unwrap();
    assert_eq!(
        recover(&response, quote, 31337, contract, false),
        peer.operator
    );
    assert_ne!(
        recover(&response, quote, 31337, contract, true),
        peer.operator
    );
}

#[tokio::test]
async fn independent_rpc_policy_attestation_flags_never_downgrade_hard_tee() {
    for available in [false, true] {
        let mut peer = Peer::start(U256::from(3), available).await;
        for policy in 0..4 {
            let mut request = Peer::request().await;
            request.confidentiality = Some(policy);
            let result = peer.client.get_job_price(request).await;
            if policy == 1 && !available {
                assert_eq!(result.unwrap_err().code(), tonic::Code::Unavailable);
                continue;
            }
            let response = result.unwrap().into_inner();
            let expected = available && (policy == 1 || policy == 3);
            assert_eq!(response.tee_attested, expected);
            assert_eq!(
                response.tee_provider,
                if expected { "independent-peer" } else { "" }
            );
            let quote = response.quote_details.as_ref().unwrap();
            assert_eq!(quote.confidentiality, policy);
            assert_eq!(
                recover(&response, quote, 31337, Address::repeat_byte(4), true),
                peer.operator
            );
        }
        for policy in [0, 2, 3] {
            let mut request = Peer::request().await;
            request.require_tee = true;
            request.confidentiality = Some(policy);
            assert_eq!(
                peer.client.get_job_price(request).await.unwrap_err().code(),
                tonic::Code::InvalidArgument
            );
        }
    }
}

#[tokio::test]
async fn independent_rpc_uint256_boundary_and_duration_are_exact_or_overflow() {
    let mut peer = Peer::start(U256::MAX - U256::from(7), false).await;
    let request = Peer::request().await;
    let response = peer
        .client
        .get_job_price(request.clone())
        .await
        .unwrap()
        .into_inner();
    let quote = response.quote_details.as_ref().unwrap();
    assert_eq!(quote.price, vec![255; 32]);
    assert_eq!(
        recover(&response, quote, 31337, Address::repeat_byte(4), true),
        peer.operator
    );
    let mut overflow = request;
    overflow.duration_secs = 2;
    assert_eq!(
        peer.client
            .get_job_price(overflow)
            .await
            .unwrap_err()
            .code(),
        tonic::Code::OutOfRange
    );
    let mut peer = Peer::start(U256::from(1), false).await;
    let mut request = Peer::request().await;
    request.duration_secs = u64::MAX;
    request.gpu_count = 8;
    let response = peer
        .client
        .get_job_price(request)
        .await
        .unwrap()
        .into_inner();
    let quote = response.quote_details.as_ref().unwrap();
    assert_eq!(quote.duration_secs, u64::MAX);
    assert_eq!(
        U256::from_be_slice(&quote.price),
        U256::from(u64::MAX) * U256::from(8) + U256::from(7)
    );
    assert_eq!(
        recover(&response, quote, 31337, Address::repeat_byte(4), true),
        peer.operator
    );
}
