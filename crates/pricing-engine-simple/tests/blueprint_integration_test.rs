use std::collections::HashMap;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use blueprint_pricing_engine_simple_lib::{
    app::{init_operator_signer, init_benchmark_cache},
    benchmark::{BenchmarkProfile, CpuBenchmarkResult, MemoryBenchmarkResult, NetworkBenchmarkResult, StorageBenchmarkResult, GpuBenchmarkResult, IoBenchmarkResult},
    config::OperatorConfig,
    error::Result,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
    pricing::{PriceModel, ResourcePricing, load_pricing_from_toml, calculate_price},
    pricing_engine,
    pricing_engine::pricing_engine_client::PricingEngineClient,
    service::rpc::server::PricingEngineService,
    types::ResourceUnit,
    init_pricing_config,
};

use blueprint_core::{error, info, warn};
use blueprint_testing_utils::{
    setup_log,
    tangle::{TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts},
};
use chrono::Utc;
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
  { kind = "CPU", count = 1, price_per_unit_rate = 0.000001 },
  
  # Memory is priced lower per MB
  { kind = "MemoryMB", count = 1024, price_per_unit_rate = 0.00000005 },
  
  # Storage is priced similar to memory but slightly cheaper
  { kind = "StorageMB", count = 1024, price_per_unit_rate = 0.00000002 },
  
  # Network has different rates for ingress and egress
  { kind = "NetworkEgressMB", count = 1024, price_per_unit_rate = 0.00000003 },
  { kind = "NetworkIngressMB", count = 1024, price_per_unit_rate = 0.00000001 },
  
  # GPU is a premium resource
  { kind = "GPU", count = 1, price_per_unit_rate = 0.000005 },
  
  # Request-based pricing
  { kind = "Request", count = 1000, price_per_unit_rate = 0.0000001 },
  
  # Function invocation pricing
  { kind = "Invocation", count = 1000, price_per_unit_rate = 0.0000002 },
  
  # Execution time pricing
  { kind = "ExecutionTimeMS", count = 1000, price_per_unit_rate = 0.00000001 }
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
        let blueprint_id_str = match key {
            Some(id) => id.to_string(),
            None => "default".to_string(),
        };
        info!("Blueprint ID: {}", blueprint_id_str);
        for resource in resources {
            info!(
                "  Resource: {} - Count: {} - Rate: ${:.6}",
                resource.kind, resource.count, resource.price_per_unit_rate
            );
        }
    }

    // Step 2: Create a benchmark profile
    let benchmark_profile = BenchmarkProfile {
        job_id: "test-job".to_string(),
        execution_mode: "native".to_string(),
        duration_secs: 60,
        timestamp: Utc::now().timestamp() as u64,
        success: true,
        cpu_details: Some(CpuBenchmarkResult {
            num_cores_detected: 2,
            avg_cores_used: 2.0,
            avg_usage_percent: 50.0,
            peak_cores_used: 2.0,
            peak_usage_percent: 80.0,
            benchmark_duration_ms: 1000,
            primes_found: 1000,
            max_prime: 20000,
            primes_per_second: 1000.0,
            cpu_model: "Test CPU".to_string(),
            cpu_frequency_mhz: 2500.0,
        }),
        memory_details: Some(MemoryBenchmarkResult {
            avg_memory_mb: 1024.0,
            peak_memory_mb: 1536.0,
            block_size_kb: 64,
            total_size_mb: 1024,
            operations_per_second: 1000.0,
            transfer_rate_mb_s: 2000.0,
            access_mode: blueprint_pricing_engine_simple_lib::benchmark::MemoryAccessMode::Sequential,
            operation_type: blueprint_pricing_engine_simple_lib::benchmark::MemoryOperationType::Read,
            latency_ns: 50.0,
            duration_ms: 1000,
        }),
        storage_details: Some(StorageBenchmarkResult {
            storage_available_gb: 100.0,
        }),
        network_details: Some(NetworkBenchmarkResult {
            network_rx_mb: 100.0,
            network_tx_mb: 50.0,
            download_speed_mbps: 100.0,
            upload_speed_mbps: 50.0,
            latency_ms: 20.0,
            duration_ms: 1000,
            packet_loss_percent: 0.1,
            jitter_ms: 2.0,
        }),
        io_details: Some(IoBenchmarkResult {
            read_mb: 100.0,
            write_mb: 80.0,
            read_iops: 500.0,
            write_iops: 400.0,
            avg_read_latency_ms: 5.0,
            avg_write_latency_ms: 8.0,
            max_read_latency_ms: 10.0,
            max_write_latency_ms: 15.0,
            test_mode: blueprint_pricing_engine_simple_lib::benchmark::io::IoTestMode::RndRw,
            block_size: 4096,
            total_file_size: 128 * 1024 * 1024, // 128 MB
            num_files: 2,
            duration_ms: 1000,
        }),
        gpu_details: Some(GpuBenchmarkResult {
            gpu_available: true,
            gpu_memory_mb: 4000.0,
            gpu_model: "Test GPU Model".to_string(),
            gpu_frequency_mhz: 1500.0,
        }),
    };

    // Step 3: Calculate a price based on the benchmark profile and pricing data
    // Default TTL in blocks (e.g., 1 hour with 6-second blocks = 600 blocks)
    let ttl_blocks = 600u64;
    
    let price_model = calculate_price(
        benchmark_profile.clone(),
        &pricing_data,
        Some(blueprint_id),
        ttl_blocks
    )?;

    // Debug: Print the calculated price model
    info!("\nCalculated Price Model:");
    info!("Total Cost Rate: ${:.6}", price_model.total_cost);
    for resource in &price_model.resources {
        info!(
            "  Resource: {} - Count: {} - Rate: ${:.6}",
            resource.kind, resource.count, resource.price_per_unit_rate
        );
    }

    // Step 4: Set up multiple pricing engines with different pricing strategies
    let mut servers = Vec::new();
    let mut clients = Vec::new();
    let mut cleanup_paths = Vec::new();

    // Create 3 different operators with different rate multipliers
    let rate_multipliers = vec![1.0, 1.2, 1.4];

    for (i, multiplier) in rate_multipliers.iter().enumerate() {
        let port = 9000 + i as u16;
        let addr = format!("127.0.0.1:{}", port);
        let socket_addr = match addr.parse() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to parse socket address: {}", e);
                continue;
            }
        };

        // Create a unique config for each operator
        let mut config = create_test_config();
        config.rpc_port = port;
        config.rpc_bind_address = addr.clone();
        config.database_path = format!("./data/test_benchmark_cache_{}", i);
        config.keystore_path = format!("/tmp/test-keystore-{}", i);

        // Initialize the benchmark cache
        let benchmark_cache = init_benchmark_cache(&Arc::new(config.clone())).await?;
        
        // Store the benchmark profile in the cache
        benchmark_cache.store_profile(blueprint_id, &benchmark_profile)?;

        // Initialize the pricing config
        let pricing_config = init_pricing_config(config_file_path.to_str().unwrap()).await?;
        
        // Apply the rate multiplier to each resource's price
        // This is test-only code to demonstrate different operator pricing
        {
            let mut pricing_map = pricing_config.lock().await;
            
            // Apply multiplier to default resources
            if let Some(resources) = pricing_map.get_mut(&None) {
                for resource in resources.iter_mut() {
                    resource.price_per_unit_rate *= multiplier;
                }
            }
            
            // Apply multiplier to blueprint-specific resources if they exist
            if let Some(resources) = pricing_map.get_mut(&Some(blueprint_id)) {
                for resource in resources.iter_mut() {
                    resource.price_per_unit_rate *= multiplier;
                }
            }
        }

        // Initialize the operator signer
        let operator_signer = init_operator_signer(&config, &config.keystore_path)?;

        // Create the pricing engine service
        let service = PricingEngineService::new(
            Arc::new(config.clone()),
            benchmark_cache,
            pricing_config,
            operator_signer,
        );

        // Start the gRPC server
        let server = tonic::transport::Server::builder()
            .add_service(pricing_engine::pricing_engine_server::PricingEngineServer::new(
                service,
            ))
            .serve(socket_addr);

        // Spawn the server in a background task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.await {
                error!("Server error: {}", e);
            }
        });

        servers.push(server_handle);

        // Connect to the server
        let client = loop {
            match tonic::transport::Endpoint::new(format!("http://{}", addr)) {
                Ok(endpoint) => {
                    match endpoint.connect().await {
                        Ok(channel) => {
                            break PricingEngineClient::new(channel);
                        }
                        Err(e) => {
                            warn!("Failed to connect to endpoint: {}", e);
                            sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to create endpoint: {}", e);
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
            }
        };

        clients.push(client);

        // Add paths to clean up later
        cleanup_paths.push((config.database_path, config.keystore_path));
    }

    // Step 5: Request quotes from all operators and compare
    let mut quote_responses = Vec::new();

    // Calculate expected cost for a 1-hour TTL
    let expected_total = price_model.total_cost;
    let expected_cost = expected_total * (ttl_blocks as f64);

    // Generate proof of work for the request
    let challenge = generate_challenge(blueprint_id, Utc::now().timestamp() as u64);
    let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

    for (i, client) in clients.iter_mut().enumerate() {
        // Create a request for the pricing engine
        let request = pricing_engine::GetPriceRequest {
            blueprint_id,
            ttl_blocks,
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
            info!("  TTL: {} blocks", details.ttl_blocks);
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
    // Calculate expected cost for a given TTL in blocks
    info!("\n=== Cost Calculation Verification ===");
    info!("Base price per block: ${:.6}", expected_total);
    info!(
        "Expected cost for {} blocks: ${:.6}",
        ttl_blocks, expected_cost
    );

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
        database_path: "./data/test_benchmark_cache".to_string(),
        rpc_port: 9000,
        rpc_bind_address: "127.0.0.1:9000".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["benchmark".to_string()],
        benchmark_duration: 10,
        benchmark_interval: 1,
        keypair_path: "/tmp/test-keypair".to_string(),
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300,
    }
}
