use std::fs;
use std::sync::Arc;
use std::time::Duration;

use blueprint_pricing_engine_lib::{
    app::{init_benchmark_cache, init_operator_signer},
    benchmark::{
        BenchmarkProfile, CpuBenchmarkResult, GpuBenchmarkResult, IoBenchmarkResult,
        MemoryBenchmarkResult, NetworkBenchmarkResult, StorageBenchmarkResult,
    },
    config::OperatorConfig,
    error::Result,
    init_pricing_config,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
    pricing::{calculate_price, load_pricing_from_toml},
    pricing_engine::{self, asset::AssetType, QuoteDetails, pricing_engine_client::PricingEngineClient},
    service::rpc::server::PricingEngineService,
    utils::{bytes_to_u128, u32_to_u128_bytes},
};
use blueprint_core::{error, info, warn};
use blueprint_tangle_extra::serde::{new_bounded_string, BoundedVec};
use blueprint_testing_utils::{
    setup_log,
    tangle::{TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts},
};
use chrono::Utc;
use sp_core::ecdsa;
use tangle_subxt::subxt::utils::{H160, AccountId32};
use tangle_subxt::subxt::tx::Signer;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_arithmetic::per_things::Percent;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::{Asset, AssetSecurityCommitment};
use tangle_subxt::{
    tangle_testnet_runtime::api::{
        runtime_types::tangle_primitives::services::pricing::{PricingQuote, ResourcePricing},
        services::calls::types::{register::RegistrationArgs, request::RequestArgs},
    },
};
use tempfile::tempdir;
use tokio::time::sleep;
use tonic::transport::Channel;

