use std::path::PathBuf;
use cargo_tangle::command::{create, deploy, debug};
use blueprint_runner::config::{BlueprintEnvironment, Protocol, ProtocolSettings, SupportedChains};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use blueprint_runner::error::ConfigError;
use blueprint_runner::tangle::config::TangleProtocolSettings;
use cargo_tangle::command::create::{new_blueprint, BlueprintType};
use cargo_tangle::command::deploy::eigenlayer::deploy_eigenlayer;
use cargo_tangle::command::deploy::tangle::deploy_tangle;
use cargo_tangle::command::jobs::submit::submit_job;
use cargo_tangle::command::list::blueprints::{list_blueprints, print_blueprints};
use cargo_tangle::command::list::requests::{list_requests, print_requests};
use cargo_tangle::command::register::register;
use cargo_tangle::command::run::run_eigenlayer_avs;
use cargo_tangle::command::run::tangle::{run_blueprint, RunOpts};
use cargo_tangle::command::service::accept::accept_request;
use cargo_tangle::command::service::reject::reject_request;
use cargo_tangle::command::service::request::request_service;
use cargo_tangle::command::keys::{export_key, generate_key, generate_mnemonic, import_key, list_keys, prompt_for_keys};
use clap::{Parser, Subcommand};
use dotenv::from_path;
use tangle_subxt::subxt::blocks::ExtrinsicEvents;
use tangle_subxt::subxt::client::OnlineClientT;
use tangle_subxt::subxt::Config;
use tangle_subxt::subxt::tx::TxProgress;
use tangle_subxt::subxt_core::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::assets::events::created::AssetId;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_arithmetic::per_things::Percent;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::{Asset, AssetSecurityCommitment};
use url::Url;
use blueprint_crypto::KeyTypeId;
use blueprint_crypto::sp_core::{SpEcdsa, SpSr25519};
use blueprint_crypto_core::KeyType;
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_keystore::backends::Backend;
use blueprint_std::env;
use cargo_tangle::command::debug::spawn::ServiceSpawnMethod;

/// Tangle CLI tool
#[derive(Parser, Debug)]
#[clap(
    bin_name = "cargo-tangle",
    version,
    propagate_version = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
    #[command(flatten)]
    features: clap_cargo::Features,
    #[command(subcommand)]
    command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
enum Commands {
    /// Blueprint subcommand
    #[command(visible_alias = "bp")]
    Blueprint {
        #[command(subcommand)]
        command: BlueprintCommands,
    },

    /// Key management
    #[command(visible_alias = "k")]
    Key {
        #[command(subcommand)]
        command: KeyCommands,
    },

    /// Service debugging
    #[command(visible_alias = "d")]
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum KeyCommands {
    /// Generate a new key
    #[command(visible_alias = "g")]
    Generate {
        /// The type of key to generate (sr25519, ed25519, ecdsa, bls381, bls377, bn254)
        #[arg(short = 't', long, value_enum)]
        key_type: KeyTypeId,
        /// The path to save the key to
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
        /// The seed to use for key generation (hex format without 0x prefix)
        #[arg(long)]
        seed: Option<Vec<u8>>,
        /// Show the secret key in output
        #[arg(short = 'v', long)]
        show_secret: bool,
    },
    /// Import a key into the keystore
    #[command(visible_alias = "i")]
    Import {
        /// The type of key to import (sr25519, ed25519, ecdsa, bls381, bls377, bn254)
        #[arg(short = 't', long, value_enum)]
        key_type: Option<KeyTypeId>,
        /// The secret key to import (hex format without 0x prefix)
        #[arg(short = 'x', long)]
        secret: Option<String>,
        /// The path to the keystore
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
        /// The protocol you are generating keys for (Eigenlayer or Tangle). Only matters for some keys.
        #[arg(short = 'p', long, default_value = "tangle")]
        protocol: Protocol,
    },
    /// Export a key from the keystore
    #[command(visible_alias = "e")]
    Export {
        /// The type of key to export (sr25519, ed25519, ecdsa, bls381, bls377, bn254)
        #[arg(short = 't', long, value_enum)]
        key_type: KeyTypeId,
        /// The public key to export (hex format without 0x prefix)
        #[arg(short = 'p', long)]
        public: String,
        /// The path to the keystore
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
    },
    /// List all keys in the keystore
    #[command(visible_alias = "l")]
    List {
        /// The path to the keystore
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
    },
    /// Generate a new mnemonic phrase
    #[command(visible_alias = "m")]
    GenerateMnemonic {
        /// Number of words in the mnemonic (12, 15, 18, 21, or 24)
        #[arg(short = 'w', long, value_parser = clap::value_parser!(u32).range(12..=24))]
        word_count: Option<u32>,
    },
}

#[derive(Subcommand, Debug)]
pub enum BlueprintCommands {
    /// Create a new blueprint
    #[command(visible_alias = "c")]
    Create {
        /// The name of the blueprint
        #[arg(short = 'n', long, value_name = "NAME", env = "NAME")]
        name: String,

        #[command(flatten)]
        source: Option<create::Source>,

        #[command(flatten)]
        blueprint_type: Option<BlueprintType>,

        /// Define a value for template variables (can be used multiple times)
        /// Example: --define gh-username=myusername
        /// Example with spaces: --define "project-description=My Blueprint description"
        #[arg(
            long,
            short = 'd',
            number_of_values = 1,
            conflicts_with = "template_values_file"
        )]
        define: Vec<String>,

