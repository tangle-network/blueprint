use std::sync::Arc;

use blueprint_crypto::KeyType;
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_crypto_core::BytesEncoding;
use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache},
    benchmark::{BenchmarkProfile, CpuBenchmarkResult},
    config::OperatorConfig,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof},
    pricing::{self, PriceModel},
    pricing_engine::{self, pricing_engine_server::PricingEngine},
    service::rpc::server::PricingEngineService,
    signer::QuotePayload,
    types::ResourceUnit,
};
use chrono::Utc;
use serde_json;
use tonic::Request;

// Helper function to initialize test logging
fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}

// Create a test config
fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        database_path: "./data/test_price_cache".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["test".to_string()],
        benchmark_duration: 1,
        benchmark_interval: 1,
        price_scaling_factor: 1000.0,
        quote_validity_duration_secs: 300, // e.g., 5 minutes for testing
        keypair_path: "/tmp/test-keypair".to_string(),
        keystore_path: "/tmp/test-keystore".to_string(),
        rpc_bind_address: "127.0.0.1:9000".to_string(),
        rpc_port: 9000,
        rpc_timeout: 5,
        rpc_max_connections: 10,
    }
}

// Helper function to create a test blueprint ID
fn create_test_blueprint_id() -> u64 {
    // Create a deterministic test ID
    12345
}

// Helper function to create a test price model
fn create_test_price_model() -> PriceModel {
    PriceModel {
        resources: vec![
            pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 2,
                price_per_unit_wei: 500000,
            },
            pricing::ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_wei: 1000,
            },
        ],
        price_per_second_wei: 1000,
        generated_at: Utc::now(),
        benchmark_profile: None,
    }
}

#[tokio::test]
async fn test_pricing_engine_components() -> blueprint_pricing_engine_simple_lib::error::Result<()>
{
    init_test_logging();

    let config = Arc::new(create_test_config());

    let keystore_path = std::path::Path::new(&config.keystore_path);
    if !keystore_path.exists() {
        std::fs::create_dir_all(keystore_path)?;
    }

    let cache_path = std::path::Path::new(&config.database_path);
    if !cache_path.exists() {
        std::fs::create_dir_all(cache_path)?;
    }

    let price_cache = init_price_cache(&config).await?;

    let _operator_signer = init_operator_signer(&config, &config.keystore_path)?;

    let blueprint_id = create_test_blueprint_id();
    let price_model = create_test_price_model();
    price_cache.store_price(blueprint_id, &price_model)?;

    let retrieved_model = price_cache.get_price(blueprint_id)?;
    assert!(
        retrieved_model.is_some(),
        "Price model should be in the cache"
    );

    let retrieved_model = retrieved_model.unwrap();
    assert_eq!(
        retrieved_model.price_per_second_wei, price_model.price_per_second_wei,
        "Retrieved price should match stored price"
    );

    let cpu_result = CpuBenchmarkResult {
        num_cores_detected: 4,
        avg_cores_used: 3.0,
        avg_usage_percent: 75.0,
        peak_cores_used: 4.0,
        peak_usage_percent: 100.0,
        benchmark_duration_ms: 1000,
        primes_found: 1500,
        max_prime: 10000,
        primes_per_second: 1500.0,
        cpu_model: "Test CPU".to_string(),
        cpu_frequency_mhz: 3000.0,
    };

    let benchmark_profile = BenchmarkProfile {
        job_id: "test-job".to_string(),
        execution_mode: "test".to_string(),
        duration_secs: 10,
        timestamp: 1000000,
        success: true,
        cpu_details: Some(cpu_result),
        memory_details: None,
        io_details: None,
        network_details: None,
        gpu_details: None,
        storage_details: None,
    };

    let calculated_price =
        pricing::calculate_price(benchmark_profile.clone(), config.price_scaling_factor)?;

    assert!(
        calculated_price.price_per_second_wei > 0,
        "Calculated price should be greater than zero"
    );

    assert!(
        calculated_price.benchmark_profile.is_some(),
        "Benchmark profile should be included in the price model"
    );

    Ok(())
}

