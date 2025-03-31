use crate::constants::{AGGREGATOR_PRIVATE_KEY, PRIVATE_KEY};
use crate::contexts::aggregator::AggregatorContext;
use crate::contexts::client::AggregatorClient;
use crate::contexts::combined::CombinedContext;
use crate::contexts::x_square::EigenSquareContext;
use crate::contracts::SquaringTask;
use crate::jobs::compute_x_square::xsquare_eigen;
use crate::jobs::initialize_task::initialize_bls_task;
use crate::tests::core::{
    DelegationManagerConfig, DeployedCoreContracts, DeploymentConfigData, EigenPodManagerConfig,
    RewardsCoordinatorConfig, StrategyFactoryConfig, StrategyManagerConfig,
};
use crate::tests::deploy::registry_coordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use crate::tests::deploy::registry_coordinator::IStakeRegistryTypes::StrategyParams;
use crate::tests::deploy::{DeployedContracts, registry_coordinator::RegistryCoordinator};
use crate::tests::permissions::setup_avs_permissions;
use alloy_network::EthereumWallet;
use alloy_primitives::aliases::U96;
use alloy_primitives::{Address, Bytes, U256, address};
use alloy_provider::Provider;
use alloy_signer_local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_provider_ws;
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::evm::util::{get_provider_from_signer, get_provider_http};
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use blueprint_sdk::testing::utils::anvil::wait_for_responses;
use blueprint_sdk::testing::utils::eigenlayer::EigenlayerTestHarness;
use blueprint_sdk::testing::utils::setup_log;
use blueprint_sdk::{Router, error, info, warn};
use eigensdk::client_avsregistry::writer::AvsRegistryChainWriter;
use eigensdk::logging::get_test_logger;
use futures::StreamExt;
use tokio::sync::oneshot;

