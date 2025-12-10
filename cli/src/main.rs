use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use alloy_primitives::{Address, Bytes, U256};
use blueprint_client_tangle_evm::{TangleEvmClient, TransactionResult, contracts::ITangleTypes};
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_manager::config::SourceType;
use blueprint_runner::config::{BlueprintEnvironment, Protocol, SupportedChains};
use blueprint_runner::error::ConfigError;
use cargo_tangle::command::create::{BlueprintType, new_blueprint};
use cargo_tangle::command::debug::{self, DebugCommands};
use cargo_tangle::command::deploy::eigenlayer::deploy_eigenlayer;
use cargo_tangle::command::deploy::tangle as deploy_tangle;
use cargo_tangle::command::jobs::{
    check::wait_for_job_result,
    helpers::{
        JobSchema, list_jobs, load_job_call_details, load_job_schema, print_job_call_details,
        print_job_summaries,
    },
    submit::submit_job as submit_job_call,
};
use cargo_tangle::command::keys::{
    SupportedKey, export_key, generate_key, generate_mnemonic, import_key, list_keys,
    prompt_for_keys,
};
use cargo_tangle::command::list;
use cargo_tangle::command::operator;
use cargo_tangle::command::run::run_eigenlayer_avs;
use cargo_tangle::command::run::tangle::{RunOpts, run_blueprint};
use cargo_tangle::command::service::{
    approve_service, approve_service_with_commitments, build_request_params, join_service,
    leave_service, reject_service, request_service, with_security_requirements,
};
use cargo_tangle::command::signer::load_evm_signer;
use cargo_tangle::command::tangle::{
    PreferredSourceArg, SpawnMethod, TangleClientArgs, parse_address,
};
use cargo_tangle::settings::{
    RuntimePreferences, load_protocol_settings, load_runtime_preferences, write_runtime_preferences,
};
use cargo_tangle::utils::find_registration_inputs;
use clap::{Parser, Subcommand};
use color_eyre::eyre::{Context, Result, ensure, eyre};
use serde_json::json;
use url::Url;

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

    /// Cloud deployment
    #[cfg(feature = "remote-providers")]
    #[command(visible_alias = "c")]
    Cloud {
        #[command(subcommand)]
        command: cargo_tangle::command::cloud::CloudCommands,
    },

    /// Key management
    #[command(visible_alias = "k")]
    Key {
        #[command(subcommand)]
        command: KeyCommands,
    },

    /// Operator utilities.
    #[command(visible_alias = "op")]
    Operator {
        #[command(subcommand)]
        command: OperatorCommands,
    },
}

#[derive(Subcommand, Debug)]
enum BlueprintCommands {
    /// Create a new blueprint
    #[command(visible_alias = "c")]
    Create {
        #[arg(short = 'n', long, value_name = "NAME", env = "NAME")]
        name: String,
        #[command(flatten)]
        source: Option<cargo_tangle::command::create::Source>,
        #[command(flatten)]
        blueprint_type: Option<BlueprintType>,
        #[arg(
            long,
            short = 'd',
            number_of_values = 1,
            conflicts_with = "template_values_file"
        )]
        define: Vec<String>,
        #[arg(long, value_name = "FILE", conflicts_with = "define")]
        template_values_file: Option<String>,
        #[arg(long)]
        skip_prompts: bool,
    },

    /// Deploy a blueprint
    #[command(visible_alias = "d")]
    Deploy {
        #[command(subcommand)]
        target: DeployTarget,
    },

    /// Run a blueprint
    #[command(visible_alias = "r")]
    Run {
        #[arg(short = 'p', long, value_enum, default_value = "tangle-evm")]
        protocol: Protocol,
        #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:8545")]
        http_rpc_url: Url,
        #[arg(long, value_name = "URL", default_value = "ws://127.0.0.1:8546")]
        ws_rpc_url: Url,
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
        #[arg(short = 'w', long, default_value = "local")]
        network: String,
        #[arg(short = 'd', long)]
        data_dir: Option<PathBuf>,
        #[arg(short = 'n', long)]
        bootnodes: Option<Vec<String>>,
        #[arg(short = 'f', long, default_value = "./settings.env")]
        settings_file: Option<PathBuf>,
        #[arg(long, env)]
        allow_unchecked_attestations: bool,
        /// Preferred runtime for the service.
        #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
        spawn_method: SpawnMethod,
        /// Override the manager's preferred blueprint source.
        #[arg(long, value_enum)]
        preferred_source: Option<PreferredSourceArg>,
        /// Force the manager to execute inside a VM sandbox.
        #[arg(long)]
        vm: bool,
        /// Disable the VM sandbox for the manager.
        #[arg(long)]
        no_vm: bool,
        /// Persist runtime overrides back to the settings file.
        #[arg(long)]
        save_runtime_prefs: bool,
    },

    /// Generate registration inputs for a blueprint without registering on-chain
    #[command(visible_alias = "pre")]
    Preregister {
        #[arg(short = 'p', long, value_enum, default_value = "tangle-evm")]
        protocol: Protocol,
        #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:8545")]
        http_rpc_url: Url,
        #[arg(long, value_name = "URL", default_value = "ws://127.0.0.1:8546")]
        ws_rpc_url: Url,
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
        #[arg(short = 'n', long, default_value = "local")]
        network: String,
        #[arg(short = 'd', long)]
        data_dir: Option<PathBuf>,
        #[arg(short = 'f', long, default_value = "./settings.env")]
        settings_file: Option<PathBuf>,
        /// Preferred runtime for the service.
        #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
        spawn_method: SpawnMethod,
        /// Override the manager's preferred blueprint source.
        #[arg(long, value_enum)]
        preferred_source: Option<PreferredSourceArg>,
        /// Force the manager to execute inside a VM sandbox.
        #[arg(long)]
        vm: bool,
        /// Disable the VM sandbox for the manager.
        #[arg(long)]
        no_vm: bool,
        /// Persist runtime overrides back to the settings file.
        #[arg(long)]
        save_runtime_prefs: bool,
    },

    /// Register as a Tangle EVM operator
    #[command(visible_alias = "reg")]
    Register {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long, value_name = "URL")]
        rpc_endpoint: Option<String>,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long, value_name = "FILE")]
        registration_inputs: Option<PathBuf>,
    },
    /// Listing helpers for blueprints/services.
    #[command(visible_alias = "ls")]
    List {
        #[command(subcommand)]
        command: ListCommands,
    },
    /// Spawn local harnesses and debugging utilities.
    #[command(visible_alias = "dbg")]
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },

    /// Jobs helpers (submit, watch, inspect).
    #[command(visible_alias = "j")]
    Jobs {
        #[command(subcommand)]
        command: JobsCommands,
    },

    /// Service lifecycle helpers.
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },
}

