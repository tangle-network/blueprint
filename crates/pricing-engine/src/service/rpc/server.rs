use crate::benchmark_cache::BenchmarkCache;
use crate::config::OperatorConfig;
use crate::pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof};
use crate::pricing::calculate_price;
use crate::signer::{OperatorSigner, SignableQuote, SignedQuote as SignerSignedQuote};
use blueprint_core::{error, info, warn};
use blueprint_crypto::BytesEncoding;
use chrono::Utc;
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};

use crate::pricing_engine::{
    AssetSecurityCommitment, GetJobPriceRequest, GetJobPriceResponse, GetPriceRequest,
    GetPriceResponse, JobQuoteDetails as ProtoJobQuoteDetails, QuoteDetails,
    ResourcePricing as ProtoResourcePricing,
    pricing_engine_server::{PricingEngine, PricingEngineServer},
};

/// Per-job pricing configuration: (service_id, job_index) → price in wei
///
/// Operators configure per-job prices either statically (TOML config) or dynamically.
/// If no entry exists for a (service_id, job_index) pair, the RPC returns NOT_FOUND.
pub type JobPricingConfig = std::collections::HashMap<(u64, u32), alloy_primitives::U256>;

/// x402 settlement configuration for cross-chain payment options.
///
/// When set, the RPC server will include settlement options in `GetJobPriceResponse`,
/// allowing clients to pay via x402 on any supported chain/token.
#[derive(Debug, Clone)]
pub struct X402SettlementConfig {
    /// Operator's x402 gateway endpoint URL.
    pub x402_endpoint: String,
    /// Accepted tokens for x402 settlement, with conversion rates.
    pub accepted_tokens: Vec<X402AcceptedToken>,
}

/// An accepted token for x402 settlement.
#[derive(Debug, Clone)]
pub struct X402AcceptedToken {
    /// CAIP-2 network identifier, e.g. `"eip155:8453"` for Base.
    pub network: String,
    /// Token contract/mint address.
    pub asset: String,
    /// Human-readable symbol.
    pub symbol: String,
    /// Token decimals.
    pub decimals: u8,
    /// Operator's receive address on this chain.
    pub pay_to: String,
    /// Exchange rate: token units per native unit (e.g. 3200 USDC per ETH).
    pub rate_per_native_unit: rust_decimal::Decimal,
    /// Markup in basis points.
    pub markup_bps: u16,
}

pub struct PricingEngineService {
    config: Arc<OperatorConfig>,
    benchmark_cache: Arc<BenchmarkCache>,
    pricing_config:
        Arc<Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>>,
    job_pricing_config: Arc<Mutex<JobPricingConfig>>,
    signer: Arc<Mutex<OperatorSigner>>,
    pow_difficulty: u32,
    /// Optional x402 settlement config. When set, `GetJobPriceResponse` includes
    /// cross-chain payment options alongside the signed quote.
    x402_config: Option<X402SettlementConfig>,
}

impl PricingEngineService {
    pub fn new(
        config: Arc<OperatorConfig>,
        benchmark_cache: Arc<BenchmarkCache>,
        pricing_config: Arc<
            Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
        >,
        signer: Arc<Mutex<OperatorSigner>>,
    ) -> Self {
        Self {
            config,
            benchmark_cache,
            pricing_config,
            job_pricing_config: Arc::new(Mutex::new(JobPricingConfig::new())),
            signer,
            pow_difficulty: DEFAULT_POW_DIFFICULTY,
            x402_config: None,
        }
    }

    /// Create with explicit job pricing config
    pub fn with_job_pricing(
        config: Arc<OperatorConfig>,
        benchmark_cache: Arc<BenchmarkCache>,
        pricing_config: Arc<
            Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
        >,
        job_pricing_config: Arc<Mutex<JobPricingConfig>>,
        signer: Arc<Mutex<OperatorSigner>>,
    ) -> Self {
        Self {
            config,
            benchmark_cache,
            pricing_config,
            job_pricing_config,
            signer,
            pow_difficulty: DEFAULT_POW_DIFFICULTY,
            x402_config: None,
        }
    }

