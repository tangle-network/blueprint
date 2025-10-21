use crate::error::{EigenlayerExtraError, Result};
use alloy_primitives::{Address, FixedBytes};
use blueprint_core::info;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;
use blueprint_runner::config::BlueprintEnvironment;
use eigensdk::client_elcontracts::writer::ELChainWriter;
use eigensdk::types::operator::Operator;

/// Manager for operator lifecycle operations
///
/// Provides high-level abstractions for operator registration, deregistration,
/// and metadata updates on EigenLayer.
#[derive(Clone)]
pub struct OperatorLifecycleManager {
    env: BlueprintEnvironment,
}

/// Operator metadata for updates
#[derive(Debug, Clone)]
pub struct OperatorMetadata {
    /// URL pointing to operator metadata JSON
    pub metadata_url: String,
    /// Address that can approve delegations (use Address::ZERO to disable)
    pub delegation_approver_address: Address,
}

impl OperatorLifecycleManager {
    /// Create a new OperatorLifecycleManager
    pub fn new(env: BlueprintEnvironment) -> Self {
        Self { env }
    }

    /// Get the operator address and private key from keystore
    ///
    /// # Errors
    ///
    /// * Keystore errors if ECDSA key not found or cannot be exposed
    fn get_operator_credentials(&self) -> Result<(Address, String)> {
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

        let operator_address = ecdsa_secret
            .alloy_address()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let private_key = alloy_primitives::hex::encode(ecdsa_secret.0.to_bytes());

        Ok((operator_address, private_key))
    }

    /// Deregister the operator from this AVS's operator sets
    ///
    /// Removes the operator from the operator sets configured for this AVS,
    /// preventing them from receiving new work or participating in this AVS.
    ///
    /// # Errors
    ///
    /// * Transaction errors
    /// * Configuration errors
    /// * Operator not registered
    pub async fn deregister_operator(&self) -> Result<FixedBytes<32>> {
        let (operator_address, private_key) = self.get_operator_credentials()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let el_writer = ELChainWriter::new(
            contract_addresses.strategy_manager_address,
            contract_addresses.rewards_coordinator_address,
            Some(contract_addresses.permission_controller_address),
            Some(contract_addresses.allocation_manager_address),
            contract_addresses.registry_coordinator_address,
            eigensdk::client_elcontracts::reader::ELChainReader::new(
                Some(contract_addresses.allocation_manager_address),
                contract_addresses.delegation_manager_address,
                contract_addresses.rewards_coordinator_address,
                contract_addresses.avs_directory_address,
                Some(contract_addresses.permission_controller_address),
                self.env.http_rpc_endpoint.to_string(),
            ),
            self.env.http_rpc_endpoint.to_string(),
            private_key,
        );

        // Deregister from this AVS's operator sets
        let tx_hash = el_writer
            .deregister_from_operator_sets(
                operator_address,
                contract_addresses.service_manager_address,
                contract_addresses.operator_sets.clone(),
            )
            .await
            .map_err(|e| EigenlayerExtraError::EigenSdk(e.to_string()))?;

        info!(
            "Operator {} deregistered from AVS {} operator sets {:?}: {:?}",
            operator_address,
            contract_addresses.service_manager_address,
            contract_addresses.operator_sets,
            tx_hash
        );

        Ok(tx_hash)
    }

    /// Update operator metadata
    ///
    /// Updates the operator's metadata URL and delegation approver address.
    ///
    /// # Arguments
    ///
    /// * `metadata` - New metadata configuration
    ///
    /// # Errors
    ///
    /// * Transaction errors
    /// * Configuration errors
    pub async fn update_operator_metadata(
        &self,
        metadata: OperatorMetadata,
    ) -> Result<FixedBytes<32>> {
        let (operator_address, private_key) = self.get_operator_credentials()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let el_writer = ELChainWriter::new(
            contract_addresses.strategy_manager_address,
            contract_addresses.rewards_coordinator_address,
            Some(contract_addresses.permission_controller_address),
            Some(contract_addresses.allocation_manager_address),
            contract_addresses.registry_coordinator_address,
            eigensdk::client_elcontracts::reader::ELChainReader::new(
                Some(contract_addresses.allocation_manager_address),
                contract_addresses.delegation_manager_address,
                contract_addresses.rewards_coordinator_address,
                contract_addresses.avs_directory_address,
                Some(contract_addresses.permission_controller_address),
                self.env.http_rpc_endpoint.to_string(),
            ),
            self.env.http_rpc_endpoint.to_string(),
            private_key,
        );

        // Create updated operator details
        let operator_details = Operator {
            address: operator_address,
            delegation_approver_address: metadata.delegation_approver_address,
            metadata_url: metadata.metadata_url,
            allocation_delay: Some(30), // Default allocation delay
            _deprecated_earnings_receiver_address: None,
            staker_opt_out_window_blocks: Some(50400),
        };

        let tx_hash = el_writer
            .update_operator_details(operator_details)
            .await
            .map_err(|e| EigenlayerExtraError::EigenSdk(e.to_string()))?;

        info!(
            "Operator {} metadata updated successfully: {:?}",
            operator_address, tx_hash
        );

        Ok(tx_hash)
    }

    /// Get operator status
    ///
    /// Returns whether the operator is currently registered.
    ///
    /// # Errors
    ///
    /// * Contract interaction errors
    /// * Configuration errors
    pub async fn get_operator_status(&self) -> Result<bool> {
        let (operator_address, _) = self.get_operator_credentials()?;
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        let el_chain_reader = eigensdk::client_elcontracts::reader::ELChainReader::new(
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
    

    #[tokio::test]
    #[ignore] // Requires EigenLayer deployment
    async fn test_lifecycle_manager_creation() {
        // This test would require a full BlueprintEnvironment setup
        // with EigenLayer contract addresses
    }
}
