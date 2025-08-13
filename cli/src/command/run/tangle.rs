use alloy_signer_local::PrivateKeySigner;
use blueprint_crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use blueprint_manager::executor::run_blueprint_manager;
use blueprint_runner::config::BlueprintEnvironment;
use color_eyre::eyre::{Result, eyre};
use dialoguer::console::style;
use indicatif::{ProgressBar, ProgressStyle};
use sp_core::sr25519;
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;
use url::Url;

#[derive(Clone)]
pub struct RunOpts {
    /// The HTTP RPC URL of the Tangle Network
    pub http_rpc_url: Url,
    /// The WS RPC URL of the Tangle Network
    pub ws_rpc_url: Url,
    /// The signer for Tangle operations
    pub signer: Option<TanglePairSigner<sr25519::Pair>>,
    /// The signer for EVM operations
    pub signer_evm: Option<PrivateKeySigner>,
    /// The blueprint ID to run
    pub blueprint_id: Option<u64>,
    /// The keystore path
    pub keystore_path: Option<String>,
    /// The data directory path
    pub data_dir: Option<PathBuf>,
    /// Whether to allow invalid GitHub attestations (binary integrity checks)
    ///
    /// This will also allow for running the manager without the GitHub CLI installed.
    pub allow_unchecked_attestations: bool,
}

/// Runs a blueprint using the blueprint manager
///
/// # Arguments
///
/// * `opts` - Options for running the blueprint
///
/// # Errors
///
/// Returns an error if:
/// * Blueprint ID is not provided
/// * Failed to create or configure the blueprint manager
/// * Failed to run the blueprint
#[allow(clippy::missing_panics_doc)]
pub async fn run_blueprint(opts: RunOpts) -> Result<()> {
    let blueprint_id = opts
        .blueprint_id
        .ok_or_else(|| eyre!("Blueprint ID is required"))?;

    let mut blueprint_config = BlueprintEnvironment::default();
    blueprint_config.http_rpc_endpoint = opts.http_rpc_url;
    blueprint_config.ws_rpc_endpoint = opts.ws_rpc_url;

    if let Some(keystore_path) = opts.keystore_path {
        blueprint_config.keystore_uri = keystore_path;
    }

    blueprint_config.keystore_uri = std::path::absolute(&blueprint_config.keystore_uri)?
        .display()
        .to_string();

    blueprint_config.data_dir = opts.data_dir.unwrap_or_else(|| PathBuf::from("./data"));

    let blueprint_manager_config = BlueprintManagerConfig {
        paths: Paths {
            keystore_uri: blueprint_config.keystore_uri.clone(),
            data_dir: blueprint_config.data_dir.clone(),
            ..Default::default()
        },
        verbose: 2,
        pretty: true,
        instance_id: Some(format!("Blueprint-{}", blueprint_id)),
        allow_unchecked_attestations: opts.allow_unchecked_attestations,
        ..Default::default()
    };

    let ctx = BlueprintManagerContext::new(blueprint_manager_config).await?;

    println!(
        "{}",
        style(format!(
            "Starting blueprint manager for blueprint ID: {}",
            blueprint_id
        ))
        .cyan()
        .bold()
    );

    let shutdown_signal = async move {
        let _ = signal::ctrl_c().await;
        println!(
            "{}",
            style("Received shutdown signal, stopping blueprint manager")
                .yellow()
                .bold()
        );
    };

    println!(
        "{}",
        style("Preparing Blueprint to run, this may take a few minutes...").cyan()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message("Initializing Blueprint");
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut handle = run_blueprint_manager(ctx, blueprint_config, shutdown_signal).await?;

    pb.finish_with_message("Blueprint initialized successfully!");

    println!(
        "{}",
        style("Starting blueprint execution...").green().bold()
    );
    handle.start()?;

    println!(
        "{}",
        style("Blueprint is running. Press Ctrl+C to stop.").cyan()
    );
    handle.await?;

    println!("{}", style("Blueprint manager has stopped").green());
    Ok(())
}
