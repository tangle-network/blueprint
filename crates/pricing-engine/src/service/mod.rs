//! Service module for the Tangle Cloud Pricing Engine
//!
//! This module provides the main service orchestration for the pricing engine,
//! including blockchain integration and RPC services for a single operator.

pub mod blockchain;
pub mod rpc;

use crate::{
    error::{Error, Result},
    models::PricingModel,
    rfq::{RfqProcessor, RfqProcessorConfig},
};
use blockchain::{event::BlockchainEvent, listener::EventListener};
use blueprint_crypto::KeyType;
use blueprint_networking::service_handle::NetworkServiceHandle;
use rpc::{OperatorInfo, server::RpcServer};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};

/// Service state enum for lifecycle management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    /// Service is initializing
    Initializing,
    /// Service is running
    Running,
    /// Service is shutting down
    ShuttingDown,
    /// Service has shut down
    ShutDown,
}

/// Command enum for the service control channel
enum ServiceCommand {
    /// Stop the service
    Stop(oneshot::Sender<()>),
}

/// Configuration for the pricing engine service
pub struct ServiceConfig<K: KeyType> {
    /// RPC server address
    pub rpc_addr: SocketAddr,
    /// Substrate node websocket URL
    pub node_url: Option<String>,
    /// Path to the keystore for signing transactions
    pub keystore_path: Option<String>,
    /// Operator name
    pub operator_name: String,
    /// Operator description
    pub operator_description: Option<String>,
    /// Operator public key (on-chain identity)
    pub operator_public_key: K::Public,
    /// Supported blueprints
    pub supported_blueprints: Vec<String>,
    /// Network service handle for RFQ functionality
    pub network_handle: Option<Arc<NetworkServiceHandle<K>>>,
}

/// The main pricing engine service
pub struct Service<K: KeyType> {
    /// The signing key for the operator
    signing_key: K::Secret,

    /// Current state of the service
    state: ServiceState,

    /// The operator information
    operator_info: OperatorInfo<K>,

    /// Available pricing models
    pricing_models: Vec<PricingModel>,

    /// The blockchain event listener
    event_listener: Option<Arc<EventListener>>,

    /// The RFQ processor for handling quote requests
    rfq_processor: Option<Arc<RfqProcessor<K>>>,

    /// RPC server handle
    rpc_server: Option<jsonrpsee::server::ServerHandle>,

    /// Command channel for service control
    command_tx: mpsc::Sender<ServiceCommand>,
    command_rx: Option<mpsc::Receiver<ServiceCommand>>,

    /// Channel for blockchain events
    event_tx: mpsc::Sender<BlockchainEvent>,
    event_rx: Option<mpsc::Receiver<BlockchainEvent>>,

    /// Service configuration
    config: ServiceConfig<K>,
}

impl<K: KeyType> Service<K> {
    /// Create a new pricing engine service
    pub fn new(
        config: ServiceConfig<K>,
        initial_models: Vec<PricingModel>,
        signing_key: K::Secret,
    ) -> Self {
        let (command_tx, command_rx) = mpsc::channel(32);
        let (event_tx, event_rx) = mpsc::channel(128);

        // Create operator info directly from config
        let operator_info = OperatorInfo::<K> {
            public_key: K::public_from_secret(&signing_key),
            name: config.operator_name.clone(),
            description: config.operator_description.clone(),
            supported_blueprints: config.supported_blueprints.clone(),
        };

        Self {
            signing_key,
            state: ServiceState::Initializing,
            operator_info,
            pricing_models: initial_models,
            event_listener: None,
            rfq_processor: None,
            rpc_server: None,
            command_tx,
            command_rx: Some(command_rx),
            event_tx,
            event_rx: Some(event_rx),
            config,
        }
    }

