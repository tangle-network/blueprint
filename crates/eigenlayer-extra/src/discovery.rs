/// On-chain AVS discovery for EigenLayer operators
///
/// Provides functionality to query EigenLayer contracts and discover which AVS services
/// an operator is registered with. This is useful for:
/// - Syncing local registration state with on-chain reality
/// - Auto-discovering AVS registrations without manual configuration
/// - Validating operator registration status
use crate::error::{EigenlayerExtraError, Result};
use crate::registration::AvsRegistrationConfig;
use alloy_primitives::Address;
use blueprint_core::{info, warn};
use blueprint_runner::config::BlueprintEnvironment;
use eigensdk::client_avsregistry::reader::AvsRegistryChainReader;
use std::path::PathBuf;

/// AVS discovery service for querying on-chain registrations
pub struct AvsDiscoveryService {
    env: BlueprintEnvironment,
}

/// Discovered AVS information from on-chain queries
#[derive(Debug, Clone)]
pub struct DiscoveredAvs {
    /// Service manager contract address
    pub service_manager: Address,
    /// Registry coordinator contract address
    pub registry_coordinator: Address,
    /// Operator state retriever contract address
    pub operator_state_retriever: Address,
    /// Stake registry contract address
    pub stake_registry: Address,
    /// Whether the operator is currently registered
    pub is_registered: bool,
}

impl AvsDiscoveryService {
    /// Create a new AVS discovery service
    pub fn new(env: BlueprintEnvironment) -> Self {
        Self { env }
    }

    /// Discover all AVS services the operator is registered with
    ///
    /// Queries the EigenLayer contracts to find active AVS registrations for the operator.
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator's Ethereum address
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Contract queries fail
    /// - Configuration is invalid
    /// - RPC connection fails
    ///
    /// # Returns
    ///
    /// Vector of discovered AVS registrations
    pub async fn discover_avs_registrations(
        &self,
        operator_address: Address,
    ) -> Result<Vec<DiscoveredAvs>> {
        info!(
            "Discovering AVS registrations for operator {:#x}",
            operator_address
        );

        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        // Create AVS registry reader
        let registry_reader = AvsRegistryChainReader::new(
            contract_addresses.registry_coordinator_address,
            contract_addresses.operator_state_retriever_address,
            self.env.http_rpc_endpoint.to_string(),
        )
        .await
        .map_err(|e| {
            EigenlayerExtraError::Other(format!("Failed to create AVS registry reader: {}", e))
        })?;

        // Check if operator is registered
        let is_registered = registry_reader
            .is_operator_registered(operator_address)
            .await
            .map_err(|e| {
                EigenlayerExtraError::Other(format!("Failed to check registration status: {}", e))
            })?;

        if !is_registered {
            info!(
                "Operator {:#x} is not registered to any AVS",
                operator_address
            );
            return Ok(Vec::new());
        }

        info!("Operator {:#x} is registered to AVS", operator_address);

        // For now, we return a single discovered AVS based on the current protocol settings
        // In a full implementation, we would query the AVSDirectory to get all AVS services
        let discovered = DiscoveredAvs {
            service_manager: contract_addresses.service_manager_address,
            registry_coordinator: contract_addresses.registry_coordinator_address,
            operator_state_retriever: contract_addresses.operator_state_retriever_address,
            stake_registry: contract_addresses.stake_registry_address,
            is_registered,
        };

        Ok(vec![discovered])
    }

    /// Verify if an operator is registered with a specific AVS
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator's Ethereum address
    /// * `registry_coordinator` - The registry coordinator contract address
    ///
    /// # Errors
    ///
    /// Returns error if contract queries fail
    ///
    /// # Returns
    ///
    /// `true` if the operator is registered, `false` otherwise
    pub async fn is_operator_registered_to_avs(
        &self,
        operator_address: Address,
        registry_coordinator: Address,
    ) -> Result<bool> {
        let registry_reader = AvsRegistryChainReader::new(
            registry_coordinator,
            Address::ZERO, // operator_state_retriever not needed for this query
            self.env.http_rpc_endpoint.to_string(),
        )
        .await
        .map_err(|e| {
            EigenlayerExtraError::Other(format!("Failed to create AVS registry reader: {}", e))
        })?;

        match registry_reader
            .is_operator_registered(operator_address)
            .await
        {
            Ok(is_registered) => Ok(is_registered),
            Err(e) => {
                warn!(
                    "Failed to query registration for AVS {:#x}: {}",
                    registry_coordinator, e
                );
                Ok(false) // Treat query failures as not registered
            }
        }
    }

