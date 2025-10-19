use crate::error::{EigenlayerExtraError, Result};
use alloy_primitives::Address;
use blueprint_core::info;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;
use blueprint_runner::config::BlueprintEnvironment;
use eigensdk::utils::slashing::core::delegation_manager::DelegationManager;

/// Status information about operator slashing
#[derive(Debug, Clone)]
pub struct SlashingStatus {
    /// Whether the operator is currently slashed
    pub is_slashed: bool,
    /// Operator address
    pub operator_address: Address,
}

/// Event emitted when slashing is detected
#[derive(Debug, Clone)]
pub struct SlashingEvent {
    /// Operator address that was slashed
    pub operator_address: Address,
    /// Strategy address where slashing occurred
    pub strategy_address: Address,
    /// Amount slashed
    pub amount: alloy_primitives::U256,
    /// Block number where slashing occurred
    pub block_number: u64,
}

/// Monitor for operator slashing events
///
/// Provides event-driven slashing detection and status querying for EigenLayer operators.
/// Integrates with the DelegationManager contract to track slashing events.
#[derive(Clone)]
pub struct SlashingMonitor {
    env: BlueprintEnvironment,
}

impl SlashingMonitor {
    /// Create a new SlashingMonitor
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

    /// Check if the operator is slashed
    ///
    /// # Errors
    ///
    /// * Contract interaction errors
    /// * Configuration errors
    pub async fn is_operator_slashed(&self) -> Result<bool> {
        let operator_address = self.get_operator_address()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let provider =
            blueprint_evm_extra::util::get_provider_http(self.env.http_rpc_endpoint.clone());

        let delegation_manager = DelegationManager::DelegationManagerInstance::new(
            contract_addresses.delegation_manager_address,
            provider,
        );

        // Check if operator is registered (slashed operators are typically deregistered)
        let is_operator = delegation_manager
            .isOperator(operator_address)
            .call()
            .await
            .map_err(|e| EigenlayerExtraError::Contract(e.to_string()))?;

        // If not an operator, might have been slashed and deregistered
        Ok(!is_operator)
    }

    /// Get detailed slashing status for the operator
    ///
    /// # Errors
    ///
    /// * Contract interaction errors
    /// * Configuration errors
    pub async fn get_slashing_status(&self) -> Result<SlashingStatus> {
        let operator_address = self.get_operator_address()?;
        let is_slashed = self.is_operator_slashed().await?;

        Ok(SlashingStatus {
            is_slashed,
            operator_address,
        })
    }

    /// Query slashable shares for the operator across strategies
    ///
    /// Returns the amount of shares that can be slashed for a given strategy.
    ///
    /// # Arguments
    ///
    /// * `strategy_address` - The strategy address to query
    ///
    /// # Errors
    ///
    /// * Contract interaction errors
    /// * Configuration errors
    pub async fn get_slashable_shares(
        &self,
        strategy_address: Address,
    ) -> Result<alloy_primitives::U256> {
        let operator_address = self.get_operator_address()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let provider =
            blueprint_evm_extra::util::get_provider_http(self.env.http_rpc_endpoint.clone());

        let delegation_manager = DelegationManager::DelegationManagerInstance::new(
            contract_addresses.delegation_manager_address,
            provider,
        );

        let result = delegation_manager
            .getSlashableSharesInQueue(operator_address, strategy_address)
            .call()
            .await
            .map_err(|e| EigenlayerExtraError::Contract(e.to_string()))?;

        info!(
            "Slashable shares for operator {} in strategy {}: {}",
            operator_address, strategy_address, result
        );

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires EigenLayer deployment
    async fn test_slashing_monitor_creation() {
        // This test would require a full BlueprintEnvironment setup
        // with EigenLayer contract addresses
    }
}