/// Test the full pricing and request flow with a blueprint on a local Tangle Testnet
#[tokio::test]
async fn test_full_pricing_flow_with_blueprint() -> Result<()> {
    setup_log();

    let (temp_dir, blueprint_dir) = create_test_blueprint();
    let harness: TangleTestHarness<()> = TangleTestHarness::setup(temp_dir).await.unwrap();

    std::env::set_current_dir(&blueprint_dir).unwrap();

    // Register all operators for service, but don't request service yet
    let setup_services_opts = SetupServicesOpts {
        exit_after_registration: false,
        skip_service_request: true,
        registration_args: vec![RegistrationArgs::default(); 4].try_into().unwrap(),
        request_args: RequestArgs::default(),
    };
    let (mut test_env, _service_id, blueprint_id) = harness
        .setup_services_with_options::<4>(setup_services_opts)
        .await
        .unwrap();
    test_env.initialize().await.unwrap();

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

    let config_temp_dir = tempdir()?;
    let config_file_path = config_temp_dir.path().join("pricing.toml");
    fs::write(&config_file_path, pricing_toml_content)?;

    info!("TOML configuration content:\n{}", pricing_toml_content);

    let pricing_data = load_pricing_from_toml(config_file_path.to_str().unwrap())?;

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
            access_mode: blueprint_pricing_engine_lib::benchmark::MemoryAccessMode::Sequential,
            operation_type: blueprint_pricing_engine_lib::benchmark::MemoryOperationType::Read,
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
            test_mode: blueprint_pricing_engine_lib::benchmark::io::IoTestMode::RndRw,
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

    let ttl_blocks = 600u64;

    let price_model = calculate_price(
        benchmark_profile.clone(),
        &pricing_data,
        Some(blueprint_id),
        ttl_blocks,
    )?;

    info!("\nCalculated Price Model:");
    info!("Total Cost Rate: ${:.6}", price_model.total_cost);
    for resource in &price_model.resources {
        info!(
            "  Resource: {} - Count: {} - Rate: ${:.6}",
            resource.kind, resource.count, resource.price_per_unit_rate
        );
    }

    let mut servers = Vec::new();
    let mut clients = Vec::new();
    let mut cleanup_paths = Vec::new();

    let rate_multipliers = [1.0, 1.2, 1.4];

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

        let mut config = create_test_config();
        config.rpc_port = port;
        config.rpc_bind_address = addr.clone();
        config.database_path = format!("./data/test_benchmark_cache_{}", i);
        config.keystore_path = format!("/tmp/test-keystore-{}", i);

        let benchmark_cache = init_benchmark_cache(&Arc::new(config.clone())).await?;

        benchmark_cache.store_profile(blueprint_id, &benchmark_profile)?;

        let pricing_config = init_pricing_config(config_file_path.to_str().unwrap()).await?;

        {
            let mut pricing_map = pricing_config.lock().await;

            if let Some(resources) = pricing_map.get_mut(&None) {
                for resource in resources.iter_mut() {
                    resource.price_per_unit_rate *= multiplier;
                }
            }

            if let Some(resources) = pricing_map.get_mut(&Some(blueprint_id)) {
                for resource in resources.iter_mut() {
                    resource.price_per_unit_rate *= multiplier;
                }
            }
        }

        let operator_signer = init_operator_signer(&config, &config.keystore_path)?;

        let service = PricingEngineService::new(
            Arc::new(config.clone()),
            benchmark_cache,
            pricing_config,
            operator_signer,
        );

        let server = tonic::transport::Server::builder()
            .add_service(pricing_engine::pricing_engine_server::PricingEngineServer::new(service))
            .serve(socket_addr);

        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.await {
                error!("Server error: {}", e);
            }
        });

        servers.push(server_handle);

        let client = loop {
            match tonic::transport::Endpoint::new(format!("http://{}", addr)) {
                Ok(endpoint) => match endpoint.connect().await {
                    Ok(channel) => {
                        break PricingEngineClient::new(channel);
                    }
                    Err(e) => {
                        warn!("Failed to connect to endpoint: {}", e);
                        sleep(Duration::from_millis(100)).await;
                        continue;
                    }
                },
                Err(e) => {
                    warn!("Failed to create endpoint: {}", e);
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
            }
        };

        clients.push(client);
        cleanup_paths.push((config.database_path, config.keystore_path));
    }

    let mut quote_responses = Vec::new();

    let expected_total = price_model.total_cost;
    let expected_cost = expected_total * (ttl_blocks as f64);

    let operator_endpoints = vec![
        "http://127.0.0.1:9000".to_string(),
        "http://127.0.0.1:9001".to_string(),
        "http://127.0.0.1:9002".to_string(),
    ];

    // Collect quotes from operators
    for (operator_index, operator_endpoint) in operator_endpoints.iter().enumerate() {
        // Skip operator 0 (Alice) as it will be the requester
        if operator_index == 0 {
            info!("Skipping operator 0 (Alice) as it will be the requester");
            continue;
        }

        info!("Requesting quote from operator {}", operator_index);
        let mut client = PricingEngineClient::new(
            Channel::from_shared(operator_endpoint.clone())
                .expect("Invalid URI")
                .connect_lazy(),
        );

        // Generate a fresh timestamp for each quote request
        let challenge_timestamp = Utc::now().timestamp() as u64;

        let request = tonic::Request::new(pricing_engine::GetPriceRequest {
            blueprint_id: blueprint_id as u64,
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
            proof_of_work: generate_proof(
                &generate_challenge(blueprint_id, challenge_timestamp),
                DEFAULT_POW_DIFFICULTY,
            )
            .await?,
            security_requirements: Some(pricing_engine::AssetSecurityRequirements {
                asset: Some(pricing_engine::Asset {
                    asset_type: Some(pricing_engine::asset::AssetType::Custom(u32_to_u128_bytes(
                        0,
                    ))),
                }),
                minimum_exposure_percent: 50,
                maximum_exposure_percent: 80,
            }),
            challenge_timestamp,
        });

        let response = client.get_price(request).await.unwrap();
        let quote_details = response.get_ref().quote_details.clone().unwrap();
        let signature = response.get_ref().signature.clone();
        let operator_id = response.get_ref().operator_id.clone();

        info!(
            "Received quote from operator {}: ${:.6} (ID: {:?})",
            operator_index, quote_details.total_cost_rate, operator_id
        );

        // Print detailed information about the signature for debugging
        info!("Signature length: {}", signature.len());
        info!("Signature: {:?}", signature);
        info!("Operator ID length: {}", operator_id.len());
        info!("Operator ID: {:?}", operator_id);

        quote_responses.push((operator_index, quote_details, operator_id, signature));
    }

    if quote_responses.is_empty() {
        info!("No quotes received from any operator");
    } else {
        info!("Received {} quotes:", quote_responses.len());
        for (i, details, operator_id, _signature) in &quote_responses {
            info!(
                "Operator {}: ${:.6} (ID: {:?})",
                i, details.total_cost_rate, operator_id
            );
        }

        // Sort quotes by total cost rate (cheapest first)
        quote_responses.sort_by(|a, b| {
            a.1.total_cost_rate
                .partial_cmp(&b.1.total_cost_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some((operator_index, quote_details, operator_id, signature)) =
            quote_responses.first()
        {
            info!(
                "Selected cheapest quote from Operator {}: ${:.6} (ID: {:?})",
                operator_index, quote_details.total_cost_rate, operator_id
            );

            let signature_bytes = if signature.len() == 65 {
                signature[..65].try_into().unwrap()
            } else if signature.len() == 64 {
                let mut sig_array = [0u8; 65];
                sig_array[0..64].copy_from_slice(&signature[..64]);
                sig_array[64] = 0;
                sig_array
            } else {
                panic!("Unexpected signature length: {}", signature.len());
            };

            let node_handles = test_env.node_handles().await;
            let signer = node_handles.first().unwrap().signer.clone();
            let signer = signer.into_inner();
            let account_id = signer.account_id();
            let operators = vec![account_id.clone()];
            let quote_signatures = vec![signature_bytes.into()];

            let QuoteDetails {
                blueprint_id,
                resources,
                security_commitments,
                ttl_blocks,
                total_cost_rate,
                timestamp,
                expiry,
            } = quote_details.clone();

            // 3. Hash the quote details (similar to what happens in signer.rs)
            // We can't directly use the same hashing function, but we can log the details
            info!("Quote details being hashed:");
            info!("  Blueprint ID: {}", blueprint_id);
            info!("  TTL Blocks: {}", ttl_blocks);
            info!("  Total Cost Rate: {}", total_cost_rate);
            info!("  Timestamp: {}", timestamp);
            info!("  Expiry: {}", expiry);
            info!("  Resources count: {}", resources.len());
            if let Some(sc) = &security_commitments {
                info!(
                    "  Security commitments count: {}",
                    sc.asset.as_ref().map_or(0, |_| 1)
                );
            }

            let security_commitment = security_commitments.clone().unwrap();
            info!("Quote details: {:?}", quote_details);

            let mapped_resources: Vec<tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::pricing::ResourcePricing> = resources
                .iter()
                .map(|resource| tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::pricing::ResourcePricing {
                    kind: new_bounded_string(resource.kind.clone()),
                    count: resource.count,
                    price_per_unit_rate: (resource.price_per_unit_rate * 1e6) as u64,
                })
                .collect();
            let resources = BoundedVec::<ResourcePricing>(mapped_resources.clone());
            info!("Mapped resources: {:?}", mapped_resources);

            let inner_asset_type = security_commitment.asset.unwrap().asset_type.unwrap();
            let asset = match inner_asset_type {
                AssetType::Custom(asset) => {
                    let asset_id = bytes_to_u128(&asset);
                    info!("Asset ID: {}", asset_id);
                    Asset::Custom(asset_id)
                }
                AssetType::Erc20(address) => {
                    let address_bytes: [u8; 20] = address
                        .as_slice()
                        .try_into()
                        .expect("ERC20 address should be 20 bytes");
                    info!("ERC20 address: {:?}", address_bytes);
                    Asset::Erc20(H160::from(address_bytes))
                }
            };
            let exposure_percent = Percent(security_commitment.exposure_percent as u8);
            info!("Exposure percent: {:?}", exposure_percent);
            let mapped_security_commitment =
                vec![tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityCommitment {
                    asset: asset.clone(),
                    exposure_percent,
                }];
            info!(
                "Mapped security commitment: {:?}",
                mapped_security_commitment
            );

            let security_commitments =
                BoundedVec::<AssetSecurityCommitment<u128>>(mapped_security_commitment.clone());

            let quotes = vec![PricingQuote {
                blueprint_id,
                ttl_blocks,
                resources: resources.clone(),
                security_commitments: security_commitments.clone(),
                total_cost_rate: total_cost_rate as u64,
                timestamp: timestamp,
                expiry: expiry,
            }];
            info!("Quotes: {:?}", quotes);

            let request_args = RequestArgs::default();
            info!("Request args: {:?}", request_args);

            info!(
                "Submitting service request with requester account: {:?}",
                harness.sr25519_signer.account_id()
            );
            info!("Using operator account: {:?}", account_id);

            // Ensure security commitments match what's expected by the Tangle runtime
            let mapped_security_commitment =
                vec![tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityCommitment {
                    asset: Asset::Custom(0), // Use a simple Custom(0) asset
                    exposure_percent: Percent(50), // Use a simple 50% exposure
                }];
            info!(
                "Using simplified security commitment: {:?}",
                mapped_security_commitment
            );

            // Try with a try/catch to get more detailed error information
            harness
                .request_service_with_quotes(
                    blueprint_id,
                    RequestArgs::default(), // Use default request args
                    operators,
                    quotes,
                    quote_signatures,
                    mapped_security_commitment,
                    None,
                )
                .await
                .unwrap();

            let operators = test_env.nodes.read().await.len();
            info!("Found {} operators in the test environment", operators);

            if *operator_index < operators {
                match harness.submit_job(blueprint_id, 0, vec![]).await {
                    Ok(job) => {
                        info!("Service request submitted successfully!");
                        info!("Job ID: {:?}", job);
                    }
                    Err(e) => {
                        panic!("Failed to submit job: {e}");
                    }
                }
            } else {
                panic!(
                    "Selected operator index {} is out of bounds",
                    operator_index
                );
            }
        } else {
            panic!("No quotes available to submit service request");
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
