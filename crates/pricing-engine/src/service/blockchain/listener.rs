//! Blockchain event listener for the Tangle Cloud Pricing Engine
//!
//! This module implements a listener for blockchain events using the tangle-subxt library,
//! which provides a type-safe interface for interacting with Tangle Network.

use std::sync::Arc;

use crate::service::blockchain::event::handle_events;

use super::{
    event::BlockchainEvent,
    types::{SubxtError, TangleClient},
};
use crossbeam_channel::Sender;
use futures::stream::StreamExt;
use tracing::{debug, error, info, warn};

// Import tangle-subxt
use tangle_subxt::{
    self as tangle,
    subxt::{OnlineClient, PolkadotConfig},
};

/// Listener for blockchain events
pub struct EventListener {
    /// Connection to the blockchain node
    client: TangleClient,
    /// Event channel sender
    event_tx: Sender<BlockchainEvent>,
}

impl EventListener {
    /// Create a new event listener
    pub async fn new(
        node_url: String,
        event_tx: Sender<BlockchainEvent>,
    ) -> Result<Self, SubxtError> {
        info!("Connecting to Tangle node at {}", node_url);
        let client = OnlineClient::<PolkadotConfig>::from_url(&node_url).await?;

        Ok(Self {
            client: Arc::new(client),
            event_tx,
        })
    }

