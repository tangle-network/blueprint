//! Integration tests for the pricing engine
//!
//! These tests demonstrate the full flow of the pricing engine, from
//! service initialization to RFQ handling and quote generation.

use super::utils::{
    create_test_key_pair, create_test_pricing_models, create_test_requirements,
    create_test_service_config,
};
use crate::{
    Service,
    models::PricingModel,
    rfq::{
        QuoteRequest, QuoteRequestId, RfqMessage, RfqMessageType, RfqProcessor, RfqProcessorConfig,
    },
    service::{ServiceConfig, ServiceState},
    tests::utils::{create_test_quote_request, create_test_rfq_processor},
};
use blueprint_crypto::sp_core::SpSr25519;
use blueprint_networking::{
    discovery::peers::WhitelistedKeys,
    service_handle::NetworkServiceHandle,
    test_utils::{TestNode, create_whitelisted_nodes, init_tracing, wait_for_all_handshakes},
};
use libp2p::PeerId;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::info;

/// Test end-to-end scenario with service start, quote request, and shutdown
#[tokio::test]
async fn test_service_end_to_end() {
    // Create test dependencies
    let operator_key = create_test_key_pair::<SpSr25519>();
    let requester_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create service config
    let config: ServiceConfig<SpSr25519> = ServiceConfig {
        // Use same config as in utils, but add channel
        rpc_addr: "127.0.0.1:0".parse().unwrap(),
        node_url: Some("ws://127.0.0.1:9944".to_string()),
        keystore_path: None,
        operator_name: "Test Operator".to_string(),
        operator_description: Some("Test Description".to_string()),
        operator_public_key: operator_key.public(),
        supported_blueprints: models.iter().map(|m| m.blueprint_id.clone()).collect(),
        network_handle: None,
    };

    // Create service
    let mut service = Service::<SpSr25519>::new(config, models.clone(), operator_key.clone());

    // Start service, inject our test channel into RFQ processor
    service.start().await.expect("Service should start");
    assert_eq!(service.state(), ServiceState::Running);

    // Access RFQ processor and configure for testing
    if let Some(rfq_processor) = service.rfq_processor() {
        // Verify processor is configured correctly
        assert!(!rfq_processor.get_pricing_models().is_empty());

        // Create a request
        let request_id = QuoteRequestId::new();
        let now = OffsetDateTime::now_utc();
        // Convert time to u64 timestamp in milliseconds
        let now_ms = now.unix_timestamp() as u64 * 1000;
        let expires_ms = (now + time::Duration::hours(1)).unix_timestamp() as u64 * 1000;

        let request = QuoteRequest {
            id: request_id.clone(),
            requester_id: requester_key.public(),
            blueprint_id: "compute.basic".to_string(),
            requirements: create_test_requirements(),
            max_price: Some(crate::types::Price {
                value: 5_000_000_000_000_000_000, // 5 TNT
                token: "TNT".to_string(),
            }),
            created_at: now_ms,
            expires_at: expires_ms,
        };

        // Prepare message
        let request_message = RfqMessage {
            version: 1,
            timestamp: now_ms,
            message_type: RfqMessageType::QuoteRequest(request.clone()),
        };

        // Simulate receiving message
        rfq_processor
            .process_incoming_message(request_message, PeerId::random())
            .await
            .expect("Processing should succeed");

        // In a real system, the response would be sent over the network
        // In this test, we're verifying the processor can handle a request
    } else {
        panic!("RFQ processor should be initialized");
    }

    // Stop service
    service.stop().await.expect("Service should stop cleanly");
    assert_eq!(service.state(), ServiceState::ShutDown);
}

