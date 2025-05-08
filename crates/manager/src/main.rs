#![cfg(not(unix))]
compile_error!("The blueprint manager cannot be run on non-unix systems");

use blueprint_manager::config::BlueprintManagerConfig;
use blueprint_manager::sdk::entry;
use clap::Parser;

#[tokio::main]
#[allow(clippy::needless_return)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut blueprint_manager_config = BlueprintManagerConfig::parse();

    blueprint_manager_config.data_dir = std::path::absolute(&blueprint_manager_config.data_dir)?;

    entry::setup_blueprint_manager_logger(
        blueprint_manager_config.verbose,
        blueprint_manager_config.pretty,
        "blueprint",
    )?;

    // TODO: blueprint-manager CLI mode
    eprintln!("TODO: blueprint-manager CLI mode");
    return Ok(());

    // let blueprint_config_settings = std::fs::read_to_string(blueprint_config)?;
    // let blueprint_config: BlueprintConfig =
    //     toml::from_str(&blueprint_config_settings).map_err(|err| msg_to_error(err.to_string()))?;
    //
    // // Allow CTRL-C to shutdown this CLI application instance
    // let shutdown_signal = async move {
    //     let _ = tokio::signal::ctrl_c().await;
    // };
    //
    // let handle =
    //     run_blueprint_manager(blueprint_manager_config, blueprint_config, shutdown_signal).await?;
    // handle.await?;
    //
    // Ok(())
}
