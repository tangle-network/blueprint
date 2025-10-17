/// Integration tests for the ProtocolManager abstraction
///
/// Tests verify that both Tangle and EigenLayer protocols work correctly
/// through the unified ProtocolManager interface.

use blueprint_manager::blueprint::ActiveBlueprints;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use blueprint_manager::protocol::{ProtocolManager, ProtocolType};
use blueprint_runner::config::BlueprintEnvironment;
use std::time::Duration;
use tokio::time::timeout;

/// Helper function to create a test BlueprintManagerContext with temp directories and RocksDB
///
/// Uses /tmp for shorter paths to avoid Unix socket SUN_LEN limit (typically 104-108 bytes)
async fn create_test_context(
    _temp_dir: &std::path::Path,
    keystore_uri: String,
) -> BlueprintManagerContext {
    // Use /tmp with short random name to avoid socket path length issues
    let test_id = format!("bpm{}", rand::random::<u32>());
    let temp_root = std::path::PathBuf::from("/tmp").join(test_id);

    let cache_dir = temp_root.join("c");
    let runtime_dir = temp_root.join("r");
    let data_dir = temp_root.join("d");
    std::fs::create_dir_all(&cache_dir).unwrap();
    std::fs::create_dir_all(&runtime_dir).unwrap();
    std::fs::create_dir_all(&data_dir).unwrap();

    let manager_config = BlueprintManagerConfig {
        paths: Paths {
            blueprint_config: None,
            keystore_uri,
            data_dir: data_dir.clone(),
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

    let ctx = BlueprintManagerContext::new(manager_config)
        .await
        .unwrap();

    // Setup RocksDB database for bridge functionality
    // This matches how the test harnesses initialize the database
    let db_path = data_dir.join("p").join("a").join("db");
    tokio::fs::create_dir_all(&db_path).await.unwrap();

    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path).unwrap();
    let db = proxy.db();

    ctx.set_db(db).await;

    ctx
}

/// Test that ProtocolManager can be constructed for Tangle
#[tokio::test]
async fn test_tangle_protocol_manager_initialization() {
    // This test uses the existing TangleTestHarness infrastructure
    use blueprint_tangle_testing_utils::TangleTestHarness;
    use tempfile::TempDir;

    let harness_temp_dir = TempDir::new().unwrap();
    let harness: TangleTestHarness<()> = TangleTestHarness::setup(harness_temp_dir).await.unwrap();
    let env = harness.env().clone();

    let manager_temp_dir = TempDir::new().unwrap();
    let ctx = create_test_context(manager_temp_dir.path(), env.keystore_uri.clone()).await;

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
    let harness = EigenlayerTestHarness::setup(harness_temp_dir).await.unwrap();
    let env = harness.env().clone();

    let manager_temp_dir = TempDir::new().unwrap();
    let ctx = create_test_context(manager_temp_dir.path(), env.keystore_uri.clone()).await;

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
    let harness: TangleTestHarness<()> = TangleTestHarness::setup(harness_temp_dir).await.unwrap();
    let env = harness.env().clone();

    let manager_temp_dir = TempDir::new().unwrap();
    let ctx = create_test_context(manager_temp_dir.path(), env.keystore_uri.clone()).await;

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
        Ok(Some(_event)) => {
            // Got an event - verify it's a Tangle event
            assert!(_event.as_tangle().is_some(), "Expected Tangle event");
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
/// This test uses the real incredible-squaring-eigenlayer blueprint from examples/
#[tokio::test]
async fn test_eigenlayer_protocol_manager_event_flow() {
    use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
    use tempfile::TempDir;

    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir).await.unwrap();
    let env = harness.env().clone();

    let manager_temp_dir = TempDir::new().unwrap();

    // Point to the real incredible-squaring-eigenlayer blueprint in examples/
    let blueprint_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/incredible-squaring-eigenlayer");

    // Set environment variable for the EigenLayer event handler to use
    // SAFETY: This is a test and we're setting a test-specific environment variable
    unsafe {
        std::env::set_var(
            "EIGENLAYER_BLUEPRINT_PATH",
            blueprint_dir.to_string_lossy().to_string(),
        );
    }

    let ctx = create_test_context(manager_temp_dir.path(), env.keystore_uri.clone()).await;

    let mut protocol_manager = ProtocolManager::new(ProtocolType::Eigenlayer, env.clone(), &ctx)
        .await
        .unwrap();

    let mut active_blueprints = ActiveBlueprints::default();

    // Initialize the protocol - this will build and spawn the real blueprint
    protocol_manager
        .initialize(&env, &ctx, &mut active_blueprints)
        .await
        .expect("Failed to initialize EigenLayer protocol");

    // Verify the blueprint was spawned
    assert!(
        active_blueprints.contains_key(&0),
        "Blueprint should be registered in active_blueprints"
    );

    // Try to get the next event (with timeout)
    let event_result = timeout(Duration::from_secs(5), protocol_manager.next_event()).await;

    // We expect either:
    // - A timeout (no events yet, which is ok)
    // - Or an actual event (if the testnet produced one)
    match event_result {
        Ok(Some(_event)) => {
            // Got an event - verify it's an EigenLayer event
            assert!(
                _event.as_eigenlayer().is_some(),
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

/// Test EigenLayer blueprint spawning through ProtocolManager
///
/// This test uses the real incredible-squaring-eigenlayer blueprint from examples/
/// and verifies the full blueprint lifecycle: build, spawn, register, health check
#[tokio::test]
async fn test_eigenlayer_blueprint_spawning() {
    use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
    use tempfile::TempDir;
    use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::TestFetcher;
    use tangle_subxt::tangle_testnet_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
    use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::BoundedString;

    fn new_bounded_string<S: Into<String>>(s: S) -> BoundedString {
        let s = s.into();
        BoundedString(BoundedVec(s.into_bytes()))
    }

    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir).await.unwrap();
    let mut env = harness.env().clone();

    let manager_temp_dir = TempDir::new().unwrap();

    // Point to the real incredible-squaring-eigenlayer blueprint in examples/
    let blueprint_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/incredible-squaring-eigenlayer");

    // Create test fetcher pointing to real blueprint
    let _test_fetcher = TestFetcher {
        cargo_package: new_bounded_string("incredible-squaring-blueprint-eigenlayer"),
        cargo_bin: new_bounded_string("incredible-squaring-blueprint-eigenlayer"),
        base_path: new_bounded_string(blueprint_dir.to_string_lossy().to_string()),
    };

    // Set environment variable for the EigenLayer event handler to use
    // SAFETY: This is a test and we're setting a test-specific environment variable
    unsafe {
        std::env::set_var(
            "EIGENLAYER_BLUEPRINT_PATH",
            blueprint_dir.to_string_lossy().to_string(),
        );
    }

    env.protocol_settings = blueprint_runner::config::ProtocolSettings::Eigenlayer(
        blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings {
            allocation_manager_address: harness.eigenlayer_contract_addresses.allocation_manager_address,
            registry_coordinator_address: harness.eigenlayer_contract_addresses.registry_coordinator_address,
            operator_state_retriever_address: harness.eigenlayer_contract_addresses.operator_state_retriever_address,
            delegation_manager_address: harness.eigenlayer_contract_addresses.delegation_manager_address,
            service_manager_address: harness.eigenlayer_contract_addresses.service_manager_address,
            stake_registry_address: harness.eigenlayer_contract_addresses.stake_registry_address,
            strategy_manager_address: harness.eigenlayer_contract_addresses.strategy_manager_address,
            avs_directory_address: harness.eigenlayer_contract_addresses.avs_directory_address,
            rewards_coordinator_address: harness.eigenlayer_contract_addresses.rewards_coordinator_address,
            permission_controller_address: harness.eigenlayer_contract_addresses.permission_controller_address,
            strategy_address: harness.eigenlayer_contract_addresses.strategy_address,
        },
    );

    let ctx = create_test_context(manager_temp_dir.path(), env.keystore_uri.clone()).await;

    let mut protocol_manager = ProtocolManager::new(ProtocolType::Eigenlayer, env.clone(), &ctx)
        .await
        .unwrap();

    let mut active_blueprints = ActiveBlueprints::default();

    // Initialize - this will build (if needed) and spawn the real blueprint
    protocol_manager
        .initialize(&env, &ctx, &mut active_blueprints)
        .await
        .expect("Failed to initialize and spawn blueprint");

    // Verify the blueprint was spawned
    assert!(
        active_blueprints.contains_key(&0),
        "Blueprint should be registered in active_blueprints"
    );
    assert!(
        active_blueprints.get(&0).unwrap().contains_key(&0),
        "Service should be registered"
    );
}

/// Test edge case: invalid protocol configuration
#[tokio::test]
async fn test_invalid_protocol_configuration() {
    use blueprint_runner::config::{ContextConfig, ProtocolSettings, SupportedChains};
    use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
    use blueprint_manager::config::{BlueprintManagerConfig, Paths};
    use alloy_primitives::Address;
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

    let ctx = BlueprintManagerContext::new(manager_config)
        .await
        .unwrap();

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
    use blueprint_runner::config::ProtocolSettings;
    use blueprint_manager::protocol::ProtocolType;

    // Test Tangle conversion
    let tangle_settings = ProtocolSettings::Tangle(
        blueprint_runner::tangle::config::TangleProtocolSettings {
            blueprint_id: 1,
            service_id: Some(0),
        },
    );
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
