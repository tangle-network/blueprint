use std::sync::Arc;
use std::time::Duration;
use std::{fs, path::PathBuf};

use blueprint_core::Job;
use blueprint_core::{error, info, warn};
use blueprint_pricing_engine_lib::{
    app::{init_benchmark_cache, init_operator_signer},
    benchmark::{
        BenchmarkProfile, CpuBenchmarkResult, GpuBenchmarkResult, IoBenchmarkResult,
        MemoryBenchmarkResult, NetworkBenchmarkResult, StorageBenchmarkResult,
    },
    error::Result,
    init_pricing_config,
    pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
    pricing::calculate_price,
    pricing_engine::{self, QuoteDetails, pricing_engine_client::PricingEngineClient},
    service::rpc::server::PricingEngineService,
    utils::u32_to_u128_bytes,
};
use blueprint_tangle_extra::layers::TangleLayer;
use blueprint_testing_utils::tangle::{InputValue, OutputValue};
use blueprint_testing_utils::{
    setup_log,
    tangle::{
        TangleTestHarness, blueprint::create_test_blueprint, harness::SetupServicesOpts,
        multi_node::NodeSlot,
    },
};
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use sp_core::hexdisplay::AsBytesRef;
use tangle_subxt::subxt::tx::Signer;
use tangle_subxt::subxt::utils::AccountId32;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_arithmetic::per_things::Percent;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::Asset;
use tangle_subxt::tangle_testnet_runtime::api::services::calls::types::{
    register::RegistrationArgs, request::RequestArgs,
};
use tempfile::tempdir;
use tokio::time::sleep;
use tonic::transport::Channel;

mod utils;

