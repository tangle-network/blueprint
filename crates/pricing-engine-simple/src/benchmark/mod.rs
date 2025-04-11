// src/benchmark/mod.rs
//
// Main module for benchmarking functionality
// This module coordinates the various benchmark components

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// Import submodules
pub mod cpu;
pub mod gpu;
pub mod io;
pub mod memory;
pub mod network;
pub mod utils;

// Re-export key types and functions
pub use cpu::{CpuBenchmarkResult, DEFAULT_MAX_PRIME, run_cpu_benchmark};
pub use gpu::{
    DEFAULT_AMD_GPU_MEMORY, DEFAULT_INTEL_GPU_MEMORY, DEFAULT_NVIDIA_GPU_MEMORY,
    DEFAULT_UNKNOWN_GPU_MEMORY, run_gpu_benchmark,
};
pub use io::{IoBenchmarkResult, IoTestMode, run_io_benchmark};
pub use memory::run_memory_benchmark;
pub use network::run_network_benchmark;
pub use utils::{get_io_stats, get_network_stats, run_and_monitor_command};

/// Main benchmark profile that contains all benchmark results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkProfile {
    pub job_id: String,         // Corresponds to blueprint_id or similar
    pub execution_mode: String, // e.g., "native", "docker"
    pub avg_cpu_cores: f32,
    pub peak_cpu_cores: f32,
    pub avg_memory_mb: f32,
    pub peak_memory_mb: f32,
    pub io_read_mb: f32,
    pub io_write_mb: f32,
    pub network_rx_mb: f32,        // Network received (download)
    pub network_tx_mb: f32,        // Network transmitted (upload)
    pub storage_available_gb: f32, // Available storage
    pub gpu_available: bool,       // Whether GPU is available
    pub gpu_memory_mb: f32,        // GPU memory if available
    pub duration_secs: u64,
    pub timestamp: u64,                          // Unix timestamp
    pub success: bool, // Indicate if benchmark command finished successfully
    pub cpu_details: Option<CpuBenchmarkResult>, // Detailed CPU benchmark results
}

/// Configuration specific to a single benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkRunConfig {
    pub command: String,           // Command to run
    pub args: Vec<String>,         // Arguments for the command
    pub job_id: String,            // Identifier for the thing being benchmarked
    pub mode: String,              // e.g., native
    pub max_duration: Duration,    // Max time to run the benchmark process
    pub sample_interval: Duration, // How often to sample metrics
    pub run_cpu_test: bool,        // Whether to run CPU test
    pub run_memory_test: bool,     // Whether to run memory test
    pub run_io_test: bool,         // Whether to run I/O test
    pub run_network_test: bool,    // Whether to run network test
    pub run_gpu_test: bool,        // Whether to run GPU test
}

impl Default for BenchmarkRunConfig {
    fn default() -> Self {
        Self {
            command: "echo".to_string(),
            args: vec!["benchmark".to_string()],
            job_id: "benchmark".to_string(),
            mode: "native".to_string(),
            max_duration: Duration::from_secs(30),
            sample_interval: Duration::from_millis(500),
            run_cpu_test: true,
            run_memory_test: true,
            run_io_test: true,
            run_network_test: true,
            run_gpu_test: true,
        }
    }
}

