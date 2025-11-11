use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use blueprint_manager::executor::run_blueprint_manager;
use blueprint_runner::config::{BlueprintEnvironment, SupportedChains};
use color_eyre::eyre::Result;
use dialoguer::console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;

/// Run an Eigenlayer AVS using the blueprint manager
///
/// This function sets up and runs an EigenLayer AVS using the unified blueprint manager,
/// which provides consistent process management and lifecycle control across protocols.
///
/// # Arguments
///
/// * `mut config` - Blueprint environment configuration
/// * `chain` - The blockchain network to connect to
/// * `keystore_path` - Optional path to the keystore directory
/// * `data_dir` - Optional data directory path
/// * `allow_unchecked_attestations` - Whether to skip binary integrity checks
///
/// # Errors
///
/// Returns an error if:
/// * Eigenlayer protocol settings are missing or invalid
/// * Failed to create or configure the blueprint manager
/// * Failed to run the blueprint
#[allow(clippy::missing_panics_doc)]
pub async fn run_eigenlayer_avs(
    mut config: BlueprintEnvironment,
    _chain: SupportedChains,
    keystore_path: Option<String>,
    data_dir: Option<PathBuf>,
    allow_unchecked_attestations: bool,
) -> Result<()> {
    // Ensure keystore path is set and absolute
    if let Some(keystore) = keystore_path {
        config.keystore_uri = keystore;
    }
    config.keystore_uri = std::path::absolute(&config.keystore_uri)?
        .display()
        .to_string();

    // Set data directory
    config.data_dir = data_dir.unwrap_or_else(|| PathBuf::from("./data"));

    // Verify protocol settings are for Eigenlayer
    let _eigenlayer_settings = config.protocol_settings.eigenlayer()
        .map_err(|_| color_eyre::eyre::eyre!(
            "Eigenlayer protocol settings are required. Use --eigenlayer flag or set protocol_settings in config."
        ))?;

    println!(
        "{}",
        style("Starting EigenLayer AVS via Blueprint Manager")
            .cyan()
            .bold()
    );

    // Configure blueprint manager
    let blueprint_manager_config = BlueprintManagerConfig {
        paths: Paths {
            keystore_uri: config.keystore_uri.clone(),
            data_dir: config.data_dir.clone(),
            ..Default::default()
        },
        verbose: 2,
        pretty: true,
        instance_id: Some("EigenLayer-AVS".to_string()),
        allow_unchecked_attestations,
        ..Default::default()
    };

    let ctx = BlueprintManagerContext::new(blueprint_manager_config).await?;

    println!(
        "{}",
        style("Preparing EigenLayer AVS to run, this may take a few minutes...").cyan()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message("Initializing EigenLayer AVS");
    pb.enable_steady_tick(Duration::from_millis(100));

    let shutdown_signal = async move {
        let _ = signal::ctrl_c().await;
        println!(
            "{}",
            style("Received shutdown signal, stopping EigenLayer AVS")
                .yellow()
                .bold()
        );
    };

    let mut handle = run_blueprint_manager(ctx, config, shutdown_signal).await?;

    pb.finish_with_message("EigenLayer AVS initialized successfully!");

    println!(
        "{}",
        style("Starting EigenLayer AVS execution...").green().bold()
    );
    handle.start()?;

    println!(
        "{}",
        style("EigenLayer AVS is running. Press Ctrl+C to stop.").cyan()
    );
    handle.await?;

    println!("{}", style("EigenLayer AVS has stopped").green());
    Ok(())
}
