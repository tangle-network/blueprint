// src/benchmark/utils.rs
//
// Utility functions for benchmarking

use crate::error::{PricingError, Result};
use blueprint_core::{debug, info, warn};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use sysinfo::{Pid, System};

use super::{
    BenchmarkProfile, BenchmarkRunConfig, CpuBenchmarkResult, MemoryAccessMode,
    MemoryBenchmarkResult, MemoryOperationType,
};

/// Helper function to get disk I/O statistics
pub fn get_io_stats() -> Result<(u64, u64)> {
    // Use /proc/diskstats on Linux to get disk I/O stats
    // Format: https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats
    let diskstats = std::fs::read_to_string("/proc/diskstats")
        .map_err(|e| PricingError::Benchmark(format!("Failed to read diskstats: {}", e)))?;

    // Sum up I/O stats from all physical disks (not partitions or virtual devices)
    let mut total_read_bytes = 0;
    let mut total_write_bytes = 0;

    for line in diskstats.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 14 {
            continue;
        }

        let device_name = parts[2];
        // Check if this is a physical disk (not a partition, loop, or ram device)
        if (device_name.starts_with("sd") || 
            device_name.starts_with("nvme") || 
            device_name.starts_with("vd") || 
            device_name.starts_with("xvd")) && 
            // Exclude partitions (devices with numbers at the end)
            !device_name.chars().last().unwrap_or('x').is_numeric()
        {
            // Field 6 is sectors read, field 10 is sectors written
            // Each sector is 512 bytes
            if let (Ok(sectors_read), Ok(sectors_written)) =
                (parts[5].parse::<u64>(), parts[9].parse::<u64>())
            {
                total_read_bytes += sectors_read * 512;
                total_write_bytes += sectors_written * 512;
            }
        }
    }

    Ok((total_read_bytes, total_write_bytes))
}

/// Helper function to get network statistics using system commands
pub fn get_network_stats() -> Result<(u64, u64)> {
    // Use /proc/net/dev on Linux to get network stats
    let netdev = std::fs::read_to_string("/proc/net/dev")
        .map_err(|e| PricingError::Benchmark(format!("Failed to read network stats: {}", e)))?;

    let mut total_rx_bytes = 0;
    let mut total_tx_bytes = 0;

    for line in netdev.lines() {
        if line.contains(":") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }

            let interface = parts[0].trim();
            // Skip loopback interface
            if interface == "lo" {
                continue;
            }

            let stats: Vec<&str> = parts[1].split_whitespace().collect();
            if stats.len() < 10 {
                continue;
            }

            // First field is received bytes, 9th field is transmitted bytes
            if let (Ok(rx), Ok(tx)) = (stats[0].parse::<u64>(), stats[8].parse::<u64>()) {
                total_rx_bytes += rx;
                total_tx_bytes += tx;
            }
        }
    }

    Ok((total_rx_bytes, total_tx_bytes))
}

/// Run a command and monitor its resource usage
pub fn run_and_monitor_command(config: &BenchmarkRunConfig) -> Result<BenchmarkProfile> {
    info!(
        "Running command '{}' with args {:?}",
        config.command, config.args
    );

    // Start the command
    let mut child = Command::new(&config.command)
        .args(&config.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| PricingError::Benchmark(format!("Failed to start command: {}", e)))?;

    // Initialize system info collector
    let mut system = System::new();
    system.refresh_all();

    // Track metrics over time
    let start_time = Instant::now();
    let pid = Pid::from_u32(child.id());

    // Initialize accumulators for average calculations
    let mut total_cpu: f32 = 0.0;
    let mut total_memory: f32 = 0.0;
    let mut peak_cpu: f32 = 0.0;
    let mut peak_memory: f32 = 0.0;
    let mut sample_count: u32 = 0;

    // Monitoring loop
    while start_time.elapsed() < config.max_duration {
        // Check if process is still running
        match child.try_wait() {
            Ok(Some(status)) => {
                info!("Benchmark process exited with status: {}", status);
                break;
            }
            Ok(None) => {
                // Process still running, continue monitoring
            }
            Err(e) => {
                warn!("Error checking benchmark process status: {}", e);
                break;
            }
        }

        // Refresh system information
        system.refresh_all();

        // Find our process in the system processes
        if let Some(process) = system.process(pid) {
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory() as f32 / 1024.0 / 1024.0; // Convert to MB

            // Update accumulators
            total_cpu += cpu_usage;
            total_memory += memory_usage;
            peak_cpu = peak_cpu.max(cpu_usage);
            peak_memory = peak_memory.max(memory_usage);
            sample_count += 1;

            debug!(
                "Sample {}: CPU: {:.2}%, Memory: {:.2} MB",
                sample_count, cpu_usage, memory_usage
            );
        } else {
            warn!("Process {} not found in system processes", pid);
            break;
        }

        // Wait for next sample interval
        thread::sleep(config.sample_interval);
    }

    // Ensure process is terminated if it's still running
    let success = match child.try_wait() {
        Ok(Some(status)) => status.success(),
        Ok(None) => {
            // Process still running, need to kill it
            match child.kill() {
                Ok(_) => {
                    warn!("Had to kill benchmark process as it exceeded max duration");
                    false
                }
                Err(e) => {
                    warn!("Failed to kill benchmark process: {}", e);
                    false
                }
            }
        }
        Err(e) => {
            warn!("Error checking benchmark process status: {}", e);
            false
        }
    };

    // Calculate averages
    let avg_cpu = if sample_count > 0 {
        total_cpu / sample_count as f32
    } else {
        0.0
    };
    let avg_memory = if sample_count > 0 {
        total_memory / sample_count as f32
    } else {
        0.0
    };

    // Convert CPU usage to cores (sysinfo returns percentage, divide by 100 to get core count)
    let avg_cpu_cores = avg_cpu / 100.0;
    let peak_cpu_cores = peak_cpu / 100.0;

    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Create benchmark profile
    let profile = BenchmarkProfile {
        job_id: config.job_id.clone(),
        execution_mode: config.mode.clone(),
        duration_secs: start_time.elapsed().as_secs(),
        timestamp,
        success,
        cpu_details: Some(CpuBenchmarkResult {
            num_cores_detected: System::new().cpus().len(),
            avg_cores_used: avg_cpu_cores,
            avg_usage_percent: avg_cpu,
            peak_cores_used: peak_cpu_cores,
            peak_usage_percent: peak_cpu,
            benchmark_duration_ms: start_time.elapsed().as_millis() as u64,
            primes_found: 0,        // Not applicable for general command monitoring
            max_prime: 0,           // Not applicable
            primes_per_second: 0.0, // Not applicable
            cpu_model: "Unknown".to_string(), // Could extract from system info if needed
            cpu_frequency_mhz: 0.0, // Could extract from system info if needed
        }),
        memory_details: Some(MemoryBenchmarkResult {
            avg_memory_mb: avg_memory,
            peak_memory_mb: peak_memory,
            block_size_kb: 0,
            total_size_mb: 0,
            operations_per_second: 0.0,
            transfer_rate_mb_s: 0.0,
            access_mode: MemoryAccessMode::Sequential,
            operation_type: MemoryOperationType::None,
            latency_ns: 0.0,
            duration_ms: 0,
        }),
        io_details: None,
        network_details: None,
        gpu_details: None,
        storage_details: None,
    };

    info!("Command benchmark completed: {:?}", profile);
    Ok(profile)
}
