use std::fs;
use std::sync::Arc;

use blueprint_core::{error, info};
use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache},
    config::OperatorConfig,
    error::Result,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
    pricing::{PriceModel, load_pricing_from_toml},
    pricing_engine,
    pricing_engine::pricing_engine_client::PricingEngineClient,
    pricing_engine::pricing_engine_server::PricingEngineServer,
    service::rpc::server::PricingEngineService,
};
use blueprint_testing_utils::{
    setup_log,
    tangle::{TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts},
};
use chrono::Utc;
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::{
    register::RegistrationArgs, request::RequestArgs,
};
use tempfile::tempdir;
use tonic::transport::{Channel, Server};

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
  { kind = "CPU", count = 1, price_per_unit_rate = 100 },
  
  # Memory is priced lower per MB
  { kind = "MemoryMB", count = 1024, price_per_unit_rate = 1 },
  
  # Storage is priced similar to memory but slightly cheaper
  { kind = "StorageMB", count = 1024, price_per_unit_rate = 1 },
  
  # Network has different rates for ingress and egress
  { kind = "NetworkEgressMB", count = 1024, price_per_unit_rate = 2 },
  { kind = "NetworkIngressMB", count = 1024, price_per_unit_rate = 1 },
  
  # GPU is a premium resource
  { kind = "GPU", count = 1, price_per_unit_rate = 500 },
  
  # Request-based pricing
  { kind = "Request", count = 1000, price_per_unit_rate = 5 },
  
  # Function invocation pricing
  { kind = "Invocation", count = 1000, price_per_unit_rate = 10 },
  
  # Execution time pricing
  { kind = "ExecutionTimeMS", count = 1000, price_per_unit_rate = 1 }
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
                "    Resource: {}, Count: {}, Price: {}",
                resource.kind, resource.count, resource.price_per_unit_rate
            );
        }
    }

    // Validate default pricing exists
    let default_resources = pricing_data
        .get(&None)
        .expect("Default pricing should be available");
    assert!(
        !default_resources.is_empty(),
        "Default pricing should not be empty"
    );

    // Define expected resource types and rates (all integer values)
    let expected_resources = [
        ("CPU", 1, 100),
        ("MemoryMB", 1024, 1),
        ("StorageMB", 1024, 1),
        ("NetworkEgressMB", 1024, 2),
        ("NetworkIngressMB", 1024, 1),
        ("GPU", 1, 500),
        ("Request", 1000, 5),
        ("Invocation", 1000, 10),
        ("ExecutionTimeMS", 1000, 1),
    ];

    // Verify each resource type exists with correct pricing
    for (resource_name, expected_count, expected_rate) in expected_resources.iter() {
        let resource = default_resources
            .iter()
            .find(|r| r.kind.to_string() == *resource_name);
        assert!(
            resource.is_some(),
            "Resource {} should be present",
            resource_name
        );

        let resource = resource.unwrap();
        assert_eq!(
            resource.count, *expected_count as u64,
            "{} count doesn't match expected value",
            resource_name
        );

        assert_eq!(
            resource.price_per_unit_rate, *expected_rate as u128,
            "{} price doesn't match expected value",
            resource_name
        );

        info!(
            "Verified {}: Count = {}, Rate = {}",
            resource_name, resource.count, resource.price_per_unit_rate
        );
    }

    // Calculate expected total price per second based on all resources
    let mut expected_total: u128 = 0;
    for resource in default_resources {
        expected_total = expected_total.saturating_add(
            resource
                .price_per_unit_rate
                .saturating_mul(resource.count as u128),
        );
    }

    info!("Pricing validation successful");
    info!("  Total price per second: {} rate units", expected_total);

    // Step 2: Set up multiple pricing engines with different price models
    let mut pricing_services = Vec::new();
    let mut operator_signers = Vec::new();
    let mut price_caches = Vec::new();
    let mut cleanup_paths = Vec::new();

    // Create and start pricing engine servers for each operator
    for i in 0..3 {
        let mut config = create_test_config();
        config.database_path = format!("./data/test_price_cache_{}", i);
        config.keystore_path = format!("/tmp/test-keystore_{}", i);
        config.rpc_port = 9000 + i;
        config.rpc_bind_address = format!("127.0.0.1:{}", 9000 + i);

        // Set different rate multipliers for each operator to simulate different pricing strategies
        config.rate_multiplier = 1.0 + (i as f64 * 0.2); // 1.0, 1.2, 1.4

        let config = Arc::new(config);

        // Create necessary directories
        let keystore_path = std::path::Path::new(&config.keystore_path);
        if !keystore_path.exists() {
            std::fs::create_dir_all(keystore_path)?;
        }

        let cache_path = std::path::Path::new(&config.database_path);
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_path)?;
        }

        cleanup_paths.push((cache_path.to_path_buf(), keystore_path.to_path_buf()));

        // Initialize price cache
        let price_cache = init_price_cache(&config).await?;

        // Initialize operator signer
        let operator_signer = init_operator_signer(&config, &config.keystore_path)?;

        // Create a temporary TOML file for this operator
        let temp_pricing_dir = tempdir()?;
        let pricing_file_path = temp_pricing_dir.path().join("pricing.toml");
        fs::write(&pricing_file_path, pricing_toml_content)?;

        // Load pricing from TOML
        let pricing_data = load_pricing_from_toml(pricing_file_path.to_str().unwrap())?;

        // Get default resources
        let default_resources = pricing_data
            .get(&None)
            .expect("Default pricing should be available")
            .clone();

        // Calculate price per second based on resources and operator-specific rate multiplier
        let mut total_price_per_second: u128 = 0;
        for resource in &default_resources {
            total_price_per_second = total_price_per_second.saturating_add(
                resource
                    .price_per_unit_rate
                    .saturating_mul(resource.count as u128),
            );
        }

        // Apply the operator's rate multiplier
        total_price_per_second = (total_price_per_second as f64 * config.rate_multiplier) as u128;

        // Create a price model with the calculated price per second
        let price_model = PriceModel {
            resources: default_resources,
            price_per_second_rate: total_price_per_second,
            generated_at: Utc::now(),
            benchmark_profile: None,
        };

        // Store the price model in the cache
        price_cache.store_price(blueprint_id, &price_model)?;

        // Create a pricing engine service
        let pricing_service =
            PricingEngineService::new(config.clone(), price_cache.clone(), operator_signer.clone());

        // Start the gRPC server in the background
        let server_service =
            PricingEngineService::new(config.clone(), price_cache.clone(), operator_signer.clone());

        // Start the server in a separate task
        let addr = config.rpc_bind_address.parse().unwrap();
        tokio::spawn(async move {
            if let Err(e) = Server::builder()
                .add_service(PricingEngineServer::new(server_service))
                .serve(addr)
                .await
            {
                error!("Error starting server: {}", e);
            }
        });

        // Store services and related objects for later use
        pricing_services.push(pricing_service);
        operator_signers.push(operator_signer);
        price_caches.push(price_cache);

        info!(
            "Started pricing engine server {} at {} with rate multiplier {}",
            i, config.rpc_bind_address, config.rate_multiplier
        );
    }

    // Give the servers a moment to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Step 3: Simulate a user (Alice) requesting quotes from all operators
    let ttl_seconds = 3600; // 1 hour
    let timestamp = Utc::now().timestamp() as u64;

    // Generate a proof of work
    let challenge = generate_challenge(blueprint_id, timestamp);
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

    info!("\n=== Alice is requesting quotes from all operators ===");

    // Store all responses for comparison
    let mut quote_responses = Vec::new();

    // Send request to each pricing engine
    for i in 0..3 {
        let addr = format!("http://127.0.0.1:{}", 9000 + i);
        info!("Connecting to pricing engine at {}", addr);

        // Create a client
        let channel = match Channel::from_shared(addr).unwrap().connect().await {
            Ok(channel) => channel,
            Err(e) => {
                error!("Failed to connect to pricing engine {}: {}", i, e);
                continue;
            }
        };

        let mut client = PricingEngineClient::new(channel);

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
            info!("  Total Cost: {} rate units", details.total_cost_rate);
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
            let a_cost = a.1.total_cost_rate.parse::<u128>().unwrap_or(0);
            let b_cost = b.1.total_cost_rate.parse::<u128>().unwrap_or(0);
            a_cost.cmp(&b_cost)
        });

        for (i, details, operator_id) in &quote_responses {
            info!(
                "Operator {}: {} rate units (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }

        // Identify the cheapest quote
        if let Some((i, details, operator_id)) = quote_responses.first() {
            info!(
                "\nCheapest quote is from Operator {}: {} rate units (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }
    }

    // Step 4: Verify the total cost calculation
    // Calculate expected cost for a 1-hour TTL
    let expected_cost = expected_total.saturating_mul(ttl_seconds as u128);
    info!("\n=== Cost Calculation Verification ===");
    info!("Base price per second: {} rate units", expected_total);
    info!(
        "Expected cost for {} seconds: {} rate units",
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
