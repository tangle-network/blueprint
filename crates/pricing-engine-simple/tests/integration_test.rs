use std::path::PathBuf;

use blueprint_pricing_engine_simple_lib::{
    app::{init_price_cache, load_operator_config},
    cache::PriceCache,
    config::{OperatorConfig, load_config_from_path},
    error::Result,
    pricing::PriceModel,
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
        keypair_path: "/tmp/test-keypair".to_string(),
        rpc_bind_address: "127.0.0.1".to_string(),
        rpc_port: 9000,
        rpc_timeout: 30,
        rpc_max_connections: 100,
    }
}

// Create a test price model
fn create_test_price_model() -> PriceModel {
    PriceModel {
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

    // Create a blueprint hash and price model
    let blueprint_hash = "test-blueprint-hash".to_string();
    let price_model = create_test_price_model();

    // Store the price model in the cache
    price_cache.store_price(&blueprint_hash, &price_model)?;

    // Retrieve the price model from the cache
    let retrieved_model = price_cache.get_price(&blueprint_hash)?;
    assert!(retrieved_model.is_some());

    // Try to retrieve a non-existent blueprint hash
    let non_existent_hash = "non-existent-hash".to_string();
    let non_existent_model = price_cache.get_price(&non_existent_hash)?;
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
        r#"
        database_path = "./data/price_cache"
        benchmark_command = "echo"
        benchmark_args = ["test"]
        benchmark_duration = 10
        benchmark_interval = 1
        price_scaling_factor = 1000000.0
        keypair_path = "/tmp/test-keypair"
        rpc_bind_address = "127.0.0.1"
        rpc_port = 9000
        rpc_timeout = 30
        rpc_max_connections = 100
        "#,
    )
    .unwrap();

    // Test loading the operator config
    let config = load_operator_config(&config_path).await?;

    // Test initializing the price cache
    let price_cache = init_price_cache(&config).await?;

    // Verify the cache is initialized
    let blueprint_hash = "test-blueprint".to_string();
    let price = price_cache.get_price(&blueprint_hash)?;
    assert!(price.is_none());

    // Clean up
    let _ = std::fs::remove_file(config_path);

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
        r#"
        database_path = "./data/price_cache"
        benchmark_command = "echo"
        benchmark_args = ["test"]
        benchmark_duration = 10
        benchmark_interval = 1
        price_scaling_factor = 1000000.0
        keypair_path = "/tmp/test-keypair"
        rpc_bind_address = "127.0.0.1"
        rpc_port = 9000
        rpc_timeout = 30
        rpc_max_connections = 100
        "#,
    )
    .unwrap();

    // Load the config
    let config = load_config_from_path(&config_path)?;

    // Verify the config was loaded correctly
    assert_eq!(config.database_path, "./data/price_cache");
    assert_eq!(config.benchmark_command, "echo");
    assert_eq!(config.price_scaling_factor, 1000000.0);
    assert_eq!(config.rpc_port, 9000);

    // Clean up
    let _ = std::fs::remove_file(config_path);

    Ok(())
}
