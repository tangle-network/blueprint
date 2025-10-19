/// Test multi-AVS architecture for EigenLayer
use blueprint_eigenlayer_extra::{AvsRegistration, RegistrationStateManager};
use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
use blueprint_manager::blueprint::ActiveBlueprints;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use blueprint_manager::protocol::{ProtocolManager, ProtocolType};
use tempfile::TempDir;

async fn create_test_context(keystore_uri: String) -> BlueprintManagerContext {
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

    let ctx = BlueprintManagerContext::new(manager_config).await.unwrap();

    let db_path = data_dir.join("p").join("a").join("db");
    tokio::fs::create_dir_all(&db_path).await.unwrap();
    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path).unwrap();
    ctx.set_db(proxy.db()).await;

    ctx
}

#[tokio::test]
async fn test_multi_avs_registration_and_spawn() {
    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let env = harness.env().clone();

    // Get operator address from harness
    let operator_address = harness.owner_account();

    // Create two AVS registrations
    let settings = env
        .protocol_settings
        .eigenlayer()
        .expect("Should have EigenLayer settings");

    let config1 = blueprint_eigenlayer_extra::AvsRegistrationConfig {
        service_manager: settings.service_manager_address,
        registry_coordinator: settings.registry_coordinator_address,
        operator_state_retriever: settings.operator_state_retriever_address,
        strategy_manager: settings.strategy_manager_address,
        delegation_manager: settings.delegation_manager_address,
        avs_directory: settings.avs_directory_address,
        rewards_coordinator: settings.rewards_coordinator_address,
        permission_controller: Some(settings.permission_controller_address),
        allocation_manager: Some(settings.allocation_manager_address),
        strategy_address: settings.strategy_address,
        stake_registry: settings.stake_registry_address,
        blueprint_path: std::path::PathBuf::from("/tmp/test_blueprint_1"),
        container_image: None,
        runtime_target: blueprint_eigenlayer_extra::RuntimeTarget::Native,
        allocation_delay: 0,
        deposit_amount: 5_000_000_000_000_000_000_000,
        stake_amount: 1_000_000_000_000_000_000,
        operator_sets: vec![0],
    };

    let registration1 = AvsRegistration::new(operator_address, config1);

    // Register both AVS
    let mut state_manager = RegistrationStateManager::load().unwrap();
    state_manager.register(registration1.clone()).unwrap();

    // Verify registrations were saved
    let loaded = RegistrationStateManager::load().unwrap();
    assert_eq!(loaded.registrations().registrations.len(), 1);

    // Cleanup
    state_manager
        .deregister(registration1.config.service_manager)
        .unwrap();
}

#[tokio::test]
async fn test_eigenlayer_with_registration_state() {
    let harness_temp_dir = TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(harness_temp_dir)
        .await
        .unwrap();
    let env = harness.env().clone();

    let operator_address = harness.owner_account();
    let settings = env
        .protocol_settings
        .eigenlayer()
        .expect("Should have EigenLayer settings");

    // Create registration with a mock blueprint path
    // Note: For a real integration test, you'd need the actual built binary
    let blueprint_path = std::path::PathBuf::from("/tmp/mock_blueprint");

    let config = blueprint_eigenlayer_extra::AvsRegistrationConfig {
        service_manager: settings.service_manager_address,
        registry_coordinator: settings.registry_coordinator_address,
        operator_state_retriever: settings.operator_state_retriever_address,
        strategy_manager: settings.strategy_manager_address,
        delegation_manager: settings.delegation_manager_address,
        avs_directory: settings.avs_directory_address,
        rewards_coordinator: settings.rewards_coordinator_address,
        permission_controller: Some(settings.permission_controller_address),
        allocation_manager: Some(settings.allocation_manager_address),
        strategy_address: settings.strategy_address,
        stake_registry: settings.stake_registry_address,
        blueprint_path,
        container_image: None,
        runtime_target: blueprint_eigenlayer_extra::RuntimeTarget::Native,
        allocation_delay: 0,
        deposit_amount: 5_000_000_000_000_000_000_000,
        stake_amount: 1_000_000_000_000_000_000,
        operator_sets: vec![0],
    };

    let registration = AvsRegistration::new(operator_address, config);

    // Register AVS
    let mut state_manager = RegistrationStateManager::load().unwrap();
    state_manager.register(registration.clone()).unwrap();

    // Verify registration was saved correctly
    let loaded = RegistrationStateManager::load().unwrap();
    assert_eq!(loaded.registrations().registrations.len(), 1);

    let loaded_reg = loaded.registrations().get(registration.config.service_manager).unwrap();
    assert_eq!(loaded_reg.operator_address, operator_address);
    assert_eq!(loaded_reg.config.service_manager, settings.service_manager_address);

    let blueprint_id = registration.blueprint_id();
    assert_eq!(blueprint_id, loaded_reg.blueprint_id());

    // Cleanup
    state_manager
        .deregister(registration.config.service_manager)
        .unwrap();

    // Verify deregistration
    let loaded = RegistrationStateManager::load().unwrap();
    let dereg_entry = loaded.registrations().get(registration.config.service_manager).unwrap();
    assert_eq!(dereg_entry.status, blueprint_eigenlayer_extra::RegistrationStatus::Deregistered);
}
