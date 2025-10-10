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
    /// The address of the slasher contract
    pub slasher_address: Address,
    /// The address of the pause registry contract
    pub pause_registry_address: Address,
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
            slasher_address: settings
                .slasher
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
            pause_registry_address: settings
                .pause_registry
                .ok_or(ConfigError::MissingEigenlayerContractAddresses)?,
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
            slasher_address: address!("12699471dF8dca329C76D72823B1b79d55709384"),
            pause_registry_address: address!("9a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"),
            allocation_manager_address: address!("8a791620dd6260079bf849dc5567adc3f2fdc318"),
            registry_coordinator_address: address!("fd471836031dc5108809d173a067e8486b9047a3"),
            operator_state_retriever_address: address!("922d6956c99e12dfeb3224dea977d0939758a1fe"),
            delegation_manager_address: address!("cf7ed3acca5a467e9e704c703e8d87f634fb0fc9"),
            service_manager_address: address!("2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3"), // Squaring service manager
            stake_registry_address: address!("7bc06c482dead17c0e297afbc32f6e63d3846650"),
            strategy_manager_address: address!("a513e6e4b8f2a923d98304ec87f64353c4d5c853"),
            avs_directory_address: address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e"),
            rewards_coordinator_address: address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82"),
            permission_controller_address: address!("322813fd9a801c5507c9de605d63cea4f2ce6c44"),
            strategy_address: address!("ec4cfde48eadca2bc63e94bb437bbeace1371bf3"),
        }
    }
}
