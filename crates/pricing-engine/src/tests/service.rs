//! Tests for the pricing engine service
//!
//! These tests verify the service lifecycle management and high-level
//! functionality of the pricing service.

use std::thread;

use super::utils::{create_test_key_pair, create_test_pricing_models, create_test_service_config};
use crate::{Price, Service, service::ServiceState};
use blueprint_core::info;
use blueprint_crypto::sp_core::SpSr25519;
use blueprint_testing_utils::setup_log;

/// Test service lifecycle - start and stop
#[tokio::test]
async fn test_service_lifecycle() {
    setup_log();

    let signing_key = create_test_key_pair::<SpSr25519>();
    let models = create_test_pricing_models();
    // Create config using the signing key
    let config = create_test_service_config::<SpSr25519>(&signing_key.public(), None);
    let mut service = Service::<SpSr25519>::new(config, models.clone(), signing_key);

    // Start the service
    service.start().await.expect("Service should start");
    assert_eq!(service.state(), ServiceState::Running);

    // Get pricing models
    let pricing_models = service.get_pricing_models();
    assert_eq!(pricing_models.len(), models.len());

    // Update pricing model
    let mut updated_model = models[0].clone();
    updated_model.base_price = Some(Price {
        value: 100,
        token: "USD".to_string(),
    });
    service.update_pricing_model(updated_model);
    let pricing_models = service.get_pricing_models();
    assert_eq!(pricing_models.len(), models.len());

    // Stop the service
    service.stop().await.expect("Service should stop");
    assert_eq!(service.state(), ServiceState::ShutDown);
}
