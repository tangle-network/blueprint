/// Common test utilities for EigenLayer integration tests
///
/// This module provides shared helpers to reduce code duplication across test files.
use blueprint_eigenlayer_extra::{AvsRegistrationConfig, RuntimeTarget};
use blueprint_testing_utils::eigenlayer::EigenlayerTestHarness;
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use std::path::PathBuf;

pub const ANVIL_PRIVATE_KEYS: [&str; 10] = [
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6",
    "47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a",
    "8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba",
    "92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e",
    "4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356",
    "dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97",
    "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
];

/// Create a test `BlueprintManagerContext` with temp directories and `RocksDB`
///
/// Uses /tmp for shorter paths to avoid Unix socket `SUN_LEN` limit (typically 104-108 bytes)
///
/// # Arguments
///
/// * `keystore_uri` - Path to keystore
///
/// # Panics
///
/// Panics if directory creation fails, context initialization fails, or database setup fails
#[allow(dead_code)]
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
///
/// # Panics
///
/// Panics if the harness does not have EigenLayer protocol settings configured
#[must_use]
#[allow(dead_code)]
pub fn create_avs_config(
    harness: &EigenlayerTestHarness,
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


/// Deploy core Eigenlayer contract & AVS contract
pub fn deploy_eigenlayer() {
   // TODO(daniel) 
}