// src/benchmark.rs
use crate::error::{PricingError, Result};
use blueprint_core::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use sysinfo::{Pid, System};
use num_cpus;

// CPU BENCHMARK CONSTANTS
const DEFAULT_MAX_PRIME: u64 = 20000;

/// Detailed results from CPU benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuBenchmarkResult {
    pub num_cores_detected: usize,      // Number of CPU cores detected on the system
    pub avg_cores_used: f32,            // Average number of CPU cores used during benchmark
    pub avg_usage_percent: f32,         // Average CPU usage as a percentage
    pub peak_cores_used: f32,           // Peak number of CPU cores used during benchmark
    pub peak_usage_percent: f32,        // Peak CPU usage as a percentage
    pub benchmark_duration_ms: u64,     // Time taken to run the benchmark in milliseconds
    pub primes_found: u64,              // Number of prime numbers found
    pub max_prime: u64,                 // Upper limit for prime number calculation
    pub primes_per_second: f32,         // Rate of prime number calculation
    pub cpu_model: String,              // CPU model information if available
    pub cpu_frequency_mhz: f32,         // CPU frequency in MHz if available
}

impl Default for CpuBenchmarkResult {
    fn default() -> Self {
        Self {
            num_cores_detected: 0,
            avg_cores_used: 0.0,
            avg_usage_percent: 0.0,
            peak_cores_used: 0.0,
            peak_usage_percent: 0.0,
            benchmark_duration_ms: 0,
            primes_found: 0,
            max_prime: 0,
            primes_per_second: 0.0,
            cpu_model: "Unknown".to_string(),
            cpu_frequency_mhz: 0.0,
        }
    }
}

// GPU BENCHMARK CONSTANTS
const DEFAULT_UNKNOWN_GPU_MEMORY: f32 = 512.0;
const DEFAULT_INTEL_GPU_MEMORY: f32 = 1024.0;
const DEFAULT_NVIDIA_GPU_MEMORY: f32 = 2048.0;
const DEFAULT_AMD_GPU_MEMORY: f32 = 2048.0;

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
    pub timestamp: u64, // Unix timestamp
    pub success: bool,  // Indicate if benchmark command finished successfully
    pub cpu_details: Option<CpuBenchmarkResult>, // Detailed CPU benchmark results
}

// Configuration specific to a single benchmark run
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
) -> Result<BenchmarkProfile> {
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
    if let Ok(output) = Command::new("df").args(&["-h", "/"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        // Parse the output to get available space
        for line in output_str.lines().skip(1) {
            // Skip header line
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                // Format is typically: Filesystem Size Used Avail Use% Mounted on
                let avail = parts[3];
                if avail.ends_with('G') {
                    // Extract the numeric part
                    if let Ok(gb) = avail.trim_end_matches('G').parse::<f32>() {
                        profile.storage_available_gb = gb;
                        break;
                    }
                }
            }
        }
    }

    // Create a config for individual benchmark runs
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
            Ok((read_mb, write_mb)) => {
                profile.io_read_mb = read_mb;
                profile.io_write_mb = write_mb;
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
            Ok((gpu_available, gpu_memory_mb)) => {
                profile.gpu_available = gpu_available;
                profile.gpu_memory_mb = gpu_memory_mb;
            }
            Err(e) => {
                warn!("GPU benchmark failed: {}", e);
                profile.success = false;
            }
        }
    }

    // Record the total duration
    profile.duration_secs = start_time.elapsed().as_secs();

    info!("Benchmark suite completed: {:?}", profile);
    Ok(profile)
}

