/// Integration tests for the `ProtocolManager` abstraction
///
/// Tests verify that both Tangle and EigenLayer protocols work correctly
/// through the unified `ProtocolManager` interface.
mod common;

use blueprint_manager::blueprint::ActiveBlueprints;
use blueprint_manager::config::BlueprintManagerContext;
use blueprint_manager::protocol::{ProtocolManager, ProtocolType};
use blueprint_runner::config::BlueprintEnvironment;
use std::time::Duration;
use tokio::time::timeout;

/// Test that ProtocolManager can be constructed for Tangle
#[tokio::test]
async fn test_tangle_protocol_manager_initialization() {
    // This test uses the existing TangleTestHarness infrastructure
    use blueprint_tangle_testing_utils::TangleTestHarness;
    use tempfile::TempDir;

    let harness_temp_dir = TempDir::new().unwrap();
    let harness: TangleTestHarness<()> = Box::pin(TangleTestHarness::setup(harness_temp_dir)).await.unwrap();
    let env = harness.env().clone();

    let _manager_temp_dir = TempDir::new().unwrap();
    let ctx = common::create_test_context(env.keystore_uri.clone()).await;

    // Create ProtocolManager with Tangle
    let result = ProtocolManager::new(ProtocolType::Tangle, env, &ctx).await;

    assert!(
        result.is_ok(),
        "Failed to create Tangle ProtocolManager: {:?}",
        result.err()
    );
}

/// Test that ProtocolManager can be constructed for EigenLayer
#[tokio::test]
async fn test_eigenlayer_protocol_manager_initialization() {
    // This test uses the existing EigenlayerTestHarness infrastructure
    use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
    use tempfile::TempDir;

    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let env = harness.env().clone();

    let _manager_temp_dir = TempDir::new().unwrap();
    let ctx = common::create_test_context(env.keystore_uri.clone()).await;

    // Create ProtocolManager with EigenLayer
    let result = ProtocolManager::new(ProtocolType::Eigenlayer, env, &ctx).await;

    assert!(
        result.is_ok(),
        "Failed to create EigenLayer ProtocolManager: {:?}",
        result.err()
    );
}

/// Test that ProtocolManager can initialize and receive events from Tangle
#[tokio::test]
async fn test_tangle_protocol_manager_event_flow() {
    use blueprint_tangle_testing_utils::TangleTestHarness;
    use tempfile::TempDir;

    let harness_temp_dir = TempDir::new().unwrap();
    let harness: TangleTestHarness<()> = Box::pin(TangleTestHarness::setup(harness_temp_dir)).await.unwrap();
    let env = harness.env().clone();

    let _manager_temp_dir = TempDir::new().unwrap();
    let ctx = common::create_test_context(env.keystore_uri.clone()).await;

    let mut protocol_manager = ProtocolManager::new(ProtocolType::Tangle, env.clone(), &ctx)
        .await
        .unwrap();

    let mut active_blueprints = ActiveBlueprints::default();

    // Initialize the protocol
    protocol_manager
        .initialize(&env, &ctx, &mut active_blueprints)
        .await
        .expect("Failed to initialize Tangle protocol");

    // Try to get the next event (with timeout)
    let event_result = timeout(Duration::from_secs(5), protocol_manager.next_event()).await;

    // We expect either:
    // - A timeout (no events yet, which is ok)
    // - Or an actual event (if the testnet produced one)
    match event_result {
        Ok(Some(event)) => {
            // Got an event - verify it's a Tangle event
            assert!(event.as_tangle().is_some(), "Expected Tangle event");
        }
        Ok(None) => {
            panic!("Protocol manager returned None, expected Some or timeout");
        }
        Err(_) => {
            // Timeout is acceptable for this test - just verifying the flow works
        }
    }
}

/// Test that ProtocolManager can initialize and receive events from EigenLayer
///
/// With multi-AVS architecture, blueprints only spawn when there are active registrations.
/// This test verifies initialization succeeds without registrations.
#[tokio::test]
async fn test_eigenlayer_protocol_manager_event_flow() {
    use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
    use tempfile::TempDir;

    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let env = harness.env().clone();

    let _manager_temp_dir = TempDir::new().unwrap();
    let ctx = common::create_test_context(env.keystore_uri.clone()).await;

    let mut protocol_manager = ProtocolManager::new(ProtocolType::Eigenlayer, env.clone(), &ctx)
        .await
        .unwrap();

    let mut active_blueprints = ActiveBlueprints::default();

    // Initialize the protocol - with no registrations, no blueprints will spawn
    protocol_manager
        .initialize(&env, &ctx, &mut active_blueprints)
        .await
        .expect("Failed to initialize EigenLayer protocol");

    // With no registrations, no blueprints should be spawned
    assert!(
        active_blueprints.is_empty(),
        "No blueprints should be spawned without registrations"
    );

    // Try to get the next event (with timeout)
    let event_result = timeout(Duration::from_secs(5), protocol_manager.next_event()).await;

    // We expect either:
    // - A timeout (no events yet, which is ok)
    // - Or an actual event (if the testnet produced one)
    match event_result {
        Ok(Some(event)) => {
            // Got an event - verify it's an EigenLayer event
            assert!(
                event.as_eigenlayer().is_some(),
                "Expected EigenLayer event"
            );
        }
        Ok(None) => {
            panic!("Protocol manager returned None, expected Some or timeout");
        }
        Err(_) => {
            // Timeout is acceptable for this test - just verifying the flow works
        }
    }
}

