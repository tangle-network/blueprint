use blueprint_core::info;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, SourceType, Paths};
use blueprint_runner::config::{BlueprintEnvironment, SupportedChains};
use blueprint_manager::executor::run_blueprint_manager;
use blueprint_std::fs;
use blueprint_std::path::PathBuf;
use color_eyre::eyre::{Result, eyre};
use tokio::signal;
use toml::Value;

fn get_binary_name() -> Result<String> {
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let cargo_data: Value = toml::from_str(&cargo_toml)?;

    // First check for [[bin]] section
    if let Some(Value::Array(bins)) = cargo_data.get("bin") {
        if let Some(first_bin) = bins.first() {
            if let Some(name) = first_bin.get("name").and_then(|n| n.as_str()) {
                return Ok(name.to_string());
            }
        }
    }

    // If no [[bin]] section, try package name
    if let Some(package) = cargo_data.get("package") {
        if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
            return Ok(name.to_string());
        }
    }

    Err(eyre!("Could not find binary name in Cargo.toml"))
}

/// Run a compiled Eigenlayer AVS binary with the provided options
///
/// # Errors
///
/// * Failed to build the binary (if needed)
/// * The binary fails to run, for any reason
#[allow(clippy::missing_panics_doc)]
pub async fn run_eigenlayer_avs(
    config: BlueprintEnvironment,
    chain: SupportedChains,
    binary_path: Option<PathBuf>,
) -> Result<()> {
    let binary_path = if let Some(path) = binary_path {
        path
    } else {
        let target_dir = PathBuf::from("./target/release");
        let binary_name = get_binary_name()?;
        target_dir.join(&binary_name)
    };

    info!(
        "Attempting to run Eigenlayer AVS binary at: {}",
        binary_path.display()
    );

    info!("Preparing Blueprint to run, this may take a few minutes...");

    let default_blueprint_name = "Eigenlayer";
    let default_blueprint_id: u64 = 0;
    let default_instance_id = format!("{}-{}", default_blueprint_name, default_blueprint_id);

    let blueprint_manager_config = BlueprintManagerConfig {
        paths: Paths {
            keystore_uri: config.keystore_uri.clone(),
            data_dir: config.data_dir.clone(),
            cache_dir: "./blueprint-manager/cache".into(),
            runtime_dir: "./blueprint-manager/runtime".into(),
            eigen_blueprint_binary_path: Some(binary_path),
            ..Default::default()
        },
        chain: Some(chain),
        verbose: 2,
        pretty: true,
        allow_unchecked_attestations: true,
        test_mode: true,
        instance_id: Some(default_instance_id.clone()),
        preferred_source: SourceType::Native,
        ..Default::default()
    };
    let manager_ctx = BlueprintManagerContext::new(blueprint_manager_config).await?;

    let shutdown_signal = async move {
        let _ = signal::ctrl_c().await;
        info!("Received shutdown signal, stopping blueprint manager");
    };
    let mut manager_handle = run_blueprint_manager(manager_ctx, config.clone(), shutdown_signal).await?;

    info!("Starting Blueprint manager...");
    manager_handle.start()?;

    info!("Blueprint Manager is running. Press Ctrl+C to stop.");
    manager_handle.await?;

    info!("Blueprint manager has stopped");

    Ok(())
}
