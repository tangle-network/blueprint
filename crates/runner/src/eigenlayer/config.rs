use crate::config::{BlueprintSettings, ProtocolSettingsT};
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
}

impl ProtocolSettingsT for EigenlayerProtocolSettings {
    type Settings = Self;

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
        })
    }

    fn protocol(&self) -> &'static str {
        "eigenlayer"
    }

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for EigenlayerProtocolSettings {
    fn default() -> Self {
        Self {
            allocation_manager_address: address!("8a791620dd6260079bf849dc5567adc3f2fdc318"),
            registry_coordinator_address: address!("22753e4264fddc6181dc7cce468904a80a363e44"),
            operator_state_retriever_address: address!("ab16a69a5a8c12c732e0deff4be56a70bb64c926"),
            delegation_manager_address: address!("cf7ed3acca5a467e9e704c703e8d87f634fb0fc9"),
            service_manager_address: address!("f8e31cb472bc70500f08cd84917e5a1912ec8397"), // Depends on AVS
            stake_registry_address: address!("c96304e3c037f81da488ed9dea1d8f2a48278a75"), // Differs when using ECDSA Base
            strategy_manager_address: address!("a513e6e4b8f2a923d98304ec87f64353c4d5c853"),
            avs_directory_address: address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e"),
            rewards_coordinator_address: address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82"),
            permission_controller_address: address!("4ed7c70f96b99c776995fb64377f0d4ab3b0e1c1"),
        }
    }
}