    /// Start the blockchain event listener component
    ///
    /// This method establishes a connection to the blockchain node and starts
    /// listening for relevant events.
    ///
    /// # Returns
    /// `Ok(())` if the blockchain listener was started successfully, or
    /// an error if the connection failed.
    pub async fn start_blockchain_listener(&mut self) -> Result<()> {
        if self.event_listener.is_some() {
            // Already started
            info!("Blockchain listener already running");
            return Ok(());
        }

        // Ensure we have a node URL to connect to
        let node_url = match &self.config.node_url {
            Some(url) => url.clone(),
            None => return Err(Error::Other("No blockchain node URL provided".to_string())),
        };

        info!("Starting blockchain event listener for node: {}", node_url);

        // Create and initialize the event listener
        let event_listener = EventListener::new(node_url, self.event_tx.clone())
            .await
            .map_err(|e| Error::ChainConnection(e.to_string()))?;

        // Start listening for blockchain events
        let event_listener = Arc::new(event_listener);
        let listener_clone = event_listener.clone();
        tokio::spawn(async move {
            if let Err(e) = listener_clone.run().await {
                error!("Blockchain event listener error: {}", e);
            }
        });

        self.event_listener = Some(event_listener);
        info!("Blockchain event listener started successfully");

        // Start the event handler for processing blockchain events
        if let Some(event_rx) = self.event_rx.take() {
            tokio::spawn(async move {
                Self::handle_events(event_rx).await;
            });
            info!("Blockchain event processor started");
        }

        Ok(())
    }

    /// Start the server components (RPC and networking)
    ///
    /// This method starts the RPC server and initializes the RFQ processor
    /// for handling quote requests.
    ///
    /// # Returns
    /// `Ok(())` if the server components were started successfully, or
    /// an error if any component failed to start.
    pub async fn start_server(&mut self) -> Result<()> {
        // Initialize and start the RFQ processor first if network handle is provided
        if let Some(network_handle) = &self.config.network_handle {
            info!("Initializing RFQ processor");

            // Get the public key from the signing key
            let public_key = K::public_from_secret(&self.signing_key);

            // Configure the RFQ processor
            let rfq_config = RfqProcessorConfig {
                local_peer_id: network_handle.local_peer_id,
                operator_name: self.config.operator_name.clone(),
                pricing_models: self.pricing_models.clone(),
                provider_public_key: Some(public_key), // Set the public key explicitly
                ..Default::default()
            };

            // Create the RFQ processor with the network handle
            // The network handle is cloned and wrapped in an Arc inside RfqProcessor::new
            let handle_clone = (**network_handle).clone();
            let rfq_processor =
                RfqProcessor::new(rfq_config, self.signing_key.clone(), handle_clone);

            // Store the processor wrapped in an Arc
            let processor_arc = Arc::new(rfq_processor);
            self.rfq_processor = Some(processor_arc);

            info!("RFQ processor started with network handle");
        } else {
            info!("No network handle provided, RFQ functionality disabled");
        }

        // Start the RPC server with the RFQ processor
        info!("Starting RPC server at {}", self.config.rpc_addr);
        let mut rpc_server =
            RpcServer::new(self.operator_info.clone(), self.pricing_models.clone());

        // Connect the RFQ processor to the RPC server if available
        if let Some(rfq_processor) = &self.rfq_processor {
            rpc_server = rpc_server.with_rfq_processor(rfq_processor.clone());
        }

        // Bind the server to the specified address
        let server_handle = rpc_server.start(self.config.rpc_addr).await?;
        self.rpc_server = Some(server_handle);

        info!("RPC server started successfully");
        self.state = ServiceState::Running;

        Ok(())
    }

    /// Start the complete pricing engine service
    ///
    /// This method starts both the server components and attempts to start
    /// the blockchain listener. If the blockchain listener fails to start,
    /// the service will continue to operate in offline mode.
    ///
    /// # Returns
    /// `Ok(())` if the service started successfully (even in offline mode),
    /// or an error if the server components failed to start.
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting Tangle Cloud Pricing Engine");

        // Always start the server components (RPC and networking)
        self.start_server().await?;

        // Attempt to start the blockchain listener if a node URL is provided
        if self.config.node_url.is_some() {
            match self.start_blockchain_listener().await {
                Ok(_) => info!("Service running with blockchain integration"),
                Err(e) => info!("Service running in offline mode: {}", e),
            }
        } else {
            info!("Service running in offline mode (no blockchain node URL provided)");

            // Since we're not starting the blockchain listener, we need to
            // discard the event receiver to avoid resource leaks
            self.event_rx.take();
        }