#[tokio::test(flavor = "multi_thread")]
async fn test_eigenlayer_incredible_squaring_blueprint() {
    run_eigenlayer_incredible_squaring_test(false, 1).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn test_eigenlayer_pre_register_incredible_squaring_blueprint() {
    run_eigenlayer_incredible_squaring_test(true, 1).await;
}

async fn run_eigenlayer_incredible_squaring_test(
    exit_after_registration: bool,
    expected_responses: usize,
) {
    setup_log();

    // Initialize test harness
    let temp_dir = tempfile::TempDir::new().unwrap();
    let harness = EigenlayerTestHarness::setup(temp_dir).await.unwrap();

    let env = harness.env().clone();
    let http_endpoint = harness.http_endpoint.to_string();

    let aggregator_private_key = AGGREGATOR_PRIVATE_KEY.to_string();
    let private_key = PRIVATE_KEY.to_string();
    // let contract_addresses = harness.eigenlayer_contract_addresses;
    // let delegation_manager_address = contract_addresses.delegation_manager_address;
    // let permission_controller_address = contract_addresses.permission_controller_address;
    // let allocation_manager_address = contract_addresses.allocation_manager_address;
    // let avs_directory_addr = contract_addresses.avs_directory_address;
    // let rewards_coordinator_addr = contract_addresses.rewards_coordinator_address;
    // let pauser_registry_addr = PAUSER_REGISTRY_ADDR;
    // let strategy_manager_addr = contract_addresses.strategy_manager_address;

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
            max_rewards_duration: 864000u32,
            max_retroactive_length: 432000u32,
            max_future_length: 86400u32,
            genesis_rewards_timestamp: 1672531200u32,
            updater: harness.owner_account(),
            activation_delay: 0u32,
            calculation_interval_seconds: 86400u32,
            global_operator_commission_bips: 1000u16,
        },
        strategy_factory: StrategyFactoryConfig {
            init_paused_status: U256::from(0),
        },
    };

    let core_contracts = crate::tests::core::deploy_core_contracts(
        &http_endpoint,
        &private_key,
        harness.owner_account(),
        core_config,
        Some(address!("00000000219ab540356cBB839Cbe05303d7705Fa")),
        Some(1_564_000),
    )
    .await
    .unwrap();

    let DeployedCoreContracts {
        proxy_admin: proxy_admin_address,
        delegation_manager: delegation_manager_address,
        delegation_manager_impl: delegation_manager_impl_address,
        avs_directory: avs_directory_address,
        avs_directory_impl: avs_directory_impl_address,
        strategy_manager: strategy_manager_address,
        strategy_manager_impl: strategy_manager_impl_address,
        eigen_pod_manager: eigen_pod_manager_address,
        eigen_pod_manager_impl: eigen_pod_manager_impl_address,
        allocation_manager: allocation_manager_address,
        allocation_manager_impl: allocation_manager_impl_address,
        rewards_coordinator: rewards_coordinator_address,
        rewards_coordinator_impl: rewards_coordinator_impl_address,
        eigen_pod_beacon: eigen_pod_beacon_address,
        pauser_registry: pauser_registry_address,
        strategy_factory: strategy_factory_address,
        strategy_factory_impl: strategy_factory_impl_address,
        strategy_beacon: strategy_beacon_address,
        permission_controller: permission_controller_address,
        permission_controller_impl: permission_controller_impl_address,
    } = core_contracts;

    let core_contracts_json = serde_json::to_string_pretty(&core_contracts).unwrap();
    std::fs::write("core_contracts.json", core_contracts_json).unwrap();

    let avs_contracts = crate::tests::deploy::deploy_avs_contracts(
        &env.http_rpc_endpoint,
        &private_key,
        harness.owner_account(),
        1,
        vec![10],
        harness.owner_account(),
        permission_controller_address,
        allocation_manager_address,
        avs_directory_address,
        delegation_manager_address,
        pauser_registry_address,
        rewards_coordinator_address,
        strategy_factory_address,
        harness.task_generator_account(),
        harness.aggregator_account(),
        10,
    )
    .await
    .unwrap();

    let DeployedContracts {
        proxy_admin: proxy_admin_address,
        squaring_service_manager: service_manager_address,
        squaring_task_manager: task_manager_address,
        squaring_service_manager_impl: service_manager_impl_address,
        registry_coordinator: registry_coordinator_address,
        bls_apk_registry: bls_apk_registry_address,
        index_registry: index_registry_address,
        stake_registry: stake_registry_address,
        operator_state_retriever: operator_state_retriever_address,
        strategy: strategy_address,
        pauser_registry: pauser_registry_address,
        token: token_address,
        instant_slasher: instant_slasher_address,
        socket_registry: socket_registry_address,
    } = avs_contracts;

    let avs_contracts_json = serde_json::to_string_pretty(&avs_contracts).unwrap();
    std::fs::write("avs_contracts.json", avs_contracts_json).unwrap();
    info!("AVS Contracts deployed at: {:?}", avs_contracts);

    info!("Setting AVS permissions and Metadata...");
    // Extract necessary data from harness before moving it
    let ws_endpoint = harness.ws_endpoint.to_string();
    let accounts = harness.accounts().to_vec();
    let task_generator_address = harness.task_generator_account();
    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ");
    warn!("Private key: {}", private_key);
    warn!(
        "Aggregator private key: {}",
        AGGREGATOR_PRIVATE_KEY.as_str()
    );
    let signer_wallet = get_provider_from_signer(&private_key, &http_endpoint);
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(&http_endpoint, wallet.clone());

    match setup_avs_permissions(
        &core_contracts,
        &avs_contracts,
        &signer_wallet,
        harness.owner_account(),
        "https://github.com/tangle-network/avs/blob/main/metadata.json".to_string(),
    )
    .await
    {
        Ok(_) => info!("Successfully set up AVS permissions"),
        Err(e) => {
            error!("Failed to set up AVS permissions: {}", e);
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

    info!(
        "Attempting to create quorum with strategy: {}",
        strategy_address
    );

    let create_quorum_call = registry_coordinator.createTotalDelegatedStakeQuorum(
        operator_set_param.clone(),
        minimum_stake,
        vec![strategy_params],
    );

    info!("Sent createTotalDelegatedStakeQuorum transaction");

    let create_quorum_receipt = get_receipt(create_quorum_call).await;
    match create_quorum_receipt {
        Ok(receipt) => {
            info!("Quorum created with receipt: {:?}", receipt);
            if !receipt.status() {
                error!("Failed to create quorum: {:?}", receipt);
                panic!("Failed to create quorum: {:?}", receipt);
            } else {
                info!(
                    "Quorum created with transaction hash: {:?}",
                    receipt.transaction_hash
                );
            }
        }
        Err(e) => {
            error!("Failed to create quorum: {}", e);
            panic!("Failed to create quorum: {}", e);
        }
    }

    // let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
    //     get_test_logger(),
    //     env.http_rpc_endpoint.clone(),
    //     private_key,
    //     registry_coordinator_address,
    //     operator_state_retriever_address,
    // )
    // .await
    // .unwrap();

    // avs_writer.

    // Spawn Task Spawner and Task Response Listener
    let successful_responses = Arc::new(Mutex::new(0));
    let successful_responses_clone = successful_responses.clone();

    // Create task response listener
    let response_listener = setup_task_response_listener(
        ws_endpoint,
        task_manager_address,
        successful_responses.clone(),
    );

    // Create task spawner
    let task_spawner = setup_task_spawner(
        http_endpoint.clone(),
        registry_coordinator_address,
        task_generator_address,
        accounts,
        task_manager_address,
    );

    tokio::spawn(async move {
        task_spawner.await;
    });
    tokio::spawn(async move {
        response_listener.await;
    });

    info!("Starting Blueprint Execution...");
    // Create aggregator client context
    let server_address = format!("{}:{}", "127.0.0.1", 8081);
    let eigen_client_context = EigenSquareContext {
        client: AggregatorClient::new(&server_address).unwrap(),
        std_config: env.clone(),
    };

    // Create the aggregator context
    let aggregator_context =
        AggregatorContext::new(server_address, task_manager_address, wallet, env.clone())
            .await
            .unwrap();
    let aggregator_context_clone = aggregator_context.clone();

    // Create the combined context for both tasks
    let combined_context = CombinedContext::new(
        eigen_client_context,
        Some(aggregator_context.clone()),
        env.clone(),
    );

    let current_block_number = provider.get_block_number().await.unwrap();

    // Create task producer
    let client = Arc::new(provider);
    let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig {
            poll_interval: Duration::from_secs(1),
            start_block: current_block_number,
            confirmations: 1,
            step: 1,
        },
    );

    info!("Setting up Blueprint Runner...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default())
        .with_exit_after_register(exit_after_registration);

    info!("Created Eigenlayer BLS config");

    // Create and run the blueprint runner
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let runner_handle = tokio::spawn(async move {
        let result = BlueprintRunner::builder(eigen_config, env.clone())
            .router(
                Router::new()
                    .always(xsquare_eigen)
                    .always(initialize_bls_task)
                    .with_context(combined_context),
            )
            .producer(task_producer)
            .background_service(aggregator_context)
            .with_shutdown_handler(async {
                info!("Shutting down task manager service");
            })
            .run()
            .await;

        let _ = shutdown_tx.send(result);
    });

    info!("Built Blueprint Runner");

    // Wait for the process to complete or timeout
    let timeout_duration = Duration::from_secs(300);
    info!("Waiting for responses...");
    let result = wait_for_responses(
        successful_responses.clone(),
        expected_responses,
        timeout_duration,
    )
    .await;

    info!("Responses found, shutting down...");

    // Start the shutdown/cleanup process
    aggregator_context_clone.shutdown().await;

    // Abort the runner
    runner_handle.abort();

    // Clean up the ./db directory
    let _ = std::fs::remove_dir_all("./db");

    match result {
        Ok(Ok(())) => {
            info!("Test completed successfully with {expected_responses} tasks responded to.");
        }
        _ => {
            panic!(
                "Test failed with {} successful responses out of {} required",
                successful_responses_clone.lock().unwrap(),
                expected_responses
            );
        }
    }
}

pub struct SquaringAvsContracts {
    pub task_manager_address: Address,
    pub service_manager_address: Address,
}

pub fn setup_task_spawner(
    http_endpoint: String,
    registry_coordinator_address: Address,
    task_generator_address: Address,
    accounts: Vec<Address>,
    task_manager_address: Address,
) -> impl std::future::Future<Output = ()> {
    setup_log();
    info!("Setting up task spawner...");
    let provider = get_provider_http(&http_endpoint);
    let task_manager = SquaringTask::new(task_manager_address, provider.clone());
    let registry_coordinator =
        RegistryCoordinator::new(registry_coordinator_address, provider.clone());

    let operators = vec![vec![accounts[0]]];
    let quorums = Bytes::from(vec![0]);
    async move {
        loop {
            // Delay to allow for proper task initialization
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            info!("Creating a new task...");
            if get_receipt(
                task_manager
                    .createSquaringTask(U256::from(2), 100u32, quorums.clone())
                    .from(task_generator_address),
            )
            .await
            .unwrap()
            .status()
            {
                info!("Created a new task...");
            }

            if get_receipt(
                registry_coordinator.updateOperatorsForQuorum(operators.clone(), quorums.clone()),
            )
            .await
            .unwrap()
            .status()
            {
                info!("Updated operators for quorum...");
            }

            // Wait for task initialization to complete
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "cast rpc anvil_mine 1 --rpc-url {} > /dev/null",
                    http_endpoint
                ))
                .output()
                .await
                .unwrap();
            info!("Mined a block...");
        }
    }
}

pub fn setup_task_response_listener(
    ws_endpoint: String,
    task_manager_address: Address,
    successful_responses: Arc<Mutex<usize>>,
) -> impl std::future::Future<Output = ()> {
    async move {
        setup_log();
        let task_manager =
            SquaringTask::new(task_manager_address, get_provider_ws(&ws_endpoint).await);
        info!("Setting up task response listener...");
        let filter = task_manager.TaskResponded_filter().filter;
        let mut event_stream = match task_manager.provider().subscribe_logs(&filter).await {
            Ok(stream) => stream.into_stream(),
            Err(e) => {
                error!("Failed to subscribe to logs: {:?}", e);
                return;
            }
        };
        while let Some(event) = event_stream.next().await {
            let SquaringTask::TaskResponded {
                taskResponse: _, ..
            } = event
                .log_decode::<SquaringTask::TaskResponded>()
                .unwrap()
                .inner
                .data;
            let mut counter = match successful_responses.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    error!("Failed to lock successful_responses: {}", e);
                    return;
                }
            };
            *counter += 1;
        }
    }
}
