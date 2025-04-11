// src/benchmark/network.rs
//
// Network benchmarking module for measuring network performance

use crate::error::{PricingError, Result};
use blueprint_core::info;
use std::process::Command;
use std::time::Duration;

use super::BenchmarkRunConfig;

/// Run a network benchmark to measure network performance
pub fn run_network_benchmark(config: &BenchmarkRunConfig) -> Result<(f32, f32)> {
    info!("Running network benchmark");

    // Get initial network stats
    let (initial_rx_bytes, initial_tx_bytes) = super::utils::get_network_stats()?;

    // Create a network-intensive workload
    // We'll use curl to download a file from a reliable server
    let network_command = "curl";
    let network_args = vec![
        "-s".to_string(),
        "-o".to_string(),
        "/dev/null".to_string(),
        "https://speed.cloudflare.com/__down?bytes=10000000".to_string(), // 10MB download from Cloudflare
    ];

    // Run the command without monitoring (we'll measure network directly)
    let status = Command::new(network_command)
        .args(&network_args)
        .status()
        .map_err(|e| PricingError::Benchmark(format!("Failed to run network benchmark: {}", e)))?;

    if !status.success() {
        return Err(PricingError::Benchmark(
            "Network benchmark command failed".to_string(),
        ));
    }

    // Wait a moment for network stats to update
    std::thread::sleep(Duration::from_secs(1));

    // Get final network stats
    let (final_rx_bytes, final_tx_bytes) = super::utils::get_network_stats()?;

    // Calculate network usage in MB
    let rx_mb = (final_rx_bytes - initial_rx_bytes) as f32 / 1024.0 / 1024.0;
    let tx_mb = (final_tx_bytes - initial_tx_bytes) as f32 / 1024.0 / 1024.0;

    info!(
        "Network benchmark completed: RX: {:.2} MB, TX: {:.2} MB",
        rx_mb, tx_mb
    );
    Ok((rx_mb, tx_mb))
}
