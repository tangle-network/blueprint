/// Common test utilities for EigenLayer integration tests
///
/// This module provides shared helpers to reduce code duplication across test files.

use blueprint_eigenlayer_extra::{AvsRegistrationConfig, RuntimeTarget};
use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use std::path::PathBuf;

/// Create a test BlueprintManagerContext with temp directories and RocksDB
///
/// Uses /tmp for shorter paths to avoid Unix socket SUN_LEN limit (typically 104-108 bytes)
///
/// # Arguments
///
/// * `keystore_uri` - Path to keystore
pub async fn create_test_context(keystore_uri: String) -> BlueprintManagerContext {
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

    // Setup RocksDB database for bridge functionality
    let db_path = data_dir.join("p").join("a").join("db");
    tokio::fs::create_dir_all(&db_path).await.unwrap();

    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path).unwrap();
    let db = proxy.db();

    ctx.set_db(db).await;

    ctx
}

/// Create AVS registration config from test harness
///
/// # Arguments
///
/// * `harness` - EigenLayer test harness
/// * `blueprint_path` - Path to blueprint binary
/// * `runtime_target` - Runtime target (Native, Hypervisor, Container)
pub fn create_avs_config(
    harness: &EigenlayerTestHarness<()>,
    blueprint_path: PathBuf,
    runtime_target: RuntimeTarget,
) -> AvsRegistrationConfig {
    let settings = harness
        .env()
        .protocol_settings
        .eigenlayer()
        .expect("Should have EigenLayer settings");

    AvsRegistrationConfig {
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
        runtime_target,
        allocation_delay: 0,
        deposit_amount: 5_000_000_000_000_000_000_000,
        stake_amount: 1_000_000_000_000_000_000,
        operator_sets: vec![0],
    }
}
