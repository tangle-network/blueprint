use std::sync::Arc;

use blueprint_crypto::KeyType;
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_pricing_engine_simple_lib::pricing_engine::GetPriceRequest;
use blueprint_pricing_engine_simple_lib::pricing_engine::pricing_engine_server::PricingEngine;
use blueprint_pricing_engine_simple_lib::service::rpc::server::PricingEngineService;

use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache},
    benchmark::{BenchmarkProfile, CpuBenchmarkResult},
    cache::BlueprintHash,
    config::OperatorConfig,
    error::Result,
    pricing::{self, PriceModel},
    signer::QuotePayload as SignerQuotePayload, // Import the signer's payload
};

use blueprint_crypto_core::BytesEncoding;
use chrono::Utc;
use tonic::Request;

// Helper function to initialize test logging
fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}

// Helper function to create a test configuration
fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        database_path: "./data/test_price_cache".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["test".to_string()],
        benchmark_duration: 1,
        benchmark_interval: 1,
        price_scaling_factor: 1000.0,
        keypair_path: "/tmp/test-keypair".to_string(),
        keystore_path: "/tmp/test-keystore".to_string(),
        rpc_bind_address: "127.0.0.1".to_string(),
        rpc_port: 9000,
        rpc_timeout: 5,
        rpc_max_connections: 10,
        quote_validity_duration_secs: 300, // e.g., 5 minutes for testing
    }
}

// Helper function to create a test blueprint hash
fn create_test_blueprint_hash() -> ([u8; 32], BlueprintHash) {
    // Create a deterministic test hash (32 bytes of data)
    let hash_bytes = [1u8; 32];
    // Convert to hex string
    (hash_bytes, hex::encode(hash_bytes))
}

// Helper function to create a test price model
fn create_test_price_model() -> PriceModel {
    let cpu_result = CpuBenchmarkResult {
        num_cores_detected: 4,
        avg_cores_used: 2.5,
        avg_usage_percent: 60.0,
        peak_cores_used: 4.0,
        peak_usage_percent: 90.0,
        benchmark_duration_ms: 1000,
        primes_found: 1000,
        max_prime: 10000,
        primes_per_second: 1000.0,
        cpu_model: "Test CPU".to_string(),
        cpu_frequency_mhz: 2500.0,
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

    PriceModel {
        price_per_second_wei: 1000000, // 1M wei per second
        generated_at: Utc::now(),
        benchmark_profile: Some(benchmark_profile),
    }
}

#[tokio::test]
async fn test_pricing_engine_components() -> Result<()> {
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

    let _operator_signer_arc = init_operator_signer(&config).await?;

    let (_blueprint_bytes, blueprint_hash_hex) = create_test_blueprint_hash();
    let price_model = create_test_price_model();
    price_cache.store_price(&blueprint_hash_hex, &price_model)?;

    let retrieved_model = price_cache.get_price(&blueprint_hash_hex)?;
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
async fn test_rpc_get_price_flow() -> Result<()> {
    init_test_logging();

    let config = Arc::new(create_test_config());
    let (blueprint_bytes, blueprint_hash_hex) = create_test_blueprint_hash();
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
    let operator_signer = init_operator_signer(&config).await?;

    price_cache.store_price(&blueprint_hash_hex, &price_model)?;

    let pricing_service =
        PricingEngineService::new(config.clone(), price_cache.clone(), operator_signer.clone());

    let request = Request::new(GetPriceRequest {
        blueprint_hash_hex: blueprint_hash_hex.clone(),
    });

    let response = pricing_service.get_price(request).await;

    assert!(response.is_ok(), "RPC call failed: {:?}", response.err());
    let response = response.unwrap().into_inner();

    assert!(response.quote.is_some(), "Response should contain a quote");
    let signed_quote_rpc = response.quote.unwrap();

    assert!(
        signed_quote_rpc.payload.is_some(),
        "Quote should contain a payload"
    );
    let payload_rpc = signed_quote_rpc.payload.unwrap();

    assert_eq!(
        payload_rpc.blueprint_hash,
        blueprint_bytes.to_vec(),
        "Blueprint hash in payload mismatch"
    );
    assert_eq!(
        payload_rpc.price_wei,
        price_model.price_per_second_wei.to_string(),
        "Price mismatch"
    );
    assert!(
        payload_rpc.expiry > payload_rpc.timestamp,
        "Expiry should be after timestamp"
    );

    // Verify signature
    // Reconstruct the *exact* payload structure that was signed inside get_price
    // This must match the `signer::QuotePayload` struct.

    // Derive the blueprint_id from the first 8 bytes of the original hash
    // This MUST match the derivation logic in the server
    let derived_blueprint_id = u64::from_be_bytes(
        blueprint_bytes[0..8]
            .try_into()
            .expect("Slice length mismatch"),
    );

    let internal_payload_for_verification = SignerQuotePayload {
        // Use the derived ID that the server signed
        blueprint_id: derived_blueprint_id,
        price_wei: payload_rpc
            .price_wei
            .parse::<u128>()
            .expect("Invalid price format in response"),
        expiry: payload_rpc.expiry,
        timestamp: payload_rpc.timestamp,
    };

    // Serialize the reconstructed internal payload to get the message bytes
    let message_bytes = bincode::serialize(&internal_payload_for_verification)
        .expect("Failed to serialize internal payload for verification");

    let public_key_bytes = signed_quote_rpc.signer_pubkey;
    let signature_bytes = signed_quote_rpc.signature;

    let public_key = <K256Ecdsa as KeyType>::Public::from_bytes(&public_key_bytes)
        .expect("Failed to deserialize public key from response");

    let signature = <K256Ecdsa as KeyType>::Signature::from_bytes(&signature_bytes)
        .expect("Failed to deserialize signature from response");

    let is_valid = K256Ecdsa::verify(&public_key, &message_bytes, &signature);
    assert!(is_valid, "RPC response signature verification failed");

    assert_eq!(
        public_key_bytes,
        operator_signer.lock().await.public_key().to_bytes(),
        "Signer public key mismatch"
    );

    Ok(())
}
