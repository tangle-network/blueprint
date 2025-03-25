//! RFQ message processor for the Tangle Cloud Pricing Engine
//!
//! This module implements the core processing logic for RFQ messages,
//! integrating with the existing networking infrastructure to send and
//! receive messages.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_crypto::{self, KeyType};
use blueprint_networking::service_handle::NetworkServiceHandle;
use blueprint_networking::types::{MessageRouting, ParticipantId, ParticipantInfo};
use futures::StreamExt;
use libp2p::{PeerId, gossipsub::IdentTopic};
use sp_core::sr25519;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use super::protocol::{
    DEFAULT_QUOTE_COLLECTION_TIMEOUT, DEFAULT_QUOTE_TTL, DEFAULT_RFQ_REQUEST_TTL, RFQ_TOPIC_NAME,
};
use super::types::{
    PriceQuote, PriceQuoteResponse, QuoteRequest, QuoteRequestId, RfqError, RfqMessage,
    RfqMessageType, RfqResult, SignedPriceQuote,
};
use crate::Price;
use crate::calculation::{PricingContext, calculate_service_price};
use crate::models::PricingModel;
use crate::types::ResourceRequirement;

/// Configuration for the RFQ processor
#[derive(Debug, Clone)]
pub struct RfqProcessorConfig {
    /// Our local peer ID
    pub local_peer_id: PeerId,

    /// Our operator name
    pub operator_name: String,

    /// Available pricing models
    pub pricing_models: Vec<PricingModel>,

    /// Request time-to-live
    pub request_ttl: Duration,

    /// Quote time-to-live
    pub quote_ttl: Duration,

    /// Default timeout for quote collection
    pub quote_collection_timeout: Duration,

    /// Maximum number of quotes to collect
    pub max_quotes: usize,
}

impl Default for RfqProcessorConfig {
    fn default() -> Self {
        Self {
            local_peer_id: PeerId::random(),
            operator_name: "Unknown Operator".to_string(),
            pricing_models: Vec::new(),
            request_ttl: Duration::from_secs(DEFAULT_RFQ_REQUEST_TTL),
            quote_ttl: Duration::from_secs(DEFAULT_QUOTE_TTL),
            quote_collection_timeout: Duration::from_secs(DEFAULT_QUOTE_COLLECTION_TIMEOUT),
            max_quotes: 50,
        }
    }
}

/// Command enum for controlling the RFQ processor
enum RfqCommand<K: KeyType> {
    /// Send a request for quotes
    SendRequest {
        request: QuoteRequest,
        response_channel: oneshot::Sender<RfqResult<Vec<SignedPriceQuote<K>>>>,
    },

    /// Process an incoming message
    ProcessMessage {
        message: RfqMessage,
        source: Option<PeerId>,
    },

    /// Cancel a previous request
    CancelRequest { request_id: QuoteRequestId },

    /// Set the available pricing models
    SetPricingModels { models: Vec<PricingModel> },

    /// Shutdown the processor
    Shutdown {
        response_channel: oneshot::Sender<()>,
    },
}

/// State of the RFQ processor
struct RfqProcessorState<K: KeyType> {
    /// Requests we're currently waiting for replies to
    pending_requests: HashMap<QuoteRequestId, PendingRequest<K>>,

    /// Requests we've seen recently (to avoid duplicates)
    seen_requests: HashSet<QuoteRequestId>,

    /// Available pricing models
    pricing_models: Vec<PricingModel>,
}

/// A request we're waiting for quotes for
#[derive(Clone)]
struct PendingRequest<K: KeyType> {
    /// The original request
    request: QuoteRequest,

    /// Collected quotes so far
    quotes: Vec<SignedPriceQuote<K>>,

    /// Response channel to deliver quotes
    response_channel: Option<oneshot::Sender<RfqResult<Vec<SignedPriceQuote<K>>>>>,

    /// When this request was started
    started_at: Instant,
}

/// Request for Quote processor that handles sending and receiving quote requests
pub struct RfqProcessor<K: KeyType> {
    /// Configuration for the processor
    config: RfqProcessorConfig,

