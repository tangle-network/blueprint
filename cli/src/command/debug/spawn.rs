use crate::command::run::tangle::run_blueprint;
use crate::command::tangle::{DevnetStack, SpawnMethod, run_opts_from_stack};
use crate::settings::load_protocol_settings;
use blueprint_runner::config::Protocol;
use clap::Args;
use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct SpawnArgs {
    /// Optional settings file to seed blueprint configuration.
    #[arg(long, value_name = "FILE", default_value = "./settings.env")]
    pub settings_file: PathBuf,
    /// Stream Anvil stdout/stderr for debugging.
    #[arg(long)]
    pub include_anvil_logs: bool,
    /// Allow unchecked attestations when running the manager.
    #[arg(long)]
    pub allow_unchecked_attestations: bool,
    /// Preferred runtime for the spawned service.
    #[arg(long, value_enum, default_value_t = SpawnMethod::Vm)]
    pub spawn_method: SpawnMethod,
}

pub async fn execute(args: SpawnArgs) -> Result<()> {
    let settings = load_protocol_settings(Protocol::TangleEvm, &args.settings_file)
        .map_err(|e| eyre!(e.to_string()))?;
    let tangle_settings = settings
        .tangle_evm()
        .map_err(|e| eyre!("failed to load Tangle settings: {e}"))?;

    let stack = DevnetStack::spawn(args.include_anvil_logs).await?;

    println!(
        "Anvil testnet ready at HTTP {} / WS {}",
        stack.http_rpc_url(),
        stack.ws_rpc_url()
    );
    println!("Press Ctrl+C to stop the blueprint and tear down the stack.");

    let run_opts = run_opts_from_stack(
        &stack,
        tangle_settings,
        args.allow_unchecked_attestations,
        args.spawn_method,
    );

    // Run the blueprint manager until the user terminates the process.
    run_blueprint(run_opts).await?;

    stack.shutdown().await;
    Ok(())
}
