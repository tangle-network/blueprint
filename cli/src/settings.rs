use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use alloy_primitives::Address;
use blueprint_manager::config::SourceType;
use blueprint_runner::config::{Protocol, ProtocolSettings};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use blueprint_runner::error::ConfigError;
use blueprint_runner::tangle::config::TangleProtocolSettings;
use dotenv::from_path;

pub fn load_protocol_settings(
    protocol: Protocol,
    settings_file: &PathBuf,
) -> Result<ProtocolSettings, ConfigError> {
    from_path(settings_file)
        .map_err(|e| ConfigError::Other(format!("Failed to load settings file: {e}").into()))?;

    match protocol {
        Protocol::Eigenlayer => {
            let addresses = EigenlayerProtocolSettings {
                allocation_manager_address: env_var("ALLOCATION_MANAGER_ADDRESS")?,
                registry_coordinator_address: env_var("REGISTRY_COORDINATOR_ADDRESS")?,
                operator_state_retriever_address: env_var("OPERATOR_STATE_RETRIEVER_ADDRESS")?,
                delegation_manager_address: env_var("DELEGATION_MANAGER_ADDRESS")?,
                service_manager_address: env_var("SERVICE_MANAGER_ADDRESS")?,
                stake_registry_address: env_var("STAKE_REGISTRY_ADDRESS")?,
                strategy_manager_address: env_var("STRATEGY_MANAGER_ADDRESS")?,
                strategy_address: env_var("STRATEGY_ADDRESS")?,
                avs_directory_address: env_var("AVS_DIRECTORY_ADDRESS")?,
                rewards_coordinator_address: env_var("REWARDS_COORDINATOR_ADDRESS")?,
                permission_controller_address: env_var("PERMISSION_CONTROLLER_ADDRESS")?,
                allocation_delay: env_var_default("ALLOCATION_DELAY", 0)?,
                deposit_amount: env_var_default("DEPOSIT_AMOUNT", 5_000_000_000_000_000_000_000)?,
                stake_amount: env_var_default("STAKE_AMOUNT", 1_000_000_000_000_000_000)?,
                operator_sets: env_var_list("OPERATOR_SETS")
                    .map(|sets| sets.into_iter().map(|v| v as u32).collect())
                    .unwrap_or_else(|| vec![0]),
                staker_opt_out_window_blocks: env_var_default(
                    "STAKER_OPT_OUT_WINDOW_BLOCKS",
                    50_400,
                )?,
                metadata_url: std::env::var("METADATA_URL")
                    .unwrap_or_else(|_| "https://github.com/tangle-network/blueprint".to_string()),
            };
            Ok(ProtocolSettings::Eigenlayer(addresses))
        }
        Protocol::Tangle => {
            let blueprint_id = std::env::var("BLUEPRINT_ID")
                .map_err(|_| ConfigError::Other("Missing BLUEPRINT_ID".into()))?
                .parse()
                .map_err(|_| ConfigError::Other("Invalid BLUEPRINT_ID".into()))?;
            let service_id = std::env::var("SERVICE_ID")
                .ok()
                .and_then(|v| v.parse().ok());
            let tangle_contract = env_var("TANGLE_CONTRACT")?;
            let restaking_contract = env_var("RESTAKING_CONTRACT")?;
            let status_registry_contract = env_var("STATUS_REGISTRY_CONTRACT")?;

            Ok(ProtocolSettings::Tangle(TangleProtocolSettings {
                blueprint_id,
                service_id,
                tangle_contract,
                restaking_contract,
                status_registry_contract,
            }))
        }
        _ => Err(ConfigError::UnexpectedProtocol("Unsupported protocol")),
    }
}

fn env_var(name: &str) -> Result<Address, ConfigError> {
    let value =
        std::env::var(name).map_err(|_| ConfigError::Other(format!("Missing {name}").into()))?;
    Address::from_str(&value).map_err(|_| ConfigError::Other(format!("Invalid {name}").into()))
}