/// Test edge case: invalid protocol configuration
#[tokio::test]
async fn test_invalid_protocol_configuration() {
    use alloy_primitives::Address;
    use blueprint_manager::config::{BlueprintManagerConfig, Paths};
    use blueprint_runner::config::{ContextConfig, ProtocolSettings, SupportedChains};
    use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
    use url::Url;

    let temp_dir = tempfile::TempDir::new().unwrap();
    let keystore_path = temp_dir.path().join("keystore");
    let data_dir = temp_dir.path().join("data");
    let cache_dir = temp_dir.path().join("cache");
    let runtime_dir = temp_dir.path().join("runtime");

    std::fs::create_dir_all(&keystore_path).unwrap();
    std::fs::create_dir_all(&data_dir).unwrap();
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::create_dir_all(&runtime_dir).unwrap();

    // Create dummy contract addresses (they're valid addresses but won't be reachable)
    let dummy_address = Address::ZERO;

    // Create environment with invalid RPC endpoints but valid protocol settings
    let context_config = ContextConfig::create_config(
        Url::parse("http://invalid-host:9999").unwrap(),
        Url::parse("ws://invalid-host:9999").unwrap(),
        keystore_path.to_string_lossy().into_owned(),
        None,
        data_dir.clone(),
        None,
        SupportedChains::LocalTestnet,
        blueprint_runner::config::Protocol::Eigenlayer,
        ProtocolSettings::Eigenlayer(EigenlayerProtocolSettings {
            allocation_manager_address: dummy_address,
            registry_coordinator_address: dummy_address,
            operator_state_retriever_address: dummy_address,
            delegation_manager_address: dummy_address,
            service_manager_address: dummy_address,
            stake_registry_address: dummy_address,
            strategy_manager_address: dummy_address,
            avs_directory_address: dummy_address,
            rewards_coordinator_address: dummy_address,
            permission_controller_address: dummy_address,
            strategy_address: dummy_address,
            allocation_delay: 0,
            deposit_amount: 5_000_000_000_000_000_000_000,
            stake_amount: 1_000_000_000_000_000_000,
            operator_sets: vec![0],
            staker_opt_out_window_blocks: 50400,
            metadata_url: "https://github.com/tangle-network/blueprint".to_string(),
        }),
    );

    let env = BlueprintEnvironment::load_with_config(context_config).unwrap();

    // Create manager config with temp directories
    let manager_config = BlueprintManagerConfig {
        paths: Paths {
            blueprint_config: None,
            keystore_uri: keystore_path.to_string_lossy().into_owned(),
            data_dir,
            cache_dir,
            runtime_dir,
        },
        verbose: 0,
        pretty: false,
        instance_id: None,
        test_mode: true,
        allow_unchecked_attestations: true,
        ..Default::default()
    };

    let ctx = BlueprintManagerContext::new(manager_config).await.unwrap();

    // Attempting to create ProtocolManager should fail due to connection issues
    let result = timeout(
        Duration::from_secs(10),
        ProtocolManager::new(ProtocolType::Eigenlayer, env, &ctx),
    )
    .await;

    // We expect either a timeout or an error
    assert!(
        result.is_err() || result.unwrap().is_err(),
        "Should fail with invalid configuration"
    );
}

/// Test edge case: protocol type mismatch
#[tokio::test]
async fn test_protocol_type_conversion() {
    use blueprint_manager::protocol::ProtocolType;
    use blueprint_runner::config::ProtocolSettings;

    // Test Tangle conversion
    let tangle_settings =
        ProtocolSettings::Tangle(blueprint_runner::tangle::config::TangleProtocolSettings {
            blueprint_id: 1,
            service_id: Some(0),
        });
    let protocol_type: ProtocolType = (&tangle_settings).into();
    assert!(matches!(protocol_type, ProtocolType::Tangle));

    // Test EigenLayer conversion
    let eigenlayer_settings = ProtocolSettings::Eigenlayer(
        blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings::default(),
    );
    let protocol_type: ProtocolType = (&eigenlayer_settings).into();
    assert!(matches!(protocol_type, ProtocolType::Eigenlayer));

    // Test None defaults to Tangle
    let none_settings = ProtocolSettings::None;
    let protocol_type: ProtocolType = (&none_settings).into();
    assert!(matches!(protocol_type, ProtocolType::Tangle));
}