    /// Enable x402 settlement options in `GetJobPriceResponse`.
    ///
    /// When configured, every job quote response will include cross-chain
    /// payment options that clients can use to settle via x402.
    pub fn with_x402_settlement(mut self, config: X402SettlementConfig) -> Self {
        self.x402_config = Some(config);
        self
    }
}

#[tonic::async_trait]
impl PricingEngine for PricingEngineService {
    async fn get_price(
        &self,
        request: Request<GetPriceRequest>,
    ) -> Result<Response<GetPriceResponse>, Status> {
        let req = request.into_inner();
        let blueprint_id = req.blueprint_id;
        let ttl_blocks = req.ttl_blocks;
        let proof_of_work = req.proof_of_work;

        info!(
            "Received GetPrice request for blueprint ID: {}",
            blueprint_id
        );

        let current_timestamp = Utc::now().timestamp() as u64;
        let challenge_timestamp = if req.challenge_timestamp > 0 {
            if req.challenge_timestamp < current_timestamp.saturating_sub(30) {
                warn!(
                    "Challenge timestamp is too old: {}",
                    req.challenge_timestamp
                );
                return Err(Status::invalid_argument("Challenge timestamp is too old"));
            }
            if req.challenge_timestamp > current_timestamp + 30 {
                warn!(
                    "Challenge timestamp is too far in the future: {}",
                    req.challenge_timestamp
                );
                return Err(Status::invalid_argument(
                    "Challenge timestamp is too far in the future",
                ));
            }
            req.challenge_timestamp
        } else {
            return Err(Status::invalid_argument(
                "Challenge timestamp is missing or invalid",
            ));
        };

        let challenge = generate_challenge(blueprint_id, challenge_timestamp);
        if !verify_proof(&challenge, &proof_of_work, self.pow_difficulty).map_err(|e| {
            warn!("Failed to verify proof of work: {}", e);
            Status::invalid_argument("Invalid proof of work")
        })? {
            warn!("Invalid proof of work for blueprint ID: {}", blueprint_id);
            return Err(Status::invalid_argument("Invalid proof of work"));
        }

        // Get the benchmark profile from cache
        let benchmark_profile = match self.benchmark_cache.get_profile(blueprint_id) {
            Ok(Some(profile)) => profile,
            _ => {
                warn!(
                    "No benchmark profile found for blueprint ID: {}",
                    blueprint_id
                );
                return Err(Status::not_found(format!(
                    "No benchmark profile found for blueprint ID: {blueprint_id}"
                )));
            }
        };

        let security_requirements = match req.security_requirements {
            Some(requirements) => requirements.clone(),
            None => {
                return Err(Status::invalid_argument("Missing security requirements"));
            }
        };

        // Get the pricing configuration
        let pricing_config = self.pricing_config.lock().await;

        // Calculate the price based on the benchmark profile, pricing config, and TTL
        let price_model = match calculate_price(
            benchmark_profile,
            &pricing_config,
            Some(blueprint_id),
            ttl_blocks,
            Some(security_requirements.clone()),
        ) {
            Ok(model) => model,
            Err(e) => {
                error!(
                    "Failed to calculate price for blueprint ID {}: {:?}",
                    blueprint_id, e
                );
                return Err(Status::internal("Failed to calculate price"));
            }
        };

        // Get the total cost from the price model
        let crate::pricing::PriceModel {
            resources: price_resources,
            total_cost,
            ..
        } = price_model;

        let security_commitment = AssetSecurityCommitment {
            asset: security_requirements.asset.clone(),
            exposure_percent: security_requirements.minimum_exposure_percent,
        };

        // Prepare the response
        let expiry_time = Utc::now().timestamp() as u64 + self.config.quote_validity_duration_secs;
        let timestamp = Utc::now().timestamp() as u64;

        // Convert our internal resource pricing to proto resource pricing
        let proto_resources: Vec<ProtoResourcePricing> = price_resources
            .iter()
            .map(|rp| ProtoResourcePricing {
                kind: format!("{:?}", rp.kind),
                count: rp.count,
                // Convert Decimal to f64 for the proto type
                price_per_unit_rate: rp.price_per_unit_rate.to_f64().unwrap_or(0.0),
            })
            .collect();

        // Create the quote details directly using proto types
        let quote_details = QuoteDetails {
            blueprint_id,
            ttl_blocks,
            // Convert Decimal to f64 for the proto type
            total_cost_rate: total_cost.to_f64().unwrap_or(0.0),
            timestamp,
            expiry: expiry_time,
            resources: proto_resources,
            security_commitments: vec![security_commitment],
        };

        let signable_quote = SignableQuote::new(quote_details, total_cost).map_err(|e| {
            error!(
                "Failed to prepare signable quote for blueprint ID {}: {}",
                blueprint_id, e
            );
            Status::internal("Failed to build signable quote")
        })?;

        // Generate proof of work for the response
        let response_pow = generate_proof(&challenge, self.pow_difficulty)
            .await
            .map_err(|e| {
                error!("Failed to generate proof of work: {}", e);
                Status::internal("Failed to generate proof of work")
            })?;

        // Sign the quote using the hash-based approach
        let signed_quote: SignerSignedQuote = match self
            .signer
            .lock()
            .await
            .sign_quote(signable_quote, response_pow.clone())
        {
            Ok(quote) => quote,
            Err(e) => {
                error!("Failed to sign quote for {}: {}", blueprint_id, e);
                return Err(Status::internal("Failed to sign price quote"));
            }
        };

        // Create the response
        let response = GetPriceResponse {
            quote_details: Some(signed_quote.quote_details.clone()),
            signature: signed_quote.signature.to_bytes().to_vec(),
            operator_id: signed_quote.operator_id.0.to_vec(),
            proof_of_work: signed_quote.proof_of_work,
        };

        info!("Sending signed quote for blueprint ID: {}", blueprint_id);
        Ok(Response::new(response))
    }

