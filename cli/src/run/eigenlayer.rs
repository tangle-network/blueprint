use color_eyre::eyre::{Result, eyre};
use gadget_config::{GadgetConfiguration, supported_chains::SupportedChains};
use reqwest::Url;
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use toml::Value;
use tracing::info;

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
pub async fn run_eigenlayer_avs(config: GadgetConfiguration, chain: SupportedChains) -> Result<()> {
    let target_dir = PathBuf::from("./target/release");
    let binary_name = get_binary_name()?;
    let binary_path = target_dir.join(&binary_name);

    // Always rebuild to ensure we have the latest version
    info!("Building AVS binary...");
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()?;

    if !status.success() {
        return Err(eyre!("Failed to build AVS binary"));
    }

    // Get contract addresses
    let contract_addresses = config.protocol_settings.eigenlayer()
        .map_err(|_| eyre!("Missing Eigenlayer contract addresses"))?;

    // Run the AVS binary with the provided options
    info!("Starting AVS...");
    let mut command = Command::new(&binary_path);
    
    // Add the run subcommand
    command.arg("run");

    // Required arguments
    command
        .arg("--http-rpc-url").arg(&config.http_rpc_endpoint)
        .arg("--ws-rpc-url").arg(&config.ws_rpc_endpoint)
        .arg("--keystore-uri").arg(&config.keystore_uri)
        .arg("--chain").arg(&chain.to_string())
        .arg("--protocol").arg("eigenlayer");

    // Optional arguments
    // TODO: Implement Keystore Password
    // if let Some(password) = &config.keystore_password {
    //     command.arg("--keystore-password").arg(password);
    // }

    // Contract addresses
    command
        .arg("--registry-coordinator").arg(contract_addresses.registry_coordinator_address.to_string())
        .arg("--operator-state-retriever").arg(contract_addresses.operator_state_retriever_address.to_string())
        .arg("--delegation-manager").arg(contract_addresses.delegation_manager_address.to_string())
        .arg("--strategy-manager").arg(contract_addresses.strategy_manager_address.to_string())
        .arg("--service-manager").arg(contract_addresses.service_manager_address.to_string())
        .arg("--stake-registry").arg(contract_addresses.stake_registry_address.to_string())
        .arg("--avs-directory").arg(contract_addresses.avs_directory_address.to_string())
        .arg("--rewards-coordinator").arg(contract_addresses.rewards_coordinator_address.to_string());

    let child = command.spawn()?;

    info!("AVS is running with PID: {}", child.id());
    Ok(())
}
