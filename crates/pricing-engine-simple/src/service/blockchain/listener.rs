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
use tokio::sync::mpsc::Sender;
use tracing::{debug, error, info, warn};

use tangle_subxt::subxt::{OnlineClient, PolkadotConfig};

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
                                if let Err(e) = self.event_tx.send(event).await {
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
}