    /// Our local keypair for signing quotes
    key_pair: K::Secret,

    /// Command channel
    command_tx: mpsc::Sender<RfqCommand<K>>,

    /// Network handle for sending messages
    network_handle: Option<Arc<NetworkServiceHandle<K>>>,

    /// Background task handle
    _task_handle: Option<JoinHandle<()>>,

    /// Internal state shared with the processing task
    state: Arc<Mutex<RfqProcessorState<K>>>,
}

impl<K: KeyType> RfqProcessor<K> {
    /// Create a new RFQ processor
    pub fn new(config: RfqProcessorConfig, key_pair: K::Secret) -> Self {
        let (command_tx, command_rx) = mpsc::channel(100);

        let state = Arc::new(Mutex::new(RfqProcessorState {
            pending_requests: HashMap::new(),
            seen_requests: HashSet::new(),
            pricing_models: config.pricing_models.clone(),
        }));

        let state_clone = state.clone();
        let command_rx_clone = command_rx;
        let config_clone = config.clone();
        let key_pair_clone = key_pair.clone();

        // Start the processing task
        let task_handle = tokio::spawn(async move {
            Self::processing_task(
                command_rx_clone,
                state_clone,
                config_clone,
                key_pair_clone,
                None,
            )
            .await;
        });

        Self {
            config,
            key_pair,
            command_tx,
            network_handle: None,
            _task_handle: Some(task_handle),
            state,
        }
    }

    /// Start the RFQ processor with the given network handle
    pub fn start_with_network_handle(mut self, network_handle: NetworkServiceHandle<K>) -> Self {
        self.network_handle = Some(Arc::new(network_handle));

        // Subscribe to the RFQ topic
        if let Some(network) = &self.network_handle {
            // In a real implementation, you would call network.subscribe_to_topic(RFQ_TOPIC_NAME)
            // This is just a placeholder since the method doesn't exist in the provided code
            info!("Would subscribe to RFQ topic: {}", RFQ_TOPIC_NAME);
        }

        info!("RFQ processor started");
        self
    }

    /// Send a request for quotes
    pub async fn send_request(
        &self,
        blueprint_id: impl Into<String>,
        requirements: Vec<ResourceRequirement>,
    ) -> RfqResult<Vec<SignedPriceQuote<K>>> {
        let (response_tx, response_rx) = oneshot::channel();

        // Create a request
        let sender_id = self.get_public_key_bytes();
        let request = QuoteRequest::new(
            sender_id,
            blueprint_id,
            requirements,
            None,
            self.config.request_ttl,
        );

        // Send the command
        self.command_tx
            .send(RfqCommand::SendRequest {
                request,
                response_channel: response_tx,
            })
            .await
            .map_err(|_| RfqError::Other("Failed to send request command".to_string()))?;

        // Wait for the response with timeout
        let quotes = timeout(self.config.quote_collection_timeout, response_rx)
            .await
            .map_err(|_| RfqError::Timeout)?
            .map_err(|_| RfqError::Other("Response channel closed".to_string()))??;

        Ok(quotes)
    }

    /// Process an incoming RFQ message
    pub async fn process_incoming_message(
        &self,
        message: RfqMessage,
        source: Option<PeerId>,
    ) -> RfqResult<()> {
        self.command_tx
            .send(RfqCommand::ProcessMessage { message, source })
            .await
            .map_err(|_| RfqError::Other("Failed to send process message command".to_string()))?;

        Ok(())
    }

    /// Cancel a pending request
    pub async fn cancel_request(&self, request_id: QuoteRequestId) -> RfqResult<()> {
        self.command_tx
            .send(RfqCommand::CancelRequest { request_id })
            .await
            .map_err(|_| RfqError::Other("Failed to send cancel request command".to_string()))?;

        Ok(())
    }

