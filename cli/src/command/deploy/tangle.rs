use crate::command::deploy::definition::{
    BinaryArtifactSpec, BlueprintDefinitionInput, DefinitionOverrides, FetcherKind,
    GithubArtifactSpec, NativeSourceOverride, RemoteArtifactSpec, SourceSummary,
    SourceSummaryDetails, load_blueprint_definition,
};
use crate::command::run::tangle::run_blueprint;
use crate::command::signer::KEYSTORE_PATH_ENV;
use crate::command::tangle::{DevnetStack, SpawnMethod, parse_address, run_opts_from_stack};
use crate::settings::load_protocol_settings;
use alloy_primitives::Address;
use blueprint_client_tangle::{
    TangleClient, TangleClientConfig, TangleSettings, TransactionResult,
};
use blueprint_runner::config::Protocol;
use blueprint_runner::tangle::config::TangleProtocolSettings;
use clap::{Args, ValueEnum};
use color_eyre::eyre::{Result, eyre};
use std::env;
use std::fmt;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

#[derive(Args, Debug, Clone)]
pub struct TangleDeployArgs {
    /// Target network for the deployment.
    #[arg(long, value_enum, default_value_t = DeploymentNetwork::Devnet)]
    pub network: DeploymentNetwork,
    /// Optional settings file with Tangle EVM configuration.
    #[arg(long, value_name = "FILE", default_value = "./settings.env")]
    pub settings_file: PathBuf,
    /// Stream Anvil stdout/stderr for debugging.
    #[arg(long)]
    pub include_anvil_logs: bool,
    /// Allow unchecked attestations when running the manager.
    #[arg(long)]
    pub allow_unchecked_attestations: bool,
    /// Preferred runtime for the service.
    #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
    pub spawn_method: SpawnMethod,
    /// Auto-shutdown the devnet run after the specified number of seconds.
    #[arg(long, value_name = "SECONDS")]
    pub exit_after_seconds: Option<u64>,
    /// Path to the blueprint definition file (JSON/YAML/TOML) when targeting non-devnet networks.
    #[arg(long, value_name = "FILE")]
    pub definition: Option<PathBuf>,
    /// Override the first native source with CLI-provided metadata.
    #[arg(long, value_enum)]
    pub artifact_source: Option<NativeArtifactSource>,
    /// Entry point for the overridden native source.
    #[arg(long)]
    pub artifact_entrypoint: Option<String>,
    /// Owner for GitHub artifact overrides.
    #[arg(long)]
    pub github_owner: Option<String>,
    /// Repository for GitHub artifact overrides.
    #[arg(long)]
    pub github_repo: Option<String>,
    /// Tag for GitHub artifact overrides.
    #[arg(long)]
    pub github_tag: Option<String>,
    /// Distribution manifest URL for HTTP/IPFS artifact overrides.
    #[arg(long)]
    pub remote_dist_url: Option<String>,
    /// Archive URL for HTTP/IPFS artifact overrides.
    #[arg(long)]
    pub remote_archive_url: Option<String>,
    /// Binary descriptors in the form NAME:ARCH:OS:SHA256[:BLAKE3].
    #[arg(long = "artifact-binary", value_name = "NAME:ARCH:OS:SHA256[:BLAKE3]")]
    pub artifact_binaries: Vec<String>,
    /// Override the HTTP RPC endpoint (non-devnet).
    #[arg(long)]
    pub http_rpc_url: Option<Url>,
    /// Override the WebSocket RPC endpoint (non-devnet).
    #[arg(long)]
    pub ws_rpc_url: Option<Url>,
    /// Override the keystore path (non-devnet).
    #[arg(long)]
    pub keystore_path: Option<PathBuf>,
    /// Override the Tangle contract address (non-devnet).
    #[arg(long)]
    pub tangle_contract: Option<String>,
    /// Override the MultiAssetDelegation contract address (non-devnet).
    #[arg(long)]
    pub restaking_contract: Option<String>,
    /// Override the OperatorStatusRegistry contract address (non-devnet).
    #[arg(long)]
    pub status_registry_contract: Option<String>,
}

