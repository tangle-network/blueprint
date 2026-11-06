use crate::error::Result;
use blueprint_core::warn;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorConfig {
    /// Path to store database files (e.g., price cache)
    pub database_path: String,

    /// Maximum duration for benchmark runs (in seconds)
    pub benchmark_duration: u64,

    /// Interval for sampling metrics during benchmarks (in seconds)
    pub benchmark_interval: u64,

    /// Path to the keystore directory
    pub keystore_path: PathBuf,

    /// Address to bind the RPC server to
    pub rpc_bind_address: String,

    /// Port for the RPC server
    pub rpc_port: u16,

    /// Timeout for RPC requests (in seconds)
    pub rpc_timeout: u64,

    /// Maximum number of concurrent RPC connections
    pub rpc_max_connections: u32,

    /// Validity duration for generated quotes (in seconds)
    pub quote_validity_duration_secs: u64,

    /// Canonical GPU class -> integer wei per GPU-second, encoded as decimal strings in TOML.
    #[serde(default, with = "gpu_pricing")]
    pub gpu_pricing: std::collections::HashMap<String, alloy_primitives::U256>,
}

impl Default for OperatorConfig {
    fn default() -> Self {
        OperatorConfig {
            database_path: "./data/price_cache".to_string(),
            benchmark_duration: 60,
            benchmark_interval: 1,
            keystore_path: PathBuf::from("./data/keystore"),
            rpc_bind_address: String::from("127.0.0.1"),
            rpc_port: 9000,
            rpc_timeout: 30,
            rpc_max_connections: 100,
            quote_validity_duration_secs: 300,
            gpu_pricing: std::collections::HashMap::new(),
        }
    }
}

/// Load configuration from a specified path
pub fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<OperatorConfig> {
    // Check if the file exists
    let path = path.as_ref();
    if !path.exists() {
        warn!(
            "Config file not found at {}, using defaults",
            path.display()
        );
        return Ok(OperatorConfig::default());
    }

    // Read the file content
    let content = std::fs::read_to_string(path).map_err(|e| {
        crate::error::PricingError::Config(format!("Failed to read config file: {e}"))
    })?;

    // Parse the TOML content
    let config: OperatorConfig = toml::from_str(&content).map_err(|e| {
        crate::error::PricingError::Config(format!("Failed to parse config file: {e}"))
    })?;

    Ok(config)
}

mod gpu_pricing {
    use alloy_primitives::U256;
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
    use std::collections::HashMap;

    pub fn serialize<S: Serializer>(
        rates: &HashMap<String, U256>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        rates
            .iter()
            .map(|(class, rate)| (class, rate.to_string()))
            .collect::<HashMap<_, _>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<HashMap<String, U256>, D::Error> {
        HashMap::<String, String>::deserialize(deserializer)?
            .into_iter()
            .map(|(class, value)| {
                if blueprint_core::gpu::find(&class).is_none() {
                    return Err(D::Error::custom(format!("unknown GPU class: {class}")));
                }
                let rate = U256::from_str_radix(&value, 10).map_err(D::Error::custom)?;
                if rate.is_zero() {
                    return Err(D::Error::custom(format!(
                        "GPU rate must be positive: {class}"
                    )));
                }
                Ok((class, rate))
            })
            .collect()
    }
}