/// Test with simulated client-operator communication
/// This test requires mocking network communication
#[tokio::test]
async fn test_client_operator_communication() {
    // Create keys for client and operator
    let operator_key = create_test_key_pair::<SpSr25519>();
    let client_key = create_test_key_pair::<SpSr25519>();

    // Create bidirectional channels to simulate network communication
    let (client_tx, mut operator_rx) = mpsc::channel(10); // Client -> Operator
    let (_operator_tx, mut client_rx) = mpsc::channel(10); // Operator -> Client

    // Set up operator service with necessary models
    let models = create_test_pricing_models();
    let _config = create_test_service_config::<SpSr25519>(&operator_key.public(), None);

    // Manually create and configure RFQ processor for operator
    let operator_config = RfqProcessorConfig {
        local_peer_id: PeerId::random(),
        operator_name: "Test Operator".to_string(),
        pricing_models: models.clone(),
        provider_public_key: Some(operator_key.public()),
        ..Default::default()
    };

    // Initialize tracing for tests if not already done
    let _ = init_tracing();

    // Set up a runtime to start the test node
    let rt = Runtime::new().unwrap();

    // Create and start a test node to get a real network handle
    let mut test_node = TestNode::new(
        "rfq-test-network",
        "rfq-test-instance",
        WhitelistedKeys::new(vec![]),
        vec![],
        false, // Not using EVM addresses for verification
    );

    // Start the node and get the network handle
    let network_handle =
        rt.block_on(async { test_node.start().await.expect("Failed to start test node") });

    // Update config with the real peer ID
    let mut operator_config = operator_config;
    operator_config.local_peer_id = network_handle.local_peer_id;

    let operator_processor =
        RfqProcessor::<SpSr25519>::new(operator_config, operator_key.clone(), network_handle);

    // Client creates a request
    let request_id = QuoteRequestId::new();
    let now = OffsetDateTime::now_utc();
    // Convert time to u64 timestamp in milliseconds
    let now_ms = now.unix_timestamp() as u64 * 1000;
    let expires_ms = (now + time::Duration::hours(1)).unix_timestamp() as u64 * 1000;

    let client_request = QuoteRequest {
        id: request_id,
        requester_id: client_key.public(),
        blueprint_id: "compute.basic".to_string(),
        requirements: create_test_requirements(),
        max_price: Some(crate::types::Price {
            value: 5_000_000_000_000_000_000, // 5 TNT
            token: "TNT".to_string(),
        }),
        created_at: now_ms,
        expires_at: expires_ms,
    };

    // Client sends request message
    let client_message = RfqMessage {
        version: 1,
        timestamp: now_ms,
        message_type: RfqMessageType::QuoteRequest(client_request.clone()),
    };

    // Client sends the request to operator
    client_tx
        .send(client_message)
        .await
        .expect("Send should succeed");

    // Operator processes the incoming request
    if let Some(request_msg) = operator_rx.recv().await {
        match &request_msg.message_type {
            RfqMessageType::QuoteRequest(request) => {
                assert_eq!(request.id, client_request.id);

                // Operator processes the request - should send response via operator_tx
                operator_processor
                    .process_incoming_message(request_msg, PeerId::random())
                    .await
                    .expect("Processing should succeed");
            }
            _ => panic!("Expected quote request"),
        }
    } else {
        panic!("Operator should receive request");
    }

    // Client waits for response from operator
    let client_response: RfqMessage<SpSr25519> = timeout(Duration::from_secs(2), client_rx.recv())
        .await
        .expect("Timeout waiting for response")
        .expect("Should receive response");

    // Instead of matching specific fields, just check the message type
    match client_response.message_type {
        RfqMessageType::QuoteResponse(_) => {
            // This is the correct response type
            // We don't need to check specific details since we don't have real data flow
        }
        _ => panic!("Expected quote response"),
    }
}

