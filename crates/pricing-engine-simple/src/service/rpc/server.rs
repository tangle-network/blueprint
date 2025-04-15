// src/service/rpc/server.rs

use crate::cache::{BlueprintHash, PriceCache};
use crate::config::OperatorConfig;
use crate::signer::{
    OperatorSigner, QuotePayload as SignerQuotePayload, SignedQuote as SignerSignedQuote,
};
use blueprint_crypto::BytesEncoding;
use blueprint_crypto::k256::K256Ecdsa;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};
use tracing::{debug, error, info, warn};

use crate::pricing_engine::{
    GetPriceRequest, GetPriceResponse, QuotePayload, SignedQuote,
    pricing_engine_server::{PricingEngine, PricingEngineServer},
};

pub struct PricingEngineService {
    config: Arc<OperatorConfig>,
    cache: Arc<PriceCache>,
    signer: Arc<Mutex<OperatorSigner<K256Ecdsa>>>,
}

impl PricingEngineService {
    pub fn new(
        config: Arc<OperatorConfig>,
        cache: Arc<PriceCache>,
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
        let blueprint_hash_hex: BlueprintHash = req.blueprint_hash_hex; // Already a String
        info!(
            "Received GetPrice request for blueprint: {}",
            blueprint_hash_hex
        );

        let blueprint_hash_bytes: [u8; 32] = hex::decode(&blueprint_hash_hex)
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

        let expiry_time = Utc::now().timestamp() as u64 + self.config.quote_validity_duration_secs;
        let timestamp = Utc::now().timestamp() as u64;

        let blueprint_id = if blueprint_hash_bytes.len() >= 8 {
            u64::from_be_bytes(blueprint_hash_bytes[0..8].try_into().unwrap())
        } else {
            warn!("Blueprint hash is shorter than 8 bytes, using 0 for ID");
            0
        };

        let signer_payload = SignerQuotePayload {
            blueprint_id,
            price_wei: price_model.price_per_second_wei,
            expiry: expiry_time,
            timestamp,
        };
        debug!("Constructed signer payload: {:?}", signer_payload);

        let signed_quote: SignerSignedQuote<K256Ecdsa> =
            match self.signer.lock().await.sign_quote(signer_payload) {
                Ok(quote) => quote,
                Err(e) => {
                    error!("Failed to sign quote for {}: {}", blueprint_hash_hex, e);
                    return Err(Status::internal("Failed to sign price quote"));
                }
            };
        debug!(
            "Signed quote generated. Signature length: {}",
            signed_quote.signature.to_bytes().len()
        );

        let blueprint_hash_bytes_for_response = hex::decode(&blueprint_hash_hex)
            .map_err(|_| Status::internal("Failed to decode blueprint hash for response"))?;

        let response_payload = QuotePayload {
            blueprint_hash: blueprint_hash_bytes_for_response,
            price_wei: signed_quote.payload.price_wei.to_string(),
            expiry: signed_quote.payload.expiry,
            timestamp: signed_quote.payload.timestamp,
        };

        let response = GetPriceResponse {
            quote: Some(SignedQuote {
                payload: Some(response_payload),
                signature: signed_quote.signature.to_bytes().to_vec(),
                signer_pubkey: signed_quote.signer_pubkey.to_bytes().to_vec(),
            }),
        };

        info!("Sending signed quote for blueprint: {}", blueprint_hash_hex);
        Ok(Response::new(response))
    }
}

// Function to run the server (called from main.rs)
pub async fn run_rpc_server(
    config: Arc<OperatorConfig>,
    cache: Arc<PriceCache>,
    signer: Arc<Mutex<OperatorSigner<K256Ecdsa>>>,
) -> anyhow::Result<()> {
    let addr = config.rpc_bind_address.parse()?;
    info!("gRPC server listening on {}", addr);

    let pricing_service = PricingEngineService::new(config, cache, signer);
    let server = PricingEngineServer::new(pricing_service);

    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
