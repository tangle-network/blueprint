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
            allocation_manager_address: address!("8A791620dd6260079BF849Dc5567aDC3F2FdC318"),
            registry_coordinator_address: address!("a7c59f010700930003b33ab25a7a0679c860f29c"),
            operator_state_retriever_address: address!("e3011a37a904ab90c8881a99bd1f6e21401f1522"),
            delegation_manager_address: address!("Dc64a140Aa3E981100a9becA4E685f962f0cF6C9"),
            service_manager_address: address!("c0f115a19107322cfbf1cdbc7ea011c19ebdb4f8"), // Depends on AVS
            stake_registry_address: address!("34b40ba116d5dec75548a9e9a8f15411461e8c70"), // Differs when using ECDSA Base
            strategy_manager_address: address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707"),
            avs_directory_address: address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e"),
            rewards_coordinator_address: address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82"),
            permission_controller_address: address!("322813fd9a801c5507c9de605d63cea4f2ce6c44"),
        }
    }
}

//             allocation_manager_address: address!("8A791620dd6260079BF849Dc5567aDC3F2FdC318"),
//             registry_coordinator_address: address!("faaddc93baf78e89dcf37ba67943e1be8f37bb8c"),
//             operator_state_retriever_address: address!("1f10f3ba7acb61b2f50b9d6ddcf91a6f787c0e82"),
//             delegation_manager_address: address!("Dc64a140Aa3E981100a9becA4E685f962f0cF6C9"),
//             service_manager_address: address!("c96304e3c037f81da488ed9dea1d8f2a48278a75"), // Depends on AVS
//             stake_registry_address: address!("d0141e899a65c95a556fe2b27e5982a6de7fdd7a"), // Differs when using ECDSA Base
//             strategy_manager_address: address!("5FC8d32690cc91D4c39d9d3abcBD16989F875707"),
//             avs_directory_address: address!("b7f8bc63bbcad18155201308c8f3540b07f84f5e"),
//             rewards_coordinator_address: address!("0dcd1bf9a1b36ce34237eeafef220932846bcd82"),
//             permission_controller_address: address!("322813fd9a801c5507c9de605d63cea4f2ce6c44"),
