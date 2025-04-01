//! Test utilities for the pricing engine
//!
//! This module contains helper functions and types for testing the pricing engine.

use crate::{
    models::{PricingModel, create_fixed_price_model},
    rfq::{QuoteRequest, QuoteRequestId, RfqProcessor, RfqProcessorConfig},
    service::ServiceConfig,
    types::{Price, ResourceRequirement, ResourceUnit, TimePeriod},
};
use blueprint_crypto::KeyType;
use blueprint_networking::service_handle::NetworkServiceHandle;
use libp2p::PeerId;
use std::sync::Arc;
use time::OffsetDateTime;

/// Creates a test key pair for use in tests directly using the KeyType trait
///
/// This leverages the KeyType trait's own methods to ensure compatibility with production code
pub fn create_test_key_pair<K: KeyType>() -> K::Secret {
    // Use the KeyType's own generate_with_seed method
    K::generate_with_seed(None).expect("Failed to generate key pair")
}

/// Creates example pricing models using the actual factory methods
pub fn create_test_pricing_models() -> Vec<PricingModel> {
    let token = "TNT".to_string();

    // Use the actual factory methods from models.rs
    vec![
        // Create a fixed price model using the factory method
        create_fixed_price_model(
            "Basic Compute",
            "compute.basic",
            Price {
                value: 1_000_000_000_000_000_000, // 1 TNT
                token: token.clone(),
            },
            TimePeriod::Hour,
        )
        .with_description("Test compute resources"),
        // Create another model using the factory method
        create_fixed_price_model(
            "Basic Storage",
            "storage.basic",
            Price {
                value: 2_000_000_000_000_000_000, // 2 TNT
                token: token.clone(),
            },
            TimePeriod::Hour,
        )
        .with_description("Test storage solution"),
    ]
}

/// Creates a set of test resource requirements
pub fn create_test_requirements() -> Vec<ResourceRequirement> {
    vec![
        ResourceRequirement {
            unit: ResourceUnit::CPU,
            quantity: 2,
        },
        ResourceRequirement {
            unit: ResourceUnit::MemoryMB,
            quantity: 4096,
        },
        ResourceRequirement {
            unit: ResourceUnit::StorageMB,
            quantity: 100,
        },
    ]
}

/// Creates a test quote request
pub fn create_test_quote_request<K: KeyType>(requester_public_key: &K::Public) -> QuoteRequest<K> {
    let now = OffsetDateTime::now_utc();
    let request_id = QuoteRequestId::new();
    // Convert time to u64 timestamp in milliseconds
    let now_ms = now.unix_timestamp() as u64 * 1000;
    let expires_ms = (now + time::Duration::hours(1)).unix_timestamp() as u64 * 1000;

    QuoteRequest::<K> {
        id: request_id,
        requester_id: requester_public_key.clone(),
        blueprint_id: "compute.basic".to_string(),
        requirements: create_test_requirements(),
        max_price: Some(Price {
            value: 5_000_000_000_000_000_000, // 5 TNT
            token: "TNT".to_string(),
        }),
        created_at: now_ms,
        expires_at: expires_ms,
    }
}

/// Create a test service configuration
///
/// # Arguments
/// * `operator_public_key` - The operator's public key
/// * `network_handle` - Optional network handle for P2P communication
///
/// # Returns
/// A test service configuration
pub fn create_test_service_config<K: KeyType>(
    operator_public_key: &K::Public,
    network_handle: Option<Arc<NetworkServiceHandle<K>>>,
) -> ServiceConfig<K> {
    ServiceConfig {
        rpc_addr: "127.0.0.1:9000".parse().unwrap(),
        node_url: None,
        keystore_path: None,
        operator_name: "Test Operator".to_string(),
        operator_description: Some("For testing purposes".to_string()),
        operator_public_key: operator_public_key.clone(),
        supported_blueprints: vec!["compute.basic".to_string()],
        network_handle,
    }
}

/// Create an RFQ processor that matches the production initialization path
pub fn create_test_rfq_processor<K: KeyType>(
    signing_key: &K::Secret,
    pricing_models: Vec<PricingModel>,
) -> RfqProcessor<K> {
    use blueprint_networking::discovery::peers::WhitelistedKeys;
    use blueprint_networking::test_utils::{TestNode, init_tracing};
    use tokio::runtime::Runtime;

    // Initialize tracing for tests if not already done
    let _ = init_tracing();

    // Get the public key using the KeyType trait methods
    let public_key = K::public_from_secret(signing_key);

    // Create configuration that matches production patterns
    let config = RfqProcessorConfig {
        local_peer_id: PeerId::random(), // This will be replaced by the real node's peer ID
        operator_name: "Test Operator".to_string(),
        pricing_models,
        provider_public_key: Some(public_key),
        ..Default::default()
    };

    // Create a single-threaded runtime for starting the test node
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
    let mut config = config;
    config.local_peer_id = network_handle.local_peer_id;

    // Create the RFQ processor with a real network handle
    RfqProcessor::new(config, signing_key.clone(), network_handle)
}
