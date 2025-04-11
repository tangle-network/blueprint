use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache},
    benchmark::{BenchmarkProfile, CpuBenchmarkResult},
    cache::BlueprintHash,
    config::OperatorConfig,
    error::Result,
    pricing::{self, PriceModel},
    service::blockchain::event::BlockchainEvent,
    signer::{BlueprintId, QuotePayload},
};

use blueprint_crypto_core::BytesEncoding;
use chrono::Utc;

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
    }
}

// Helper function to create a test blueprint hash
fn create_test_blueprint_hash() -> BlueprintHash {
    // Create a deterministic test hash (32 bytes of data)
    let hash_bytes = [0u8; 32];
    // Convert to hex string
    hex::encode(hash_bytes)
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

// Helper function to simulate a blockchain event
fn create_test_blockchain_event() -> BlockchainEvent {
    // Create a mock Registered event
    // This is a simplified version that avoids using the actual Registered struct
    // which has fields that are difficult to construct in a test
    BlockchainEvent::Unknown("Test event".to_string())
}

#[tokio::test]
async fn test_pricing_engine_components() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Step 1: Set up configuration and components
    let config = Arc::new(create_test_config());

    // Create keystore directory if it doesn't exist
    let keystore_path = std::path::Path::new(&config.keystore_path);
    if !keystore_path.exists() {
        std::fs::create_dir_all(keystore_path)?;
    }

    // Create cache directory if it doesn't exist
    let cache_path = std::path::Path::new(&config.database_path);
    if !cache_path.exists() {
        std::fs::create_dir_all(cache_path)?;
    }

    // Initialize price cache
    let price_cache = init_price_cache(&config).await?;

    // Initialize operator signer
    let operator_signer = init_operator_signer(&config).await?;

    // Step 2: Add a test price model to the cache
    let blueprint_hash = create_test_blueprint_hash();
    let price_model = create_test_price_model();
    price_cache.store_price(&blueprint_hash, &price_model)?;

    // Step 3: Verify the price model was stored correctly
    let retrieved_model = price_cache.get_price(&blueprint_hash)?;
    assert!(
        retrieved_model.is_some(),
        "Price model should be in the cache"
    );

    let retrieved_model = retrieved_model.unwrap();
    assert_eq!(
        retrieved_model.price_per_second_wei, price_model.price_per_second_wei,
        "Retrieved price should match stored price"
    );

    // Step 4: Test the operator signer
    // Extract the public key for verification
    let public_key = operator_signer.public_key();

    // Create a test quote payload
    let blueprint_id: BlueprintId = 12345;
    let quote_payload = QuotePayload {
        blueprint_id,
        price_wei: 1000000,
        expiry: 1000,
        timestamp: 500,
    };

    // Extract the signer from Arc for testing
    // This will fail if there are other references to the Arc
    let mut operator_signer = match Arc::try_unwrap(operator_signer) {
        Ok(signer) => signer,
        Err(_) => panic!("Could not unwrap Arc, there are other references"),
    };

    // Sign the quote
    let signed_quote = operator_signer.sign_quote(quote_payload)?;

    // Verify the signature is present
    assert!(
        signed_quote.signature.to_bytes().len() > 0,
        "Signature should not be empty"
    );

    // Verify the public key matches
    assert_eq!(
        signed_quote.signer_pubkey.to_bytes(),
        public_key.to_bytes(),
        "Signer public key should match"
    );

    // Step 5: Test price calculation
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

    // Verify the price calculation
    assert!(
        calculated_price.price_per_second_wei > 0,
        "Calculated price should be greater than zero"
    );

    // Verify the benchmark profile was included
    assert!(
        calculated_price.benchmark_profile.is_some(),
        "Benchmark profile should be included in the price model"
    );

    Ok(())
}

// Test for simulating blockchain events
#[tokio::test]
async fn test_blockchain_event_handling() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Set up configuration and components
    let config = Arc::new(create_test_config());

    // Create cache directory if it doesn't exist
    let cache_path = std::path::Path::new(&config.database_path);
    if !cache_path.exists() {
        std::fs::create_dir_all(cache_path)?;
    }

    // Initialize price cache
    let price_cache = init_price_cache(&config).await?;

    // Create a channel for blockchain events
    let (event_tx, event_rx) = mpsc::channel::<BlockchainEvent>(10);

    // Start the event processor
    let processor_handle = blueprint_pricing_engine_simple_lib::app::spawn_event_processor(
        event_rx,
        price_cache.clone(),
        config.clone(),
    );

    // Create and send a test blockchain event
    let event = create_test_blockchain_event();
    event_tx.send(event).await.expect("Failed to send event");

    // Give the processor time to process the event
    sleep(Duration::from_millis(500)).await;

    // Shut down the processor
    processor_handle.abort();

    Ok(())
}
