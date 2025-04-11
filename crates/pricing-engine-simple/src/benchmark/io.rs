// src/benchmark/io.rs
//
// I/O benchmarking module for measuring disk I/O performance

use crate::error::{PricingError, Result};
use blueprint_core::info;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::BenchmarkRunConfig;
use super::utils;

// Constants for I/O benchmark
const DEFAULT_FILE_SIZE: u64 = 128 * 1024 * 1024; // 128 MB
const DEFAULT_BLOCK_SIZE: usize = 4096; // 4 KB - standard filesystem block size
const DEFAULT_NUM_FILES: usize = 2;
// Use a temporary directory that's guaranteed to be writable
const DEFAULT_TEST_DIR: &str = "/tmp/blueprint_io_benchmark";
const DEFAULT_FILE_PREFIX: &str = "test_file";
const FILE_CHECKSUM_LENGTH: usize = 4; // 4 bytes for CRC32
const FILE_OFFSET_LENGTH: usize = 8; // 8 bytes for offset

/// I/O test modes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IoTestMode {
    SeqWrite,   // Sequential write (file creation)
    SeqRewrite, // Sequential rewrite
    SeqRead,    // Sequential read
    RndRead,    // Random read
    RndWrite,   // Random write
    RndRw,      // Random read/write
}

/// Detailed results from I/O benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoBenchmarkResult {
    pub read_mb: f32,              // Total MB read
    pub write_mb: f32,             // Total MB written
    pub read_iops: f32,            // Read operations per second
    pub write_iops: f32,           // Write operations per second
    pub avg_read_latency_ms: f32,  // Average read latency in ms
    pub avg_write_latency_ms: f32, // Average write latency in ms
    pub max_read_latency_ms: f32,  // Maximum read latency in ms
    pub max_write_latency_ms: f32, // Maximum write latency in ms
    pub test_mode: IoTestMode,     // Test mode used
    pub block_size: usize,         // Block size used
    pub total_file_size: u64,      // Total file size
    pub num_files: usize,          // Number of files used
    pub duration_ms: u64,          // Test duration in milliseconds
}

/// Run an I/O-intensive benchmark to measure disk I/O performance
pub fn run_io_benchmark(_config: &BenchmarkRunConfig) -> Result<IoBenchmarkResult> {
    info!("Running I/O benchmark");

    // Get initial I/O stats for logging purposes
    let (initial_read_bytes, initial_write_bytes) = utils::get_io_stats()?;

    println!(
        "Initial I/O stats: Read: {} bytes, Write: {} bytes",
        initial_read_bytes, initial_write_bytes
    );

    // Clean up any existing test files first to ensure a clean state
    let _ = cleanup_test_files();

    // Run a comprehensive I/O benchmark with proper error handling
    let result = match run_comprehensive_io_benchmark() {
        Ok(result) => result,
        Err(e) => {
            // Ensure cleanup happens even if the benchmark fails
            let _ = cleanup_test_files();
            return Err(e);
        }
    };

    // Get final I/O stats for logging purposes
    let (final_read_bytes, final_write_bytes) = utils::get_io_stats()?;

    println!(
        "Final I/O stats: Read: {} bytes, Write: {} bytes",
        final_read_bytes, final_write_bytes
    );

    // Calculate I/O in MB from system stats (for logging only)
    let read_mb = (final_read_bytes - initial_read_bytes) as f32 / 1024.0 / 1024.0;
    let write_mb = (final_write_bytes - initial_write_bytes) as f32 / 1024.0 / 1024.0;

    println!(
        "I/O benchmark completed: Read: {:.2} MB, Write: {:.2} MB",
        read_mb, write_mb
    );

    // Log detailed benchmark results
    println!("I/O benchmark details:");
    println!("  Test mode: {:?}", result.test_mode);
    println!("  Block size: {} KB", result.block_size / 1024);
    println!(
        "  Total file size: {} MB",
        result.total_file_size / 1024 / 1024
    );
    println!("  Number of files: {}", result.num_files);
    println!("  Duration: {} ms", result.duration_ms);
    println!("  Read IOPS: {:.2}", result.read_iops);
    println!("  Write IOPS: {:.2}", result.write_iops);
    println!("  Avg read latency: {:.2} ms", result.avg_read_latency_ms);
    println!("  Avg write latency: {:.2} ms", result.avg_write_latency_ms);
    println!("  Max read latency: {:.2} ms", result.max_read_latency_ms);
    println!("  Max write latency: {:.2} ms", result.max_write_latency_ms);

    // Return the detailed benchmark results instead of just the system I/O stats
    Ok(result)
}

