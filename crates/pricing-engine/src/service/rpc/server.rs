//! RPC server implementation for the Tangle Cloud Pricing Engine
//!
//! This module provides a JSON-RPC server that allows users to query
//! pricing information and obtain signed price quotes from the operator.

use std::{fmt, net::SocketAddr, sync::Arc};

use blueprint_crypto::KeyType;
use jsonrpsee::{
    core::RpcResult,
    server::{RpcModule, ServerBuilder, ServerHandle},
    types::error::{ErrorObject, ErrorObjectOwned},
};
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::{
    calculation::{self, PricingContext, calculate_service_price},
    error::{Error, PricingError, Result},
    models::PricingModel,
    rfq::RfqProcessor,
    types::{Price, ResourceRequirement},
};

/// RPC API trait for the pricing engine
#[jsonrpsee::proc_macros::rpc(server)]
pub trait PricingApi {
    /// Get operator information
    #[method(name = "pricing_getOperatorInfo")]
    fn get_operator_info(&self) -> RpcResult<OperatorInfo>;

    /// Get available pricing models for the operator
    #[method(name = "pricing_getPricingModels")]
    fn get_pricing_models(&self) -> RpcResult<Vec<PricingModelInfo>>;

    /// Calculate price for a service with specified requirements
    #[method(name = "pricing_calculatePrice")]
    fn calculate_price(&self, request: PriceCalculationRequest) -> RpcResult<PriceQuote>;

    /// Submit an RFQ request and get quotes from connected operators
    #[method(name = "pricing_requestForQuote")]
    fn request_for_quote(&self, request: RfqRequest) -> RpcResult<String>;

    /// Get results from a previously submitted RFQ request
    #[method(name = "pricing_getRfqResults")]
    fn get_rfq_results(&self, request_id: String) -> RpcResult<RfqResponse>;
}

/// Operator information returned by the RPC API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorInfo {
    /// Operator identifier (public key)
    pub id: String,
    /// Operator name
    pub name: String,
    /// Operator description
    pub description: Option<String>,
    /// Supported blueprint IDs
    pub supported_blueprints: Vec<String>,
}

/// Pricing model information returned by the RPC API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModelInfo {
    /// Model identifier
    pub id: String,
    /// Model name
    pub name: String,
    /// Model description
    pub description: Option<String>,
    /// Blueprint ID this model applies to
    pub blueprint_id: String,
    /// Whether this model is currently active
    pub active: bool,
}

/// Request to calculate the price for a service
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PriceCalculationRequest {
    /// Blueprint ID for the service
    pub blueprint_id: String,
    /// Requirements for the service
    pub requirements: Vec<ResourceRequirement>,
    /// Duration of the service in seconds (optional)
    pub duration: Option<u64>,
}

/// Price quote response from the operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceQuote {
    /// The calculated price
    pub price: u64,
    /// The currency of the price (e.g., "TNT")
    pub currency: String,
    /// The pricing model used
    pub model_id: String,
    /// Expiration timestamp for this quote
    pub expires_at: u64,
    /// Operator signature of the quote (can be verified on-chain)
    pub signature: Option<String>,
}

/// Request for RFQ (Request for Quote) via RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfqRequest {
    /// Blueprint ID for the service
    pub blueprint_id: String,
    /// Resource requirements
    pub requirements: Vec<ResourceRequirement>,
    /// Optional maximum price willing to pay
    pub max_price: Option<u64>,
    /// Optional timeout in seconds
    pub timeout_seconds: Option<u64>,
}

/// Response for an RFQ request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfqResponse {
    /// Request ID
    pub request_id: String,
    /// List of quotes received
    pub quotes: Vec<RfqQuoteInfo>,
    /// Status of the request
    pub status: RfqRequestStatus,
}

/// Status of an RFQ request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RfqRequestStatus {
    /// Request is pending
    Pending,
    /// Request completed successfully
    Completed,
    /// Request timed out
    TimedOut,
    /// Request failed
    Failed(String),
}

/// Quote information from an RFQ response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfqQuoteInfo {
    /// Provider ID
    pub provider_id: String,
    /// Provider name
    pub provider_name: String,
    /// Price amount
    pub price: u64,
    /// Currency
    pub currency: String,
    /// Model ID used for the quote
    pub model_id: String,
    /// When the quote expires
    pub expires_at: u64,
}

/// RPC server for the pricing engine
pub struct RpcServer<K: KeyType> {
    /// Operator information
    operator_info: OperatorInfo,
    /// Available pricing models
    pricing_models: Vec<PricingModel>,
    /// RFQ processor for handling RFQ requests
    rfq_processor: Option<Arc<RfqProcessor<K>>>,
    /// Pending RFQ requests
    pending_rfqs: Arc<std::sync::Mutex<std::collections::HashMap<String, RfqRequestStatus>>>,
}

