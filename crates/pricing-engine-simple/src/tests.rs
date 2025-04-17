use std::time::Duration;

use blueprint_testing_utils::setup_log;

use crate::benchmark::{BenchmarkProfile, BenchmarkRunConfig, run_benchmark_suite};
use crate::pricing::calculate_price;
use crate::types::ResourceUnit;

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
        true, // run_network_test
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

    // Define a scaling factor (USD per CPU core)
    let scaling_factor = 0.001; // $0.001 per CPU core

    // Calculate price
    let price_model = calculate_price(profile.clone(), scaling_factor).unwrap();

    // Verify the price calculation (1.0 cores * $0.001 = $0.001 per second)
    assert!((price_model.price_per_second_rate - 0.001).abs() < 1e-6, 
        "Expected price_per_second_rate to be 0.001, got {}", price_model.price_per_second_rate);

    // Verify resources were created correctly
    assert!(!price_model.resources.is_empty());

    // Find CPU resource
    let cpu_resource = price_model
        .resources
        .iter()
        .find(|r| matches!(r.kind, ResourceUnit::CPU));

    // Verify CPU resource exists and has correct values
    assert!(cpu_resource.is_some());
    let cpu = cpu_resource.unwrap();
    assert_eq!(cpu.count, 1);
    assert!((cpu.price_per_unit_rate - 0.001).abs() < 1e-6, 
        "Expected CPU price_per_unit_rate to be 0.001, got {}", cpu.price_per_unit_rate);

    // Test cost calculation for different time periods
    let minute_cost = price_model.calculate_total_cost(60);
    let hour_cost = price_model.calculate_total_cost(3600);
    let day_cost = price_model.calculate_total_cost(86400);

    // Verify cost calculations
    assert!((minute_cost - 0.06).abs() < 1e-6, 
        "Expected minute cost to be 0.06, got {}", minute_cost);
    assert!((hour_cost - 3.6).abs() < 1e-6, 
        "Expected hour cost to be 3.6, got {}", hour_cost);
    assert!((day_cost - 86.4).abs() < 1e-6, 
        "Expected day cost to be 86.4, got {}", day_cost);
}

#[test]
fn test_calculate_price_high_cpu() {
    setup_log();

    // Create a benchmark profile with 4.0 CPU cores
    let profile = create_test_benchmark_profile(4.0);

    // Define a scaling factor (USD per CPU core)
    let scaling_factor = 0.001; // $0.001 per CPU core

    // Calculate price
    let price_model = calculate_price(profile.clone(), scaling_factor).unwrap();

    // Verify the price calculation (4.0 cores * $0.001 = $0.004 per second)
    assert!((price_model.price_per_second_rate - 0.004).abs() < 1e-6, 
        "Expected price_per_second_rate to be 0.004, got {}", price_model.price_per_second_rate);

    // Find CPU resource
    let cpu_resource = price_model
        .resources
        .iter()
        .find(|r| matches!(r.kind, ResourceUnit::CPU));

    // Verify CPU resource exists and has correct values
    assert!(cpu_resource.is_some());
    let cpu = cpu_resource.unwrap();
    assert_eq!(cpu.count, 4);
    assert!((cpu.price_per_unit_rate - 0.001).abs() < 1e-6, 
        "Expected CPU price_per_unit_rate to be 0.001, got {}", cpu.price_per_unit_rate);
}