/// Run a comprehensive I/O benchmark with different I/O patterns
fn run_comprehensive_io_benchmark() -> Result<IoBenchmarkResult> {
    // Create test directory if it doesn't exist
    std::fs::create_dir_all(DEFAULT_TEST_DIR)
        .map_err(|e| PricingError::Benchmark(format!("Failed to create test directory: {}", e)))?;

    // Prepare test files
    prepare_test_files()?;

    // Run a mix of tests - we are only keeping the results of the random read/write test
    // let _seq_write_result = run_io_test(IoTestMode::SeqWrite)?;
    // let _seq_read_result = run_io_test(IoTestMode::SeqRead)?;
    // let _rnd_read_result = run_io_test(IoTestMode::RndRead)?;
    // let _rnd_write_result = run_io_test(IoTestMode::RndWrite)?;
    let rnd_rw_result = run_io_test(IoTestMode::RndRw)?;

    // Clean up test files
    cleanup_test_files()?;

    // Return the random read/write result as it's the most comprehensive
    Ok(rnd_rw_result)
}

/// Prepare test files for I/O benchmark
fn prepare_test_files() -> Result<()> {
    info!("Preparing test files for I/O benchmark");

    let file_size = DEFAULT_FILE_SIZE;
    let block_size = DEFAULT_BLOCK_SIZE;
    let num_files = DEFAULT_NUM_FILES;

    // Create test directory if it doesn't exist
    std::fs::create_dir_all(DEFAULT_TEST_DIR)
        .map_err(|e| PricingError::Benchmark(format!("Failed to create test directory: {}", e)))?;

    for i in 0..num_files {
        let file_path = get_test_file_path(i);

        // Create file and write random data with checksums
        // Use standard I/O - more reliable across systems
        let mut options = OpenOptions::new();
        options.create(true).write(true).truncate(true);

        // Create the file
        let mut file = options
            .open(&file_path)
            .map_err(|e| PricingError::Benchmark(format!("Failed to create test file: {}", e)))?;

        let blocks_per_file = file_size / block_size as u64;

        // Allocate a buffer for writing
        let mut buffer = vec![0u8; block_size];

        for block_idx in 0..blocks_per_file {
            let offset = block_idx * block_size as u64;
            // Fill buffer with random data and embed checksum and offset
            fill_buffer(&mut buffer, offset);

            file.write_all(&buffer).map_err(|e| {
                PricingError::Benchmark(format!("Failed to write to test file: {}", e))
            })?;

            // Only sync occasionally to improve performance
            if block_idx % 1000 == 0 {
                let _ = file.sync_data(); // Ignore sync errors as they're non-critical
            }
        }

        // Sync file to disk
        let _ = file.sync_all(); // Ignore sync errors as they're non-critical
    }

    info!(
        "Created {} test files, {} MB each",
        num_files,
        file_size / 1024 / 1024
    );
    Ok(())
}