impl<K: KeyType> RpcServer<K> {
    /// Create a new RPC server
    pub fn new(operator_info: OperatorInfo, pricing_models: Vec<PricingModel>) -> Self {
        Self {
            operator_info,
            pricing_models,
            rfq_processor: None,
            pending_rfqs: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// Set the RFQ processor
    pub fn with_rfq_processor(mut self, processor: Arc<RfqProcessor<K>>) -> Self {
        self.rfq_processor = Some(processor);
        self
    }

    /// Create a JSON-RPC error object with the given code and message
    fn create_error(code: i32, message: String) -> ErrorObjectOwned {
        ErrorObject::owned(code, message, None::<()>)
    }

    /// Start the RPC server
    pub async fn start(self, addr: SocketAddr) -> Result<ServerHandle> {
        let server = ServerBuilder::default()
            .build(addr)
            .await
            .map_err(|e| Error::Other(format!("Failed to build RPC server: {}", e)))?;

        let mut module = RpcModule::new(());

        // Register the RPC methods
        let operator_info = self.operator_info.clone();
        module.register_async_method("pricing_getOperatorInfo", move |_, _, _| {
            let info = operator_info.clone();
            async move { Ok::<_, ErrorObjectOwned>(info) }
        })?;

        let pricing_models = self.pricing_models.clone();
        module.register_async_method("pricing_getPricingModels", move |_, _, _| {
            let models = pricing_models.clone();
            let model_infos = models
                .iter()
                .map(|m| PricingModelInfo {
                    id: m.name.clone(),
                    name: m.name.clone(),
                    description: m.description.clone(),
                    blueprint_id: m.blueprint_id.clone(),
                    active: true,
                })
                .collect::<Vec<_>>();

            async move { Ok::<_, ErrorObjectOwned>(model_infos) }
        })?;

        let pricing_models = self.pricing_models.clone();
        module.register_async_method("pricing_calculatePrice", move |params, _, _| {
            let pricing_models = pricing_models.clone();

            async move {
                let request = params.parse::<PriceCalculationRequest>().map_err(|e| {
                    Self::create_error(100, format!("Failed to parse parameters: {}", e))
                })?;

                let models = pricing_models;

                // Find applicable model
                let model = models
                    .iter()
                    .find(|m| m.blueprint_id == request.blueprint_id)
                    .ok_or_else(|| {
                        Self::create_error(
                            101,
                            format!(
                                "No pricing model available for blueprint {}",
                                request.blueprint_id
                            ),
                        )
                    })?;

                // Calculate price
                let context = PricingContext {
                    provider_id: "local_operator".to_string(),
                };

                let price_result = calculate_service_price(&request.requirements, model, &context)
                    .map_err(|e| {
                        Self::create_error(102, format!("Price calculation error: {}", e))
                    })?;

                // Create the price quote
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                Ok::<_, ErrorObjectOwned>(PriceQuote {
                    price: price_result.value as u64,
                    currency: price_result.token,
                    model_id: model.name.clone(),
                    expires_at: now + 3600, // 1 hour validity
                    signature: None,        // No signature in this simple calculation
                })
            }
        })?;

        let rfq_processor = self.rfq_processor.clone();
        let pending_rfqs = self.pending_rfqs.clone();
        module.register_async_method("pricing_requestForQuote", move |params, _, _| {
            let processor = rfq_processor.clone();
            let pending = pending_rfqs.clone();

            async move {
                let request = params.parse::<RfqRequest>().map_err(|e| {
                    Self::create_error(200, format!("Failed to parse parameters: {}", e))
                })?;

                let processor = processor.ok_or_else(|| {
                    Self::create_error(201, "RFQ processor not configured".to_string())
                })?;

                // Convert the RPC request to an RFQ request
                processor
                    .send_request(request.blueprint_id, request.requirements)
                    .await
                    .map_err(|e| Self::create_error(202, format!("RFQ error: {}", e)))?;

                let request_id = format!("rfq_{}", uuid::Uuid::new_v4());

                // Store the pending request
                pending
                    .lock()
                    .unwrap()
                    .insert(request_id.clone(), RfqRequestStatus::Pending);

                Ok::<_, ErrorObjectOwned>(request_id)
            }
        })?;

        let pending_rfqs = self.pending_rfqs.clone();
        module.register_async_method("pricing_getRfqResults", move |params, _, _| {
            let pending = pending_rfqs.clone();

            async move {
                let request_id = params.parse::<String>().map_err(|e| {
                    Self::create_error(300, format!("Failed to parse parameters: {}", e))
                })?;

                // Check if request exists and get status
                let status = {
                    let pending_guard = pending.lock().unwrap();
                    pending_guard
                        .get(&request_id)
                        .cloned()
                        .unwrap_or(RfqRequestStatus::Failed("Request not found".to_string()))
                };

                // In a real implementation, we'd retrieve the actual quotes
                // For now, return an empty response with the correct status
                let response = RfqResponse {
                    request_id: request_id.clone(),
                    quotes: Vec::new(),
                    status,
                };

                Ok::<_, ErrorObjectOwned>(response)
            }
        })?;

        // Start the server
        let server_handle = server.start(module);

        info!("RPC server started on {}", addr);
        Ok(server_handle)
    }
}

/// Service request handler for the pricing engine
pub struct ServiceRequestHandler {}