fn env_var_default<T>(name: &str, default: T) -> Result<T, ConfigError>
where
    T: FromStr + Copy,
{
    Ok(std::env::var(name)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default))
}

fn env_var_list(name: &str) -> Option<Vec<u64>> {
    std::env::var(name)
        .ok()
        .map(|s| s.split(',').filter_map(|v| v.trim().parse().ok()).collect())
}

const PREFERRED_SOURCE_KEY: &str = "PREFERRED_SOURCE";
const USE_VM_KEY: &str = "USE_VM";

#[derive(Clone, Copy, Debug, Default)]
pub struct RuntimePreferences {
    pub preferred_source: Option<SourceType>,
    pub use_vm: Option<bool>,
}

pub fn load_runtime_preferences() -> RuntimePreferences {
    let preferred_source = std::env::var(PREFERRED_SOURCE_KEY)
        .ok()
        .and_then(|value| parse_source_type(&value));
    let use_vm = std::env::var(USE_VM_KEY)
        .ok()
        .and_then(|value| parse_bool(&value));
    RuntimePreferences {
        preferred_source,
        use_vm,
    }
}

pub fn write_runtime_preferences(
    path: &Path,
    prefs: RuntimePreferences,
) -> Result<(), ConfigError> {
    let mut updates = Vec::new();
    if let Some(source) = prefs.preferred_source {
        updates.push((
            PREFERRED_SOURCE_KEY.to_string(),
            source_to_str(source).to_string(),
        ));
    }
    if let Some(use_vm) = prefs.use_vm {
        updates.push((USE_VM_KEY.to_string(), use_vm.to_string()));
    }

    if updates.is_empty() {
        return Ok(());
    }

    write_env_entries(path, &updates)
}

fn parse_source_type(value: &str) -> Option<SourceType> {
    match value.to_lowercase().as_str() {
        "native" => Some(SourceType::Native),
        "container" => Some(SourceType::Container),
        "wasm" => Some(SourceType::Wasm),
        _ => None,
    }
}

fn source_to_str(source: SourceType) -> &'static str {
    match source {
        SourceType::Native => "native",
        SourceType::Container => "container",
        SourceType::Wasm => "wasm",
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.to_lowercase().as_str() {
        "1" | "true" | "yes" => Some(true),
        "0" | "false" | "no" => Some(false),
        _ => None,
    }
}

fn write_env_entries(path: &Path, updates: &[(String, String)]) -> Result<(), ConfigError> {
    let mut lines = if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| {
            ConfigError::Other(format!("Failed to read {}: {e}", path.display()).into())
        })?;
        parse_env_lines(&content)
    } else {
        Vec::new()
    };

    let mut remaining = updates.iter().cloned().collect::<BTreeMap<_, _>>();

    for line in &mut lines {
        if let EnvLine::Entry { key, value } = line {
            if let Some(new_value) = remaining.remove(key) {
                *value = new_value;
            }
        }
    }

    for (key, value) in remaining {
        lines.push(EnvLine::Entry { key, value });
    }

    let mut content = String::new();
    for line in lines {
        match line {
            EnvLine::Entry { key, value } => {
                content.push_str(&format!("{key}={value}\n"));
            }
            EnvLine::Other(raw) => {
                content.push_str(&raw);
                content.push('\n');
            }
        }
    }

    fs::write(path, content)
        .map_err(|e| ConfigError::Other(format!("Failed to write {}: {e}", path.display()).into()))
}

fn parse_env_lines(input: &str) -> Vec<EnvLine> {
    input
        .lines()
        .map(|line| {
            if let Some((key, value)) = line.split_once('=') {
                EnvLine::Entry {
                    key: key.trim().to_string(),
                    value: value.trim().to_string(),
                }
            } else {
                EnvLine::Other(line.to_string())
            }
        })
        .collect()
}

enum EnvLine {
    Entry { key: String, value: String },
    Other(String),
}
