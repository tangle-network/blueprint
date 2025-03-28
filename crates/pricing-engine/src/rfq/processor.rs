//! RFQ message processor for the Tangle Cloud Pricing Engine
//!
//! This module implements the core processing logic for RFQ messages,
//! integrating with the existing networking infrastructure to send and
//! receive quotes between clients and operators.
//!
//! # Message Flow
//! 1. Client broadcasts a `QuoteRequest` over gossip
//! 2. Operators receive the request, generate quotes based on their pricing models
//! 3. Operators send signed `PriceQuote` responses directly to the requester
//! 4. Client collects quotes and returns them to the caller

use blueprint_crypto::{self, KeyType};
use blueprint_networking::service::NetworkCommandMessage;
use blueprint_networking::service_handle::NetworkServiceHandle;
use blueprint_networking::types::{MessageRouting, ProtocolMessage};
use libp2p::PeerId;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
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
use crate::calculation::calculate_service_price;
use crate::models::PricingModel;
use crate::types::ResourceRequirement;

/// Configuration for the RFQ processor
#[derive(Debug, Clone)]
pub struct RfqProcessorConfig<K: KeyType> {
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

    /// Provider public key for signing quotes
    pub provider_public_key: Option<K::Public>,
}

impl<K: KeyType> Default for RfqProcessorConfig<K> {
    fn default() -> Self {
        Self {
            local_peer_id: PeerId::random(),
            operator_name: "Unknown Operator".to_string(),
            pricing_models: Vec::new(),
            request_ttl: Duration::from_secs(DEFAULT_RFQ_REQUEST_TTL),
            quote_ttl: Duration::from_secs(DEFAULT_QUOTE_TTL),
            quote_collection_timeout: Duration::from_secs(DEFAULT_QUOTE_COLLECTION_TIMEOUT),
            max_quotes: 50,
            provider_public_key: None,
        }
    }
}

/// Command enum for controlling the RFQ processor
enum RfqCommand<K: KeyType> {
    /// Send a request for quotes
    SendRequest {
        request: QuoteRequest<K>,
        response_channel: oneshot::Sender<RfqResult<Vec<SignedPriceQuote<K>>>>,
    },

