mod attest;
mod blueprint;
mod blueprint_versions;
mod chain;
mod delegator;
mod jobs;
mod keys;
mod operator;
mod service;

use self::{
    attest::AttestCommands, blueprint::BlueprintCommands, delegator::DelegatorCommands,
    keys::KeyCommands, operator::OperatorCommands,
};
use cargo_tangle::command::dev::{self, DevCommands};
use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

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

pub(crate) async fn run() -> Result<()> {
    let args = collect_args();
    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Blueprint { command } => blueprint::run(command).await?,
        Commands::Key { command } => keys::run(command).await?,
        Commands::Delegator { command } => delegator::run(command).await?,
        #[cfg(feature = "remote-providers")]
        Commands::Cloud { command } => {
            cargo_tangle::command::cloud::execute(command).await?;
        }
        Commands::Operator { command } => operator::run(command).await?,
        Commands::Dev { command } => match command {
            DevCommands::Up(args) => dev::up::execute(args).await?,
            DevCommands::Down(args) => dev::down::execute(args)?,
            DevCommands::Status => dev::status::execute()?,
        },
        Commands::Harness { command } => {
            cargo_tangle::command::harness::execute(command).await?;
        }
        Commands::Attest { command } => attest::run(command).await?,
    }
    Ok(())
}

fn collect_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|x| x == "tangle") {
        std::env::args().skip(1).collect()
    } else {
        args
    }
}

pub(crate) fn init_tracing_subscriber() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::prelude::*;
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .pretty();

    tracing_subscriber::registry()
        .with(cargo_tangle::telemetry::cli_env_filter())
        .with(fmt_layer)
        .init();
}
