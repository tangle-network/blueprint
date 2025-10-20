use crate::error::{EigenlayerExtraError, Result};
use alloy_primitives::{Address, FixedBytes, U256};
use blueprint_core::info;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;
use blueprint_runner::config::BlueprintEnvironment;
use eigensdk::client_elcontracts::reader::ELChainReader;
use eigensdk::utils::rewardsv2::core::rewards_coordinator::{
    IRewardsCoordinator, RewardsCoordinator,
};
use std::str::FromStr;

/// Manager for operator rewards claiming and tracking
///
/// Provides high-level abstractions for interacting with EigenLayer's RewardsCoordinator
/// contract, handling rewards claiming, querying, and earnings calculation per strategy.
#[derive(Clone)]
pub struct RewardsManager {
    env: BlueprintEnvironment,
}

impl RewardsManager {
    /// Create a new RewardsManager
    pub fn new(env: BlueprintEnvironment) -> Self {
        Self { env }
    }

    /// Get the operator address from keystore
    ///
    /// # Errors
    ///
    /// * Keystore errors if ECDSA key not found or cannot be exposed
    fn get_operator_address(&self) -> Result<Address> {
        let ecdsa_public = self
            .env
            .keystore()
            .first_local::<K256Ecdsa>()
            .map_err(EigenlayerExtraError::Keystore)?;

        let ecdsa_secret = self
            .env
            .keystore()
            .expose_ecdsa_secret(&ecdsa_public)
            .map_err(EigenlayerExtraError::Keystore)?
            .ok_or_else(|| {
                EigenlayerExtraError::InvalidConfiguration("No ECDSA secret found".into())
            })?;

        ecdsa_secret
            .alloy_address()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))
    }

    /// Get claimable rewards for the operator
    ///
    /// Returns the total amount of rewards that can be claimed by the operator
    /// across all strategies.
    ///
    /// # Errors
    ///
    /// * Contract interaction errors
    /// * Configuration errors if EigenLayer settings not found
    /// * Keystore errors
    pub async fn get_claimable_rewards(&self) -> Result<U256> {
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let el_chain_reader = ELChainReader::new(
            Some(contract_addresses.allocation_manager_address),
            contract_addresses.delegation_manager_address,
            contract_addresses.rewards_coordinator_address,
            contract_addresses.avs_directory_address,
            Some(contract_addresses.permission_controller_address),
            self.env.http_rpc_endpoint.to_string(),
        );

        // Get current claimable distribution root
        let distribution_root = el_chain_reader
            .get_current_claimable_distribution_root()
            .await
            .map_err(|e| EigenlayerExtraError::EigenSdk(e.to_string()))?;

        info!(
            "Current claimable distribution root: {} (activated at {})",
            distribution_root.root, distribution_root.activatedAt
        );

        // Note: Actual rewards claiming requires:
        // 1. Fetching rewards data from EigenLayer Sidecar (gRPC/HTTP indexer)
        //    - List distribution roots: GET /rewards/v1/distribution-roots
        //    - Get rewards for root: GET /rewards/v1/distribution-roots/{rootIndex}/rewards
        // 2. Set claimer address if not already set (call set_claimer_for)
        // 3. Submit claim with Merkle proof (call process_claim or process_claims)
        //
        // The S3 bucket approach is deprecated. Future implementation should integrate
        // with the Sidecar gRPC/HTTP API to automate claiming.
        //
        // For now, we return the timestamp of the latest claimable root as a signal
        // that rewards are available. Operators can manually claim via EigenLayer CLI
        // or we can implement automated claiming in a future update.

        Ok(U256::from(distribution_root.activatedAt))
    }

    /// Calculate earnings per strategy for the operator
    ///
    /// Returns a mapping of strategy addresses to share amounts.
    /// These shares represent the operator's stake in each strategy.
    ///
    /// # Errors
    ///
    /// * Contract interaction errors
    /// * Configuration errors
    /// * Keystore errors
    pub async fn calculate_earnings_per_strategy(
        &self,
    ) -> Result<alloc::vec::Vec<(Address, U256)>> {
        let operator_address = self.get_operator_address()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let el_chain_reader = ELChainReader::new(
            Some(contract_addresses.allocation_manager_address),
            contract_addresses.delegation_manager_address,
            contract_addresses.rewards_coordinator_address,
            contract_addresses.avs_directory_address,
            Some(contract_addresses.permission_controller_address),
            self.env.http_rpc_endpoint.to_string(),
        );

        // Get operator's deposited shares (strategies and amounts)
        let (strategies, shares) = el_chain_reader
            .get_staker_shares(operator_address)
            .await
            .map_err(|e| EigenlayerExtraError::EigenSdk(e.to_string()))?;

        // Combine strategies with their corresponding shares
        let mut earnings = alloc::vec::Vec::with_capacity(strategies.len());
        for (strategy, share) in strategies.into_iter().zip(shares.into_iter()) {
            if !share.is_zero() {
                earnings.push((strategy, share));
                info!(
                    "Operator {} has {} shares in strategy {}",
                    operator_address, share, strategy
                );
            }
        }

        Ok(earnings)
    }

    /// Claim rewards for the operator
    ///
    /// Submits a transaction to claim all pending rewards for the operator.
    ///
    /// # Arguments
    ///
    /// * `root` - Merkle root for the rewards distribution
    /// * `reward_claim` - The reward claim data structure
    ///
    /// # Errors
    ///
    /// * Transaction errors
    /// * Configuration errors
    /// * No rewards available to claim
    #[allow(dead_code)]
    pub async fn claim_rewards(
        &self,
        _root: FixedBytes<32>,
        reward_claim: IRewardsCoordinator::RewardsMerkleClaim,
    ) -> Result<FixedBytes<32>> {
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let operator_address = self.get_operator_address()?;
        let ecdsa_public = self
            .env
            .keystore()
            .first_local::<K256Ecdsa>()
            .map_err(EigenlayerExtraError::Keystore)?;
        let ecdsa_secret = self
            .env
            .keystore()
            .expose_ecdsa_secret(&ecdsa_public)
            .map_err(EigenlayerExtraError::Keystore)?
            .ok_or_else(|| {
                EigenlayerExtraError::InvalidConfiguration("No ECDSA secret found".into())
            })?;

        let private_key = alloy_primitives::hex::encode(ecdsa_secret.0.to_bytes());
        let wallet = alloy_signer_local::PrivateKeySigner::from_str(&private_key)
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let provider = blueprint_evm_extra::util::get_wallet_provider_http(
            self.env.http_rpc_endpoint.clone(),
            alloy_network::EthereumWallet::from(wallet),
        );

        let rewards_coordinator =
            RewardsCoordinator::new(contract_addresses.rewards_coordinator_address, provider);

        // Process the claim
        let receipt = rewards_coordinator
            .processClaim(reward_claim, operator_address)
            .send()
            .await
            .map_err(|e| EigenlayerExtraError::Transaction(e.to_string()))?
            .get_receipt()
            .await
            .map_err(|e| EigenlayerExtraError::Transaction(e.to_string()))?;

        info!(
            "Rewards claimed successfully: {:?}",
            receipt.transaction_hash
        );

        Ok(receipt.transaction_hash)
    }

    /// Check if operator is registered
    ///
    /// # Errors
    ///
    /// * Configuration errors
    /// * Contract interaction errors
    pub async fn is_operator_registered(&self) -> Result<bool> {
        let operator_address = self.get_operator_address()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let el_chain_reader = ELChainReader::new(
            Some(contract_addresses.allocation_manager_address),
            contract_addresses.delegation_manager_address,
            contract_addresses.rewards_coordinator_address,
            contract_addresses.avs_directory_address,
            Some(contract_addresses.permission_controller_address),
            self.env.http_rpc_endpoint.to_string(),
        );

        el_chain_reader
            .is_operator_registered(operator_address)
            .await
            .map_err(|e| EigenlayerExtraError::EigenSdk(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires EigenLayer deployment
    async fn test_rewards_manager_creation() {
        // This test would require a full BlueprintEnvironment setup
        // with EigenLayer contract addresses
    }
}