impl TangleDeployArgs {
    fn definition_overrides(&self) -> Result<Option<DefinitionOverrides>> {
        let override_requested = self.artifact_source.is_some()
            || self.artifact_entrypoint.is_some()
            || !self.artifact_binaries.is_empty()
            || self.github_owner.is_some()
            || self.github_repo.is_some()
            || self.github_tag.is_some()
            || self.remote_dist_url.is_some()
            || self.remote_archive_url.is_some();

        if !override_requested {
            return Ok(None);
        }

        let fetcher = self.artifact_source.ok_or_else(|| {
            eyre!("--artifact-source must be provided when overriding native metadata")
        })?;

        let entrypoint = self
            .artifact_entrypoint
            .as_ref()
            .ok_or_else(|| {
                eyre!("--artifact-entrypoint is required when overriding native metadata")
            })?
            .trim()
            .to_string();

        if entrypoint.is_empty() {
            return Err(eyre!("--artifact-entrypoint must not be empty"));
        }

        if self.artifact_binaries.is_empty() {
            return Err(eyre!(
                "at least one --artifact-binary must be specified when overriding native metadata"
            ));
        }

        let binaries = self
            .artifact_binaries
            .iter()
            .map(|raw| parse_cli_binary(raw))
            .collect::<Result<Vec<_>>>()?;

        let override_spec = match fetcher {
            NativeArtifactSource::Github => {
                let owner = self
                    .github_owner
                    .as_ref()
                    .ok_or_else(|| eyre!("--github-owner is required for GitHub artifacts"))?
                    .clone();
                let repo = self
                    .github_repo
                    .as_ref()
                    .ok_or_else(|| eyre!("--github-repo is required for GitHub artifacts"))?
                    .clone();
                let tag = self
                    .github_tag
                    .as_ref()
                    .ok_or_else(|| eyre!("--github-tag is required for GitHub artifacts"))?
                    .clone();

                NativeSourceOverride::github(
                    entrypoint,
                    GithubArtifactSpec {
                        owner,
                        repo,
                        tag,
                        binaries,
                    },
                )
            }
            NativeArtifactSource::Http | NativeArtifactSource::Ipfs => {
                let dist_url = self
                    .remote_dist_url
                    .as_ref()
                    .ok_or_else(|| eyre!("--remote-dist-url is required for HTTP/IPFS artifacts"))?
                    .clone();
                let archive_url = self
                    .remote_archive_url
                    .as_ref()
                    .ok_or_else(|| {
                        eyre!("--remote-archive-url is required for HTTP/IPFS artifacts")
                    })?
                    .clone();

                NativeSourceOverride::remote(
                    entrypoint,
                    fetcher.to_fetcher_kind(),
                    RemoteArtifactSpec {
                        dist_url,
                        archive_url,
                        binaries,
                    },
                )
            }
        };

        let mut overrides = DefinitionOverrides::default();
        overrides.push_native(override_spec);
        Ok(Some(overrides))
    }
}