#[test]
fn test_calculate_price_different_scaling_factors() {
    setup_log();

    // Create a benchmark profile with 2.0 CPU cores
    let profile = create_test_benchmark_profile(2.0);

    // Test different scaling factors
    let scaling_factors = [0.0005, 0.001, 0.002, 0.005];
    let expected_prices = [0.001, 0.002, 0.004, 0.01]; // 2.0 cores * scaling factor

    for (i, &scaling_factor) in scaling_factors.iter().enumerate() {
        // Calculate price
        let price_model = calculate_price(profile.clone(), scaling_factor).unwrap();

        // Verify the price calculation
        let expected_price = expected_prices[i];
        assert!((price_model.price_per_second_rate - expected_price).abs() < 1e-6, 
            "With scaling factor {}, expected price_per_second_rate to be {}, got {}", 
            scaling_factor, expected_price, price_model.price_per_second_rate);

        // Find CPU resource
        let cpu_resource = price_model
            .resources
            .iter()
            .find(|r| matches!(r.kind, ResourceUnit::CPU));

        // Verify CPU resource exists and has correct values
        assert!(cpu_resource.is_some());
        let cpu = cpu_resource.unwrap();
        assert_eq!(cpu.count, 2);
        assert!((cpu.price_per_unit_rate - scaling_factor).abs() < 1e-6, 
            "With scaling factor {}, expected CPU price_per_unit_rate to be {}, got {}", 
            scaling_factor, scaling_factor, cpu.price_per_unit_rate);

        // Test cost calculation for different time periods
        let minute_cost = price_model.calculate_total_cost(60);
        let hour_cost = price_model.calculate_total_cost(3600);
        let day_cost = price_model.calculate_total_cost(86400);

        // Verify cost calculations
        let expected_minute_cost = expected_price * 60.0;
        let expected_hour_cost = expected_price * 3600.0;
        let expected_day_cost = expected_price * 86400.0;

        assert!((minute_cost - expected_minute_cost).abs() < 1e-6, 
            "With scaling factor {}, expected minute cost to be {}, got {}", 
            scaling_factor, expected_minute_cost, minute_cost);
        assert!((hour_cost - expected_hour_cost).abs() < 1e-6, 
            "With scaling factor {}, expected hour cost to be {}, got {}", 
            scaling_factor, expected_hour_cost, hour_cost);
        assert!((day_cost - expected_day_cost).abs() < 1e-6, 
            "With scaling factor {}, expected day cost to be {}, got {}", 
            scaling_factor, expected_day_cost, day_cost);
    }
}

#[test]
fn test_calculate_price_negative_scaling_factor() {
    setup_log();

    // Create a benchmark profile with 2.0 CPU cores
    let profile = create_test_benchmark_profile(2.0);

    // Define a negative scaling factor (should be treated as 0)
    let scaling_factor = -0.001;

    // Calculate price
    let price_model = calculate_price(profile.clone(), scaling_factor).unwrap();

    // Verify the price calculation (negative scaling factor should result in 0 price)
    // Since we're now using max(0.0, value), the price should be 0.0
    assert!((price_model.price_per_second_rate - 0.0).abs() < 1e-6, 
        "Expected price_per_second_rate to be 0.0 with negative scaling factor, got {}", 
        price_model.price_per_second_rate);

    // Find CPU resource
    let cpu_resource = price_model
        .resources
        .iter()
        .find(|r| matches!(r.kind, ResourceUnit::CPU));

    // Verify CPU resource exists and has price_per_unit_rate of 0.0
    assert!(cpu_resource.is_some());
    let cpu = cpu_resource.unwrap();
    assert!((cpu.price_per_unit_rate - 0.0).abs() < 1e-6,
        "Expected CPU price_per_unit_rate to be 0.0 with negative scaling factor, got {}", 
        cpu.price_per_unit_rate);
}

#[test]
fn test_io_benchmark() {
    setup_log();

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

    // Run the IO benchmark
    let result = crate::benchmark::io::run_io_benchmark(&config).unwrap();

    // Print the results
    println!("IO Benchmark Results:");
    println!("  Read: {:.2} MB", result.read_mb);
    println!("  Write: {:.2} MB", result.write_mb);
    println!("  Read IOPS: {:.2}", result.read_iops);
    println!("  Write IOPS: {:.2}", result.write_iops);
    println!("  Read Latency: {:.2} ms", result.avg_read_latency_ms);
    println!("  Write Latency: {:.2} ms", result.avg_write_latency_ms);
    println!("  Duration: {} ms", result.duration_ms);

    // Verify that we got some results
    assert!(result.read_mb > 0.0);
    assert!(result.write_mb > 0.0);
    assert!(result.read_iops > 0.0);
    assert!(result.write_iops > 0.0);
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
        max_duration: Duration::from_secs(30),
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
    println!("  Average Memory: {:.2} MB", result.avg_memory_mb);
    println!("  Peak Memory: {:.2} MB", result.peak_memory_mb);
    println!("  Block Size: {} KB", result.block_size_kb);
    println!("  Total Size: {} MB", result.total_size_mb);
    println!("  Operations/sec: {:.2}", result.operations_per_second);
    println!("  Transfer Rate: {:.2} MB/s", result.transfer_rate_mb_s);
    println!("  Latency: {:.2} ns", result.latency_ns);
    println!("  Duration: {} ms", result.duration_ms);

    // Verify that we got some results
    assert!(result.avg_memory_mb > 0.0);
    assert!(result.peak_memory_mb > 0.0);
    assert!(result.operations_per_second > 0.0);
    assert!(result.transfer_rate_mb_s > 0.0);
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
    assert!(result.upload_speed_mbps > 0.0);
}

