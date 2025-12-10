use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use alloy_primitives::Address;
use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};
use blueprint_runner::tangle_evm::config::TangleEvmProtocolSettings;
use blueprint_testing_utils::anvil::{
    SeededTangleEvmTestnet, TangleEvmHarness,
    tangle_evm::{LOCAL_SERVICE_ID, insert_default_operator_key},
};
use clap::Args;
use color_eyre::eyre::{Result, eyre};
use tempfile::TempDir;
use url::Url;

use crate::command::run::tangle::RunOpts;
use blueprint_manager::config::SourceType;

/// Shared CLI arguments for connecting to the Tangle EVM stack.
#[derive(Args, Debug, Clone)]
pub struct TangleClientArgs {
    /// HTTP RPC endpoint.
    #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:8545")]
    pub http_rpc_url: Url,
    /// WebSocket RPC endpoint.
    #[arg(long, value_name = "URL", default_value = "ws://127.0.0.1:8546")]
    pub ws_rpc_url: Url,
    /// Path to the keystore directory.
    #[arg(long, default_value = "./keystore")]
    pub keystore_path: PathBuf,
    /// Tangle contract address.
    #[arg(long, value_name = "ADDRESS")]
    pub tangle_contract: String,
    /// Restaking contract address.
    #[arg(long, value_name = "ADDRESS")]
    pub restaking_contract: String,
    /// Optional status registry contract address.
    #[arg(long, value_name = "ADDRESS")]
    pub status_registry_contract: Option<String>,
}

impl TangleClientArgs {
    fn parse_addresses(&self) -> Result<(Address, Address, Address)> {
        let tangle = parse_address(&self.tangle_contract, "TANGLE_CONTRACT")?;
        let restaking = parse_address(&self.restaking_contract, "RESTAKING_CONTRACT")?;
        let status = if let Some(value) = &self.status_registry_contract {
            parse_address(value, "STATUS_REGISTRY_CONTRACT")?
        } else {
            Address::ZERO
        };
        Ok((tangle, restaking, status))
    }

    /// Build a client config using the provided blueprint/service identifiers.
    pub fn client_config(
        &self,
        blueprint_id: u64,
        service_id: Option<u64>,
    ) -> Result<TangleEvmClientConfig> {
        let (tangle, restaking, status) = self.parse_addresses()?;
        let settings = TangleEvmSettings {
            blueprint_id,
            service_id,
            tangle_contract: tangle,
            restaking_contract: restaking,
            status_registry_contract: status,
        };

        Ok(TangleEvmClientConfig::new(
            self.http_rpc_url.clone(),
            self.ws_rpc_url.clone(),
            self.keystore_path.display().to_string(),
            settings,
        ))
    }

    /// Connect a `TangleEvmClient` using these arguments.
    pub async fn connect(
        &self,
        blueprint_id: u64,
        service_id: Option<u64>,
    ) -> Result<TangleEvmClient> {
        let config = self.client_config(blueprint_id, service_id)?;
        TangleEvmClient::new(config)
            .await
            .map_err(|e| eyre!(e.to_string()))
    }

    /// Absolute keystore path on disk.
    #[must_use]
    pub fn keystore_path(&self) -> &PathBuf {
        &self.keystore_path
    }
}

/// Parse an address string with a helpful label.
pub fn parse_address(value: &str, name: &str) -> Result<Address> {
    Address::from_str(value).map_err(|_| eyre!("Invalid {name}: {value}"))
}

/// Preferred spawning strategy for local runs.
#[derive(clap::ValueEnum, Clone, Copy, Debug, Default)]
pub enum SpawnMethod {
    #[default]
    Vm,
    Native,
    Container,
}

impl SpawnMethod {
    #[must_use]
    pub fn preferred_source(self) -> SourceType {
        match self {
            SpawnMethod::Container => SourceType::Container,
            SpawnMethod::Vm | SpawnMethod::Native => SourceType::Native,
        }
    }

    #[must_use]
    pub fn use_vm(self) -> bool {
        matches!(self, SpawnMethod::Vm)
    }
}

