//! Tangle Client Configuration
//!
//! This module provides configuration types for the Tangle client that don't
//! create cyclic dependencies with the runner crate.

extern crate alloc;

use alloc::string::String;
use alloy_primitives::Address;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

/// Protocol settings for Tangle
///
/// This contains the EVM-specific configuration for connecting to Tangle contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleSettings {
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

impl Default for TangleSettings {
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

/// Client configuration for connecting to Tangle contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangleClientConfig {
    /// HTTP RPC endpoint for the EVM network
    pub http_rpc_endpoint: Url,
    /// WebSocket RPC endpoint for the EVM network
    pub ws_rpc_endpoint: Url,
    /// Path to the keystore directory
    pub keystore_uri: String,
    /// Data directory for the client
    pub data_dir: PathBuf,
    /// Protocol-specific settings
    pub settings: TangleSettings,
    /// Whether the client is in test mode
    pub test_mode: bool,
    /// When true, avoid on-chain submissions from this client.
    #[serde(default)]
    pub dry_run: bool,
}

impl TangleClientConfig {
    /// Create a new client config with required parameters
    pub fn new(
        http_rpc_endpoint: impl Into<Url>,
        ws_rpc_endpoint: impl Into<Url>,
        keystore_uri: impl Into<String>,
        settings: TangleSettings,
    ) -> Self {
        Self {
            http_rpc_endpoint: http_rpc_endpoint.into(),
            ws_rpc_endpoint: ws_rpc_endpoint.into(),
            keystore_uri: keystore_uri.into(),
            data_dir: PathBuf::default(),
            settings,
            test_mode: false,
            dry_run: false,
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

    /// Set dry-run mode
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Get keystore configuration
    pub fn keystore_config(&self) -> blueprint_keystore::KeystoreConfig {
        blueprint_keystore::KeystoreConfig::new().fs_root(self.keystore_uri.replace("file://", ""))
    }
}
