// src/config.rs
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use toml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorConfig {
    /// Path to store database files (e.g., price cache)
    pub database_path: String,

    /// Command to run for benchmarking
    pub benchmark_command: String,

    /// Arguments for the benchmark command
    pub benchmark_args: Vec<String>,

    /// Maximum duration for benchmark runs (in seconds)
    pub benchmark_duration: u64,

    /// Interval for sampling metrics during benchmarks (in seconds)
    pub benchmark_interval: u64,

    /// Scaling factor for pricing (e.g., Wei per CPU core)
    pub price_scaling_factor: f64,

    /// Path to the operator's keypair file
    pub keypair_path: String,

    /// Path to the keystore directory
    pub keystore_path: String,

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

/// Load configuration from a file or use defaults
pub fn load_config() -> Result<OperatorConfig> {
    // For simplicity, we'll use hardcoded values
    Ok(OperatorConfig {
        database_path: "./data/price_cache".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["Simulated benchmark".to_string()],
        benchmark_duration: 60,
        benchmark_interval: 1,
        price_scaling_factor: 1_000_000.0, // 1M Wei per CPU core
        keypair_path: "./data/operator_key".to_string(),
        keystore_path: "./data/keystore".to_string(),
        rpc_bind_address: "127.0.0.1".to_string(),
        rpc_port: 9000,
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300, // Default to 5 minutes
    })
}

/// Load configuration from a specified path
pub fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<OperatorConfig> {
    // Check if the file exists
    let path = path.as_ref();
    if !path.exists() {
        return load_config();
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
