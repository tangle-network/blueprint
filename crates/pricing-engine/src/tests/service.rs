//! Tests for the pricing engine service
//!
//! These tests verify the service lifecycle management and high-level
//! functionality of the pricing service.

use super::utils::{create_test_key_pair, create_test_pricing_models, create_test_service_config};
use crate::{Service, service::ServiceState};
use blueprint_crypto::sp_core::SpSr25519;

/// Test service initialization
#[tokio::test]
async fn test_service_init() {
    let signing_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create config using the signing key
    let config = create_test_service_config::<SpSr25519>(&signing_key.public(), None);

    // No need for manual key conversion anymore
    let service = Service::<SpSr25519>::new(config, models.clone(), signing_key);

    assert_eq!(service.state(), ServiceState::Initializing);
    assert_eq!(service.get_pricing_models().len(), models.len());
}

/// Test service lifecycle - start and stop
#[tokio::test]
async fn test_service_lifecycle() {
    let signing_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create config using the signing key
    let config = create_test_service_config::<SpSr25519>(&signing_key.public(), None);

    let mut service = Service::<SpSr25519>::new(config, models.clone(), signing_key);

    // Start the service
    service.start().await.expect("Service should start");
    assert_eq!(service.state(), ServiceState::Running);

    // Stop the service
    service.stop().await.expect("Service should stop");
    assert_eq!(service.state(), ServiceState::ShutDown);
}

/// Test service with RFQ functionality
#[tokio::test]
#[ignore = "Requires network setup"]
async fn test_service_with_rfq() {
    let signing_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create service config
    let config = create_test_service_config::<SpSr25519>(&signing_key.public(), None);

    // Create service
    let mut service = Service::<SpSr25519>::new(config, models.clone(), signing_key);

    // Start service
    service.start().await.expect("Service should start");

    // Check that the RFQ processor is not initialized (since we didn't provide a network handle)
    assert!(service.rfq_processor().is_none());

    // Stop service
    service.stop().await.expect("Service should stop");
}

#[tokio::test]
async fn test_service_rpc_server() {
    let signing_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();

    // Create config using the signing key
    let config = create_test_service_config::<SpSr25519>(&signing_key.public(), None);

    let mut service = Service::<SpSr25519>::new(config, models.clone(), signing_key);

    // Start the service
    service.start().await.expect("Service should start");

    // RPC server should be running on some address
    assert!(service.state() == ServiceState::Running);

    // Stop the service
    service.stop().await.expect("Service should stop");
}
