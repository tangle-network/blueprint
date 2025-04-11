// src/benchmark/io.rs
//
// I/O benchmarking module for measuring disk I/O performance

use crate::error::{PricingError, Result};
use blueprint_core::info;
use std::process::Command;
use std::time::Duration;

use super::BenchmarkRunConfig;

/// Run an I/O-intensive benchmark to measure disk I/O performance
pub fn run_io_benchmark(config: &BenchmarkRunConfig) -> Result<(f32, f32)> {
    info!("Running I/O benchmark");

    // Get initial I/O stats
    let (initial_read_bytes, initial_write_bytes) = super::utils::get_io_stats()?;

    // Create an I/O-intensive workload
    let io_command = "bash";
    let io_args = vec![
        "-c".to_string(),
        r#"
        # I/O benchmark using dd
        # Write 1GB of data
        dd if=/dev/zero of=/tmp/io_test bs=1M count=1024 conv=fsync 2>/dev/null
        # Read it back
        dd if=/tmp/io_test of=/dev/null bs=1M 2>/dev/null
        # Clean up
        rm /tmp/io_test
        "#
        .to_string(),
    ];

    // Run the command without monitoring (we'll measure I/O directly)
    let status = Command::new(io_command)
        .args(&io_args)
        .status()
        .map_err(|e| PricingError::Benchmark(format!("Failed to run I/O benchmark: {}", e)))?;

    if !status.success() {
        return Err(PricingError::Benchmark(
            "I/O benchmark command failed".to_string(),
        ));
    }

    // Wait a moment for I/O stats to update
    std::thread::sleep(Duration::from_secs(1));

    // Get final I/O stats
    let (final_read_bytes, final_write_bytes) = super::utils::get_io_stats()?;

    // Calculate I/O in MB
    let read_mb = (final_read_bytes - initial_read_bytes) as f32 / 1024.0 / 1024.0;
    let write_mb = (final_write_bytes - initial_write_bytes) as f32 / 1024.0 / 1024.0;

    info!(
        "I/O benchmark completed: Read: {:.2} MB, Write: {:.2} MB",
        read_mb, write_mb
    );
    Ok((read_mb, write_mb))
}
