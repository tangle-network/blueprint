//! Tangle EVM Protocol Configuration
//!
//! Provides configuration for running blueprints on Tangle v2 EVM contracts.

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::config::{BlueprintEnvironment, BlueprintSettings, Protocol, ProtocolSettingsT};
use crate::error::RunnerError;
use crate::BlueprintConfig;

/// Protocol settings for Tangle EVM (v2)
///
/// This contains the EVM-specific configuration for connecting to Tangle v2 contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleEvmProtocolSettings {
    /// The blueprint ID registered in the Tangle contract
    pub blueprint_id: u64,
    /// The service ID for the Tangle blueprint instance
    ///
    /// Note: This will be `None` if running in Registration Mode.
    pub service_id: Option<u64>,
    /// The Tangle core contract address
    pub tangle_contract: Address,
    /// The MultiAssetDelegation (restaking) contract address
    pub restaking_contract: Address,
}

impl Default for TangleEvmProtocolSettings {
    fn default() -> Self {
        Self {
            blueprint_id: 0,
            service_id: None,
            // Default to zero address - must be configured
            tangle_contract: Address::ZERO,
            restaking_contract: Address::ZERO,
        }
    }
}

impl ProtocolSettingsT for TangleEvmProtocolSettings {
    fn load(_settings: BlueprintSettings) -> Result<Self, Box<dyn Error + Send + Sync>> {
        use crate::error::ConfigError;

        // Parse blueprint_id from environment
        let blueprint_id: u64 = std::env::var("BLUEPRINT_ID")
            .map_err(|_| ConfigError::MissingBlueprintId)?
            .parse()
            .map_err(|_| ConfigError::MissingBlueprintId)?;

        // Parse service_id from environment (optional)
        let service_id: Option<u64> = std::env::var("SERVICE_ID")
            .ok()
            .and_then(|s| s.parse().ok());

        // Parse contract addresses from environment
        let tangle_contract = std::env::var("TANGLE_CONTRACT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Address::ZERO);

        let restaking_contract = std::env::var("RESTAKING_CONTRACT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Address::ZERO);

        Ok(Self {
            blueprint_id,
            service_id,
            tangle_contract,
            restaking_contract,
        })
    }

    fn protocol_name(&self) -> &'static str {
        "tangle-evm"
    }

    fn protocol(&self) -> Protocol {
        Protocol::TangleEvm
    }
}

/// Runtime configuration for Tangle EVM blueprints
#[derive(Clone, Debug, Default)]
pub struct TangleEvmConfig {
    /// RPC endpoint for operator registration announcements
    pub rpc_address: String,
    /// Whether to exit after registration
    pub exit_after_register: bool,
}

impl TangleEvmConfig {
    /// Create a new TangleEvmConfig with the given RPC address
    #[must_use]
    pub fn new(rpc_address: impl Into<String>) -> Self {
        Self {
            rpc_address: rpc_address.into(),
            exit_after_register: true,
        }
    }

    /// Set whether to exit after registration
    #[must_use]
    pub fn with_exit_after_register(mut self, should_exit: bool) -> Self {
        self.exit_after_register = should_exit;
        self
    }
}

impl BlueprintConfig for TangleEvmConfig {
    async fn register(&self, env: &BlueprintEnvironment) -> Result<(), RunnerError> {
        register_impl(&self.rpc_address, env).await
    }

    async fn requires_registration(&self, env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
        requires_registration_impl(env).await
    }

    fn should_exit_after_registration(&self) -> bool {
        self.exit_after_register
    }
}

/// Check if operator registration is required
async fn requires_registration_impl(env: &BlueprintEnvironment) -> Result<bool, RunnerError> {
    use super::error::TangleEvmError;
    use blueprint_client_tangle_evm::TangleEvmClient;

    let settings = env.protocol_settings.tangle_evm()?;

    // Create the EVM client
    let client = TangleEvmClient::new(env.clone())
        .await
        .map_err(|e| TangleEvmError::Contract(e.to_string()))?;

    // Check if operator is registered for the blueprint
    let is_registered = client
        .is_operator_registered(settings.blueprint_id, client.account())
        .await
        .map_err(|e| TangleEvmError::Contract(e.to_string()))?;

    Ok(!is_registered)
}

/// Register the operator on the Tangle EVM contract
async fn register_impl(rpc_address: &str, env: &BlueprintEnvironment) -> Result<(), RunnerError> {
    use super::error::TangleEvmError;

    let settings = env.protocol_settings.tangle_evm()?;

    // For now, we log the registration intent
    // Full implementation requires transaction signing with alloy
    blueprint_core::info!(
        "Tangle EVM registration: blueprint_id={}, rpc_address={}, tangle_contract={:?}",
        settings.blueprint_id,
        rpc_address,
        settings.tangle_contract
    );

    // TODO: Implement actual registration transaction
    // This requires:
    // 1. Creating a signer from the keystore
    // 2. Building the registerOperator transaction
    // 3. Sending and waiting for confirmation

    // For now, return an error indicating this is not yet implemented
    // This allows the code to compile while we implement the full flow
    Err(TangleEvmError::MissingConfig(
        "Registration transaction not yet implemented - use manual registration".into(),
    )
    .into())
}