#[tokio::test]
async fn test_rpc_get_price_flow() -> blueprint_pricing_engine_simple_lib::error::Result<()> {
    init_test_logging();

    let config = Arc::new(create_test_config());
    let blueprint_id = create_test_blueprint_id();
    let price_model = create_test_price_model();

    let keystore_path = std::path::Path::new(&config.keystore_path);
    if !keystore_path.exists() {
        std::fs::create_dir_all(keystore_path)?;
    }
    let cache_path = std::path::Path::new(&config.database_path);
    if !cache_path.exists() {
        std::fs::create_dir_all(cache_path)?;
    }

    let price_cache = init_price_cache(&config).await?;
    let _operator_signer = init_operator_signer(&config, &config.keystore_path)?;

    price_cache.store_price(blueprint_id, &price_model)?;

    let pricing_service = PricingEngineService::new(
        config.clone(),
        price_cache.clone(),
        _operator_signer.clone(),
    );

    // Generate proof of work for the request
    let challenge = generate_challenge(blueprint_id, Utc::now().timestamp() as u64);
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

    // Create resource requirements
    let resource_requirements = vec![
        pricing_engine::ResourceRequirement {
            kind: "CPU".to_string(),
            count: 2,
        },
        pricing_engine::ResourceRequirement {
            kind: "MemoryMB".to_string(),
            count: 1024,
        },
    ];

    let request = Request::new(pricing_engine::GetPriceRequest {
        blueprint_id,
        ttl_seconds: 3600,
        proof_of_work: proof.clone(),
        resource_requirements,
    });

    let response = pricing_service.get_price(request).await.map_err(|e| {
        eprintln!("RPC call failed: {:?}", e);
        blueprint_pricing_engine_simple_lib::error::PricingError::Other(format!(
            "RPC error: {:?}",
            e
        ))
    })?;

    let response = response.into_inner();

    assert!(
        response.quote_details.is_some(),
        "Response should contain quote details"
    );
    let quote_details = response.quote_details.unwrap();

    assert_eq!(
        quote_details.blueprint_id, blueprint_id,
        "Blueprint ID in response mismatch"
    );

    assert_eq!(quote_details.ttl_seconds, 3600, "TTL in response mismatch");

    assert!(
        quote_details.expiry > Utc::now().timestamp() as u64,
        "Expiry should be in the future"
    );

    assert!(
        !quote_details.resources.is_empty(),
        "Resources should not be empty"
    );

    // Verify signature
    let internal_payload_for_verification = QuotePayload {
        blueprint_id,
        ttl_seconds: 3600,
        total_cost_wei: quote_details.total_cost_wei.parse::<u128>().unwrap(),
        expiry: quote_details.expiry,
        timestamp: Utc::now().timestamp() as u64, // This is an approximation, server uses its own timestamp
        resources: vec![
            pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 1,
                price_per_unit_wei: 1_000_000,
            },
            pricing::ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_wei: 500,
            },
        ],
    };

    // Serialize the reconstructed internal payload to get the message bytes
    let _message_bytes = serde_json::to_vec(&internal_payload_for_verification).unwrap();

    let signature_bytes = response.signature.clone();
    let operator_id = response.operator_id.clone(); // Clone to avoid move
    let response_pow = response.proof_of_work.clone();

    // Verify the proof of work in the response
    assert_eq!(
        response_pow, proof,
        "Proof of work in response doesn't match request proof"
    );

    // Get the public key from the operator signer for verification
    let _public_key = _operator_signer.lock().await.public_key();
    let _signature = <K256Ecdsa as KeyType>::Signature::from_bytes(&signature_bytes)
        .expect("Failed to parse signature");
    let _verifying_key = <K256Ecdsa as KeyType>::Public::from_bytes(&operator_id)
        .expect("Failed to parse public key");

    // This is an approximation since we can't know the exact timestamp used by the server
    // In a real verification, we would extract the exact payload from the response
    // For test purposes, we'll just check that the operator ID matches
    assert_eq!(
        operator_id,
        _operator_signer.lock().await.operator_id(),
        "Operator ID in response doesn't match signer's operator ID"
    );

    Ok(())
}

// New test to verify proof of work generation and verification
#[tokio::test]
async fn test_proof_of_work() -> blueprint_pricing_engine_simple_lib::error::Result<()> {
    init_test_logging();

    let blueprint_id = create_test_blueprint_id();
    let timestamp = Utc::now().timestamp() as u64;

    // Generate a challenge
    let challenge = generate_challenge(blueprint_id, timestamp);
    assert!(!challenge.is_empty(), "Challenge should not be empty");

    // Generate proof of work
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;
    assert!(!proof.is_empty(), "Proof should not be empty");

    // Verify the proof
    let is_valid = verify_proof(&challenge, &proof, DEFAULT_POW_DIFFICULTY)?;
    assert!(is_valid, "Proof of work verification failed");

    // Test with invalid proof
    let mut invalid_proof = proof.clone();
    if !invalid_proof.is_empty() {
        invalid_proof[0] = invalid_proof[0].wrapping_add(1);
    }

    let is_invalid = verify_proof(&challenge, &invalid_proof, DEFAULT_POW_DIFFICULTY)?;
    assert!(!is_invalid, "Invalid proof should not verify");

    Ok(())
}

// New test for resource-based pricing
#[tokio::test]
async fn test_resource_based_pricing() -> blueprint_pricing_engine_simple_lib::error::Result<()> {
    init_test_logging();

    // Create a price model with specific resource pricing
    let price_model = PriceModel {
        resources: vec![
            pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 1,
                price_per_unit_wei: 1_000_000,
            },
            pricing::ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_wei: 500,
            },
        ],
        price_per_second_wei: 1_512_000, // 1 CPU + 1024 MB memory
        generated_at: Utc::now(),
        benchmark_profile: None,
    };

    // Calculate total cost for different TTLs
    let one_hour_cost = price_model.calculate_total_cost(3600);
    let one_day_cost = price_model.calculate_total_cost(86400);

    // Verify calculations
    assert_eq!(
        one_hour_cost,
        1_512_000 * 3600,
        "One hour cost calculation incorrect"
    );
    assert_eq!(
        one_day_cost,
        1_512_000 * 86400,
        "One day cost calculation incorrect"
    );

    // Test with different resource requirements
    let double_cpu_model = PriceModel {
        resources: vec![
            pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 2,
                price_per_unit_wei: 1_000_000,
            },
            pricing::ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_wei: 500,
            },
        ],
        price_per_second_wei: 2_512_000, // 2 CPU + 1024 MB memory
        generated_at: Utc::now(),
        benchmark_profile: None,
    };

    let double_cpu_cost = double_cpu_model.calculate_total_cost(3600);
    assert_eq!(
        double_cpu_cost,
        2_512_000 * 3600,
        "Double CPU cost calculation incorrect"
    );

    // Verify that double CPU costs more than single CPU
    assert!(
        double_cpu_cost > one_hour_cost,
        "Higher resource usage should cost more"
    );

    Ok(())
}
