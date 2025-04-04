// src/service/rpc/server.rs

use crate::cache::{BlueprintHash, PriceCache};
use crate::config::OperatorConfig;
use crate::error::{PricingError, Result as PricingResult};
use crate::signer::{
    BlueprintHashBytes, OperatorSigner, QuotePayload as SignerQuotePayload,
    SignedQuote as SignerSignedQuote,
};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::{Request, Response, Status, transport::Server};
use tracing::{debug, error, info, warn};

// Assuming proto definitions are generated into this module
// You might need to adjust the path based on your build script (e.g., tonic-build)
pub mod pricing_proto {
    tonic::include_proto!("pricing_engine"); // Matches the package name in pricing.proto
}

use pricing_proto::{
    GetPriceRequest, GetPriceResponse, QuotePayload, SignedQuote,
    pricing_engine_server::{PricingEngine, PricingEngineServer},
};

pub struct PricingEngineService {
    config: Arc<OperatorConfig>,
    cache: Arc<PriceCache>,
    signer: Arc<OperatorSigner>,
}

impl PricingEngineService {
    pub fn new(
        config: Arc<OperatorConfig>,
        cache: Arc<PriceCache>,
        signer: Arc<OperatorSigner>,
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
        let blueprint_hash_hex: BlueprintHash = req.blueprint_hash_hex; // Already a String
        info!(
            "Received GetPrice request for blueprint: {}",
            blueprint_hash_hex
        );

        // 1. Decode Blueprint Hash
        let blueprint_hash_bytes: BlueprintHashBytes = hex::decode(&blueprint_hash_hex)
            .map_err(|e| {
                warn!(
                    "Invalid hex for blueprint hash '{}': {}",
                    blueprint_hash_hex, e
                );
                Status::invalid_argument(format!("Invalid blueprint_hash_hex format: {}", e))
            })?
            .try_into()
            .map_err(|v: Vec<u8>| {
                warn!(
                    "Incorrect byte length for blueprint hash '{}': expected 32, got {}",
                    blueprint_hash_hex,
                    v.len()
                );
                Status::invalid_argument(format!(
                    "Blueprint hash must be 32 bytes, got {}",
                    v.len()
                ))
            })?;

        // 2. Look up PriceModel in Cache
        let price_model = self
            .cache
            .get_price(&blueprint_hash_hex)
            .map_err(|e| {
                error!("Cache lookup failed for {}: {}", blueprint_hash_hex, e);
                Status::internal("Failed to access price cache")
            })?
            .ok_or_else(|| {
                warn!(
                    "Price not found in cache for blueprint: {}",
                    blueprint_hash_hex
                );
                Status::not_found(format!(
                    "Price not found for blueprint {}",
                    blueprint_hash_hex
                ))
            })?;

        // 3. Check if PriceModel is stale? (Optional - depends on requirements)
        // You might want to check price_model.generated_at against a max age.

        // 4. Calculate Expiry
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                error!("System time error: {}", e);
                Status::internal("Failed to get current time")
            })?
            .as_secs();
        let expiry = now + self.config.quote_validity_duration.as_secs();

        // 5. Create Quote Payload
        // Using the price_per_second_wei from the model. A real system might
        // use request parameters (like duration) to calculate the final price.
        let quote_payload = SignerQuotePayload {
            blueprint_hash: blueprint_hash_bytes,
            price_wei: price_model.price_per_second_wei, // Using price_per_second directly
            expiry,
            timestamp: now,
        };
        debug!("Prepared quote payload: {:?}", quote_payload);

        // 6. Sign Payload
        let signed_quote: SignerSignedQuote =
            self.signer.sign_quote(quote_payload).map_err(|e| {
                error!("Failed to sign quote for {}: {}", blueprint_hash_hex, e);
                Status::internal("Failed to sign price quote")
            })?;
        debug!(
            "Signed quote generated. Signature length: {}",
            signed_quote.signature.len()
        );

        // 7. Convert to gRPC Response Type
        let response_payload = QuotePayload {
            blueprint_hash: signed_quote.payload.blueprint_hash.to_vec(),
            // Convert u128 to String for protobuf compatibility
            price_wei: signed_quote.payload.price_wei.to_string(),
            expiry: signed_quote.payload.expiry,
            timestamp: signed_quote.payload.timestamp,
        };

        let response_quote = SignedQuote {
            payload: Some(response_payload), // Use Some for message fields
            signature: signed_quote.signature,
            signer_pubkey: signed_quote.signer_pubkey,
        };

        let response = GetPriceResponse {
            quote: Some(response_quote), // Use Some for message fields
        };

        info!("Sending signed quote for blueprint: {}", blueprint_hash_hex);
        Ok(Response::new(response))
    }
}

// Function to run the server (called from main.rs)
pub async fn run_rpc_server(
    config: Arc<OperatorConfig>,
    cache: Arc<PriceCache>,
    signer: Arc<OperatorSigner>,
) -> anyhow::Result<()> {
    let addr = config.rpc_bind_address.parse()?;
    info!("gRPC server listening on {}", addr);

    let pricing_service = PricingEngineService::new(config, cache, signer);
    let server = PricingEngineServer::new(pricing_service);

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