        /// Path to a file containing template values
        /// File should contain key=value pairs, one per line
        #[arg(long, value_name = "FILE", conflicts_with = "define")]
        template_values_file: Option<String>,

        /// Skip all interactive prompts, using defaults for any variables not provided with `--define` or `--template-values-file`
        #[arg(long)]
        skip_prompts: bool,
    },

    /// Deploy a blueprint to the Tangle Network or Eigenlayer.
    #[command(visible_alias = "d")]
    Deploy {
        #[command(subcommand)]
        target: DeployTarget,
    },

    /// Run a blueprint
    #[command(visible_alias = "r")]
    Run {
        /// The protocol to run (eigenlayer or tangle)
        #[arg(short = 'p', long, value_enum)]
        protocol: Protocol,

        /// The HTTP RPC endpoint URL (required)
        #[arg(short = 'u', long, default_value = "http://127.0.0.1:9944")]
        rpc_url: Url,

        /// The keystore path (defaults to ./keystore)
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,

        /// The path to the AVS binary
        ///
        /// If not provided, the binary will be built if possible
        #[arg(short = 'b', long)]
        binary_path: Option<PathBuf>,

        /// The network to connect to (local, testnet, mainnet)
        #[arg(short = 'w', long, default_value = "local")]
        network: String,

        /// The data directory path (defaults to ./data)
        #[arg(short = 'd', long)]
        data_dir: Option<PathBuf>,

        /// Optional bootnodes to connect to
        #[arg(short = 'n', long)]
        bootnodes: Option<Vec<String>>,

        /// Path to the protocol settings env file
        #[arg(short = 'f', long, default_value = "./settings.env")]
        settings_file: Option<PathBuf>,

        /// Whether to allow invalid GitHub attestations (binary integrity checks)
        ///
        /// This will also allow for running the manager without the GitHub CLI installed.
        #[arg(long, env)]
        allow_unchecked_attestations: bool,
    },

    /// List service requests for a Tangle blueprint
    #[command(visible_alias = "ls")]
    ListRequests {
        /// WebSocket RPC URL to use
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: Url,
    },

    /// List Blueprints on target Tangle network
    #[command(visible_alias = "lb")]
    ListBlueprints {
        /// WebSocket RPC URL to use
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: String,
    },

    /// Register for a Tangle blueprint
    #[command(visible_alias = "reg")]
    Register {
        /// WebSocket RPC URL to use
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: String,
        /// The blueprint ID to register
        #[arg(long)]
        blueprint_id: u64,
        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,
        /// The URL of the pricing RPC
        #[arg(long, env = "PRICING_RPC_URL", default_value = "ws://127.0.0.1:9000")]
        pricing_rpc_address: Url,
    },

    /// Accept a Tangle service request
    #[command(visible_alias = "accept")]
    AcceptRequest {
        /// WebSocket RPC URL to use
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: String,
        /// The minimum exposure percentage to request
        #[arg(long, default_value = "50")]
        min_exposure_percent: u8,
        /// The maximum exposure percentage to request
        #[arg(long, default_value = "80")]
        max_exposure_percent: u8,
        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,
        /// The restaking percentage to use
        #[arg(long, default_value = "50")]
        restaking_percent: u8,
        /// The request ID to respond to
        #[arg(long)]
        request_id: u64,
    },

    /// Reject a Tangle service request
    #[command(visible_alias = "reject")]
    RejectRequest {
        /// WebSocket RPC URL to use
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: String,
        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,
        /// The request ID to respond to
        #[arg(long)]
        request_id: u64,
    },

    /// Request a Tangle service
    #[command(visible_alias = "req")]
    RequestService {
        /// WebSocket RPC URL to use
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: String,
        /// The blueprint ID to request
        #[arg(long)]
        blueprint_id: u64,
        /// The minimum exposure percentage to request
        #[arg(long, default_value = "50")]
        min_exposure_percent: u8,
        /// The maximum exposure percentage to request
        #[arg(long, default_value = "80")]
        max_exposure_percent: u8,
        /// The target operators to request
        #[arg(long)]
        target_operators: Vec<AccountId32>,
        /// The value to request
        #[arg(long)]
        value: u128,
        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,
        /// Optional path to a JSON file containing request parameters
        #[arg(long)]
        params_file: Option<String>,
    },

    /// Submit a job to a service
    #[command(name = "submit")]
    SubmitJob {
        /// The RPC endpoint to connect to
        #[arg(long, env = "WS_RPC_URL", default_value = "ws://127.0.0.1:9944")]
        ws_rpc_url: String,
        /// The service ID to submit the job to
        #[arg(long)]
        service_id: Option<u64>,
        /// The blueprint ID to submit the job to
        #[arg(long)]
        blueprint_id: u64,
        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI")]
        keystore_uri: String,
        /// The job ID to submit
        #[arg(long)]
        job: u8,
        /// Optional path to a JSON file containing job parameters
        #[arg(long)]
        params_file: Option<String>,
        /// Whether to wait for the job to complete
        #[arg(long)]
        watcher: bool,
    },

    /// Deploy a Master Blueprint Service Manager (MBSM) contract to the Tangle Network
    #[command(visible_alias = "mbsm")]
    DeployMBSM {
        /// The HTTP RPC URL to use
        #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:9944", env)]
        http_rpc_url: Url,

        /// Force deployment even if the contract is already deployed
        #[arg(short, long, value_name = "VALUE", default_value_t = false)]
        force: bool,
    },

    /// EigenLayer AVS management commands
    #[command(visible_alias = "el")]
    Eigenlayer {
        #[command(subcommand)]
        command: EigenlayerCommands,
    },
}

