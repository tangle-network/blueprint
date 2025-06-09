#[cfg(not(unix))]
compile_error!("The blueprint manager cannot be run on non-unix systems");

use blueprint_manager::config::BlueprintManagerCli;
use blueprint_manager::run_blueprint_manager;
use blueprint_manager::sdk::entry;
use blueprint_runner::config::BlueprintEnvironment;
use clap::Parser;

#[tokio::main]
#[allow(clippy::needless_return)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let BlueprintManagerCli { mut config } = BlueprintManagerCli::parse();

    config.data_dir = std::path::absolute(&config.data_dir)?;

    entry::setup_blueprint_manager_logger(config.verbose, config.pretty, "blueprint")?;

    let blueprint_config = match config.blueprint_config.as_deref() {
        Some(config_path) => {
            let blueprint_config_settings = std::fs::read_to_string(config_path)?;
            match toml::from_str(&blueprint_config_settings) {
                Ok(config) => config,
                Err(e) => {
                    blueprint_core::error!(
                        "Failed to parse config file at `{}`: {e}",
                        config_path.display()
                    );
                    return Err(e.into());
                }
            }
        }
        None => {
            blueprint_core::warn!("No config file specified, using defaults");
            BlueprintEnvironment::default()
        }
    };

    // Allow CTRL-C to shutdown this CLI application instance
    let shutdown_signal = async move {
        let _ = tokio::signal::ctrl_c().await;
    };

    let handle = run_blueprint_manager(config, blueprint_config, shutdown_signal).await?;
    handle.await?;

    Ok(())
}