#[derive(Subcommand, Debug)]
enum KeyCommands {
    /// Generate a new key
    #[command(visible_alias = "g")]
    Generate {
        #[arg(short = 't', long, value_enum)]
        key_type: SupportedKey,
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
        seed: Option<Vec<u8>>,
        #[arg(short = 'v', long)]
        show_secret: bool,
    },
    /// Import a key into the keystore
    #[command(visible_alias = "i")]
    Import {
        #[arg(short = 't', long, value_enum)]
        key_type: Option<SupportedKey>,
        #[arg(short = 'x', long)]
        secret: Option<String>,
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
        #[arg(short = 'p', long, value_enum, default_value = "tangle-evm")]
        protocol: Protocol,
    },
    /// Export a key from the keystore
    #[command(visible_alias = "e")]
    Export {
        #[arg(short = 't', long, value_enum)]
        key_type: SupportedKey,
        #[arg(short = 'p', long)]
        public: String,
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
    },
    /// List all keys in the keystore
    #[command(visible_alias = "l")]
    List {
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
    },
    /// Generate a new mnemonic
    #[command(visible_alias = "m")]
    GenerateMnemonic {
        #[arg(short = 'w', long, value_parser = clap::value_parser!(u32).range(12..=24))]
        word_count: Option<u32>,
    },
}

#[derive(Subcommand, Debug)]
enum ListCommands {
    /// List all registered blueprints.
    Blueprints {
        #[command(flatten)]
        network: TangleClientArgs,
    },
    /// List all pending service requests.
    Requests {
        #[command(flatten)]
        network: TangleClientArgs,
    },
    /// List all services.
    Services {
        #[command(flatten)]
        network: TangleClientArgs,
    },
}

