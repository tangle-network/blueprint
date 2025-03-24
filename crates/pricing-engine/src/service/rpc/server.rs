//! RPC server implementation for the Tangle Cloud Pricing Engine
//!
//! This module provides a JSON-RPC server that allows users to query
//! pricing information and obtain signed price quotes from the operator.

use std::{net::SocketAddr, sync::Arc};

use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    server::{RpcModule, ServerBuilder, ServerHandle},
};
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::{
    calculation::calculate_service_price,
    error::Result,
    models::PricingModel,
    rfq::{QuoteRequest, QuoteRequestId, RfqMessage, RfqMessageType, RfqProcessor},
    types::{PricingContext, ResourceRequirement, ServiceCategory},
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
    /// Supported service categories
    pub supported_categories: Vec<ServiceCategory>,
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
    /// Service category this model applies to
    pub category: ServiceCategory,
    /// Whether this model is currently active
    pub active: bool,
}

/// Request to calculate the price for a service
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct PriceCalculationRequest {
    /// Blueprint ID for the service
    pub blueprint_id: String,
    /// Requirements for the service
    pub requirements: ResourceRequirement,
    /// Service category
    pub category: ServiceCategory,
    /// Duration of the service in seconds (optional)
    pub duration: Option<u64>,
}

/// Price quote response from the operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceQuote {
    /// The calculated price
    pub price: u64,
    /// The currency of the price (e.g., "TNGL")
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
    /// Service category
    pub category: ServiceCategory,
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
pub struct RpcServer {
    /// Operator information
    operator_info: OperatorInfo,
    /// Available pricing models
    pricing_models: Vec<PricingModel>,
    /// RFQ processor for handling RFQ requests
    rfq_processor: Option<Arc<RfqProcessor>>,
    /// Pending RFQ requests
    pending_rfqs: Arc<std::sync::Mutex<std::collections::HashMap<String, RfqRequestStatus>>>,
}

impl RpcServer {
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
    pub fn with_rfq_processor(mut self, processor: Arc<RfqProcessor>) -> Self {
        self.rfq_processor = Some(processor);
        self
    }

    /// Start the RPC server
    pub async fn start(self, addr: SocketAddr) -> Result<ServerHandle, JsonRpseeError> {
        let server = ServerBuilder::default().build(addr).await?;
        let mut module = RpcModule::new(());

        // Register the RPC methods
        let operator_info = self.operator_info.clone();
        module.register_async_method("pricing_getOperatorInfo", move |_, _, _| {
            let info = operator_info.clone();
            async move { Ok::<_, JsonRpseeError>(info) }
        })?;

        let pricing_models = self.pricing_models.clone();
        module.register_async_method("pricing_getPricingModels", move |_, _, _| {
            let models = pricing_models.clone();
            async move {
                let model_infos = models
                    .iter()
                    .map(|model| PricingModelInfo {
                        id: format!("model_{}", model.name.to_lowercase().replace(" ", "_")),
                        name: model.name.clone(),
                        description: model.description.clone(),
                        category: model.category,
                        active: true,
                    })
                    .collect();

                Ok::<_, JsonRpseeError>(model_infos)
            }
        })?;

        let pricing_models_for_calc = self.pricing_models.clone();
        let operator_id = self.operator_info.id.clone();
        module.register_async_method("pricing_calculatePrice", move |params, _, _| {
            let models = pricing_models_for_calc.clone();
            let provider_id = operator_id.clone();

            async move {
                let request: PriceCalculationRequest = params.parse()?;

                // Find models that match the category
                let matching_models = models
                    .iter()
                    .filter(|m| m.category == request.category)
                    .collect::<Vec<_>>();

                if matching_models.is_empty() {
                    return Err(JsonRpseeError::Custom(format!(
                        "No pricing models available for category {:?}",
                        request.category
                    )));
                }

                // Find the best price
                let mut best_price = u64::MAX;
                let mut best_model_id = None;

                // Context for price calculation
                let context = PricingContext {
                    provider_id: provider_id.clone(),
                };

                // Calculate price for each matching model
                for model in matching_models {
                    match calculate_service_price(&request.requirements, model, &context) {
                        Ok(price) => {
                            if price < best_price {
                                best_price = price;
                                best_model_id = Some(format!(
                                    "model_{}",
                                    model.name.to_lowercase().replace(" ", "_")
                                ));
                            }
                        }
                        Err(e) => {
                            debug!("Error calculating price with model {}: {}", model.name, e);
                        }
                    }
                }

                // Return the price quote
                if let Some(model_id) = best_model_id {
                    // Current timestamp plus 10 minutes (example expiration)
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    let expires_at = now + 10 * 60; // 10 minutes from now

                    // In a real implementation, we would sign the price quote here
                    // using the operator's key
                    let signature = None; // Placeholder for actual signature

                    Ok(PriceQuote {
                        price: best_price,
                        currency: "TNGL".to_string(),
                        model_id,
                        expires_at,
                        signature,
                    })
                } else {
                    Err(JsonRpseeError::Custom(
                        "Failed to calculate price".to_string(),
                    ))
                }
            }
        })?;

        // Register RFQ methods if RFQ processor is available
        if let Some(rfq_processor) = self.rfq_processor.clone() {
            let pending_rfqs = self.pending_rfqs.clone();

            module.register_async_method("pricing_requestForQuote", move |params, _, _| {
                let rfq = rfq_processor.clone();
                let pending = pending_rfqs.clone();

                async move {
                    let request: RfqRequest = params.parse()?;

                    // Generate a request ID
                    let request_id = uuid::Uuid::new_v4().to_string();

                    // Store as pending
                    {
                        let mut pending = pending.lock().unwrap();
                        pending.insert(request_id.clone(), RfqRequestStatus::Pending);
                    }

                    // Start request processing in background
                    let req_id = request_id.clone();
                    let pending_clone = pending.clone();

                    tokio::spawn(async move {
                        // Convert timeout if provided
                        let timeout = request
                            .timeout_seconds
                            .map(|secs| std::time::Duration::from_secs(secs));

                        // Send the RFQ request
                        match rfq
                            .send_request(request.category, request.requirements)
                            .await
                        {
                            Ok(quotes) => {
                                // Update status to completed
                                let mut pending = pending_clone.lock().unwrap();
                                pending.insert(req_id, RfqRequestStatus::Completed);
                            }
                            Err(e) => {
                                // Update status to failed
                                let mut pending = pending_clone.lock().unwrap();
                                pending.insert(req_id, RfqRequestStatus::Failed(e.to_string()));
                            }
                        }
                    });

                    // Return the request ID
                    Ok(request_id)
                }
            })?;

            let pending_rfqs = self.pending_rfqs.clone();
            module.register_async_method("pricing_getRfqResults", move |params, _, _| {
                let pending = pending_rfqs.clone();

                async move {
                    let request_id: String = params.parse()?;

                    // Check if request exists and get status
                    let status = {
                        let pending = pending.lock().unwrap();
                        pending
                            .get(&request_id)
                            .cloned()
                            .unwrap_or(RfqRequestStatus::Failed("Request not found".to_string()))
                    };

                    // TODO: In a full implementation, we'd retrieve the actual quotes
                    // from storage or the RFQ processor

                    Ok(RfqResponse {
                        request_id,
                        quotes: Vec::new(), // Placeholder for actual quotes
                        status,
                    })
                }
            })?;
        }

        info!("Starting RPC server at {}", addr);
        let server_handle = server.start(module)?;

        Ok(server_handle)
    }
}

/// Service request handler to process signed price quotes and handle on-chain submission
pub struct ServiceRequestHandler {}