/// Run a CPU-intensive benchmark to measure CPU performance
///
/// This implementation calculates prime numbers up to a certain limit
fn run_cpu_benchmark(_config: &BenchmarkRunConfig) -> Result<(f32, f32, CpuBenchmarkResult)> {
    info!("Running CPU benchmark (prime number calculation)");
    
    let num_cores = num_cpus::get();
    info!("Detected {} CPU cores", num_cores);
    
    let system = Arc::new(Mutex::new(System::new_all()));
    
    let total_primes = Arc::new(Mutex::new(0u64));
    
    let start = Instant::now();
    
    let mut handles = Vec::new();
    
    let primes_per_thread = DEFAULT_MAX_PRIME / num_cores as u64;
    
    for i in 0..num_cores {
        let thread_total_primes = Arc::clone(&total_primes);
        let thread_system = Arc::clone(&system);
        
        let start_num = 2 + i as u64 * primes_per_thread;
        let end_num = if i == num_cores - 1 {
            DEFAULT_MAX_PRIME
        } else {
            start_num + primes_per_thread - 1
        };
        
        let handle = thread::spawn(move || {
            let primes_found = calculate_primes_range(start_num, end_num);
            
            let mut total = thread_total_primes.lock().unwrap();
            *total += primes_found;
            
            let usage = {
                let mut sys = thread_system.lock().unwrap();
                sys.refresh_cpu();
                sys.global_cpu_info().cpu_usage()
            };
            
            (primes_found, usage)
        });
        
        handles.push(handle);
    }
    
    let monitoring_system = Arc::clone(&system);
    let monitoring_handle = thread::spawn(move || {
        let mut cpu_usage_samples = Vec::new();
        let monitor_start = Instant::now();
        
        while monitor_start.elapsed() < Duration::from_secs(10) {
            {
                let mut sys = monitoring_system.lock().unwrap();
                sys.refresh_cpu();
                cpu_usage_samples.push(sys.global_cpu_info().cpu_usage());
            }
            thread::sleep(Duration::from_millis(100));
        }
        
        let avg_usage = if !cpu_usage_samples.is_empty() {
            cpu_usage_samples.iter().sum::<f32>() / cpu_usage_samples.len() as f32
        } else {
            0.0
        };
        
        let peak_usage = cpu_usage_samples.iter().fold(0.0f32, |max, &val| max.max(val));
        
        (avg_usage, peak_usage)
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let (avg_usage, peak_usage) = monitoring_handle.join().unwrap();
    
    let count = *total_primes.lock().unwrap();
    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_millis() as u64;
    
    let avg_cpu_cores = avg_usage / 100.0 * num_cores as f32;
    let peak_cpu_cores = peak_usage / 100.0 * num_cores as f32;
    
    let mut cpu_model = "Unknown".to_string();
    let mut cpu_frequency_mhz = 0.0;
    
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        if let Some(model_line) = cpuinfo.lines().find(|line| line.starts_with("model name")) {
            if let Some(model) = model_line.split(':').nth(1) {
                cpu_model = model.trim().to_string();
            }
        }
        
        if let Some(freq_line) = cpuinfo.lines().find(|line| line.starts_with("cpu MHz")) {
            if let Some(freq_str) = freq_line.split(':').nth(1) {
                if let Ok(freq) = freq_str.trim().parse::<f32>() {
                    cpu_frequency_mhz = freq;
                }
            }
        }
    }
    
    let cpu_result = CpuBenchmarkResult {
        num_cores_detected: num_cores,
        avg_cores_used: avg_cpu_cores,
        avg_usage_percent: avg_usage,
        peak_cores_used: peak_cpu_cores,
        peak_usage_percent: peak_usage,
        benchmark_duration_ms: elapsed_ms,
        primes_found: count,
        max_prime: DEFAULT_MAX_PRIME,
        primes_per_second: (count as f32) / (elapsed.as_secs_f32()),
        cpu_model,
        cpu_frequency_mhz,
    };
    
    info!(
        "CPU Benchmark: Found {} prime numbers up to {} in {:?}",
        cpu_result.primes_found, cpu_result.max_prime, elapsed
    );
    
    info!(
        "CPU Usage: Avg {:.2}% ({:.2} cores), Peak {:.2}% ({:.2} cores)",
        cpu_result.avg_usage_percent, cpu_result.avg_cores_used, 
        cpu_result.peak_usage_percent, cpu_result.peak_cores_used
    );
    
    info!(
        "CPU Model: {}, Frequency: {:.2} MHz",
        cpu_result.cpu_model, cpu_result.cpu_frequency_mhz
    );
    
    info!(
        "Performance: {:.2} primes/second",
        cpu_result.primes_per_second
    );
    
    // Return average and peak CPU cores used along with detailed results
    Ok((cpu_result.avg_cores_used, cpu_result.peak_cores_used, cpu_result))
}

/// Calculate prime numbers in a specific range
/// This is the core CPU benchmark algorithm based on sysbench
fn calculate_primes_range(start: u64, end: u64) -> u64 {
    let mut count = 0;
    
    // Ensure start is odd (except if start is 2)
    let start_num = if start == 2 {
        // 2 is prime
        count += 1;
        3
    } else if start % 2 == 0 {
        start + 1
    } else {
        start
    };
    
    // Check odd numbers from start_num to end
    for c in (start_num..=end).step_by(2) {
        let sqrt_c = (c as f64).sqrt() as u64;
        let mut is_prime = true;
        
        // Check if c is divisible by any number from 2 to sqrt(c)
        for i in 2..=sqrt_c {
            if c % i == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            count += 1;
        }
    }
    
    count
}

/// Calculate prime numbers up to max_prime
/// This is the core CPU benchmark algorithm based on sysbench
fn calculate_primes(max_prime: u64) -> u64 {
    calculate_primes_range(2, max_prime)
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
            info!("Detected NVIDIA GPU with {} MB memory", memory_mb);
            return Ok((true, memory_mb));
        }
        _ => {
            debug!("No NVIDIA GPU detected, checking other GPU types");
        }
    }

    // Try to detect AMD GPU using rocm-smi
    let amd_output = Command::new("rocm-smi")
        .args(&["--showmeminfo", "vram"])
        .output();

    match amd_output {
        Ok(output) if output.status.success() => {
            // Parse the output to get GPU memory (simplified)
            let stdout = String::from_utf8_lossy(&output.stdout);
            // More robust parsing for AMD GPUs
            let memory_mb = parse_amd_gpu_memory(&stdout);
            info!("Detected AMD GPU with {} MB memory", memory_mb);
            return Ok((true, memory_mb));
        }
        _ => {
            debug!("No AMD GPU detected, checking other GPU types");
        }
    }

    // Try to detect Intel GPU using intel_gpu_top
    let intel_output = Command::new("intel_gpu_top")
        .args(&["-l", "1"]) // List info once
        .output();

    match intel_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse memory from intel_gpu_top output
            let memory_mb = parse_intel_gpu_memory(&stdout);
            info!("Detected Intel GPU with {} MB memory", memory_mb);
            return Ok((true, memory_mb));
        }
        _ => {
            debug!("No Intel GPU detected via intel_gpu_top, trying alternative methods");
        }
    }

    // Try to detect Intel GPU using lspci
    let lspci_output = Command::new("lspci").args(&["-v"]).output();

    match lspci_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("Intel Corporation")
                && (stdout.contains("VGA compatible controller")
                    || stdout.contains("Display controller")
                    || stdout.contains("3D controller"))
            {
                // Intel GPU detected, but memory size unknown
                // Estimate based on common values
                info!("Detected Intel integrated GPU, estimating memory");
                return Ok((true, DEFAULT_INTEL_GPU_MEMORY)); // Estimate 1GB for integrated GPUs
            }
        }
        _ => {
            debug!("No Intel GPU detected via lspci");
        }
    }

    // Check for GPUs using glxinfo
    let glxinfo_output = Command::new("glxinfo").args(&["-B"]).output();

    match glxinfo_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            // First try to extract the video memory directly
            let memory_mb = parse_glxinfo_memory(&stdout);
            if memory_mb > 0.0 {
                println!("Detected GPU with {} MB memory via glxinfo", memory_mb);
                return Ok((true, memory_mb));
            }

            // If memory parsing failed, try to identify the GPU from renderer string
            if stdout.contains("OpenGL renderer") {
                // Extract vendor and renderer info
                let renderer_line = stdout
                    .lines()
                    .find(|line| line.contains("OpenGL renderer string"))
                    .unwrap_or("");

                println!("Detected GPU via glxinfo: {}", renderer_line);

                // Estimate memory based on vendor if we couldn't parse it directly
                if renderer_line.contains("NVIDIA") {
                    return Ok((true, DEFAULT_NVIDIA_GPU_MEMORY)); // Estimate for NVIDIA
                } else if renderer_line.contains("AMD") || renderer_line.contains("ATI") {
                    return Ok((true, DEFAULT_AMD_GPU_MEMORY)); // Estimate for AMD
                } else if renderer_line.contains("Intel") {
                    return Ok((true, DEFAULT_INTEL_GPU_MEMORY)); // Estimate for Intel
                } else {
                    return Ok((true, DEFAULT_UNKNOWN_GPU_MEMORY)); // Conservative estimate for unknown
                }
            }
        }
        _ => {
            debug!("No GPU detected via glxinfo");
        }
    }

    // Check for any GPU using lshw
    let lshw_output = Command::new("lshw")
        .args(&["-C", "display", "-short"])
        .output();

    match lshw_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty()
                && (stdout.contains("display") || stdout.contains("VGA") || stdout.contains("3D"))
            {
                // Some GPU detected, but details unknown
                info!("Detected GPU via lshw, but memory size unknown");
                return Ok((true, DEFAULT_UNKNOWN_GPU_MEMORY)); // Conservative estimate
            }
        }
        _ => {
            debug!("No GPU detected via lshw");
        }
    }

    // No GPU detected after all checks
    info!("No GPU detected after all detection methods");
    Ok((false, 0.0))
}