    /// Process an incoming message
    ProcessMessage {
        message: RfqMessage<K>,
        source: PeerId,
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
pub struct RfqProcessorState<K: KeyType> {
    /// Requests we're currently waiting for replies to
    pending_requests: HashMap<QuoteRequestId, PendingRequest<K>>,

    /// Requests we've seen recently (to avoid duplicates)
    seen_requests: HashSet<QuoteRequestId>,

    /// Available pricing models
    pricing_models: Vec<PricingModel>,
}

/// A request we're waiting for quotes for
struct PendingRequest<K: KeyType> {
    /// The original request
    request: QuoteRequest<K>,

    /// Collected quotes so far
    quotes: Vec<SignedPriceQuote<K>>,

    /// Response channel to deliver quotes
    response_channel: Option<oneshot::Sender<RfqResult<Vec<SignedPriceQuote<K>>>>>,

    /// When this request was started
    started_at: Instant,
}

/// Request for Quote processor that handles sending and receiving quote requests
///
/// This processor integrates with the networking layer to:
/// - Send RFQ requests as a client and collect responses
/// - Process incoming RFQ requests as an operator and send quotes
/// - Handle message serialization, signing, and validation
pub struct RfqProcessor<K: KeyType> {
    /// Configuration for the processor
    config: RfqProcessorConfig<K>,

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
    ///
    /// # Arguments
    /// * `config` - Configuration for the RFQ processor
    /// * `key_pair` - Keypair used for signing quotes
    /// * `network_handle` - The network handle used for sending/receiving messages
    ///
    /// # Returns
    /// An initialized RFQ processor connected to the network
    pub fn new(
        mut config: RfqProcessorConfig<K>,
        key_pair: K::Secret,
        network_handle: NetworkServiceHandle<K>,
    ) -> Self {
        // Set provider public key from key_pair if not already set
        if config.provider_public_key.is_none() {
            config.provider_public_key = Some(K::public_from_secret(&key_pair));
        }

        let (command_tx, command_rx) = mpsc::channel(100);

        let state = Arc::new(Mutex::new(RfqProcessorState {
            pending_requests: HashMap::new(),
            seen_requests: HashSet::new(),
            pricing_models: config.pricing_models.clone(),
        }));

        // Wrap the network handle in an Arc
        let network_handle_arc = Arc::new(network_handle);

        info!(
            "Creating RFQ processor with network handle, peer_id: {}",
            network_handle_arc.local_peer_id
        );

        // Subscribe to the RFQ topic immediately
        if let Err(e) = network_handle_arc.send_network_message(
            NetworkCommandMessage::SubscribeToTopic(RFQ_TOPIC_NAME.to_string()),
        ) {
            warn!("Failed to subscribe to RFQ topic: {}", e);
        } else {
            info!("Subscribed to RFQ topic: {}", RFQ_TOPIC_NAME);
        }

        // Clone the command channel and network handle for listener task
        let command_tx_clone = command_tx.clone();

        // Create a new network handle for receiving messages
        // Since next_protocol_message requires mutable access, we need a separate handle
        let mut network_recv_handle = (*network_handle_arc).clone();

        // Spawn a task to listen for network messages with mutable handle
        tokio::spawn(async move {
            info!("Starting network message listener for RFQ processor");
            Self::network_message_listener(command_tx_clone, &mut network_recv_handle).await;
        });

        let state_clone = state.clone();
        let command_rx_clone = command_rx;
        let config_clone = config.clone();
        let key_pair_clone = key_pair.clone();
        let network_handle_task_clone = network_handle_arc.clone();

        // Start the processing task with the network handle
        let task_handle = tokio::spawn(async move {
            Self::processing_task(
                command_rx_clone,
                state_clone,
                config_clone,
                key_pair_clone,
                Some(network_handle_task_clone),
            )
            .await;
        });

        info!("RFQ processor created and initialized with network handle");

        Self {
            config,
            key_pair,
            command_tx,
            network_handle: Some(network_handle_arc),
            _task_handle: Some(task_handle),
            state,
        }
    }

    /// Send a request for quotes
    ///
    /// # Arguments
    /// * `blueprint_id` - The blueprint ID to request quotes for
    /// * `requirements` - Resource requirements for the service
    ///
    /// # Returns
    /// A vector of signed price quotes collected from operators
    ///
    /// # Errors
    /// Returns an error if the request fails to be sent or times out
    pub async fn send_request(
        &self,
        blueprint_id: impl Into<String>,
        requirements: Vec<ResourceRequirement>,
    ) -> RfqResult<Vec<SignedPriceQuote<K>>> {
        let (response_tx, response_rx) = oneshot::channel();

        // Create a request
        let public_key = K::public_from_secret(&self.key_pair);
        let request = QuoteRequest::<K>::new(
            public_key.clone(),
            blueprint_id,
            requirements,
            None,
            self.config.request_ttl,
        );

        info!(
            "Sending RFQ request id={} for blueprint={} with {} requirements",
            request.id.to_string(),
            request.blueprint_id,
            request.requirements.len()
        );

        // Send the command to process the request
        self.command_tx
            .send(RfqCommand::SendRequest {
                request,
                response_channel: response_tx,
            })
            .await
            .map_err(|_| RfqError::Other("Failed to send request command".to_string()))?;

        // Wait for the response with timeout
        let start_time = Instant::now();
        info!(
            "Waiting for quote responses with timeout of {} seconds...",
            self.config.quote_collection_timeout.as_secs()
        );

        // Await the result with timeout
        let result = match timeout(self.config.quote_collection_timeout, response_rx).await {
            Ok(channel_result) => match channel_result {
                Ok(inner_result) => inner_result,
                Err(_) => Err(RfqError::Other("Response channel closed".to_string())),
            },
            Err(_) => {
                error!(
                    "Timeout waiting for quotes after {} seconds",
                    self.config.quote_collection_timeout.as_secs()
                );
                Err(RfqError::Timeout)
            }
        };

        // Log outcome and timing information
        let elapsed = start_time.elapsed();
        match &result {
            Ok(quotes) => {
                info!(
                    "Successfully collected {} quotes in {:.2}s",
                    quotes.len(),
                    elapsed.as_secs_f32()
                );

                if !quotes.is_empty() {
                    for (i, quote) in quotes.iter().enumerate() {
                        // We kept public_key value for this comparison
                        let provider_id_matches = public_key == quote.quote.provider_id;
                        info!(
                            "Quote #{}: {} {} from {} ({})",
                            i + 1,
                            quote.quote.price.value,
                            quote.quote.price.token,
                            quote.quote.provider_name,
                            if provider_id_matches {
                                "LOCAL"
                            } else {
                                "REMOTE"
                            }
                        );
                    }
                } else {
                    warn!("No quotes received from any operators");
                }
            }
            Err(e) => error!("Error collecting quotes: {}", e),
        }

        result
    }

    /// Process an incoming RFQ message
    ///
    /// # Arguments
    /// * `message` - The RFQ message to process
    /// * `source` - Optional source peer ID
    ///
    /// # Returns
    /// Success if the message was processed, error otherwise
    pub async fn process_incoming_message(
        &self,
        message: RfqMessage<K>,
        source: PeerId,
    ) -> RfqResult<()> {
        self.command_tx
            .send(RfqCommand::ProcessMessage { message, source })
            .await
            .map_err(|_| RfqError::Other("Failed to send process message command".to_string()))?;

        Ok(())
    }

    /// Cancel a pending request
    ///
    /// # Arguments
    /// * `request_id` - The ID of the request to cancel
    ///
    /// # Returns
    /// Success if the request was canceled, error otherwise
    pub async fn cancel_request(&self, request_id: QuoteRequestId) -> RfqResult<()> {
        self.command_tx
            .send(RfqCommand::CancelRequest { request_id })
            .await
            .map_err(|_| RfqError::Other("Failed to send cancel request command".to_string()))?;

        Ok(())
    }

    /// Update the available pricing models
    ///
    /// # Arguments
    /// * `models` - New pricing models to use
    ///
    /// # Returns
    /// Success if the models were updated, error otherwise
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
    ///
    /// # Returns
    /// Success if the processor was shutdown, error otherwise
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
    ///
    /// This task runs in the background and processes commands from the command channel.
    /// It also periodically cleans up expired requests.
    async fn processing_task(
        mut command_rx: mpsc::Receiver<RfqCommand<K>>,
        state: Arc<Mutex<RfqProcessorState<K>>>,
        config: RfqProcessorConfig<K>,
        key_pair: K::Secret,
        network_handle: Option<Arc<NetworkServiceHandle<K>>>,
    ) {
        // Log network handle status at task startup
        if let Some(handle) = &network_handle {
            info!(
                "RFQ processing task started with network handle. Peer ID: {}",
                handle.local_peer_id
            );
        } else {
            error!("RFQ processing task started WITHOUT a network handle!");
        }

        // Start a periodic task for cleaning up expired requests
        let state_clone = state.clone();
        let cleanup_task = tokio::spawn(async move {
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
                    info!(
                        "Processing SendRequest command for blueprint: {}",
                        request.blueprint_id
                    );
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

        // Cancel the cleanup task
        cleanup_task.abort();
        info!("RFQ processor stopped");
    }

    /// Handle sending a request for quotes
    ///
    /// This broadcasts the request to all operators on the RFQ topic.
    /// Responses will be collected by the message processing code.
    async fn handle_send_request(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        config: &RfqProcessorConfig<K>,
        key_pair: &K::Secret,
        network: Option<&Arc<NetworkServiceHandle<K>>>,
        request: QuoteRequest<K>,
        response_channel: oneshot::Sender<RfqResult<Vec<SignedPriceQuote<K>>>>,
    ) -> RfqResult<()> {
        // Detailed logging of network handle status
        match &network {
            Some(handle) => info!(
                "handle_send_request has network handle with peer_id: {}",
                handle.local_peer_id
            ),
            None => error!("handle_send_request called WITHOUT network handle!"),
        }

        // First, generate our own quote for this request
        // This ensures we always return at least one quote (our own)
        let mut initial_quotes = Vec::new();
        info!("Generating local quote for own request");
        match Self::generate_quote(state, config, &request) {
            Ok(quote) => {
                // Sign the quote
                match SignedPriceQuote::<K>::new(quote, key_pair) {
                    Ok(signed_quote) => {
                        info!("Successfully generated and signed local quote");
                        initial_quotes.push(signed_quote);
                    }
                    Err(e) => {
                        error!("Failed to sign local quote: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to generate local quote: {}", e);
            }
        }

        // Add to pending requests
        {
            let mut state = state.lock().unwrap();
            state.pending_requests.insert(
                request.id,
                PendingRequest {
                    request: request.clone(),
                    quotes: initial_quotes.clone(), // Start with our own quote
                    response_channel: Some(response_channel),
                    started_at: Instant::now(),
                },
            );
            debug!(
                request_id = ?request.id.to_string(),
                "Added pending RFQ request with {} initial quotes",
                initial_quotes.len()
            );
        }

        // Broadcast the request
        if let Some(network) = network {
            // Log detailed network information
            info!(
                "Broadcasting RFQ request with ID {:?} over network to peer: {}",
                request.id, network.local_peer_id
            );

            let message = RfqMessage::new(RfqMessageType::QuoteRequest(request.clone()));
            let message_bytes = match bincode::serialize(&message) {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("Failed to serialize RFQ message: {}", e);
                    return Err(RfqError::Other(format!(
                        "Failed to serialize RFQ message: {}",
                        e
                    )));
                }
            };

            // Create routing for broadcast (gossip)
            let routing = MessageRouting {
                message_id: rand::random::<u64>(),
                round_id: 0,
                sender: network.local_peer_id,
                recipient: None, // None means broadcast/gossip
            };

            // Send as gossip message
            match network.send(routing, message_bytes) {
                Ok(_) => info!(
                    "Successfully broadcast RFQ request with ID: {:?}",
                    request.id
                ),
                Err(e) => {
                    error!("Failed to send RFQ broadcast: {}", e);
                    return Err(RfqError::Network(format!(
                        "Failed to send RFQ broadcast: {}",
                        e
                    )));
                }
            }

            debug!(
                request_id = ?request.id.to_string(),
                blueprint_id = %request.blueprint_id,
                "Sent RFQ broadcast"
            );
        } else {
            error!("No network handle available when trying to send RFQ request");

            // If there's no network handle but we have generated a local quote,
            // we should still return it to the client
            if !initial_quotes.is_empty() {
                info!("No network handle, but returning local quote directly");
                // Remove the pending request to get the response channel
                let mut state = state.lock().unwrap();
                if let Some(pending_request) = state.pending_requests.remove(&request.id) {
                    // If we have the response channel, use it
                    if let Some(resp_channel) = pending_request.response_channel {
                        let _ = resp_channel.send(Ok(initial_quotes));
                    }
                }
            } else {
                return Err(RfqError::Network("No network handle available".to_string()));
            }
        }

        Ok(())
    }

    /// Handle processing an incoming message
    ///
    /// This handles both quote requests and responses.
    /// For requests: generate a quote and send it directly to the requester
    /// For responses: add the quote to the corresponding pending request
    async fn handle_process_message(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        config: &RfqProcessorConfig<K>,
        key_pair: &K::Secret,
        network: Option<&Arc<NetworkServiceHandle<K>>>,
        message: RfqMessage<K>,
        source: PeerId,
    ) -> RfqResult<()> {
        // Log detailed message info
        info!("Processing message: {}", message.debug_info());

        match message.message_type {
            RfqMessageType::QuoteRequest(request) => {
                // Check if we've seen this request before
                let seen_already = {
                    let mut seen = false;
                    match state.lock() {
                        Ok(mut state) => {
                            if state.seen_requests.contains(&request.id) {
                                seen = true;
                            } else {
                                state.seen_requests.insert(request.id);
                            }
                        }
                        Err(e) => {
                            error!("Failed to lock state: {}", e);
                            return Err(RfqError::Other("Failed to lock state".to_string()));
                        }
                    }
                    seen
                };

                if seen_already {
                    debug!(
                        request_id = ?request.id.to_string(),
                        "Ignoring duplicate RFQ request"
                    );
                    return Ok(());
                }

                // Check if request is expired
                if request.is_expired() {
                    debug!(
                        request_id = ?request.id.to_string(),
                        "Ignoring expired RFQ request"
                    );
                    return Ok(());
                }

                // Generate a quote
                let quote = match Self::generate_quote(state, config, &request) {
                    Ok(quote) => quote,
                    Err(e) => {
                        debug!(
                            request_id = ?request.id.to_string(),
                            error = %e,
                            "Failed to generate quote for request"
                        );
                        return Ok(());
                    }
                };

                // Sign the quote
                let signed_quote = SignedPriceQuote::<K>::new(quote, key_pair)?;

                // Send the response directly to the requester
                if let Some(network) = network {
                    let response = PriceQuoteResponse {
                        request_id: request.id,
                        quotes: vec![signed_quote],
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                    };

                    // Serialize the response
                    let response_bytes = bincode::serialize(&response)?;

                    // Create a response message
                    let message =
                        RfqMessage::<K>::new(RfqMessageType::QuoteResponse(response_bytes));
                    let message_bytes = bincode::serialize(&message)?;

                    // Create routing for direct message to the requester
                    let routing = MessageRouting {
                        message_id: rand::random::<u64>(),
                        round_id: 0,
                        sender: network.local_peer_id,
                        recipient: Some(source),
                    };

                    // Send direct P2P message
                    network.send(routing, message_bytes).map_err(|e| {
                        RfqError::Network(format!("Failed to send quote response: {}", e))
                    })?;

                    debug!(
                        request_id = ?request.id.to_string(),
                        requester = %source,
                        "Sent quote response"
                    );
                } else {
                    return Err(RfqError::Network("No network handle available".to_string()));
                }
            }
            RfqMessageType::QuoteResponse(response_bytes) => {
                // Deserialize the response
                let response: PriceQuoteResponse<K> = bincode::deserialize(&response_bytes)?;

                // Log response details
                info!(
                    "Received quote response for request ID: {:?}, containing {} quotes from source: {:?}",
                    response.request_id,
                    response.quotes.len(),
                    source
                );

                // Check if we're waiting for this response
                let pending_request = {
                    let mut state = state.lock().unwrap();
                    let request = state.pending_requests.remove(&response.request_id);
                    if request.is_none() {
                        info!(
                            "Received quote response for unknown request ID: {:?}",
                            response.request_id
                        );
                    }
                    request
                };

                if let Some(mut pending_request) = pending_request {
                    // Add quotes to the pending request
                    let prev_count = pending_request.quotes.len();
                    pending_request.quotes.extend(response.quotes);
                    let new_count = pending_request.quotes.len();
                    let added_count = new_count - prev_count;

                    info!(
                        request_id = ?response.request_id.to_string(),
                        added_quotes = added_count,
                        total_quotes = new_count,
                        "Added peer quotes to pending request"
                    );

                    // Check if we've reached the maximum number of quotes
                    if pending_request.quotes.len() >= config.max_quotes {
                        // Return the quotes to the requester
                        if let Some(channel) = pending_request.response_channel.take() {
                            info!(
                                "Maximum quotes reached ({}), completing request with {} quotes",
                                config.max_quotes,
                                pending_request.quotes.len()
                            );
                            let _ = channel.send(Ok(pending_request.quotes.clone()));
                            return Ok(());
                        } else {
                            warn!("Response channel already consumed, can't send quotes");
                        }
                    } else {
                        debug!(
                            "Currently have {}/{} needed quotes",
                            pending_request.quotes.len(),
                            config.max_quotes
                        );
                    }

                    // Put the pending request back
                    let mut state = state.lock().unwrap();
                    state
                        .pending_requests
                        .insert(response.request_id, pending_request);
                } else {
                    warn!(
                        "Received quote response for non-pending request: {:?}",
                        response.request_id
                    );
                }
            }
            RfqMessageType::CancelRequest(request_id) => {
                // Remove the pending request
                let mut state = state.lock().unwrap();
                if state.pending_requests.remove(&request_id).is_some() {
                    debug!(
                        request_id = ?request_id.to_string(),
                        "Canceled request"
                    );
                }
            }
        }

        Ok(())
    }

    /// Handle cancelling a request
    ///
    /// This removes the pending request and broadcasts a cancellation message.
    async fn handle_cancel_request(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        network: Option<&Arc<NetworkServiceHandle<K>>>,
        request_id: QuoteRequestId,
    ) -> RfqResult<()> {
        // Remove the pending request
        {
            let mut state = state.lock().unwrap();
            if state.pending_requests.remove(&request_id).is_some() {
                debug!(
                    request_id = ?request_id.to_string(),
                    "Removed pending request"
                );
            }
        }

        // Broadcast the cancellation
        if let Some(network) = network {
            let message = RfqMessage::<K>::new(RfqMessageType::CancelRequest(request_id));
            let message_bytes = bincode::serialize(&message)?;

            // Create routing for broadcast
            let routing = MessageRouting {
                message_id: rand::random::<u64>(),
                round_id: 0,
                sender: network.local_peer_id,
                recipient: None, // None means broadcast/gossip
            };

            // Send as gossip message
            network
                .send(routing, message_bytes)
                .map_err(|e| RfqError::Network(format!("Failed to send cancel request: {}", e)))?;

            debug!(
                request_id = ?request_id.to_string(),
                "Broadcast request cancellation"
            );
        }

        Ok(())
    }

    /// Generate a quote for the given request
    ///
    /// This finds the best matching pricing model and calculates a price quote.
    pub fn generate_quote(
        state: &Arc<Mutex<RfqProcessorState<K>>>,
        config: &RfqProcessorConfig<K>,
        request: &QuoteRequest<K>,
    ) -> RfqResult<PriceQuote<K>> {
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

        // For each resource requirement, calculate the price
        if request.requirements.is_empty() {
            return Err(RfqError::QuoteGeneration(
                "No resource requirements provided".to_string(),
            ));
        }

        // Find best price among all matching models
        let mut best_price = None;
        let mut best_model_id = None;

        // Calculate price for each matching model
        for model in matching_models.iter() {
            // Calculate price using all requirements
            match calculate_service_price(&request.requirements, model) {
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

            // Get provider public key or return error if not set
            let provider_key = config.provider_public_key.clone().ok_or_else(|| {
                RfqError::QuoteGeneration("Provider public key not set".to_string())
            })?;

            let quote = PriceQuote::<K>::new(
                request.id,
                provider_key,
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
    ///
    /// This removes expired requests and completes them with the quotes collected so far.
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
                    debug!(
                        request_id = ?id.to_string(),
                        "Cleaning up expired request with {} quotes",
                        request.quotes.len()
                    );

                    if let Some(channel) = request.response_channel {
                        // Complete the request with the quotes collected so far
                        let _ = channel.send(Ok(request.quotes));
                    }
                }
            }
        }

        let mut state = state.lock().unwrap();
        let seen_requests_before = state.seen_requests.len();

        // In real implementation we'd have timestamps for each entry
        // For now we'll just limit the total size of the set
        const MAX_SEEN_REQUESTS: usize = 10000;
        if state.seen_requests.len() > MAX_SEEN_REQUESTS {
            // Create a new empty set and replace the old one
            state.seen_requests = HashSet::new();
            debug!(
                "Pruned seen requests cache (was {} entries)",
                seen_requests_before
            );
        }
    }

    /// Returns a copy of the current pricing models
    ///
    /// This method returns a clone of the pricing models vector, which can be useful
    /// for inspection and monitoring purposes.
    ///
    /// # Returns
    ///
    /// A vector of PricingModel objects that represent all currently loaded models
    pub fn get_pricing_models(&self) -> Vec<PricingModel> {
        let state = self.state.lock().unwrap();
        state.pricing_models.clone()
    }

    /// Check if a blueprint is supported by this processor
    fn supports_blueprint(&self, blueprint_id: &str) -> bool {
        let state = self.state.lock().expect("Failed to lock state");
        state
            .pricing_models
            .iter()
            .any(|m| m.blueprint_id == blueprint_id)
    }

    /// Listen for and process incoming network messages
    ///
    /// This method runs in a background task and continuously listens for
    /// incoming messages from the network, deserializes them, and sends
    /// them to the processor for handling.
    async fn network_message_listener(
        command_tx: mpsc::Sender<RfqCommand<K>>,
        network_handle: &mut NetworkServiceHandle<K>, // Take a mutable reference
    ) {
        info!("Network message listener started");

        while let Some(message) = network_handle.next_protocol_message() {
            match message {
                ProtocolMessage {
                    protocol,
                    routing,
                    payload,
                } => {
                    if protocol != RFQ_TOPIC_NAME {
                        continue;
                    }

                    // Try to deserialize as an RFQ message
                    match bincode::deserialize::<RfqMessage<K>>(&payload) {
                        Ok(rfq_message) => {
                            info!(
                                "Received network message: {} from {}",
                                rfq_message.debug_info(),
                                routing.sender,
                            );

                            // Send to processor for handling
                            if let Err(e) = command_tx
                                .send(RfqCommand::ProcessMessage {
                                    message: rfq_message,
                                    source: routing.sender,
                                })
                                .await
                            {
                                error!("Failed to forward network message to processor: {}", e);
                            }
                        }
                        Err(e) => {
                            debug!("Received non-RFQ message or failed to deserialize: {}", e);
                        }
                    }
                }
            }
        }

        warn!(
            "Network message listener exited, RFQ processor will no longer receive network messages"
        );
    }
}