const OPERATOR_COUNT: usize = 4;
const REQUESTER_INDEX: usize = 0;
const RATE_MULTIPLIERS: [f64; 3] = [1.0, 1.2, 1.4];
const BASE_PORT: u16 = 9000;

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
        registration_args: vec![RegistrationArgs::default(); OPERATOR_COUNT]
            .try_into()
            .unwrap(),
        request_args: RequestArgs::default(),
    };
    let (mut test_env, _service_id, blueprint_id) = harness
        .setup_services_with_options::<OPERATOR_COUNT>(setup_services_opts)
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

    let pricing_data = init_pricing_config(config_file_path.to_str().unwrap()).await?;
    let pricing_data = pricing_data.lock().await;

    info!("Loaded pricing data:");
    for (key, resources) in &*pricing_data {
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
        None,
    )?;

    info!("\nCalculated Price Model:");
    info!("Total Cost Rate: ${:.6}", price_model.total_cost);
    for resource in &price_model.resources {
        info!(
            "  Resource: {} - Count: {} - Rate: ${:.6}",
            resource.kind, resource.count, resource.price_per_unit_rate
        );
    }

    let nodes = test_env.nodes.read().await.clone();

    let mut servers = Vec::new();
    let mut clients = Vec::new();
    let mut cleanup_paths = Vec::new();
    let mut operator_endpoints = Vec::new();
    let mut node_handles = Vec::new();

    for i in 0..OPERATOR_COUNT {
        // Skip the requester
        if i == REQUESTER_INDEX {
            continue;
        }

        let multiplier = RATE_MULTIPLIERS[i - 1];
        let port = BASE_PORT + i as u16;
        let addr = format!("127.0.0.1:{}", port);
        let socket_addr = match addr.parse() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Failed to parse socket address: {}", e);
                continue;
            }
        };
        operator_endpoints.push(format!("http://{}", addr));

        let node_handle = match nodes[i].clone() {
            NodeSlot::Occupied(node) => node,
            NodeSlot::Empty => panic!("Node {} is not initialized", i),
        };
        node_handles.push(node_handle.clone());
        let operator_env = node_handle.test_env.read().await.env.clone();
        let database_path = operator_env
            .data_dir
            .clone()
            .unwrap_or(PathBuf::from(format!("./data/test_benchmark_cache_{}", i)));

        let mut config = utils::create_test_config();
        config.rpc_port = port;
        config.rpc_bind_address = addr.clone();
        config.database_path = database_path.to_str().unwrap().to_string();
        config.keystore_path = PathBuf::from(&operator_env.keystore_uri);

        let benchmark_cache = init_benchmark_cache(&Arc::new(config.clone())).await?;

        benchmark_cache.store_profile(blueprint_id, &benchmark_profile)?;

        let pricing_config = init_pricing_config(config_file_path.to_str().unwrap()).await?;

        {
            let mut pricing_map = pricing_config.lock().await;

            if let Some(resources) = pricing_map.get_mut(&None) {
                for resource in resources.iter_mut() {
                    resource.price_per_unit_rate *= Decimal::from_f64(multiplier).unwrap();
                }
            }

            if let Some(resources) = pricing_map.get_mut(&Some(blueprint_id)) {
                for resource in resources.iter_mut() {
                    resource.price_per_unit_rate *= Decimal::from_f64(multiplier).unwrap();
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

    // Collect quotes from operators
    for (operator_index, operator_endpoint) in operator_endpoints.iter().enumerate() {
        // Skip the requester account
        if operator_index == REQUESTER_INDEX {
            info!(
                "Skipping operator {} as it will be the requester",
                operator_index
            );
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

        info!("Signature length: {}", signature.len());
        info!("Signature: {:?}", signature);
        info!("Operator ID length: {}", operator_id.len());
        info!("Operator ID: {:?}", operator_id);

        quote_responses.push((operator_index, quote_details, operator_id, signature));
    }

    if quote_responses.is_empty() {
        panic!("No quotes received from any operator");
    }
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

    let (operator_index, quote_details, operator_id, signature) = quote_responses
        .first()
        .expect("No quotes available to submit service request");

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

    let operator_id_bytes: [u8; 32] = operator_id.as_bytes_ref().try_into().unwrap();
    let operators = vec![AccountId32::from(operator_id_bytes)];
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

    let quotes = vec![
        blueprint_pricing_engine_lib::utils::create_on_chain_quote_type(quote_details).unwrap(),
    ];
    info!("Quotes: {:?}", quotes);

    let request_args = RequestArgs::default();
    info!("Request args: {:?}", request_args);

    info!(
        "Submitting service request with requester account: {:?}",
        harness.sr25519_signer.account_id()
    );
    info!("Using operator account: {:?}", operator_id);

    // We are just using the minimum as the default for testing
    let mapped_security_commitment =
            vec![tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityCommitment {
                asset: Asset::Custom(0),
                exposure_percent: Percent(50),
            }];

    let service_id = harness
        .request_service_with_quotes(
            blueprint_id,
            RequestArgs::default(),
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

    assert!(
        *operator_index < operators,
        "Selected operator index {} is out of bounds",
        operator_index
    );

    let operator_handle = node_handles[*operator_index].clone();
    operator_handle
        .add_job(utils::square.layer(TangleLayer))
        .await;
    operator_handle.start_runner(()).await.unwrap();

    // Submit job and
    let job = match harness
        .submit_job(
            service_id,
            utils::XSQUARE_JOB_ID,
            vec![InputValue::Uint64(5)],
        )
        .await
    {
        Ok(job) => {
            info!("Service request submitted successfully!");
            info!("Job ID: {:?}", job);
            assert_eq!(job.service_id, service_id);
            job
        }
        Err(e) => {
            panic!("Failed to submit job: {e}");
        }
    };

    // Wait for job execution and verify results
    let results = harness
        .wait_for_job_execution(service_id, job)
        .await
        .unwrap();
    harness.verify_job(&results, vec![OutputValue::Uint64(25)]);

    // Clean up
    for (cache_path, keystore_path) in cleanup_paths {
        let _ = std::fs::remove_dir_all(cache_path);
        let _ = std::fs::remove_dir_all(keystore_path);
    }

    Ok(())
}
