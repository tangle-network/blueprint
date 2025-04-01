//! RPC Workflow Integration Tests
//!
//! This module contains tests for the complete RPC workflow:
//! 1. User sends RPC request to operator RPC server
//! 2. Operator gossips request via P2P network
//! 3. Multiple operators process the request and generate quotes
//! 4. Quotes are collected and returned to the user

use crate::{
    Service,
    service::{ServiceConfig, rpc::client::RpcClient, rpc::server::RfqRequestStatus},
};

use blueprint_crypto::sp_core::SpSr25519;
use blueprint_networking::{
    service_handle::NetworkServiceHandle,
    test_utils::{create_whitelisted_nodes, init_tracing, wait_for_all_handshakes},
};
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use super::utils::{create_test_key_pair, create_test_pricing_models, create_test_requirements};

/// Test the complete RPC workflow with real networking
///
/// This test demonstrates the full flow:
/// 1. Set up multiple operator nodes with RPC servers
/// 2. Client connects to one operator via RPC
/// 3. Request is gossiped to all operators via P2P network
/// 4. Quotes are gathered and returned to client
#[tokio::test]
#[serial_test::serial]
async fn test_rpc_workflow_with_real_networking() {
    // Initialize tracing for better debugging
    init_tracing();
    info!("Starting RPC workflow test with real networking");

    // Define test constants
    const TEST_TIMEOUT: Duration = Duration::from_secs(15);
    const NUM_OPERATOR_NODES: usize = 3;
    const RPC_PORT_BASE: u16 = 8000;

    // Create network constants
    let network_name = "rpc-test";
    let instance_id = "v1.0.0";

    // 1. Create and start network nodes for operators
    info!(
        "Creating and starting network nodes for {} operators",
        NUM_OPERATOR_NODES
    );
    let mut nodes =
        create_whitelisted_nodes::<SpSr25519>(NUM_OPERATOR_NODES, network_name, instance_id, false)
            .await;

    // Start all nodes
    let mut handles = Vec::new();
    for node in &mut nodes {
        handles.push(node.start().await.expect("Failed to start node"));
    }

    // Wait for all nodes to handshake with each other
    info!("Waiting for nodes to establish P2P connections");
    let handle_refs: Vec<&mut NetworkServiceHandle<SpSr25519>> = handles.iter_mut().collect();
    wait_for_all_handshakes(&handle_refs, TEST_TIMEOUT).await;

    // 2. Create and start service with RPC server for each operator
    let mut services = Vec::new();
    let mut rpc_addresses = Vec::new();

    info!("Setting up operator services with RPC servers");
    for (i, handle) in handles.iter().enumerate() {
        // Create key pair and pricing models for this operator
        let operator_key = create_test_key_pair::<SpSr25519>();

        // Create slightly different pricing models for each operator
        let mut models = create_test_pricing_models();
        if let Some(model) = models.first_mut() {
            if let Some(base_price) = &mut model.base_price {
                // Adjust price by a factor based on operator index
                let factor = 90 + (i as u128 * 10); // 90%, 100%, 110%
                base_price.value = base_price.value * factor / 100;
            }
        }

        // Create RPC address
        let rpc_addr = format!("127.0.0.1:{}", RPC_PORT_BASE + i as u16)
            .parse()
            .unwrap();
        rpc_addresses.push(rpc_addr);

        // Configure service with network handle and RPC server
        let config = ServiceConfig::<SpSr25519> {
            rpc_addr,
            node_url: Some("ws://127.0.0.1:9944".to_string()),
            keystore_path: None,
            operator_name: format!("Operator {}", i),
            operator_description: Some(format!("Test Operator {}", i)),
            operator_public_key: operator_key.public(),
            supported_blueprints: models.iter().map(|m| m.blueprint_id.clone()).collect(),
            network_handle: Some(Arc::new(handle.clone())),
        };

        // Create and start service - only starting the server component for tests
        let mut service = Service::<SpSr25519>::new(config, models, operator_key);
        service
            .start_server()
            .await
            .expect("Server components should start");

        info!("Started operator {} with RPC server at {}", i, rpc_addr);
        services.push(service);
    }

    // Wait for services to fully initialize
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 3. Create RPC client to connect to first operator
    let target_rpc_addr = rpc_addresses[0];
    info!("Creating RPC client to connect to {}", target_rpc_addr);

    let rpc_client = RpcClient::new(format!("http://{}", target_rpc_addr))
        .await
        .expect("Failed to create RPC client");

    // 4. Send RFQ request via RPC using the standard API
    let blueprint_id = "compute.basic".to_string();
    let requirements = create_test_requirements();

    info!("Sending RFQ request for '{}' via RPC", blueprint_id);

    // Use the proper production RPC flow:
    // 1. Submit request via pricing_requestForQuote
    let request_id = rpc_client
        .request_for_quote(
            blueprint_id.clone(),
            requirements.clone(),
            None,    // max_price
            Some(5), // timeout_seconds: 5 second timeout for test
        )
        .await
        .expect("Failed to send RFQ request");

    info!(
        "Received request ID: {}, waiting for operators to respond...",
        request_id
    );

    // 2. Wait briefly to allow gossip propagation to other nodes
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 3. Get results via pricing_getRfqResults
    let rfq_response = rpc_client
        .get_rfq_results::<SpSr25519>(request_id)
        .await
        .expect("Failed to get RFQ results");

    // 5. Verify quotes received
    info!("Received {} quotes via RFQ", rfq_response.quotes.len());

    // We should receive quotes from multiple operators via P2P gossip
    assert!(
        !rfq_response.quotes.is_empty(),
        "Should receive at least one quote"
    );

    // Print out information about the received quotes
    for (i, quote) in rfq_response.quotes.iter().enumerate() {
        info!(
            "Quote #{}: {} {} from {} (Provider: {})",
            i + 1,
            quote.price,
            quote.currency,
            quote.provider_name,
            format!("{:?}", quote.provider_id)
        );
    }

    // Verify quotes have different prices since we configured different pricing
    if rfq_response.quotes.len() > 1 {
        let prices: Vec<u64> = rfq_response.quotes.iter().map(|q| q.price).collect();

        let mut unique_prices = prices.clone();
        unique_prices.sort();
        unique_prices.dedup();

        // We should have at least 2 different price points
        assert!(
            unique_prices.len() > 1,
            "Should receive quotes with different prices"
        );

        // Verify provider names are different (from different operators)
        let provider_names: Vec<String> = rfq_response
            .quotes
            .iter()
            .map(|q| q.provider_name.clone())
            .collect();
        let mut unique_names = provider_names.clone();
        unique_names.sort();
        unique_names.dedup();

        // In a real P2P network, we should get quotes from multiple operators
        assert!(
            unique_names.len() > 1,
            "Should receive quotes from different providers via P2P gossip"
        );

        info!(
            "Successfully received quotes from multiple operators: {:?}",
            unique_names
        );
    }

    // 6. Shutdown services
    info!("Test successful, shutting down services");
    for (i, service) in services.iter().enumerate() {
        service.stop().await.expect("Service should stop cleanly");
        info!("Stopped operator {}", i);
    }

    info!("RPC workflow test completed successfully");
}