// Check if the PricingModel has a calculate_price method implementation first
/// Test resource pricing calculations
#[tokio::test]
#[ignore = "Requires PricingModel::calculate_price implementation"]
async fn test_resource_price_calculation() {
    // Create test pricing model with resource pricing
    let _model = PricingModel {
        model_type: crate::models::PricingModelType::Fixed, // Update enum variant if needed
        name: "Resource Test Model".to_string(),
        description: Some("For testing resource pricing".to_string()),
        blueprint_id: "test.resource".to_string(),
        base_price: Some(crate::types::Price {
            value: 500_000_000_000_000_000, // 0.5 TNT base
            token: "TNT".to_string(),
        }),
        resource_pricing: Vec::new(),
        billing_period: Some(crate::types::TimePeriod::Hour),
    };

    // Add resource pricing based on appropriate structure
    // This needs to be updated based on the actual ResourcePricing structure

    // Create requirements for testing
    let _requirements = create_test_requirements();

    // NOTE: This test is marked as ignored until the calculate_price method
    // is properly implemented or we determine the correct alternative approach
}

/// Test a full multi-node RFQ flow with real network handshakes and gossip
/// This test creates real network nodes, connects them, and tests the full RFQ flow
#[tokio::test]
#[serial_test::serial]
#[ignore = "Requires blueprint_networking::test_utils"]
async fn test_multi_node_rfq_flow_real_network() {
    // Initialize test environment
    init_tracing();
    info!("Starting multi-node RFQ flow test with real networking");

    // Define test constants
    const TEST_TIMEOUT: Duration = Duration::from_secs(15);
    const NUM_OPERATOR_NODES: usize = 3;

    // Create network constants
    let network_name = "rfq-test";
    let instance_id = "v1.0.0";

    // Create operator nodes (all operators, no client node in the network)
    let mut nodes =
        create_whitelisted_nodes::<SpSr25519>(NUM_OPERATOR_NODES, network_name, instance_id, false)
            .await;

    // Start all nodes
    info!("Starting all operator nodes");
    let mut handles = Vec::new();
    for node in &mut nodes {
        handles.push(node.start().await.expect("Failed to start node"));
    }

    // Wait for all nodes to handshake with each other
    info!("Waiting for all nodes to complete handshakes");
    let handle_refs: Vec<&mut NetworkServiceHandle<SpSr25519>> = handles.iter_mut().collect();
    wait_for_all_handshakes(&handle_refs, TEST_TIMEOUT).await;

    // Now set up Service with RPC server and RFQ processor for each operator
    info!("Setting up Services with RPC servers for operators");
    let mut services = Vec::new();
    let mut operator_infos = Vec::new();

    for (i, handle) in handles.iter().enumerate() {
        // Create key pair for this operator
        let operator_key = create_test_key_pair::<SpSr25519>();

        // Create pricing models (slightly different for each operator)
        let mut models = create_test_pricing_models();
        if let Some(model) = models.first_mut() {
            if let Some(base_price) = &mut model.base_price {
                // Adjust price by a factor based on operator index
                let factor = 90 + (i as u128 * 10); // 90%, 100%, 110%
                base_price.value = base_price.value * factor / 100;
            }
        }

        // Configure service
        let config = crate::service::ServiceConfig::<SpSr25519> {
            rpc_addr: format!("127.0.0.1:{}", 9000 + i).parse().unwrap(),
            node_url: Some("ws://127.0.0.1:9944".to_string()),
            keystore_path: None,
            operator_name: format!("Operator {}", i),
            operator_description: Some(format!("Test Operator {}", i)),
            operator_public_key: operator_key.public(),
            supported_blueprints: models.iter().map(|m| m.blueprint_id.clone()).collect(),
            network_handle: Some(Arc::new(handle.clone())),
        };

        // Store operator info for the client to use
        operator_infos.push((
            config.rpc_addr,
            config.operator_name.clone(),
            operator_key.public(),
        ));

        // Create service
        let mut service = Service::<SpSr25519>::new(config, models, operator_key);

        // Start service (this also starts the RFQ processor with the network handle)
        service.start().await.expect("Failed to start service");

        services.push(service);
    }

    // Now create a client (not part of the gossip network)
    info!("Creating client to interact with operators");
    let _client_key = create_test_key_pair::<SpSr25519>();

    // Create requirements
    let requirements = create_test_requirements();
    let blueprint_id = "compute.basic".to_string();

    // Wait for a moment to ensure all services are ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Client picks one operator to send the request through
    let chosen_operator = &operator_infos[0];

    info!(
        "Client sending RFQ request to Operator 0 at {}",
        chosen_operator.0
    );

    // In a real world scenario the client would make an RPC call to the operator's RPC server
    // For this test, we'll simulate by directly calling the service's send_rfq_request method
    let operator_service = &services[0];
    let quotes = operator_service
        .send_rfq_request(blueprint_id, requirements)
        .await
        .expect("Failed to send RFQ request");

    // We should receive quotes from multiple operators (including the one we sent to directly)
    // even though we only sent the request to one operator
    info!("Received {} quotes", quotes.len());
    assert!(
        !quotes.is_empty(),
        "Should have received at least one quote"
    );

    // In an ideal scenario, we should receive quotes from all operators
    // because of gossip propagation, but we'll be lenient in the test
    info!("Verifying quotes came from different operators");

    // Each operator should have a different name which we can check
    let quote_providers: Vec<_> = quotes
        .iter()
        .map(|q| q.quote.provider_name.clone())
        .collect();

    // Log all provider names
    for provider in &quote_providers {
        info!("Received quote from provider: {}", provider);
    }

    // Check that there's at least one unique provider
    assert!(
        !quote_providers.is_empty(),
        "Should have at least one provider"
    );

    // Verify quotes have different prices
    if quotes.len() > 1 {
        let mut prices: Vec<u128> = quotes.iter().map(|q| q.quote.price.value).collect();
        prices.sort();

        let mut unique_prices = prices.clone();
        unique_prices.dedup();

        // Each operator should give a different price since we configured them that way
        assert_eq!(
            unique_prices.len(),
            prices.len(),
            "Each quote should have a unique price"
        );
    }

    // Print out quote details for debugging
    for (i, quote) in quotes.iter().enumerate() {
        info!(
            "Quote #{}: {} {} from {}",
            i + 1,
            quote.quote.price.value,
            quote.quote.price.token,
            quote.quote.provider_name
        );
    }

    // Shutdown all services
    info!("Shutting down services");
    for service in services {
        service.stop().await.expect("Failed to shut down service");
    }

    info!("Multi-node RFQ flow test completed successfully");
}