    /// Start listening for events
    pub async fn run(&self) -> Result<(), SubxtError> {
        info!("Starting blockchain event listener");

        // Subscribe to finalized blocks
        let mut blocks_sub = self.client.blocks().subscribe_finalized().await?;
        info!("Subscribed to finalized blocks");

        // Process finalized blocks
        while let Some(block_result) = blocks_sub.next().await {
            match block_result {
                Ok(block) => {
                    let block_number = block.header().number;
                    let block_hash = block.hash();
                    debug!(
                        "Processing finalized block #{} ({})",
                        block_number, block_hash
                    );

                    // Get events for this block
                    match block.events().await {
                        Ok(events) => {
                            let blockchain_events = handle_events(events).await;
                            for event in blockchain_events {
                                if let Err(e) = self.event_tx.send(event) {
                                    error!("Failed to send event: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error getting events for block {}: {}", block_number, e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error getting block: {}", e);
                }
            }
        }

        warn!("Block subscription ended, stopping listener");
        Ok(())
    }

    /// Handle a Services pallet event
    async fn handle_services_event(
        &self,
        event: tangle::subxt::events::EventDetails<tangle::subxt::PolkadotConfig>,
        block_number: u32,
    ) {
        let variant_name = event.variant_name();

        match variant_name {
            "Registered" => {
                // Extract fields from the Registered event
                let fields = match event.field_values() {
                    Ok(fields) => fields,
                    Err(e) => {
                        error!("Failed to get field values for Registered event: {}", e);
                        return;
                    }
                };

                // In a real implementation, we would extract the actual values from fields
                // For now, we'll create a placeholder event
                let provider = format!("provider-{}", block_number);
                let blueprint_id = block_number as u64;

                let blockchain_event = BlockchainEvent::Registered {
                    provider,
                    blueprint_id,
                    preferences: OperatorPreferences {
                        auto_approve_threshold: Some(1000),
                        max_concurrent_services: 10,
                    },
                    registration_args: vec![],
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send Registered event: {}", e);
                }
            }
            "Unregistered" => {
                // Extract fields from the Unregistered event
                let operator = format!("operator-{}", block_number);
                let blueprint_id = block_number as u64;

                let blockchain_event = BlockchainEvent::Unregistered {
                    operator,
                    blueprint_id,
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send Unregistered event: {}", e);
                }
            }
            "PriceTargetsUpdated" => {
                // Extract fields from the PriceTargetsUpdated event
                let operator = format!("operator-{}", block_number);
                let blueprint_id = block_number as u64;

                let blockchain_event = BlockchainEvent::PriceTargetsUpdated {
                    operator,
                    blueprint_id,
                    price_targets: PriceTargets {
                        base_price: 100,
                        resource_multipliers: vec![
                            ("CPU".to_string(), 10),
                            ("Memory".to_string(), 5),
                        ],
                    },
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send PriceTargetsUpdated event: {}", e);
                }
            }
            "ServiceRequested" => {
                // Extract fields from the ServiceRequested event
                let owner = format!("owner-{}", block_number % 100);
                let request_id = block_number as u64;
                let blueprint_id = (block_number % 10) as u64;

                let blockchain_event = BlockchainEvent::ServiceRequested {
                    owner,
                    request_id,
                    blueprint_id,
                    pending_approvals: vec![format!("operator-{}", block_number % 5)],
                    approved: vec![],
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send ServiceRequested event: {}", e);
                }
            }
            "ServiceRequestApproved" => {
                // Extract fields from the ServiceRequestApproved event
                let operator = format!("operator-{}", block_number % 5);
                let request_id = block_number as u64;
                let blueprint_id = (block_number % 10) as u64;

                let blockchain_event = BlockchainEvent::ServiceRequestApproved {
                    operator,
                    request_id,
                    blueprint_id,
                    pending_approvals: vec![],
                    approved: vec![operator.clone()],
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send ServiceRequestApproved event: {}", e);
                }
            }
            "ServiceRequestRejected" => {
                // Extract fields from the ServiceRequestRejected event
                let operator = format!("operator-{}", block_number % 5);
                let request_id = block_number as u64;
                let blueprint_id = (block_number % 10) as u64;

                let blockchain_event = BlockchainEvent::ServiceRequestRejected {
                    operator,
                    request_id,
                    blueprint_id,
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send ServiceRequestRejected event: {}", e);
                }
            }
            "ServiceInitiated" => {
                // Extract fields from the ServiceInitiated event
                let owner = format!("owner-{}", block_number % 100);
                let request_id = (block_number - 1) as u64;
                let service_id = block_number as u64;
                let blueprint_id = (block_number % 10) as u64;

                let blockchain_event = BlockchainEvent::ServiceInitiated {
                    owner,
                    request_id,
                    service_id,
                    blueprint_id,
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send ServiceInitiated event: {}", e);
                }
            }
            "ServiceTerminated" => {
                // Extract fields from the ServiceTerminated event
                let owner = format!("owner-{}", block_number % 100);
                let service_id = (block_number - 10) as u64;
                let blueprint_id = (block_number % 10) as u64;

                let blockchain_event = BlockchainEvent::ServiceTerminated {
                    owner,
                    service_id,
                    blueprint_id,
                };

                if let Err(e) = self.event_tx.send(blockchain_event).await {
                    error!("Failed to send ServiceTerminated event: {}", e);
                }
            }
            _ => {
                debug!("Ignoring Services event: {}", variant_name);
            }
        }
    }

    /// Submit a transaction to approve a service request using the tangle-subxt API
    pub async fn approve_service_request(
        &self,
        signer: &impl tangle::subxt::tx::Signer<tangle::subxt::PolkadotConfig>,
        request_id: u64,
        price: u64,
    ) -> Result<String, SubxtError> {
        info!(
            "Approving service request {} with price {}",
            request_id, price
        );

        // NOTE: In a real implementation, this would directly use the tangle-subxt API
        // to create the transaction, like:
        //
        // let tx = tangle::tx().services().approve_service_request(request_id, price);
        // let tx_hash = self.client.tx().sign_and_submit_default(&tx, signer).await?;
        // return Ok(tx_hash.to_string());

        // For now, return a placeholder
        Ok(format!("0x{:x}", price))
    }

    /// Submit a transaction to reject a service request using the tangle-subxt API
    pub async fn reject_service_request(
        &self,
        signer: &impl tangle::subxt::tx::Signer<tangle::subxt::PolkadotConfig>,
        request_id: u64,
        reason: String,
    ) -> Result<String, SubxtError> {
        info!(
            "Rejecting service request {} with reason: {}",
            request_id, reason
        );

        // NOTE: In a real implementation, this would directly use the tangle-subxt API
        // to create the transaction, like:
        //
        // let tx = tangle::tx().services().reject_service_request(request_id);
        // let tx_hash = self.client.tx().sign_and_submit_default(&tx, signer).await?;
        // return Ok(tx_hash.to_string());

        // For now, return a placeholder
        Ok(format!("0x{:x}", request_id))
    }
}
