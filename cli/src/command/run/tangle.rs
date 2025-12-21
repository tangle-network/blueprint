use alloy_primitives::Address;
use blueprint_manager::config::{
    BlueprintManagerConfig, BlueprintManagerContext, Paths, SourceType,
};
use blueprint_manager::executor::run_blueprint_manager;
use blueprint_runner::config::{BlueprintEnvironment, ProtocolSettings};
use blueprint_runner::tangle_evm::config::TangleEvmProtocolSettings;
use color_eyre::eyre::Result;
use dialoguer::console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;
use url::Url;

#[derive(Clone)]
pub struct RunOpts {
    pub http_rpc_url: Url,
    pub ws_rpc_url: Url,
    pub blueprint_id: u64,
    pub service_id: Option<u64>,
    pub tangle_contract: Address,
    pub restaking_contract: Address,
    pub status_registry_contract: Address,
    pub keystore_path: String,
    pub data_dir: Option<PathBuf>,
    pub allow_unchecked_attestations: bool,
    pub registration_mode: bool,
    pub registration_capture_only: bool,
    pub preferred_source: SourceType,
    pub use_vm: bool,
    pub dry_run: bool,
}

pub async fn run_blueprint(opts: RunOpts) -> Result<()> {
    let mut blueprint_config = BlueprintEnvironment::default();
    blueprint_config.http_rpc_endpoint = opts.http_rpc_url;
    blueprint_config.ws_rpc_endpoint = opts.ws_rpc_url;
    blueprint_config.keystore_uri = opts.keystore_path;
    blueprint_config.data_dir = opts.data_dir.unwrap_or_else(|| PathBuf::from("./data"));
    blueprint_config.registration_mode = opts.registration_mode;
    blueprint_config.registration_capture_only = opts.registration_capture_only;
    blueprint_config.dry_run = opts.dry_run;
    blueprint_config.protocol_settings = ProtocolSettings::TangleEvm(TangleEvmProtocolSettings {
        blueprint_id: opts.blueprint_id,
        service_id: opts.service_id,
        tangle_contract: opts.tangle_contract,
        restaking_contract: opts.restaking_contract,
        status_registry_contract: opts.status_registry_contract,
    });

    #[allow(unused_mut)]
    let mut blueprint_manager_config = BlueprintManagerConfig {
        paths: Paths {
            keystore_uri: blueprint_config.keystore_uri.clone(),
            data_dir: blueprint_config.data_dir.clone(),
            ..Default::default()
        },
        verbose: 2,
        pretty: true,
        instance_id: Some(format!("Blueprint-{}", opts.blueprint_id)),
        allow_unchecked_attestations: opts.allow_unchecked_attestations,
        preferred_source: opts.preferred_source,
        ..Default::default()
    };

    #[cfg(feature = "vm-debug")]
    {
        blueprint_manager_config.vm_sandbox_options.no_vm = !opts.use_vm;
    }

    let ctx = BlueprintManagerContext::new(blueprint_manager_config).await?;

    println!(
        "{}",
        style(format!(
            "Starting blueprint manager for blueprint ID: {}",
            opts.blueprint_id
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