/// EigenLayer AVS management commands
#[derive(Subcommand, Debug)]
pub enum EigenlayerCommands {
    /// Register with a new EigenLayer AVS
    #[command(visible_alias = "reg")]
    Register {
        /// Path to the AVS registration configuration file (JSON)
        #[arg(long, value_name = "FILE")]
        config: PathBuf,

        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,

        /// Runtime target for blueprint execution (native, hypervisor)
        /// Defaults to hypervisor for production. Use 'native' for local testing only.
        /// Note: Hypervisor requires Linux/KVM. Container support coming soon.
        #[arg(long, value_name = "RUNTIME")]
        runtime: Option<String>,

        /// Perform on-chain verification after registration
        #[arg(long)]
        verify: bool,
    },

    /// Deregister from an EigenLayer AVS
    #[command(visible_alias = "dereg")]
    Deregister {
        /// Service manager address of the AVS to deregister from
        #[arg(long, value_name = "ADDRESS")]
        service_manager: String,

        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,
    },

    /// List all registered EigenLayer AVS services
    #[command(visible_alias = "ls")]
    List {
        /// Show only active registrations
        #[arg(long)]
        active_only: bool,

        /// Output format (json, table)
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Synchronize local registrations with on-chain state
    Sync {
        /// HTTP RPC endpoint for EigenLayer contracts
        #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:8545")]
        http_rpc_url: Url,

        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,

        /// Path to the protocol settings file
        #[arg(long, value_name = "FILE")]
        settings_file: Option<PathBuf>,
    },

    /// Show rewards for an earner address
    #[command(visible_alias = "show")]
    ShowRewards {
        /// Earner Ethereum address
        #[arg(long, value_name = "ADDRESS")]
        earner_address: String,

        /// Sidecar API URL (defaults to mainnet)
        #[arg(long, value_name = "URL")]
        sidecar_url: Option<String>,

        /// Network (mainnet or holesky)
        #[arg(long, default_value = "mainnet")]
        network: String,
    },

    /// Claim rewards for an earner
    #[command(visible_alias = "claim")]
    ClaimRewards {
        /// Earner Ethereum address
        #[arg(long, value_name = "ADDRESS")]
        earner_address: String,

        /// Recipient address (defaults to earner)
        #[arg(long, value_name = "ADDRESS")]
        recipient_address: Option<String>,

        /// Token addresses to claim (empty = all)
        #[arg(long, value_delimiter = ',')]
        tokens: Vec<String>,

        /// `RewardsCoordinator` contract address
        #[arg(long, value_name = "ADDRESS")]
        rewards_coordinator: String,

        /// Sidecar API URL (defaults to mainnet)
        #[arg(long, value_name = "URL")]
        sidecar_url: Option<String>,

        /// Network (mainnet or holesky)
        #[arg(long, default_value = "mainnet")]
        network: String,

        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,

        /// HTTP RPC endpoint
        #[arg(long, default_value = "http://127.0.0.1:8545")]
        rpc_url: String,

        /// Batch claim file (YAML)
        #[arg(long, value_name = "FILE")]
        batch_file: Option<String>,
    },

    /// Set claimer address for the operator
    #[command(visible_alias = "set-claimer")]
    SetClaimer {
        /// Claimer Ethereum address
        #[arg(long, value_name = "ADDRESS")]
        claimer_address: String,

        /// `RewardsCoordinator` contract address
        #[arg(long, value_name = "ADDRESS")]
        rewards_coordinator: String,

        /// The keystore URI to use
        #[arg(long, env = "KEYSTORE_URI", default_value = "./keystore")]
        keystore_uri: String,

        /// HTTP RPC endpoint
        #[arg(long, default_value = "http://127.0.0.1:8545")]
        rpc_url: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum DeployTarget {
    /// Deploy to Tangle Network
    Tangle {
        /// HTTP RPC URL to use
        #[arg(
            long,
            value_name = "URL",
            default_value = "https://rpc.tangle.tools",
            env,
            required_unless_present = "devnet"
        )]
        http_rpc_url: String,
        /// Tangle RPC URL to use
        #[arg(
            long,
            value_name = "URL",
            default_value = "wss://rpc.tangle.tools",
            env,
            required_unless_present = "devnet"
        )]
        ws_rpc_url: String,
        /// The package to deploy (if the workspace has multiple packages).
        #[arg(short = 'p', long, value_name = "PACKAGE", env = "CARGO_PACKAGE")]
        package: Option<String>,
        /// Start a local devnet using a Tangle test node
        #[arg(long)]
        devnet: bool,
        /// The keystore path (defaults to ./keystore)
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
    },
    /// Deploy to Eigenlayer
    Eigenlayer {
        /// HTTP RPC URL to use
        #[arg(long, value_name = "URL", env, required_unless_present = "devnet")]
        rpc_url: Option<String>,
        /// Path to the contracts
        #[arg(long)]
        contracts_path: Option<String>,
        /// Whether to deploy contracts in an interactive ordered manner
        #[arg(long)]
        ordered_deployment: bool,
        /// Network to deploy to (local, testnet, mainnet)
        #[arg(short = 'w', long, default_value = "local")]
        network: String,
        /// Start a local devnet using Anvil (only valid with network=local)
        #[arg(long)]
        devnet: bool,
        /// The keystore path (defaults to ./keystore)
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum DebugCommands {
    Spawn {
        /// HTTP RPC URL to use
        #[arg(long, value_name = "URL", env)]
        http_rpc_url: Option<Url>,
        /// WS RPC URL to use
        #[arg(long, value_name = "URL", env)]
        ws_rpc_url: Option<Url>,
        /// The package to deploy (if the workspace has multiple packages).
        #[arg(short = 'p', long, value_name = "PACKAGE", env = "CARGO_PACKAGE")]
        package: Option<String>,

        /// The ID of the service to spawn
        #[arg(default_value_t = 0)]
        id: u32,
        #[arg(default_value = "service")]
        service_name: String,
        #[arg(long, required_if_eq_any([("method", "native"), ("method", "vm")]))]
        binary: Option<PathBuf>,
        #[arg(long, conflicts_with = "binary", required_if_eq("method", "container"))]
        image: Option<String>,
        #[arg(long, default_value_t = Protocol::Tangle)]
        protocol: Protocol,
        /// How to run the service
        #[arg(value_enum, long, default_value_t = ServiceSpawnMethod::Native)]
        method: ServiceSpawnMethod,
        /// Verify network connection before starting the service
        #[arg(long, default_value_t = true)]
        #[cfg(feature = "vm-debug")]
        verify_network_connection: bool,
    },
}

#[tokio::main]
#[allow(clippy::needless_return)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_tracing_subscriber();
    // Install the default crypto provider for rustls
    cargo_tangle::install_crypto_provider();
    let args: Vec<String> = if std::env::args().nth(1).is_some_and(|x| x.eq("tangle")) {
        // since this runs as a cargo subcommand, we need to skip the first argument
        // to get the actual arguments for the subcommand
        std::env::args().skip(1).collect()
    } else {
        std::env::args().collect()
    };

    // Parse the CLI arguments
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Blueprint { command } => match command {
            BlueprintCommands::Create {
                name,
                source,
                blueprint_type,
                define,
                template_values_file,
                skip_prompts,
            } => {
                new_blueprint(
                    &name,
                    source,
                    blueprint_type,
                    define,
                    &template_values_file,
                    skip_prompts,
                )?;
            }
            BlueprintCommands::Deploy { target } => match target {
                DeployTarget::Tangle {
                    http_rpc_url,
                    ws_rpc_url,
                    package,
                    devnet,
                    keystore_path,
                } => {
                    let manifest_path = cli
                        .manifest
                        .manifest_path
                        .unwrap_or_else(|| PathBuf::from("Cargo.toml"));
                    Box::pin(deploy_tangle(
                        http_rpc_url,
                        ws_rpc_url,
                        package,
                        devnet,
                        keystore_path,
                        manifest_path,
                    ))
                    .await?;
                }
                DeployTarget::Eigenlayer {
                    rpc_url,
                    contracts_path,
                    ordered_deployment,
                    network,
                    devnet,
                    keystore_path,
                } => {
                    deploy_eigenlayer(
                        rpc_url,
                        contracts_path,
                        ordered_deployment,
                        network,
                        devnet,
                        keystore_path,
                    )
                    .await?;
                }
            },
            BlueprintCommands::Run {
                protocol,
                rpc_url,
                keystore_path,
                binary_path: _,
                network,
                data_dir,
                bootnodes,
                settings_file,
                allow_unchecked_attestations,
            } => {
                let settings_file =
                    settings_file.unwrap_or_else(|| PathBuf::from("./settings.env"));
                let protocol_settings = if settings_file.exists() {
                    load_protocol_settings(protocol, &settings_file)?
                } else if protocol == Protocol::Tangle {
                    println!("Please enter the Blueprint ID:");
                    let mut blueprint_id = String::new();
                    std::io::stdin().read_line(&mut blueprint_id)?;
                    let blueprint_id: u64 = blueprint_id.trim().parse()?;
                    println!("Please enter the Service ID:");
                    let mut service_id = String::new();
                    std::io::stdin().read_line(&mut service_id)?;
                    let service_id: u64 = service_id.trim().parse()?;
                    ProtocolSettings::Tangle(TangleProtocolSettings {
                        blueprint_id,
                        service_id: Some(service_id),
                    })
                } else {
                    return Err(color_eyre::Report::msg(format!(
                        "The --settings-file flag needs to be provided with a valid path, or the file `{}` needs to exist",
                        settings_file.display()
                    )));
                };

                let chain = match network.to_lowercase().as_str() {
                    "local" => SupportedChains::LocalTestnet,
                    "testnet" => SupportedChains::Testnet,
                    "mainnet" => {
                        if rpc_url.as_str().contains("127.0.0.1")
                            || rpc_url.as_str().contains("localhost")
                        {
                            SupportedChains::LocalMainnet
                        } else {
                            SupportedChains::Mainnet
                        }
                    }
                    _ => {
                        return Err(color_eyre::Report::msg(format!(
                            "Invalid network: {}",
                            network
                        )));
                    }
                };

                let mut config = BlueprintEnvironment::default();

                let mut ws_url = rpc_url.clone();
                match rpc_url.scheme() {
                    "http" => ws_url.set_scheme("ws").unwrap(),
                    "https" => ws_url.set_scheme("wss").unwrap(),
                    _ => {
                        return Err(color_eyre::Report::msg(format!(
                            "Invalid scheme: {}",
                            rpc_url.scheme()
                        )));
                    }
                }

                config.http_rpc_endpoint = rpc_url.clone();
                config.ws_rpc_endpoint = ws_url;
                let keystore_path = keystore_path.unwrap_or_else(|| PathBuf::from("./keystore"));
                if !keystore_path.exists() {
                    println!(
                        "Keystore not found at {}. Let's set up your keys.",
                        keystore_path.display()
                    );
                    let keys = prompt_for_keys(vec![KeyTypeId::Sr25519, KeyTypeId::Ecdsa])?;
                    let keystore = Keystore::new(KeystoreConfig::new().fs_root(&keystore_path))?;
                    for (key_type, key) in keys {
                        match key_type {
                            KeyTypeId::Sr25519 => {
                                let key = SpSr25519::generate_with_string(key)?;
                                keystore.insert::<SpSr25519>(&key)?;
                            }
                            KeyTypeId::Ecdsa => {
                                let key = SpEcdsa::generate_with_string(key)?;
                                keystore.insert::<SpEcdsa>(&key)?;
                            }
                            _ => {}
                        }
                    }
                }
                config.keystore_uri = keystore_path.to_string_lossy().to_string();
                config.bootnodes = bootnodes
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|addr| addr.parse().ok())
                    .collect();
                config.protocol_settings = protocol_settings.clone();
                config.test_mode = network == "local";

                match protocol {
                    Protocol::Eigenlayer => {
                        run_eigenlayer_avs(
                            config,
                            chain,
                            None, // keystore_path already set in config
                            data_dir,
                            allow_unchecked_attestations,
                        )
                        .await?;
                    }
                    Protocol::Tangle => {
                        // Create the run options for the Tangle blueprint
                        let run_opts = RunOpts {
                            http_rpc_url: config.http_rpc_endpoint.clone(),
                            ws_rpc_url: config.ws_rpc_endpoint.clone(),
                            signer: None, // We'll get the signer from the keystore
                            signer_evm: None, // We'll get the signer from the keystore
                            blueprint_id: Some(
                                protocol_settings
                                    .tangle().map(|t| t.blueprint_id)
                                    .map_err(|e| color_eyre::Report::msg(format!("Blueprint ID is required in the protocol settings: {e:?}")))?,
                            ),
                            keystore_path: Some(config.keystore_uri.clone()),
                            data_dir,
                            allow_unchecked_attestations,
                        };

                        // Run the blueprint
                        run_blueprint(run_opts).await?;
                    }
                    _ => {
                        return Err(ConfigError::UnsupportedProtocol(protocol.to_string()).into());
                    }
                }
            }
            BlueprintCommands::ListRequests { ws_rpc_url } => {
                let requests = list_requests(ws_rpc_url.to_string()).await?;
                print_requests(requests);
            }
            BlueprintCommands::ListBlueprints { ws_rpc_url } => {
                let blueprints = list_blueprints(ws_rpc_url.to_string()).await?;
                print_blueprints(blueprints);
            }
            BlueprintCommands::Register {
                ws_rpc_url,
                blueprint_id,
                keystore_uri,
                pricing_rpc_address,
            } => register(ws_rpc_url, blueprint_id, keystore_uri, pricing_rpc_address).await?,
            BlueprintCommands::AcceptRequest {
                ws_rpc_url,
                min_exposure_percent,
                max_exposure_percent,
                restaking_percent,
                keystore_uri,
                request_id,
            } => {
                accept_request(
                    ws_rpc_url,
                    min_exposure_percent,
                    max_exposure_percent,
                    restaking_percent,
                    keystore_uri,
                    request_id,
                )
                .await?;
            }
            BlueprintCommands::RejectRequest {
                ws_rpc_url,
                keystore_uri,
                request_id,
            } => {
                reject_request(ws_rpc_url, keystore_uri, request_id).await?;
            }
            BlueprintCommands::RequestService {
                ws_rpc_url,
                blueprint_id,
                min_exposure_percent,
                max_exposure_percent,
                target_operators,
                value,
                keystore_uri,
                params_file,
            } => {
                request_service(
                    ws_rpc_url,
                    blueprint_id,
                    min_exposure_percent,
                    max_exposure_percent,
                    target_operators,
                    value,
                    keystore_uri,
                    params_file,
                )
                .await?;
            }
            BlueprintCommands::SubmitJob {
                ws_rpc_url,
                service_id,
                blueprint_id,
                keystore_uri,
                job,
                params_file,
                watcher,
            } => {
                submit_job(
                    ws_rpc_url,
                    service_id,
                    blueprint_id,
                    keystore_uri,
                    job,
                    params_file,
                    watcher,
                )
                .await?;
            }
            BlueprintCommands::DeployMBSM {
                http_rpc_url,
                force,
            } => {
                deploy::mbsm::deploy_mbsm(http_rpc_url, force).await?;
            }

            BlueprintCommands::Eigenlayer { command } => match command {
                EigenlayerCommands::Register {
                    config,
                    keystore_uri,
                    runtime,
                    verify,
                } => {
                    cargo_tangle::command::eigenlayer::register_avs(
                        &config,
                        &keystore_uri,
                        runtime.as_deref(),
                        verify,
                    )
                    .await?;
                }
                EigenlayerCommands::Deregister {
                    service_manager,
                    keystore_uri,
                } => {
                    cargo_tangle::command::eigenlayer::deregister_avs(
                        &service_manager,
                        &keystore_uri,
                    )
                    .await?;
                }
                EigenlayerCommands::List {
                    active_only,
                    format,
                } => {
                    cargo_tangle::command::eigenlayer::list_avs_registrations(active_only, &format)
                        .await?;
                }
                EigenlayerCommands::Sync {
                    http_rpc_url,
                    keystore_uri,
                    settings_file,
                } => {
                    cargo_tangle::command::eigenlayer::sync_avs_registrations(
                        &http_rpc_url,
                        &keystore_uri,
                        settings_file.as_deref(),
                    )
                    .await?;
                }
                EigenlayerCommands::ShowRewards {
                    earner_address,
                    sidecar_url,
                    network,
                } => {
                    cargo_tangle::command::eigenlayer::show_rewards(
                        &earner_address,
                        sidecar_url.as_deref(),
                        Some(&network),
                    )
                    .await?;
                }
                EigenlayerCommands::ClaimRewards {
                    earner_address,
                    recipient_address,
                    tokens,
                    rewards_coordinator,
                    sidecar_url,
                    network,
                    keystore_uri,
                    rpc_url,
                    batch_file,
                } => {
                    use alloy_primitives::Address;
                    let rewards_coord_addr = Address::parse_checksummed(&rewards_coordinator, None)
                        .map_err(|e| {
                            color_eyre::eyre::eyre!("Invalid rewards coordinator address: {}", e)
                        })?;

                    cargo_tangle::command::eigenlayer::claim_rewards(
                        &earner_address,
                        recipient_address.as_deref(),
                        tokens,
                        rewards_coord_addr,
                        sidecar_url.as_deref(),
                        Some(&network),
                        &keystore_uri,
                        &rpc_url,
                        batch_file.as_deref(),
                    )
                    .await?;
                }
                EigenlayerCommands::SetClaimer {
                    claimer_address,
                    rewards_coordinator,
                    keystore_uri,
                    rpc_url,
                } => {
                    use alloy_primitives::Address;
                    let rewards_coord_addr = Address::parse_checksummed(&rewards_coordinator, None)
                        .map_err(|e| {
                            color_eyre::eyre::eyre!("Invalid rewards coordinator address: {}", e)
                        })?;

                    cargo_tangle::command::eigenlayer::set_claimer(
                        &claimer_address,
                        rewards_coord_addr,
                        &keystore_uri,
                        &rpc_url,
                    )
                    .await?;
                }
            },
        },
        Commands::Key { command } => match command {
            KeyCommands::Generate {
                key_type,
                output,
                seed,
                show_secret,
            } => {
                let seed = seed.map(hex::decode).transpose()?;
                let (public, secret) =
                    generate_key(key_type, output.as_ref(), seed.as_deref(), show_secret)?;

                eprintln!("Generated {:?} key:", key_type);
                eprintln!("Public key: {}", public);
                if show_secret || output.is_none() {
                    eprintln!(
                        "Private key: {}",
                        secret.expect("Failed to find secret key")
                    );
                }
            }
            KeyCommands::Import {
                key_type,
                secret,
                keystore_path,
                protocol,
            } => {
                if let Some(key_type) = key_type {
                    // If key_type is provided, require secret
                    let secret = secret.ok_or_else(|| {
                        color_eyre::eyre::eyre!("Secret key is required when key type is specified")
                    })?;
                    let public = import_key(protocol, key_type, &secret, &keystore_path)?;
                    eprintln!("Imported {:?} key:", key_type);
                    eprintln!("Public key: {}", public);
                } else {
                    // If no key_type provided, use interactive prompt
                    let key_pairs = prompt_for_keys(vec![])?;
                    for (key_type, secret) in key_pairs {
                        let public = import_key(protocol, key_type, &secret, &keystore_path)?;
                        eprintln!("Imported {:?} key:", key_type);
                        eprintln!("Public key: {}", public);
                    }
                }
            }
            KeyCommands::Export {
                key_type,
                public,
                keystore_path,
            } => {
                let secret = export_key(key_type, &public, &keystore_path)?;
                eprintln!("Exported {:?} key:", key_type);
                eprintln!("Public key: {}", public);
                eprintln!("Private key: {}", secret);
            }
            KeyCommands::List { keystore_path } => {
                let keys = list_keys(&keystore_path)?;
                eprintln!("Keys in keystore:");
                for (key_type, public) in keys {
                    eprintln!("{:?}: {}", key_type, public);
                }
            }
            KeyCommands::GenerateMnemonic { word_count } => {
                let mnemonic = generate_mnemonic(word_count)?;
                eprintln!("Generated mnemonic phrase:");
                eprintln!("{}", mnemonic);
                eprintln!(
                    "\nWARNING: Store this mnemonic phrase securely. It can be used to recover your keys."
                );
            }
        },
        Commands::Debug { command } => match command {
            DebugCommands::Spawn {
                mut http_rpc_url,
                mut ws_rpc_url,
                package,
                id,
                service_name,
                binary,
                image,
                protocol,
                method,
                #[cfg(feature = "vm-debug")]
                verify_network_connection,
            } => {
                match (&mut http_rpc_url, &mut ws_rpc_url) {
                    (Some(http), None) => match http.scheme() {
                        "http" => {
                            let mut ws = http.clone();
                            ws.set_scheme("ws").unwrap();
                            ws_rpc_url = Some(ws);
                        }
                        "https" => {
                            let mut ws = http.clone();
                            ws.set_scheme("wss").unwrap();
                            ws_rpc_url = Some(ws);
                        }
                        _ => panic!("Unknown URL scheme"),
                    },
                    (None, Some(ws)) => match ws.scheme() {
                        "ws" => {
                            let mut http = ws.clone();
                            http.set_scheme("http").unwrap();
                            http_rpc_url = Some(http);
                        }
                        "wss" => {
                            let mut http = ws.clone();
                            http.set_scheme("https").unwrap();
                            http_rpc_url = Some(http);
                        }
                        _ => panic!("Unknown URL scheme"),
                    },
                    (Some(_), Some(_)) | (None, None) => {}
                }

                let manifest_path = cli
                    .manifest
                    .manifest_path
                    .unwrap_or_else(|| PathBuf::from("Cargo.toml"));

                Box::pin(debug::spawn::execute(
                    http_rpc_url,
                    ws_rpc_url,
                    manifest_path,
                    package,
                    id,
                    service_name,
                    binary,
                    image,
                    protocol,
                    method,
                    #[cfg(feature = "vm-debug")]
                    verify_network_connection,
                ))
                .await?;
            }
        },
    }
    Ok(())
}

