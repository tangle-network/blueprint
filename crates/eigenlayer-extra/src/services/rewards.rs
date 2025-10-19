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
        let operator_address = self.get_operator_address()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let provider =
            blueprint_evm_extra::util::get_provider_http(self.env.http_rpc_endpoint.clone());

        let rewards_coordinator =
            RewardsCoordinator::new(contract_addresses.rewards_coordinator_address, provider);

        // Get cumulative claimed rewards
        let claimed = rewards_coordinator
            .cumulativeClaimed(operator_address, Address::ZERO) // Use ZERO for native token
            .call()
            .await
            .map_err(|e| EigenlayerExtraError::Contract(e.to_string()))?;

        Ok(claimed)
    }

    /// Calculate earnings per strategy for the operator
    ///
    /// Returns a mapping of strategy addresses to earnings amounts.
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

        // TODO: eigensdk v2.0.0 doesn't have get_operator_details yet
        // For now, return empty vec as we'd need to query strategy-specific earnings
        // This would require additional contract calls to get the list of strategies
        // and then query earnings for each one
        info!(
            "Operator {} earnings calculation pending (eigensdk API incomplete)",
            operator_address
        );
        Ok(alloc::vec::Vec::new())
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
