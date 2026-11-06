use blueprint_pricing_engine_lib::CpuBenchmarkResult;
use blueprint_pricing_engine_lib::pricing::ResourcePricing;
use blueprint_pricing_engine_lib::types::ResourceUnit;
use blueprint_pricing_engine_lib::{BenchmarkProfile, OperatorConfig};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        database_path: "./data/test_benchmark_cache".to_string(),
        benchmark_duration: 10,
        benchmark_interval: 1,
        keystore_path: PathBuf::from("./tests/tmp-keystore"),
        rpc_bind_address: "127.0.0.1".to_string(),
        rpc_port: 9000,
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300,
        ..OperatorConfig::default()
    }
}

/// Non-zero placeholder requester for tests that don't care about the
/// specific buyer address. Production callers MUST pass the real buyer.
#[allow(dead_code)]
pub fn test_requester_bytes() -> Vec<u8> {
    // 0x000000000000000000000000000000000000bEEF
    let mut bytes = vec![0u8; 20];
    bytes[18] = 0xbe;
    bytes[19] = 0xef;
    bytes
}

#[allow(dead_code)]
pub fn sample_benchmark_profile(blueprint_id: u64) -> BenchmarkProfile {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    BenchmarkProfile {
        job_id: blueprint_id.to_string(),
        execution_mode: "native".to_string(),
        duration_secs: 1,
        timestamp: now,
        success: true,
        cpu_details: Some(CpuBenchmarkResult {
            num_cores_detected: 4,
            avg_cores_used: 2.0,
            avg_usage_percent: 50.0,
            peak_cores_used: 2.0,
            peak_usage_percent: 75.0,
            benchmark_duration_ms: 10,
            primes_found: 128,
            max_prime: 1024,
            primes_per_second: 64.0,
            cpu_model: "anvil-ci".to_string(),
            cpu_frequency_mhz: 3200.0,
        }),
        io_details: None,
        memory_details: None,
        network_details: None,
        gpu_details: None,
        storage_details: None,
    }
}

#[allow(dead_code)]
pub fn sample_pricing_map(blueprint_id: Option<u64>) -> HashMap<Option<u64>, Vec<ResourcePricing>> {
    let pricing = ResourcePricing {
        kind: ResourceUnit::CPU,
        count: 2,
        price_per_unit_rate: Decimal::new(5, 6),
    };

    HashMap::from([(blueprint_id, vec![pricing])])
}