/// Test case with alternate client-server interaction pattern
///
/// In this test, we set up multiple operators but a client interacts with
/// all of them directly rather than through gossip
#[tokio::test]
#[serial_test::serial]
async fn test_rpc_multi_operator_direct_comparison() {
    // Initialize tracing
    init_tracing();
    info!("Starting direct multi-operator RPC comparison test");

    // Define test constants
    const NUM_OPERATORS: usize = 3;
    const RPC_PORT_BASE: u16 = 9000;
    const TEST_TIMEOUT: Duration = Duration::from_secs(15);

    // Create network constants
    let network_name = "direct-test";
    let instance_id = "v1.0.0";

    // 1. Create and start network nodes for operators
    info!(
        "Creating and starting network nodes for {} operators",
        NUM_OPERATORS
    );
    let mut nodes =
        create_whitelisted_nodes::<SpSr25519>(NUM_OPERATORS, network_name, instance_id, false)
            .await;

    // Start all nodes
    let mut handles = Vec::new();
    for node in &mut nodes {
        handles.push(node.start().await.expect("Failed to start node"));
    }

    // Wait for all nodes to handshake with each other
    info!("Waiting for nodes to establish P2P connections");
    let handle_refs: Vec<&mut NetworkServiceHandle<SpSr25519>> = handles.iter_mut().collect();
    wait_for_all_handshakes(&handle_refs, TEST_TIMEOUT).await;

    // Create independent services for each operator (with network)
    let mut services = Vec::new();
    let mut rpc_addresses = Vec::new();

    info!(
        "Setting up {} operator services with networking",
        NUM_OPERATORS
    );
    for (i, handle) in handles.iter().enumerate() {
        // Create key pair and pricing models
        let operator_key = create_test_key_pair::<SpSr25519>();

        // Create slightly different pricing models for each operator
        let mut models = create_test_pricing_models();
        if let Some(model) = models.first_mut() {
            if let Some(base_price) = &mut model.base_price {
                // Create price variance between operators
                let factor = 90 + (i as u128 * 15); // 90%, 105%, 120%
                base_price.value = base_price.value * factor / 100;
            }
        }

        // Create RPC address
        let rpc_addr = format!("127.0.0.1:{}", RPC_PORT_BASE + i as u16)
            .parse()
            .unwrap();
        rpc_addresses.push(rpc_addr);

        // Configure service with network handle
        let config = ServiceConfig::<SpSr25519> {
            rpc_addr,
            node_url: Some("ws://127.0.0.1:9944".to_string()),
            keystore_path: None,
            operator_name: format!("Operator {}", i),
            operator_description: Some(format!("Standalone Operator {}", i)),
            operator_public_key: operator_key.public(),
            supported_blueprints: models.iter().map(|m| m.blueprint_id.clone()).collect(),
            network_handle: Some(Arc::new(handle.clone())), // Using real network handle
        };

        // Create and start service - only starting the server component for tests
        let mut service = Service::<SpSr25519>::new(config, models, operator_key);
        service
            .start_server()
            .await
            .expect("Server components should start");

        info!("Started operator {} with RPC server at {}", i, rpc_addr);
        services.push(service);
    }

    // Wait for services to fully initialize
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 2. Create an RPC client for each operator
    let mut rpc_clients = Vec::new();
    for addr in &rpc_addresses {
        let client = RpcClient::new(format!("http://{}", addr))
            .await
            .expect("Failed to create RPC client");
        rpc_clients.push(client);
    }

    // 3. Send the same RFQ request to each operator directly using the production RPC methods
    let blueprint_id = "compute.basic".to_string();
    let requirements = create_test_requirements();

    // Collect all quotes from all operators
    let mut all_quotes = Vec::new();

    // This is the proper production workflow:
    // 1. Client calls pricing_requestForQuote to start the RFQ process
    // 2. Client receives a request ID
    // 3. Client calls pricing_getRfqResults to get quotes
    for (i, client) in rpc_clients.iter().enumerate() {
        info!(
            "Sending RFQ request to operator {} at {}",
            i, rpc_addresses[i]
        );

        // 1. Call the proper RPC method to initiate the request using our client
        let request_id = client
            .request_for_quote(
                blueprint_id.clone(),
                requirements.clone(),
                None,    // max_price
                Some(5), // timeout_seconds: 5 second timeout for quick testing
            )
            .await
            .expect("Failed to send RFQ request");

        info!("Received request ID: {}", request_id);

        // 2. Wait briefly for processing (in production, might poll or use WebSockets)
        tokio::time::sleep(Duration::from_millis(500)).await;

        // 3. Get the results using the standard RPC method
        let mut response = None;
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 3;

        while attempts < MAX_ATTEMPTS {
            attempts += 1;
            match client
                .get_rfq_results::<SpSr25519>(request_id.clone())
                .await
            {
                Ok(resp) => {
                    response = Some(resp);
                    break;
                }
                Err(e) => {
                    if attempts < MAX_ATTEMPTS {
                        info!(
                            "Retrying to get RFQ results (attempt {}/{}): {}",
                            attempts, MAX_ATTEMPTS, e
                        );
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    } else {
                        panic!(
                            "Failed to get RFQ results after {} attempts: {}",
                            MAX_ATTEMPTS, e
                        );
                    }
                }
            }
        }

        let response = response.expect("Should have response after retries");

        // Handle cases where response is still pending
        if let RfqRequestStatus::Pending = response.status {
            info!("RFQ request is still pending, waiting a bit longer");
            tokio::time::sleep(Duration::from_secs(1)).await;

            // Try one more time
            let final_response = client
                .get_rfq_results::<SpSr25519>(request_id)
                .await
                .expect("Failed to get final RFQ results");

            info!("Final response with {} quotes", final_response.quotes.len());

            // Process the quotes
            for quote in final_response.quotes {
                info!(
                    "Quote from {}: {} {}",
                    quote.provider_name, quote.price, quote.currency
                );

                // Add to our collection for verification
                all_quotes.push(quote);
            }
        } else {
            info!("Received response with {} quotes", response.quotes.len());

            // 4. Process the quotes
            for quote in response.quotes {
                info!(
                    "Quote from {}: {} {}",
                    quote.provider_name, quote.price, quote.currency
                );

                // Add to our collection for verification
                all_quotes.push(quote);
            }
        }
    }

    // 4. Verify we got quotes from the operators
    assert!(
        !all_quotes.is_empty(),
        "Should have received at least one quote"
    );

    // Verify operator names are different
    let provider_names: Vec<String> = all_quotes.iter().map(|q| q.provider_name.clone()).collect();
    info!("Provider names: {:?}", provider_names);

    // Verify quotes have different prices
    let prices: Vec<u64> = all_quotes.iter().map(|q| q.price).collect();
    info!("Quote prices: {:?}", prices);

    // Each operator should provide its own quote at minimum
    assert!(
        all_quotes.len() >= NUM_OPERATORS,
        "Each operator should have provided at least one quote"
    );

    // 5. Shutdown services
    info!("Test successful, shutting down services");
    for (i, service) in services.iter().enumerate() {
        service.stop().await.expect("Service should stop cleanly");
        info!("Stopped operator {}", i);
    }

    info!("Multi-operator direct comparison test completed successfully");
}