/// Run a specific I/O test mode
fn run_io_test(mode: IoTestMode) -> Result<IoBenchmarkResult> {
    info!("Running I/O test mode: {:?}", mode);

    let file_size = DEFAULT_FILE_SIZE;
    let block_size = DEFAULT_BLOCK_SIZE;
    let num_files = DEFAULT_NUM_FILES;
    let total_file_size = file_size * num_files as u64;

    // Prepare buffers and counters
    let mut buffer = vec![0u8; block_size];
    let mut read_count = 0u64;
    let mut write_count = 0u64;
    let mut read_bytes = 0u64;
    let mut write_bytes = 0u64;
    let mut read_latencies = Vec::new();
    let mut write_latencies = Vec::new();
    let mut validation_errors = 0u64;

    // Verify all test files exist before proceeding
    for i in 0..num_files {
        let file_path = get_test_file_path(i);
        if !file_path.exists() {
            return Err(PricingError::Benchmark(format!(
                "Test file does not exist: {:?}",
                file_path
            )));
        }
    }

    // Open all files with appropriate flags - use standard I/O for reliability
    let mut files = Vec::with_capacity(num_files);
    for i in 0..num_files {
        let file_path = get_test_file_path(i);

        // Standard I/O - most reliable across systems
        let mut options = OpenOptions::new();
        options.read(true).write(true);

        let file = options
            .open(&file_path)
            .map_err(|e| PricingError::Benchmark(format!("Failed to open test file: {}", e)))?;
        files.push(file);
    }

    // Create RNG for random operations
    let mut rng = StdRng::from_entropy();

    // Start timing
    let start = Instant::now();
    let test_duration = Duration::from_secs(5); // Run each test for 5 seconds

    // Run the test
    while start.elapsed() < test_duration {
        match mode {
            IoTestMode::SeqWrite | IoTestMode::SeqRewrite => {
                // Sequential write
                for file in &mut files {
                    file.seek(SeekFrom::Start(0))
                        .map_err(|e| PricingError::Benchmark(format!("Failed to seek: {}", e)))?;

                    let blocks_per_file = file_size / block_size as u64;
                    let mut sync_counter = 0;

                    for block_idx in 0..blocks_per_file {
                        let offset = block_idx * block_size as u64;
                        // Fill buffer with random data and embed checksum and offset
                        fill_buffer(&mut buffer, offset);

                        let write_start = Instant::now();
                        file.write_all(&buffer).map_err(|e| {
                            PricingError::Benchmark(format!("Failed to write: {}", e))
                        })?;

                        // Only sync occasionally to improve performance
                        sync_counter += 1;
                        if sync_counter >= 100 {
                            let _ = file.sync_data(); // Ignore sync errors
                            sync_counter = 0;
                        }

                        let write_duration = write_start.elapsed();

                        write_latencies.push(write_duration.as_secs_f32() * 1000.0); // ms
                        write_count += 1;
                        write_bytes += block_size as u64;
                    }

                    // Sync after each file
                    let _ = file.sync_all(); // Ignore sync errors
                }
            }
            IoTestMode::SeqRead => {
                // Sequential read
                for file in &mut files {
                    file.seek(SeekFrom::Start(0))
                        .map_err(|e| PricingError::Benchmark(format!("Failed to seek: {}", e)))?;

                    let blocks_per_file = file_size / block_size as u64;

                    for block_idx in 0..blocks_per_file {
                        let offset = block_idx * block_size as u64;

                        let read_start = Instant::now();
                        match file.read_exact(&mut buffer) {
                            Ok(_) => {
                                let read_duration = read_start.elapsed();

                                // Validate buffer (check checksum and offset)
                                if !validate_buffer(&buffer, offset) {
                                    validation_errors += 1;
                                }

                                read_latencies.push(read_duration.as_secs_f32() * 1000.0); // ms
                                read_count += 1;
                                read_bytes += block_size as u64;
                            }
                            Err(e) => {
                                // Log error but continue with the benchmark
                                info!("Read error (continuing): {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            IoTestMode::RndRead => {
                // Random read
                for _ in 0..1000 {
                    // Perform 1000 random reads
                    let file_idx = rng.gen_range(0..num_files);
                    let block_idx = rng.gen_range(0..(file_size / block_size as u64));
                    let offset = block_idx * block_size as u64;

                    let file = &mut files[file_idx];
                    match file.seek(SeekFrom::Start(offset)) {
                        Ok(_) => {
                            let read_start = Instant::now();
                            match file.read_exact(&mut buffer) {
                                Ok(_) => {
                                    let read_duration = read_start.elapsed();

                                    // Validate buffer
                                    if !validate_buffer(&buffer, offset) {
                                        validation_errors += 1;
                                    }

                                    read_latencies.push(read_duration.as_secs_f32() * 1000.0); // ms
                                    read_count += 1;
                                    read_bytes += block_size as u64;
                                }
                                Err(e) => {
                                    // Log error but continue with the benchmark
                                    info!("Random read error (continuing): {}", e);
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            // Log error but continue with the benchmark
                            info!("Seek error (continuing): {}", e);
                            continue;
                        }
                    }
                }
            }
            IoTestMode::RndWrite => {
                // Random write
                for _ in 0..1000 {
                    // Perform 1000 random writes
                    let file_idx = rng.gen_range(0..num_files);
                    let block_idx = rng.gen_range(0..(file_size / block_size as u64));
                    let offset = block_idx * block_size as u64;

                    // Fill buffer with random data and embed checksum and offset
                    fill_buffer(&mut buffer, offset);

                    let file = &mut files[file_idx];
                    match file.seek(SeekFrom::Start(offset)) {
                        Ok(_) => {
                            let write_start = Instant::now();
                            match file.write_all(&buffer) {
                                Ok(_) => {
                                    // Only sync every 10 writes to improve performance
                                    if write_count % 10 == 0 {
                                        let _ = file.sync_data(); // Ignore sync errors
                                    }

                                    let write_duration = write_start.elapsed();

                                    write_latencies.push(write_duration.as_secs_f32() * 1000.0); // ms
                                    write_count += 1;
                                    write_bytes += block_size as u64;
                                }
                                Err(e) => {
                                    // Log error but continue with the benchmark
                                    info!("Random write error (continuing): {}", e);
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            // Log error but continue with the benchmark
                            info!("Seek error (continuing): {}", e);
                            continue;
                        }
                    }
                }

                // Sync all files at the end
                for file in &mut files {
                    let _ = file.sync_all(); // Ignore sync errors
                }
            }
            IoTestMode::RndRw => {
                // Random read/write mix (70% reads, 30% writes)
                for _ in 0..1000 {
                    // Perform 1000 random operations
                    let is_read = rng.gen_bool(0.7); // 70% reads
                    let file_idx = rng.gen_range(0..num_files);
                    let block_idx = rng.gen_range(0..(file_size / block_size as u64));
                    let offset = block_idx * block_size as u64;

                    let file = &mut files[file_idx];
                    match file.seek(SeekFrom::Start(offset)) {
                        Ok(_) => {
                            if is_read {
                                let read_start = Instant::now();
                                match file.read_exact(&mut buffer) {
                                    Ok(_) => {
                                        let read_duration = read_start.elapsed();

                                        // Validate buffer
                                        if !validate_buffer(&buffer, offset) {
                                            validation_errors += 1;
                                        }

                                        read_latencies.push(read_duration.as_secs_f32() * 1000.0); // ms
                                        read_count += 1;
                                        read_bytes += block_size as u64;
                                    }
                                    Err(e) => {
                                        // Log error but continue with the benchmark
                                        info!("Read error in RndRw (continuing): {}", e);
                                        continue;
                                    }
                                }
                            } else {
                                // Fill buffer with random data and embed checksum and offset
                                fill_buffer(&mut buffer, offset);

                                let write_start = Instant::now();
                                match file.write_all(&buffer) {
                                    Ok(_) => {
                                        // Only sync every 10 writes to improve performance
                                        if write_count % 10 == 0 {
                                            let _ = file.sync_data(); // Ignore sync errors
                                        }

                                        let write_duration = write_start.elapsed();

                                        write_latencies.push(write_duration.as_secs_f32() * 1000.0); // ms
                                        write_count += 1;
                                        write_bytes += block_size as u64;
                                    }
                                    Err(e) => {
                                        // Log error but continue with the benchmark
                                        info!("Write error in RndRw (continuing): {}", e);
                                        continue;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // Log error but continue with the benchmark
                            info!("Seek error in RndRw (continuing): {}", e);
                            continue;
                        }
                    }
                }

                // Sync all files at the end
                for file in &mut files {
                    let _ = file.sync_all(); // Ignore sync errors
                }
            }
        }
    }

    // Calculate test duration
    let duration_ms = start.elapsed().as_millis() as u64;

    // Log validation errors if any
    if validation_errors > 0 {
        info!(
            "WARNING: {} data validation errors detected during I/O benchmark",
            validation_errors
        );
    }

    // Calculate statistics
    let read_mb = read_bytes as f32 / 1024.0 / 1024.0;
    let write_mb = write_bytes as f32 / 1024.0 / 1024.0;
    let read_iops = if duration_ms > 0 {
        read_count as f32 / (duration_ms as f32 / 1000.0)
    } else {
        0.0
    };
    let write_iops = if duration_ms > 0 {
        write_count as f32 / (duration_ms as f32 / 1000.0)
    } else {
        0.0
    };

    // Calculate latency statistics
    let avg_read_latency_ms = if !read_latencies.is_empty() {
        read_latencies.iter().sum::<f32>() / read_latencies.len() as f32
    } else {
        0.0
    };

    let avg_write_latency_ms = if !write_latencies.is_empty() {
        write_latencies.iter().sum::<f32>() / write_latencies.len() as f32
    } else {
        0.0
    };

    let max_read_latency_ms = read_latencies.iter().fold(0.0f32, |max, &val| max.max(val));
    let max_write_latency_ms = write_latencies
        .iter()
        .fold(0.0f32, |max, &val| max.max(val));

    // Create and return result
    let result = IoBenchmarkResult {
        read_mb,
        write_mb,
        read_iops,
        write_iops,
        avg_read_latency_ms,
        avg_write_latency_ms,
        max_read_latency_ms,
        max_write_latency_ms,
        test_mode: mode,
        block_size,
        total_file_size,
        num_files,
        duration_ms,
    };

    Ok(result)
}

/// Clean up test files after benchmark
fn cleanup_test_files() -> Result<()> {
    info!("Cleaning up I/O benchmark test files");

    // Try to remove each test file
    for i in 0..DEFAULT_NUM_FILES {
        let file_path = get_test_file_path(i);
        if file_path.exists() {
            if let Err(e) = std::fs::remove_file(&file_path) {
                // Log error but continue with cleanup
                info!("Failed to remove test file (continuing): {}", e);
            }
        }
    }

    // Try to remove the test directory
    if Path::new(DEFAULT_TEST_DIR).exists() {
        let _ = std::fs::remove_dir(DEFAULT_TEST_DIR);
    }

    Ok(())
}

/// Get the path for a test file
fn get_test_file_path(index: usize) -> PathBuf {
    Path::new(DEFAULT_TEST_DIR).join(format!("{}_{}", DEFAULT_FILE_PREFIX, index))
}

/// Fill a buffer with random data and embed checksum and offset
fn fill_buffer(buffer: &mut [u8], offset: u64) {
    let data_size = buffer.len() - FILE_CHECKSUM_LENGTH - FILE_OFFSET_LENGTH;

    // Fill main data area with random values
    let mut rng = StdRng::from_entropy();
    for i in 0..data_size {
        buffer[i] = rng.r#gen();
    }

    // Calculate checksum of the data
    let checksum = calculate_crc32(&buffer[0..data_size]);

    // Store checksum at the end of data
    let checksum_bytes = checksum.to_le_bytes();
    buffer[data_size..data_size + FILE_CHECKSUM_LENGTH].copy_from_slice(&checksum_bytes);

    // Store offset at the end of the buffer
    let offset_bytes = offset.to_le_bytes();
    buffer[data_size + FILE_CHECKSUM_LENGTH..].copy_from_slice(&offset_bytes);
}

/// Validate buffer by checking checksum and offset
fn validate_buffer(buffer: &[u8], expected_offset: u64) -> bool {
    let data_size = buffer.len() - FILE_CHECKSUM_LENGTH - FILE_OFFSET_LENGTH;

    // Extract stored checksum
    let mut checksum_bytes = [0u8; 4];
    checksum_bytes.copy_from_slice(&buffer[data_size..data_size + FILE_CHECKSUM_LENGTH]);
    let stored_checksum = u32::from_le_bytes(checksum_bytes);

    // Calculate checksum of the data
    let calculated_checksum = calculate_crc32(&buffer[0..data_size]);

    // Extract stored offset
    let mut offset_bytes = [0u8; 8];
    offset_bytes.copy_from_slice(&buffer[data_size + FILE_CHECKSUM_LENGTH..]);
    let stored_offset = u64::from_le_bytes(offset_bytes);

    // Verify both checksum and offset
    calculated_checksum == stored_checksum && stored_offset == expected_offset
}

/// Calculate CRC32 checksum for a buffer
fn calculate_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = if (crc & 1) != 0 { 0xEDB88320u32 } else { 0 };
            crc = (crc >> 1) ^ mask;
        }
    }
    !crc
}
