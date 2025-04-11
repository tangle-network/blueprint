use std::time::Duration;

use blueprint_testing_utils::setup_log;

use crate::benchmark::{BenchmarkProfile, BenchmarkRunConfig, run_benchmark_suite};
use crate::pricing::calculate_price;

// Helper function to create a test benchmark profile
fn create_test_benchmark_profile(avg_cpu_cores: f32) -> BenchmarkProfile {
    BenchmarkProfile {
        job_id: "test-job".to_string(),
        execution_mode: "native".to_string(),
        duration_secs: 60,
        timestamp: 1643723400, // Fixed timestamp for testing
        success: true,
        cpu_details: Some(crate::benchmark::CpuBenchmarkResult {
            num_cores_detected: 4,
            avg_cores_used: avg_cpu_cores,
            avg_usage_percent: avg_cpu_cores * 25.0, // Assuming 4 cores total
            peak_cores_used: avg_cpu_cores * 1.5,
            peak_usage_percent: avg_cpu_cores * 1.5 * 25.0,
            benchmark_duration_ms: 10000,
            primes_found: 1000,
            max_prime: 10000,
            primes_per_second: 100.0,
            cpu_model: "Test CPU".to_string(),
            cpu_frequency_mhz: 3000.0,
        }),
        memory_details: Some(crate::benchmark::MemoryBenchmarkResult {
            avg_memory_mb: 100.0,
            peak_memory_mb: 150.0,
            block_size_kb: 1024,
            total_size_mb: 1024,
            operations_per_second: 10000.0,
            transfer_rate_mb_s: 2048.0,
            access_mode: crate::benchmark::MemoryAccessMode::Sequential,
            operation_type: crate::benchmark::MemoryOperationType::Write,
            latency_ns: 250.0,
            duration_ms: 5000,
        }),
        io_details: Some(crate::benchmark::IoBenchmarkResult {
            read_mb: 10.0,
            write_mb: 5.0,
            read_iops: 100.0,
            write_iops: 50.0,
            avg_read_latency_ms: 0.1,
            avg_write_latency_ms: 0.2,
            max_read_latency_ms: 1.0,
            max_write_latency_ms: 2.0,
            test_mode: crate::benchmark::IoTestMode::RndRw,
            block_size: 4096,
            total_file_size: 1024 * 1024 * 100, // 100 MB
            num_files: 2,
            duration_ms: 5000,
        }),
        network_details: Some(crate::benchmark::NetworkBenchmarkResult {
            network_rx_mb: 20.0,
            network_tx_mb: 10.0,
            download_speed_mbps: 100.0,
            upload_speed_mbps: 50.0,
            latency_ms: 15.0,
            duration_ms: 5000,
            packet_loss_percent: 0.0,
            jitter_ms: 2.5,
        }),
        gpu_details: Some(crate::benchmark::GpuBenchmarkResult {
            gpu_available: false,
            gpu_memory_mb: 0.0,
            gpu_model: "Test GPU".to_string(),
            gpu_frequency_mhz: 0.0,
        }),
        storage_details: Some(crate::benchmark::StorageBenchmarkResult {
            storage_available_gb: 100.0,
        }),
    }
}

#[test]
fn test_benchmark_suite() {
    setup_log();

    let result = run_benchmark_suite(
        "test-suite".to_string(),
        "test".to_string(),
        Duration::from_secs(30),
        Duration::from_millis(500),
        true, // run_cpu_test
        true, // run_memory_test
        true, // run_io_test
        true,  // run_network_test
        true, // run_gpu_test
    );
    assert!(result.is_ok());

    let profile = result.unwrap();
    println!("Profile: {:#?}", profile);

    assert!(profile.success)
}

#[test]
fn test_calculate_price_basic() {
    setup_log();

    // Create a simple benchmark profile with 1.0 CPU cores
    let profile = create_test_benchmark_profile(1.0);

    // Define a scaling factor (Wei per CPU core)
    let scaling_factor = 1_000_000.0;

    // Calculate price
    let price_model = calculate_price(profile.clone(), scaling_factor).unwrap();

    // Verify the price calculation (1.0 cores * 1,000,000 Wei = 1,000,000 Wei)
    assert_eq!(price_model.price_per_second_wei, 1_000_000);

    // Verify the benchmark profile was stored in the price model
    assert!(price_model.benchmark_profile.is_some());
    let stored_profile = price_model.benchmark_profile.unwrap();
    assert_eq!(stored_profile.job_id, "test-job");
    assert_eq!(stored_profile.cpu_details.unwrap().avg_cores_used, 1.0);
}

#[test]
fn test_calculate_price_zero_cpu() {
    // Create a profile with zero CPU usage
    let profile = create_test_benchmark_profile(0.0);
    let scaling_factor = 1_000_000.0;

    // Calculate price
    let price_model = calculate_price(profile, scaling_factor).unwrap();

    // Price should be zero when CPU usage is zero
    assert_eq!(price_model.price_per_second_wei, 0);
}

#[test]
fn test_calculate_price_high_cpu() {
    // Create a profile with high CPU usage (8 cores)
    let profile = create_test_benchmark_profile(8.0);
    let scaling_factor = 1_000_000.0;

    // Calculate price
    let price_model = calculate_price(profile, scaling_factor).unwrap();

    // Price should be 8 million Wei per second (8.0 cores * 1,000,000 Wei)
    assert_eq!(price_model.price_per_second_wei, 8_000_000);
}