    async fn get_job_price(
        &self,
        request: Request<GetJobPriceRequest>,
    ) -> Result<Response<GetJobPriceResponse>, Status> {
        let req = request.into_inner();
        let service_id = req.service_id;
        let job_index = req.job_index;

        info!(
            "Received GetJobPrice request for service {} job index {}",
            service_id, job_index
        );

        // Validate challenge timestamp
        let current_timestamp = Utc::now().timestamp() as u64;
        let challenge_timestamp = if req.challenge_timestamp > 0 {
            if req.challenge_timestamp < current_timestamp.saturating_sub(30) {
                return Err(Status::invalid_argument("Challenge timestamp is too old"));
            }
            if req.challenge_timestamp > current_timestamp + 30 {
                return Err(Status::invalid_argument(
                    "Challenge timestamp is too far in the future",
                ));
            }
            req.challenge_timestamp
        } else {
            return Err(Status::invalid_argument(
                "Challenge timestamp is missing or invalid",
            ));
        };

        // Verify proof of work (use service_id as the challenge seed)
        let challenge = generate_challenge(service_id, challenge_timestamp);
        if !verify_proof(&challenge, &req.proof_of_work, self.pow_difficulty).map_err(|e| {
            warn!("Failed to verify proof of work: {}", e);
            Status::invalid_argument("Invalid proof of work")
        })? {
            return Err(Status::invalid_argument("Invalid proof of work"));
        }

        // Look up per-job price from config
        let job_pricing = self.job_pricing_config.lock().await;
        let price = match job_pricing.get(&(service_id, job_index)) {
            Some(p) => *p,
            None => {
                warn!(
                    "No job pricing configured for service {} job index {}",
                    service_id, job_index
                );
                return Err(Status::not_found(format!(
                    "No pricing configured for service {service_id} job index {job_index}"
                )));
            }
        };
        drop(job_pricing);

        let timestamp = current_timestamp;
        let expiry = timestamp + self.config.quote_validity_duration_secs;

        let proto_details = ProtoJobQuoteDetails {
            service_id,
            job_index,
            price: price.to_be_bytes_vec(),
            timestamp,
            expiry,
        };

        // Generate proof of work for response
        let response_pow = generate_proof(&challenge, self.pow_difficulty)
            .await
            .map_err(|e| {
                error!("Failed to generate proof of work: {}", e);
                Status::internal("Failed to generate proof of work")
            })?;

        // Sign with EIP-712
        let signed = self
            .signer
            .lock()
            .await
            .sign_job_quote(&proto_details, response_pow)
            .map_err(|e| {
                error!(
                    "Failed to sign job quote for service {} job {}: {}",
                    service_id, job_index, e
                );
                Status::internal("Failed to sign job price quote")
            })?;

        // Build x402 settlement options if configured
        let (settlement_options, x402_endpoint) = if let Some(x402) = &self.x402_config {
            let options = compute_settlement_options(&x402.accepted_tokens, price)
                .into_iter()
                .map(|opt| crate::pricing_engine::SettlementOption {
                    network: opt.network,
                    asset: opt.asset,
                    symbol: opt.symbol,
                    amount: opt.amount,
                    pay_to: opt.pay_to,
                    scheme: opt.scheme,
                })
                .collect();
            (options, x402.x402_endpoint.clone())
        } else {
            (vec![], String::new())
        };

        let response = GetJobPriceResponse {
            quote_details: Some(signed.quote_details),
            signature: signed.signature.to_bytes().to_vec(),
            operator_id: signed.operator_id.0.to_vec(),
            proof_of_work: signed.proof_of_work,
            settlement_options,
            x402_endpoint,
        };

        info!(
            "Sending signed job quote for service {} job index {}",
            service_id, job_index
        );
        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signer::QuoteSigningDomain;
    use alloy_primitives::U256;
    use blueprint_crypto::k256::K256SigningKey;
    use blueprint_crypto::{BytesEncoding, KeyType};

    /// Deterministic test key (32 bytes, non-zero)
    const TEST_KEY: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];

