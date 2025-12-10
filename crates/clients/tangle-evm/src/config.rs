//! Tangle EVM Client Configuration
//!
//! This module provides configuration types for the Tangle EVM client that don't
//! create cyclic dependencies with the runner crate.

extern crate alloc;

use alloc::string::String;
use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

/// Protocol settings for Tangle EVM (v2)
///
/// This contains the EVM-specific configuration for connecting to Tangle v2 contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleEvmSettings {
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
    /// Operator status registry contract used for heartbeats
    pub status_registry_contract: Address,
}

impl Default for TangleEvmSettings {
    fn default() -> Self {
        Self {
            blueprint_id: 0,
            service_id: None,
            // Default to zero address - must be configured
            tangle_contract: Address::ZERO,
            restaking_contract: Address::ZERO,
            status_registry_contract: Address::ZERO,
        }
    }
}

/// Client configuration for connecting to Tangle EVM contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleEvmClientConfig {
    /// HTTP RPC endpoint for the EVM network
    pub http_rpc_endpoint: Url,
    /// WebSocket RPC endpoint for the EVM network
    pub ws_rpc_endpoint: Url,
    /// Path to the keystore directory
    pub keystore_uri: String,
    /// Data directory for the client
    pub data_dir: PathBuf,
    /// Protocol-specific settings
    pub settings: TangleEvmSettings,
    /// Whether the client is in test mode
    pub test_mode: bool,
}

impl TangleEvmClientConfig {
    /// Create a new client config with required parameters
    pub fn new(
        http_rpc_endpoint: impl Into<Url>,
        ws_rpc_endpoint: impl Into<Url>,
        keystore_uri: impl Into<String>,
        settings: TangleEvmSettings,
    ) -> Self {
        Self {
            http_rpc_endpoint: http_rpc_endpoint.into(),
            ws_rpc_endpoint: ws_rpc_endpoint.into(),
            keystore_uri: keystore_uri.into(),
            data_dir: PathBuf::default(),
            settings,
            test_mode: false,
        }
    }

    /// Set the data directory
    pub fn data_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.data_dir = path.into();
        self
    }

    /// Set test mode
    pub fn test_mode(mut self, test_mode: bool) -> Self {
        self.test_mode = test_mode;
        self
    }

    /// Get keystore configuration
    pub fn keystore_config(&self) -> blueprint_keystore::KeystoreConfig {
        blueprint_keystore::KeystoreConfig::new().fs_root(self.keystore_uri.replace("file://", ""))
    }
}
