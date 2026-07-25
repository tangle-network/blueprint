use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use alloy_primitives::{Address, Bytes, U256};
use blueprint_client_tangle::{
    BlueprintSelectionMode, DelegationMode, TangleClient, TransactionResult,
    contracts::ITangleTypes,
};
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_manager::config::SourceType;
use blueprint_runner::config::{BlueprintEnvironment, Protocol};
use blueprint_runner::error::ConfigError;
use cargo_tangle::command::create::{BlueprintType, TemplateVariables, new_blueprint};
use cargo_tangle::command::debug::{self, DebugCommands};
use cargo_tangle::command::delegator;
use cargo_tangle::command::deploy::tangle as deploy_tangle;
use cargo_tangle::command::dev::{self, DevCommands};
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
use color_eyre::eyre::{Context, Result, bail, ensure, eyre};
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
    /// Create, deploy, run, and manage blueprints.
    ///
    /// Blueprints are service templates that define jobs, pricing, and operator requirements.
    /// Use this to develop and deploy your own blueprint services.
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

    /// Generate, import, export, and list cryptographic keys.
    ///
    /// Manage ECDSA and BLS keys used for signing transactions and attestations.
    #[command(visible_alias = "k")]
    Key {
        #[command(subcommand)]
        command: KeyCommands,
    },

    /// Deposit, delegate, and withdraw stake as a delegator.
    ///
    /// Delegators provide economic security by staking tokens to operators.
    /// Earn rewards while supporting the network's security.
    #[command(visible_alias = "del")]
    Delegator {
        #[command(subcommand)]
        command: DelegatorCommands,
    },

    /// Register, stake, and manage services as an operator.
    ///
    /// Operators run blueprint services and earn rewards for their work.
    /// Manage your stake, delegation settings, and service participation.
    #[command(visible_alias = "op")]
    Operator {
        #[command(subcommand)]
        command: OperatorCommands,
    },

    /// Zero-config developer workspace: spin up a local devnet and write
    /// `.tangle.toml` so every subsequent cargo-tangle command works with
    /// no contract-address / RPC flags. Pair with `harness` when you need
    /// the full multi-blueprint + router stack.
    Dev {
        #[command(subcommand)]
        command: DevCommands,
    },

    /// Spin up a local Tangle dev environment with multiple blueprints
    /// running against real on-chain infrastructure.
    #[command(visible_alias = "h")]
    Harness {
        #[command(subcommand)]
        command: cargo_tangle::command::harness::HarnessCommands,
    },

    /// Auditor flow: attest, revoke, list attestations against blueprint binary versions.
    ///
    /// Auditors register their report against a published `(blueprintId, versionId)`.
    /// Trust weighting is delegated to the `BlueprintAuditors` registry; weights
    /// are applied off-chain when computing `cargo tangle blueprint trust-score`.
    Attest {
        #[command(subcommand)]
        command: AttestCommands,
    },
}