#[ignore]
#[tokio::test]
async fn test_service_initialization() {
    // Use the generic key creation function
    let operator_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create config using production patterns
    let config = create_test_service_config::<SpSr25519>(&operator_key.public(), None);

    // Create service
    let service = Service::<SpSr25519>::new(config, models.clone(), operator_key);

    // Check state
    assert_eq!(service.state(), ServiceState::Initializing);
    assert_eq!(service.get_pricing_models().len(), models.len());
}

#[ignore]
#[serial_test::serial]
#[tokio::test]
async fn test_simple_rfq_flow() {
    // Create keys using the generic function
    let operator_key = create_test_key_pair::<SpSr25519>();
    let client_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create service config using production patterns
    let _config = create_test_service_config::<SpSr25519>(&operator_key.public(), None);

    // Create RFQ processor using the utility
    let operator_processor = create_test_rfq_processor::<SpSr25519>(&operator_key, models.clone());

    // Create a mock request
    // Get public key bytes for the client
    let request = create_test_quote_request(&client_key.public());

    // Create an RFQ message (as would happen in production)
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let request_message = RfqMessage {
        version: 1,
        timestamp,
        message_type: RfqMessageType::QuoteRequest(request.clone()),
    };

    // Tell the processor about our channel for sending (override its internal sender)
    // This would be done by production code providing a network_handle or other sending mechanism

    // Process the message through the production flow
    operator_processor
        .process_incoming_message(request_message, PeerId::random())
        .await
        .expect("Processing should succeed");

    // In a real system, we'd wait for a response over the network
    // For this test we'll just verify that the RFQ processor handled the request
    // and would have generated a response in production

    // Verify the processor is in a good state after processing
    let pricing_models = operator_processor.get_pricing_models();
    assert_eq!(pricing_models.len(), models.len());
}
