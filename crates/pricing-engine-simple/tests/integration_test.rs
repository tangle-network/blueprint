use std::path::PathBuf;
use std::sync::Arc;

use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache, load_operator_config},
    cache::PriceCache,
    config::{OperatorConfig, load_config_from_path},
    error::Result,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof, verify_proof},
    pricing::PriceModel,
    signer::QuotePayload,
    types::ResourceUnit,
};
use chrono::Utc;

// Create a test config
fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        database_path: "./data/price_cache".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["test".to_string()],
        benchmark_duration: 10,
        benchmark_interval: 1,
        price_scaling_factor: 1000000.0,
        quote_validity_duration_secs: 300,
        keypair_path: "/tmp/test-keypair".to_string(),
        keystore_path: "/tmp/test-keystore".to_string(),
        rpc_bind_address: "127.0.0.1:9000".to_string(),
        rpc_port: 9000,
        rpc_timeout: 30,
        rpc_max_connections: 100,
    }
}

// Create a test price model
fn create_test_price_model() -> PriceModel {
    PriceModel {
        resources: vec![
            blueprint_pricing_engine_simple_lib::pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 2,
                price_per_unit_wei: 500000,
            },
            blueprint_pricing_engine_simple_lib::pricing::ResourcePricing {
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

// Initialize logging for tests
fn init_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("debug"))
        .try_init();
}

#[tokio::test]
async fn test_price_cache_operations() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Create a test config
    let config = create_test_config();

    // Initialize price cache
    let price_cache = PriceCache::new(&config.database_path)?;

    // Create a blueprint ID and price model
    let blueprint_id: u64 = 12345;
    let price_model = create_test_price_model();

    // Store the price model in the cache
    price_cache.store_price(blueprint_id, &price_model)?;

    // Retrieve the price model from the cache
    let retrieved_model = price_cache.get_price(blueprint_id)?;
    assert!(retrieved_model.is_some());

    // Try to retrieve a non-existent blueprint ID
    let non_existent_id: u64 = 99999;
    let non_existent_model = price_cache.get_price(non_existent_id)?;
    assert!(non_existent_model.is_none());

    Ok(())
}

#[tokio::test]
async fn test_app_functions() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Create a test config file
    let config_path = PathBuf::from("/tmp/test-operator-config.toml");
    std::fs::write(
        &config_path,
        r#"database_path = "./data/price_cache"
benchmark_command = "echo"
benchmark_args = ["test"]
benchmark_duration = 10
benchmark_interval = 1
price_scaling_factor = 1000000.0
keypair_path = "/tmp/test-keypair"
keystore_path = "/tmp/test-keystore"
rpc_bind_address = "127.0.0.1:9000"
rpc_port = 9000
rpc_timeout = 30
rpc_max_connections = 100
quote_validity_duration_secs = 300
"#,
    )
    .unwrap();

    // Test loading the operator config
    let config = load_operator_config(&config_path).await?;

    // Test initializing the price cache
    let price_cache = init_price_cache(&config).await?;

    // Verify the cache is initialized
    let blueprint_id: u64 = 12345;
    let price = price_cache.get_price(blueprint_id)?;
    assert!(price.is_none());

    // Clean up
    let _ = std::fs::remove_file(config_path);
    let _ = std::fs::remove_dir_all("/tmp/test-keystore");

    Ok(())
}

#[tokio::test]
async fn test_config_loading() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Create a test config file
    let config_path = PathBuf::from("/tmp/test-operator-config.toml");
    std::fs::write(
        &config_path,
        r#"database_path = "./data/price_cache"
benchmark_command = "echo"
benchmark_args = ["test"]
benchmark_duration = 10
benchmark_interval = 1
price_scaling_factor = 1000000.0
keypair_path = "/tmp/test-keypair"
keystore_path = "/tmp/test-keystore"
rpc_bind_address = "127.0.0.1:9000"
rpc_port = 9000
rpc_timeout = 30
rpc_max_connections = 100
quote_validity_duration_secs = 300
"#,
    )
    .unwrap();

    // Load the config
    let config = load_config_from_path(&config_path)?;

    // Verify the config was loaded correctly
    assert_eq!(config.database_path, "./data/price_cache");
    assert_eq!(config.benchmark_command, "echo");
    assert_eq!(config.price_scaling_factor, 1000000.0);
    assert_eq!(config.keystore_path, "/tmp/test-keystore");
    assert_eq!(config.rpc_port, 9000);
    assert_eq!(config.rpc_bind_address, "127.0.0.1:9000");
    assert_eq!(config.quote_validity_duration_secs, 300);

    // Clean up
    let _ = std::fs::remove_file(config_path);

    Ok(())
}

#[tokio::test]
async fn test_operator_signer() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Create a test config
    let config = Arc::new(create_test_config());

    // Create keystore directory if it doesn't exist
    let keystore_path = std::path::Path::new(&config.keystore_path);
    if !keystore_path.exists() {
        std::fs::create_dir_all(keystore_path)?;
    }

    // Initialize the operator signer
    let operator_signer_arc = init_operator_signer(&config, &config.keystore_path)?;

    // Extract the public key for later comparison
    let _public_key = operator_signer_arc.lock().await.public_key();
    let _operator_id = operator_signer_arc.lock().await.operator_id();

    // Create a test quote payload
    let payload = QuotePayload {
        blueprint_id: 123,
        ttl_seconds: 3600,
        total_cost_wei: 1000000,
        expiry: (Utc::now().timestamp() + 3600) as u64, // 1 hour from now
        timestamp: Utc::now().timestamp() as u64,
        resources: vec![
            blueprint_pricing_engine_simple_lib::pricing::ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 1,
                price_per_unit_wei: 1000000,
            },
            blueprint_pricing_engine_simple_lib::pricing::ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 512,
                price_per_unit_wei: 500,
            },
        ],
    };

    // Generate a proof of work for testing
    let challenge = generate_challenge(payload.blueprint_id, payload.timestamp);
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

    // Sign the payload
    let signed_quote = operator_signer_arc
        .lock()
        .await
        .sign_quote(payload.clone(), proof.clone())?;

    // Verify the signature
    assert_eq!(signed_quote.payload.blueprint_id, 123);
    assert_eq!(signed_quote.payload.ttl_seconds, 3600);
    assert_eq!(signed_quote.payload.total_cost_wei, 1000000);
    assert_eq!(signed_quote.proof_of_work, proof);

    // Clean up - but don't remove the keystore directory since other tests might use it
    // Let the operating system clean it up when the test process exits
    // let _ = std::fs::remove_dir_all("/tmp/test-keystore");

    Ok(())
}

#[tokio::test]
async fn test_proof_of_work() -> Result<()> {
    // Initialize logging
    init_test_logging();

    // Test parameters
    let blueprint_id = 12345;
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

    // Test with different difficulty levels
    let lower_difficulty = DEFAULT_POW_DIFFICULTY - 4;
    let lower_proof = generate_proof(&challenge, lower_difficulty).await?;

    // Lower difficulty proof should be faster to generate
    let is_valid_lower = verify_proof(&challenge, &lower_proof, lower_difficulty)?;
    assert!(is_valid_lower, "Lower difficulty proof verification failed");

    // Lower difficulty proof should not pass higher difficulty verification
    let is_valid_higher = verify_proof(&challenge, &lower_proof, DEFAULT_POW_DIFFICULTY)?;
    assert!(
        !is_valid_higher,
        "Lower difficulty proof should not pass higher difficulty verification"
    );

    Ok(())
}