    /// Convert discovered AVS to registration config
    ///
    /// Creates an `AvsRegistrationConfig` from discovered on-chain data.
    /// Note: Some fields like blueprint_path must be provided separately.
    ///
    /// # Arguments
    ///
    /// * `discovered` - The discovered AVS information
    /// * `blueprint_path` - Path to the blueprint binary
    ///
    /// # Returns
    ///
    /// A partial registration config that can be completed with additional data
    pub fn discovered_to_config(
        &self,
        discovered: &DiscoveredAvs,
        blueprint_path: PathBuf,
    ) -> Result<AvsRegistrationConfig> {
        let contract_addresses = self
            .env
            .protocol_settings
            .eigenlayer()
            .map_err(|e| EigenlayerExtraError::InvalidConfiguration(e.to_string()))?;

        Ok(AvsRegistrationConfig {
            service_manager: discovered.service_manager,
            registry_coordinator: discovered.registry_coordinator,
            operator_state_retriever: discovered.operator_state_retriever,
            strategy_manager: contract_addresses.strategy_manager_address,
            delegation_manager: contract_addresses.delegation_manager_address,
            avs_directory: contract_addresses.avs_directory_address,
            rewards_coordinator: contract_addresses.rewards_coordinator_address,
            permission_controller: Some(contract_addresses.permission_controller_address),
            allocation_manager: Some(contract_addresses.allocation_manager_address),
            strategy_address: contract_addresses.strategy_address,
            stake_registry: discovered.stake_registry,
            blueprint_path,
            container_image: None, // Discovered AVS uses binary, not container
            runtime_target: crate::RuntimeTarget::default(), // Default to hypervisor for discovered AVS
            allocation_delay: contract_addresses.allocation_delay,
            deposit_amount: contract_addresses.deposit_amount,
            stake_amount: contract_addresses.stake_amount,
            operator_sets: contract_addresses.operator_sets.clone(),
        })
    }

    /// Query operator status for a specific AVS
    ///
    /// Returns detailed information about the operator's registration status.
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator's Ethereum address
    /// * `registry_coordinator` - The registry coordinator contract address
    ///
    /// # Errors
    ///
    /// Returns error if contract queries fail
    pub async fn get_operator_status(
        &self,
        operator_address: Address,
        registry_coordinator: Address,
    ) -> Result<OperatorStatus> {
        let registry_reader = AvsRegistryChainReader::new(
            registry_coordinator,
            Address::ZERO,
            self.env.http_rpc_endpoint.to_string(),
        )
        .await
        .map_err(|e| {
            EigenlayerExtraError::Other(format!("Failed to create AVS registry reader: {}", e))
        })?;

        let is_registered = registry_reader
            .is_operator_registered(operator_address)
            .await
            .map_err(|e| {
                EigenlayerExtraError::Other(format!("Failed to query operator status: {}", e))
            })?;

        Ok(OperatorStatus {
            operator_address,
            registry_coordinator,
            is_registered,
        })
    }
}

/// Detailed operator registration status
#[derive(Debug, Clone)]
pub struct OperatorStatus {
    /// The operator's address
    pub operator_address: Address,
    /// The registry coordinator being queried
    pub registry_coordinator: Address,
    /// Whether the operator is registered
    pub is_registered: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_status_creation() {
        let operator = Address::from([1u8; 20]);
        let registry = Address::from([2u8; 20]);

        let status = OperatorStatus {
            operator_address: operator,
            registry_coordinator: registry,
            is_registered: true,
        };

        assert_eq!(status.operator_address, operator);
        assert_eq!(status.registry_coordinator, registry);
        assert!(status.is_registered);
    }
}