#[test]
fn test_calculate_price_different_scaling_factors() {
    // Create a consistent profile with 2.0 CPU cores
    let profile = create_test_benchmark_profile(2.0);

    // Test with different scaling factors
    let low_scaling = 100.0;
    let medium_scaling = 10_000.0;
    let high_scaling = 1_000_000_000.0;

    // Calculate prices
    let low_price = calculate_price(profile.clone(), low_scaling).unwrap();
    let medium_price = calculate_price(profile.clone(), medium_scaling).unwrap();
    let high_price = calculate_price(profile.clone(), high_scaling).unwrap();

    // Verify scaling works proportionally
    // 2.0 cores * 100 Wei = 200 Wei
    assert_eq!(low_price.price_per_second_wei, 200);
    // 2.0 cores * 10,000 Wei = 20,000 Wei
    assert_eq!(medium_price.price_per_second_wei, 20_000);
    // 2.0 cores * 1,000,000,000 Wei = 2,000,000,000 Wei
    assert_eq!(high_price.price_per_second_wei, 2_000_000_000);
}

#[test]
fn test_calculate_price_negative_scaling_factor() {
    // Create a profile with 1.0 CPU cores
    let profile = create_test_benchmark_profile(1.0);

    // Use a negative scaling factor (which should result in 0 price due to max(0.0, price))
    let scaling_factor = -1000.0;

    // Calculate price
    let price_model = calculate_price(profile, scaling_factor).unwrap();

    // Price should be clamped to 0
    assert_eq!(price_model.price_per_second_wei, 0);
}

#[test]
fn test_io_benchmark() {
    // Create a simple benchmark config
    let config = BenchmarkRunConfig {
        job_id: "io-test".to_string(),
        mode: "test".to_string(),
        command: "echo".to_string(),
        args: vec!["benchmark".to_string()],
        max_duration: Duration::from_secs(30),
        sample_interval: Duration::from_millis(100),
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: true,
        run_network_test: false,
        run_gpu_test: false,
    };

    // Run the I/O benchmark
    let result = crate::benchmark::io::run_io_benchmark(&config).unwrap();

    // Print the results
    println!("I/O Benchmark Results:");
    println!("  Read: {:.2} MB", result.read_mb);
    println!("  Write: {:.2} MB", result.write_mb);
    println!("  Read IOPS: {:.2}", result.read_iops);
    println!("  Write IOPS: {:.2}", result.write_iops);
    println!("  Avg Read Latency: {:.2} ms", result.avg_read_latency_ms);
    println!("  Avg Write Latency: {:.2} ms", result.avg_write_latency_ms);

    // Verify that we got some results
    assert!(result.read_mb >= 0.0);
    assert!(result.write_mb >= 0.0);
    assert!(result.read_iops >= 0.0);
    assert!(result.write_iops >= 0.0);
}

#[test]
fn test_memory_benchmark() {
    setup_log();

    // Create a simple benchmark config
    let config = BenchmarkRunConfig {
        job_id: "memory-test".to_string(),
        mode: "test".to_string(),
        command: "echo".to_string(),
        args: vec!["benchmark".to_string()],
        max_duration: Duration::from_secs(10),
        sample_interval: Duration::from_millis(100),
        run_cpu_test: false,
        run_memory_test: true,
        run_io_test: false,
        run_network_test: false,
        run_gpu_test: false,
    };

    // Run the memory benchmark
    let result = crate::benchmark::memory::run_memory_benchmark(&config).unwrap();

    // Print the results
    println!("Memory Benchmark Results:");
    println!("  Block Size: {} KB", result.block_size_kb);
    println!("  Total Size: {} MB", result.total_size_mb);
    println!("  Operations/sec: {:.2}", result.operations_per_second);
    println!("  Transfer Rate: {:.2} MB/s", result.transfer_rate_mb_s);
    println!("  Access Mode: {:?}", result.access_mode);
    println!("  Operation Type: {:?}", result.operation_type);
    println!("  Avg Latency: {:.2} ns", result.latency_ns);
    println!("  Duration: {} ms", result.duration_ms);

    // Verify that we got some results
    assert!(result.operations_per_second > 0.0);
    assert!(result.transfer_rate_mb_s > 0.0);
    assert!(result.latency_ns > 0.0);
    assert!(result.duration_ms > 0);
}

#[test]
fn test_network_benchmark() {
    setup_log();

    // Create a simple benchmark config
    let config = BenchmarkRunConfig {
        job_id: "network-test".to_string(),
        mode: "test".to_string(),
        command: "echo".to_string(),
        args: vec!["benchmark".to_string()],
        max_duration: Duration::from_secs(30),
        sample_interval: Duration::from_millis(100),
        run_cpu_test: false,
        run_memory_test: false,
        run_io_test: false,
        run_network_test: true,
        run_gpu_test: false,
    };

    // Run the network benchmark
    let result = crate::benchmark::network::run_network_benchmark(&config).unwrap();

    // Print the results
    println!("Network Benchmark Results:");
    println!("  Data Received: {:.2} MB", result.network_rx_mb);
    println!("  Data Transmitted: {:.2} MB", result.network_tx_mb);
    println!("  Download Speed: {:.2} Mbps", result.download_speed_mbps);
    println!("  Upload Speed: {:.2} Mbps", result.upload_speed_mbps);
    println!("  Latency: {:.2} ms", result.latency_ms);
    println!("  Jitter: {:.2} ms", result.jitter_ms);
    println!("  Packet Loss: {:.2}%", result.packet_loss_percent);
    println!("  Duration: {} ms", result.duration_ms);

    // Verify that we got some results
    assert!(result.network_rx_mb > 0.0);
    assert!(result.network_tx_mb > 0.0);
    assert!(result.download_speed_mbps > 0.0);
    assert!(result.latency_ms >= 0.0);
    assert!(result.duration_ms > 0);
}
