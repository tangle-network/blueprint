//! Round keeper for MultiAssetDelegation round advancement automation
//!
//! Monitors the MultiAssetDelegation contract and advances rounds
//! when enough time has passed since the last round advancement.

use super::keeper::{BackgroundKeeper, KeeperConfig, KeeperError, KeeperHandle, KeeperResult};
use alloy::sol;
use blueprint_core::{debug, info, warn};
use tokio::sync::broadcast;

// Define the MultiAssetDelegation interface with only the functions we need
sol! {
    #[sol(rpc)]
    interface IMultiAssetDelegationRounds {
        /// Get current round number
        function currentRound() external view returns (uint64);

        /// Get round duration in seconds
        function roundDuration() external view returns (uint64);

        /// Get timestamp of last round advancement
        function lastRoundAdvance() external view returns (uint64);

        /// Advance to next round (reverts if too early)
        function advanceRound() external;

        /// Emitted when round is advanced
        event RoundAdvanced(uint64 indexed round);
    }
}

/// Keeper that monitors and advances rounds on MultiAssetDelegation
pub struct RoundKeeper;

impl RoundKeeper {}

impl BackgroundKeeper for RoundKeeper {
    const NAME: &'static str = "RoundKeeper";

    fn start(config: KeeperConfig, mut shutdown: broadcast::Receiver<()>) -> KeeperHandle {
        let handle = tokio::spawn(async move {
            info!("[{}] Starting round keeper", Self::NAME);

            let mad_address = config.multi_asset_delegation.ok_or_else(|| {
                KeeperError::Config("MultiAssetDelegation address not configured".into())
            })?;

            info!(
                "[{}] Monitoring MultiAssetDelegation at {}",
                Self::NAME,
                mad_address
            );

            loop {
                tokio::select! {
                    _ = shutdown.recv() => {
                        info!("[{}] Received shutdown signal", Self::NAME);
                        break;
                    }
                    _ = tokio::time::sleep(config.round_check_interval) => {
                        match Self::check_and_execute(&config).await {
                            Ok(true) => info!("[{}] Round advanced", Self::NAME),
                            Ok(false) => debug!("[{}] Round not ready yet", Self::NAME),
                            Err(e) => warn!("[{}] Error checking round: {}", Self::NAME, e),
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
        let mad_address = config.multi_asset_delegation.ok_or_else(|| {
            KeeperError::Config("MultiAssetDelegation address not configured".into())
        })?;

        // Check if round can be advanced (read-only)
        let read_provider = config.get_read_provider().await?;
        let contract = IMultiAssetDelegationRounds::new(mad_address, read_provider);

        // Check timing
        let last_advance: u64 =
            contract.lastRoundAdvance().call().await.map_err(|e| {
                KeeperError::Contract(format!("Failed to get lastRoundAdvance: {}", e))
            })?;

        let duration: u64 =
            contract.roundDuration().call().await.map_err(|e| {
                KeeperError::Contract(format!("Failed to get roundDuration: {}", e))
            })?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| KeeperError::Config(format!("System time error: {}", e)))?
            .as_secs();

        // Check if we can advance (first round or enough time passed)
        let can_advance = last_advance == 0 || now >= (last_advance + duration);

        if !can_advance {
            let current: u64 =
                contract.currentRound().call().await.map_err(|e| {
                    KeeperError::Contract(format!("Failed to get currentRound: {}", e))
                })?;

            let ready_at = last_advance + duration;
            let remaining = ready_at.saturating_sub(now);
            debug!(
                "[{}] Round {} not ready, {} seconds remaining",
                RoundKeeper::NAME,
                current,
                remaining
            );

            return Ok(false);
        }

        // Get current round for logging
        let current_round: u64 = contract
            .currentRound()
            .call()
            .await
            .map_err(|e| KeeperError::Contract(format!("Failed to get currentRound: {}", e)))?;

        info!(
            "[{}] Round {} can be advanced, submitting transaction",
            RoundKeeper::NAME,
            current_round
        );

        // Submit advance round transaction
        let provider = config.get_provider().await?;
        let contract = IMultiAssetDelegationRounds::new(mad_address, provider);

        let receipt = contract
            .advanceRound()
            .send()
            .await
            .map_err(|e| KeeperError::Transaction(format!("Failed to send advanceRound: {}", e)))?
            .get_receipt()
            .await
            .map_err(|e| {
                KeeperError::Transaction(format!("Failed to get advanceRound receipt: {}", e))
            })?;

        info!(
            "[{}] Advanced to round {}, tx: {:?}",
            RoundKeeper::NAME,
            current_round + 1,
            receipt.transaction_hash
        );

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_keeper_name() {
        assert_eq!(RoundKeeper::NAME, "RoundKeeper");
    }
}