#[test]
fn test_resource_pricing() {
    // Create a price model with specific resource pricing
    let price_model = crate::pricing::PriceModel {
        resources: vec![
            crate::pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 2,
                price_per_unit_rate: 0.001, // $0.001 per CPU core
            },
            crate::pricing::ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_rate: 0.00005, // $0.00005 per MB
            },
        ],
        price_per_second_rate: 0.053_2, // (2 * 0.001) + (1024 * 0.00005)
        generated_at: chrono::Utc::now(),
        benchmark_profile: None,
    };

    // Test total cost calculation for different TTLs
    let one_minute_cost = price_model.calculate_total_cost(60);
    let one_hour_cost = price_model.calculate_total_cost(3600);
    let one_day_cost = price_model.calculate_total_cost(86400);

    // Expected costs
    let expected_one_minute = 0.053_2 * 60.0;
    let expected_one_hour = 0.053_2 * 3600.0;
    let expected_one_day = 0.053_2 * 86400.0;

    // Verify calculations with floating-point comparison
    assert!((one_minute_cost - expected_one_minute).abs() < 1e-6, 
        "One minute cost calculation incorrect. Expected: {}, Got: {}", 
        expected_one_minute, one_minute_cost);
    
    assert!((one_hour_cost - expected_one_hour).abs() < 1e-6, 
        "One hour cost calculation incorrect. Expected: {}, Got: {}", 
        expected_one_hour, one_hour_cost);
    
    assert!((one_day_cost - expected_one_day).abs() < 1e-6, 
        "One day cost calculation incorrect. Expected: {}, Got: {}", 
        expected_one_day, one_day_cost);
    
    // Print the costs for information
    println!("Resource pricing test:");
    println!("  Price per second: ${:.6}", price_model.price_per_second_rate);
    println!("  One minute cost: ${:.6}", one_minute_cost);
    println!("  One hour cost: ${:.6}", one_hour_cost);
    println!("  One day cost: ${:.6}", one_day_cost);
}

#[test]
fn test_pow_challenge_generation() {
    use crate::pow::generate_challenge;

    // Generate challenges with different inputs
    let blueprint_id_1 = 12345;
    let timestamp_1 = 1643723400;
    let challenge_1 = generate_challenge(blueprint_id_1, timestamp_1);

    let blueprint_id_2 = 12345;
    let timestamp_2 = 1643723401; // Different timestamp
    let challenge_2 = generate_challenge(blueprint_id_2, timestamp_2);

    let blueprint_id_3 = 54321; // Different blueprint ID
    let timestamp_3 = 1643723400;
    let challenge_3 = generate_challenge(blueprint_id_3, timestamp_3);

    // Verify challenges are not empty
    assert!(!challenge_1.is_empty());
    assert!(!challenge_2.is_empty());
    assert!(!challenge_3.is_empty());

    // Verify different inputs produce different challenges
    assert_ne!(
        challenge_1, challenge_2,
        "Different timestamps should produce different challenges"
    );
    assert_ne!(
        challenge_1, challenge_3,
        "Different blueprint IDs should produce different challenges"
    );

    // Verify same inputs produce the same challenge (deterministic)
    let challenge_1_repeat = generate_challenge(blueprint_id_1, timestamp_1);
    assert_eq!(
        challenge_1, challenge_1_repeat,
        "Same inputs should produce the same challenge"
    );
}
