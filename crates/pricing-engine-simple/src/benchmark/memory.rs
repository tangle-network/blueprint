// src/benchmark/memory.rs
//
// Memory benchmarking module for measuring memory performance

use crate::error::{PricingError, Result};
use blueprint_core::info;
use rand::Rng;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

use super::{BenchmarkRunConfig, MemoryAccessMode, MemoryBenchmarkResult, MemoryOperationType};

// Memory benchmark constants
const DEFAULT_BLOCK_SIZE_KB: u64 = 1024; // 1MB block size
const DEFAULT_TOTAL_SIZE_MB: u64 = 1024; // 1GB total size
const DEFAULT_ACCESS_MODE: MemoryAccessMode = MemoryAccessMode::Sequential;
const DEFAULT_OPERATION_TYPE: MemoryOperationType = MemoryOperationType::Write;
const MEMORY_SAMPLE_INTERVAL_MS: u64 = 100; // Sample memory usage every 100ms

/// Run a memory-intensive benchmark to measure memory performance
pub fn run_memory_benchmark(config: &BenchmarkRunConfig) -> Result<MemoryBenchmarkResult> {
    info!("Running memory benchmark");

    // Configuration parameters
    let block_size_kb = DEFAULT_BLOCK_SIZE_KB;
    let total_size_mb = DEFAULT_TOTAL_SIZE_MB;
    let access_mode = DEFAULT_ACCESS_MODE;
    let operation_type = DEFAULT_OPERATION_TYPE;

    // Calculate sizes in bytes
    let block_size_bytes = block_size_kb * 1024;
    let total_size_bytes = total_size_mb * 1024 * 1024;

    // Number of operations to perform
    let num_operations = total_size_bytes / block_size_bytes;

    info!("Memory benchmark configuration:");
    info!("  Block size: {} KB", block_size_kb);
    info!("  Total size: {} MB", total_size_mb);
    info!("  Access mode: {:?}", access_mode);
    info!("  Operation type: {:?}", operation_type);
    info!("  Number of operations: {}", num_operations);

    // Determine number of threads to use (use half of available cores)
    let num_cores = num_cpus::get();
    let num_threads = std::cmp::max(1, num_cores / 2);

    // Divide operations among threads
    let ops_per_thread = num_operations / num_threads as u64;

    info!("Using {} threads for memory benchmark", num_threads);

    // Create shared counters for tracking performance
    let total_operations = Arc::new(Mutex::new(0u64));
    let total_bytes = Arc::new(Mutex::new(0u64));
    let total_latency_ns = Arc::new(Mutex::new(0u64));

    // For tracking memory usage
    let memory_samples = Arc::new(Mutex::new(Vec::new()));
    let benchmark_running = Arc::new(Mutex::new(true));

    // Start memory monitoring in a separate thread
    let memory_samples_clone = Arc::clone(&memory_samples);
    let benchmark_running_clone = Arc::clone(&benchmark_running);
    let memory_monitor_handle = thread::spawn(move || {
        let pid = process::id();
        let mut sys = System::new();

        while *benchmark_running_clone.lock().unwrap() {
            sys.refresh_all();

            if let Some(process) = sys.process(Pid::from_u32(pid)) {
                // Memory usage in bytes, convert to MB
                let memory_used_mb = process.memory() as f32 / 1024.0 / 1024.0;
                memory_samples_clone.lock().unwrap().push(memory_used_mb);
            }

            thread::sleep(Duration::from_millis(MEMORY_SAMPLE_INTERVAL_MS));
        }
    });

    // Start time measurement
    let start_time = Instant::now();

    // Create and start worker threads
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let thread_ops = ops_per_thread;
        let thread_total_ops = Arc::clone(&total_operations);
        let thread_total_bytes = Arc::clone(&total_bytes);
        let thread_total_latency = Arc::clone(&total_latency_ns);
        let thread_access_mode = access_mode;
        let thread_operation_type = operation_type;
        let thread_block_size_bytes = block_size_bytes;
        let thread_max_duration = config.max_duration;
        let thread_start_time = start_time;

        let handle = thread::spawn(move || {
            // Allocate memory block for this thread
            let mut buffer = vec![0u8; thread_block_size_bytes as usize];

            // Initialize buffer with some data
            let mut rng = rand::thread_rng();
            for byte in buffer.iter_mut() {
                *byte = (rng.gen_range(0..256)) as u8;
            }

            // Prepare for random access if needed
            let mut offsets = Vec::new();
            if thread_access_mode == MemoryAccessMode::Random {
                // Pre-generate random offsets
                for _ in 0..thread_ops {
                    let offset = rng.gen_range(0..buffer.len());
                    offsets.push(offset);
                }
            }

            let mut thread_ops_count = 0;
            let mut thread_latency_ns = 0;

            // Perform memory operations
            for _ in 0..thread_ops {
                let op_start = Instant::now();

                match thread_operation_type {
                    MemoryOperationType::Read => {
                        match thread_access_mode {
                            MemoryAccessMode::Sequential => {
                                // Sequential read - read entire buffer
                                let mut sum = 0u8;
                                for &byte in &buffer {
                                    sum = sum.wrapping_add(byte);
                                }
                                // Use sum to prevent compiler optimization
                                if sum == 123 {
                                    buffer[0] = sum;
                                }
                            }
                            MemoryAccessMode::Random => {
                                // Random read - read from random offsets
                                let mut sum = 0u8;
                                let offset_len = std::cmp::min(offsets.len(), thread_ops as usize);
                                for offset in offsets.iter().take(offset_len) {
                                    sum = sum.wrapping_add(buffer[*offset]);
                                }
                                // Use sum to prevent compiler optimization
                                if sum == 123 {
                                    buffer[0] = sum;
                                }
                            }
                        }
                    }
                    MemoryOperationType::Write => {
                        match thread_access_mode {
                            MemoryAccessMode::Sequential => {
                                // Sequential write - write to entire buffer
                                for (j, byte) in buffer.iter_mut().enumerate() {
                                    *byte = (j % 256) as u8;
                                }
                            }
                            MemoryAccessMode::Random => {
                                // Random write - write to random offsets
                                let offset_len = std::cmp::min(offsets.len(), thread_ops as usize);
                                for offset in offsets.iter().take(offset_len) {
                                    buffer[*offset] = (thread_id % 256) as u8;
                                }
                            }
                        }
                    }
                    MemoryOperationType::None => {
                        // Just iterate through memory locations without reading or writing
                        match thread_access_mode {
                            MemoryAccessMode::Sequential => {
                                // Sequential iteration
                                for _ in 0..buffer.len() {
                                    // No-op, just iterate
                                    std::hint::black_box(());
                                }
                            }
                            MemoryAccessMode::Random => {
                                // Random iteration
                                let offset_len = std::cmp::min(offsets.len(), thread_ops as usize);
                                for offset in offsets.iter().take(offset_len) {
                                    // No-op, just access random index
                                    std::hint::black_box(offset);
                                }
                            }
                        }
                    }
                }

                // Measure latency for this operation
                let op_duration = op_start.elapsed();
                thread_latency_ns += op_duration.as_nanos() as u64;
                thread_ops_count += 1;

                // Check if we've reached the maximum duration
                if thread_start_time.elapsed() > thread_max_duration {
                    break;
                }
            }

            // Update shared counters
            let mut total_ops = thread_total_ops.lock().unwrap();
            *total_ops += thread_ops_count;

            let mut total_b = thread_total_bytes.lock().unwrap();
            *total_b += thread_ops_count * thread_block_size_bytes;

            let mut total_lat = thread_total_latency.lock().unwrap();
            *total_lat += thread_latency_ns;

            (thread_ops_count, thread_latency_ns)
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    let mut thread_results = Vec::new();
    for handle in handles {
        match handle.join() {
            Ok(result) => thread_results.push(result),
            Err(e) => {
                // Signal memory monitoring thread to stop
                *benchmark_running.lock().unwrap() = false;

                return Err(PricingError::Benchmark(format!(
                    "Thread panicked during memory benchmark: {:?}",
                    e
                )));
            }
        }
    }

    // Signal memory monitoring thread to stop
    *benchmark_running.lock().unwrap() = false;

    // Wait for memory monitoring thread to finish
    if let Err(e) = memory_monitor_handle.join() {
        return Err(PricingError::Benchmark(format!(
            "Memory monitoring thread panicked: {:?}",
            e
        )));
    }

    // Calculate total duration
    let duration = start_time.elapsed();
    let duration_ms = duration.as_millis() as u64;

    // Get final counters
    let total_ops = *total_operations.lock().unwrap();
    let total_b = *total_bytes.lock().unwrap();
    let total_lat = *total_latency_ns.lock().unwrap();

    // Calculate memory usage statistics
    let memory_samples = memory_samples.lock().unwrap();
    let (avg_memory_mb, peak_memory_mb) = if !memory_samples.is_empty() {
        let sum: f32 = memory_samples.iter().sum();
        let avg = sum / memory_samples.len() as f32;
        let peak = *memory_samples
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        (avg, peak)
    } else {
        (0.0, 0.0)
    };

    // Calculate performance metrics
    let operations_per_second = if duration_ms > 0 {
        (total_ops as f64 * 1000.0 / duration_ms as f64) as f32
    } else {
        0.0
    };

    let transfer_rate_mb_s = if duration_ms > 0 {
        (total_b as f64 / 1024.0 / 1024.0 * 1000.0 / duration_ms as f64) as f32
    } else {
        0.0
    };

    let avg_latency_ns = if total_ops > 0 {
        (total_lat as f64 / total_ops as f64) as f32
    } else {
        0.0
    };

    // Log results
    info!("Memory benchmark completed:");
    info!("  Duration: {} ms", duration_ms);
    info!("  Operations: {}", total_ops);
    info!("  Operations/sec: {:.2}", operations_per_second);
    info!("  Transfer rate: {:.2} MB/s", transfer_rate_mb_s);
    info!("  Avg latency: {:.2} ns", avg_latency_ns);
    info!("  Avg memory usage: {:.2} MB", avg_memory_mb);
    info!("  Peak memory usage: {:.2} MB", peak_memory_mb);

    // Create and return result
    Ok(MemoryBenchmarkResult {
        avg_memory_mb,
        peak_memory_mb,
        block_size_kb,
        total_size_mb,
        operations_per_second,
        transfer_rate_mb_s,
        access_mode,
        operation_type,
        latency_ns: avg_latency_ns,
        duration_ms,
    })
}
