/// Common test utilities for EigenLayer integration tests
///
/// This module provides shared helpers to reduce code duplication across test files.
use alloy_primitives::Address;
use blueprint_chain_setup::anvil::AnvilTestnet;
use blueprint_chain_setup::anvil::keys::ANVIL_PRIVATE_KEYS;
use blueprint_eigenlayer_extra::{AvsRegistrationConfig, RuntimeTarget};
use blueprint_manager::config::{BlueprintManagerConfig, BlueprintManagerContext, Paths};
use blueprint_runner::eigenlayer::config::EigenlayerProtocolSettings;
use blueprint_testing_utils::eigenlayer::{
    EigenlayerTestHarness, deploy_eigenlayer_core_contracts, get_accounts, get_aggregator_account,
    get_owner_account, get_task_generator_account,
};
use eigenlayer_contract_deployer::deploy::{DeployedContracts, deploy_avs_contracts};
use eigenlayer_contract_deployer::helpers::get_provider_from_signer;
use eigenlayer_contract_deployer::permissions::setup_avs_permissions;
use std::path::PathBuf;

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
///
/// * Params
/// * `testnet` - Anvil testnet
///
/// * Returns
/// * `(EigenlayerTestHarness, Vec<Address>)` - Eigenlayer test harness and accounts
/// * `EigenlayerTestHarness` - Eigenlayer test harness
/// * `Vec<Address>` - Accounts
///
/// * # Errors
/// * `Error` - Error deploying core contracts
/// * `Error` - Error deploying AVS contracts
/// * `Error` - Error setting up AVS permissions
///
/// * # Panics
/// * Panics if error deploying core contracts
pub async fn setup_incredible_squaring_avs_harness(
    testnet: AnvilTestnet,
) -> (EigenlayerTestHarness, Vec<Address>) {
    let http_endpoint = testnet.http_endpoint.clone();
    let accounts = get_accounts(http_endpoint.clone()).await;
    let owner_account = get_owner_account(&accounts);
    let task_generator_account = get_task_generator_account(&accounts);
    let aggregator_account = get_aggregator_account(&accounts);

    // Owner account private key
    let private_key = ANVIL_PRIVATE_KEYS[0].to_string();
    let temp_dir = tempfile::TempDir::new().unwrap();

    let core_contracts =
        deploy_eigenlayer_core_contracts(http_endpoint.as_str(), &private_key, owner_account)
            .await
            .unwrap();

    let avs_contracts = deploy_avs_contracts(
        http_endpoint.as_str(),
        &private_key,
        owner_account,
        1,
        core_contracts.permission_controller,
        core_contracts.allocation_manager,
        core_contracts.avs_directory,
        core_contracts.delegation_manager,
        core_contracts.pauser_registry,
        core_contracts.rewards_coordinator,
        core_contracts.strategy_factory,
        task_generator_account,
        aggregator_account,
        10,
    )
    .await
    .unwrap();

    let DeployedContracts {
        registry_coordinator: registry_coordinator_address,
        strategy: strategy_address,
        ..
    } = avs_contracts;

    blueprint_core::info!("AVS Contracts deployed at: {:?}", avs_contracts);

    blueprint_core::info!("Setting AVS permissions and Metadata...");
    let signer_wallet = get_provider_from_signer(&private_key, http_endpoint.as_str());

    match setup_avs_permissions(
        &core_contracts,
        &avs_contracts,
        &signer_wallet,
        owner_account,
        "https://github.com/tangle-network/avs/blob/main/metadata.json".to_string(),
    )
    .await
    {
        Ok(_set_up_avs_res) => blueprint_core::info!("Successfully set up AVS permissions"),
        Err(e) => {
            blueprint_core::error!("Failed to set up AVS permissions: {e}");
            panic!("Failed to set up AVS permissions: {e}");
        }
    }

    // Initialize test harness
    let harness = EigenlayerTestHarness::setup(
        private_key.as_str(),
        temp_dir,
        testnet,
        Some(EigenlayerProtocolSettings {
            allocation_manager_address: core_contracts.allocation_manager,
            registry_coordinator_address,
            operator_state_retriever_address: avs_contracts.operator_state_retriever,
            delegation_manager_address: core_contracts.delegation_manager,
            service_manager_address: avs_contracts.squaring_service_manager,
            stake_registry_address: avs_contracts.stake_registry,
            strategy_manager_address: core_contracts.strategy_manager,
            avs_directory_address: core_contracts.avs_directory,
            rewards_coordinator_address: core_contracts.rewards_coordinator,
            permission_controller_address: core_contracts.permission_controller,
            strategy_address,
            // Registration parameters (use defaults for testing)
            allocation_delay: 0,
            deposit_amount: 5_000_000_000_000_000_000_000,
            stake_amount: 1_000_000_000_000_000_000,
            operator_sets: vec![0],
            staker_opt_out_window_blocks: 50400,
            metadata_url: "https://github.com/tangle-network/blueprint".to_string(),
        }),
    )
    .await
    .unwrap();

    (harness, accounts)
}
