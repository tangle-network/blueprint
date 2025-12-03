//! Tangle EVM Protocol Configuration
//!
//! Provides configuration for running blueprints on Tangle v2 EVM contracts.

use alloy_primitives::{Address, Bytes};
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
    use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};

    let settings = env.protocol_settings.tangle_evm()?;

    // Create the client config from environment
    let client_config = TangleEvmClientConfig {
        http_rpc_endpoint: env.http_rpc_endpoint.clone(),
        ws_rpc_endpoint: env.ws_rpc_endpoint.clone(),
        settings: TangleEvmSettings {
            blueprint_id: settings.blueprint_id,
            service_id: settings.service_id,
            tangle_contract: settings.tangle_contract,
            restaking_contract: settings.restaking_contract,
        },
        keystore_uri: env.keystore_uri.clone(),
        data_dir: env.data_dir.clone(),
        test_mode: env.test_mode,
    };

    // Create the EVM client
    let client = TangleEvmClient::new(client_config)
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
    use alloy_primitives::B256;
    use alloy_provider::ProviderBuilder;
    use alloy_signer_local::PrivateKeySigner;
    use blueprint_client_tangle_evm::contracts::ITangle;
    use blueprint_crypto::k256::K256Ecdsa;
    use blueprint_keystore::backends::Backend;
    use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;

    let settings = env.protocol_settings.tangle_evm()?;

    blueprint_core::info!(
        "Starting Tangle EVM registration: blueprint_id={}, rpc_address={}, tangle_contract={:?}",
        settings.blueprint_id,
        rpc_address,
        settings.tangle_contract
    );

    // 1. Get ECDSA key from keystore
    let ecdsa_public = env
        .keystore()
        .first_local::<K256Ecdsa>()
        .map_err(|e| TangleEvmError::Keystore(e.to_string()))?;

    let ecdsa_secret = env
        .keystore()
        .expose_ecdsa_secret(&ecdsa_public)
        .map_err(|e| TangleEvmError::Keystore(e.to_string()))?
        .ok_or_else(|| TangleEvmError::Keystore("No ECDSA secret found in keystore".into()))?;

    // 2. Create wallet/signer from secret key
    let secret_bytes = ecdsa_secret.0.to_bytes();
    let secret_b256 = B256::from_slice(&secret_bytes);
    let wallet = PrivateKeySigner::from_bytes(&secret_b256)
        .map_err(|e| TangleEvmError::Keystore(format!("Failed to create signer: {e}")))?;

    let operator_address = wallet.address();
    blueprint_core::info!("Operator address: {}", operator_address);

    // 3. Create provider with signer
    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .connect(env.http_rpc_endpoint.as_str())
        .await
        .map_err(|e| TangleEvmError::Contract(format!("Failed to connect to RPC: {e}")))?;

    // 4. Create contract instance with signed provider
    let tangle_contract = ITangle::new(settings.tangle_contract, &provider);

    // 5. Build and send registerOperator transaction
    // The preferences field is empty bytes for now (can be extended for custom preferences)
    let preferences = Bytes::new();

    blueprint_core::info!(
        "Sending registerOperator transaction for blueprint_id={}",
        settings.blueprint_id
    );

    let tx = tangle_contract.registerOperator(settings.blueprint_id, preferences);

    let pending_tx = tx
        .send()
        .await
        .map_err(|e| TangleEvmError::Transaction(format!("Failed to send transaction: {e}")))?;

    blueprint_core::info!(
        "Transaction sent, waiting for confirmation: {:?}",
        pending_tx.tx_hash()
    );

    // 6. Wait for transaction confirmation
    let receipt = pending_tx
        .get_receipt()
        .await
        .map_err(|e| TangleEvmError::Transaction(format!("Failed to get receipt: {e}")))?;

    if !receipt.status() {
        return Err(TangleEvmError::Transaction("Transaction reverted".into()).into());
    }

    blueprint_core::info!(
        "Registration successful! tx_hash={:?}, block={:?}",
        receipt.transaction_hash,
        receipt.block_number
    );

    // 7. Optionally update RPC address if provided
    if !rpc_address.is_empty() {
        blueprint_core::info!(
            "Updating RPC address to: {} for blueprint_id={}",
            rpc_address,
            settings.blueprint_id
        );

        let rpc_tx = tangle_contract.updateRpcAddress(
            settings.blueprint_id,
            rpc_address.to_string(),
        );

        let pending_rpc_tx = rpc_tx
            .send()
            .await
            .map_err(|e| TangleEvmError::Transaction(format!("Failed to update RPC address: {e}")))?;

        let rpc_receipt = pending_rpc_tx
            .get_receipt()
            .await
            .map_err(|e| TangleEvmError::Transaction(format!("Failed to get RPC update receipt: {e}")))?;

        if !rpc_receipt.status() {
            blueprint_core::warn!("RPC address update transaction reverted");
        } else {
            blueprint_core::info!(
                "RPC address updated successfully! tx_hash={:?}",
                rpc_receipt.transaction_hash
            );
        }
    }

    Ok(())
}