    fn test_config() -> Arc<OperatorConfig> {
        Arc::new(OperatorConfig {
            quote_validity_duration_secs: 300,
            ..OperatorConfig::default()
        })
    }

    fn test_signer() -> Arc<Mutex<OperatorSigner>> {
        let keypair = K256SigningKey::from_bytes(&TEST_KEY).unwrap();
        let domain = QuoteSigningDomain {
            chain_id: 1,
            verifying_contract: alloy_primitives::Address::ZERO,
        };
        let signer = OperatorSigner::new(&OperatorConfig::default(), keypair, domain).unwrap();
        Arc::new(Mutex::new(signer))
    }

    fn test_benchmark_cache() -> Arc<BenchmarkCache> {
        Arc::new(BenchmarkCache::new("/tmp/test_bench_cache").unwrap())
    }

    fn test_pricing_config()
    -> Arc<Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>>
    {
        Arc::new(Mutex::new(std::collections::HashMap::new()))
    }

    fn test_job_pricing_config(entries: Vec<((u64, u32), U256)>) -> Arc<Mutex<JobPricingConfig>> {
        let mut map = JobPricingConfig::new();
        for ((sid, idx), price) in entries {
            map.insert((sid, idx), price);
        }
        Arc::new(Mutex::new(map))
    }

    /// Trivial difficulty for test PoW — avoids 30s+ proof generation on slow CI.
    const TEST_POW_DIFFICULTY: u32 = 1;

    fn make_service(job_entries: Vec<((u64, u32), U256)>) -> PricingEngineService {
        let mut svc = PricingEngineService::with_job_pricing(
            test_config(),
            test_benchmark_cache(),
            test_pricing_config(),
            test_job_pricing_config(job_entries),
            test_signer(),
        );
        svc.pow_difficulty = TEST_POW_DIFFICULTY;
        svc
    }

    /// Generate a valid PoW + timestamp for a given service_id.
    async fn valid_pow(service_id: u64) -> (u64, Vec<u8>) {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let challenge = crate::pow::generate_challenge(service_id, timestamp);
        let proof = crate::pow::generate_proof(&challenge, TEST_POW_DIFFICULTY)
            .await
            .unwrap();
        (timestamp, proof)
    }