#[derive(Subcommand, Debug)]
enum ServiceCommands {
    /// Request a service from the network.
    Request {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Target operator addresses.
        #[arg(long = "operator", required = true)]
        operators: Vec<String>,
        /// Optional operator exposures (basis points, matches --operator ordering).
        #[arg(long = "operator-exposure-bps")]
        operator_exposures: Vec<u16>,
        /// Additional permitted callers.
        #[arg(long = "permitted-caller")]
        permitted_callers: Vec<String>,
        /// Optional config file (raw bytes).
        #[arg(long = "config-file", value_name = "PATH")]
        config_file: Option<PathBuf>,
        /// Optional hex-encoded config payload.
        #[arg(long = "config-hex", value_name = "HEX")]
        config_hex: Option<String>,
        /// Service TTL in seconds (0 = never expires).
        #[arg(long, default_value_t = 600)]
        ttl: u64,
        /// Payment token address.
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        payment_token: String,
        /// Payment amount (wei).
        #[arg(long, default_value_t = 0)]
        payment_amount: u128,
        /// Optional security requirement (format KIND:TOKEN:MIN:MAX, repeated).
        #[arg(
            long = "security-requirement",
            value_name = "KIND:TOKEN:MIN:MAX",
            value_parser = parse_security_requirement
        )]
        security_requirements: Vec<SecurityRequirementArg>,
        /// Emit transaction logs as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Approve a pending service request.
    Approve {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Request identifier.
        #[arg(long)]
        request_id: u64,
        /// Restaking percentage to commit.
        #[arg(long, default_value_t = 50)]
        restaking_percent: u8,
        /// Explicit security commitments (format KIND:TOKEN:EXPOSURE, repeated).
        #[arg(
            long = "security-commitment",
            value_name = "KIND:TOKEN:EXPOSURE",
            value_parser = parse_security_commitment
        )]
        security_commitments: Vec<SecurityCommitmentArg>,
        /// Emit transaction logs as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Reject a pending service request.
    Reject {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Request identifier.
        #[arg(long)]
        request_id: u64,
        /// Emit transaction logs as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Join a dynamic service as an operator.
    Join {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Exposure (basis points) to request.
        #[arg(long, default_value_t = MAX_BPS)]
        exposure_bps: u16,
        /// Emit transaction logs as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Leave a dynamic service via the legacy helper.
    Leave {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Emit transaction logs as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Spawn a service runtime for the configured blueprint.
    Spawn {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Preferred runtime.
        #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
        spawn_method: SpawnMethod,
        /// Directory for blueprint data.
        #[arg(long, value_name = "PATH")]
        data_dir: Option<PathBuf>,
        /// Allow unchecked attestations when spinning up the manager.
        #[arg(long)]
        allow_unchecked_attestations: bool,
        /// Override the manager's preferred blueprint source.
        #[arg(long, value_enum)]
        preferred_source: Option<PreferredSourceArg>,
        /// Force the manager to execute inside a VM sandbox.
        #[arg(long)]
        vm: bool,
        /// Disable the VM sandbox for the manager.
        #[arg(long)]
        no_vm: bool,
    },
    /// List all services.
    List {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Emit JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
    /// List all service requests.
    Requests {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Emit JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },
    /// Show a specific service request.
    Show {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Request identifier.
        #[arg(long)]
        request_id: u64,
    },
}

#[derive(Subcommand, Debug)]
enum OperatorCommands {
    /// Show operator heartbeat/status details.
    Status {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Operator address (defaults to the local operator).
        #[arg(long = "operator")]
        operator: Option<String>,
        /// Emit JSON instead of human-readable output.
        #[arg(long)]
        json: bool,
    },
    /// Submit a heartbeat to the OperatorStatusRegistry contract.
    #[command(visible_alias = "hb")]
    Heartbeat {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Status code to report (0 = healthy).
        #[arg(long, default_value_t = 0)]
        status_code: u8,
        /// Emit JSON instead of human-readable output.
        #[arg(long)]
        json: bool,
    },
    /// Join a dynamic service as an operator.
    Join {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Requested exposure in basis points.
        #[arg(long, default_value_t = 10_000)]
        exposure_bps: u16,
        /// Emit JSON transaction logs.
        #[arg(long)]
        json: bool,
    },
    /// Leave a dynamic service.
    Leave {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Emit JSON transaction logs.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
enum DeployTarget {
    /// Deploy to Eigenlayer
    Eigenlayer {
        #[arg(long, value_name = "URL", env, required_unless_present = "devnet")]
        rpc_url: Option<String>,
        #[arg(long)]
        contracts_path: Option<String>,
        #[arg(long)]
        ordered_deployment: bool,
        #[arg(short = 'w', long, default_value = "local")]
        network: String,
        #[arg(long)]
        devnet: bool,
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
    },
    /// Deploy to a Tangle EVM environment
    Tangle(cargo_tangle::command::deploy::tangle::TangleDeployArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_tracing_subscriber();
    cargo_tangle::install_crypto_provider();

    let args = collect_args();
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
                DeployTarget::Tangle(args) => {
                    deploy_tangle::execute(args).await?;
                }
            },
            BlueprintCommands::Run {
                protocol,
                http_rpc_url,
                ws_rpc_url,
                keystore_path,
                network,
                data_dir,
                bootnodes,
                settings_file,
                allow_unchecked_attestations,
                spawn_method,
                preferred_source,
                vm,
                no_vm,
                save_runtime_prefs,
            } => {
                let settings_file =
                    settings_file.unwrap_or_else(|| PathBuf::from("./settings.env"));
                let protocol_settings = load_protocol_settings(protocol, &settings_file)?;
                let runtime_prefs = load_runtime_preferences();
                let (preferred_source_value, use_vm_value) = resolve_runtime_options(
                    spawn_method,
                    runtime_prefs,
                    preferred_source,
                    vm,
                    no_vm,
                )?;

                if save_runtime_prefs {
                    write_runtime_preferences(
                        &settings_file,
                        RuntimePreferences {
                            preferred_source: Some(preferred_source_value),
                            use_vm: Some(use_vm_value),
                        },
                    )?;
                }

                let keystore_path = keystore_path.unwrap_or_else(|| PathBuf::from("./keystore"));
                ensure_keys(&keystore_path, &[SupportedKey::Ecdsa])?;

                let mut config = BlueprintEnvironment::default();
                config.http_rpc_endpoint = http_rpc_url.clone();
                config.ws_rpc_endpoint = ws_rpc_url.clone();
                config.keystore_uri = keystore_path.to_string_lossy().to_string();
                config.bootnodes = bootnodes
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|addr| addr.parse().ok())
                    .collect();
                config.protocol_settings = protocol_settings.clone();
                config.test_mode = network == "local";

                let chain = parse_supported_chain(&network, &http_rpc_url)?;

                match protocol {
                    Protocol::Eigenlayer => {
                        run_eigenlayer_avs(
                            config,
                            chain,
                            None,
                            data_dir,
                            allow_unchecked_attestations,
                        )
                        .await?;
                    }
                    Protocol::TangleEvm => {
                        let settings = protocol_settings.tangle_evm().map_err(|e| eyre!("{e}"))?;

                        let run_opts = RunOpts {
                            http_rpc_url,
                            ws_rpc_url,
                            blueprint_id: settings.blueprint_id,
                            service_id: settings.service_id,
                            tangle_contract: settings.tangle_contract,
                            restaking_contract: settings.restaking_contract,
                            status_registry_contract: settings.status_registry_contract,
                            keystore_path: config.keystore_uri.clone(),
                            data_dir,
                            allow_unchecked_attestations,
                            registration_mode: false,
                            registration_capture_only: false,
                            preferred_source: preferred_source_value,
                            use_vm: use_vm_value,
                        };
                        run_blueprint(run_opts).await?;
                    }
                    _ => return Err(ConfigError::UnsupportedProtocol(protocol.to_string()).into()),
                }
            }
            BlueprintCommands::Register {
                network,
                rpc_endpoint,
                blueprint_id,
                registration_inputs,
            } => {
                register_operator(network, rpc_endpoint, blueprint_id, registration_inputs).await?;
            }
            BlueprintCommands::List { command } => match command {
                ListCommands::Blueprints { network } => {
                    let client = network.connect(0, None).await?;
                    let blueprints = list::blueprints::list_blueprints(&client).await?;
                    list::blueprints::print_blueprints(&blueprints);
                }
                ListCommands::Requests { network } => {
                    let client = network.connect(0, None).await?;
                    let requests = list::requests::list_requests(&client).await?;
                    list::requests::print_requests(&requests, false);
                }
                ListCommands::Services { network } => {
                    let client = network.connect(0, None).await?;
                    let services = list::services::list_services(&client).await?;
                    list::services::print_services(&services, false);
                }
            },
            BlueprintCommands::Debug { command } => match command {
                DebugCommands::Spawn(args) => {
                    debug::spawn::execute(args).await?;
                }
            },
            BlueprintCommands::Preregister {
                protocol,
                http_rpc_url,
                ws_rpc_url,
                keystore_path,
                network: _,
                data_dir,
                settings_file,
                spawn_method,
                preferred_source,
                vm,
                no_vm,
                save_runtime_prefs,
            } => {
                if protocol != Protocol::TangleEvm {
                    return Err(eyre!(
                        "Preregistration is only supported for the Tangle EVM protocol"
                    ));
                }

                let settings_file =
                    settings_file.unwrap_or_else(|| PathBuf::from("./settings.env"));
                let protocol_settings = load_protocol_settings(protocol, &settings_file)?;
                let settings = protocol_settings.tangle_evm().map_err(|e| eyre!("{e}"))?;

                let keystore_path = keystore_path.unwrap_or_else(|| PathBuf::from("./keystore"));
                ensure_keys(&keystore_path, &[SupportedKey::Ecdsa])?;

                let prereg_data_dir = data_dir.clone();
                let base_data_dir = prereg_data_dir
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("./data"));

                let runtime_prefs = load_runtime_preferences();
                let (preferred_source_value, use_vm_value) = resolve_runtime_options(
                    spawn_method,
                    runtime_prefs,
                    preferred_source,
                    vm,
                    no_vm,
                )?;

                if save_runtime_prefs {
                    write_runtime_preferences(
                        &settings_file,
                        RuntimePreferences {
                            preferred_source: Some(preferred_source_value),
                            use_vm: Some(use_vm_value),
                        },
                    )?;
                }

                let run_opts = RunOpts {
                    http_rpc_url,
                    ws_rpc_url,
                    blueprint_id: settings.blueprint_id,
                    service_id: None,
                    tangle_contract: settings.tangle_contract,
                    restaking_contract: settings.restaking_contract,
                    status_registry_contract: settings.status_registry_contract,
                    keystore_path: keystore_path.to_string_lossy().to_string(),
                    data_dir: prereg_data_dir,
                    allow_unchecked_attestations: false,
                    registration_mode: true,
                    registration_capture_only: true,
                    preferred_source: preferred_source_value,
                    use_vm: use_vm_value,
                };

                run_blueprint(run_opts).await?;

                let payload_path = find_registration_inputs(&base_data_dir, settings.blueprint_id)
                    .ok_or_else(|| {
                        eyre!(
                            "Registration payload not found under {}",
                            base_data_dir.display()
                        )
                    })?;

                println!(
                    "Registration payload for blueprint {} saved to {}",
                    settings.blueprint_id,
                    payload_path.display()
                );
            }
            BlueprintCommands::Jobs { command } => match command {
                JobsCommands::List {
                    network,
                    blueprint_id,
                    json,
                } => {
                    let client = network.connect(blueprint_id, None).await?;
                    let jobs = list_jobs(&client, blueprint_id).await?;
                    print_job_summaries(&jobs, json);
                }
                JobsCommands::Show {
                    network,
                    blueprint_id,
                    service_id,
                    call_id,
                    json,
                } => {
                    let client = network.connect(blueprint_id, Some(service_id)).await?;
                    let details =
                        load_job_call_details(&client, blueprint_id, service_id, call_id).await?;
                    print_job_call_details(&details, json);
                }
                JobsCommands::Submit {
                    network,
                    blueprint_id,
                    service_id,
                    job,
                    payload_hex,
                    payload_file,
                    params_file,
                    prompt,
                    watch,
                    timeout_secs,
                    json,
                } => {
                    let client = network.connect(blueprint_id, Some(service_id)).await?;
                    let mut schema_cache: Option<JobSchema> = None;

                    let payload = match (payload_hex, payload_file, params_file, prompt) {
                        (Some(hex_value), None, None, false) => decode_payload_hex(&hex_value)?,
                        (None, Some(path), None, false) => read_payload_file(&path)?,
                        (None, None, Some(path), false) => {
                            ensure_schema_loaded(&mut schema_cache, &client, blueprint_id, job)
                                .await?;
                            schema_cache
                                .as_ref()
                                .expect("schema should be loaded")
                                .encode_params_from_file(&path)?
                        }
                        (None, None, None, true) => {
                            ensure_schema_loaded(&mut schema_cache, &client, blueprint_id, job)
                                .await?;
                            schema_cache
                                .as_ref()
                                .expect("schema should be loaded")
                                .prompt_for_params()?
                        }
                        (Some(_), Some(_), _, _) => {
                            return Err(eyre!(
                                "Specify only one of --payload-hex, --payload-file, --params-file, or --prompt"
                            ));
                        }
                        _ => {
                            return Err(eyre!(
                                "Provide job inputs via --payload-hex, --payload-file, --params-file, or --prompt"
                            ));
                        }
                    };

                    let submission =
                        submit_job_call(&client, service_id, job, payload.clone()).await?;
                    log_tx("Job submission", &submission.tx, json);
                    if json {
                        println!(
                            "{}",
                            json!({
                                "event": "job_submitted",
                                "service_id": service_id,
                                "blueprint_id": blueprint_id,
                                "job": job,
                                "call_id": submission.call_id,
                                "tx_hash": format!("{:#x}", submission.tx.tx_hash),
                            })
                        );
                    } else {
                        println!(
                            "Submitted job {job} to service {service_id}. Call ID: {} (tx: {:#x})",
                            submission.call_id, submission.tx.tx_hash
                        );
                    }
                    if watch {
                        let bytes = wait_for_job_result(
                            &client,
                            service_id,
                            submission.call_id,
                            Duration::from_secs(timeout_secs),
                        )
                        .await?;
                        match ensure_schema_loaded(&mut schema_cache, &client, blueprint_id, job)
                            .await
                        {
                            Ok(()) => {
                                let schema = schema_cache.as_ref().expect("schema present");
                                match schema.decode_and_format_results(&bytes) {
                                    Ok(Some(lines)) => {
                                        if json {
                                            println!(
                                                "{}",
                                                json!({
                                                    "event": "job_result",
                                                    "service_id": service_id,
                                                    "call_id": submission.call_id,
                                                    "decoded": lines,
                                                    "length": bytes.len(),
                                                })
                                            );
                                        } else {
                                            println!(
                                                "Job result ready ({} bytes). Decoded output:",
                                                bytes.len()
                                            );
                                            for line in lines {
                                                println!("  {line}");
                                            }
                                        }
                                    }
                                    Ok(None) => print_raw_job_result(&bytes),
                                    Err(err) => {
                                        eprintln!("Failed to decode job result: {err}");
                                        print_raw_job_result(&bytes);
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("Unable to load job schema: {err}");
                                print_raw_job_result(&bytes);
                            }
                        }
                    }
                }
                JobsCommands::Watch {
                    network,
                    blueprint_id,
                    service_id,
                    call_id,
                    timeout_secs,
                } => {
                    let client = network.connect(blueprint_id, Some(service_id)).await?;
                    let bytes = wait_for_job_result(
                        &client,
                        service_id,
                        call_id,
                        Duration::from_secs(timeout_secs),
                    )
                    .await?;
                    print_raw_job_result(&bytes);
                }
            },
            BlueprintCommands::Service { command } => match command {
                ServiceCommands::Request {
                    network,
                    blueprint_id,
                    operators,
                    operator_exposures,
                    permitted_callers,
                    config_file,
                    config_hex,
                    ttl,
                    payment_token,
                    payment_amount,
                    security_requirements,
                    json,
                } => {
                    let operators = parse_address_list(&operators, "operator")?;
                    let operator_exposures =
                        normalize_operator_exposures(&operator_exposures, operators.len())?;
                    let permitted_callers =
                        parse_address_list(&permitted_callers, "permitted caller")?;
                    let config = load_config_payload(config_file, config_hex)?;
                    let payment_token = parse_address(&payment_token, "PAYMENT_TOKEN")?;
                    let security_requirements: Vec<ITangleTypes::AssetSecurityRequirement> =
                        security_requirements
                            .into_iter()
                            .map(requirement_to_abi)
                            .collect();
                    let client = network.connect(blueprint_id, None).await?;
                    let params = with_security_requirements(
                        build_request_params(
                            blueprint_id,
                            operators,
                            operator_exposures,
                            permitted_callers,
                            ttl,
                            payment_token,
                            U256::from(payment_amount),
                            config,
                        ),
                        security_requirements,
                    );
                    let (tx, request_id) = request_service(&client, params).await?;

                    log_tx("Service request", &tx, json);
                    if json {
                        println!(
                            "{}",
                            json!({
                                "event": "service_request_id",
                                "request_id": request_id,
                                "tx_hash": format!("{:#x}", tx.tx_hash),
                            })
                        );
                    } else {
                        println!("Request ID: {request_id}");
                    }
                }
                ServiceCommands::Approve {
                    network,
                    request_id,
                    restaking_percent,
                    json,
                    security_commitments,
                } => {
                    let client = network.connect(0, None).await?;
                    let tx = if security_commitments.is_empty() {
                        approve_service(&client, request_id, restaking_percent).await?
                    } else {
                        let commitments: Vec<ITangleTypes::AssetSecurityCommitment> =
                            security_commitments
                                .into_iter()
                                .map(commitment_to_abi)
                                .collect();
                        approve_service_with_commitments(&client, request_id, commitments).await?
                    };
                    log_tx("Service approval", &tx, json);
                }
                ServiceCommands::Reject {
                    network,
                    request_id,
                    json,
                } => {
                    let client = network.connect(0, None).await?;
                    let tx = reject_service(&client, request_id).await?;
                    log_tx("Service rejection", &tx, json);
                }
                ServiceCommands::Join {
                    network,
                    service_id,
                    exposure_bps,
                    json,
                } => {
                    ensure!(exposure_bps > 0, "Exposure must be greater than 0 bps");
                    ensure!(
                        exposure_bps <= MAX_BPS,
                        "Exposure cannot exceed {MAX_BPS} bps"
                    );
                    let client = network.connect(0, Some(service_id)).await?;
                    let tx = join_service(&client, service_id, exposure_bps).await?;
                    log_tx("Service join", &tx, json);
                    if json {
                        println!(
                            "{}",
                            json!({
                                "event": "service_joined",
                                "service_id": service_id,
                                "exposure_bps": exposure_bps,
                                "tx_hash": format!("{:#x}", tx.tx_hash),
                            })
                        );
                    } else {
                        println!("Joined service {service_id} with exposure {exposure_bps} bps");
                    }
                }
                ServiceCommands::Leave {
                    network,
                    service_id,
                    json,
                } => {
                    let client = network.connect(0, Some(service_id)).await?;
                    let operator = client.account();
                    let operator_info = client
                        .get_service_operator(service_id, operator)
                        .await
                        .map_err(|e| eyre!(e.to_string()))?;
                    ensure!(
                        operator_info.active,
                        "Operator is not active in service {service_id}"
                    );

                    let tx = leave_service(&client, service_id).await?;
                    log_tx("Service leave", &tx, json);
                    if json {
                        println!(
                            "{}",
                            json!({
                                "event": "service_left",
                                "service_id": service_id,
                                "tx_hash": format!("{:#x}", tx.tx_hash),
                            })
                        );
                    } else {
                        println!("Left service {service_id}");
                    }
                }
                ServiceCommands::Spawn {
                    network,
                    blueprint_id,
                    service_id,
                    spawn_method,
                    data_dir,
                    allow_unchecked_attestations,
                    preferred_source,
                    vm,
                    no_vm,
                } => {
                    let client_config = network.client_config(blueprint_id, Some(service_id))?;
                    let settings = client_config.settings.clone();
                    let (preferred_source_value, use_vm_value) = resolve_runtime_options(
                        spawn_method,
                        RuntimePreferences::default(),
                        preferred_source,
                        vm,
                        no_vm,
                    )?;
                    let run_opts = RunOpts {
                        http_rpc_url: client_config.http_rpc_endpoint.clone(),
                        ws_rpc_url: client_config.ws_rpc_endpoint.clone(),
                        blueprint_id,
                        service_id: Some(service_id),
                        tangle_contract: settings.tangle_contract,
                        restaking_contract: settings.restaking_contract,
                        status_registry_contract: settings.status_registry_contract,
                        keystore_path: network.keystore_path().display().to_string(),
                        data_dir,
                        allow_unchecked_attestations,
                        registration_mode: false,
                        registration_capture_only: false,
                        preferred_source: preferred_source_value,
                        use_vm: use_vm_value,
                    };
                    run_blueprint(run_opts).await?;
                }
                ServiceCommands::List { network, json } => {
                    let client = network.connect(0, None).await?;
                    let services = list::services::list_services(&client).await?;
                    list::services::print_services(&services, json);
                }
                ServiceCommands::Requests { network, json } => {
                    let client = network.connect(0, None).await?;
                    let requests = list::requests::list_requests(&client).await?;
                    list::requests::print_requests(&requests, json);
                }
                ServiceCommands::Show {
                    network,
                    request_id,
                } => {
                    let client = network.connect(0, None).await?;
                    let request = client
                        .get_service_request_info(request_id)
                        .await
                        .map_err(|e| eyre!(e.to_string()))?;
                    list::requests::print_request(&request);
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

                println!("Generated {key_type:?} key:");
                println!("Public key: {public}");
                if show_secret || output.is_none() {
                    println!("Private key: {}", secret.expect("Missing secret"));
                }
            }
            KeyCommands::Import {
                key_type,
                secret,
                keystore_path,
                protocol,
            } => {
                if let Some(kind) = key_type {
                    let secret = secret.ok_or_else(|| eyre!("Secret key is required"))?;
                    let public = import_key(protocol, kind, &secret, &keystore_path)?;
                    println!("Imported {kind:?} key:");
                    println!("Public key: {public}");
                } else {
                    let key_pairs = prompt_for_keys(vec![])?;
                    for (kind, secret) in key_pairs {
                        let public = import_key(protocol, kind, &secret, &keystore_path)?;
                        println!("Imported {kind:?} key:");
                        println!("Public key: {public}");
                    }
                }
            }
            KeyCommands::Export {
                key_type,
                public,
                keystore_path,
            } => {
                let secret = export_key(key_type, &public, &keystore_path)?;
                println!("Private key: {secret}");
            }
            KeyCommands::List { keystore_path } => {
                let keys = list_keys(&keystore_path)?;
                for (kind, public) in keys {
                    println!("{kind:?}: {public}");
                }
            }
            KeyCommands::GenerateMnemonic { word_count } => {
                let mnemonic = generate_mnemonic(word_count)?;
                println!("Mnemonic: {mnemonic}");
            }
        },
        #[cfg(feature = "remote-providers")]
        Commands::Cloud { command } => {
            cargo_tangle::command::cloud::execute(command).await?;
        }
        Commands::Operator { command } => match command {
            OperatorCommands::Status {
                network,
                blueprint_id,
                service_id,
                operator,
                json,
            } => {
                let client = network.connect(blueprint_id, Some(service_id)).await?;
                let operator_address = if let Some(value) = operator {
                    parse_address(&value, "OPERATOR")?
                } else {
                    client.account()
                };
                let status = client
                    .operator_status(service_id, operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                operator::print_status(&status, json);
            }
            OperatorCommands::Heartbeat {
                network,
                blueprint_id,
                service_id,
                status_code,
                json,
            } => {
                let config = network.client_config(blueprint_id, Some(service_id))?;
                let keystore = cargo_tangle::command::signer::load_keystore(network.keystore_path())?;
                let mut signing_key =
                    cargo_tangle::command::signer::load_ecdsa_signing_key(&keystore)?;
                operator::submit_heartbeat(
                    config.http_rpc_endpoint.as_str(),
                    config.settings.status_registry_contract,
                    &mut signing_key,
                    service_id,
                    blueprint_id,
                    status_code,
                    json,
                )
                .await?;
            }
            OperatorCommands::Join {
                network,
                blueprint_id,
                service_id,
                exposure_bps,
                json,
            } => {
                let client = network.connect(blueprint_id, Some(service_id)).await?;
                let tx = client
                    .join_service(service_id, exposure_bps)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator join", &tx, json);
            }
            OperatorCommands::Leave {
                network,
                blueprint_id,
                service_id,
                json,
            } => {
                let client = network.connect(blueprint_id, Some(service_id)).await?;
                let tx = client
                    .leave_service(service_id)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator leave", &tx, json);
            }
        },
    }

    Ok(())
}

fn parse_address_list(values: &[String], label: &str) -> Result<Vec<Address>> {
    values
        .iter()
        .map(|value| parse_address(value, label))
        .collect()
}

fn normalize_operator_exposures(
    exposures: &[u16],
    operator_len: usize,
) -> Result<Option<Vec<u16>>> {
    if exposures.is_empty() {
        return Ok(None);
    }
    ensure!(
        exposures.len() == operator_len,
        "Expected {operator_len} operator exposure values but received {}",
        exposures.len()
    );
    exposures
        .iter()
        .enumerate()
        .try_for_each(|(idx, value)| -> Result<()> {
            ensure!(
                *value <= MAX_BPS,
                "Operator exposure #{idx} exceeds {MAX_BPS} bps"
            );
            Ok(())
        })?;
    Ok(Some(exposures.to_vec()))
}

#[derive(Clone, Debug)]
struct SecurityRequirementArg {
    kind: AssetKindArg,
    token: Address,
    min: u16,
    max: u16,
}

#[derive(Clone, Debug)]
struct SecurityCommitmentArg {
    kind: AssetKindArg,
    token: Address,
    exposure: u16,
}

#[derive(Clone, Copy, Debug)]
enum AssetKindArg {
    Native,
    Erc20,
}

fn parse_security_requirement(value: &str) -> std::result::Result<SecurityRequirementArg, String> {
    let parts: Vec<_> = value.split(':').collect();
    if parts.len() != 4 {
        return Err("Expected format KIND:TOKEN:MIN:MAX".to_string());
    }
    let kind = parse_asset_kind(parts[0])?;
    let token = parse_token(kind, parts[1])?;
    let min = parse_bps(parts[2], "min exposure")?;
    let max = parse_bps(parts[3], "max exposure")?;
    if min == 0 {
        return Err("minimum exposure must be greater than 0".to_string());
    }
    if min > max {
        return Err("minimum exposure cannot exceed maximum exposure".to_string());
    }
    Ok(SecurityRequirementArg {
        kind,
        token,
        min,
        max,
    })
}

fn parse_security_commitment(value: &str) -> std::result::Result<SecurityCommitmentArg, String> {
    let parts: Vec<_> = value.split(':').collect();
    if parts.len() != 3 {
        return Err("Expected format KIND:TOKEN:EXPOSURE".to_string());
    }
    let kind = parse_asset_kind(parts[0])?;
    let token = parse_token(kind, parts[1])?;
    let exposure = parse_bps(parts[2], "exposure")?;
    Ok(SecurityCommitmentArg {
        kind,
        token,
        exposure,
    })
}

fn parse_asset_kind(value: &str) -> std::result::Result<AssetKindArg, String> {
    match value.to_lowercase().as_str() {
        "native" | "eth" => Ok(AssetKindArg::Native),
        "erc20" => Ok(AssetKindArg::Erc20),
        other => Err(format!("unsupported asset kind '{other}'")),
    }
}

fn parse_token(kind: AssetKindArg, raw: &str) -> std::result::Result<Address, String> {
    match kind {
        AssetKindArg::Native => {
            if raw.is_empty() || raw == "_" || raw == "0" {
                return Ok(Address::ZERO);
            }
            if raw.eq_ignore_ascii_case("native") {
                return Ok(Address::ZERO);
            }
            let addr = Address::from_str(raw)
                .map_err(|e| format!("invalid native token placeholder '{raw}': {e}"))?;
            if addr != Address::ZERO {
                return Err("native asset must use the zero address placeholder".to_string());
            }
            Ok(Address::ZERO)
        }
        AssetKindArg::Erc20 => {
            Address::from_str(raw).map_err(|e| format!("invalid ERC-20 address '{raw}': {e}"))
        }
    }
}

fn parse_bps(value: &str, label: &str) -> std::result::Result<u16, String> {
    let parsed = value
        .parse::<u16>()
        .map_err(|e| format!("invalid {label} '{value}': {e}"))?;
    if parsed > MAX_BPS {
        return Err(format!("{label} '{value}' exceeds {MAX_BPS} bps"));
    }
    Ok(parsed)
}

fn requirement_to_abi(arg: SecurityRequirementArg) -> ITangleTypes::AssetSecurityRequirement {
    ITangleTypes::AssetSecurityRequirement {
        asset: asset_to_abi(arg.kind, arg.token),
        minExposureBps: arg.min,
        maxExposureBps: arg.max,
    }
}

fn commitment_to_abi(arg: SecurityCommitmentArg) -> ITangleTypes::AssetSecurityCommitment {
    ITangleTypes::AssetSecurityCommitment {
        asset: asset_to_abi(arg.kind, arg.token),
        exposureBps: arg.exposure,
    }
}

fn asset_to_abi(kind: AssetKindArg, token: Address) -> ITangleTypes::Asset {
    let kind_value = match kind {
        AssetKindArg::Native => ITangleTypes::AssetKind::from_underlying(0).into_underlying(),
        AssetKindArg::Erc20 => ITangleTypes::AssetKind::from_underlying(1).into_underlying(),
    };
    ITangleTypes::Asset {
        kind: kind_value,
        token,
    }
}

const MAX_BPS: u16 = 10_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposures_are_optional() {
        assert!(normalize_operator_exposures(&[], 2).unwrap().is_none());

        let exposures = normalize_operator_exposures(&[5_000, 5_000], 2)
            .unwrap()
            .expect("exposures present");
        assert_eq!(exposures, vec![5_000, 5_000]);
    }

    #[test]
    fn exposures_length_enforced() {
        let err = normalize_operator_exposures(&[5_000], 2).unwrap_err();
        assert!(
            err.to_string()
                .contains("Expected 2 operator exposure values")
        );
    }

    #[test]
    fn exposures_bps_capped() {
        let err = normalize_operator_exposures(&[10_001], 1).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn parse_security_requirement_native() {
        let arg = parse_security_requirement("native:_:100:200").expect("valid requirement");
        assert!(matches!(arg.kind, AssetKindArg::Native));
        assert_eq!(arg.token, Address::ZERO);
        assert_eq!(arg.min, 100);
        assert_eq!(arg.max, 200);
    }

    #[test]
    fn parse_security_requirement_checks_bounds() {
        let err = parse_security_requirement("native:_:0:0").unwrap_err();
        assert!(err.contains("minimum exposure"));

        let err = parse_security_requirement("native:_:200:100").unwrap_err();
        assert!(err.contains("cannot exceed"));
    }

    #[test]
    fn parse_security_commitment_erc20() {
        let token = "0x0000000000000000000000000000000000000001";
        let arg =
            parse_security_commitment(&format!("erc20:{token}:7500")).expect("valid commitment");
        assert!(matches!(arg.kind, AssetKindArg::Erc20));
        assert_eq!(arg.token, Address::from_str(token).unwrap());
        assert_eq!(arg.exposure, 7_500);
    }
}

async fn ensure_schema_loaded(
    cache: &mut Option<JobSchema>,
    client: &TangleEvmClient,
    blueprint_id: u64,
    job_index: u8,
) -> Result<()> {
    if cache.is_none() {
        let schema = load_job_schema(client, blueprint_id, job_index).await?;
        *cache = Some(schema);
    }
    Ok(())
}

fn decode_payload_hex(value: &str) -> Result<Bytes> {
    let trimmed = value.trim();
    let raw = trimmed.trim_start_matches("0x");
    let bytes = hex::decode(raw).context("invalid payload hex")?;
    Ok(Bytes::from(bytes))
}

fn read_payload_file(path: &PathBuf) -> Result<Bytes> {
    let data = fs::read(path)
        .with_context(|| format!("Failed to read payload file {}", path.display()))?;
    Ok(Bytes::from(data))
}

fn print_raw_job_result(bytes: &[u8]) {
    println!(
        "Job result ready ({} bytes): 0x{}",
        bytes.len(),
        hex::encode(bytes)
    );
}

fn load_config_payload(config_file: Option<PathBuf>, config_hex: Option<String>) -> Result<Bytes> {
    if let Some(path) = config_file {
        let data = fs::read(&path)
            .with_context(|| format!("Failed to read config file {}", path.display()))?;
        return Ok(Bytes::from(data));
    }

    if let Some(hex_value) = config_hex {
        let trimmed = hex_value.trim_start_matches("0x");
        let bytes = hex::decode(trimmed).context("invalid config hex")?;
        return Ok(Bytes::from(bytes));
    }

    Ok(Bytes::new())
}

fn log_tx(prefix: &str, tx: &TransactionResult, json: bool) {
    if json {
        println!(
            "{}",
            json!({
                "event": "tx_submitted",
                "action": prefix,
                "tx_hash": format!("{:#x}", tx.tx_hash),
            })
        );
        println!(
            "{}",
            json!({
                "event": "tx_confirmed",
                "action": prefix,
                "tx_hash": format!("{:#x}", tx.tx_hash),
                "block": tx.block_number,
                "gas_used": tx.gas_used,
                "success": tx.success,
            })
        );
        return;
    }

    println!("{prefix}: submitted tx_hash={:#x}", tx.tx_hash);
    if tx.success {
        println!(
            "{prefix}: confirmed block={:?} gas_used={}",
            tx.block_number, tx.gas_used
        );
    } else {
        println!(
            "{prefix} failed: tx_hash={:#x} block={:?}",
            tx.tx_hash, tx.block_number
        );
    }
}

async fn register_operator(
    network: TangleClientArgs,
    rpc_endpoint: Option<String>,
    blueprint_id: u64,
    registration_inputs: Option<PathBuf>,
) -> Result<()> {
    ensure_keys(network.keystore_path(), &[SupportedKey::Ecdsa])?;

    let registration_payload = if let Some(path) = registration_inputs {
        Some(Bytes::from(fs::read(&path).map_err(|e| {
            eyre!("Failed to read registration inputs: {e}")
        })?))
    } else {
        None
    };

    let rpc_endpoint = rpc_endpoint.unwrap_or_else(|| network.http_rpc_url.to_string());
    let client = network.connect(blueprint_id, None).await?;
    let signer = load_evm_signer(network.keystore_path())?;

    println!("Registering operator {}", signer.operator_address);
    let tx = client
        .register_operator(blueprint_id, rpc_endpoint, registration_payload)
        .await?;

    log_tx("Registration", &tx, false);
    println!("Operator ready: {}", signer.operator_address);

    Ok(())
}

fn resolve_runtime_options(
    spawn_method: SpawnMethod,
    stored: RuntimePreferences,
    preferred_override: Option<PreferredSourceArg>,
    vm_flag: bool,
    no_vm_flag: bool,
) -> Result<(SourceType, bool)> {
    let mut preferred_source = stored
        .preferred_source
        .unwrap_or(spawn_method.preferred_source());
    if let Some(arg) = preferred_override {
        preferred_source = SourceType::from(arg);
    }

    let mut use_vm = stored.use_vm.unwrap_or(spawn_method.use_vm());
    match (vm_flag, no_vm_flag) {
        (true, true) => {
            return Err(eyre!(
                "Use either --vm or --no-vm when overriding manager runtime"
            ));
        }
        (true, false) => use_vm = true,
        (false, true) => use_vm = false,
        _ => {}
    }

    Ok((preferred_source, use_vm))
}

fn collect_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|x| x == "tangle") {
        std::env::args().skip(1).collect()
    } else {
        args
    }
}

fn ensure_keys(path: &PathBuf, required: &[SupportedKey]) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    let keystore = Keystore::new(KeystoreConfig::new().fs_root(path))?;
    let mut missing = Vec::new();

    if required.contains(&SupportedKey::Ecdsa) && keystore.list_local::<K256Ecdsa>()?.is_empty() {
        missing.push(SupportedKey::Ecdsa);
    }

    if !missing.is_empty() {
        println!(
            "Keystore at {} is missing required keys. Let's set them up.",
            path.display()
        );
        let inputs = prompt_for_keys(missing)?;
        for (kind, secret) in inputs {
            import_key(Protocol::TangleEvm, kind, &secret, path)?;
        }
    }

    Ok(())
}

fn parse_supported_chain(network: &str, rpc_url: &Url) -> Result<SupportedChains, ConfigError> {
    match network.to_lowercase().as_str() {
        "local" => Ok(SupportedChains::LocalTestnet),
        "testnet" => Ok(SupportedChains::Testnet),
        "mainnet" => {
            if rpc_url.as_str().contains("127.0.0.1") || rpc_url.as_str().contains("localhost") {
                Ok(SupportedChains::LocalMainnet)
            } else {
                Ok(SupportedChains::Mainnet)
            }
        }
        other => Err(ConfigError::Other(
            format!("Invalid network: {other}").into(),
        )),
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
#[derive(Subcommand, Debug)]
enum JobsCommands {
    /// List job definitions for a blueprint.
    List {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Show metadata for a specific job call.
    Show {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Call identifier to inspect.
        #[arg(long)]
        call_id: u64,
        /// Emit JSON output.
        #[arg(long)]
        json: bool,
    },
    /// Submit a job invocation to a service.
    Submit {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Job index exported by the blueprint router.
        #[arg(long)]
        job: u8,
        /// Hex-encoded ABI payload for the job inputs.
        #[arg(long = "payload-hex", value_name = "HEX")]
        payload_hex: Option<String>,
        /// File containing raw bytes to use as job inputs.
        #[arg(long = "payload-file", value_name = "FILE")]
        payload_file: Option<PathBuf>,
        /// JSON file containing structured inputs that match the job schema.
        #[arg(
            long = "params-file",
            value_name = "FILE",
            conflicts_with_all = ["payload_hex", "payload_file"]
        )]
        params_file: Option<PathBuf>,
        /// Prompt for each argument interactively using the job schema.
        #[arg(
            long,
            conflicts_with_all = ["payload_hex", "payload_file", "params_file"],
            action = clap::ArgAction::SetTrue
        )]
        prompt: bool,
        /// Wait for a result after submitting.
        #[arg(long)]
        watch: bool,
        /// Timeout (seconds) when waiting for a result.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
        /// Emit transaction logs as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Wait for a job result using a call identifier.
    Watch {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint identifier.
        #[arg(long)]
        blueprint_id: u64,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Call identifier returned by `jobs submit`.
        #[arg(long)]
        call_id: u64,
        /// Timeout (seconds) before bailing.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
}
