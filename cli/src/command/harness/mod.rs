use clap::{Args, Subcommand};
use color_eyre::eyre::Result;
use std::path::PathBuf;

pub mod commands;
pub mod config;
pub mod orchestrator;

#[derive(Subcommand, Debug)]
pub enum HarnessCommands {
    /// Start the local Tangle dev environment with all configured blueprints.
    /// Each blueprint repo ships its own harness.toml. Use --compose to run multiple.
    Up(UpArgs),
    /// Stop a running harness (reads pid file).
    Down,
    /// Show status of a running harness.
    Status,
    /// Tail logs from all blueprints.
    Logs(LogsArgs),
    /// Run E2E tests defined in each blueprint's harness.toml `test_command`.
    /// Boots the harness, runs tests, reports results, shuts down.
    Test(TestArgs),
    /// List all discoverable harness.toml files across known blueprint directories.
    List,
}

/// Chain target flags — shared by `up` and `test`. CLI flags override harness.toml.
#[derive(Args, Debug, Clone, Default)]
pub struct ChainArgs {
    /// Use local Anvil (default). Pass --no-anvil to connect to a remote chain.
    #[arg(long)]
    pub no_anvil: bool,
    /// HTTP RPC URL (overrides harness.toml chain.rpc_url).
    #[arg(long)]
    pub rpc_url: Option<String>,
    /// WebSocket RPC URL (overrides harness.toml chain.ws_url).
    #[arg(long)]
    pub ws_url: Option<String>,
    /// Chain ID (overrides harness.toml chain.chain_id).
    #[arg(long)]
    pub chain_id: Option<u64>,
    /// Tangle contract address on the target chain.
    #[arg(long)]
    pub tangle_contract: Option<String>,
    /// Path to operator keystore.
    #[arg(long)]
    pub keystore_path: Option<String>,
    /// Router URL to register operators with.
    #[arg(long)]
    pub router_url: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpArgs {
    /// Path to harness config TOML (default: ./harness.toml or ~/.tangle/harness.toml).
    #[arg(long, short = 'c')]
    pub config: Option<PathBuf>,
    /// Only start these blueprints (comma-separated names).
    #[arg(long)]
    pub only: Option<String>,
    /// Stream anvil logs.
    #[arg(long)]
    pub include_anvil_logs: bool,
    /// Compose multiple per-repo harnesses onto a shared chain.
    /// Comma-separated paths to blueprint directories, each containing harness.toml.
    #[arg(long)]
    pub compose: Option<String>,
    /// Chain target overrides.
    #[command(flatten)]
    pub chain: ChainArgs,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    /// Path to harness config TOML.
    #[arg(long, short = 'c')]
    pub config: Option<PathBuf>,
    /// Only test these blueprints (comma-separated names).
    #[arg(long)]
    pub only: Option<String>,
    /// Compose multiple per-repo harnesses (same as `up --compose`).
    #[arg(long)]
    pub compose: Option<String>,
    /// Chain target overrides.
    #[command(flatten)]
    pub chain: ChainArgs,
}

#[derive(Args, Debug)]
pub struct LogsArgs {
    /// Follow logs (tail -f).
    #[arg(long, short = 'f')]
    pub follow: bool,
    /// Only show logs for these blueprints.
    #[arg(long)]
    pub only: Option<String>,
}

pub async fn execute(command: HarnessCommands) -> Result<()> {
    match command {
        HarnessCommands::Up(args) => commands::up::run(args).await,
        HarnessCommands::Down => commands::down::run().await,
        HarnessCommands::Status => commands::status::run().await,
        HarnessCommands::Logs(args) => commands::logs::run(args).await,
        HarnessCommands::Test(args) => commands::test::run(args).await,
        HarnessCommands::List => commands::list::run().await,
    }
}
