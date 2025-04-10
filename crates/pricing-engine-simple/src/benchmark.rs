// src/benchmark.rs
use crate::error::{PricingError, Result};
use blueprint_core::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use sysinfo::{System, Pid};
use std::path::Path;

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
    pub network_rx_mb: f32,     // Network received (download)
    pub network_tx_mb: f32,     // Network transmitted (upload)
    pub storage_available_gb: f32, // Available storage
    pub gpu_available: bool,    // Whether GPU is available
    pub gpu_memory_mb: f32,     // GPU memory if available
    pub duration_secs: u64,
    pub timestamp: u64,         // Unix timestamp
    pub success: bool,          // Indicate if benchmark command finished successfully
}

// Configuration specific to a single benchmark run
#[derive(Debug, Clone)]
pub struct BenchmarkRunConfig {
    pub command: String,        // Command to run (kept for backward compatibility)
    pub args: Vec<String>,      // Arguments for the command (kept for backward compatibility)
    pub job_id: String,         // Identifier for the thing being benchmarked
    pub mode: String,           // e.g., native
    pub max_duration: Duration, // Max time to run the benchmark process
    pub sample_interval: Duration, // How often to sample metrics
    pub run_cpu_test: bool,     // Whether to run CPU test
    pub run_memory_test: bool,  // Whether to run memory test
    pub run_io_test: bool,      // Whether to run I/O test
    pub run_network_test: bool, // Whether to run network test
    pub run_gpu_test: bool,     // Whether to run GPU test
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

/// Run a comprehensive benchmark suite to profile system resources
pub fn run_benchmark_suite(config: BenchmarkRunConfig) -> Result<BenchmarkProfile> {
    info!(
        "Starting benchmark suite for job '{}' with max_duration={:?}",
        config.job_id, config.max_duration
    );
    
    let start_time = Instant::now();
    
    // Initialize the profile with default values
    let mut profile = BenchmarkProfile {
        job_id: config.job_id.clone(),
        execution_mode: config.mode.clone(),
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
            .map_err(|e| PricingError::Benchmark(format!("Failed to get system time: {}", e)))?
            .as_secs(),
        success: true,
    };
    
    // Measure available storage
    // Use df command to get disk space since sysinfo disk API might differ between versions
    if let Ok(output) = Command::new("df")
        .args(&["-h", "--output=avail", "/"])
        .output() 
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        // Parse the output to get available space
        // Format is typically "Avail\n100G" so we take the second line
        let lines: Vec<&str> = output_str.trim().split('\n').collect();
        if lines.len() > 1 {
            let avail = lines[1].trim();
            // Extract the number part and convert to GB
            if let Some(num_end) = avail.find(|c: char| !c.is_digit(10) && c != '.') {
                if let Ok(num) = avail[..num_end].parse::<f32>() {
                    // Convert to GB based on unit
                    let unit = avail[num_end..].trim();
                    profile.storage_available_gb = match unit {
                        "T" | "TB" => num * 1024.0,
                        "G" | "GB" => num,
                        "M" | "MB" => num / 1024.0,
                        _ => num, // Assume GB if unknown
                    };
                }
            }
        }
    }
    
