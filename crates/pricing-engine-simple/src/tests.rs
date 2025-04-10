use crate::benchmark::BenchmarkProfile;
use crate::pricing::calculate_price;

// Helper function to create a test benchmark profile
fn create_test_benchmark_profile(avg_cpu_cores: f32) -> BenchmarkProfile {
    BenchmarkProfile {
        job_id: "test-job".to_string(),
        execution_mode: "native".to_string(),
        avg_cpu_cores,
        peak_cpu_cores: avg_cpu_cores * 1.5,
        avg_memory_mb: 100.0,
        peak_memory_mb: 150.0,
        io_read_mb: 10.0,
        io_write_mb: 5.0,
        network_rx_mb: 20.0,
        network_tx_mb: 10.0,
        storage_available_gb: 100.0,
        gpu_available: false,
        gpu_memory_mb: 0.0,
        duration_secs: 60,
        timestamp: 1643723400, // Fixed timestamp for testing
        success: true,
    }
}

#[test]
fn test_calculate_price_basic() {
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
    assert_eq!(stored_profile.avg_cpu_cores, 1.0);
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
