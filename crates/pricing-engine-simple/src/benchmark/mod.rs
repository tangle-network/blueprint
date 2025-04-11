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
    pub duration_secs: u64,
    pub timestamp: u64,                                  // Unix timestamp
    pub success: bool, // Indicate if benchmark command finished successfully
    pub cpu_details: Option<CpuBenchmarkResult>, // Detailed CPU benchmark results
    pub io_details: Option<IoBenchmarkResult>, // Detailed I/O benchmark results
    pub memory_details: Option<MemoryBenchmarkResult>, // Detailed memory benchmark results
    pub network_details: Option<NetworkBenchmarkResult>, // Detailed network benchmark results
    pub gpu_details: Option<GpuBenchmarkResult>, // Detailed GPU benchmark results
    pub storage_details: Option<StorageBenchmarkResult>, // Detailed storage benchmark results
}

/// Memory benchmark results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryBenchmarkResult {
    pub avg_memory_mb: f32,
    pub peak_memory_mb: f32,
    pub block_size_kb: u64,                  // Memory block size in KB
    pub total_size_mb: u64,                  // Total memory size in MB
    pub operations_per_second: f32,          // Operations per second
    pub transfer_rate_mb_s: f32,             // Transfer rate in MB/s
    pub access_mode: MemoryAccessMode,       // Sequential or random access
    pub operation_type: MemoryOperationType, // Read, write or none
    pub latency_ns: f32,                     // Average latency in nanoseconds
    pub duration_ms: u64,                    // Benchmark duration in milliseconds
}

/// Memory access modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryAccessMode {
    Sequential,
    Random,
}

/// Memory operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryOperationType {
    Read,
    Write,
    None,
}

/// Network benchmark results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkBenchmarkResult {
    pub network_rx_mb: f32,
    pub network_tx_mb: f32,
    pub download_speed_mbps: f32, // Download speed in Mbps
    pub upload_speed_mbps: f32,   // Upload speed in Mbps
    pub latency_ms: f32,          // Network latency in milliseconds
    pub duration_ms: u64,         // Benchmark duration in milliseconds
    pub packet_loss_percent: f32, // Packet loss percentage
    pub jitter_ms: f32,           // Jitter in milliseconds
}

/// GPU benchmark results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpuBenchmarkResult {
    pub gpu_available: bool,    // Whether GPU is available
    pub gpu_memory_mb: f32,     // GPU memory if available
    pub gpu_model: String,      // GPU model name
    pub gpu_frequency_mhz: f32, // GPU frequency in MHz if available
}

/// Storage benchmark results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageBenchmarkResult {
    pub storage_available_gb: f32, // Available storage
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
    use blueprint_core::{info, warn};

    info!(
        "Starting benchmark suite for job '{}' with max_duration={}s",
        job_id,
        max_duration.as_secs()
    );

    // Create a benchmark run configuration
    let config = BenchmarkRunConfig {
        command: "".to_string(),
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

    // Initialize the benchmark profile
    let mut profile = BenchmarkProfile {
        job_id: job_id.clone(),
        execution_mode: mode.clone(),
        duration_secs: 0,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        success: true,
        cpu_details: None,
        io_details: None,
        memory_details: None,
        network_details: None,
        gpu_details: None,
        storage_details: None,
    };

    // Measure available storage
    if let Ok(output) = std::process::Command::new("df").args(&["-h", "/"]).output() {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            if let Some(line) = output_str.lines().nth(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let avail = parts[3];
                    // Convert to GB - handle different formats (e.g., "10G", "1.5T")
                    if avail.ends_with('G') {
                        if let Ok(gb) = avail.trim_end_matches('G').parse::<f32>() {
                            profile.storage_details = Some(StorageBenchmarkResult {
                                storage_available_gb: gb,
                            });
                        }
                    } else if avail.ends_with('T') {
                        if let Ok(tb) = avail.trim_end_matches('T').parse::<f32>() {
                            profile.storage_details = Some(StorageBenchmarkResult {
                                storage_available_gb: tb * 1024.0,
                            });
                        }
                    } else if avail.ends_with('M') {
                        if let Ok(mb) = avail.trim_end_matches('M').parse::<f32>() {
                            profile.storage_details = Some(StorageBenchmarkResult {
                                storage_available_gb: mb / 1024.0,
                            });
                        }
                    }
                }
            }
        }
    }

    // Start time
    let start_time = Instant::now();

    // Run the CPU benchmark if requested
    if run_cpu_test {
        info!("Running CPU benchmark test");
        match run_cpu_benchmark(&config) {
            Ok(cpu_details) => {
                profile.cpu_details = Some(cpu_details);
                info!("CPU benchmark completed successfully");
            }
            Err(e) => {
                warn!("CPU benchmark failed: {}", e);
            }
        }
    }

    // Run the memory benchmark if requested
    if run_memory_test {
        match run_memory_benchmark(&config) {
            Ok(memory_details) => {
                profile.memory_details = Some(memory_details);
            }
            Err(e) => {
                warn!("Memory benchmark failed: {}", e);
            }
        }
    }

    // Run the I/O benchmark if requested
    if run_io_test {
        match run_io_benchmark(&config) {
            Ok(io_result) => {
                profile.io_details = Some(io_result);
            }
            Err(e) => {
                warn!("I/O benchmark failed: {}", e);
            }
        }
    }

    // Run the network benchmark if requested
    if run_network_test {
        match run_network_benchmark(&config) {
            Ok(network_details) => {
                profile.network_details = Some(network_details);
            }
            Err(e) => {
                warn!("Network benchmark failed: {}", e);
            }
        }
    }

    // Run the GPU benchmark if requested
    if run_gpu_test {
        match run_gpu_benchmark(&config) {
            Ok(gpu_details) => {
                profile.gpu_details = Some(gpu_details);
            }
            Err(e) => {
                warn!("GPU benchmark failed: {}", e);
                // Some systems legitimately don't have GPUs
                profile.gpu_details = Some(GpuBenchmarkResult {
                    gpu_available: false,
                    gpu_memory_mb: 0.0,
                    gpu_model: "".to_string(),
                    gpu_frequency_mhz: 0.0,
                });
            }
        }
    }

    // Record the total duration
    profile.duration_secs = start_time.elapsed().as_secs();

    info!("Benchmark suite completed: {:?}", profile);
    Ok(profile)
}

// Maintain backward compatibility with the original function
pub fn run_benchmark(config: BenchmarkRunConfig) -> crate::error::Result<BenchmarkProfile> {
    utils::run_and_monitor_command(&config)
}
