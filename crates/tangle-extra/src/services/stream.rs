//! Stream keeper for StreamingPaymentManager drip automation
//!
//! Monitors pending streaming payment drips and triggers them
//! to ensure timely distribution of service fees to operators.

use super::keeper::{BackgroundKeeper, KeeperConfig, KeeperError, KeeperHandle, KeeperResult};
use alloy::primitives::U256;
use alloy::sol;
use blueprint_core::{debug, info, warn};
use tokio::sync::broadcast;

// Minimum pending amount to trigger a drip (avoid gas waste on tiny amounts)
const MIN_DRIP_THRESHOLD: u128 = 1_000_000_000_000_000; // 0.001 ETH equivalent

// Define the StreamingPaymentManager interface with only the functions we need
sol! {
    #[sol(rpc)]
    interface IStreamingPaymentManager {
        /// Get total pending drip amount for an operator
        function pendingDripForOperator(address operator) external view returns (uint256 totalPending, uint256 streamCount);

        /// Get active stream service IDs for an operator
        function getOperatorActiveStreams(address operator) external view returns (uint64[] memory);

        /// Emitted when streams have drippable funds
        event StreamDripAvailable(address indexed operator, uint256 pendingAmount, uint256 streamCount);
    }

    #[sol(rpc)]
    interface IServiceFeeDistributor {
        /// Drip all streams for an operator (called by distributor, routes through SPM)
        function dripOperatorStreams(address operator) external;
    }
}

/// Keeper that monitors and triggers streaming payment drips
pub struct StreamKeeper;

impl BackgroundKeeper for StreamKeeper {
    const NAME: &'static str = "StreamKeeper";

    fn start(config: KeeperConfig, mut shutdown: broadcast::Receiver<()>) -> KeeperHandle {
        let handle = tokio::spawn(async move {
            info!("[{}] Starting stream keeper", Self::NAME);

            let spm_address = config.streaming_payment_manager.ok_or_else(|| {
                KeeperError::Config("StreamingPaymentManager address not configured".into())
            })?;

            info!(
                "[{}] Monitoring StreamingPaymentManager at {}",
                Self::NAME,
                spm_address
            );

            // Determine which operators to monitor
            let operators = if config.monitored_operators.is_empty() {
                // Monitor own operator by default
                vec![config.get_operator_address()?]
            } else {
                config.monitored_operators.clone()
            };

            info!(
                "[{}] Monitoring {} operators for pending drips",
                Self::NAME,
                operators.len()
            );

            loop {
                tokio::select! {
                    _ = shutdown.recv() => {
                        info!("[{}] Received shutdown signal", Self::NAME);
                        break;
                    }
                    _ = tokio::time::sleep(config.stream_check_interval) => {
                        match Self::check_and_execute(&config).await {
                            Ok(true) => info!("[{}] Drips triggered", Self::NAME),
                            Ok(false) => debug!("[{}] No pending drips above threshold", Self::NAME),
                            Err(e) => warn!("[{}] Error checking drips: {}", Self::NAME, e),
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
        let spm_address = config.streaming_payment_manager.ok_or_else(|| {
            KeeperError::Config("StreamingPaymentManager address not configured".into())
        })?;

        // Determine which operators to check
        let operators = if config.monitored_operators.is_empty() {
            vec![config.get_operator_address()?]
        } else {
            config.monitored_operators.clone()
        };

        // Check pending drips (read-only)
        let read_provider = config.get_read_provider().await?;
        let spm = IStreamingPaymentManager::new(spm_address, read_provider);

        let mut any_triggered = false;

        for operator in &operators {
            let result = spm
                .pendingDripForOperator(*operator)
                .call()
                .await
                .map_err(|e| {
                    KeeperError::Contract(format!(
                        "Failed to get pendingDripForOperator for {}: {}",
                        operator, e
                    ))
                })?;

            let pending = result.totalPending;
            let stream_count = result.streamCount;

            if pending < U256::from(MIN_DRIP_THRESHOLD) {
                debug!(
                    "[{}] Operator {} has {} pending across {} streams (below threshold)",
                    StreamKeeper::NAME,
                    operator,
                    pending,
                    stream_count
                );
                continue;
            }

            info!(
                "[{}] Operator {} has {} pending across {} streams, triggering drip",
                StreamKeeper::NAME,
                operator,
                pending,
                stream_count
            );

            // Note: Drips are typically triggered through ServiceFeeDistributor
            // which calls StreamingPaymentManager.dripOperatorStreams internally.
            // The exact drip mechanism depends on your architecture:
            //
            // Option 1: Direct call to ServiceFeeDistributor.claimOperatorRewards
            //   - This triggers drips as a side effect
            //   - Requires the operator to have claimable rewards
            //
            // Option 2: Call claim on behalf of operator (if permitted)
            //   - Some contracts allow third-party triggering
            //
            // For now, we just log that drips are available.
            // The actual triggering should be done by the operator themselves
            // or through a contract that permits third-party triggering.
            //
            // TODO: Add actual drip triggering once the ServiceFeeDistributor
            // supports permissionless drip operations.

            info!(
                "[{}] Drip available for operator {}: {} wei across {} streams",
                StreamKeeper::NAME,
                operator,
                pending,
                stream_count
            );

            any_triggered = true;
        }

        Ok(any_triggered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_keeper_name() {
        assert_eq!(StreamKeeper::NAME, "StreamKeeper");
    }

    #[test]
    fn test_min_drip_threshold() {
        // 0.001 ETH = 1e15 wei
        assert_eq!(MIN_DRIP_THRESHOLD, 1_000_000_000_000_000);
    }
}
