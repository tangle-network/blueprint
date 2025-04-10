// src/benchmark.rs
use crate::error::{PricingError, Result};
use blueprint_core::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkProfile {
    pub job_id: String,         // Corresponds to blueprint_id or similar
    pub execution_mode: String, // e.g., "native", "docker"
    pub avg_cpu_cores: f32,
    pub peak_cpu_cores: f32,
    pub avg_memory_mb: f32,
    pub peak_memory_mb: f32,
    pub io_read_mb: f32,  // TODO: Implement IO tracking
    pub io_write_mb: f32, // TODO: Implement IO tracking
    pub duration_secs: u64,
    pub timestamp: u64, // Unix timestamp
    pub success: bool,  // Indicate if benchmark command finished successfully
}

// Configuration specific to a single benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkRunConfig {
    pub command: String,
    pub args: Vec<String>,
    pub job_id: String,            // Identifier for the thing being benchmarked
    pub mode: String,              // e.g., native
    pub max_duration: Duration,    // Max time to run the benchmark process
    pub sample_interval: Duration, // How often to sample metrics
}

pub fn run_benchmark(config: BenchmarkRunConfig) -> Result<BenchmarkProfile> {
    info!(
        "Starting benchmark for job '{}': cmd='{}', args='{:?}', max_duration={:?}",
        config.job_id, config.command, config.args, config.max_duration
    );

    // Redirect stdout/stderr to prevent clutter, or capture if needed
    let mut child = Command::new(&config.command)
        .args(&config.args)
        .stdout(Stdio::null()) // Or Stdio::piped() to capture
        .stderr(Stdio::null()) // Or Stdio::piped()
        .spawn()
        .map_err(|e| {
            PricingError::Benchmark(format!(
                "Failed to spawn command '{}': {}",
                config.command, e
            ))
        })?;

    let pid = sysinfo::Pid::from_u32(child.id());
    let mut system = System::new();
    let mut cpu_samples = Vec::new();
    let mut mem_samples = Vec::new();
    let mut peak_cpu: f32 = 0.0;
    let mut peak_mem: f32 = 0.0;

    let start = Instant::now();
    let mut process_exited = false;

    while start.elapsed() < config.max_duration {
        // Check if process exited prematurely
        match child.try_wait() {
            Ok(Some(status)) => {
                info!(
                    "Benchmark process {} exited early with status: {}",
                    pid, status
                );
                process_exited = true;
                break; // Stop sampling
            }
            Ok(None) => {
                // Process still running
            }
            Err(e) => {
                error!("Error waiting for benchmark process {}: {}", pid, e);
                process_exited = true; // Assume the worst
                break;
            }
        }

        system.refresh_process(pid);
        if let Some(proc) = system.process(pid) {
            // CPU usage is percentage of *one* core, divide by core count for system %
            let cpu_usage_percent: f32 = proc.cpu_usage(); // 0-100 * num_cores
            let cpu_cores: f32 = cpu_usage_percent / 100.0; // Normalize to number of cores

            let mem_kb = proc.memory(); // Memory in KB
            let mem_mb = mem_kb as f32 / 1024.0;

            debug!(
                "Sample for {}: CPU Usage={}%, CPU Cores={}, Mem={} MB",
                pid, cpu_usage_percent, cpu_cores, mem_mb
            );

            cpu_samples.push(cpu_cores);
            mem_samples.push(mem_mb);
            peak_cpu = peak_cpu.max(cpu_cores);
            peak_mem = peak_mem.max(mem_mb);
        } else {
            // Process might have just exited between try_wait and refresh_process
            if !process_exited {
                // Only log if we didn't already know it exited
                warn!(
                    "Benchmark process {} not found during refresh, assuming exit.",
                    pid
                );
            }
            process_exited = true;
            break;
        }
        thread::sleep(config.sample_interval);
    }

    // Ensure process is terminated if it ran for max_duration
    if !process_exited {
        info!("Benchmark duration reached for {}. Killing process.", pid);
        if let Err(e) = child.kill() {
            // Log error, but proceed with profile generation
            error!("Failed to kill benchmark process {}: {}", pid, e);
            // Consider if this should be a Benchmark error
        }
        // Wait briefly to allow OS cleanup? Might not be necessary.
        // let _ = child.wait();
    }

    let final_status = match child.wait() {
        Ok(status) => status.success(),
        Err(e) => {
            error!("Error obtaining final status for process {}: {}", pid, e);
            false // Assume failure if we can't get status
        }
    };

    let duration_secs = start.elapsed().as_secs();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let profile = BenchmarkProfile {
        job_id: config.job_id.clone(),
        execution_mode: config.mode.clone(),
        avg_cpu_cores: mean(&cpu_samples),
        peak_cpu_cores: peak_cpu,
        avg_memory_mb: mean(&mem_samples),
        peak_memory_mb: peak_mem,
        io_read_mb: 0.0,  // Placeholder
        io_write_mb: 0.0, // Placeholder
        duration_secs,
        timestamp,
        success: final_status, // Reflect if the command itself succeeded
    };

    info!(
        "Benchmark finished for job '{}'. Profile: {:?}",
        config.job_id, profile
    );
    Ok(profile)
}

fn mean(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        0.0
    } else {
        samples.iter().copied().sum::<f32>() / samples.len() as f32
    }
}

// Example test (consider placing in tests/ directory)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // This test runs an actual process and takes time
    fn benchmark_sleep_process() {
        let config = BenchmarkRunConfig {
            command: "sleep".into(),
            args: vec!["2".into()], // Short sleep
            job_id: "sleep-test-job".into(),
            mode: "native".into(),
            max_duration: Duration::from_secs(4), // Longer than sleep
            sample_interval: Duration::from_millis(500), // Sample faster
        };

        let result = run_benchmark(config);
        assert!(result.is_ok());
        let profile = result.unwrap();
        assert_eq!(profile.job_id, "sleep-test-job");
        assert!(profile.success); // sleep should exit successfully
        assert!(profile.duration_secs >= 2); // Should run for at least 2 secs
        // Sleep uses minimal CPU/Mem, check for non-negative values mostly
        assert!(profile.avg_cpu_cores >= 0.0);
        assert!(profile.peak_cpu_cores >= 0.0);
        assert!(profile.avg_memory_mb >= 0.0);
        assert!(profile.peak_memory_mb >= 0.0);
    }
}
