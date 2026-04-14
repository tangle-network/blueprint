use clap::{Args, Subcommand};
use color_eyre::eyre::Result;
use std::path::PathBuf;

pub mod commands;
pub mod config;
pub mod orchestrator;

#[derive(Subcommand, Debug)]
pub enum HarnessCommands {
    /// Start the local Tangle dev environment with all configured blueprints.
    Up(UpArgs),
    /// Stop a running harness (reads pid file).
    Down,
    /// Show status of a running harness.
    Status,
    /// Tail logs from all blueprints.
    Logs(LogsArgs),
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
    }
}
