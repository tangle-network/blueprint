use crate::error::Result;
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
            quote_validity_duration_secs: 300, // Default to 5 minutes
        }
    }
}

/// Load configuration from a specified path
pub fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<OperatorConfig> {
    // Check if the file exists
    let path = path.as_ref();
    if !path.exists() {
        return Ok(OperatorConfig::default());
    }

    // Read the file content
    let content = std::fs::read_to_string(path).map_err(|e| {
        crate::error::PricingError::Config(format!("Failed to read config file: {}", e))
    })?;

    // Parse the TOML content
    let config: OperatorConfig = toml::from_str(&content).map_err(|e| {
        crate::error::PricingError::Config(format!("Failed to parse config file: {}", e))
    })?;

    Ok(config)
}
