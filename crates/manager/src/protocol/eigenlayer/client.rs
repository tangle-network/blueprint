/// EigenLayer Protocol Client
///
/// Handles connection to EigenLayer AVS and streams EVM events.
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::protocol::types::{EigenlayerProtocolEvent, ProtocolEvent};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use blueprint_core::{debug, info, warn};
use blueprint_runner::config::BlueprintEnvironment;
use std::time::Duration;
use tokio::time::sleep;

/// EigenLayer protocol client implementation
///
/// This client polls for new blocks from an EVM RPC endpoint and streams logs.
/// Unlike Tangle's finality notifications, EVM uses a polling model.
pub struct EigenlayerProtocolClient {
    provider: Box<dyn Provider + Send + Sync>,
    last_block: u64,
    poll_interval: Duration,
    contract_addresses: Vec<alloy_primitives::Address>,
}

impl EigenlayerProtocolClient {
    /// Create a new EigenLayer protocol client
    pub async fn new(env: BlueprintEnvironment, _ctx: &BlueprintManagerContext) -> Result<Self> {
        // Get HTTP RPC URL from environment - it's already a url::Url
        let http_rpc_url = env.http_rpc_endpoint.clone();

        // Create alloy provider using the URL string
        let provider = ProviderBuilder::new().connect_http(http_rpc_url);

        // Get the current block number to start from
        let current_block = provider
            .get_block_number()
            .await
            .map_err(|e| Error::Other(format!("Failed to get current block number: {e}")))?;

        info!("EigenLayer client initialized at block {}", current_block);

        // TODO: Get contract addresses from environment or config
        // For now, we'll poll all logs (empty filter)
        let contract_addresses = Vec::new();

        Ok(Self {
            provider: Box::new(provider),
            last_block: current_block,
            poll_interval: Duration::from_secs(12), // Ethereum block time
            contract_addresses,
        })
    }

    /// Poll for new blocks and return logs
    async fn poll_next_block(&mut self) -> Result<Option<ProtocolEvent>> {
        // Sleep for poll interval
        sleep(self.poll_interval).await;

        // Get the latest block number
        let latest_block = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| Error::Other(format!("Failed to get latest block: {e}")))?;

        // Check if we have a new block
        if latest_block <= self.last_block {
            debug!(
                "No new blocks (current: {}, latest: {})",
                self.last_block, latest_block
            );
            return Ok(None);
        }

        // Move to next block
        let block_number = self.last_block + 1;
        self.last_block = block_number;

        info!("Processing EigenLayer block {}", block_number);

        // Get the block details
        let block = self
            .provider
            .get_block_by_number(BlockNumberOrTag::Number(block_number))
            .await
            .map_err(|e| Error::Other(format!("Failed to get block {}: {e}", block_number)))?
            .ok_or_else(|| Error::Other(format!("Block {} not found", block_number)))?;

        let block_hash = block.header.hash;

        // Create filter for logs in this block
        let filter = Filter::new().at_block_hash(block_hash);

        // If we have specific contract addresses, filter by them
        let filter = if self.contract_addresses.is_empty() {
            filter
        } else {
            filter.address(self.contract_addresses.clone())
        };

        // Get logs for this block
        let logs = self.provider.get_logs(&filter).await.map_err(|e| {
            Error::Other(format!(
                "Failed to get logs for block {}: {e}",
                block_number
            ))
        })?;

        debug!("Found {} logs in block {}", logs.len(), block_number);

        // Create protocol event
        Ok(Some(ProtocolEvent::Eigenlayer(EigenlayerProtocolEvent {
            block_number,
            block_hash: block_hash.0.to_vec(),
            logs,
        })))
    }
}

impl EigenlayerProtocolClient {
    /// Get the next protocol event from EigenLayer
    ///
    /// This method polls for new blocks and returns logs when a new block is available.
    pub async fn next_event(&mut self) -> Option<ProtocolEvent> {
        loop {
            match self.poll_next_block().await {
                Ok(Some(event)) => return Some(event),
                Ok(None) => {
                    // No new block yet, keep polling
                }
                Err(e) => {
                    warn!("Error polling EigenLayer blocks: {}", e);
                    sleep(self.poll_interval).await;
                }
            }
        }
    }
}
