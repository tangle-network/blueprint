use crate::benchmark_cache::BenchmarkCache;
use crate::config::OperatorConfig;
use crate::pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof};
use crate::pricing::{
    SubscriptionPricing, calculate_event_price, calculate_price_with_curve,
    calculate_subscription_price,
};
use crate::signer::{OperatorSigner, SignableQuote, SignedQuote as SignerSignedQuote};
use blueprint_core::{error, info, warn};
use blueprint_crypto::BytesEncoding;
use chrono::Utc;
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};
use tower_http::cors::{Any, CorsLayer};

use crate::pricing_engine::{
    AssetSecurityCommitment, GetJobPriceRequest, GetJobPriceResponse, GetPriceRequest,
    GetPriceResponse, JobQuoteDetails as ProtoJobQuoteDetails, PricingModelHint, QuoteDetails,
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
///
/// Note: `X402AcceptedToken` mirrors `blueprint_x402::config::AcceptedToken`.
/// A cyclic dependency (`runner -> qos -> remote-providers -> pricing-engine`)
/// prevents importing from `blueprint-x402` directly. If you change the
/// conversion logic here, update `AcceptedToken::convert_wei_to_amount` in
/// `crates/x402/src/config.rs` as well.
#[derive(Debug, Clone)]
pub struct X402SettlementConfig {
    /// Operator's x402 gateway endpoint URL.
    pub x402_endpoint: String,
    /// Accepted tokens for x402 settlement, with conversion rates.
    pub accepted_tokens: Vec<X402AcceptedToken>,
}

/// An accepted token for x402 settlement.
///
/// Mirrors `blueprint_x402::config::AcceptedToken`. See [`X402SettlementConfig`]
/// for why this is duplicated.
#[derive(Debug, Clone)]
pub struct X402AcceptedToken {
    /// CAIP-2 network identifier, e.g. `"eip155:8453"` for Base.
    pub network: String,
    /// Token contract address on the EVM chain.
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

/// Subscription pricing config: `Option<blueprint_id>` → `SubscriptionPricing`.
pub type SubscriptionPricingConfig = std::collections::HashMap<Option<u64>, SubscriptionPricing>;

/// Maximum allowed clock drift between client and server for PoW challenge timestamps.
const CHALLENGE_TIMESTAMP_TOLERANCE_SECS: u64 = 30;

pub struct PricingEngineService {
    config: Arc<OperatorConfig>,
    benchmark_cache: Arc<BenchmarkCache>,
    pricing_config:
        Arc<Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>>,
    job_pricing_config: Arc<Mutex<JobPricingConfig>>,
    subscription_config: Arc<Mutex<SubscriptionPricingConfig>>,
    signer: Arc<Mutex<OperatorSigner>>,
    pow_difficulty: u32,
    /// Optional x402 settlement config. When set, `GetJobPriceResponse` includes
    /// cross-chain payment options alongside the signed quote.
    x402_config: Option<X402SettlementConfig>,
    /// TEE pricing configuration. Controls availability, multiplier, and provider name.
    tee_config: crate::pricing::TeePricing,
    /// Optional TTL pricing curve for non-linear duration-based pricing.
    ttl_curve: Option<crate::pricing::TtlPricingCurve>,
}

impl PricingEngineService {
    /// Backward-compatible constructor for deployments that only need PAY_ONCE quotes.
    /// `GetJobPrice` requests return NOT_FOUND unless job pricing is attached.
    pub fn new(
        config: Arc<OperatorConfig>,
        benchmark_cache: Arc<BenchmarkCache>,
        pricing_config: Arc<
            Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
        >,
        signer: Arc<Mutex<OperatorSigner>>,
    ) -> Self {
        Self::new_with_configs(
            config,
            benchmark_cache,
            pricing_config,
            Arc::new(Mutex::new(JobPricingConfig::new())),
            SubscriptionPricingConfig::new(),
            signer,
        )
    }

    /// Fully configured constructor including per-job and subscription pricing maps.
    pub fn new_with_configs(
        config: Arc<OperatorConfig>,
        benchmark_cache: Arc<BenchmarkCache>,
        pricing_config: Arc<
            Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
        >,
        job_pricing_config: Arc<Mutex<JobPricingConfig>>,
        subscription_config: SubscriptionPricingConfig,
        signer: Arc<Mutex<OperatorSigner>>,
    ) -> Self {
        Self {
            config,
            benchmark_cache,
            pricing_config,
            job_pricing_config,
            subscription_config: Arc::new(Mutex::new(subscription_config)),
            signer,
            pow_difficulty: DEFAULT_POW_DIFFICULTY,
            x402_config: None,
            tee_config: crate::pricing::TeePricing::default(),
            ttl_curve: None,
        }
    }

    /// Backward-compatible constructor for deployments that only use per-job RFQ.
    /// Subscription and event-driven GetPrice requests will return NOT_FOUND unless
    /// subscription config is attached via `with_subscription_pricing`.
    pub fn with_job_pricing(
        config: Arc<OperatorConfig>,
        benchmark_cache: Arc<BenchmarkCache>,
        pricing_config: Arc<
            Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
        >,
        job_pricing_config: Arc<Mutex<JobPricingConfig>>,
        signer: Arc<Mutex<OperatorSigner>>,
    ) -> Self {
        Self::new_with_configs(
            config,
            benchmark_cache,
            pricing_config,
            job_pricing_config,
            SubscriptionPricingConfig::new(),
            signer,
        )
    }

    /// Attach subscription/event-driven pricing config after construction.
    pub fn with_subscription_pricing(
        mut self,
        subscription_config: SubscriptionPricingConfig,
    ) -> Self {
        self.subscription_config = Arc::new(Mutex::new(subscription_config));
        self
    }

    /// Enable x402 settlement options in `GetJobPriceResponse`.
    ///
    /// When configured, every job quote response will include cross-chain
    /// payment options that clients can use to settle via x402.
    pub fn with_x402_settlement(mut self, config: X402SettlementConfig) -> Self {
        self.x402_config = Some(config);
        self
    }

    /// Attach TEE pricing configuration.
    ///
    /// Controls whether this operator advertises TEE capability and
    /// what price multiplier to apply for TEE-attested quotes.
    pub fn with_tee_pricing(mut self, tee_config: crate::pricing::TeePricing) -> Self {
        self.tee_config = tee_config;
        self
    }

    /// Attach a TTL pricing curve for non-linear duration-based pricing.
    ///
    /// When set, the pricing engine applies a curve multiplier to TTL-based
    /// quotes instead of pure linear scaling. This allows operators to offer
    /// volume discounts for longer commitments.
    pub fn with_ttl_curve(mut self, curve: crate::pricing::TtlPricingCurve) -> Self {
        self.ttl_curve = Some(curve);
        self
    }

    /// Override the proof-of-work difficulty for testing.
    ///
    /// The default difficulty (`DEFAULT_POW_DIFFICULTY = 20`) can take too long
    /// on slow CI runners, causing challenge timestamps to expire. Use a low
    /// value (e.g. 1) in integration tests.
    pub fn with_pow_difficulty(mut self, difficulty: u32) -> Self {
        self.pow_difficulty = difficulty;
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
        let pricing_model = req.pricing_model; // 0=PayOnce, 1=Subscription, 2=EventDriven

        // Validate the requester address up-front: tnt-core v0.13.0+ rejects
        // wildcard (`address(0)`) quotes on-chain, so emitting one would just
        // produce an unredeemable quote. Reject at the gRPC boundary.
        let requester = parse_requester(&req.requester)?;

        info!(
            "Received GetPrice request for blueprint ID: {} (pricing_model={})",
            blueprint_id, pricing_model
        );

        let current_timestamp = Utc::now().timestamp() as u64;
        let challenge_timestamp = if req.challenge_timestamp > 0 {
            if req.challenge_timestamp
                < current_timestamp.saturating_sub(CHALLENGE_TIMESTAMP_TOLERANCE_SECS)
            {
                warn!(
                    "Challenge timestamp is too old: {}",
                    req.challenge_timestamp
                );
                return Err(Status::invalid_argument("Challenge timestamp is too old"));
            }
            if req.challenge_timestamp > current_timestamp + CHALLENGE_TIMESTAMP_TOLERANCE_SECS {
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

        let security_requirements = match req.security_requirements {
            Some(requirements) => requirements.clone(),
            None => {
                return Err(Status::invalid_argument("Missing security requirements"));
            }
        };

        // Branch on pricing model BEFORE benchmark lookup.
        // Subscription and event-driven modes don't need benchmarks.
        let model = PricingModelHint::try_from(pricing_model).map_err(|_| {
            Status::invalid_argument(format!(
                "Unknown pricing_model value: {pricing_model}. Expected 0 (PAY_ONCE), 1 (SUBSCRIPTION), or 2 (EVENT_DRIVEN)"
            ))
        })?;
        let price_model = match model {
            PricingModelHint::Subscription => {
                // SUBSCRIPTION: flat rate per billing interval
                let sub_config = self.subscription_config.lock().await;
                let pricing = sub_config
                    .get(&Some(blueprint_id))
                    .or_else(|| sub_config.get(&None))
                    .ok_or_else(|| {
                        Status::not_found(format!(
                            "No subscription pricing configured for blueprint {blueprint_id}"
                        ))
                    })?;
                info!(
                    "Subscription pricing for blueprint {}: rate={}, interval={}s",
                    blueprint_id, pricing.subscription_rate, pricing.subscription_interval
                );
                calculate_subscription_price(pricing, Some(&security_requirements))
            }
            PricingModelHint::EventDriven => {
                // EVENT_DRIVEN: flat rate per event
                let sub_config = self.subscription_config.lock().await;
                let pricing = sub_config
                    .get(&Some(blueprint_id))
                    .or_else(|| sub_config.get(&None))
                    .ok_or_else(|| {
                        Status::not_found(format!(
                            "No event pricing configured for blueprint {blueprint_id}"
                        ))
                    })?;
                calculate_event_price(pricing, Some(&security_requirements))
            }
            PricingModelHint::PayOnce => {
                // PAY_ONCE: resource-based pricing with benchmarks
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

                let pricing_config = self.pricing_config.lock().await;
                match calculate_price_with_curve(
                    benchmark_profile,
                    &pricing_config,
                    Some(blueprint_id),
                    ttl_blocks,
                    Some(&security_requirements),
                    self.ttl_curve.as_ref(),
                ) {
                    Ok(model) => model,
                    Err(e) => {
                        error!(
                            "Failed to calculate price for blueprint ID {}: {:?}",
                            blueprint_id, e
                        );
                        return Err(Status::internal("Failed to calculate price"));
                    }
                }
            }
        };

        // Get the total cost from the price model
        let crate::pricing::PriceModel {
            resources: price_resources,
            total_cost: base_cost,
            ..
        } = price_model;

        // Apply TEE pricing multiplier if requested
        let require_tee = req.require_tee;
        let total_cost =
            crate::pricing::apply_tee_pricing(base_cost, require_tee, &self.tee_config).map_err(
                |e| match e {
                    crate::error::PricingError::TeeNotAvailable => {
                        Status::unavailable("TEE execution is not available on this operator")
                    }
                    other => Status::internal(format!("TEE pricing error: {other}")),
                },
            )?;
        let (tee_attested, tee_provider) = if require_tee && self.tee_config.available {
            (true, self.tee_config.provider.clone())
        } else {
            (false, String::new())
        };

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
            .map(|rp| {
                let rate = rp.price_per_unit_rate.to_f64().ok_or_else(|| {
                    Status::internal(format!(
                        "Price rate {} for {:?} exceeds f64 range",
                        rp.price_per_unit_rate, rp.kind
                    ))
                })?;
                Ok(ProtoResourcePricing {
                    kind: format!("{:?}", rp.kind),
                    count: rp.count,
                    price_per_unit_rate: rate,
                })
            })
            .collect::<std::result::Result<Vec<_>, Status>>()?;

        // Precision limitation: the proto field `total_cost_rate` is `double` (f64),
        // so Decimal values exceeding f64's 53-bit mantissa will lose precision.
        // Changing this requires a proto schema migration.
        let total_cost_f64 = total_cost.to_f64().ok_or_else(|| {
            Status::internal(format!("Total cost {total_cost} exceeds f64 range"))
        })?;

        // Create the quote details directly using proto types
        let quote_details = QuoteDetails {
            blueprint_id,
            ttl_blocks,
            total_cost_rate: total_cost_f64,
            timestamp,
            expiry: expiry_time,
            resources: proto_resources,
            security_commitments: vec![security_commitment],
            requester: requester.0.to_vec(),
        };

        let confidentiality = if require_tee {
            crate::signer::Confidentiality::Required
        } else {
            crate::signer::Confidentiality::Any
        };
        let signable_quote = SignableQuote::with_confidentiality(
            quote_details,
            total_cost,
            confidentiality,
            requester,
        )
        .map_err(|e| {
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

        // Create the response with 65-byte signature (r || s || v)
        let mut sig_bytes = signed_quote.signature.to_bytes().to_vec();
        sig_bytes.push(27 + signed_quote.recovery_id);
        let response = GetPriceResponse {
            quote_details: Some(signed_quote.quote_details.clone()),
            signature: sig_bytes,
            operator_id: signed_quote.operator_id.0.to_vec(),
            proof_of_work: signed_quote.proof_of_work,
            tee_attested,
            tee_provider,
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

        // Validate the requester address up-front: tnt-core v0.13.0+ rejects
        // wildcard (`address(0)`) quotes on-chain. Reject at the gRPC boundary
        // so downstream signing code can rely on a non-zero `requester`.
        let requester = parse_requester(&req.requester)?;

        // Validate the inputs hash up-front: tnt-core v0.18.0+ binds the signed
        // price to keccak256(inputs) with no wildcard — a quote signed over a
        // missing/malformed hash could never be redeemed on-chain.
        let inputs_hash = parse_inputs_hash(&req.inputs_hash)?;

        info!(
            "Received GetJobPrice request for service {} job index {}",
            service_id, job_index
        );

        // Validate challenge timestamp
        let current_timestamp = Utc::now().timestamp() as u64;
        let challenge_timestamp = if req.challenge_timestamp > 0 {
            if req.challenge_timestamp
                < current_timestamp.saturating_sub(CHALLENGE_TIMESTAMP_TOLERANCE_SECS)
            {
                return Err(Status::invalid_argument("Challenge timestamp is too old"));
            }
            if req.challenge_timestamp > current_timestamp + CHALLENGE_TIMESTAMP_TOLERANCE_SECS {
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

        // Reject TEE requests if operator doesn't support TEE
        let require_tee = req.require_tee;
        if require_tee && !self.tee_config.available {
            return Err(Status::unavailable(
                "TEE execution is not available on this operator",
            ));
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

        // Apply TEE multiplier to the wei price if TEE is requested.
        let price = if require_tee {
            // Convert U256 → Decimal, apply multiplier, convert back
            let price_dec = rust_decimal::Decimal::from_str_exact(&price.to_string())
                .map_err(|e| Status::internal(format!("price→Decimal: {e}")))?;
            let adjusted = price_dec * self.tee_config.multiplier;
            let adjusted_str = adjusted.floor().to_string();
            alloy_primitives::U256::from_str_radix(&adjusted_str, 10)
                .map_err(|e| Status::internal(format!("Decimal→U256: {e}")))?
        } else {
            price
        };

        let (tee_attested, tee_provider) = if require_tee && self.tee_config.available {
            (true, self.tee_config.provider.clone())
        } else {
            (false, String::new())
        };

        let timestamp = current_timestamp;
        let expiry = timestamp + self.config.quote_validity_duration_secs;

        let confidentiality = if require_tee { 1u32 } else { 0u32 };
        let proto_details = ProtoJobQuoteDetails {
            service_id,
            job_index,
            price: price.to_be_bytes_vec(),
            timestamp,
            expiry,
            confidentiality,
            requester: requester.0.to_vec(),
            inputs_hash: inputs_hash.to_vec(),
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

        // 65-byte signature (r || s || v)
        let mut sig_bytes = signed.signature.to_bytes().to_vec();
        sig_bytes.push(27 + signed.recovery_id);
        let response = GetJobPriceResponse {
            quote_details: Some(signed.quote_details),
            signature: sig_bytes,
            operator_id: signed.operator_id.0.to_vec(),
            proof_of_work: signed.proof_of_work,
            settlement_options,
            x402_endpoint,
            tee_attested,
            tee_provider,
        };

        info!(
            "Sending signed job quote for service {} job index {}",
            service_id, job_index
        );
        Ok(Response::new(response))
    }
}

/// Parse and validate the proto `requester` bytes into an `Address`.
///
/// Rejects with `Status::invalid_argument` if the field is empty, the wrong
/// length, or the zero address. tnt-core v0.13.0+ refuses to verify quotes
/// signed for `address(0)` (audit Round 2 economic F1, PRs #124/#125), so
/// emitting one would just produce an unredeemable quote — fail fast at the
/// gRPC boundary so downstream signing code can rely on a non-zero requester.
fn parse_requester(bytes: &[u8]) -> std::result::Result<alloy_primitives::Address, Status> {
    if bytes.is_empty() {
        return Err(Status::invalid_argument(
            "requester required and must be non-zero",
        ));
    }
    if bytes.len() != 20 {
        return Err(Status::invalid_argument(format!(
            "requester must be a 20-byte address, got {} bytes",
            bytes.len()
        )));
    }
    let mut buf = [0u8; 20];
    buf.copy_from_slice(bytes);
    let addr = alloy_primitives::Address::from(buf);
    if addr.is_zero() {
        return Err(Status::invalid_argument(
            "requester required and must be non-zero",
        ));
    }
    Ok(addr)
}

/// Parse and validate the `inputs_hash` bytes from a job price request. Rejects
/// anything that is not exactly 32 bytes: tnt-core v0.18.0+ verifies
/// `keccak256(submitted inputs) == quote.details.inputsHash` with no wildcard,
/// so a quote signed over a missing or malformed hash is unredeemable — fail
/// fast at the gRPC boundary.
fn parse_inputs_hash(bytes: &[u8]) -> std::result::Result<alloy_primitives::B256, Status> {
    if bytes.is_empty() {
        return Err(Status::invalid_argument(
            "inputs_hash required: keccak256 of the exact job inputs to be submitted on-chain",
        ));
    }
    if bytes.len() != 32 {
        return Err(Status::invalid_argument(format!(
            "inputs_hash must be a 32-byte keccak256 hash, got {} bytes",
            bytes.len()
        )));
    }
    Ok(alloy_primitives::B256::from_slice(bytes))
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
///
/// Note: this conversion logic is intentionally kept in sync with
/// `AcceptedToken::convert_wei_to_amount` in `crates/x402/src/config.rs`.
/// A cyclic dependency prevents importing it directly.
fn convert_settlement_token(
    token: &X402AcceptedToken,
    price_wei: alloy_primitives::U256,
) -> std::result::Result<SettlementOptionInternal, String> {
    let wei_decimal = rust_decimal::Decimal::from_str_exact(&price_wei.to_string())
        .map_err(|e| format!("wei→Decimal: {e}"))?;
    let native_unit = rust_decimal::Decimal::from(10u64.pow(18));
    let native_amount = wei_decimal / native_unit;
    let token_amount = native_amount * token.rate_per_native_unit;
    let markup = rust_decimal::Decimal::ONE
        + rust_decimal::Decimal::from(token.markup_bps) / rust_decimal::Decimal::from(10_000u32);
    let final_amount = token_amount * markup;
    let token_unit = rust_decimal::Decimal::from(10u64.pow(u32::from(token.decimals)));
    let smallest_units = (final_amount * token_unit).floor().to_string();

    Ok(SettlementOptionInternal {
        network: token.network.clone(),
        asset: token.asset.clone(),
        symbol: token.symbol.clone(),
        amount: smallest_units,
        pay_to: token.pay_to.clone(),
        scheme: "exact".into(),
    })
}

fn compute_settlement_options(
    accepted_tokens: &[X402AcceptedToken],
    price_wei: alloy_primitives::U256,
) -> Vec<SettlementOptionInternal> {
    accepted_tokens
        .iter()
        .filter_map(|token| match convert_settlement_token(token, price_wei) {
            Ok(opt) => Some(opt),
            Err(e) => {
                warn!("Dropping settlement option for {}: {e}", token.symbol);
                None
            }
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
    job_pricing_config: Arc<Mutex<JobPricingConfig>>,
    subscription_config: SubscriptionPricingConfig,
    signer: Arc<Mutex<OperatorSigner>>,
) -> anyhow::Result<()> {
    run_rpc_server_with_tee(
        config,
        benchmark_cache,
        pricing_config,
        job_pricing_config,
        subscription_config,
        signer,
        crate::pricing::TeePricing::default(),
    )
    .await
}

/// Run the gRPC server with TEE pricing configuration.
pub async fn run_rpc_server_with_tee(
    config: Arc<OperatorConfig>,
    benchmark_cache: Arc<BenchmarkCache>,
    pricing_config: Arc<
        Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
    >,
    job_pricing_config: Arc<Mutex<JobPricingConfig>>,
    subscription_config: SubscriptionPricingConfig,
    signer: Arc<Mutex<OperatorSigner>>,
    tee_config: crate::pricing::TeePricing,
) -> anyhow::Result<()> {
    let addr = format!("{}:{}", config.rpc_bind_address, config.rpc_port).parse()?;
    info!("gRPC server listening on {}", addr);

    let pricing_service = PricingEngineService::new_with_configs(
        config,
        benchmark_cache,
        pricing_config,
        job_pricing_config,
        subscription_config,
        signer,
    )
    .with_tee_pricing(tee_config);
    let server = PricingEngineServer::new(pricing_service);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any)
        .expose_headers(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(server)
        .serve(addr)
        .await?;

    Ok(())
}
