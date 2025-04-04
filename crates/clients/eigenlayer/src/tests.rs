use super::*;
use alloy_primitives::U256;
use alloy_primitives::address;
use alloy_primitives::aliases::U96;
use alloy_provider::Provider;
use blueprint_eigenlayer_testing_utils::EigenlayerTestHarness;

use blueprint_chain_setup_anvil::get_receipt;
use blueprint_core_testing_utils::setup_log;
use blueprint_evm_extra::util::get_provider_from_signer;

use eigenlayer_contract_deployer::bindings::core::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigenlayer_contract_deployer::bindings::core::registrycoordinator::IStakeRegistryTypes::StrategyParams;
use eigenlayer_contract_deployer::bindings::RegistryCoordinator;
use eigenlayer_contract_deployer::core::{
    deploy_core_contracts, DelegationManagerConfig, DeployedCoreContracts, DeploymentConfigData, EigenPodManagerConfig, RewardsCoordinatorConfig, StrategyFactoryConfig, StrategyManagerConfig
};
use client::EigenlayerClient;
use eigenlayer_contract_deployer::deploy::deploy_avs_contracts;
use eigenlayer_contract_deployer::deploy::DeployedContracts;
use eigenlayer_contract_deployer::permissions::setup_avs_permissions;

async fn setup_test_environment() -> EigenlayerTestHarness<()> {
    setup_log();

    // Initialize test harness
    let temp_dir = tempfile::TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(temp_dir).await.unwrap();

    let http_endpoint = harness.http_endpoint.to_string();
    let owner_account = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let task_generator_account = address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65");
    let aggregator_account = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    let core_config = DeploymentConfigData {
        strategy_manager: StrategyManagerConfig {
            init_paused_status: U256::from(0),
            init_withdrawal_delay_blocks: 1u32,
        },
        delegation_manager: DelegationManagerConfig {
            init_paused_status: U256::from(0),
            withdrawal_delay_blocks: 0u32,
        },
        eigen_pod_manager: EigenPodManagerConfig {
            init_paused_status: U256::from(0),
        },
        rewards_coordinator: RewardsCoordinatorConfig {
            init_paused_status: U256::from(0),
            max_rewards_duration: 864_000_u32,
            max_retroactive_length: 432_000_u32,
            max_future_length: 864_000_u32,
            genesis_rewards_timestamp: 1_672_531_200_u32,
            updater: owner_account,
            activation_delay: 0u32,
            calculation_interval_seconds: 86400u32,
            global_operator_commission_bips: 1000u16,
        },
        strategy_factory: StrategyFactoryConfig {
            init_paused_status: U256::from(0),
        },
    };

    let core_contracts = deploy_core_contracts(
        &http_endpoint,
        private_key,
        owner_account,
        core_config,
        Some(address!("00000000219ab540356cBB839Cbe05303d7705Fa")),
        Some(1_564_000),
    )
    .await
    .unwrap();

    let DeployedCoreContracts {
        delegation_manager: delegation_manager_address,
        avs_directory: avs_directory_address,
        allocation_manager: allocation_manager_address,
        rewards_coordinator: rewards_coordinator_address,
        pauser_registry: pauser_registry_address,
        strategy_factory: strategy_factory_address,
        permission_controller: permission_controller_address,
        ..
    } = core_contracts;

    let avs_contracts = deploy_avs_contracts(
        &http_endpoint,
        private_key,
        owner_account,
        1,
        permission_controller_address,
        allocation_manager_address,
        avs_directory_address,
        delegation_manager_address,
        pauser_registry_address,
        rewards_coordinator_address,
        strategy_factory_address,
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

    println!("Setting AVS permissions and Metadata...");
    println!("Private key: {}", private_key);
    let signer_wallet = get_provider_from_signer(private_key, &http_endpoint);

    match setup_avs_permissions(
        &core_contracts,
        &avs_contracts,
        &signer_wallet,
        harness.owner_account(),
        "https://github.com/tangle-network/avs/blob/main/metadata.json".to_string(),
    )
    .await
    {
        Ok(()) => println!("Successfully set up AVS permissions"),
        Err(e) => {
            println!("Failed to set up AVS permissions: {}", e);
            panic!("Failed to set up AVS permissions: {}", e);
        }
    }

    let registry_coordinator =
        RegistryCoordinator::new(registry_coordinator_address, signer_wallet.clone());

    let operator_set_param = OperatorSetParam {
        maxOperatorCount: 3,
        kickBIPsOfOperatorStake: 100,
        kickBIPsOfTotalStake: 100,
    };

    let strategy_params = StrategyParams {
        strategy: strategy_address,
        multiplier: U96::from(1),
    };

    let minimum_stake = U96::from(0);

    println!(
        "Attempting to create quorum with strategy: {}",
        strategy_address
    );

    let create_quorum_call = registry_coordinator.createTotalDelegatedStakeQuorum(
        operator_set_param.clone(),
        minimum_stake,
        vec![strategy_params],
    );

    println!("Sent createTotalDelegatedStakeQuorum transaction");

    let create_quorum_receipt = get_receipt(create_quorum_call).await;
    match create_quorum_receipt {
        Ok(receipt) => {
            println!("Quorum created with receipt: {:?}", receipt);
            if receipt.status() {
                println!(
                    "Quorum created with transaction hash: {:?}",
                    receipt.transaction_hash
                );
            } else {
                println!("Failed to create quorum: {:?}", receipt);
                panic!("Failed to create quorum: {:?}", receipt);
            }
        }
        Err(e) => {
            println!("Failed to create quorum: {}", e);
            panic!("Failed to create quorum: {}", e);
        }
    }

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    harness
}

#[tokio::test]
async fn get_provider_http() {
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let provider = client.get_provider_http();
    assert!(provider.get_block_number().await.is_ok());
}

#[tokio::test]
async fn get_provider_ws() {
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let provider = client.get_provider_ws().await.unwrap();
    assert!(provider.get_block_number().await.is_ok());
}

// TODO
// #[tokio::test]
// async fn get_slasher_address() {
//     let env = setup_test_environment().await;
//     let client = EigenlayerClient::new(env.config.clone());
//     let delegation_manager_addr = address!("dc64a140aa3e981100a9beca4e685f962f0cf6c9");
//     let result = client.get_slasher_address(delegation_manager_addr).await;
//     assert!(result.is_ok());
// }

#[tokio::test]
async fn avs_registry_reader() {
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let _ = client.avs_registry_reader().await.unwrap();
}

#[tokio::test]
async fn avs_registry_writer() {
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let _ = client
        .avs_registry_writer(private_key.to_string())
        .await
        .unwrap();
}

#[tokio::test]
async fn operator_info_service() {
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let _ = client.operator_info_service_in_memory().await.unwrap();
}

#[tokio::test]
async fn get_operator_stake_in_quorums() {
    setup_log();
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let operator_address = address!("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    let _ = client
        .get_operator_stake_in_quorums_at_current_block(operator_address.into_word())
        .await
        .unwrap();
}

#[tokio::test]
async fn get_operator_id() {
    let harness = setup_test_environment().await;
    let client = EigenlayerClient::new(harness.env().clone());
    let operator_addr = address!("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    let _ = client.get_operator_id(operator_addr).await.unwrap();
}
