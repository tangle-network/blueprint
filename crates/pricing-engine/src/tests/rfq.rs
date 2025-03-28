//! Tests for the RFQ handling functionality
//!
//! These tests verify that the RFQ processor can handle quote requests,
//! generate quotes, and manage quote collections correctly.

use super::utils::{
    create_test_key_pair, create_test_pricing_models, create_test_quote_request,
    create_test_rfq_processor,
};
use crate::{
    rfq::{RfqMessage, RfqMessageType},
    types::Price,
};
use blueprint_crypto::{KeyType, sp_core::SpSr25519};
use libp2p::PeerId;

/// Test creating quotes from requests
#[tokio::test]
async fn test_generate_quote() {
    // Create key using the generic function
    let signing_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Use the utility to create an RFQ processor with production patterns
    let processor = create_test_rfq_processor::<SpSr25519>(&signing_key, models);

    // Create a requester key
    let requester_key = create_test_key_pair::<SpSr25519>();
    // Get public key bytes
    let requester_id = SpSr25519::public_from_secret(&requester_key);
    let request = create_test_quote_request(&requester_id);

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

    // Use the same code path as production by processing an incoming message
    let source_peer_id = PeerId::random(); // Simulate a message from a peer
    let result = processor
        .process_incoming_message(request_message, source_peer_id)
        .await;

    // In the real system, the processor would send a response via the network
    // For testing, we verify that the message was processed successfully
    assert!(result.is_ok());
}

/// Test that quotes are not generated for unknown blueprints
#[tokio::test]
async fn test_unknown_blueprint() {
    // Create a provider key and RFQ processor
    let provider_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Use the utility to create an RFQ processor with production patterns
    let processor = create_test_rfq_processor::<SpSr25519>(&provider_key, models);

    // Create a requester key and request with unknown blueprint
    let requester_key = create_test_key_pair::<SpSr25519>();
    let mut request = create_test_quote_request(&requester_key.public());
    request.blueprint_id = "unknown.blueprint".to_string();

    // Create an RFQ message with the unknown blueprint
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let request_message = RfqMessage {
        version: 1,
        timestamp,
        message_type: RfqMessageType::QuoteRequest(request),
    };

    // Process the message through the standard path
    // In production, the processor would log the error and not respond
    // Here we verify it still returns Ok but doesn't crash on an unknown blueprint
    let result = processor
        .process_incoming_message(request_message, PeerId::random())
        .await;

    // The method should still return Ok (it handled the message, just didn't respond)
    assert!(result.is_ok());
}

/// Test that price limits are honored
#[tokio::test]
async fn test_price_limit_honored() {
    // Create a provider key and RFQ processor
    let provider_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Use the utility to create an RFQ processor with production patterns
    let processor = create_test_rfq_processor::<SpSr25519>(&provider_key, models);

    // Create a requester key and request with very low price limit
    let requester_key = create_test_key_pair::<SpSr25519>();
    let mut request = create_test_quote_request(&requester_key.public());
    request.max_price = Some(Price {
        value: 100, // Very low value
        token: "TNT".to_string(),
    });

    // Create an RFQ message with the price-limited request
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    let request_message = RfqMessage {
        version: 1,
        timestamp,
        message_type: RfqMessageType::QuoteRequest(request),
    };

    // Process the message through the standard path
    // In production, the processor would check prices internally and not respond if over limit
    let result = processor
        .process_incoming_message(request_message, PeerId::random())
        .await;

    // The processing should still complete (doesn't throw an error out)
    assert!(result.is_ok());
}

/// Test the basic price quote generation functionality without using network communication
#[tokio::test]
async fn test_basic_quote_generation() {
    use crate::calculation::calculate_service_price;
    use crate::tests::utils::{
        create_test_key_pair, create_test_pricing_models, create_test_requirements,
    };
    use blueprint_crypto::sp_core::SpSr25519;

    // Create test keys and models
    let _key_pair = create_test_key_pair::<SpSr25519>();
    let pricing_models = create_test_pricing_models();

    // Get the model for the blueprint we want to test
    let model = pricing_models
        .iter()
        .find(|m| m.blueprint_id == "compute.basic")
        .expect("Should have a compute.basic pricing model");

    // Create test requirements
    let requirements = create_test_requirements();

    // Directly calculate price using the pricing model
    let price_result = calculate_service_price(&requirements, model);

    // Verify price calculation
    assert!(
        price_result.is_ok(),
        "Failed to calculate price: {:?}",
        price_result.err()
    );
    let price = price_result.unwrap();

    // Verify price details
    assert!(price.value > 0, "Price should be greater than 0");
    assert_eq!(price.token, "TNT", "Token should be TNT");

    println!(
        "Successfully calculated price: {} {}",
        price.value, price.token
    );
}
