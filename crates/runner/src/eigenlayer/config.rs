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
    /// The address of the strategy contract
    pub strategy_address: Address,
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
            strategy_address: settings
                .strategy
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
            allocation_manager_address: address!("d0141e899a65c95a556fe2b27e5982a6de7fdd7a"),
            registry_coordinator_address: address!("4bf010f1b9beda5450a8dd702ed602a104ff65ee"),
            operator_state_retriever_address: address!("c582bc0317dbb0908203541971a358c44b1f3766"),
            delegation_manager_address: address!("cace1b78160ae76398f486c8a18044da0d66d86d"),
            service_manager_address: address!("638a246f0ec8883ef68280293ffe8cfbabe61b44"), // Depends on AVS
            stake_registry_address: address!("fd6f7a6a5c21a3f503ebae7a473639974379c351"), // Differs when using ECDSA Base
            strategy_manager_address: address!("c96304e3c037f81da488ed9dea1d8f2a48278a75"),
            avs_directory_address: address!("f8e31cb472bc70500f08cd84917e5a1912ec8397"),
            rewards_coordinator_address: address!("22753e4264fddc6181dc7cce468904a80a363e44"),
            permission_controller_address: address!("3aade2dcd2df6a8cac689ee797591b2913658659"),
            strategy_address: address!("f8a8b047683062b5bbbbe9d104c9177d6b6cc086"),
        }
    }
}
