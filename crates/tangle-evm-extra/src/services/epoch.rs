//! Epoch keeper for InflationPool distribution automation
//!
//! Monitors the InflationPool contract and triggers epoch distribution
//! when the epoch is ready (time has passed since last distribution).

use super::keeper::{BackgroundKeeper, KeeperConfig, KeeperError, KeeperHandle, KeeperResult};
use alloy::sol;
use blueprint_core::{debug, info, warn};
use tokio::sync::broadcast;

// Define the InflationPool interface with only the functions we need
sol! {
    #[sol(rpc)]
    interface IInflationPool {
        /// Check if epoch is ready for distribution
        function isEpochReady() external view returns (bool);

        /// Get seconds until next epoch distribution
        function secondsUntilNextEpoch() external view returns (uint256);

        /// Get current epoch number
        function currentEpoch() external view returns (uint256);

        /// Distribute rewards for current epoch
        function distributeEpoch() external;

        /// Emitted when epoch distribution is ready
        event EpochStarted(uint256 indexed epoch, uint256 distributionReadyAt, uint256 estimatedBudget);

        /// Emitted when epoch is distributed
        event EpochDistributed(
            uint256 indexed epoch,
            uint256 stakingAmount,
            uint256 operatorsAmount,
            uint256 customersAmount,
            uint256 restakersAmount,
            uint256 totalDistributed
        );
    }
}

/// Keeper that monitors and triggers epoch distributions on InflationPool
pub struct EpochKeeper;

impl BackgroundKeeper for EpochKeeper {
    const NAME: &'static str = "EpochKeeper";

    fn start(config: KeeperConfig, mut shutdown: broadcast::Receiver<()>) -> KeeperHandle {
        let handle = tokio::spawn(async move {
            info!("[{}] Starting epoch keeper", Self::NAME);

            let inflation_pool = config.inflation_pool.ok_or_else(|| {
                KeeperError::Config("InflationPool address not configured".into())
            })?;

            info!(
                "[{}] Monitoring InflationPool at {}",
                Self::NAME,
                inflation_pool
            );

            loop {
                tokio::select! {
                    _ = shutdown.recv() => {
                        info!("[{}] Received shutdown signal", Self::NAME);
                        break;
                    }
                    _ = tokio::time::sleep(config.epoch_check_interval) => {
                        match Self::check_and_execute(&config).await {
                            Ok(true) => info!("[{}] Epoch distribution triggered", Self::NAME),
                            Ok(false) => debug!("[{}] Epoch not ready yet", Self::NAME),
                            Err(e) => warn!("[{}] Error checking epoch: {}", Self::NAME, e),
                        }
                    }
                }
            }

            info!("[{}] Keeper stopped", Self::NAME);
            Ok(())
        });

        KeeperHandle {
            handle,
            name: Self::NAME,
        }
    }

    async fn check_and_execute(config: &KeeperConfig) -> KeeperResult<bool> {
        let inflation_pool = config.inflation_pool.ok_or_else(|| {
            KeeperError::Config("InflationPool address not configured".into())
        })?;

        // First check if epoch is ready (read-only)
        let read_provider = config.get_read_provider().await?;
        let pool = IInflationPool::new(inflation_pool, read_provider);

        let is_ready = pool
            .isEpochReady()
            .call()
            .await
            .map_err(|e| KeeperError::Contract(format!("Failed to check epoch ready: {}", e)))?;

        if !is_ready {
            // Log time until next epoch for debugging
            if let Ok(seconds) = pool.secondsUntilNextEpoch().call().await {
                debug!(
                    "[{}] Epoch not ready, {} seconds until next distribution",
                    EpochKeeper::NAME,
                    seconds
                );
            }
            return Ok(false);
        }

        // Epoch is ready, get current epoch for logging
        let current_epoch = pool
            .currentEpoch()
            .call()
            .await
            .map_err(|e| KeeperError::Contract(format!("Failed to get current epoch: {}", e)))?;

        info!(
            "[{}] Epoch {} is ready for distribution, submitting transaction",
            EpochKeeper::NAME,
            current_epoch
        );

        // Submit distribution transaction
        let provider = config.get_provider().await?;
        let pool = IInflationPool::new(inflation_pool, provider);

        let receipt = pool
            .distributeEpoch()
            .send()
            .await
            .map_err(|e| KeeperError::Transaction(format!("Failed to send distributeEpoch: {}", e)))?
            .get_receipt()
            .await
            .map_err(|e| {
                KeeperError::Transaction(format!("Failed to get distributeEpoch receipt: {}", e))
            })?;

        info!(
            "[{}] Epoch {} distributed successfully, tx: {:?}",
            EpochKeeper::NAME,
            current_epoch,
            receipt.transaction_hash
        );

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_keeper_name() {
        assert_eq!(EpochKeeper::NAME, "EpochKeeper");
    }
}