/// Preferred artifact source for the manager.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum PreferredSourceArg {
    Native,
    Container,
    Wasm,
}

impl From<PreferredSourceArg> for SourceType {
    fn from(value: PreferredSourceArg) -> Self {
        match value {
            PreferredSourceArg::Native => SourceType::Native,
            PreferredSourceArg::Container => SourceType::Container,
            PreferredSourceArg::Wasm => SourceType::Wasm,
        }
    }
}

/// In-memory devnet stack backed by the seeded Anvil harness.
#[derive(Debug)]
pub struct DevnetStack {
    harness: SeededTangleEvmTestnet,
    _temp_dir: TempDir,
    keystore_dir: PathBuf,
    data_dir: PathBuf,
}

impl DevnetStack {
    /// Spawn a deterministic local stack.
    pub async fn spawn(include_anvil_logs: bool) -> Result<Self> {
        let harness = TangleEvmHarness::builder()
            .include_anvil_logs(include_anvil_logs)
            .spawn()
            .await
            .map_err(|e| eyre!(e.to_string()))?;

        let temp_dir =
            TempDir::new().map_err(|e| eyre!("failed to create temporary workspace: {e}"))?;
        let keystore_dir = temp_dir.path().join("keystore");
        ensure_dir(&keystore_dir)?;
        let data_dir = temp_dir.path().join("data");
        ensure_dir(&data_dir)?;

        seed_operator_keystore(&keystore_dir)?;

        Ok(Self {
            harness,
            _temp_dir: temp_dir,
            keystore_dir,
            data_dir,
        })
    }

    /// HTTP RPC endpoint for the harness.
    #[must_use]
    pub fn http_rpc_url(&self) -> Url {
        self.harness.http_endpoint().clone()
    }

    /// WebSocket RPC endpoint for the harness.
    #[must_use]
    pub fn ws_rpc_url(&self) -> Url {
        self.harness.ws_endpoint().clone()
    }

    /// Keystore path seeded with the operator key.
    #[must_use]
    pub fn keystore_path(&self) -> String {
        self.keystore_dir.display().to_string()
    }

    /// Data directory for the stack.
    #[must_use]
    pub fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    /// Default service identifier baked into the harness.
    #[must_use]
    pub fn default_service_id(&self) -> u64 {
        LOCAL_SERVICE_ID
    }

    /// Addresses of the deployed contracts.
    #[must_use]
    pub fn tangle_contract(&self) -> Address {
        self.harness.tangle_contract
    }

    #[must_use]
    pub fn restaking_contract(&self) -> Address {
        self.harness.restaking_contract
    }

    #[must_use]
    pub fn status_registry_contract(&self) -> Address {
        self.harness.status_registry_contract
    }

    /// Consume the stack and shut down resources.
    pub async fn shutdown(self) {
        drop(self);
    }
}

impl Drop for DevnetStack {
    fn drop(&mut self) {
        // Harness and tempdir cleanup happens automatically.
    }
}

pub fn run_opts_from_stack(
    stack: &DevnetStack,
    settings: &TangleEvmProtocolSettings,
    allow_unchecked_attestations: bool,
    method: SpawnMethod,
) -> RunOpts {
    RunOpts {
        http_rpc_url: stack.http_rpc_url(),
        ws_rpc_url: stack.ws_rpc_url(),
        blueprint_id: settings.blueprint_id,
        service_id: settings.service_id.or(Some(stack.default_service_id())),
        tangle_contract: stack.tangle_contract(),
        restaking_contract: stack.restaking_contract(),
        status_registry_contract: stack.status_registry_contract(),
        keystore_path: stack.keystore_path(),
        data_dir: Some(stack.data_dir()),
        allow_unchecked_attestations,
        registration_mode: false,
        registration_capture_only: false,
        preferred_source: method.preferred_source(),
        use_vm: method.use_vm(),
    }
}

fn ensure_dir(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn seed_operator_keystore(path: &PathBuf) -> Result<()> {
    let keystore =
        blueprint_keystore::Keystore::new(blueprint_keystore::KeystoreConfig::new().fs_root(path))?;
    insert_default_operator_key(&keystore).map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}