    // ── Success path ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_job_price_success() {
        let price = U256::from(1_000_000u64); // 1M wei
        let svc = make_service(vec![((42, 0), price)]);
        let (ts, pow) = valid_pow(42).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 42,
            job_index: 0,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let resp = svc.get_job_price(req).await.unwrap().into_inner();
        let details = resp.quote_details.unwrap();
        assert_eq!(details.service_id, 42);
        assert_eq!(details.job_index, 0);
        assert_eq!(U256::from_be_slice(&details.price), price);
        assert!(details.expiry > details.timestamp);
        assert!(!resp.signature.is_empty());
        assert!(!resp.operator_id.is_empty());
        assert!(!resp.proof_of_work.is_empty());
    }

    #[tokio::test]
    async fn test_get_job_price_different_jobs_different_prices() {
        let svc = make_service(vec![
            ((10, 0), U256::from(100u64)),
            ((10, 1), U256::from(500u64)),
            ((10, 2), U256::from(999u64)),
        ]);

        for (idx, expected) in [(0u32, 100u64), (1, 500), (2, 999)] {
            let (ts, pow) = valid_pow(10).await;
            let req = Request::new(GetJobPriceRequest {
                service_id: 10,
                job_index: idx,
                proof_of_work: pow,
                challenge_timestamp: ts,
            });
            let resp = svc.get_job_price(req).await.unwrap().into_inner();
            let details = resp.quote_details.unwrap();
            assert_eq!(
                U256::from_be_slice(&details.price),
                U256::from(expected),
                "job index {idx} should have price {expected}"
            );
        }
    }

    #[tokio::test]
    async fn test_get_job_price_large_price() {
        // Near-max U256 value
        let price = U256::MAX / U256::from(2);
        let svc = make_service(vec![((1, 0), price)]);
        let (ts, pow) = valid_pow(1).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let resp = svc.get_job_price(req).await.unwrap().into_inner();
        let details = resp.quote_details.unwrap();
        assert_eq!(U256::from_be_slice(&details.price), price);
    }

    // ── Missing job pricing ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_job_price_not_found() {
        let svc = make_service(vec![]); // No pricing configured
        let (ts, pow) = valid_pow(42).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 42,
            job_index: 0,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::NotFound);
        assert!(err.message().contains("No pricing configured"));
    }

    #[tokio::test]
    async fn test_get_job_price_wrong_job_index() {
        // Pricing exists for job_index 0 but not 1
        let svc = make_service(vec![((42, 0), U256::from(100u64))]);
        let (ts, pow) = valid_pow(42).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 42,
            job_index: 1,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::NotFound);
    }

    #[tokio::test]
    async fn test_get_job_price_wrong_service_id() {
        let svc = make_service(vec![((42, 0), U256::from(100u64))]);
        let (ts, pow) = valid_pow(99).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 99,
            job_index: 0,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::NotFound);
    }

    // ── Timestamp validation ────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_job_price_missing_timestamp() {
        let svc = make_service(vec![((1, 0), U256::from(1u64))]);

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: vec![],
            challenge_timestamp: 0, // 0 = missing
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
        assert!(err.message().contains("missing"));
    }

    #[tokio::test]
    async fn test_get_job_price_timestamp_too_old() {
        let svc = make_service(vec![((1, 0), U256::from(1u64))]);
        let old_ts = chrono::Utc::now().timestamp() as u64 - 60; // 60s ago

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: vec![],
            challenge_timestamp: old_ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
        assert!(err.message().contains("too old"));
    }

    #[tokio::test]
    async fn test_get_job_price_timestamp_too_far_in_future() {
        let svc = make_service(vec![((1, 0), U256::from(1u64))]);
        let future_ts = chrono::Utc::now().timestamp() as u64 + 60; // 60s from now

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: vec![],
            challenge_timestamp: future_ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
        assert!(err.message().contains("future"));
    }

    // ── Invalid proof of work ───────────────────────────────────────────

    #[tokio::test]
    async fn test_get_job_price_invalid_pow() {
        let svc = make_service(vec![((1, 0), U256::from(1u64))]);
        let ts = chrono::Utc::now().timestamp() as u64;

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: vec![0u8; 32], // garbage PoW
            challenge_timestamp: ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
        assert!(err.message().contains("proof of work"));
    }

    #[tokio::test]
    async fn test_get_job_price_empty_pow() {
        let svc = make_service(vec![((1, 0), U256::from(1u64))]);
        let ts = chrono::Utc::now().timestamp() as u64;

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: vec![], // empty PoW
            challenge_timestamp: ts,
        });

        let err = svc.get_job_price(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    // ── Quote expiry validation ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_job_price_expiry_uses_config() {
        let mut config = OperatorConfig::default();
        config.quote_validity_duration_secs = 600; // 10 minutes

        let mut svc = PricingEngineService::with_job_pricing(
            Arc::new(config),
            test_benchmark_cache(),
            test_pricing_config(),
            test_job_pricing_config(vec![((1, 0), U256::from(100u64))]),
            test_signer(),
        );
        svc.pow_difficulty = TEST_POW_DIFFICULTY;
        let (ts, pow) = valid_pow(1).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 1,
            job_index: 0,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let resp = svc.get_job_price(req).await.unwrap().into_inner();
        let details = resp.quote_details.unwrap();
        // Expiry should be ~600s after timestamp
        let duration = details.expiry - details.timestamp;
        assert!(
            (590..=610).contains(&duration),
            "expected ~600s validity, got {duration}s"
        );
    }

    // ── Signature is valid ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_job_price_signature_verifies() {
        let keypair = K256SigningKey::from_bytes(&TEST_KEY).unwrap();
        let domain = QuoteSigningDomain {
            chain_id: 1,
            verifying_contract: alloy_primitives::Address::ZERO,
        };
        let verifying_key = keypair.verifying_key();

        let svc = make_service(vec![((42, 0), U256::from(500u64))]);
        let (ts, pow) = valid_pow(42).await;

        let req = Request::new(GetJobPriceRequest {
            service_id: 42,
            job_index: 0,
            proof_of_work: pow,
            challenge_timestamp: ts,
        });

        let resp = svc.get_job_price(req).await.unwrap().into_inner();
        let details = resp.quote_details.unwrap();

        // Reconstruct the digest and verify the signature
        let digest = crate::signer::job_quote_digest_eip712(&details, domain);
        let sig = blueprint_crypto::k256::K256Signature::from_bytes(&resp.signature).unwrap();
        assert!(
            blueprint_crypto::k256::K256Ecdsa::verify(&verifying_key, &digest, &sig),
            "signature should verify with the operator's key"
        );
    }
}