fn load_protocol_settings(
    protocol: Protocol,
    settings_file: &PathBuf,
) -> Result<ProtocolSettings, ConfigError> {
    // Load environment variables from the settings file
    from_path(settings_file)
        .map_err(|e| ConfigError::Other(format!("Failed to load settings file: {}", e).into()))?;

    match protocol {
        Protocol::Eigenlayer => {
            let addresses = EigenlayerProtocolSettings {
                allocation_manager_address: env::var("ALLOCATION_MANAGER_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid ALLOCATION_MANAGER_ADDRESS".into()))?,
                registry_coordinator_address: env::var("REGISTRY_COORDINATOR_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| {
                        ConfigError::Other("Invalid REGISTRY_COORDINATOR_ADDRESS".into())
                    })?,
                operator_state_retriever_address: env::var("OPERATOR_STATE_RETRIEVER_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| {
                        ConfigError::Other("Invalid OPERATOR_STATE_RETRIEVER_ADDRESS".into())
                    })?,
                delegation_manager_address: env::var("DELEGATION_MANAGER_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid DELEGATION_MANAGER_ADDRESS".into()))?,
                service_manager_address: env::var("SERVICE_MANAGER_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid SERVICE_MANAGER_ADDRESS".into()))?,
                stake_registry_address: env::var("STAKE_REGISTRY_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid STAKE_REGISTRY_ADDRESS".into()))?,
                strategy_manager_address: env::var("STRATEGY_MANAGER_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid STRATEGY_MANAGER_ADDRESS".into()))?,
                strategy_address: env::var("STRATEGY_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid STRATEGY_ADDRESS".into()))?,
                avs_directory_address: env::var("AVS_DIRECTORY_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid AVS_DIRECTORY_ADDRESS".into()))?,
                rewards_coordinator_address: env::var("REWARDS_COORDINATOR_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| {
                        ConfigError::Other("Invalid REWARDS_COORDINATOR_ADDRESS".into())
                    })?,
                permission_controller_address: env::var("PERMISSION_CONTROLLER_ADDRESS")
                    .map_err(|_| ConfigError::MissingEigenlayerContractAddresses)?
                    .parse()
                    .map_err(|_| {
                        ConfigError::Other("Invalid PERMISSION_CONTROLLER_ADDRESS".into())
                    })?,
                // Registration parameters (use defaults if not specified)
                allocation_delay: env::var("ALLOCATION_DELAY")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0),
                deposit_amount: env::var("DEPOSIT_AMOUNT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5_000_000_000_000_000_000_000),
                stake_amount: env::var("STAKE_AMOUNT")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1_000_000_000_000_000_000),
                operator_sets: env::var("OPERATOR_SETS")
                    .ok()
                    .and_then(|s| s.split(',').map(|v| v.parse().ok()).collect())
                    .unwrap_or_else(|| vec![0]),
                staker_opt_out_window_blocks: env::var("STAKER_OPT_OUT_WINDOW_BLOCKS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(50400),
                metadata_url: env::var("METADATA_URL")
                    .unwrap_or_else(|_| "https://github.com/tangle-network/blueprint".to_string()),
            };
            Ok(ProtocolSettings::Eigenlayer(addresses))
        }
        Protocol::Tangle => {
            let settings = TangleProtocolSettings {
                blueprint_id: env::var("BLUEPRINT_ID")
                    .map_err(|_| ConfigError::Other("Missing BLUEPRINT_ID".into()))?
                    .parse()
                    .map_err(|_| ConfigError::Other("Invalid BLUEPRINT_ID".into()))?,
                service_id: env::var("SERVICE_ID")
                    .ok()
                    .map(|id| {
                        id.parse()
                            .map_err(|_| ConfigError::Other("Invalid SERVICE_ID".into()))
                    })
                    .transpose()?,
            };
            Ok(ProtocolSettings::Tangle(settings))
        }
        _ => Err(ConfigError::UnsupportedProtocol(protocol.to_string())),
    }
}

fn init_tracing_subscriber() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::prelude::*;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .pretty();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .init();
}

#[must_use]
pub fn get_security_commitment(a: Asset<AssetId>, p: u8) -> AssetSecurityCommitment<AssetId> {
    AssetSecurityCommitment {
        asset: a,
        exposure_percent: Percent(p),
    }
}

/// Waits for a transaction to be included in a block and returns the success event.
///
/// # Arguments
///
/// * `res` - A `TxProgress` object representing the progress of a transaction.
///
/// # Returns
///
/// A `Result` containing the success event or an error.
///
/// # Panics
///
/// Panics if the transaction fails to be included in a block.
pub async fn wait_for_in_block_success<T: Config, C: OnlineClientT<T>>(
    mut res: TxProgress<T, C>,
) -> ExtrinsicEvents<T> {
    let mut val = Err("Failed to get in block success".into());
    while let Some(Ok(event)) = res.next().await {
        let Some(block) = event.as_in_block() else {
            continue;
        };
        val = block.wait_for_success().await;
    }

    val.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
