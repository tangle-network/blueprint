//! Subscription billing keeper for Tangle v2 services
//!
//! Monitors subscription-based services and triggers billing when payment intervals
//! have elapsed. This keeper automates the `billSubscriptionBatch` call so that
//! operators and the protocol collect subscription fees on time.

use super::keeper::{BackgroundKeeper, KeeperConfig, KeeperError, KeeperHandle, KeeperResult};
use alloy::primitives::Address;
use alloy::sol;
use blueprint_core::{debug, info, warn};
use std::time::Instant;
use tokio::sync::broadcast;

// Define the Tangle interface with only the functions we call
sol! {
    #[sol(rpc)]
    interface ITangle {
        /// Get total number of services
        function serviceCount() external view returns (uint64);

        /// Get a service's details
        function getService(uint64 serviceId) external view returns (Service memory);

        /// Get billable services from a list of candidate IDs
        function getBillableServices(uint64[] calldata serviceIds) external view returns (uint64[] memory);

        /// Bill a batch of subscription services
        function billSubscriptionBatch(uint64[] calldata serviceIds) external returns (uint256 billed, uint256 failed);

        struct Service {
            uint64 blueprintId;
            address owner;
            uint64 createdAt;
            uint64 ttl;
            uint64 terminatedAt;
            uint64 lastPaymentAt;
            uint32 operatorCount;
            uint32 minOperators;
            uint32 maxOperators;
            uint8 membership;
            uint8 pricing;
            uint8 status;
        }
    }
}

/// Pricing model enum matching the Solidity Types.PricingModel
const PRICING_SUBSCRIPTION: u8 = 1;
/// Service status matching Types.ServiceStatus.Active
const STATUS_ACTIVE: u8 = 1;

/// Keeper that monitors subscription services and triggers billing
pub struct SubscriptionBillingKeeper;

impl BackgroundKeeper for SubscriptionBillingKeeper {
    const NAME: &'static str = "SubscriptionBillingKeeper";

    fn start(config: KeeperConfig, mut shutdown: broadcast::Receiver<()>) -> KeeperHandle {
        let handle = tokio::spawn(async move {
            info!("[{}] Starting subscription billing keeper", Self::NAME);

            let tangle_address = config.tangle_contract.ok_or_else(|| {
                KeeperError::Config("Tangle contract address not configured".into())
            })?;

            info!(
                "[{}] Monitoring Tangle contract at {}",
                Self::NAME,
                tangle_address
            );

            // Track subscription services we know about
            let mut tracked_service_ids: Vec<u64> = Vec::new();
            let mut last_rescan = Instant::now()
                .checked_sub(config.billing_rescan_interval)
                .unwrap_or_else(Instant::now);

            loop {
                tokio::select! {
                    _ = shutdown.recv() => {
                        info!("[{}] Received shutdown signal", Self::NAME);
                        break;
                    }
                    _ = tokio::time::sleep(config.billing_check_interval) => {
                        // Rescan for new subscription services if interval has passed
                        if last_rescan.elapsed() >= config.billing_rescan_interval {
                            match rescan_services(&config, tangle_address).await {
                                Ok(ids) => {
                                    if ids.len() != tracked_service_ids.len() {
                                        info!(
                                            "[{}] Tracking {} subscription services (was {})",
                                            Self::NAME, ids.len(), tracked_service_ids.len()
                                        );
                                    }
                                    tracked_service_ids = ids;
                                    last_rescan = Instant::now();
                                }
                                Err(e) => {
                                    warn!("[{}] Error rescanning services: {}", Self::NAME, e);
                                }
                            }
                        }

                        if tracked_service_ids.is_empty() {
                            debug!("[{}] No subscription services to bill", Self::NAME);
                            continue;
                        }

                        // Check and bill in batches
                        match check_and_bill(
                            &config,
                            tangle_address,
                            &tracked_service_ids,
                        ).await {
                            Ok(true) => info!("[{}] Billing executed successfully", Self::NAME),
                            Ok(false) => debug!("[{}] No services due for billing", Self::NAME),
                            Err(e) => warn!("[{}] Error during billing check: {}", Self::NAME, e),
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
        let tangle_address = config
            .tangle_contract
            .ok_or_else(|| KeeperError::Config("Tangle contract address not configured".into()))?;

        // For the trait implementation, do a full rescan + bill cycle
        let tracked = rescan_services(config, tangle_address).await?;
        if tracked.is_empty() {
            return Ok(false);
        }
        check_and_bill(config, tangle_address, &tracked).await
    }
}

/// Scan all services to find active subscriptions
async fn rescan_services(config: &KeeperConfig, tangle_address: Address) -> KeeperResult<Vec<u64>> {
    let provider = config.get_read_provider().await?;
    let tangle = ITangle::new(tangle_address, provider);

    let count = tangle
        .serviceCount()
        .call()
        .await
        .map_err(|e| KeeperError::Contract(format!("Failed to get service count: {}", e)))?;

    let total = count;
    let mut subscription_ids = Vec::new();

    for id in 0..total {
        match tangle.getService(id).call().await {
            Ok(svc) => {
                if svc.pricing == PRICING_SUBSCRIPTION && svc.status == STATUS_ACTIVE {
                    subscription_ids.push(id);
                }
            }
            Err(e) => {
                debug!(
                    "[{}] Failed to get service {}: {}",
                    SubscriptionBillingKeeper::NAME,
                    id,
                    e
                );
            }
        }
    }

    Ok(subscription_ids)
}

/// Check which services are billable and bill them
async fn check_and_bill(
    config: &KeeperConfig,
    tangle_address: Address,
    tracked_ids: &[u64],
) -> KeeperResult<bool> {
    if tracked_ids.is_empty() {
        return Ok(false);
    }

    let read_provider = config.get_read_provider().await?;
    let tangle_read = ITangle::new(tangle_address, read_provider);

    // Check which services are billable (read-only)
    let billable = tangle_read
        .getBillableServices(tracked_ids.to_vec())
        .call()
        .await
        .map_err(|e| KeeperError::Contract(format!("Failed to get billable services: {}", e)))?;

    let billable_ids = billable;
    if billable_ids.is_empty() {
        return Ok(false);
    }

    info!(
        "[{}] Found {} billable services, submitting billing transaction",
        SubscriptionBillingKeeper::NAME,
        billable_ids.len()
    );

    // Bill in batches
    let provider = config.get_provider().await?;
    let tangle = ITangle::new(tangle_address, provider);
    let mut any_billed = false;

    for chunk in billable_ids.chunks(config.billing_max_batch_size) {
        let batch: Vec<u64> = chunk.to_vec();
        match tangle.billSubscriptionBatch(batch.clone()).send().await {
            Ok(pending) => match pending.get_receipt().await {
                Ok(receipt) => {
                    info!(
                        "[{}] Billed {} services, tx: {:?}",
                        SubscriptionBillingKeeper::NAME,
                        batch.len(),
                        receipt.transaction_hash
                    );
                    any_billed = true;
                }
                Err(e) => {
                    warn!(
                        "[{}] Failed to get billing receipt: {}",
                        SubscriptionBillingKeeper::NAME,
                        e
                    );
                }
            },
            Err(e) => {
                // Race condition: another operator may have billed first. This is harmless.
                warn!(
                    "[{}] Failed to send billing tx: {}",
                    SubscriptionBillingKeeper::NAME,
                    e
                );
            }
        }
    }

    Ok(any_billed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_keeper_name() {
        assert_eq!(SubscriptionBillingKeeper::NAME, "SubscriptionBillingKeeper");
    }
}