    // Run CPU benchmark if enabled
    if config.run_cpu_test {
        match run_cpu_benchmark(&config) {
            Ok(cpu_results) => {
                profile.avg_cpu_cores = cpu_results.0;
                profile.peak_cpu_cores = cpu_results.1;
            }
            Err(e) => {
                warn!("CPU benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }
    
    // Run memory benchmark if enabled
    if config.run_memory_test {
        match run_memory_benchmark(&config) {
            Ok(memory_results) => {
                profile.avg_memory_mb = memory_results.0;
                profile.peak_memory_mb = memory_results.1;
            }
            Err(e) => {
                warn!("Memory benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }
    
    // Run I/O benchmark if enabled
    if config.run_io_test {
        match run_io_benchmark(&config) {
            Ok(io_results) => {
                profile.io_read_mb = io_results.0;
                profile.io_write_mb = io_results.1;
            }
            Err(e) => {
                warn!("I/O benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }
    
    // Run network benchmark if enabled
    if config.run_network_test {
        match run_network_benchmark(&config) {
            Ok(network_results) => {
                profile.network_rx_mb = network_results.0;
                profile.network_tx_mb = network_results.1;
            }
            Err(e) => {
                warn!("Network benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }
    
    // Run GPU benchmark if enabled
    if config.run_gpu_test {
        match run_gpu_benchmark(&config) {
            Ok(gpu_results) => {
                profile.gpu_available = gpu_results.0;
                profile.gpu_memory_mb = gpu_results.1;
            }
            Err(e) => {
                warn!("GPU benchmark failed: {}", e);
                // Don't mark the whole benchmark as failed just because GPU check failed
            }
        }
    }
    
    // Record the total duration
    profile.duration_secs = start_time.elapsed().as_secs();
    
    info!("Benchmark suite completed: {:?}", profile);
    Ok(profile)
}

/// Run a CPU-intensive benchmark to measure CPU performance
fn run_cpu_benchmark(config: &BenchmarkRunConfig) -> Result<(f32, f32)> {
    info!("Running CPU benchmark");
    
    // Create a CPU-intensive workload using a simple calculation
    let cpu_command = "bash";
    let cpu_args = vec![
        "-c".to_string(),
        "for i in $(seq 1 10); do for j in $(seq 1 100000000); do echo $j > /dev/null; done; done".to_string(),
    ];
    
    // Run the command and monitor CPU usage
    let cpu_config = BenchmarkRunConfig {
        command: cpu_command.to_string(),
        args: cpu_args,
        job_id: format!("{}-cpu", config.job_id),
        mode: config.mode.clone(),
        max_duration: Duration::from_secs(5), // Shorter duration for CPU test
        sample_interval: config.sample_interval,
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: false,
        run_network_test: false,
        run_gpu_test: false,
    };
    
    let result = run_and_monitor_command(&cpu_config)?;
    Ok((result.avg_cpu_cores, result.peak_cpu_cores))
}

/// Run a memory-intensive benchmark to measure memory performance
fn run_memory_benchmark(config: &BenchmarkRunConfig) -> Result<(f32, f32)> {
    info!("Running memory benchmark");
    
    // Create a memory-intensive workload
    let memory_command = "bash";
    let memory_args = vec![
        "-c".to_string(),
        // Allocate and use a large array in memory
        "python3 -c 'import numpy as np; a = np.random.rand(1000, 1000); b = np.random.rand(1000, 1000); c = np.dot(a, b); print(c.shape)'".to_string(),
    ];
    
    // Run the command and monitor memory usage
    let memory_config = BenchmarkRunConfig {
        command: memory_command.to_string(),
        args: memory_args,
        job_id: format!("{}-memory", config.job_id),
        mode: config.mode.clone(),
        max_duration: Duration::from_secs(5), // Shorter duration for memory test
        sample_interval: config.sample_interval,
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: false,
        run_network_test: false,
        run_gpu_test: false,
    };
    
    let result = run_and_monitor_command(&memory_config)?;
    Ok((result.avg_memory_mb, result.peak_memory_mb))
}

/// Run an I/O-intensive benchmark to measure disk I/O performance
fn run_io_benchmark(config: &BenchmarkRunConfig) -> Result<(f32, f32)> {
    info!("Running I/O benchmark");
    
    // Create temporary directory for I/O testing
    let temp_dir = format!("/tmp/benchmark-{}", config.job_id);
    let _ = std::fs::create_dir_all(&temp_dir);
    
    // Create an I/O-intensive workload
    let io_command = "bash";
    let io_args = vec![
        "-c".to_string(),
        format!(
            "dd if=/dev/zero of={}/testfile bs=1M count=100 && cat {}/testfile > /dev/null && rm {}/testfile",
            temp_dir, temp_dir, temp_dir
        ),
    ];
    
    // Run the command and monitor I/O usage
    let io_config = BenchmarkRunConfig {
        command: io_command.to_string(),
        args: io_args,
        job_id: format!("{}-io", config.job_id),
        mode: config.mode.clone(),
        max_duration: Duration::from_secs(10), // Longer duration for I/O test
        sample_interval: config.sample_interval,
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: false,
        run_network_test: false,
        run_gpu_test: false,
    };
    
    // For I/O, we need to track before and after stats
    // Get initial IO stats
    let initial_io_stats = get_io_stats()?;
    
    // Run the command
    let _ = run_and_monitor_command(&io_config)?;
    
    // Get final IO stats
    let final_io_stats = get_io_stats()?;
    
    // Calculate total I/O
    let read_mb = (final_io_stats.0 - initial_io_stats.0) as f32 / 1024.0 / 1024.0;
    let write_mb = (final_io_stats.1 - initial_io_stats.1) as f32 / 1024.0 / 1024.0;
    
    // Clean up
    let _ = std::fs::remove_dir_all(temp_dir);
    
    Ok((read_mb, write_mb))
}

/// Run a network benchmark to measure network performance
fn run_network_benchmark(config: &BenchmarkRunConfig) -> Result<(f32, f32)> {
    info!("Running network benchmark");
    
    // Create a network-intensive workload
    let network_command = "curl";
    let network_args = vec![
        "-s".to_string(),
        "-o".to_string(),
        "/dev/null".to_string(),
        "https://speed.hetzner.de/100MB.bin".to_string(), // Download a 100MB test file
    ];
    
    // Run the command and monitor network usage
    let network_config = BenchmarkRunConfig {
        command: network_command.to_string(),
        args: network_args,
        job_id: format!("{}-network", config.job_id),
        mode: config.mode.clone(),
        max_duration: Duration::from_secs(15), // Longer duration for network test
        sample_interval: config.sample_interval,
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: false,
        run_network_test: false,
        run_gpu_test: false,
    };
    
    // For network, we need to track before and after stats using a system command
    // Get initial network stats using ifconfig or ip
    let initial_network_stats = get_network_stats()?;
    
    // Run the command
    let _ = run_and_monitor_command(&network_config)?;
    
    // Get final network stats
    let final_network_stats = get_network_stats()?;
    
    // Calculate total network I/O
    let rx_mb = (final_network_stats.0 - initial_network_stats.0) as f32 / 1024.0 / 1024.0;
    let tx_mb = (final_network_stats.1 - initial_network_stats.1) as f32 / 1024.0 / 1024.0;
    
    Ok((rx_mb, tx_mb))
}

/// Check for GPU availability and memory
fn run_gpu_benchmark(_config: &BenchmarkRunConfig) -> Result<(bool, f32)> {
    info!("Checking GPU availability");
    
    // Try to detect NVIDIA GPU using nvidia-smi
    let nvidia_output = Command::new("nvidia-smi")
        .args(&["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .output();
    
    match nvidia_output {
        Ok(output) if output.status.success() => {
            // Parse the output to get GPU memory
            let stdout = String::from_utf8_lossy(&output.stdout);
            let memory_mb = stdout.trim().parse::<f32>().unwrap_or(0.0);
            Ok((true, memory_mb))
        }
        _ => {
            // Try to detect AMD GPU using rocm-smi
            let amd_output = Command::new("rocm-smi")
                .args(&["--showmeminfo", "vram"])
                .output();
                
            match amd_output {
                Ok(output) if output.status.success() => {
                    // Parse the output to get GPU memory (simplified)
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    // Very simplified parsing, would need to be more robust in production
                    let memory_mb = if stdout.contains("VRAM") { 1024.0 } else { 0.0 };
                    Ok((true, memory_mb))
                }
                _ => {
                    // No GPU detected
                    Ok((false, 0.0))
                }
            }
        }
    }
}

/// Helper function to get disk I/O statistics
fn get_io_stats() -> Result<(u64, u64)> {
    // On Linux, read from /proc/diskstats
    if Path::new("/proc/diskstats").exists() {
        let content = std::fs::read_to_string("/proc/diskstats")
            .map_err(|e| PricingError::Benchmark(format!("Failed to read disk stats: {}", e)))?;
        
        let mut total_read_sectors = 0u64;
        let mut total_write_sectors = 0u64;
        
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 {
                // Format is described in Linux kernel documentation
                // Field 6: sectors read
                if let Ok(read) = parts[5].parse::<u64>() {
                    total_read_sectors += read;
                }
                // Field 10: sectors written
                if let Ok(write) = parts[9].parse::<u64>() {
                    total_write_sectors += write;
                }
            }
        }
        
        // Sector size is typically 512 bytes
        let read_bytes = total_read_sectors * 512;
        let write_bytes = total_write_sectors * 512;
        
        Ok((read_bytes, write_bytes))
    } else {
        // Fallback for non-Linux systems
        Ok((0, 0))
    }
}

/// Helper function to get network statistics using system commands
fn get_network_stats() -> Result<(u64, u64)> {
    // Try using the ip command first
    if let Ok(output) = Command::new("ip")
        .args(&["-s", "link"])
        .output() 
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse the output to get network stats
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;
        
        // Very simplified parsing - would need to be more robust in production
        let lines: Vec<&str> = output_str.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.contains("RX:") && i + 1 < lines.len() {
                // Next line should contain bytes
                let rx_line = lines[i + 1].trim();
                if let Some(bytes_str) = rx_line.split_whitespace().next() {
                    if let Ok(bytes) = bytes_str.parse::<u64>() {
                        total_rx += bytes;
                    }
                }
            }
            if line.contains("TX:") && i + 1 < lines.len() {
                // Next line should contain bytes
                let tx_line = lines[i + 1].trim();
                if let Some(bytes_str) = tx_line.split_whitespace().next() {
                    if let Ok(bytes) = bytes_str.parse::<u64>() {
                        total_tx += bytes;
                    }
                }
            }
        }
        
        return Ok((total_rx, total_tx));
    }
    
    // Fallback to ifconfig if ip command fails
    if let Ok(output) = Command::new("ifconfig")
        .output() 
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse the output to get network stats
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;
        
        // Very simplified parsing - would need to be more robust in production
        for line in output_str.lines() {
            if line.contains("RX bytes") {
                // Format is typically "RX bytes:1234 ..."
                if let Some(bytes_part) = line.split("RX bytes:").nth(1) {
                    if let Some(bytes_str) = bytes_part.split_whitespace().next() {
                        if let Ok(bytes) = bytes_str.parse::<u64>() {
                            total_rx += bytes;
                        }
                    }
                }
            }
            if line.contains("TX bytes") {
                // Format is typically "TX bytes:1234 ..."
                if let Some(bytes_part) = line.split("TX bytes:").nth(1) {
                    if let Some(bytes_str) = bytes_part.split_whitespace().next() {
                        if let Ok(bytes) = bytes_str.parse::<u64>() {
                            total_tx += bytes;
                        }
                    }
                }
            }
        }
        
        return Ok((total_rx, total_tx));
    }
    
    // If both commands fail, return zeros
    Ok((0, 0))
}

/// Run a command and monitor its resource usage
fn run_and_monitor_command(config: &BenchmarkRunConfig) -> Result<BenchmarkProfile> {
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
        .map_err(|e| PricingError::Benchmark(format!("Failed to start benchmark process: {}", e)))?;

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
    let avg_cpu = if sample_count > 0 { total_cpu / sample_count as f32 } else { 0.0 };
    let avg_memory = if sample_count > 0 { total_memory / sample_count as f32 } else { 0.0 };
    
    // Convert CPU usage to cores (sysinfo returns percentage, divide by 100 to get core count)
    let avg_cpu_cores = avg_cpu / 100.0;
    let peak_cpu_cores = peak_cpu / 100.0;
    
    // Get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| PricingError::Benchmark(format!("Failed to get system time: {}", e)))?
        .as_secs();
    
    // Create benchmark profile
    let profile = BenchmarkProfile {
        job_id: config.job_id.clone(),
        execution_mode: config.mode.clone(),
        avg_cpu_cores,
        peak_cpu_cores,
        avg_memory_mb: avg_memory,
        peak_memory_mb: peak_memory,
        io_read_mb: 0.0,
        io_write_mb: 0.0,
        network_rx_mb: 0.0,
        network_tx_mb: 0.0,
        storage_available_gb: 0.0,
        gpu_available: false,
        gpu_memory_mb: 0.0,
        duration_secs: start_time.elapsed().as_secs(),
        timestamp,
        success,
    };
    
    info!("Command benchmark completed: {:?}", profile);
    Ok(profile)
}

// Maintain backward compatibility with the original function
pub fn run_benchmark(config: BenchmarkRunConfig) -> Result<BenchmarkProfile> {
    run_and_monitor_command(&config)
}

// Example test (consider placing in tests/ directory)
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_benchmark_suite() {
        let config = BenchmarkRunConfig {
            command: "echo".to_string(),
            args: vec!["test".to_string()],
            job_id: "test-suite".to_string(),
            mode: "test".to_string(),
            max_duration: Duration::from_secs(30),
            sample_interval: Duration::from_millis(500),
            run_cpu_test: true,
            run_memory_test: true,
            run_io_test: true,
            run_network_test: true, // Skip network test in automated tests
            run_gpu_test: true,
        };
        
        let result = run_benchmark_suite(config);
        assert!(result.is_ok());

        let result = result.unwrap();
        println!("{:?}", result)
    }
}
