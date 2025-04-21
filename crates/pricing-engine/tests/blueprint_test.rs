// use std::sync::Arc;

// use blueprint_pricing_engine_lib::{
//     app::{init_operator_signer, init_price_cache},
//     config::OperatorConfig,
//     error::Result,
//     pow::{DEFAULT_POW_DIFFICULTY, generate_challenge, generate_proof},
//     pricing::{self, PriceModel},
//     pricing_engine,
//     pricing_engine::pricing_engine_server::PricingEngine,
//     service::rpc::server::PricingEngineService,
//     types::ResourceUnit,
// };
// use chrono::Utc;
// use tonic::Request;

// // Helper function to initialize test logging
// fn init_test_logging() {
//     let _ = tracing_subscriber::fmt()
//         .with_env_filter("debug")
//         .try_init();
// }

// // Create a test config
// fn create_test_config() -> OperatorConfig {
//     OperatorConfig {
//         database_path: "./data/test_price_cache".to_string(),
//         benchmark_command: "echo".to_string(),
//         benchmark_args: vec!["test".to_string()],
//         benchmark_duration: 1,
//         benchmark_interval: 1,
//         price_scaling_factor: 1000.0,
//         quote_validity_duration_secs: 300, // 5 minutes for testing
//         keypair_path: "/tmp/test-keypair".to_string(),
//         keystore_path: "/tmp/test-keystore".to_string(),
//         rpc_bind_address: "127.0.0.1:9000".to_string(),
//         rpc_port: 9000,
//         rpc_timeout: 5,
//         rpc_max_connections: 10,
//     }
// }

// // Helper function to create a test price model
// fn create_test_price_model() -> PriceModel {
//     PriceModel {
//         resources: vec![
//             pricing::ResourcePricing {
//                 kind: ResourceUnit::CPU,
//                 count: 2,
//                 price_per_unit_wei: 500000,
//             },
//             pricing::ResourcePricing {
//                 kind: ResourceUnit::MemoryMB,
//                 count: 1024,
//                 price_per_unit_wei: 1000,
//             },
//         ],
//         price_per_second_wei: 1000,
//         generated_at: Utc::now(),
//         benchmark_profile: None,
//     }
// }

// // Helper function to create a test blueprint ID
// fn create_test_blueprint_id() -> u64 {
//     // Create a deterministic test ID
//     12345
// }

// #[tokio::test]
// async fn test_pricing_engine_with_blueprint() -> Result<()> {
//     // Initialize logging
//     init_test_logging();

//     // Step 1: Set up the pricing engine
//     let config = Arc::new(create_test_config());

//     // Create necessary directories
//     let keystore_path = std::path::Path::new(&config.keystore_path);
//     if !keystore_path.exists() {
//         std::fs::create_dir_all(keystore_path)?;
//     }

//     let cache_path = std::path::Path::new(&config.database_path);
//     if !cache_path.exists() {
//         std::fs::create_dir_all(cache_path)?;
//     }

//     // Initialize price cache
//     let price_cache = init_price_cache(&config).await?;

//     // Initialize operator signer
//     let operator_signer = init_operator_signer(&config, &config.keystore_path)?;

//     // Store a price model for the blueprint
//     let blueprint_id = create_test_blueprint_id();
//     let price_model = create_test_price_model();
//     price_cache.store_price(blueprint_id, &price_model)?;

//     // Step 2: Create a pricing engine service
//     let pricing_service = PricingEngineService::new(
//         config.clone(),
//         price_cache.clone(),
//         operator_signer.clone(),
//     );

//     // Step 3: Simulate a user requesting a quote
//     let ttl_seconds = 3600; // 1 hour
//     let timestamp = Utc::now().timestamp() as u64;

//     // Generate a proof of work
//     let challenge = generate_challenge(blueprint_id, timestamp);
//     let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

//     // Create a request for the pricing engine with the correct fields
//     let request = Request::new(pricing_engine::GetPriceRequest {
//         blueprint_id,
//         ttl_seconds,
//         resource_requirements: vec![
//             pricing_engine::ResourceRequirement {
//                 kind: "CPU".to_string(),
//                 count: 2,
//             },
//             pricing_engine::ResourceRequirement {
//                 kind: "MemoryMB".to_string(),
//                 count: 1024,
//             },
//         ],
//         proof_of_work: proof.clone(),
//     });

//     // Get a price quote from the pricing engine
//     // Handle the tonic::Status error separately
//     let response = match pricing_service.get_price(request).await {
//         Ok(response) => response,
//         Err(status) => {
//             eprintln!("gRPC error: {}", status);
//             return Err(blueprint_pricing_engine_lib::error::PricingError::Other(
//                 format!("gRPC error: {}", status)
//             ));
//         }
//     };

//     let response_ref = response.get_ref();

//     // Verify the response fields that actually exist
//     assert!(!response_ref.operator_id.is_empty(), "Operator ID should not be empty");
//     assert!(!response_ref.signature.is_empty(), "Signature should not be empty");
//     assert!(!response_ref.proof_of_work.is_empty(), "Proof of work should not be empty");

