use crate::config::{BlueprintSettings, Protocol, ProtocolSettingsT};
use crate::error::ConfigError;
use alloy_primitives::{Address, address};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// The contract addresses used for EigenLayer Blueprint AVSs
///
/// The default values of these contracts are the addresses for our testing environment.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EigenlayerProtocolSettings {
    /// The address of the allocation manager contract
    pub allocation_manager_address: Address,
    /// The address of the registry coordinator contract
    pub registry_coordinator_address: Address,
    /// The address of the operator state retriever contract
    pub operator_state_retriever_address: Address,
    /// The address of the operator registry contract
    pub delegation_manager_address: Address,
    /// The address of the Service Manager contract
    pub service_manager_address: Address,
    /// The address of the Stake Registry contract
    pub stake_registry_address: Address,
    /// The address of the strategy manager contract
    pub strategy_manager_address: Address,
    /// The address of the avs registry contract
    pub avs_directory_address: Address,
    /// The address of the rewards coordinator contract
    pub rewards_coordinator_address: Address,
    /// The address of the permission controller contract
    pub permission_controller_address: Address,
    /// The address of the strategy contract
    pub strategy_address: Address,
}

impl ProtocolSettingsT for EigenlayerProtocolSettings {
    fn load(settings: BlueprintSettings) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(EigenlayerProtocolSettings {
            allocation_manager_address: settings
                .allocation_manager
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            registry_coordinator_address: settings
                .registry_coordinator
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            operator_state_retriever_address: settings
                .operator_state_retriever
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            delegation_manager_address: settings
                .delegation_manager
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            service_manager_address: settings
                .service_manager
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            stake_registry_address: settings
                .stake_registry
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            strategy_manager_address: settings
                .strategy_manager
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            avs_directory_address: settings
                .avs_directory
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            rewards_coordinator_address: settings
                .rewards_coordinator
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            permission_controller_address: settings
                .permission_controller
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            strategy_address: settings
                .strategy
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
        })
    }

    fn protocol_name(&self) -> &'static str {
        "eigenlayer"
    }

    fn protocol(&self) -> Protocol {
        Protocol::Eigenlayer
    }
}

impl Default for EigenlayerProtocolSettings {
    fn default() -> Self {
        Self {
            allocation_manager_address: address!("8a791620dd6260079bf849dc5567adc3f2fdc318"),
            registry_coordinator_address: address!("cd8a1c3ba11cf5ecfa6267617243239504a98d90"),
            operator_state_retriever_address: address!("b0d4afd8879ed9f52b28595d31b441d079b2ca07"),
            delegation_manager_address: address!("cf7ed3acca5a467e9e704c703e8d87f634fb0fc9"),
            service_manager_address: address!("36c02da8a0983159322a80ffe9f24b1acff8b570"), // Squaring service manager
            stake_registry_address: address!("4c5859f0f772848b2d91f1d83e2fe57935348029"),
            strategy_manager_address: address!("a513e6e4b8f2a923d98304ec87f64353c4d5c853"),
            avs_directory_address: address!("5fc8d32690cc91d4c39d9d3abcbd16989f875707"),
            rewards_coordinator_address: address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e"),
            permission_controller_address: address!("3aa5ebb10dc797cac828524e59a333d0a371443c"),
            strategy_address: address!("524f04724632eed237cba3c37272e018b3a7967e"),
        }
    }
}
