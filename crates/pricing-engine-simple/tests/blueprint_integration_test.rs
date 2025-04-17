use std::sync::Arc;

use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_price_cache},
    config::OperatorConfig,
    error::Result,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
    pricing::{PriceModel, ResourcePricing},
    pricing_engine,
    pricing_engine::pricing_engine_client::PricingEngineClient,
    pricing_engine::pricing_engine_server::PricingEngineServer,
    service::rpc::server::PricingEngineService,
    types::ResourceUnit,
};
use blueprint_testing_utils::{
    setup_log,
    tangle::{TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts},
};
use chrono::Utc;
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::{
    register::RegistrationArgs, request::RequestArgs,
};
use tonic::transport::{Channel, Server};

// Test the full flow with a blueprint on a local Tangle Testnet
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
    let (mut test_env, service_id, blueprint_id) = harness
        .setup_services_with_options::<3>(setup_services_opts)
        .await
        .unwrap();
    test_env.initialize().await.unwrap();

    // Step 1: Set up multiple pricing engines with different price models
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

        // Create a price model with different pricing based on operator index
        let mut price_model = create_test_price_model();
        price_model.price_per_second_rate = 1000 * (i as u128 + 1); // Different prices for each operator
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
                eprintln!("Error starting server: {}", e);
            }
        });

        // Store services and related objects for later use
        pricing_services.push(pricing_service);
        operator_signers.push(operator_signer);
        price_caches.push(price_cache);

        println!(
            "Started pricing engine server {} at {}",
            i, config.rpc_bind_address
        );
    }

    // Give the servers a moment to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Step 2: Simulate a user (Alice) requesting quotes from all operators
    let ttl_seconds = 3600; // 1 hour
    let timestamp = Utc::now().timestamp() as u64;

    // Generate a proof of work
    let challenge = generate_challenge(blueprint_id, timestamp);
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

    println!("\n=== Alice is requesting quotes from all operators ===");

    // Store all responses for comparison
    let mut quote_responses = Vec::new();

    // Send request to each pricing engine
    for i in 0..3 {
        let addr = format!("http://127.0.0.1:{}", 9000 + i);
        println!("Connecting to pricing engine at {}", addr);

        // Create a client
        let channel = match Channel::from_shared(addr).unwrap().connect().await {
            Ok(channel) => channel,
            Err(e) => {
                eprintln!("Failed to connect to pricing engine {}: {}", i, e);
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

        println!("Requesting quote from operator {}", i);

        // Get a price quote from the pricing engine
        let response = match client.get_price(request).await {
            Ok(response) => response,
            Err(status) => {
                eprintln!("gRPC error from operator {}: {}", i, status);
                continue;
            }
        };

        let response_ref = response.get_ref();

        // Print the response details
        println!("Received quote from operator {}:", i);
        println!("  Operator ID: {:?}", response_ref.operator_id);

        if let Some(details) = &response_ref.quote_details {
            println!("  Total Cost: {} wei", details.total_cost_rate);
            println!("  TTL: {} seconds", details.ttl_seconds);
            println!("  Timestamp: {}", details.timestamp);
            println!("  Expiry: {}", details.expiry);

            // Store the quote details for comparison
            quote_responses.push((i, details.clone(), response_ref.operator_id.clone()));
        } else {
            println!("  No quote details provided");
        }
        println!();
    }

    // Print a summary of all quotes
    println!("\n=== Quote Summary ===");
    if quote_responses.is_empty() {
        println!("No quotes received from any operator");
    } else {
        println!("Received {} quotes:", quote_responses.len());

        // Sort quotes by total cost
        quote_responses.sort_by(|a, b| {
            let a_cost = a.1.total_cost_rate.parse::<u128>().unwrap_or(0);
            let b_cost = b.1.total_cost_rate.parse::<u128>().unwrap_or(0);
            a_cost.cmp(&b_cost)
        });

        for (i, details, operator_id) in &quote_responses {
            println!(
                "Operator {}: {} wei (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }

        // Identify the cheapest quote
        if let Some((i, details, operator_id)) = quote_responses.first() {
            println!(
                "\nCheapest quote is from Operator {}: {} wei (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }
    }

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

// Helper function to create a test price model
fn create_test_price_model() -> PriceModel {
    PriceModel {
        price_per_second_rate: 1000,
        resources: vec![
            ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 1,
                price_per_unit_rate: 500,
            },
            ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_rate: 100,
            },
        ],
        generated_at: Utc::now(),
        benchmark_profile: None,
    }
}