/// Internal settlement option (pre-proto conversion).
struct SettlementOptionInternal {
    network: String,
    asset: String,
    symbol: String,
    amount: String,
    pay_to: String,
    scheme: String,
}

/// Convert a wei price into settlement options for each accepted token.
fn compute_settlement_options(
    accepted_tokens: &[X402AcceptedToken],
    price_wei: alloy_primitives::U256,
) -> Vec<SettlementOptionInternal> {
    accepted_tokens
        .iter()
        .filter_map(|token| {
            // Convert wei → native units → token units
            let wei_decimal = rust_decimal::Decimal::from_str_exact(&price_wei.to_string()).ok()?;
            let native_unit = rust_decimal::Decimal::from(10u64.pow(18));
            let native_amount = wei_decimal / native_unit;
            let token_amount = native_amount * token.rate_per_native_unit;
            let markup = rust_decimal::Decimal::ONE
                + rust_decimal::Decimal::from(token.markup_bps)
                    / rust_decimal::Decimal::from(10_000u32);
            let final_amount = token_amount * markup;
            let token_unit = rust_decimal::Decimal::from(10u64.pow(u32::from(token.decimals)));
            let smallest_units = (final_amount * token_unit).floor().to_string();

            Some(SettlementOptionInternal {
                network: token.network.clone(),
                asset: token.asset.clone(),
                symbol: token.symbol.clone(),
                amount: smallest_units,
                pay_to: token.pay_to.clone(),
                scheme: "exact".into(),
            })
        })
        .collect()
}

// Function to run the server (called from main.rs)
pub async fn run_rpc_server(
    config: Arc<OperatorConfig>,
    benchmark_cache: Arc<BenchmarkCache>,
    pricing_config: Arc<
        Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
    >,
    job_pricing_config: Option<Arc<Mutex<JobPricingConfig>>>,
    signer: Arc<Mutex<OperatorSigner>>,
) -> anyhow::Result<()> {
    let addr = format!("{}:{}", config.rpc_bind_address, config.rpc_port).parse()?;
    info!("gRPC server listening on {}", addr);

    let pricing_service = match job_pricing_config {
        Some(jpc) => PricingEngineService::with_job_pricing(
            config,
            benchmark_cache,
            pricing_config,
            jpc,
            signer,
        ),
        None => PricingEngineService::new(config, benchmark_cache, pricing_config, signer),
    };
    let server = PricingEngineServer::new(pricing_service);

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