//     // Verify the quote details if it exists
//     if let Some(details) = &response_ref.quote_details {
//         assert_eq!(details.blueprint_id, blueprint_id);
//         assert_eq!(details.ttl_seconds, ttl_seconds);
//         assert!(!details.total_cost_wei.is_empty(), "Total cost should not be empty");
//     } else {
//         panic!("Quote details should be present in the response");
//     }

//     // Clean up
//     let _ = std::fs::remove_dir_all(cache_path);
//     let _ = std::fs::remove_dir_all(keystore_path);

//     Ok(())
// }

// // Test with multiple operators
// #[tokio::test]
// async fn test_multi_operator_pricing() -> Result<()> {
//     // Initialize logging
//     init_test_logging();

//     // Set up multiple pricing engines with different price models
//     let mut pricing_services = Vec::new();
//     let mut cleanup_paths = Vec::new();
//     let blueprint_id = create_test_blueprint_id();

//     for i in 0..3 {
//         let mut config = create_test_config();
//         config.database_path = format!("./data/test_price_cache_{}", i);
//         config.keystore_path = format!("/tmp/test-keystore_{}", i);
//         config.rpc_port = 9000 + i;
//         config.rpc_bind_address = format!("127.0.0.1:{}", 9000 + i);

//         let config = Arc::new(config);

//         // Create necessary directories
//         let keystore_path = std::path::Path::new(&config.keystore_path);
//         if !keystore_path.exists() {
//             std::fs::create_dir_all(keystore_path)?;
//         }

//         let cache_path = std::path::Path::new(&config.database_path);
//         if !cache_path.exists() {
//             std::fs::create_dir_all(cache_path)?;
//         }

//         cleanup_paths.push((cache_path.to_path_buf(), keystore_path.to_path_buf()));

//         // Initialize price cache
//         let price_cache = init_price_cache(&config).await?;

//         // Initialize operator signer
//         let operator_signer = init_operator_signer(&config, &config.keystore_path)?;

//         // Create a price model with different pricing based on operator index
//         let mut price_model = create_test_price_model();
//         // Use u128 for price_per_second_wei
//         price_model.price_per_second_wei = 1000 * (i as u128 + 1); // Different prices for each operator
//         price_cache.store_price(blueprint_id, &price_model)?;

//         // Create a pricing engine service with the correct argument order
//         let pricing_service = PricingEngineService::new(
//             config.clone(),
//             price_cache.clone(),
//             operator_signer.clone(),
//         );

//         pricing_services.push(pricing_service);
//     }

//     // Simulate a user requesting quotes from all operators
//     let ttl_seconds = 3600; // 1 hour
//     let timestamp = Utc::now().timestamp() as u64;

//     // Generate a proof of work
//     let challenge = generate_challenge(blueprint_id, timestamp);
//     let proof = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

//     let mut quotes = Vec::new();

//     for pricing_service in &pricing_services {
//         // Create a request with the correct fields
//         let request = Request::new(pricing_engine::GetPriceRequest {
//             blueprint_id,
//             ttl_seconds,
//             resource_requirements: vec![
//                 pricing_engine::ResourceRequirement {
//                     kind: "CPU".to_string(),
//                     count: 2,
//                 },
//                 pricing_engine::ResourceRequirement {
//                     kind: "MemoryMB".to_string(),
//                     count: 1024,
//                 },
//             ],
//             proof_of_work: proof.clone(),
//         });

//         // Get a price quote from the pricing engine
//         // Handle the tonic::Status error separately
//         let response = match pricing_service.get_price(request).await {
//             Ok(response) => response,
//             Err(status) => {
//                 eprintln!("gRPC error: {}", status);
//                 return Err(blueprint_pricing_engine_lib::error::PricingError::Other(
//                     format!("gRPC error: {}", status)
//                 ));
//             }
//         };

//         let response_ref = response.get_ref();

//         // Store the quote details for comparison if it exists
//         if let Some(details) = &response_ref.quote_details {
//             quotes.push(details.clone());
//         }
//     }

//     // Verify that we have different quotes with different prices
//     assert_eq!(quotes.len(), 3, "Should have 3 quotes from different operators");

//     // Sort quotes by total cost (parse the string to a number)
//     quotes.sort_by(|a, b| {
//         let a_cost = a.total_cost_wei.parse::<u128>().unwrap_or(0);
//         let b_cost = b.total_cost_wei.parse::<u128>().unwrap_or(0);
//         a_cost.cmp(&b_cost)
//     });

//     // Verify that prices are different and in ascending order
//     for i in 1..quotes.len() {
//         let prev_cost = quotes[i-1].total_cost_wei.parse::<u128>().unwrap_or(0);
//         let curr_cost = quotes[i].total_cost_wei.parse::<u128>().unwrap_or(0);
//         assert!(
//             curr_cost > prev_cost,
//             "Quotes should have different prices"
//         );
//     }

//     // Clean up
//     for (cache_path, keystore_path) in cleanup_paths {
//         let _ = std::fs::remove_dir_all(cache_path);
//         let _ = std::fs::remove_dir_all(keystore_path);
//     }

//     Ok(())
// }