/// Run a comprehensive benchmark suite to measure various system resources
/// This is the main entry point for benchmarking
pub fn run_benchmark_suite(
    job_id: String,
    mode: String,
    max_duration: Duration,
    sample_interval: Duration,
    run_cpu_test: bool,
    run_memory_test: bool,
    run_io_test: bool,
    run_network_test: bool,
    run_gpu_test: bool,
) -> crate::error::Result<BenchmarkProfile> {
    use crate::error::Result;
    use blueprint_core::{info, warn};

    info!(
        "Starting benchmark suite for job '{}' with max_duration={:?}",
        job_id, max_duration
    );

    let start_time = Instant::now();

    // Initialize the profile with default values
    let mut profile = BenchmarkProfile {
        job_id: job_id.clone(),
        execution_mode: mode.clone(),
        avg_cpu_cores: 0.0,
        peak_cpu_cores: 0.0,
        avg_memory_mb: 0.0,
        peak_memory_mb: 0.0,
        io_read_mb: 0.0,
        io_write_mb: 0.0,
        network_rx_mb: 0.0,
        network_tx_mb: 0.0,
        storage_available_gb: 0.0,
        gpu_available: false,
        gpu_memory_mb: 0.0,
        duration_secs: 0,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        success: true,
        cpu_details: None,
    };

    // Measure available storage
    // Use df command to get disk space since sysinfo disk API might differ between versions
    if let Ok(output) = std::process::Command::new("df")
        .args(&["-h", "--output=avail", "/"])
        .output()
    {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            // Parse the output to get available space
            let lines: Vec<&str> = output_str.lines().collect();
            if lines.len() > 1 {
                let avail = lines[1].trim();
                // Convert to GB - handle different formats (e.g., "10G", "1.5T")
                if avail.ends_with('G') {
                    if let Ok(gb) = avail.trim_end_matches('G').parse::<f32>() {
                        profile.storage_available_gb = gb;
                    }
                } else if avail.ends_with('T') {
                    if let Ok(tb) = avail.trim_end_matches('T').parse::<f32>() {
                        profile.storage_available_gb = tb * 1024.0;
                    }
                } else if avail.ends_with('M') {
                    if let Ok(mb) = avail.trim_end_matches('M').parse::<f32>() {
                        profile.storage_available_gb = mb / 1024.0;
                    }
                }
            }
        }
    }

    // Create a config for the benchmark
    let config = BenchmarkRunConfig {
        command: "echo".to_string(),
        args: vec![],
        job_id: job_id.clone(),
        mode: mode.clone(),
        max_duration,
        sample_interval,
        run_cpu_test,
        run_memory_test,
        run_io_test,
        run_network_test,
        run_gpu_test,
    };

    // Run CPU benchmark if enabled
    if run_cpu_test {
        info!("Running CPU benchmark test");
        match run_cpu_benchmark(&config) {
            Ok((avg_cores, peak_cores, cpu_details)) => {
                profile.avg_cpu_cores = avg_cores;
                profile.peak_cpu_cores = peak_cores;
                profile.cpu_details = Some(cpu_details);
                info!("CPU benchmark completed successfully");
            }
            Err(e) => {
                warn!("CPU benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }

    // Run memory benchmark if enabled
    if run_memory_test {
        match run_memory_benchmark(&config) {
            Ok((avg_memory, peak_memory)) => {
                profile.avg_memory_mb = avg_memory;
                profile.peak_memory_mb = peak_memory;
            }
            Err(e) => {
                warn!("Memory benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }

    // Run I/O benchmark if enabled
    if run_io_test {
        match run_io_benchmark(&config) {
            Ok(io_result) => {
                profile.io_read_mb = io_result.read_mb;
                profile.io_write_mb = io_result.write_mb;
            }
            Err(e) => {
                warn!("I/O benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }

    // Run network benchmark if enabled
    if run_network_test {
        match run_network_benchmark(&config) {
            Ok((rx_mb, tx_mb)) => {
                profile.network_rx_mb = rx_mb;
                profile.network_tx_mb = tx_mb;
            }
            Err(e) => {
                warn!("Network benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }

    // Run GPU benchmark if enabled
    if run_gpu_test {
        match run_gpu_benchmark(&config) {
            Ok((gpu_available, gpu_memory)) => {
                profile.gpu_available = gpu_available;
                profile.gpu_memory_mb = gpu_memory;
            }
            Err(e) => {
                warn!("GPU benchmark failed: {}", e);
                // Don't mark the entire benchmark as failed just because GPU check failed
                // Some systems legitimately don't have GPUs
                profile.gpu_available = false;
                profile.gpu_memory_mb = 0.0;
            }
        }
    }

    // Set the total duration
    profile.duration_secs = start_time.elapsed().as_secs();

    info!("Benchmark suite completed: {:?}", profile);
    Ok(profile)
}

// Maintain backward compatibility with the original function
pub fn run_benchmark(config: BenchmarkRunConfig) -> crate::error::Result<BenchmarkProfile> {
    utils::run_and_monitor_command(&config)
}