pub async fn execute(args: TangleDeployArgs) -> Result<()> {
    let protocol_settings = load_protocol_settings(Protocol::Tangle, &args.settings_file)
        .map_err(|e| eyre!(e.to_string()))?;
    let tangle_settings = protocol_settings
        .tangle()
        .map_err(|e| eyre!("failed to load Tangle settings: {e}"))?;
    let tangle_settings = tangle_settings.clone();

    let plan = match args.network {
        DeploymentNetwork::Devnet => {
            let stack = DevnetStack::spawn(args.include_anvil_logs).await?;
            DeploymentPlan::Devnet(DevnetDeployment::new(
                stack,
                tangle_settings,
                args.allow_unchecked_attestations,
                args.spawn_method,
                args.exit_after_seconds.map(Duration::from_secs),
            ))
        }
        DeploymentNetwork::Testnet | DeploymentNetwork::Mainnet => {
            let definition_path = args
                .definition
                .as_ref()
                .ok_or_else(|| eyre!("--definition is required for testnet/mainnet deployments"))?;
            let overrides = args.definition_overrides()?;
            let loaded = load_blueprint_definition(definition_path, overrides.as_ref())?;
            print_source_summaries(&loaded.summaries);
            let remote = NetworkDeploymentConfig::from_args(&args, &tangle_settings)?;
            DeploymentPlan::Network(NetworkDeployment::new(
                args.network,
                remote,
                loaded.definition,
            ))
        }
    };

    let outcome = plan.execute().await?;
    log_deployment_summary(&outcome);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum DeploymentNetwork {
    Devnet,
    Testnet,
    Mainnet,
}

impl fmt::Display for DeploymentNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeploymentNetwork::Devnet => write!(f, "devnet"),
            DeploymentNetwork::Testnet => write!(f, "testnet"),
            DeploymentNetwork::Mainnet => write!(f, "mainnet"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum NativeArtifactSource {
    Github,
    Http,
    Ipfs,
}

impl NativeArtifactSource {
    fn to_fetcher_kind(self) -> FetcherKind {
        match self {
            NativeArtifactSource::Github => FetcherKind::Github,
            NativeArtifactSource::Http => FetcherKind::Http,
            NativeArtifactSource::Ipfs => FetcherKind::Ipfs,
        }
    }
}

#[derive(Debug)]
struct DeploymentOutcome {
    network: DeploymentNetwork,
    blueprint_id: u64,
    service_id: Option<u64>,
    transactions: Vec<TransactionResult>,
}

impl DeploymentOutcome {
    fn new(
        network: DeploymentNetwork,
        blueprint_id: u64,
        service_id: Option<u64>,
        transactions: Vec<TransactionResult>,
    ) -> Self {
        Self {
            network,
            blueprint_id,
            service_id,
            transactions,
        }
    }
}

enum DeploymentPlan {
    Devnet(DevnetDeployment),
    Network(NetworkDeployment),
}

impl DeploymentPlan {
    async fn execute(self) -> Result<DeploymentOutcome> {
        match self {
            DeploymentPlan::Devnet(plan) => plan.execute().await,
            DeploymentPlan::Network(plan) => plan.execute().await,
        }
    }
}

fn print_source_summaries(summaries: &[SourceSummary]) {
    if summaries.is_empty() {
        println!("No blueprint sources were detected in the definition.");
        return;
    }

    println!("\nBlueprint sources:");
    for summary in summaries {
        let fetcher = summary
            .fetcher
            .map(|kind| kind.to_string())
            .unwrap_or_else(|| "n/a".into());
        let entrypoint = summary
            .entrypoint
            .as_deref()
            .filter(|value| !value.is_empty())
            .unwrap_or("n/a");
        println!(
            "  [{}] kind: {}, fetcher: {fetcher}, entrypoint: {entrypoint}",
            summary.index + 1,
            summary.kind
        );

        match &summary.details {
            SourceSummaryDetails::Container {
                registry,
                image,
                tag,
            } => {
                println!("      image: {registry}/{image}:{tag}");
            }
            SourceSummaryDetails::Native { has_testing } => {
                if *has_testing {
                    println!("      includes testing harness");
                }
            }
            SourceSummaryDetails::Wasm { runtime } => {
                println!("      runtime: {:?}", runtime);
            }
        }
    }
    println!();
}

fn parse_cli_binary(value: &str) -> Result<BinaryArtifactSpec> {
    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.len() < 4 || parts.len() > 5 {
        return Err(eyre!(
            "invalid --artifact-binary `{value}`; expected NAME:ARCH:OS:SHA256[:BLAKE3]"
        ));
    }
    let blake3 = if parts.len() == 5 {
        Some(parts.pop().unwrap().to_string())
    } else {
        None
    };
    let sha256 = parts.pop().unwrap().to_string();
    let os = parts.pop().unwrap().to_string();
    let arch = parts.pop().unwrap().to_string();
    let name = parts.pop().unwrap().to_string();
    Ok(BinaryArtifactSpec {
        name,
        arch,
        os,
        sha256,
        blake3,
    })
}

#[derive(Debug)]
struct DevnetDeployment {
    stack: DevnetStack,
    settings: TangleProtocolSettings,
    allow_unchecked_attestations: bool,
    spawn_method: SpawnMethod,
    shutdown_after: Option<Duration>,
}

impl DevnetDeployment {
    fn new(
        stack: DevnetStack,
        settings: TangleProtocolSettings,
        allow_unchecked_attestations: bool,
        spawn_method: SpawnMethod,
        shutdown_after: Option<Duration>,
    ) -> Self {
        Self {
            stack,
            settings,
            allow_unchecked_attestations,
            spawn_method,
            shutdown_after,
        }
    }

    async fn execute(self) -> Result<DeploymentOutcome> {
        let Self {
            stack,
            settings,
            allow_unchecked_attestations,
            spawn_method,
            shutdown_after,
        } = self;

        println!(
            "Deploying blueprint to local Anvil devnet at HTTP {} / WS {}",
            stack.http_rpc_url(),
            stack.ws_rpc_url()
        );

        let mut run_opts = run_opts_from_stack(
            &stack,
            &settings,
            allow_unchecked_attestations,
            spawn_method,
        );
        run_opts.shutdown_after = shutdown_after;
        let service_id = run_opts.service_id;
        let run_result = run_blueprint(run_opts).await;

        stack.shutdown().await;
        run_result?;

        Ok(DeploymentOutcome::new(
            DeploymentNetwork::Devnet,
            settings.blueprint_id,
            service_id,
            Vec::new(),
        ))
    }
}

#[derive(Debug)]
struct NetworkDeployment {
    network: DeploymentNetwork,
    rpc: NetworkDeploymentConfig,
    definition: BlueprintDefinitionInput,
}

impl NetworkDeployment {
    fn new(
        network: DeploymentNetwork,
        rpc: NetworkDeploymentConfig,
        definition: BlueprintDefinitionInput,
    ) -> Self {
        Self {
            network,
            rpc,
            definition,
        }
    }

    async fn execute(self) -> Result<DeploymentOutcome> {
        let Self {
            network,
            rpc,
            definition,
        } = self;

        println!(
            "Deploying blueprint definition (metadata {}) to {}",
            definition.metadata_uri, network
        );

        let client = rpc.connect_client().await?;
        let (tx, blueprint_id) = client
            .create_blueprint(definition.encoded_bytes().to_vec())
            .await?;

        Ok(DeploymentOutcome::new(
            network,
            blueprint_id,
            None,
            vec![tx],
        ))
    }
}

#[derive(Clone, Debug)]
struct NetworkDeploymentConfig {
    http_rpc_url: Url,
    ws_rpc_url: Url,
    keystore_path: PathBuf,
    tangle_contract: Address,
    restaking_contract: Address,
    status_registry_contract: Address,
}

impl NetworkDeploymentConfig {
    fn from_args(args: &TangleDeployArgs, settings: &TangleProtocolSettings) -> Result<Self> {
        let http_rpc_url = infer_url(
            args.http_rpc_url.clone(),
            HTTP_RPC_URL_ENV,
            "--http-rpc-url",
        )?;
        let ws_rpc_url = infer_url(args.ws_rpc_url.clone(), WS_RPC_URL_ENV, "--ws-rpc-url")?;
        let keystore_path = resolve_keystore_path(args.keystore_path.clone());
        let tangle_contract = parse_contract_override(
            args.tangle_contract.as_deref(),
            settings.tangle_contract,
            "TANGLE_CONTRACT",
            "--tangle-contract",
        )?;
        let restaking_contract = parse_contract_override(
            args.restaking_contract.as_deref(),
            settings.restaking_contract,
            "RESTAKING_CONTRACT",
            "--restaking-contract",
        )?;
        let status_registry_contract = parse_contract_override(
            args.status_registry_contract.as_deref(),
            settings.status_registry_contract,
            "STATUS_REGISTRY_CONTRACT",
            "--status-registry-contract",
        )?;

        Ok(Self {
            http_rpc_url,
            ws_rpc_url,
            keystore_path,
            tangle_contract,
            restaking_contract,
            status_registry_contract,
        })
    }

    async fn connect_client(&self) -> Result<TangleClient> {
        if !self.keystore_path.exists() {
            return Err(eyre!(
                "Keystore {} not found; pass --keystore-path or set {KEYSTORE_PATH_ENV}",
                self.keystore_path.display()
            ));
        }

        let settings = TangleSettings {
            blueprint_id: 0,
            service_id: None,
            tangle_contract: self.tangle_contract,
            restaking_contract: self.restaking_contract,
            status_registry_contract: self.status_registry_contract,
        };
        let config = TangleClientConfig::new(
            self.http_rpc_url.clone(),
            self.ws_rpc_url.clone(),
            self.keystore_path.display().to_string(),
            settings,
        );
        TangleClient::new(config)
            .await
            .map_err(|e| eyre!(e.to_string()))
    }
}

fn infer_url(value: Option<Url>, env_key: &str, flag: &str) -> Result<Url> {
    if let Some(url) = value {
        return Ok(url);
    }

    match env::var(env_key) {
        Ok(raw) => Url::parse(&raw).map_err(|err| eyre!("Invalid {env_key} URL: {err}")),
        Err(_) => Err(eyre!(
            "Missing RPC endpoint. Set {env_key} in settings.env or pass {flag}."
        )),
    }
}

fn resolve_keystore_path(explicit: Option<PathBuf>) -> PathBuf {
    if let Some(path) = explicit {
        return path;
    }

    if let Ok(raw) = env::var(KEYSTORE_PATH_ENV) {
        return PathBuf::from(raw);
    }

    if let Ok(raw) = env::var("KEYSTORE_URI") {
        return PathBuf::from(raw);
    }

    PathBuf::from("./keystore")
}

fn parse_contract_override(
    value: Option<&str>,
    fallback: Address,
    env_key: &str,
    flag: &str,
) -> Result<Address> {
    if let Some(raw) = value {
        return parse_address(raw, env_key);
    }

    if fallback != Address::ZERO {
        return Ok(fallback);
    }

    Err(eyre!(
        "Missing {env_key}. Set it in settings.env or pass {flag}."
    ))
}

fn log_deployment_summary(outcome: &DeploymentOutcome) {
    println!(
        "\nDeployment complete → network={} blueprint={} service={}",
        outcome.network,
        outcome.blueprint_id,
        outcome
            .service_id
            .map_or_else(|| "-".to_string(), |id| id.to_string())
    );

    if outcome.transactions.is_empty() {
        println!("No transactions were submitted for this deployment.");
        return;
    }

    println!("Submitted transactions:");
    for tx in &outcome.transactions {
        println!(
            "  tx={:#x} block={:?} success={}",
            tx.tx_hash, tx.block_number, tx.success
        );
    }
}

const HTTP_RPC_URL_ENV: &str = "HTTP_RPC_URL";
const WS_RPC_URL_ENV: &str = "WS_RPC_URL";
