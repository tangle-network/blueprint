use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache},
    benchmark::{BenchmarkProfile, CpuBenchmarkResult, MemoryBenchmarkResult},
    config::OperatorConfig,
    error::Result,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
    pricing::{PriceModel, ResourcePricing, load_pricing_from_toml, calculate_price},
    pricing_engine,
    pricing_engine::pricing_engine_client::PricingEngineClient,
    service::rpc::server::PricingEngineService,
    types::ResourceUnit,
};

use blueprint_core::{error, info};
use blueprint_testing_utils::{
    setup_log,
    tangle::{TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts},
};
use chrono::Utc;
use log::{error, info, warn};
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::{
    register::RegistrationArgs, request::RequestArgs,
};
use tempfile::tempdir;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tonic::transport::Channel;
use tonic::Request;

/// Test the full flow with a blueprint on a local Tangle Testnet
/// This test covers:
/// 1. Loading and validating TOML pricing configuration
/// 2. Setting up multiple pricing engines with different pricing strategies
/// 3. Requesting and comparing quotes from different operators
#[tokio::test]
async fn test_full_pricing_flow_with_blueprint() -> Result<()> {
    setup_log();

    // Start a Tangle Testnet with the Tangle Test Harness
    let (temp_dir, blueprint_dir) = create_test_blueprint();
    let harness: TangleTestHarness<()> = TangleTestHarness::setup(temp_dir).await.unwrap();

    std::env::set_current_dir(&blueprint_dir).unwrap();

    // Register all operators for service, but don't request service yet
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        skip_service_request: true,
        registration_args: vec![RegistrationArgs::default(); 3].try_into().unwrap(),
        request_args: RequestArgs::default(),
    };
    let (mut test_env, _service_id, blueprint_id) = harness
        .setup_services_with_options::<3>(setup_services_opts)
        .await
        .unwrap();
    test_env.initialize().await.unwrap();

    // Step 1: Create and validate a TOML pricing configuration
    let pricing_toml_content = r#"
