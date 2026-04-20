use std::cell::OnceCell;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use alloy_primitives::Address;
use blueprint_client_tangle::{TangleClient, TangleClientConfig, TangleSettings};
use blueprint_runner::tangle::config::TangleProtocolSettings;
use blueprint_testing_utils::anvil::{
    SeededTangleTestnet, TangleHarness,
    tangle::{LOCAL_SERVICE_ID, insert_default_operator_key},
};
use clap::Args;
use color_eyre::eyre::{Result, eyre};
use tempfile::TempDir;
use url::Url;

use crate::command::run::tangle::RunOpts;
use crate::workspace::{Network, TangleWorkspace};
use blueprint_manager::config::SourceType;

/// Shared CLI arguments for connecting to the Tangle stack.
///
/// All fields are optional at the CLI level. A value is resolved from (in order):
///
///   1. The command-line flag.
///   2. The network entry in `.tangle.toml` (see [`crate::workspace`]).
///   3. For RPC URLs and keystore path only: a sensible devnet default.
///
/// This means `cargo-tangle jobs submit --job 0 --payload-hex ...` works with no
/// other flags when a `.tangle.toml` is present — contract addresses and endpoints
/// come from the workspace.
#[derive(Args, Debug, Clone, Default)]
pub struct TangleClientArgs {
    /// HTTP RPC endpoint.
    #[arg(long, value_name = "URL")]
    pub http_rpc_url: Option<Url>,
    /// WebSocket RPC endpoint.
    #[arg(long, value_name = "URL")]
    pub ws_rpc_url: Option<Url>,
    /// Path to the keystore directory.
    #[arg(long)]
    pub keystore_path: Option<PathBuf>,
    /// Tangle contract address.
    #[arg(long, value_name = "ADDRESS")]
    pub tangle_contract: Option<String>,
    /// Restaking contract address.
    #[arg(long, value_name = "ADDRESS")]
    pub staking_contract: Option<String>,
    /// Optional status registry contract address.
    #[arg(long, value_name = "ADDRESS")]
    pub status_registry_contract: Option<String>,
    /// Override the active network from `.tangle.toml`.
    #[arg(long, value_name = "NAME")]
    pub network: Option<String>,
    /// Memoised output of `resolve()` so multiple accessors don't re-read
    /// `.tangle.toml`. Also prevents inconsistency if the file changes
    /// mid-command.
    #[clap(skip)]
    #[doc(hidden)]
    resolved: OnceCell<Resolved>,
}

/// Fully-resolved parameters after merging CLI + workspace + defaults.
#[derive(Debug, Clone)]
struct Resolved {
    http_rpc_url: Url,
    ws_rpc_url: Url,
    keystore_path: PathBuf,
    tangle: Address,
    restaking: Address,
    status: Address,
}

impl TangleClientArgs {
    fn resolve(&self) -> Result<&Resolved> {
        if let Some(r) = self.resolved.get() {
            return Ok(r);
        }
        let r = self.resolve_fresh()?;
        // Ignore any race — first writer wins, we return whatever's there.
        let _ = self.resolved.set(r);
        Ok(self.resolved.get().expect("just set"))
    }

    fn resolve_fresh(&self) -> Result<Resolved> {
        let ws = TangleWorkspace::discover()?;
        let net: Option<&Network> = match (&self.network, ws.as_ref()) {
            (Some(name), Some(ws)) => Some(ws.network(name)?),
            (None, Some(ws)) => Some(ws.active_network()?),
            _ => None,
        };

        let http_rpc_url = self
            .http_rpc_url
            .clone()
            .or_else(|| net.map(|n| n.http_rpc_url.clone()))
            .unwrap_or_else(|| Url::parse("http://127.0.0.1:8545").expect("literal url"));

        // NB: historical default was port 8546 (geth convention); current
        // Anvil binds http + ws on the same port. We default to 8545 for the
        // Anvil case but document loudly for anyone relying on the old default.
        let ws_rpc_url = self
            .ws_rpc_url
            .clone()
            .or_else(|| net.map(|n| n.ws_rpc_url.clone()))
            .unwrap_or_else(|| Url::parse("ws://127.0.0.1:8545").expect("literal url"));

        let keystore_path = self
            .keystore_path
            .clone()
            .or_else(|| ws.as_ref().and_then(|w| w.defaults.keystore_path.clone()))
            .unwrap_or_else(|| PathBuf::from("./keystore"));

        let tangle = match (&self.tangle_contract, net) {
            (Some(s), _) => parse_address(s, "TANGLE_CONTRACT")?,
            (None, Some(n)) => n.tangle_contract,
            (None, None) => {
                return Err(missing_addr_err("tangle_contract", "TANGLE_CONTRACT"));
            }
        };
        let restaking = match (&self.staking_contract, net) {
            (Some(s), _) => parse_address(s, "STAKING_CONTRACT")?,
            (None, Some(n)) => n.staking_contract,
            (None, None) => {
                return Err(missing_addr_err("staking_contract", "STAKING_CONTRACT"));
            }
        };
        let status = match (&self.status_registry_contract, net) {
            (Some(s), _) => parse_address(s, "STATUS_REGISTRY_CONTRACT")?,
            (None, Some(n)) => n.status_registry_contract.unwrap_or(Address::ZERO),
            (None, None) => Address::ZERO,
        };

        Ok(Resolved {
            http_rpc_url,
            ws_rpc_url,
            keystore_path,
            tangle,
            restaking,
            status,
        })
    }