#[derive(Subcommand, Debug)]
enum BlueprintCommands {
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
enum KeyCommands {
    /// Generate a new cryptographic key pair.
    ///
    /// Creates ECDSA (for signing transactions) or BLS keys (for threshold signatures).
    #[command(visible_alias = "g")]
    Generate {
        /// Key type: ecdsa (transactions), bls-bn254 (aggregation), sr25519, ed25519.
        #[arg(short = 't', long, value_enum)]
        key_type: SupportedKey,
        /// Output file path. If omitted, prints to stdout.
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
        /// Optional seed bytes for deterministic key generation.
        #[arg(long)]
        seed: Option<Vec<u8>>,
        /// Display the secret key (use with caution).
        #[arg(short = 'v', long)]
        show_secret: bool,
    },
    /// Import an existing key into the keystore.
    ///
    /// Use this to add a pre-existing private key to your local keystore.
    #[command(visible_alias = "i")]
    Import {
        /// Key type to import. Auto-detected if omitted.
        #[arg(short = 't', long, value_enum)]
        key_type: Option<SupportedKey>,
        /// Hex-encoded secret key (without 0x prefix).
        #[arg(short = 'x', long)]
        secret: Option<String>,
        /// Path to the keystore directory.
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
        /// Target protocol for key organization.
        #[arg(short = 'p', long, value_enum, default_value = "tangle")]
        protocol: Protocol,
    },
    /// Export a key from the keystore by its public key.
    #[command(visible_alias = "e")]
    Export {
        /// Key type to export.
        #[arg(short = 't', long, value_enum)]
        key_type: SupportedKey,
        /// Public key (hex) to look up in the keystore.
        #[arg(short = 'p', long)]
        public: String,
        /// Path to the keystore directory.
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
    },
    /// List all keys stored in the keystore.
    #[command(visible_alias = "l")]
    List {
        /// Path to the keystore directory.
        #[arg(short = 'k', long)]
        keystore_path: PathBuf,
    },
    /// Generate a new BIP-39 mnemonic phrase.
    ///
    /// Use this to create a seed phrase for deriving keys.
    #[command(visible_alias = "m")]
    GenerateMnemonic {
        /// Number of words (12, 15, 18, 21, or 24). Default: 12.
        #[arg(short = 'w', long, value_parser = clap::value_parser!(u32).range(12..=24))]
        word_count: Option<u32>,
    },
}

#[derive(Subcommand, Debug)]
enum ListCommands {
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
enum ServiceCommands {
    /// Request a new service instance from operators.
    ///
    /// Creates a service request that operators can approve or reject.
    /// Payment and security requirements can be specified to filter operators.
    Request {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID to instantiate.
        #[arg(long)]
        blueprint_id: u64,
        /// Operator addresses to include (can specify multiple times).
        #[arg(long = "operator", required = true)]
        operators: Vec<String>,
        /// Exposure per operator in basis points (10000 = 100%). Matches --operator order.
        #[arg(long = "operator-exposure-bps")]
        operator_exposures: Vec<u16>,
        /// Addresses allowed to submit jobs (in addition to requester).
        #[arg(long = "permitted-caller")]
        permitted_callers: Vec<String>,
        /// File containing service configuration (raw bytes).
        #[arg(long = "config-file", value_name = "PATH")]
        config_file: Option<PathBuf>,
        /// Hex-encoded service configuration.
        #[arg(long = "config-hex", value_name = "HEX")]
        config_hex: Option<String>,
        /// Time-to-live in seconds (0 = no expiration).
        #[arg(long, default_value_t = 600)]
        ttl: u64,
        /// ERC20 token for payment (0x0 = native token).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        payment_token: String,
        /// Payment amount in wei.
        #[arg(long, default_value_t = 0)]
        payment_amount: u128,
        /// Security requirement (format: KIND:TOKEN:MIN:MAX, can repeat).
        ///
        /// KIND: 0=native, 1=erc20. MIN/MAX are stake bounds in wei.
        #[arg(
            long = "security-requirement",
            value_name = "KIND:TOKEN:MIN:MAX",
            value_parser = parse_security_requirement
        )]
        security_requirements: Vec<SecurityRequirementArg>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Approve a pending service request as an operator.
    ///
    /// Commits your stake to the service and enables job execution.
    Approve {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Request ID to approve.
        #[arg(long)]
        request_id: u64,
        /// Percentage of your stake to commit to this service (0-100).
        #[arg(long = "staking-percent", default_value_t = 50)]
        staking_percent: u8,
        /// Explicit security commitment (format: KIND:TOKEN:EXPOSURE, can repeat).
        ///
        /// Overrides automatic allocation. EXPOSURE is in wei.
        #[arg(
            long = "security-commitment",
            value_name = "KIND:TOKEN:EXPOSURE",
            value_parser = parse_security_commitment
        )]
        security_commitments: Vec<SecurityCommitmentArg>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Reject a pending service request as an operator.
    ///
    /// Declines participation in the service. If all operators reject,
    /// the request fails and payment is refunded to the requester.
    Reject {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Request ID to reject.
        #[arg(long)]
        request_id: u64,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Join a running dynamic service as an operator.
    ///
    /// For services with open membership, allows operators to join after creation.
    Join {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service ID to join.
        #[arg(long)]
        service_id: u64,
        /// Stake exposure in basis points (10000 = 100% of your delegated stake).
        #[arg(long, default_value_t = MAX_BPS)]
        exposure_bps: u16,
        /// Asset security commitment in format KIND:TOKEN:EXPOSURE_BPS.
        /// KIND: native/eth or erc20.
        /// TOKEN: Token/vault address (use _ or 0 for native).
        /// EXPOSURE_BPS: Exposure in basis points (e.g., 5000 = 50%).
        /// Can be specified multiple times for multiple commitments.
        /// Example: --commitment erc20:0x1234...abcd:5000
        #[arg(long, value_name = "KIND:TOKEN:EXPOSURE_BPS")]
        commitment: Vec<String>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Leave a dynamic service as an operator.
    ///
    /// Exits the service and recovers your committed stake after the unbonding period.
    Leave {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service ID to leave.
        #[arg(long)]
        service_id: u64,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Spawn a local service runtime for testing.
    ///
    /// Starts the blueprint runtime locally without full network participation.
    /// Useful for development and debugging.
    Spawn {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID defining the service logic.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID to spawn runtime for.
        #[arg(long)]
        service_id: u64,
        /// Runtime execution mode: vm (sandboxed) or native.
        #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
        spawn_method: SpawnMethod,
        /// Directory for blueprint data and state.
        #[arg(long, value_name = "PATH")]
        data_dir: Option<PathBuf>,
        /// Allow unchecked attestations (testing only, insecure).
        #[arg(long)]
        allow_unchecked_attestations: bool,
        /// Simulate execution without on-chain transactions.
        #[arg(long)]
        dry_run: bool,
        /// Override blueprint source: wasm, binary, or container.
        #[arg(long, value_enum)]
        preferred_source: Option<PreferredSourceArg>,
        /// Force VM sandbox execution.
        #[arg(long)]
        vm: bool,
        /// Disable VM sandbox (use native execution).
        #[arg(long)]
        no_vm: bool,
    },
    /// List all active services.
    List {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// List all pending service requests.
    Requests {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// Show details for a specific service request.
    Show {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Request ID to display.
        #[arg(long)]
        request_id: u64,
    },

    /// Set this service's upgrade policy: AUTO follows the blueprint owner's
    /// active version, APPROVE requires opt-in via `ack-version`, MANUAL pins
    /// to genesis.
    SetPolicy {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        service_id: u64,
        /// Policy to apply.
        #[arg(long, value_enum)]
        policy: cargo_tangle::command::upgrade::UpgradePolicyArg,
        #[arg(long)]
        json: bool,
    },

    /// Acknowledge a binary version for this service (under APPROVE policy).
    AckVersion {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        service_id: u64,
        #[arg(long)]
        version_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// Show the binary version a service is currently effectively running.
    EffectiveVersion {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        service_id: u64,
        /// Blueprint ID hosting this service. Required to resolve the
        /// effective version's blueprint context.
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// Show the service's upgrade policy + acked vs. available versions.
    UpgradeStatus {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        service_id: u64,
        /// Blueprint ID hosting this service.
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        json: bool,
    },

    /// List the binary versions available for this service.
    ///
    /// Talks to the local blueprint-manager (not the chain) and asks it to
    /// enumerate the published versions for the underlying blueprint, with
    /// the currently-running one flagged. Useful as the entrypoint for
    /// MANUAL-with-assist workflows (`upgrade-local`, `upgrade-whitelist`).
    Upgrades {
        #[arg(long)]
        service_id: u64,
        /// blueprint-manager local RPC base URL. Falls back to
        /// `BLUEPRINT_MANAGER_URL` env, then `http://127.0.0.1:9000`.
        #[arg(long, value_name = "URL")]
        manager_url: Option<Url>,
        #[arg(long)]
        json: bool,
    },

    /// Pre-authorize the manager to swap this service to `--version-id`.
    ///
    /// MANUAL-with-assist: stores a one-shot pin in the manager's local
    /// authz store. The manager will run its full verify-then-swap pipeline
    /// the next time it reconciles this service, then clear the pin. No
    /// on-chain ack tx is written; the service's upgrade policy must be
    /// MANUAL for this to take effect.
    UpgradeLocal {
        #[arg(long)]
        service_id: u64,
        #[arg(long)]
        version_id: u64,
        /// Don't actually pin — just report what the manager would do.
        #[arg(long)]
        dry_run: bool,
        #[arg(long, value_name = "URL")]
        manager_url: Option<Url>,
        #[arg(long)]
        json: bool,
    },

    /// Replace this service's MANUAL-with-assist whitelist of acceptable
    /// versions. Persisted across manager restarts. Pass `--versions ""` to
    /// clear.
    UpgradeWhitelist {
        #[arg(long)]
        service_id: u64,
        /// Comma-separated version IDs (e.g. `2,4,5`). Empty = clear.
        #[arg(long, value_name = "LIST")]
        versions: String,
        #[arg(long, value_name = "URL")]
        manager_url: Option<Url>,
        #[arg(long)]
        json: bool,
    },

    /// Record an explicit skip on a published version for this service.
    /// The manager will suppress alerts about it and never auto-swap into
    /// it even if it lands in `effectiveBinaryVersion`. Persists across
    /// manager restarts.
    UpgradeSkip {
        #[arg(long)]
        service_id: u64,
        #[arg(long)]
        version_id: u64,
        /// Required free-form reason. Lands in the manager's audit log.
        #[arg(long, value_name = "TEXT")]
        reason: String,
        #[arg(long, value_name = "URL")]
        manager_url: Option<Url>,
        #[arg(long)]
        json: bool,
    },

    /// Show the local-authz state (pinned, whitelisted, skipped) for this
    /// service along with the on-chain policy and currently-running binary.
    UpgradeAuthz {
        #[arg(long)]
        service_id: u64,
        #[arg(long, value_name = "URL")]
        manager_url: Option<Url>,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
enum AttestCommands {
    /// Submit a new attestation against `(blueprint_id, version_id)`.
    ///
    /// Hashes the report file (or accepts an explicit `--report-hash`), maps
    /// the severity string to its uint8 ladder entry, and submits
    /// `attestBinaryVersion`. Auditor identity = `msg.sender`.
    Submit {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        /// Path to report artifact (PDF, JSON, etc.) OR an off-chain URL.
        ///
        /// If the value resolves to an existing file, the sha256 is computed
        /// locally and the file may also be pinned to IPFS via `--pin-report-to-ipfs`.
        /// If it's a URL (e.g. `https://...`, `ipfs://...`), it's treated as
        /// `reportUri` with `reportHash = 0x0` unless `--report-hash` is provided.
        #[arg(long, value_name = "PATH_OR_URL")]
        report: String,
        /// Attestation kind.
        #[arg(long, value_enum)]
        kind: cargo_tangle::command::upgrade::AttestationKindArg,
        /// Severity discovered.
        #[arg(long, value_enum)]
        severity: cargo_tangle::command::upgrade::SeverityArg,
        /// Optional expiry as a duration from now (e.g. `6m`, `30d`, `1y`).
        #[arg(long, value_name = "DURATION")]
        expires_in: Option<String>,
        /// Pin a local report file to IPFS and use that as `reportUri`.
        #[arg(long)]
        pin_report_to_ipfs: bool,
        /// Override report hash (32-byte hex). Use when `--report` is a URL
        /// and you've computed the hash off-line.
        #[arg(long, value_name = "HEX")]
        report_hash: Option<String>,
        #[arg(long)]
        json: bool,
    },

    /// Revoke an attestation you previously submitted.
    Revoke {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        #[arg(long)]
        attestation_id: u64,
        /// Off-chain pointer describing why the attestation was withdrawn.
        #[arg(long, value_name = "URI")]
        reason: String,
        #[arg(long)]
        json: bool,
    },

    /// List all attestations for a binary version.
    List {
        #[command(flatten)]
        network: TangleClientArgs,
        #[arg(long)]
        blueprint_id: u64,
        #[arg(long)]
        version_id: u64,
        #[arg(long)]
        json: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum DelegationSelection {
    /// Delegation applies to all blueprints the operator supports.
    All,
    /// Delegation is pinned to specific blueprint IDs.
    Fixed,
}

#[derive(Subcommand, Debug)]
enum DelegatorCommands {
    /// Show all staking positions for a delegator.
    ///
    /// Displays deposits, locks, active delegations, and pending requests.
    Positions {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Delegator address to query (defaults to your address).
        #[arg(long)]
        delegator: Option<String>,
        /// Token contract address (0x0 for native ETH/TNT).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        token: String,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// List active delegations from a delegator to operators.
    Delegations {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Delegator address to query (defaults to your address).
        #[arg(long)]
        delegator: Option<String>,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// List pending unstake requests waiting for unbonding period.
    PendingUnstakes {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Delegator address to query (defaults to your address).
        #[arg(long)]
        delegator: Option<String>,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// List pending withdrawal requests waiting for unbonding period.
    PendingWithdrawals {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Delegator address to query (defaults to your address).
        #[arg(long)]
        delegator: Option<String>,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// Check ERC20 token allowance for the staking contract.
    ///
    /// Shows how many tokens the contract can spend on your behalf.
    Allowance {
        #[command(flatten)]
        network: TangleClientArgs,
        /// ERC20 token contract address.
        #[arg(long)]
        token: String,
        /// Token owner address (defaults to your address).
        #[arg(long)]
        owner: Option<String>,
        /// Spender address (defaults to staking contract).
        #[arg(long)]
        spender: Option<String>,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// Check ERC20 token balance.
    Balance {
        #[command(flatten)]
        network: TangleClientArgs,
        /// ERC20 token contract address.
        #[arg(long)]
        token: String,
        /// Address to check balance for (defaults to your address).
        #[arg(long)]
        owner: Option<String>,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// Approve ERC20 tokens for staking.
    ///
    /// Required before depositing ERC20 tokens. Sets the allowance for
    /// the staking contract to transfer tokens on your behalf.
    Approve {
        #[command(flatten)]
        network: TangleClientArgs,
        /// ERC20 token contract address.
        #[arg(long)]
        token: String,
        /// Amount to approve in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Spender address (defaults to staking contract).
        #[arg(long)]
        spender: Option<String>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Deposit tokens into the staking contract.
    ///
    /// Deposits tokens that can later be delegated to operators.
    /// For ERC20 tokens, you must approve() first.
    Deposit {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Token contract address (0x0 for native ETH/TNT).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        token: String,
        /// Amount to deposit in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Delegate deposited tokens to an operator.
    ///
    /// Assigns your stake to an operator who provides economic security for services.
    /// You earn rewards when the operator participates in services.
    Delegate {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address to delegate stake to.
        #[arg(long)]
        operator: String,
        /// Amount to delegate in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Token contract address (0x0 for native ETH/TNT).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        token: String,
        /// Blueprint selection: all (any blueprint) or fixed (specific blueprints).
        #[arg(long, value_enum, default_value = "all")]
        selection: DelegationSelection,
        /// Blueprint IDs for fixed selection (requires --selection=fixed).
        #[arg(long = "blueprint-id")]
        blueprint_ids: Vec<u64>,
        /// Delegate from existing deposit balance instead of new deposit.
        #[arg(long)]
        from_deposit: bool,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Request to undelegate stake from an operator.
    ///
    /// Initiates the unbonding period. Use execute-unstake after the period ends.
    Undelegate {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address to undelegate from.
        #[arg(long)]
        operator: String,
        /// Amount to undelegate in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Token contract address (0x0 for native ETH/TNT).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        token: String,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Execute all matured unstake requests.
    ///
    /// Completes undelegation for requests past the unbonding period.
    /// Tokens move back to your deposit balance.
    ExecuteUnstake {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Execute a specific unstake and withdraw in one transaction.
    ///
    /// Completes undelegation and immediately withdraws to your wallet.
    ExecuteUnstakeWithdraw {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address from the original delegation.
        #[arg(long)]
        operator: String,
        /// Token contract address (0x0 for native ETH/TNT).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        token: String,
        /// Share amount from your pending unstake.
        #[arg(long)]
        shares: u128,
        /// Round number from the pending unstake request.
        #[arg(long)]
        requested_round: u64,
        /// Recipient address (defaults to your address).
        #[arg(long)]
        receiver: Option<String>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Request to withdraw deposited tokens.
    ///
    /// Initiates the unbonding period for non-delegated deposits.
    /// Use execute-withdraw after the period ends.
    ScheduleWithdraw {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Token contract address (0x0 for native ETH/TNT).
        #[arg(long, default_value = "0x0000000000000000000000000000000000000000")]
        token: String,
        /// Amount to withdraw in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Execute all matured withdrawal requests.
    ///
    /// Transfers tokens back to your wallet for requests past the unbonding period.
    ExecuteWithdraw {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand, Debug)]
enum OperatorCommands {
    /// Show operator heartbeat and status for a service.
    ///
    /// Displays the last heartbeat timestamp, status code, and health information.
    Status {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID to check status for.
        #[arg(long)]
        service_id: u64,
        /// Operator address to query (defaults to your address).
        #[arg(long = "operator")]
        operator: Option<String>,
        /// Output as JSON instead of formatted display.
        #[arg(long)]
        json: bool,
    },
    /// Submit a heartbeat to signal operator liveness.
    ///
    /// Operators should submit heartbeats periodically to avoid being marked inactive.
    /// Status code 0 indicates healthy operation.
    #[command(visible_alias = "hb")]
    Heartbeat {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID to submit heartbeat for.
        #[arg(long)]
        service_id: u64,
        /// Status code: 0 = healthy, non-zero = error code.
        #[arg(long, default_value_t = 0)]
        status_code: u8,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Join a running dynamic service.
    ///
    /// For services with open membership, registers as an operator participant.
    /// Your stake exposure determines slashing risk for this service.
    Join {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID to join.
        #[arg(long)]
        service_id: u64,
        /// Stake exposure in basis points (10000 = 100%).
        #[arg(long, default_value_t = 10_000)]
        exposure_bps: u16,
        /// Asset security commitment in format KIND:TOKEN:EXPOSURE_BPS.
        /// KIND: 0=ERC20, 1=Vault, 2=Native.
        /// TOKEN: Token/vault address (use 0x0 for native).
        /// EXPOSURE_BPS: Exposure in basis points (e.g., 5000 = 50%).
        /// Can be specified multiple times for multiple commitments.
        /// Example: --commitment 0:0x1234...abcd:5000
        #[arg(long, value_name = "KIND:TOKEN:EXPOSURE_BPS")]
        commitment: Vec<String>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Leave a dynamic service.
    ///
    /// Exits service participation. Your committed stake enters the unbonding period.
    Leave {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID to leave.
        #[arg(long)]
        service_id: u64,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Show operator staking status and stake amounts.
    ///
    /// Displays total stake, delegated amounts, and operator status.
    Staking {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address to query (defaults to your address).
        #[arg(long = "operator")]
        operator: Option<String>,
        /// Output as JSON instead of formatted display.
        #[arg(long)]
        json: bool,
    },
    /// List all delegators who have staked with this operator.
    Delegators {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address to query (defaults to your address).
        #[arg(long = "operator")]
        operator: Option<String>,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// Request to unstake operator bond.
    ///
    /// Initiates the unbonding period for operator stake.
    /// Use execute-unstake after the period ends.
    ScheduleUnstake {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Amount to unstake in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Execute matured operator unstake requests.
    ///
    /// Completes unstaking for requests past the unbonding period.
    ExecuteUnstake {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Begin the process of leaving as an operator.
    ///
    /// Starts the exit period. You cannot accept new services while leaving.
    /// Use complete-leaving after the exit period ends.
    StartLeaving {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Complete the operator exit process.
    ///
    /// Finalizes leaving after the exit period. Removes operator status.
    CompleteLeaving {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Register as a new operator on the staking layer.
    ///
    /// Stakes the initial bond and enables operator status.
    /// For ERC20 bond tokens, you must approve() the staking contract first.
    Register {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Initial stake amount in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Add more stake to your operator bond.
    ///
    /// Increases your total stake, improving your capacity for services.
    /// For ERC20 bond tokens, you must approve() the additional amount first.
    IncreaseStake {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Amount to add to stake in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Get operator's delegation mode.
    ///
    /// Shows whether the operator accepts delegations and under what policy.
    GetDelegationMode {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address (defaults to local account).
        #[arg(long = "operator")]
        operator: Option<String>,
        /// Emit JSON instead of human-readable output.
        #[arg(long)]
        json: bool,
    },
    /// Set delegation mode for the operator.
    ///
    /// Controls who can delegate to this operator:
    /// - disabled: Only operator can self-stake (default)
    /// - whitelist: Only approved addresses can delegate
    /// - open: Anyone can delegate
    SetDelegationMode {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Delegation mode: disabled, whitelist, or open.
        #[arg(long, value_enum)]
        mode: DelegationModeArg,
        /// Emit JSON instead of human-readable output.
        #[arg(long)]
        json: bool,
    },
    /// Update delegation whitelist.
    ///
    /// Add or remove addresses from the operator's delegation whitelist.
    /// Only applies when delegation mode is set to "whitelist".
    UpdateWhitelist {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Delegator addresses to update.
        #[arg(long = "delegator", required = true)]
        delegators: Vec<String>,
        /// Whether to approve (true) or revoke (false) the addresses.
        #[arg(long)]
        approved: bool,
        /// Emit JSON instead of human-readable output.
        #[arg(long)]
        json: bool,
    },
    /// Check if delegator can delegate to operator.
    CanDelegate {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Operator address.
        #[arg(long)]
        operator: String,
        /// Delegator address to check.
        #[arg(long)]
        delegator: String,
        /// Emit JSON instead of human-readable output.
        #[arg(long)]
        json: bool,
    },
    /// Schedule an exit from a dynamic service.
    ///
    /// Enters the operator into the exit queue. After the exit queue duration
    /// (default 7 days), use `execute-exit` to complete the exit.
    /// Requires the minimum commitment period to have passed since joining.
    ScheduleExit {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Emit JSON transaction logs.
        #[arg(long)]
        json: bool,
    },
    /// Execute a previously scheduled exit from a service.
    ///
    /// Completes the exit after the exit queue duration has passed.
    /// Must be called after `schedule-exit` and waiting for the queue duration.
    ExecuteExit {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Emit JSON transaction logs.
        #[arg(long)]
        json: bool,
    },
    /// Cancel a previously scheduled exit from a service.
    ///
    /// Cancels the exit and keeps the operator in the service.
    /// Can only be called before `execute-exit`.
    CancelExit {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Service identifier.
        #[arg(long)]
        service_id: u64,
        /// Emit JSON transaction logs.
        #[arg(long)]
        json: bool,
    },
}

/// Delegation mode argument for CLI.
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum DelegationModeArg {
    /// Only operator can self-stake (default).
    Disabled,
    /// Only approved addresses can delegate.
    Whitelist,
    /// Anyone can delegate.
    Open,
}

#[derive(Subcommand, Debug)]
enum DeployTarget {
    /// Deploy to Tangle EVM protocol.
    ///
    /// Registers your blueprint in the Tangle contract registry.
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
                template_variables,
                define,
                template_values_file,
                skip_prompts,
            } => {
                new_blueprint(
                    &name,
                    source,
                    blueprint_type,
                    define,
                    template_variables,
                    &template_values_file,
                    skip_prompts,
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

                let settings_file =
                    settings_file.unwrap_or_else(|| PathBuf::from("./settings.env"));
                let protocol_settings = load_protocol_settings(protocol, &settings_file)?;
                let settings = protocol_settings.tangle().map_err(|e| eyre!("{e}"))?;

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
                    staking_percent,
                    json,
                    security_commitments,
                } => {
                    let client = network.connect(0, None).await?;
                    let tx = if security_commitments.is_empty() {
                        approve_service(&client, request_id, staking_percent).await?
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
                    commitment,
                    json,
                } => {
                    ensure!(exposure_bps > 0, "Exposure must be greater than 0 bps");
                    ensure!(
                        exposure_bps <= MAX_BPS,
                        "Exposure cannot exceed {MAX_BPS} bps"
                    );
                    let client = network.connect(0, Some(service_id)).await?;
                    let tx = if commitment.is_empty() {
                        join_service(&client, service_id, exposure_bps).await?
                    } else {
                        let commitments = parse_commitments(&commitment)?;
                        client
                            .join_service_with_commitments(service_id, exposure_bps, commitments)
                            .await
                            .map_err(|e| eyre!(e.to_string()))?
                    };
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
                    dry_run,
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
                        staking_contract: settings.staking_contract,
                        status_registry_contract: settings.status_registry_contract,
                        keystore_path: network.keystore_path()?.display().to_string(),
                        data_dir,
                        allow_unchecked_attestations,
                        registration_mode: false,
                        registration_capture_only: false,
                        preferred_source: preferred_source_value,
                        use_vm: use_vm_value,
                        dry_run,
                        shutdown_after: None,
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
                ServiceCommands::SetPolicy {
                    network,
                    service_id,
                    policy,
                    json,
                } => {
                    let ctx = upgrade_tx_ctx(&network)?;
                    let tx = cargo_tangle::command::upgrade::set_service_policy(
                        &ctx, service_id, policy,
                    )
                    .await?;
                    cargo_tangle::command::upgrade::print_simple_tx(
                        &format!("set-policy({} -> {})", service_id, policy.as_str()),
                        &tx,
                        json,
                    );
                }
                ServiceCommands::AckVersion {
                    network,
                    service_id,
                    version_id,
                    json,
                } => {
                    let ctx = upgrade_tx_ctx(&network)?;
                    let tx =
                        cargo_tangle::command::upgrade::ack_version(&ctx, service_id, version_id)
                            .await?;
                    cargo_tangle::command::upgrade::print_simple_tx(
                        &format!("ack-version(service={service_id}, version={version_id})"),
                        &tx,
                        json,
                    );
                }
                ServiceCommands::EffectiveVersion {
                    network,
                    service_id,
                    blueprint_id,
                    json,
                } => {
                    let view = upgrade_view_ctx(&network, None)?;
                    let v = cargo_tangle::command::upgrade::get_effective_version(
                        &view,
                        blueprint_id,
                        service_id,
                    )
                    .await?;
                    cargo_tangle::command::upgrade::print_effective_version(service_id, &v, json);
                }
                ServiceCommands::UpgradeStatus {
                    network,
                    service_id,
                    blueprint_id,
                    json,
                } => {
                    let view = upgrade_view_ctx(&network, None)?;
                    let policy =
                        cargo_tangle::command::upgrade::get_service_policy(&view, service_id)
                            .await?;
                    let acked = cargo_tangle::command::upgrade::get_service_acked_version_id(
                        &view, service_id,
                    )
                    .await?;
                    let active =
                        cargo_tangle::command::upgrade::get_active_version_id(&view, blueprint_id)
                            .await?;
                    let effective = cargo_tangle::command::upgrade::get_effective_version(
                        &view,
                        blueprint_id,
                        service_id,
                    )
                    .await?;
                    let versions =
                        cargo_tangle::command::upgrade::list_versions(&view, blueprint_id).await?;
                    let latest = versions
                        .last()
                        .map(|v| v.version_id)
                        .unwrap_or(effective.version_id);
                    let up_to_date = effective.version_id == latest;
                    let status = cargo_tangle::command::upgrade::UpgradeStatus {
                        service_id,
                        policy,
                        acked_version_id: acked,
                        active_version_id: active,
                        effective_version_id: effective.version_id,
                        latest_version_id: latest,
                        up_to_date,
                    };
                    cargo_tangle::command::upgrade::print_upgrade_status(&status, json);
                }
                ServiceCommands::Upgrades {
                    service_id,
                    manager_url,
                    json,
                } => {
                    let manager = cargo_tangle::command::upgrade_local::resolve_manager_url(
                        manager_url.as_ref(),
                    )?;
                    let list =
                        cargo_tangle::command::upgrade_local::list_upgrades(&manager, service_id)
                            .await?;
                    cargo_tangle::command::upgrade_local::print_available(&list, json);
                }
                ServiceCommands::UpgradeLocal {
                    service_id,
                    version_id,
                    dry_run,
                    manager_url,
                    json,
                } => {
                    let manager = cargo_tangle::command::upgrade_local::resolve_manager_url(
                        manager_url.as_ref(),
                    )?;
                    let result = cargo_tangle::command::upgrade_local::pin_version(
                        &manager, service_id, version_id, dry_run,
                    )
                    .await?;
                    cargo_tangle::command::upgrade_local::print_pin_result(
                        service_id, &result, json,
                    );
                }
                ServiceCommands::UpgradeWhitelist {
                    service_id,
                    versions,
                    manager_url,
                    json,
                } => {
                    let manager = cargo_tangle::command::upgrade_local::resolve_manager_url(
                        manager_url.as_ref(),
                    )?;
                    let parsed =
                        cargo_tangle::command::upgrade_local::parse_version_list(&versions)?;
                    let result = cargo_tangle::command::upgrade_local::set_whitelist(
                        &manager, service_id, parsed,
                    )
                    .await?;
                    cargo_tangle::command::upgrade_local::print_whitelist_result(
                        service_id, &result, json,
                    );
                }
                ServiceCommands::UpgradeSkip {
                    service_id,
                    version_id,
                    reason,
                    manager_url,
                    json,
                } => {
                    let manager = cargo_tangle::command::upgrade_local::resolve_manager_url(
                        manager_url.as_ref(),
                    )?;
                    let result = cargo_tangle::command::upgrade_local::add_skip(
                        &manager, service_id, version_id, reason,
                    )
                    .await?;
                    cargo_tangle::command::upgrade_local::print_skip_result(
                        service_id, &result, json,
                    );
                }
                ServiceCommands::UpgradeAuthz {
                    service_id,
                    manager_url,
                    json,
                } => {
                    let manager = cargo_tangle::command::upgrade_local::resolve_manager_url(
                        manager_url.as_ref(),
                    )?;
                    let view =
                        cargo_tangle::command::upgrade_local::show_authz(&manager, service_id)
                            .await?;
                    cargo_tangle::command::upgrade_local::print_authz(&view, json);
                }
            },
            BlueprintCommands::Ship {
                network,
                yes,
                no_build,
                package,
                binary,
                binary_uri,
                pin_ipfs,
                attestation_bundle,
                attestation_hash,
                promote,
                no_promote,
                policy_services,
                dry_run,
                blueprint_id,
                json,
            } => {
                // CI mode (--yes) defaults to JSON output so action logs stay
                // grep-able. Explicit --json (or its absence) still wins.
                let json_out = json || yes;
                let args = cargo_tangle::command::ship::ShipArgs {
                    network,
                    yes,
                    no_build,
                    package,
                    binary,
                    binary_uri,
                    pin_ipfs,
                    attestation_bundle,
                    attestation_hash,
                    promote,
                    no_promote,
                    policy_services,
                    dry_run,
                    blueprint_id,
                    json: json_out,
                };
                cargo_tangle::command::ship::run(args).await?;
            }
            BlueprintCommands::PublishVersion {
                network,
                blueprint_id,
                binary,
                binary_uri,
                pin_to_ipfs,
                attestation_bundle,
                attestation_hash,
                json,
            } => {
                let ctx = upgrade_tx_ctx(&network)?;
                let (sha256_hash, _len) = cargo_tangle::command::upgrade::hash_file(&binary)?;
                let resolved_uri = if let Some(uri) = binary_uri {
                    uri
                } else if pin_to_ipfs {
                    let pinned = cargo_tangle::command::upgrade::pin_file_to_ipfs(&binary).await?;
                    pinned.uri
                } else {
                    bail!(
                        "must supply --binary-uri or --pin-to-ipfs so the contract has a binaryUri to publish"
                    );
                };
                let resolved_attestation = if let Some(hex) = attestation_hash {
                    cargo_tangle::command::upgrade::parse_b256(&hex, "--attestation-hash")?
                } else if let Some(bundle_path) = attestation_bundle {
                    cargo_tangle::command::upgrade::hash_file(&bundle_path)?.0
                } else {
                    alloy_primitives::B256::ZERO
                };
                let result = cargo_tangle::command::upgrade::publish_version(
                    &ctx,
                    blueprint_id,
                    sha256_hash,
                    resolved_uri,
                    resolved_attestation,
                )
                .await?;
                cargo_tangle::command::upgrade::print_publish_result(&result, json);
            }
            BlueprintCommands::SetActiveVersion {
                network,
                blueprint_id,
                version_id,
                json,
            } => {
                let ctx = upgrade_tx_ctx(&network)?;
                let tx = cargo_tangle::command::upgrade::set_active_version(
                    &ctx,
                    blueprint_id,
                    version_id,
                )
                .await?;
                cargo_tangle::command::upgrade::print_simple_tx(
                    &format!("set-active-version(blueprint={blueprint_id}, version={version_id})"),
                    &tx,
                    json,
                );
            }
            BlueprintCommands::DeprecateVersion {
                network,
                blueprint_id,
                version_id,
                json,
            } => {
                let ctx = upgrade_tx_ctx(&network)?;
                let tx = cargo_tangle::command::upgrade::deprecate_version(
                    &ctx,
                    blueprint_id,
                    version_id,
                )
                .await?;
                cargo_tangle::command::upgrade::print_simple_tx(
                    &format!("deprecate-version(blueprint={blueprint_id}, version={version_id})"),
                    &tx,
                    json,
                );
            }
            BlueprintCommands::ListVersions {
                network,
                blueprint_id,
                json,
            } => {
                let view = upgrade_view_ctx(&network, None)?;
                let versions =
                    cargo_tangle::command::upgrade::list_versions(&view, blueprint_id).await?;
                let active =
                    cargo_tangle::command::upgrade::get_active_version_id(&view, blueprint_id)
                        .await?;
                cargo_tangle::command::upgrade::print_versions_table(
                    blueprint_id,
                    &versions,
                    active,
                    json,
                );
            }
            BlueprintCommands::ShowVersion {
                network,
                blueprint_id,
                version_id,
                json,
            } => {
                let view = upgrade_view_ctx(&network, None)?;
                let v =
                    cargo_tangle::command::upgrade::get_version(&view, blueprint_id, version_id)
                        .await?;
                cargo_tangle::command::upgrade::print_version_detail(blueprint_id, &v, json);
            }
            BlueprintCommands::TrustScore {
                network,
                blueprint_id,
                version_id,
                auditors_contract,
                min_score,
                json,
            } => {
                let auditors = match auditors_contract {
                    Some(s) => Some(parse_address(&s, "AUDITORS_CONTRACT")?),
                    None => None,
                };
                let view = upgrade_view_ctx(&network, auditors)?;
                let score = cargo_tangle::command::upgrade::compute_trust_score(
                    &view,
                    blueprint_id,
                    version_id,
                )
                .await?;
                cargo_tangle::command::upgrade::print_trust_score(&score, json);
                if let Some(min) = min_score {
                    if score.score < min {
                        return Err(eyre!(
                            "trust score {} < required {} for blueprint {} version {}",
                            score.score,
                            min,
                            blueprint_id,
                            version_id
                        ));
                    }
                }
            }
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
        Commands::Delegator { command } => match command {
            DelegatorCommands::Positions {
                network,
                delegator,
                token,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let delegator_address = if let Some(value) = delegator {
                    parse_address(&value, "DELEGATOR")?
                } else {
                    client.account()
                };
                let token_address = parse_address(&token, "TOKEN")?;
                let deposit = client
                    .get_deposit_info(delegator_address, token_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let locks = client
                    .get_locks(delegator_address, token_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let delegations = client
                    .get_delegations_with_blueprints(delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let unstakes = client
                    .get_pending_unstakes(delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let withdrawals = client
                    .get_pending_withdrawals(delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_positions(
                    delegator_address,
                    token_address,
                    &deposit,
                    &locks,
                    &delegations,
                    &unstakes,
                    &withdrawals,
                    json,
                );
            }
            DelegatorCommands::Delegations {
                network,
                delegator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let delegator_address = if let Some(value) = delegator {
                    parse_address(&value, "DELEGATOR")?
                } else {
                    client.account()
                };
                let delegations = client
                    .get_delegations_with_blueprints(delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_delegations(delegator_address, &delegations, json);
            }
            DelegatorCommands::PendingUnstakes {
                network,
                delegator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let delegator_address = if let Some(value) = delegator {
                    parse_address(&value, "DELEGATOR")?
                } else {
                    client.account()
                };
                let unstakes = client
                    .get_pending_unstakes(delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_pending_unstakes(delegator_address, &unstakes, json);
            }
            DelegatorCommands::PendingWithdrawals {
                network,
                delegator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let delegator_address = if let Some(value) = delegator {
                    parse_address(&value, "DELEGATOR")?
                } else {
                    client.account()
                };
                let withdrawals = client
                    .get_pending_withdrawals(delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_pending_withdrawals(delegator_address, &withdrawals, json);
            }
            DelegatorCommands::Allowance {
                network,
                token,
                owner,
                spender,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let token_address = parse_address(&token, "TOKEN")?;
                ensure!(
                    token_address != Address::ZERO,
                    "Token address must be non-zero for ERC20 allowance"
                );
                let owner_address = if let Some(value) = owner {
                    parse_address(&value, "OWNER")?
                } else {
                    client.account()
                };
                let spender_address = if let Some(value) = spender {
                    parse_address(&value, "SPENDER")?
                } else {
                    client.config.settings.staking_contract
                };
                let allowance = client
                    .erc20_allowance(token_address, owner_address, spender_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_erc20_allowance(
                    owner_address,
                    spender_address,
                    token_address,
                    allowance,
                    json,
                );
            }
            DelegatorCommands::Balance {
                network,
                token,
                owner,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let token_address = parse_address(&token, "TOKEN")?;
                ensure!(
                    token_address != Address::ZERO,
                    "Token address must be non-zero for ERC20 balance"
                );
                let owner_address = if let Some(value) = owner {
                    parse_address(&value, "OWNER")?
                } else {
                    client.account()
                };
                let balance = client
                    .erc20_balance(token_address, owner_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_erc20_balance(owner_address, token_address, balance, json);
            }
            DelegatorCommands::Approve {
                network,
                token,
                amount,
                spender,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let token_address = parse_address(&token, "TOKEN")?;
                ensure!(
                    token_address != Address::ZERO,
                    "Token address must be non-zero for ERC20 approvals"
                );
                let spender_address = if let Some(value) = spender {
                    parse_address(&value, "SPENDER")?
                } else {
                    client.config.settings.staking_contract
                };
                let tx = client
                    .erc20_approve(token_address, spender_address, U256::from(amount))
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator approve", &tx, json);
            }
            DelegatorCommands::Deposit {
                network,
                token,
                amount,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let token_address = parse_address(&token, "TOKEN")?;
                let tx = if token_address == Address::ZERO {
                    client
                        .deposit_native(U256::from(amount))
                        .await
                        .map_err(|e| eyre!(e.to_string()))?
                } else {
                    client
                        .deposit_erc20(token_address, U256::from(amount))
                        .await
                        .map_err(|e| eyre!(e.to_string()))?
                };
                log_tx("Delegator deposit", &tx, json);
            }
            DelegatorCommands::Delegate {
                network,
                operator,
                amount,
                token,
                selection,
                blueprint_ids,
                from_deposit,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = parse_address(&operator, "OPERATOR")?;
                let token_address = parse_address(&token, "TOKEN")?;
                let selection_mode = match selection {
                    DelegationSelection::All => BlueprintSelectionMode::All,
                    DelegationSelection::Fixed => BlueprintSelectionMode::Fixed,
                };
                if matches!(selection_mode, BlueprintSelectionMode::Fixed)
                    && blueprint_ids.is_empty()
                {
                    return Err(eyre!(
                        "Fixed selection requires at least one --blueprint-id"
                    ));
                }
                let tx = if from_deposit {
                    client
                        .delegate_with_options(
                            operator_address,
                            token_address,
                            U256::from(amount),
                            selection_mode,
                            blueprint_ids,
                        )
                        .await
                } else {
                    client
                        .deposit_and_delegate_with_options(
                            operator_address,
                            token_address,
                            U256::from(amount),
                            selection_mode,
                            blueprint_ids,
                        )
                        .await
                }
                .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator delegate", &tx, json);
            }
            DelegatorCommands::Undelegate {
                network,
                operator,
                amount,
                token,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = parse_address(&operator, "OPERATOR")?;
                let token_address = parse_address(&token, "TOKEN")?;
                let tx = client
                    .schedule_delegator_unstake(operator_address, token_address, U256::from(amount))
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator undelegate", &tx, json);
            }
            DelegatorCommands::ExecuteUnstake { network, json } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .execute_delegator_unstake()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator execute-unstake", &tx, json);
            }
            DelegatorCommands::ExecuteUnstakeWithdraw {
                network,
                operator,
                token,
                shares,
                requested_round,
                receiver,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = parse_address(&operator, "OPERATOR")?;
                let token_address = parse_address(&token, "TOKEN")?;
                let receiver = if let Some(value) = receiver {
                    parse_address(&value, "RECEIVER")?
                } else {
                    client.account()
                };
                let tx = client
                    .execute_delegator_unstake_and_withdraw(
                        operator_address,
                        token_address,
                        U256::from(shares),
                        requested_round,
                        receiver,
                    )
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator execute-unstake-withdraw", &tx, json);
            }
            DelegatorCommands::ScheduleWithdraw {
                network,
                token,
                amount,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let token_address = parse_address(&token, "TOKEN")?;
                let tx = client
                    .schedule_withdraw(token_address, U256::from(amount))
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator schedule-withdraw", &tx, json);
            }
            DelegatorCommands::ExecuteWithdraw { network, json } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .execute_withdraw()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Delegator execute-withdraw", &tx, json);
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
                let keystore =
                    cargo_tangle::command::signer::load_keystore(network.keystore_path()?)?;
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
                commitment,
                json,
            } => {
                let client = network.connect(blueprint_id, Some(service_id)).await?;
                let tx = if commitment.is_empty() {
                    client
                        .join_service(service_id, exposure_bps)
                        .await
                        .map_err(|e| eyre!(e.to_string()))?
                } else {
                    let commitments = parse_commitments(&commitment)?;
                    client
                        .join_service_with_commitments(service_id, exposure_bps, commitments)
                        .await
                        .map_err(|e| eyre!(e.to_string()))?
                };
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
            OperatorCommands::Staking {
                network,
                operator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = if let Some(value) = operator {
                    parse_address(&value, "OPERATOR")?
                } else {
                    client.account()
                };
                let is_registered = client
                    .is_operator(operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let staking = client
                    .get_restaking_metadata(operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let self_stake = client
                    .get_operator_self_stake(operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let delegated_stake = client
                    .get_operator_delegated_stake(operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let commission_bps = client
                    .operator_commission_bps()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                let current_round = client
                    .restaking_round()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_operator_staking(
                    operator_address,
                    &staking,
                    is_registered,
                    self_stake,
                    delegated_stake,
                    commission_bps,
                    current_round,
                    json,
                );
            }
            OperatorCommands::Delegators {
                network,
                operator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = if let Some(value) = operator {
                    parse_address(&value, "OPERATOR")?
                } else {
                    client.account()
                };
                let delegators = client
                    .get_operator_delegators(operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                delegator::print_operator_delegators(operator_address, &delegators, json);
            }
            OperatorCommands::ScheduleUnstake {
                network,
                amount,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .schedule_operator_unstake(U256::from(amount))
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator schedule-unstake", &tx, json);
            }
            OperatorCommands::ExecuteUnstake { network, json } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .execute_operator_unstake()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator execute-unstake", &tx, json);
            }
            OperatorCommands::StartLeaving { network, json } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .start_leaving()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator start-leaving", &tx, json);
            }
            OperatorCommands::CompleteLeaving { network, json } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .complete_leaving()
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator complete-leaving", &tx, json);
            }
            OperatorCommands::Register {
                network,
                amount,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .register_operator_restaking(U256::from(amount))
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator register", &tx, json);
            }
            OperatorCommands::IncreaseStake {
                network,
                amount,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .increase_stake(U256::from(amount))
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator increase-stake", &tx, json);
            }
            OperatorCommands::GetDelegationMode {
                network,
                operator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = if let Some(value) = operator {
                    parse_address(&value, "OPERATOR")?
                } else {
                    client.account()
                };
                let mode = client
                    .get_delegation_mode(operator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                if json {
                    println!(
                        "{}",
                        serde_json::to_string(&serde_json::json!({
                            "operator": format!("{operator_address:?}"),
                            "delegation_mode": format!("{mode}")
                        }))?
                    );
                } else {
                    println!("Operator: {operator_address:?}");
                    println!("Delegation Mode: {mode}");
                }
            }
            OperatorCommands::SetDelegationMode {
                network,
                mode,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let delegation_mode = match mode {
                    DelegationModeArg::Disabled => DelegationMode::Disabled,
                    DelegationModeArg::Whitelist => DelegationMode::Whitelist,
                    DelegationModeArg::Open => DelegationMode::Open,
                };
                let tx = client
                    .set_delegation_mode(delegation_mode)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator set-delegation-mode", &tx, json);
            }
            OperatorCommands::UpdateWhitelist {
                network,
                delegators,
                approved,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let delegator_addresses = parse_address_list(&delegators, "DELEGATOR")?;
                let tx = client
                    .set_delegation_whitelist(delegator_addresses, approved)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator update-whitelist", &tx, json);
            }
            OperatorCommands::CanDelegate {
                network,
                operator,
                delegator,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let operator_address = parse_address(&operator, "OPERATOR")?;
                let delegator_address = parse_address(&delegator, "DELEGATOR")?;
                let can_delegate = client
                    .can_delegate(operator_address, delegator_address)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                if json {
                    println!(
                        "{}",
                        serde_json::to_string(&serde_json::json!({
                            "operator": format!("{operator_address:?}"),
                            "delegator": format!("{delegator_address:?}"),
                            "can_delegate": can_delegate
                        }))?
                    );
                } else {
                    println!("Operator: {operator_address:?}");
                    println!("Delegator: {delegator_address:?}");
                    println!("Can Delegate: {can_delegate}");
                }
            }
            OperatorCommands::ScheduleExit {
                network,
                service_id,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .schedule_exit(service_id)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator schedule-exit", &tx, json);
            }
            OperatorCommands::ExecuteExit {
                network,
                service_id,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .execute_exit(service_id)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator execute-exit", &tx, json);
            }
            OperatorCommands::CancelExit {
                network,
                service_id,
                json,
            } => {
                let client = network.connect(0, None).await?;
                let tx = client
                    .cancel_exit(service_id)
                    .await
                    .map_err(|e| eyre!(e.to_string()))?;
                log_tx("Operator cancel-exit", &tx, json);
            }
        },
        Commands::Dev { command } => match command {
            DevCommands::Up(args) => dev::up::execute(args).await?,
            DevCommands::Down(args) => dev::down::execute(args)?,
            DevCommands::Status => dev::status::execute()?,
        },
        Commands::Harness { command } => {
            cargo_tangle::command::harness::execute(command).await?;
        }
        Commands::Attest { command } => match command {
            AttestCommands::Submit {
                network,
                blueprint_id,
                version_id,
                report,
                kind,
                severity,
                expires_in,
                pin_report_to_ipfs,
                report_hash,
                json,
            } => {
                let ctx = upgrade_tx_ctx(&network)?;
                let (resolved_hash, resolved_uri) =
                    resolve_report_inputs(&report, report_hash.as_deref(), pin_report_to_ipfs)
                        .await?;
                let expires_at = cargo_tangle::command::upgrade::duration_to_expiry_timestamp(
                    expires_in.as_deref(),
                )?;
                let result = cargo_tangle::command::upgrade::attest_version(
                    &ctx,
                    blueprint_id,
                    version_id,
                    resolved_hash,
                    resolved_uri,
                    kind,
                    severity,
                    expires_at,
                )
                .await?;
                cargo_tangle::command::upgrade::print_attest_result(&result, json);
            }
            AttestCommands::Revoke {
                network,
                blueprint_id,
                version_id,
                attestation_id,
                reason,
                json,
            } => {
                let ctx = upgrade_tx_ctx(&network)?;
                let tx = cargo_tangle::command::upgrade::revoke_attestation(
                    &ctx,
                    blueprint_id,
                    version_id,
                    attestation_id,
                    reason,
                )
                .await?;
                cargo_tangle::command::upgrade::print_simple_tx(
                    &format!(
                        "revoke-attestation(bp={blueprint_id}, ver={version_id}, att={attestation_id})"
                    ),
                    &tx,
                    json,
                );
            }
            AttestCommands::List {
                network,
                blueprint_id,
                version_id,
                json,
            } => {
                let view = upgrade_view_ctx(&network, None)?;
                let rows = cargo_tangle::command::upgrade::list_attestations(
                    &view,
                    blueprint_id,
                    version_id,
                )
                .await?;
                cargo_tangle::command::upgrade::print_attestations(
                    blueprint_id,
                    version_id,
                    &rows,
                    json,
                );
            }
        },
    }

    Ok(())
}

/// Build a `TxContext` from the shared `TangleClientArgs` resolver.
fn upgrade_tx_ctx(
    network: &cargo_tangle::command::tangle::TangleClientArgs,
) -> Result<cargo_tangle::command::upgrade::TxContext> {
    let cfg = network.client_config(0, None)?;
    Ok(cargo_tangle::command::upgrade::TxContext {
        http_rpc_url: cfg.http_rpc_endpoint.clone(),
        tangle_contract: cfg.settings.tangle_contract,
        keystore_path: PathBuf::from(cfg.keystore_uri.clone()),
    })
}

fn upgrade_view_ctx(
    network: &cargo_tangle::command::tangle::TangleClientArgs,
    auditors_contract: Option<Address>,
) -> Result<cargo_tangle::command::upgrade::ViewContext> {
    let cfg = network.client_config(0, None)?;
    Ok(cargo_tangle::command::upgrade::ViewContext {
        http_rpc_url: cfg.http_rpc_endpoint.clone(),
        tangle_contract: cfg.settings.tangle_contract,
        auditors_contract,
    })
}

async fn resolve_report_inputs(
    report: &str,
    report_hash_override: Option<&str>,
    pin_report_to_ipfs: bool,
) -> Result<(alloy_primitives::B256, String)> {
    let path = std::path::Path::new(report);
    if path.exists() {
        let (digest, _) = cargo_tangle::command::upgrade::hash_file(path)?;
        let uri = if pin_report_to_ipfs {
            cargo_tangle::command::upgrade::pin_file_to_ipfs(path)
                .await?
                .uri
        } else {
            // Local-only artifact: emit a deterministic file:// URI as a fallback so the
            // on-chain `reportUri` invariant (non-empty) is preserved.
            format!("file://{}", path.display())
        };
        return Ok((digest, uri));
    }
    // Treat as URL/URI string.
    let hash = match report_hash_override {
        Some(hex) => cargo_tangle::command::upgrade::parse_b256(hex, "--report-hash")?,
        None => alloy_primitives::B256::ZERO,
    };
    Ok((hash, report.to_string()))
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

/// Parse a list of commitment strings into ABI-compatible commitment structures.
fn parse_commitments(commitments: &[String]) -> Result<Vec<ITangleTypes::AssetSecurityCommitment>> {
    commitments
        .iter()
        .map(|s| {
            parse_security_commitment(s)
                .map(commitment_to_abi)
                .map_err(|e| eyre!("Invalid commitment '{}': {}", s, e))
        })
        .collect()
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
    client: &TangleClient,
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
            import_key(Protocol::Tangle, kind, &secret, path)?;
        }
    }

    Ok(())
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
    /// List all jobs defined by a blueprint.
    ///
    /// Shows job indices, names, input schemas, and output types.
    List {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID to list jobs for.
        #[arg(long)]
        blueprint_id: u64,
        /// Output as JSON instead of formatted table.
        #[arg(long)]
        json: bool,
    },
    /// Show details for a submitted job call.
    ///
    /// Displays inputs, outputs, status, and result data.
    Show {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID the job was submitted to.
        #[arg(long)]
        service_id: u64,
        /// Call ID returned when the job was submitted.
        #[arg(long)]
        call_id: u64,
        /// Output as JSON instead of formatted display.
        #[arg(long)]
        json: bool,
    },
    /// Submit a job to a running service.
    ///
    /// Invokes a job on the service operators. Inputs can be provided as:
    /// - Raw hex bytes (--payload-hex)
    /// - Binary file (--payload-file)
    /// - Structured JSON matching the job schema (--params-file)
    /// - Interactive prompts (--prompt)
    Submit {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID to submit the job to.
        #[arg(long)]
        service_id: u64,
        /// Job index (0-based) as defined in the blueprint.
        #[arg(long)]
        job: u8,
        /// Job inputs as hex-encoded bytes (without 0x prefix).
        #[arg(long = "payload-hex", value_name = "HEX")]
        payload_hex: Option<String>,
        /// File containing raw job input bytes.
        #[arg(long = "payload-file", value_name = "FILE")]
        payload_file: Option<PathBuf>,
        /// JSON file with structured inputs matching the job schema.
        #[arg(
            long = "params-file",
            value_name = "FILE",
            conflicts_with_all = ["payload_hex", "payload_file"]
        )]
        params_file: Option<PathBuf>,
        /// Interactively prompt for each job input.
        #[arg(
            long,
            conflicts_with_all = ["payload_hex", "payload_file", "params_file"],
            action = clap::ArgAction::SetTrue
        )]
        prompt: bool,
        /// Wait for job result after submission.
        #[arg(long)]
        watch: bool,
        /// Timeout in seconds when watching for result.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Wait for a job result by call ID.
    ///
    /// Polls for job completion and displays the result when available.
    Watch {
        #[command(flatten)]
        network: TangleClientArgs,
        /// Blueprint ID the service belongs to.
        #[arg(long)]
        blueprint_id: u64,
        /// Service ID the job was submitted to.
        #[arg(long)]
        service_id: u64,
        /// Call ID from the job submission.
        #[arg(long)]
        call_id: u64,
        /// Timeout in seconds before giving up.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
}