/// Helper function to parse AMD GPU memory from rocm-smi output
fn parse_amd_gpu_memory(output: &str) -> f32 {
    // Example output format:
    // ======= ROCm System Management Interface =======
    // ===== GPU Memory Usage (Visualization) =====
    // GPU[0] : [VRAM Total: 8176 MB, VRAM Used: 142 MB, VRAM Free: 8034 MB]

    for line in output.lines() {
        if line.contains("VRAM Total:") {
            // Extract the memory value
            if let Some(start) = line.find("VRAM Total:") {
                let memory_part = &line[start + "VRAM Total:".len()..];
                if let Some(end) = memory_part.find("MB") {
                    let memory_str = memory_part[..end].trim();
                    if let Ok(memory) = memory_str.parse::<f32>() {
                        return memory;
                    }
                }
            }
        }
    }

    DEFAULT_AMD_GPU_MEMORY
}

/// Helper function to parse Intel GPU memory from intel_gpu_top output
fn parse_intel_gpu_memory(output: &str) -> f32 {
    // Example output might contain memory information
    // Look for lines with memory information

    for line in output.lines() {
        if line.contains("memory") && line.contains("MB") {
            // Very simplified parsing - would need to be adjusted based on actual output format
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "MB" && i > 0 {
                    if let Ok(memory) = parts[i - 1].parse::<f32>() {
                        return memory;
                    }
                }
            }
        }
    }

    DEFAULT_INTEL_GPU_MEMORY
}

/// Helper function to parse GPU memory from glxinfo output
fn parse_glxinfo_memory(output: &str) -> f32 {
    // Look for "Video memory: XXXMB" in the output
    for line in output.lines() {
        if line.contains("Video memory:") {
            // Extract the memory value
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let memory_str = parts[2].trim();
                // Remove "MB" suffix if present
                let memory_str = memory_str.trim_end_matches("MB");
                if let Ok(memory) = memory_str.parse::<f32>() {
                    return memory;
                }
            }
        }
    }

    0.0 // Default to zero, parsing failed
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
    if let Ok(output) = Command::new("ip").args(&["-s", "link"]).output() {
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
    if let Ok(output) = Command::new("ifconfig").output() {
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
        .map_err(|e| {
            PricingError::Benchmark(format!("Failed to start benchmark process: {}", e))
        })?;

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
        cpu_details: None,
    };

    info!("Command benchmark completed: {:?}", profile);
    Ok(profile)
}

// Maintain backward compatibility with the original function
pub fn run_benchmark(config: BenchmarkRunConfig) -> Result<BenchmarkProfile> {
    run_and_monitor_command(&config)
}