    /// Build a client config using the provided blueprint/service identifiers.
    pub fn client_config(
        &self,
        blueprint_id: u64,
        service_id: Option<u64>,
    ) -> Result<TangleClientConfig> {
        let r = self.resolve()?;
        let settings = TangleSettings {
            blueprint_id,
            service_id,
            tangle_contract: r.tangle,
            staking_contract: r.restaking,
            status_registry_contract: r.status,
        };

        Ok(TangleClientConfig::new(
            r.http_rpc_url.clone(),
            r.ws_rpc_url.clone(),
            r.keystore_path.display().to_string(),
            settings,
        ))
    }

    /// Connect a `TangleClient` using these arguments.
    pub async fn connect(
        &self,
        blueprint_id: u64,
        service_id: Option<u64>,
    ) -> Result<TangleClient> {
        let config = self.client_config(blueprint_id, service_id)?;
        TangleClient::new(config)
            .await
            .map_err(|e| eyre!(e.to_string()))
    }

    /// Resolved keystore path (CLI > workspace default > `./keystore`).
    pub fn keystore_path(&self) -> Result<PathBuf> {
        Ok(self.resolve()?.keystore_path.clone())
    }

    /// Resolved HTTP RPC URL.
    pub fn http_rpc_url(&self) -> Result<Url> {
        Ok(self.resolve()?.http_rpc_url.clone())
    }

    /// Resolved WebSocket RPC URL.
    pub fn ws_rpc_url(&self) -> Result<Url> {
        Ok(self.resolve()?.ws_rpc_url.clone())
    }
}

fn missing_addr_err(field: &str, _env_name: &str) -> color_eyre::eyre::Report {
    eyre!(
        "missing {field}: pass --{} or define it in `.tangle.toml`. Run `cargo-tangle dev up` to auto-generate a workspace for local development.",
        field.replace('_', "-")
    )
}

#[cfg(test)]
impl TangleClientArgs {
    /// Construct a fully-specified args bundle for unit/integration tests.
    pub fn for_testing(
        http_rpc_url: Url,
        ws_rpc_url: Url,
        keystore_path: PathBuf,
        tangle_contract: impl Into<String>,
        staking_contract: impl Into<String>,
        status_registry_contract: Option<String>,
    ) -> Self {
        Self {
            http_rpc_url: Some(http_rpc_url),
            ws_rpc_url: Some(ws_rpc_url),
            keystore_path: Some(keystore_path),
            tangle_contract: Some(tangle_contract.into()),
            staking_contract: Some(staking_contract.into()),
            status_registry_contract,
            network: None,
            resolved: OnceCell::new(),
        }
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
    harness: SeededTangleTestnet,
    _temp_dir: TempDir,
    keystore_dir: PathBuf,
    data_dir: PathBuf,
}

impl DevnetStack {
    /// Spawn a deterministic local stack.
    pub async fn spawn(include_anvil_logs: bool) -> Result<Self> {
        let harness = TangleHarness::builder()
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
    pub fn staking_contract(&self) -> Address {
        self.harness.staking_contract
    }

    #[must_use]
    pub fn status_registry_contract(&self) -> Address {
        self.harness.status_registry_contract
    }

    /// Consume the stack and shut down resources. Cleanup happens via the
    /// default Drop impls of `TangleHarness` (stops anvil) and `TempDir`
    /// (removes the scratch dir).
    pub async fn shutdown(self) {
        drop(self);
    }
}

pub fn run_opts_from_stack(
    stack: &DevnetStack,
    settings: &TangleProtocolSettings,
    allow_unchecked_attestations: bool,
    method: SpawnMethod,
) -> RunOpts {
    RunOpts {
        http_rpc_url: stack.http_rpc_url(),
        ws_rpc_url: stack.ws_rpc_url(),
        blueprint_id: settings.blueprint_id,
        service_id: settings.service_id.or(Some(stack.default_service_id())),
        tangle_contract: stack.tangle_contract(),
        staking_contract: stack.staking_contract(),
        status_registry_contract: stack.status_registry_contract(),
        keystore_path: stack.keystore_path(),
        data_dir: Some(stack.data_dir()),
        allow_unchecked_attestations,
        registration_mode: false,
        registration_capture_only: false,
        preferred_source: method.preferred_source(),
        use_vm: method.use_vm(),
        dry_run: false,
        shutdown_after: None,
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
