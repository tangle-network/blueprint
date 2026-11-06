use super::{chain::log_tx, jobs::JobsCommands, keys::ensure_keys, service::ServiceCommands};
use alloy_primitives::Bytes;
use blueprint_manager::config::SourceType;
use blueprint_runner::{
    config::{BlueprintEnvironment, Protocol},
    error::ConfigError,
};
use cargo_tangle::command::create::{BlueprintType, TemplateVariables, new_blueprint};
use cargo_tangle::command::debug::{self, DebugCommands};
use cargo_tangle::command::deploy::tangle as deploy_tangle;
use cargo_tangle::command::keys::SupportedKey;
use cargo_tangle::command::list;
use cargo_tangle::command::run::tangle::{RunOpts, run_blueprint};
use cargo_tangle::command::signer::load_evm_signer;
use cargo_tangle::command::tangle::{PreferredSourceArg, SpawnMethod, TangleClientArgs};
use cargo_tangle::settings::{
    RuntimePreferences, load_protocol_settings, load_runtime_preferences, write_runtime_preferences,
};
use cargo_tangle::utils::find_registration_inputs;
use clap::Subcommand;
use color_eyre::eyre::{Result, eyre};
use std::{fs, path::PathBuf};
use url::Url;

#[derive(Subcommand, Debug)]
pub(super) enum BlueprintCommands {
    /// Create a new blueprint project from a template.
    ///
    /// Scaffolds a complete blueprint project with jobs, tests, and build configuration.
    #[command(visible_alias = "c")]
    Create {
        /// Name for the new blueprint project (used for directory and package name).
        #[arg(short = 'n', long, value_name = "NAME", env = "NAME")]
        name: String,
        #[command(flatten)]
        source: Option<cargo_tangle::command::create::Source>,
        #[command(flatten)]
        blueprint_type: Option<BlueprintType>,

        #[command(flatten)]
        template_variables: TemplateVariables,

        /// Define a value for template variables (can be used multiple times).
        ///
        /// Example: --define gh-username=myusername
        /// Example with spaces: --define "project-description=My Blueprint description"
        #[arg(
            long,
            short = 'd',
            number_of_values = 1,
            conflicts_with = "template_values_file"
        )]
        define: Vec<String>,
        /// JSON file containing template variable values.
        #[arg(long, value_name = "FILE", conflicts_with = "define")]
        template_values_file: Option<String>,
        /// Skip interactive prompts and use defaults.
        #[arg(long)]
        skip_prompts: bool,
        /// Generate into the current directory instead of creating a new <NAME>/ subdirectory.
        ///
        /// `name` still names the crate/package; it just no longer names the
        /// directory. The current directory must be empty apart from `.git`
        /// and `.gitignore`.
        #[arg(long)]
        init: bool,
    },

    /// Deploy a blueprint to a protocol.
    ///
    /// One-time per blueprint: compiles and registers your blueprint on-chain,
    /// returning a new `blueprintId`. For shipping subsequent binary versions
    /// of an already-deployed blueprint, use `cargo tangle blueprint ship`.
    #[command(visible_alias = "d")]
    Deploy {
        #[command(subcommand)]
        target: DeployTarget,
    },

    /// Run a blueprint as an operator.
    ///
    /// Starts the blueprint runtime, connects to the network, and listens for jobs.
    /// Requires keys in the keystore and protocol settings in settings.env.
    #[command(visible_alias = "r")]
    Run {
        /// Target protocol: tangle.
        #[arg(short = 'p', long, value_enum, default_value = "tangle")]
        protocol: Protocol,
        /// HTTP RPC endpoint for the EVM chain.
        #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:8545")]
        http_rpc_url: Url,
        /// WebSocket RPC endpoint for event subscriptions.
        #[arg(long, value_name = "URL", default_value = "ws://127.0.0.1:8546")]
        ws_rpc_url: Url,
        /// Path to keystore directory containing operator keys.
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
        /// Network name: local, testnet, or mainnet.
        #[arg(short = 'w', long, default_value = "local")]
        network: String,
        /// Directory for blueprint data and state.
        #[arg(short = 'd', long)]
        data_dir: Option<PathBuf>,
        /// P2P bootstrap nodes for gossip network.
        #[arg(short = 'n', long)]
        bootnodes: Option<Vec<String>>,
        /// Path to settings.env file with protocol configuration.
        #[arg(short = 'f', long, default_value = "./settings.env")]
        settings_file: Option<PathBuf>,
        /// Allow unchecked attestations (testing only, insecure).
        #[arg(long, env)]
        allow_unchecked_attestations: bool,
        /// Preferred runtime: vm (sandboxed) or native.
        #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
        spawn_method: SpawnMethod,
        /// Override blueprint source: wasm, binary, or container.
        #[arg(long, value_enum)]
        preferred_source: Option<PreferredSourceArg>,
        /// Force VM sandbox execution.
        #[arg(long)]
        vm: bool,
        /// Disable VM sandbox (use native execution).
        #[arg(long)]
        no_vm: bool,
        /// Save runtime preferences to settings file.
        #[arg(long)]
        save_runtime_prefs: bool,
    },

    /// Generate registration data without submitting on-chain.
    ///
    /// Produces the signed registration inputs needed to register as an operator
    /// for this blueprint. Useful for offline signing workflows.
    #[command(visible_alias = "pre")]
    Preregister {
        /// Target protocol: tangle.
        #[arg(short = 'p', long, value_enum, default_value = "tangle")]
        protocol: Protocol,
        /// HTTP RPC endpoint for the EVM chain.
        #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:8545")]
        http_rpc_url: Url,
        /// WebSocket RPC endpoint.
        #[arg(long, value_name = "URL", default_value = "ws://127.0.0.1:8546")]
        ws_rpc_url: Url,
        /// Path to keystore directory containing operator keys.
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
        /// Network name: local, testnet, or mainnet.
        #[arg(short = 'n', long, default_value = "local")]
        network: String,
        /// Directory for blueprint data and state.
        #[arg(short = 'd', long)]
        data_dir: Option<PathBuf>,
        /// Path to settings.env file with protocol configuration.
        #[arg(short = 'f', long, default_value = "./settings.env")]
        settings_file: Option<PathBuf>,
        /// Preferred runtime: vm (sandboxed) or native.
        #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
        spawn_method: SpawnMethod,
        /// Override blueprint source: wasm, binary, or container.
        #[arg(long, value_enum)]
        preferred_source: Option<PreferredSourceArg>,
        /// Force VM sandbox execution.
        #[arg(long)]
        vm: bool,
        /// Disable VM sandbox (use native execution).
        #[arg(long)]
        no_vm: bool,
        /// Save runtime preferences to settings file.
        #[arg(long)]
        save_runtime_prefs: bool,
    },

    /// Register as an operator for a blueprint.
    ///
    /// Submits operator registration to the Tangle contract, enabling you to
    /// receive service requests and job assignments for this blueprint.
    #[command(visible_alias = "reg")]
    Register {
        #[command(flatten)]
        network: TangleClientArgs,
        /// RPC endpoint override (uses network default if omitted).
        #[arg(long, value_name = "URL")]
        rpc_endpoint: Option<String>,
        /// Blueprint ID to register for.
        #[arg(long)]
        blueprint_id: u64,
        /// JSON file with pre-signed registration inputs from preregister.
        #[arg(long, value_name = "FILE")]
        registration_inputs: Option<PathBuf>,
    },
    /// List blueprints, services, and service requests.
    #[command(visible_alias = "ls")]
    List {
        #[command(subcommand)]
        command: ListCommands,
    },
    /// Local development and debugging utilities.
    ///
    /// Spawn test harnesses, mock services, and debugging tools.
    #[command(visible_alias = "dbg")]
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },

    /// Submit, watch, and inspect job invocations.
    ///
    /// Manage job calls to blueprint services.
    #[command(visible_alias = "j")]
    Jobs {
        #[command(subcommand)]
        command: JobsCommands,
    },

    /// Manage service lifecycle (request, approve, join, leave).
    ///
    /// Control service instantiation and operator participation.
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },

    /// One-command release: build → hash → pin → publish → (optionally) promote
    /// and bulk-flip service policies. Wraps `publish-version` +
    /// `set-active-version` + optional IPFS pin in a single interactive flow.
    /// Designed for both developer use (interactive prompts) and CI
    /// (`--yes --pin-ipfs --promote`). Publishes the next monotonic version —
    /// v0 on first run, vN otherwise.
    Ship {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Accept all prompts. Implies `--json` for CI-friendly output.
        #[arg(long)]
        yes: bool,
        /// Don't run `cargo build --release` — caller supplies `--binary`.
        #[arg(long)]
        no_build: bool,
        /// Cargo package to build (`-p <pkg>`).
        #[arg(long, short = 'p', value_name = "NAME")]
        package: Option<String>,
        /// Path to a pre-built release binary.
        #[arg(long, value_name = "PATH")]
        binary: Option<PathBuf>,
        /// Pre-existing artifact URI (`ipfs://...`, `https://...`). Skips pinning.
        #[arg(long, value_name = "URI", conflicts_with = "pin_ipfs")]
        binary_uri: Option<String>,
        /// Pin the binary to IPFS using `IPFS_API_URL` + `IPFS_API_TOKEN`, or `PINATA_JWT`.
        #[arg(long)]
        pin_ipfs: bool,
        /// sigstore/SLSA bundle whose sha256 lands on-chain as `attestationHash`.
        #[arg(long, value_name = "PATH")]
        attestation_bundle: Option<PathBuf>,
        /// Explicit `attestationHash` (32-byte hex). Conflicts with `--attestation-bundle`.
        #[arg(long, value_name = "HEX", conflicts_with = "attestation_bundle")]
        attestation_hash: Option<String>,
        /// Promote the new version to active (`setActiveBinaryVersion`).
        #[arg(long, conflicts_with = "no_promote")]
        promote: bool,
        /// Explicitly skip promotion even if interactive default is yes.
        #[arg(long)]
        no_promote: bool,
        /// Comma-separated service IDs to bulk-flip into AUTO policy.
        #[arg(long, value_name = "LIST")]
        policy_services: Option<String>,
        /// Validate everything end-to-end without submitting transactions.
        #[arg(long)]
        dry_run: bool,
        /// Override blueprint id (skips auto-detection from settings.env / metadata).
        #[arg(long)]
        blueprint_id: Option<u64>,
        /// JSON output for machine readers (also implied by `--yes`).
        #[arg(long)]
        json: bool,
    },

    /// Publish a new binary version for a blueprint (primitive).
    ///
    /// Computes the artifact sha256 locally, optionally pins it to IPFS,
    /// and submits `publishBinaryVersion` against the Tangle diamond. Caller
    /// must be the blueprint owner. For the high-level "build + pin + publish
    /// + promote" path, see `cargo tangle blueprint ship`.
    PublishVersion {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID to publish under.
        #[arg(long)]
        blueprint_id: u64,
        /// Path to the binary artifact to publish.
        #[arg(long, value_name = "PATH")]
        binary: PathBuf,
        /// Explicit URI for the binary (e.g. `ipfs://...`). Overrides `--pin-to-ipfs`.
        #[arg(long, value_name = "URI")]
        binary_uri: Option<String>,
        /// Pin the binary to IPFS via `IPFS_API_URL`+`IPFS_API_TOKEN` or `PINATA_JWT`.
        #[arg(long, conflicts_with = "binary_uri")]
        pin_to_ipfs: bool,
        /// Optional sigstore/SLSA bundle file. Its sha256 lands on-chain as `attestationHash`.
        #[arg(long, value_name = "PATH")]
        attestation_bundle: Option<PathBuf>,
        /// Explicit `attestationHash` override (32-byte hex). Conflicts with `--attestation-bundle`.
        #[arg(long, value_name = "HEX", conflicts_with = "attestation_bundle")]
        attestation_hash: Option<String>,
        /// Output JSON instead of human-readable text.
        #[arg(long)]
        json: bool,
    },

    /// Set the active binary version for a blueprint (affects services on AUTO).
    SetActiveVersion {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// Mark a binary version as deprecated (one-way).
    DeprecateVersion {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// List all published binary versions for a blueprint.
    ListVersions {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// Show a single binary version.
    ShowVersion {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// Compute the trust score for a (blueprint, version) pair.
    ///
    /// Walks all non-revoked, non-expired attestations and weights them by the
    /// `BlueprintAuditors` registry. Use in CI gates: e.g. only deploy if
    /// `score >= 80`.
    TrustScore {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        /// `BlueprintAuditors` registry address. If unset, all attesters are
        /// treated as anonymous (weight 0) and the score collapses to zero.
        #[arg(long, value_name = "ADDRESS")]
        auditors_contract: Option<String>,
        /// Fail the process with a non-zero exit code if `score < min_score`.
        /// Use for CI gating (e.g. `--min-score 80`).
        #[arg(long, value_name = "N")]
        min_score: Option<u32>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
pub(super) enum ListCommands {
    /// List all registered blueprints on the network.
    ///
    /// Shows blueprint IDs, names, owners, and operator requirements.
    Blueprints {
        #[command(flatten)]
        network: TangleClientArgs,
    },
    /// List all pending service requests awaiting operator approval.
    ///
    /// Shows requests that operators need to approve or reject.
    Requests {
        #[command(flatten)]
        network: TangleClientArgs,
    },
    /// List all active services on the network.
    ///
    /// Shows running service instances with their operators and status.
    Services {
        #[command(flatten)]
        network: TangleClientArgs,
    },
}

#[derive(Subcommand, Debug)]
pub(super) enum DeployTarget {
    /// Deploy to Tangle EVM protocol.
    ///
    /// Registers your blueprint in the Tangle contract registry.
    Tangle(cargo_tangle::command::deploy::tangle::TangleDeployArgs),
}

pub(super) async fn run(command: BlueprintCommands) -> Result<()> {
    match command {
        BlueprintCommands::Create {
            name,
            source,
            blueprint_type,
            template_variables,
            define,
            template_values_file,
            skip_prompts,
            init,
        } => {
            new_blueprint(
                &name,
                source,
                blueprint_type,
                define,
                template_variables,
                &template_values_file,
                skip_prompts,
                init,
            )?;
        }
        BlueprintCommands::Deploy { target } => match target {
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
            let settings_file = settings_file.unwrap_or_else(|| PathBuf::from("./settings.env"));
            let protocol_settings = load_protocol_settings(protocol, &settings_file)?;
            let runtime_prefs = load_runtime_preferences();
            let (preferred_source_value, use_vm_value) =
                resolve_runtime_options(spawn_method, runtime_prefs, preferred_source, vm, no_vm)?;

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

            match protocol {
                Protocol::Tangle => {
                    let settings = protocol_settings.tangle().map_err(|e| eyre!("{e}"))?;

                    let run_opts = RunOpts {
                        http_rpc_url,
                        ws_rpc_url,
                        blueprint_id: settings.blueprint_id,
                        service_id: settings.service_id,
                        tangle_contract: settings.tangle_contract,
                        staking_contract: settings.staking_contract,
                        status_registry_contract: settings.status_registry_contract,
                        keystore_path: config.keystore_uri.clone(),
                        data_dir,
                        allow_unchecked_attestations,
                        registration_mode: false,
                        registration_capture_only: false,
                        preferred_source: preferred_source_value,
                        use_vm: use_vm_value,
                        dry_run: false,
                        shutdown_after: None,
                    };
                    run_blueprint(run_opts).await?;
                }
                _ => return Err(ConfigError::UnexpectedProtocol("Unsupported protocol").into()),
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
            if protocol != Protocol::Tangle {
                return Err(eyre!(
                    "Preregistration is only supported for the Tangle EVM protocol"
                ));
            }

            let settings_file = settings_file.unwrap_or_else(|| PathBuf::from("./settings.env"));
            let protocol_settings = load_protocol_settings(protocol, &settings_file)?;
            let settings = protocol_settings.tangle().map_err(|e| eyre!("{e}"))?;

            let keystore_path = keystore_path.unwrap_or_else(|| PathBuf::from("./keystore"));
            ensure_keys(&keystore_path, &[SupportedKey::Ecdsa])?;

            let prereg_data_dir = data_dir.clone();
            let base_data_dir = prereg_data_dir
                .clone()
                .unwrap_or_else(|| PathBuf::from("./data"));

            let runtime_prefs = load_runtime_preferences();
            let (preferred_source_value, use_vm_value) =
                resolve_runtime_options(spawn_method, runtime_prefs, preferred_source, vm, no_vm)?;

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
                staking_contract: settings.staking_contract,
                status_registry_contract: settings.status_registry_contract,
                keystore_path: keystore_path.to_string_lossy().to_string(),
                data_dir: prereg_data_dir,
                allow_unchecked_attestations: false,
                registration_mode: true,
                registration_capture_only: true,
                preferred_source: preferred_source_value,
                use_vm: use_vm_value,
                dry_run: false,
                shutdown_after: None,
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
        BlueprintCommands::Jobs { command } => super::jobs::run(command).await?,
        BlueprintCommands::Service { command } => super::service::run(command).await?,
        command => super::blueprint_versions::run(command).await?,
    }
    Ok(())
}

async fn register_operator(
    network: TangleClientArgs,
    rpc_endpoint: Option<String>,
    blueprint_id: u64,
    registration_inputs: Option<PathBuf>,
) -> Result<()> {
    let keystore_path = network.keystore_path()?;
    ensure_keys(&keystore_path, &[SupportedKey::Ecdsa])?;

    let registration_payload = if let Some(path) = registration_inputs {
        Some(Bytes::from(fs::read(&path).map_err(|e| {
            eyre!("Failed to read registration inputs: {e}")
        })?))
    } else {
        None
    };

    let rpc_endpoint = rpc_endpoint.unwrap_or_else(|| {
        network
            .http_rpc_url()
            .ok()
            .map(|u| u.to_string())
            .unwrap_or_default()
    });
    let client = network.connect(blueprint_id, None).await?;
    let signer = load_evm_signer(&keystore_path)?;

    println!("Registering operator {}", signer.operator_address);
    println!("  signing key     0x{}", hex::encode(signer.public_key));
    let tx = client
        .register_operator(blueprint_id, rpc_endpoint, registration_payload)
        .await?;

    log_tx("Registration", &tx, false);
    println!("Operator ready: {}", signer.operator_address);

    Ok(())
}

pub(super) fn resolve_runtime_options(
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
