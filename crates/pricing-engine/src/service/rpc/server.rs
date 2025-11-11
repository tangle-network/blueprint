use crate::benchmark_cache::BenchmarkCache;
use crate::config::OperatorConfig;
use crate::pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof};
use crate::pricing::calculate_price;
use crate::pricing_engine::asset::AssetType;
use crate::signer::{OperatorSigner, SignedQuote as SignerSignedQuote};
use crate::utils::bytes_to_u128;
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::sp_core::SpEcdsa;
use chrono::Utc;
use rust_decimal::prelude::ToPrimitive;
use tangle_subxt::subxt::utils::H160;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_arithmetic::per_things::Percent;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::{Asset, AssetSecurityRequirement};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};
use blueprint_core::{error, info, warn};

use crate::pricing_engine::{
    AssetSecurityCommitment, GetPriceRequest, GetPriceResponse, QuoteDetails,
    ResourcePricing as ProtoResourcePricing,
    pricing_engine_server::{PricingEngine, PricingEngineServer},
};

pub struct PricingEngineService {
    config: Arc<OperatorConfig>,
    benchmark_cache: Arc<BenchmarkCache>,
    pricing_config:
        Arc<Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>>,
    signer: Arc<Mutex<OperatorSigner<SpEcdsa>>>,
}

impl PricingEngineService {
    pub fn new(
        config: Arc<OperatorConfig>,
        benchmark_cache: Arc<BenchmarkCache>,
        pricing_config: Arc<
            Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
        >,
        signer: Arc<Mutex<OperatorSigner<SpEcdsa>>>,
    ) -> Self {
        Self {
            config,
            benchmark_cache,
            pricing_config,
            signer,
        }
    }

    // Create a security commitment from a security requirement using the minimum exposure percent
    fn create_security_commitment(
        requirement: &crate::pricing_engine::AssetSecurityRequirements,
    ) -> AssetSecurityCommitment {
        AssetSecurityCommitment {
            asset: requirement.asset.clone(),
            exposure_percent: requirement.minimum_exposure_percent,
        }
    }

    fn create_security_requirement(
        requirement: &crate::pricing_engine::AssetSecurityRequirements,
    ) -> AssetSecurityRequirement<u128> {
        let asset = requirement.asset.clone().unwrap().asset_type.unwrap();
        let asset = match asset {
            AssetType::Custom(asset_type) => {
                let chain_asset_type = bytes_to_u128(&asset_type);
                Asset::Custom(chain_asset_type)
            }
            AssetType::Erc20(asset_type) => Asset::Erc20(H160::from_slice(&asset_type)),
        };
        let minimum_percent = Percent(
            requirement
                .minimum_exposure_percent
                .try_into()
                .unwrap_or_default(),
        );
        let maximum_percent = Percent(
            requirement
                .maximum_exposure_percent
                .try_into()
                .unwrap_or_default(),
        );

        AssetSecurityRequirement {
            asset,
            min_exposure_percent: minimum_percent,
            max_exposure_percent: maximum_percent,
        }
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
        if !verify_proof(&challenge, &proof_of_work, DEFAULT_POW_DIFFICULTY).map_err(|e| {
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

        let security_requirement = Self::create_security_requirement(&security_requirements);

        // Get the pricing configuration
        let pricing_config = self.pricing_config.lock().await;

        // Calculate the price based on the benchmark profile, pricing config, and TTL
        let price_model = match calculate_price(
            benchmark_profile,
            &pricing_config,
            Some(blueprint_id),
            ttl_blocks,
            Some(security_requirement),
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
        let total_cost = price_model.total_cost;

        let security_commitment = Self::create_security_commitment(&security_requirements);

        // Prepare the response
        let expiry_time = Utc::now().timestamp() as u64 + self.config.quote_validity_duration_secs;
        let timestamp = Utc::now().timestamp() as u64;

        // Convert our internal resource pricing to proto resource pricing
        let proto_resources: Vec<ProtoResourcePricing> = price_model
            .resources
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
            security_commitments: Some(security_commitment),
        };

        // Generate proof of work for the response
        let response_pow = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY)
            .await
            .map_err(|e| {
                error!("Failed to generate proof of work: {}", e);
                Status::internal("Failed to generate proof of work")
            })?;

        // Sign the quote using the hash-based approach
        let signed_quote: SignerSignedQuote<SpEcdsa> = match self
            .signer
            .lock()
            .await
            .sign_quote(quote_details.clone(), response_pow.clone())
        {
            Ok(quote) => quote,
            Err(e) => {
                error!("Failed to sign quote for {}: {}", blueprint_id, e);
                return Err(Status::internal("Failed to sign price quote"));
            }
        };

        // Create the response
        let response = GetPriceResponse {
            quote_details: Some(signed_quote.quote_details),
            signature: signed_quote.signature.to_bytes().to_vec(),
            operator_id: signed_quote.operator_id.0.to_vec(),
            proof_of_work: signed_quote.proof_of_work,
        };

        info!("Sending signed quote for blueprint ID: {}", blueprint_id);
        Ok(Response::new(response))
    }
}

// Function to run the server (called from main.rs)
pub async fn run_rpc_server(
    config: Arc<OperatorConfig>,
    benchmark_cache: Arc<BenchmarkCache>,
    pricing_config: Arc<
        Mutex<std::collections::HashMap<Option<u64>, Vec<crate::pricing::ResourcePricing>>>,
    >,
    signer: Arc<Mutex<OperatorSigner<SpEcdsa>>>,
) -> anyhow::Result<()> {
    let addr = format!("{}:{}", config.rpc_bind_address, config.rpc_port).parse()?;
    info!("gRPC server listening on {}", addr);

    let pricing_service =
        PricingEngineService::new(config, benchmark_cache, pricing_config, signer);
    let server = PricingEngineServer::new(pricing_service);

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