# Default pricing configuration with all resource types
[default]
resources = [
  # CPU is priced higher as it's a primary resource
  { kind = "CPU", count = 1, price_per_unit_rate = 0.001 },
  
  # Memory is priced lower per MB
  { kind = "MemoryMB", count = 1024, price_per_unit_rate = 0.00005 },
  
  # Storage is priced similar to memory but slightly cheaper
  { kind = "StorageMB", count = 1024, price_per_unit_rate = 0.00002 },
  
  # Network has different rates for ingress and egress
  { kind = "NetworkEgressMB", count = 1024, price_per_unit_rate = 0.00003 },
  { kind = "NetworkIngressMB", count = 1024, price_per_unit_rate = 0.00001 },
  
  # GPU is a premium resource
  { kind = "GPU", count = 1, price_per_unit_rate = 0.005 },
  
  # Request-based pricing
  { kind = "Request", count = 1000, price_per_unit_rate = 0.0001 },
  
  # Function invocation pricing
  { kind = "Invocation", count = 1000, price_per_unit_rate = 0.0002 },
  
  # Execution time pricing
  { kind = "ExecutionTimeMS", count = 1000, price_per_unit_rate = 0.00001 }
]
"#;

    // Write the TOML content to a file and validate it
    let config_temp_dir = tempdir()?;
    let config_file_path = config_temp_dir.path().join("pricing.toml");
    fs::write(&config_file_path, pricing_toml_content)?;

    info!("TOML configuration content:\n{}", pricing_toml_content);

    // Load and validate the pricing data from TOML
    let pricing_data = load_pricing_from_toml(config_file_path.to_str().unwrap())?;

    // Debug: Print all loaded pricing data
    info!("Loaded pricing data:");
    for (key, resources) in &pricing_data {
        match key {
            Some(id) => info!("  Blueprint ID: {}", id),
            None => info!("  Default pricing"),
        }

        for resource in resources {
            info!(
                "    Resource: {}, Count: {}, Price: ${:.6}",
                resource.kind, resource.count, resource.price_per_unit_rate
            );
        }
    }

    // Step 2: Verify resource pricing
    let default_resources = pricing_data.get(&None).expect("Default pricing not found");

    // Check that each resource has a valid price
    for resource in default_resources {
        info!(
            "Verifying resource pricing for {}: ${:.6} per unit",
            resource.kind, resource.price_per_unit_rate
        );
        assert!(
            resource.price_per_unit_rate > 0.0,
            "Resource price should be positive"
        );
    }

    // Step 3: Calculate expected total cost
    let mut expected_total = 0.0;

    // Verify each resource type and collect expected prices
    let expected_resource_prices = [
        ("CPU", 0.001),
        ("MemoryMB", 0.00005),
        ("StorageMB", 0.00002),
        ("NetworkEgressMB", 0.00003),
        ("NetworkIngressMB", 0.00001),
        ("GPU", 0.005),
        ("Request", 0.0001),
        ("Invocation", 0.0002),
        ("ExecutionTimeMS", 0.00001),
    ];

    for (resource_name, expected_rate) in &expected_resource_prices {
        if let Some(resource) = default_resources
            .iter()
            .find(|r| format!("{}", r.kind) == *resource_name)
        {
            info!(
                "Verifying {} price: ${:.6} (expected ${:.6})",
                resource_name, resource.price_per_unit_rate, expected_rate
            );

            // Verify the price is close to the expected value
            assert!(
                (resource.price_per_unit_rate - expected_rate).abs() < 1e-6,
                "{} price doesn't match expected value",
                resource_name
            );

            // Add to the total price (price per unit * count)
            expected_total += resource.price_per_unit_rate * resource.count as f64;
        }
    }

    info!("Expected total cost per second: ${:.6}", expected_total);

    // Calculate the price based on the benchmark results
    let cpu_count = 2;
    let memory_mb = 1024;
    let resources = vec![
        ResourcePricing {
            kind: ResourceUnit::CPU,
            count: cpu_count,
            price_per_unit_rate: 0.001,
        },
        ResourcePricing {
            kind: ResourceUnit::MemoryMB,
            count: memory_mb,
            price_per_unit_rate: 0.00005,
        },
    ];
    
    // Create a pricing configuration for testing
    let mut pricing_config = HashMap::new();
    pricing_config.insert(None, resources.clone());
    
    // Create a benchmark profile for testing
    let profile = BenchmarkProfile {
        cpu_details: Some(CpuBenchmarkResult {
            avg_cores_used: cpu_count as f32,
            ..Default::default()
        }),
        memory_details: Some(MemoryBenchmarkResult {
            avg_memory_mb: memory_mb as f32,
            ..Default::default()
        }),
        ..Default::default()
    };
    
    // Calculate the price per second
    let price_per_second = calculate_price(profile, 1.0, &pricing_config, None)?;
    
    // Create the price model
    let model = PriceModel {
        resources: resources.clone(),
        total_cost: price_per_second.total_cost,
        benchmark_profile: None,
    };

    // Test cost calculation for different time periods
    let ttl_seconds = 3600u64; // 1 hour
    let expected_cost = price_per_second.total_cost;

    info!(
        "Expected cost for {} seconds: ${:.6}",
        ttl_seconds, expected_cost
    );

    // Step 4: Set up multiple pricing engines with different rate multipliers
    let mut cleanup_paths = Vec::new();
    let mut clients = Vec::new();
    let mut operator_ids = Vec::new();

    // Create 3 pricing engines with different rate multipliers
    let rate_multipliers = [1.0, 1.2, 1.4];

    for (i, &rate_multiplier) in rate_multipliers.iter().enumerate() {
        info!("Setting up pricing engine {} with rate multiplier {}", i, rate_multiplier);

        // Create a unique config for each operator
        let mut config = create_test_config();
        config.rate_multiplier = rate_multiplier;
        config.rpc_port = 9000 + i as u16;
        config.rpc_bind_address = format!("127.0.0.1:{}", config.rpc_port);
        config.keystore_path = format!("/tmp/test-keystore-{}", i);
        config.database_path = format!("./data/test_price_cache_{}", i);

        // Initialize the operator signer
        let keystore_path = config.keystore_path.clone();
        let signer = init_operator_signer(&config, &keystore_path)?;
        let operator_id = signer.lock().await.operator_id();
        operator_ids.push(operator_id);

        // Create config Arc for price cache
        let config_arc = Arc::new(config.clone());
        
        // Initialize the price cache
        let cache = init_price_cache(&config_arc).await?;
        
        // Store pricing data in cache
        for (blueprint_id_opt, resources) in &pricing_data {
            if let Some(blueprint_id) = blueprint_id_opt {
                let price_per_second = resources.iter()
                    .map(|r| r.price_per_unit_rate * r.count as f64)
                    .sum::<f64>() * config.rate_multiplier;
                
                let model = PriceModel {
                    resources: resources.clone(),
                    total_cost: price_per_second,
                    benchmark_profile: None,
                };
                
                cache.store_price(*blueprint_id, &model)?;
            }
        }

        // Start the RPC server in a background task
        let service = PricingEngineService::new(config_arc.clone(), cache.clone(), signer.clone());
        
        let addr = config.rpc_bind_address.parse().unwrap();
        tokio::spawn(async move {
            let server = tonic::transport::Server::builder()
                .add_service(pricing_engine::pricing_engine_server::PricingEngineServer::new(service))
                .serve(addr);
                
            if let Err(e) = server.await {
                error!("Server error: {}", e);
            }
        });

        // Connect to the RPC server
        sleep(tokio::time::Duration::from_millis(100)).await;
        let channel = match Channel::from_shared(format!("http://{}", config.rpc_bind_address))
            .unwrap()
            .connect()
            .await {
                Ok(channel) => channel,
                Err(e) => {
                    error!("Failed to connect to RPC server: {}", e);
                    continue;
                }
            };
            
        let client = PricingEngineClient::new(channel);
        clients.push(client);

        // Add paths to clean up later
        cleanup_paths.push((config.database_path, config.keystore_path));

        info!("Pricing engine {} started", i);
    }

    // Step 5: Request quotes from all operators
    let mut quote_responses = Vec::new();

    // Generate a proof of work for the request
    let challenge = generate_challenge(blueprint_id, Utc::now().timestamp() as u64);
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

    for (i, client) in clients.iter_mut().enumerate() {
        // Create a request for the pricing engine
        let request = pricing_engine::GetPriceRequest {
            blueprint_id,
            ttl_seconds,
            resource_requirements: vec![
                pricing_engine::ResourceRequirement {
                    kind: "CPU".to_string(),
                    count: 2,
                },
                pricing_engine::ResourceRequirement {
                    kind: "MemoryMB".to_string(),
                    count: 1024,
                },
            ],
            proof_of_work: proof.clone(),
        };

        info!("Requesting quote from operator {}", i);

        // Get a price quote from the pricing engine
        let response = match client.get_price(request).await {
            Ok(response) => response,
            Err(status) => {
                error!("gRPC error from operator {}: {}", i, status);
                continue;
            }
        };

        let response_ref = response.get_ref();

        // Print the response details
        info!("Received quote from operator {}:", i);
        info!("  Operator ID: {:?}", response_ref.operator_id);

        if let Some(details) = &response_ref.quote_details {
            info!("  Total Cost: ${:.6}", details.total_cost_rate);
            info!("  TTL: {} seconds", details.ttl_seconds);
            info!("  Timestamp: {}", details.timestamp);
            info!("  Expiry: {}", details.expiry);

            // Store the quote details for comparison
            quote_responses.push((i, details.clone(), response_ref.operator_id.clone()));
        } else {
            info!("  No quote details provided");
        }
    }

    // Print a summary of all quotes
    info!("\n=== Quote Summary ===");
    if quote_responses.is_empty() {
        info!("No quotes received from any operator");
    } else {
        info!("Received {} quotes:", quote_responses.len());

        // Sort quotes by total cost
        quote_responses.sort_by(|a, b| {
            a.1.total_cost_rate.partial_cmp(&b.1.total_cost_rate).unwrap_or(std::cmp::Ordering::Equal)
        });

        for (i, details, operator_id) in &quote_responses {
            info!(
                "Operator {}: ${:.6} (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }

        // Identify the cheapest quote
        if let Some((i, details, operator_id)) = quote_responses.first() {
            info!(
                "\nCheapest quote is from Operator {}: ${:.6} (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }
    }

    // Step 6: Verify the total cost calculation
    // Calculate expected cost for a 1-hour TTL
    info!("\n=== Cost Calculation Verification ===");
    info!("Base price per second: ${:.6}", expected_total);
    info!(
        "Expected cost for {} seconds: ${:.6}",
        ttl_seconds, expected_cost
    );
    info!("Operator rate multipliers: 1.0, 1.2, 1.4");

    // Clean up
    for (cache_path, keystore_path) in cleanup_paths {
        let _ = std::fs::remove_dir_all(cache_path);
        let _ = std::fs::remove_dir_all(keystore_path);
    }

    Ok(())
}

// Helper function to create a test configuration
fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        keystore_path: "/tmp/test-keystore".to_string(),
        database_path: "./data/test_price_cache".to_string(),
        rpc_port: 9000,
        rpc_bind_address: "127.0.0.1:9000".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["benchmark".to_string()],
        benchmark_duration: 10,
        benchmark_interval: 1,
        rate_multiplier: 1.0,
        keypair_path: "/tmp/test-keypair".to_string(),
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300,
    }
}
