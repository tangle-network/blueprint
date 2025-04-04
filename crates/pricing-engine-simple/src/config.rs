// src/config.rs
use crate::error::{PricingError, Result};
use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct OperatorConfig {
    pub database_path: PathBuf,
    pub keypair_path: PathBuf,
    #[serde(with = "humantime_serde")]
    pub quote_validity_duration: Duration,
    // Add other config fields like RPC bind address, blockchain node URL, etc.
    pub rpc_bind_address: String,
    // We need info on how to run benchmarks per blueprint
    // This is simplified - likely needs a more complex structure
    // mapping blueprint IDs/hashes to benchmark commands/images.
    pub benchmark_command: String,
    #[serde(default = "default_benchmark_args")]
    pub benchmark_args: Vec<String>,
    #[serde(with = "humantime_serde", default = "default_benchmark_duration")]
    pub benchmark_duration: Duration,
    #[serde(with = "humantime_serde", default = "default_benchmark_interval")]
    pub benchmark_interval: Duration,
    #[serde(default = "default_price_scaling_factor")]
    pub price_scaling_factor: f64, // Simple factor for pricing
}

// Defaults for serde
fn default_benchmark_args() -> Vec<String> {
    vec![]
}
fn default_benchmark_duration() -> Duration {
    Duration::from_secs(10)
}
fn default_benchmark_interval() -> Duration {
    Duration::from_secs(1)
}
fn default_price_scaling_factor() -> f64 {
    1.0
}

// Basic loading function - enhance as needed (e.g., use config crate)
pub fn load_config() -> Result<OperatorConfig> {
    // In a real app, load from a file (e.g., "config.toml") or env vars
    // For simplicity here, we'll hardcode some values, but ideally use the config crate.
    // Example using config crate (requires setup):
    /*
    let builder = config::Config::builder()
        .add_source(config::File::with_name("config/operator.toml").required(false))
        .add_source(config::Environment::with_prefix("OPERATOR"));
    let cfg = builder.build()?.try_deserialize::<OperatorConfig>()?;
    Ok(cfg)
    */

    // --- Hardcoded Example (replace with real loading) ---
    let config = OperatorConfig {
        database_path: PathBuf::from("./operator_db"),
        keypair_path: PathBuf::from("./operator_keypair.bin"),
        quote_validity_duration: Duration::from_secs(300), // 5 minutes
        rpc_bind_address: "0.0.0.0:50051".to_string(),
        benchmark_command: "sleep".to_string(), // Default/example command
        benchmark_args: vec!["5".to_string()],  // Example args
        benchmark_duration: Duration::from_secs(6),
        benchmark_interval: Duration::from_secs(1),
        price_scaling_factor: 100.0, // e.g., 100 Wei per avg CPU core second
    };
    std::fs::create_dir_all(&config.database_path)?; // Ensure DB path exists
    Ok(config)
    // --- End Hardcoded Example ---
}