        // Mark the service as running
        self.state = ServiceState::Running;
        info!("Tangle Cloud Pricing Engine started successfully");

        Ok(())
    }

    /// Run the service until it is stopped
    pub async fn run_until_stopped(&mut self) -> Result<()> {
        // Wait for a stop command
        if let Some(mut command_rx) = self.command_rx.take() {
            while let Some(command) = command_rx.recv().await {
                match command {
                    ServiceCommand::Stop(sender) => {
                        info!("Stopping service");
                        self.state = ServiceState::ShuttingDown;

                        // Clean up resources
                        if let Some(rfq_processor) = &self.rfq_processor {
                            if let Err(e) = rfq_processor.shutdown().await {
                                error!("Error shutting down RFQ processor: {}", e);
                            }
                        }

                        self.state = ServiceState::ShutDown;
                        let _ = sender.send(());
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Stop the service
    pub async fn stop(&self) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(ServiceCommand::Stop(tx))
            .await
            .map_err(|_| Error::ServiceShutdown("Failed to send stop command".to_string()))?;

        rx.await.map_err(|_| {
            Error::ServiceShutdown("Failed to receive stop confirmation".to_string())
        })?;

        info!("Service stopped");
        Ok(())
    }

    /// Send a request for quotes
    pub async fn send_rfq_request(
        &self,
        blueprint_id: String,
        requirements: Vec<crate::types::ResourceRequirement>,
    ) -> Result<Vec<crate::rfq::SignedPriceQuote<K>>> {
        if let Some(rfq) = &self.rfq_processor {
            rfq.send_request(blueprint_id, requirements)
                .await
                .map_err(|e| Error::Other(format!("RFQ error: {}", e)))
        } else {
            Err(Error::Other("RFQ functionality not enabled".to_string()))
        }
    }

    /// Get the RFQ processor
    pub fn rfq_processor(&self) -> Option<&Arc<RfqProcessor<K>>> {
        self.rfq_processor.as_ref()
    }

    /// Handle blockchain events
    async fn handle_events(mut event_rx: mpsc::Receiver<BlockchainEvent>) {
        while let Some(event) = event_rx.recv().await {
            debug!("Received blockchain event: {:?}", event);

            // Process events based on their type
            match &event {
                BlockchainEvent::Registered(_registered) => {
                    // TODO: Update operator info
                }
                BlockchainEvent::Unregistered(_unregistered) => {
                    // TODO: Update operator info
                }
                BlockchainEvent::PriceTargetsUpdated(_price_targets_updated) => {
                    // TODO: Update pricing models
                }
                BlockchainEvent::ServiceRequested(_service_requested) => {
                    // TODO: Process service request
                }
                BlockchainEvent::ServiceRequestApproved(_service_request_approved) => {
                    // TODO: Process service request approval
                }
                BlockchainEvent::ServiceRequestRejected(_service_request_rejected) => {
                    // TODO: Process service request rejection
                }
                BlockchainEvent::ServiceTerminated(_service_terminated) => {
                    // TODO: Process service termination
                }
                BlockchainEvent::ServiceInitiated(_service_initiated) => {
                    // TODO: Process service initiation
                }
            }
        }
    }

    /// Get the current service state
    pub fn state(&self) -> ServiceState {
        self.state
    }

    /// Get the pricing models
    ///
    /// # Returns
    /// A clone of the current pricing models vector
    pub fn get_pricing_models(&self) -> Vec<PricingModel> {
        self.pricing_models.clone()
    }

    /// Add or update a pricing model
    pub fn update_pricing_model(&mut self, model: PricingModel) {
        // Check if we already have a model for this category
        let existing_index = self
            .pricing_models
            .iter()
            .position(|m| m.name == model.name);

        if let Some(index) = existing_index {
            // Update existing model
            self.pricing_models[index] = model;
        } else {
            // Add new model
            self.pricing_models.push(model);
        }

        // Update supported categories in operator info
        let supported_blueprints = self
            .pricing_models
            .iter()
            .map(|m| m.blueprint_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        self.operator_info.supported_blueprints = supported_blueprints;
    }
}
