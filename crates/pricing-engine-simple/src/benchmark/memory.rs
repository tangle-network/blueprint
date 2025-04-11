// src/benchmark/memory.rs
//
// Memory benchmarking module for measuring memory performance

use crate::error::Result;
use blueprint_core::info;

use super::{BenchmarkRunConfig, MemoryBenchmarkResult};

/// Run a memory-intensive benchmark to measure memory performance
pub fn run_memory_benchmark(config: &BenchmarkRunConfig) -> Result<MemoryBenchmarkResult> {
    info!("Running memory benchmark");

    // Create a memory-intensive workload
    // This simple benchmark allocates and writes to a large array
    let memory_command = "bash";
    let memory_args = vec![
        "-c".to_string(),
        r#"
        # Memory benchmark using dd to allocate and write memory
        # Create a 1GB file in memory
        dd if=/dev/zero of=/dev/shm/memory_test bs=1M count=1024 2>/dev/null
        # Read it back
        dd if=/dev/shm/memory_test of=/dev/null bs=1M 2>/dev/null
        # Clean up
        rm /dev/shm/memory_test
        "#
        .to_string(),
    ];

    // Run the command and monitor memory usage
    let memory_config = BenchmarkRunConfig {
        command: memory_command.to_string(),
        args: memory_args,
        job_id: format!("{}-memory", config.job_id),
        mode: config.mode.clone(),
        max_duration: std::time::Duration::from_secs(20), // Longer duration for memory test
        sample_interval: config.sample_interval,
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: false,
        run_network_test: false,
        run_gpu_test: false,
    };

    let result = super::utils::run_and_monitor_command(&memory_config)?;

    Ok(MemoryBenchmarkResult {
        avg_memory_mb: result
            .memory_details
            .as_ref()
            .map_or(0.0, |m| m.avg_memory_mb),
        peak_memory_mb: result
            .memory_details
            .as_ref()
            .map_or(0.0, |m| m.peak_memory_mb),
    })
}
