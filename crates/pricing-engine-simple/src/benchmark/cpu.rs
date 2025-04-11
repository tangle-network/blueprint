// src/benchmark/cpu.rs
//
// CPU benchmarking module based on sysbench's CPU test

use crate::error::Result;
use blueprint_core::info;
use num_cpus;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::System;

use super::BenchmarkRunConfig;

// CPU benchmark constants
pub const DEFAULT_MAX_PRIME: u64 = 20000;

/// Detailed results from CPU benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuBenchmarkResult {
    pub num_cores_detected: usize, // Number of CPU cores detected on the system
    pub avg_cores_used: f32,       // Average number of CPU cores used during benchmark
    pub avg_usage_percent: f32,    // Average CPU usage as a percentage
    pub peak_cores_used: f32,      // Peak number of CPU cores used during benchmark
    pub peak_usage_percent: f32,   // Peak CPU usage as a percentage
    pub benchmark_duration_ms: u64, // Time taken to run the benchmark in milliseconds
    pub primes_found: u64,         // Number of prime numbers found
    pub max_prime: u64,            // Upper limit for prime number calculation
    pub primes_per_second: f32,    // Rate of prime number calculation
    pub cpu_model: String,         // CPU model information if available
    pub cpu_frequency_mhz: f32,    // CPU frequency in MHz if available
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

/// Run a CPU-intensive benchmark to measure CPU performance
///
/// This implementation calculates prime numbers up to a certain limit
pub fn run_cpu_benchmark(_config: &BenchmarkRunConfig) -> Result<(f32, f32, CpuBenchmarkResult)> {
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

        let peak_usage = cpu_usage_samples
            .iter()
            .fold(0.0f32, |max, &val| max.max(val));

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
        cpu_result.avg_usage_percent,
        cpu_result.avg_cores_used,
        cpu_result.peak_usage_percent,
        cpu_result.peak_cores_used
    );

    info!(
        "CPU Model: {}, Frequency: {:.2} MHz",
        cpu_result.cpu_model, cpu_result.cpu_frequency_mhz
    );

    info!(
        "Performance: {:.2} primes/second",
        cpu_result.primes_per_second
    );

    Ok((
        cpu_result.avg_cores_used,
        cpu_result.peak_cores_used,
        cpu_result,
    ))
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
pub fn calculate_primes(max_prime: u64) -> u64 {
    calculate_primes_range(2, max_prime)
}