    /// Update the available pricing models
    pub async fn set_pricing_models(&self, models: Vec<PricingModel>) -> RfqResult<()> {
        self.command_tx
            .send(RfqCommand::SetPricingModels { models })
            .await
            .map_err(|_| {
                RfqError::Other("Failed to send set pricing models command".to_string())
            })?;

        Ok(())
    }

    /// Shutdown the processor
    pub async fn shutdown(&self) -> RfqResult<()> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(RfqCommand::Shutdown {
                response_channel: tx,
            })
            .await
            .map_err(|_| RfqError::Other("Failed to send shutdown command".to_string()))?;

        rx.await
            .map_err(|_| RfqError::Other("Shutdown response channel closed".to_string()))?;

        Ok(())
    }

    /// Background task for processing commands
    async fn processing_task(
        mut command_rx: mpsc::Receiver<RfqCommand<K>>,
        state: Arc<Mutex<RfqProcessorState<K>>>,
        config: RfqProcessorConfig,
        key_pair: K::Secret,
        network_handle: Option<Arc<NetworkServiceHandle<K>>>,
    ) {
        // Start a periodic task for cleaning up expired requests
        let state_clone = state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                Self::cleanup_expired_requests(&state_clone).await;
            }
        });

        // Process commands
        while let Some(command) = command_rx.recv().await {
            match command {
                RfqCommand::SendRequest {
                    request,
                    response_channel,
                } => {
                    if let Err(e) = Self::handle_send_request(
                        &state,
                        &config,
                        &key_pair,
                        network_handle.as_ref(),
                        request,
                        response_channel,
                    )
                    .await
                    {
                        error!("Failed to send RFQ request: {}", e);
                    }
                }
                RfqCommand::ProcessMessage { message, source } => {
                    if let Err(e) = Self::handle_process_message(
                        &state,
                        &config,
                        &key_pair,
                        network_handle.as_ref(),
                        message,
                        source,
                    )
                    .await
                    {
                        error!("Failed to process RFQ message: {}", e);
                    }
                }
                RfqCommand::CancelRequest { request_id } => {
                    if let Err(e) =
                        Self::handle_cancel_request(&state, network_handle.as_ref(), request_id)
                            .await
                    {
                        error!("Failed to cancel request: {}", e);
                    }
                }
                RfqCommand::SetPricingModels { models } => {
                    let mut state = state.lock().unwrap();
                    state.pricing_models = models;
                    debug!("Updated pricing models");
                }
                RfqCommand::Shutdown { response_channel } => {
                    info!("Shutting down RFQ processor");
                    let _ = response_channel.send(());
                    break;
                }
            }
        }

        info!("RFQ processor stopped");
    }

    /// Handle sending a request for quotes
    async fn handle_send_request(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        config: &RfqProcessorConfig,
        key_pair: &K::Secret,
        network: Option<&Arc<NetworkServiceHandle<K>>>,
        request: QuoteRequest,
        response_channel: oneshot::Sender<RfqResult<Vec<SignedPriceQuote<K>>>>,
    ) -> RfqResult<()> {
        // Add to pending requests
        {
            let mut state = state.lock().unwrap();
            state.pending_requests.insert(
                request.id,
                PendingRequest {
                    request: request.clone(),
                    quotes: Vec::new(),
                    response_channel: Some(response_channel),
                    started_at: Instant::now(),
                },
            );
        }

        // Broadcast the request
        if let Some(network) = network {
            let message = RfqMessage::new(RfqMessageType::QuoteRequest(request));
            let message_bytes = bincode::serialize(&message)?;

            // Create routing for broadcast
            let routing = MessageRouting {
                message_id: rand::random::<u64>(),
                round_id: 0,
                recipient: None,
            };

            // In a real implementation, you would call network.send_message(routing, message_bytes)
            // This is just a placeholder since the method doesn't exist in the provided code
            debug!(
                "Would send RFQ broadcast with {} bytes",
                message_bytes.len()
            );
        } else {
            warn!("No network handle available for sending RFQ");
        }

        Ok(())
    }

    /// Handle processing an incoming message
    async fn handle_process_message(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        config: &RfqProcessorConfig,
        key_pair: &K::Secret,
        network: Option<&Arc<NetworkServiceHandle<K>>>,
        message: RfqMessage,
        source: Option<PeerId>,
    ) -> RfqResult<()> {
        match message.message_type {
            RfqMessageType::QuoteRequest(request) => {
                // Check if we've seen this request before
                {
                    let state = state.lock().unwrap();
                    if state.seen_requests.contains(&request.id) {
                        return Ok(());
                    }
                }

                // Add to seen requests
                {
                    let mut state = state.lock().unwrap();
                    state.seen_requests.insert(request.id);
                }

                // Check if request is expired
                if request.is_expired() {
                    debug!("Received expired RFQ request");
                    return Ok(());
                }

                // Generate a quote
                let quote = Self::generate_quote(state, config, &request)?;

                // Sign the quote
                let signed_quote = SignedPriceQuote::new(quote, key_pair)?;

                // Send the response
                if let Some(network) = network {
                    let response = PriceQuoteResponse {
                        request_id: request.id,
                        quotes: vec![signed_quote],
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                    };

                    // Serialize the response (in a real implementation, we'd encrypt this)
                    let response_bytes = bincode::serialize(&response)?;

                    // Create a response message
                    let message = RfqMessage::new(RfqMessageType::QuoteResponse(response_bytes));
                    let message_bytes = bincode::serialize(&message)?;

                    // Create routing for direct message
                    let routing = MessageRouting {
                        message_id: rand::random::<u64>(),
                        round_id: 0,
                        recipient: Some(ParticipantInfo {
                            id: participant_id_from_peer(&source.unwrap_or(PeerId::random())),
                            verification_id_key: None,
                        }),
                    };

                    // In a real implementation, you would call network.send_message(routing, message_bytes)
                    // This is just a placeholder since the method doesn't exist in the provided code
                    debug!(
                        "Would send quote response with {} bytes",
                        message_bytes.len()
                    );
                } else {
                    warn!("No network handle available for sending quote response");
                }
            }
            RfqMessageType::QuoteResponse(response_bytes) => {
                // Deserialize the response (in a real implementation, we'd decrypt this)
                let response: PriceQuoteResponse<K> = bincode::deserialize(&response_bytes)
                    .map_err(|e| RfqError::Deserialization(e.to_string()))?;

                // Check if we're waiting for this response
                let mut pending_request = {
                    let mut state = state.lock().unwrap();
                    state.pending_requests.remove(&response.request_id)
                };

                if let Some(mut pending_request) = pending_request {
                    // Add quotes to the pending request
                    pending_request.quotes.extend(response.quotes);

                    // Check if we've reached the maximum number of quotes
                    if pending_request.quotes.len() >= config.max_quotes {
                        // Return the quotes to the requester
                        if let Some(channel) = pending_request.response_channel.take() {
                            let _ = channel.send(Ok(pending_request.quotes.clone()));
                        }
                    } else {
                        // Put the pending request back
                        let mut state = state.lock().unwrap();
                        state
                            .pending_requests
                            .insert(response.request_id, pending_request);
                    }
                }
            }
            RfqMessageType::CancelRequest(request_id) => {
                // Remove the pending request
                let mut state = state.lock().unwrap();
                state.pending_requests.remove(&request_id);
            }
        }

        Ok(())
    }

    /// Handle cancelling a request
    async fn handle_cancel_request(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        network: Option<&Arc<NetworkServiceHandle<K>>>,
        request_id: QuoteRequestId,
    ) -> RfqResult<()> {
        // Remove the pending request
        {
            let mut state = state.lock().unwrap();
            state.pending_requests.remove(&request_id);
        }

        // Broadcast the cancellation
        if let Some(network) = network {
            let message = RfqMessage::new(RfqMessageType::CancelRequest(request_id));
            let message_bytes = bincode::serialize(&message)?;

            // Create routing for broadcast
            let routing = MessageRouting {
                message_id: rand::random::<u64>(),
                round_id: 0,
                sender: network.local_peer_id,
                recipient: None,
            };

            // In a real implementation, you would call network.send_message(routing, message_bytes)
            // This is just a placeholder since the method doesn't exist in the provided code
            debug!(
                "Would send cancel request broadcast with {} bytes",
                message_bytes.len()
            );
        }

        Ok(())
    }

    /// Generate a quote for the given request
    fn generate_quote(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        config: &RfqProcessorConfig,
        request: &QuoteRequest,
    ) -> RfqResult<PriceQuote> {
        // Get the pricing models
        let pricing_models = {
            let state = state.lock().unwrap();
            state.pricing_models.clone()
        };

        // Find models that match the blueprint_id
        let matching_models = pricing_models
            .iter()
            .filter(|m| m.blueprint_id == request.blueprint_id)
            .collect::<Vec<_>>();

        if matching_models.is_empty() {
            return Err(RfqError::QuoteGeneration(format!(
                "No pricing models available for blueprint {}",
                request.blueprint_id
            )));
        }

        // Calculate prices for each model
        let mut best_price = None;
        let mut best_model_id = None;

        // For each resource requirement, calculate the price
        if request.requirements.is_empty() {
            return Err(RfqError::QuoteGeneration(
                "No resource requirements provided".to_string(),
            ));
        }

        // Context for price calculation
        let context = PricingContext {
            provider_id: config.local_peer_id.to_string(),
        };

        // Calculate price for each matching model
        for model in matching_models.iter() {
            // Calculate price using all requirements
            match calculate_service_price(&request.requirements, model, &context) {
                Ok(price) => {
                    if best_price.is_none()
                        || best_price
                            .as_ref()
                            .map_or(true, |p: &Price| price.value < p.clone().value)
                    {
                        best_price = Some(price);
                        best_model_id = Some(format!(
                            "model_{}",
                            model.name.to_lowercase().replace(' ', "_")
                        ));
                    }
                }
                Err(e) => {
                    debug!("Error calculating price with model {}: {}", model.name, e);
                }
            }
        }

        // Create the quote
        if let (Some(price), Some(model_id)) = (best_price, best_model_id) {
            // Get the billing period from the model
            let billing_period = matching_models
                .iter()
                .find(|m| format!("model_{}", m.name.to_lowercase().replace(' ', "_")) == model_id)
                .and_then(|m| m.billing_period.clone());

            let quote = PriceQuote::new(
                request.id,
                config.local_peer_id.to_bytes(),
                config.operator_name.clone(),
                price,
                model_id,
                billing_period,
                config.quote_ttl,
            );

            Ok(quote)
        } else {
            Err(RfqError::QuoteGeneration(
                "Failed to calculate price".to_string(),
            ))
        }
    }

    /// Clean up expired requests
    async fn cleanup_expired_requests(state: &Arc<Mutex<RfqProcessorState<K>>>) {
        let mut expired_requests = Vec::new();

        // Find expired requests
        {
            let state = state.lock().unwrap();
            for (id, request) in &state.pending_requests {
                if request.request.is_expired() {
                    expired_requests.push(*id);
                }
            }
        }

        // Remove expired requests
        if !expired_requests.is_empty() {
            let mut state = state.lock().unwrap();
            for id in expired_requests {
                if let Some(request) = state.pending_requests.remove(&id) {
                    if let Some(channel) = request.response_channel {
                        let _ = channel.send(Ok(request.quotes));
                    }
                }
            }
        }

        // Clean up seen requests (keep only recent ones)
        // In a production implementation, we'd use a time-based approach
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut state = state.lock().unwrap();
        state.seen_requests.retain(|_| {
            // For simplicity, we're not tracking timestamps for seen requests
            // In a real implementation, we'd remove old entries based on time
            true
        });
    }

    /// Get our public key as bytes
    fn get_public_key_bytes(&self) -> Vec<u8> {
        // This would extract the public key bytes from the key pair
        // For simplicity, we're using a placeholder
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
}
