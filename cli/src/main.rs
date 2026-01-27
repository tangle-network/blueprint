use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use alloy_primitives::{Address, Bytes, U256};
use blueprint_client_tangle_evm::{
    BlueprintSelectionMode, DelegationMode, TangleEvmClient, TransactionResult,
    contracts::ITangleTypes,
};
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_keystore::{Keystore, KeystoreConfig, backends::Backend};
use blueprint_manager::config::SourceType;
use blueprint_runner::config::{BlueprintEnvironment, Protocol, SupportedChains};
use blueprint_runner::error::ConfigError;
use cargo_tangle::command::create::{BlueprintType, TemplateVariables, new_blueprint};
use cargo_tangle::command::debug::{self, DebugCommands};
use cargo_tangle::command::delegator;
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

    /// Deploy a blueprint to a protocol (Tangle or Eigenlayer).
    ///
    /// Compiles and publishes your blueprint to the on-chain registry.
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
        /// Target protocol: tangle-evm or eigenlayer.
        #[arg(short = 'p', long, value_enum, default_value = "tangle-evm")]
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
        /// Target protocol: tangle-evm or eigenlayer.
        #[arg(short = 'p', long, value_enum, default_value = "tangle-evm")]
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
        #[arg(short = 'p', long, value_enum, default_value = "tangle-evm")]
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
        #[arg(long, default_value_t = 50)]
        restaking_percent: u8,
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
    /// Check ERC20 token allowance for the restaking contract.
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
        /// Spender address (defaults to restaking contract).
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
    /// Approve ERC20 tokens for restaking.
    ///
    /// Required before depositing ERC20 tokens. Sets the allowance for
    /// the restaking contract to transfer tokens on your behalf.
    Approve {
        #[command(flatten)]
        network: TangleClientArgs,
        /// ERC20 token contract address.
        #[arg(long)]
        token: String,
        /// Amount to approve in wei (smallest token unit).
        #[arg(long)]
        amount: u128,
        /// Spender address (defaults to restaking contract).
        #[arg(long)]
        spender: Option<String>,
        /// Output transaction details as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Deposit tokens into the restaking contract.
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
    /// Show operator restaking status and stake amounts.
    ///
    /// Displays total stake, delegated amounts, and operator status.
    Restaking {
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
    /// Register as a new operator on the restaking layer.
    ///
    /// Stakes the initial bond and enables operator status.
    /// For ERC20 bond tokens, you must approve() the restaking contract first.
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
    /// Deploy to Eigenlayer AVS registry.
    ///
    /// Registers your blueprint as an Eigenlayer AVS (Actively Validated Service).
    Eigenlayer {
        /// RPC endpoint URL (required unless --devnet is set).
        #[arg(long, value_name = "URL", env, required_unless_present = "devnet")]
        rpc_url: Option<String>,
        /// Path to compiled contract artifacts.
        #[arg(long)]
        contracts_path: Option<String>,
        /// Deploy contracts in dependency order.
        #[arg(long)]
        ordered_deployment: bool,
        /// Network name: local, testnet, or mainnet.
        #[arg(short = 'w', long, default_value = "local")]
        network: String,
        /// Use built-in devnet configuration.
        #[arg(long)]
        devnet: bool,
        /// Path to keystore directory containing operator keys.
        #[arg(short = 'k', long)]
        keystore_path: Option<PathBuf>,
    },
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
                        restaking_contract: settings.restaking_contract,
                        status_registry_contract: settings.status_registry_contract,
                        keystore_path: network.keystore_path().display().to_string(),
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
                    client.config.settings.restaking_contract
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
                    client.config.settings.restaking_contract
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
                    cargo_tangle::command::signer::load_keystore(network.keystore_path())?;
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
            OperatorCommands::Restaking {
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
                let restaking = client
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
                delegator::print_operator_restaking(
                    operator_address,
                    &restaking,
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
