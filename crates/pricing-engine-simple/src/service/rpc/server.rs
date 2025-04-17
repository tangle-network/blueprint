// src/service/rpc/server.rs

use crate::config::OperatorConfig;
use crate::pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof};
use crate::signer::{
    OperatorSigner, QuotePayload as SignerQuotePayload, SignedQuote as SignerSignedQuote,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::K256Ecdsa;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};
use tracing::{error, info, warn};

use crate::pricing_engine::{
    GetPriceRequest, GetPriceResponse, QuoteDetails, ResourcePricing as ProtoResourcePricing,
    pricing_engine_server::{PricingEngine, PricingEngineServer},
};

pub struct PricingEngineService {
    config: Arc<OperatorConfig>,
    cache: Arc<crate::cache::PriceCache>,
    signer: Arc<Mutex<OperatorSigner<K256Ecdsa>>>,
}

impl PricingEngineService {
    pub fn new(
        config: Arc<OperatorConfig>,
        cache: Arc<crate::cache::PriceCache>,
        signer: Arc<Mutex<OperatorSigner<K256Ecdsa>>>,
    ) -> Self {
        Self {
            config,
            cache,
            signer,
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
        let ttl_seconds = req.ttl_seconds;
        let proof_of_work = req.proof_of_work;

        info!(
            "Received GetPrice request for blueprint ID: {}",
            blueprint_id
        );

        // Verify proof of work
        let challenge = generate_challenge(blueprint_id, Utc::now().timestamp() as u64);
        if !verify_proof(&challenge, &proof_of_work, DEFAULT_POW_DIFFICULTY).map_err(|e| {
            warn!("Failed to verify proof of work: {}", e);
            Status::invalid_argument("Invalid proof of work")
        })? {
            warn!("Invalid proof of work for blueprint ID: {}", blueprint_id);
            return Err(Status::invalid_argument("Invalid proof of work"));
        }

        // Check if we have a price model for this blueprint
        let price_model = match self.cache.get_price(blueprint_id) {
            Ok(Some(model)) => model,
            _ => {
                warn!(
                    "Price not found or cache error for blueprint ID: {}. Using defaults.",
                    blueprint_id
                );
                // Fallback to default pricing or error
                // For now, let's use a default PriceModel or return an error
                // This part needs refinement based on desired behavior
                return Err(Status::not_found(format!(
                    "Price not found for blueprint ID: {}",
                    blueprint_id
                )));
            }
        };

        // Calculate total cost based on TTL
        let total_cost = price_model.calculate_total_cost(ttl_seconds);

        // Prepare the response
        let expiry_time = Utc::now().timestamp() as u64 + self.config.quote_validity_duration_secs;
        let timestamp = Utc::now().timestamp() as u64;

        // Convert our internal resource pricing to proto resource pricing
        let proto_resources: Vec<ProtoResourcePricing> = price_model
            .resources
            .iter()
            .map(|rp| ProtoResourcePricing {
                kind: format!("{:?}", rp.kind), // Format ResourceUnit as String
                count: rp.count,
                price_per_unit_rate: rp.price_per_unit_rate.to_string(),
            })
            .collect();

        // Create the quote payload
        let signer_payload = SignerQuotePayload {
            blueprint_id,
            ttl_seconds,
            total_cost_rate: total_cost,
            resources: price_model.resources.clone(),
            expiry: expiry_time,
            timestamp,
        };

        // Generate proof of work for the response
        let response_pow = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY)
            .await
            .map_err(|e| {
                error!("Failed to generate proof of work: {}", e);
                Status::internal("Failed to generate proof of work")
            })?;

        // Sign the quote
        let signed_quote: SignerSignedQuote<K256Ecdsa> = match self
            .signer
            .lock()
            .await
            .sign_quote(signer_payload, response_pow.clone())
        {
            Ok(quote) => quote,
            Err(e) => {
                error!("Failed to sign quote for {}: {}", blueprint_id, e);
                return Err(Status::internal("Failed to sign price quote"));
            }
        };

        // Create the response
        let quote_details = QuoteDetails {
            blueprint_id,
            ttl_seconds,
            total_cost_rate: signed_quote.payload.total_cost_rate.to_string(),
            timestamp: signed_quote.payload.timestamp,
            expiry: signed_quote.payload.expiry,
            resources: proto_resources,
        };

        let response = GetPriceResponse {
            quote_details: Some(quote_details),
            signature: signed_quote.signature.to_bytes().to_vec(),
            operator_id: signed_quote.operator_id.to_vec(),
            proof_of_work: signed_quote.proof_of_work,
        };

        info!("Sending signed quote for blueprint ID: {}", blueprint_id);
        Ok(Response::new(response))
    }
}

// Function to run the server (called from main.rs)
pub async fn run_rpc_server(
    config: Arc<OperatorConfig>,
    cache: Arc<crate::cache::PriceCache>,
    signer: Arc<Mutex<OperatorSigner<K256Ecdsa>>>,
) -> anyhow::Result<()> {
    let addr = config.rpc_bind_address.parse()?;
    info!("gRPC server listening on {}", addr);

    let pricing_service = PricingEngineService::new(config, cache, signer);
    let server = PricingEngineServer::new(pricing_service);

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
