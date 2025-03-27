use crate::constants::{AGGREGATOR_PRIVATE_KEY, PRIVATE_KEY};
use crate::contexts::aggregator::AggregatorContext;
use crate::contexts::client::AggregatorClient;
use crate::contexts::combined::CombinedContext;
use crate::contexts::x_square::EigenSquareContext;
use crate::contracts::SquaringTask;
use crate::jobs::compute_x_square::xsquare_eigen;
use crate::jobs::initialize_task::initialize_bls_task;
use crate::tests::deploy::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use crate::tests::deploy::IStakeRegistryTypes::StrategyParams;
use crate::tests::deploy::SlashingRegistryCoordinator;
use alloy_network::EthereumWallet;
use alloy_primitives::aliases::U96;
use alloy_primitives::{Address, Bytes, U256};
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
use blueprint_sdk::testing::utils::eigenlayer::env::{PAUSER_REGISTRY_ADDR, STRATEGY_ADDR};
use blueprint_sdk::testing::utils::setup_log;
use blueprint_sdk::{Router, error, info, warn};
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
    let contract_addresses = harness.eigenlayer_contract_addresses;
    let delegation_manager_address = contract_addresses.delegation_manager_address;
    let permission_controller_address = contract_addresses.permission_controller_address;
    let allocation_manager_address = contract_addresses.allocation_manager_address;
    let avs_directory_addr = contract_addresses.avs_directory_address;
    let rewards_coordinator_addr = contract_addresses.rewards_coordinator_address;
    let pauser_registry_addr = PAUSER_REGISTRY_ADDR;
    let strategy_manager_addr = contract_addresses.strategy_manager_address;

    let avs_contracts = crate::tests::deploy::deploy_avs_contracts(
        &env.http_rpc_endpoint,
        &private_key,
        harness.owner_account(),
        1,
        vec![10],
        harness.owner_account(),
        permission_controller_address,
        allocation_manager_address,
        avs_directory_addr,
        delegation_manager_address,
        pauser_registry_addr,
        rewards_coordinator_addr,
        harness.task_generator_account(),
        harness.aggregator_account(),
        10,
    )
    .await
    .unwrap();

    info!("AVS Contracts deployed at: {:?}", avs_contracts);

    let service_manager_address = avs_contracts.squaring_service_manager;
    let task_manager_address = avs_contracts.squaring_task_manager;

    // Ensure we have the correct Service Manager Address
    let env_service_manager = harness
        .eigenlayer_contract_addresses
        .service_manager_address;
    assert_eq!(env_service_manager, service_manager_address);

    // Extract necessary data from harness before moving it
    let ws_endpoint = harness.ws_endpoint.to_string();
    let registry_coordinator_address = harness
        .eigenlayer_contract_addresses
        .registry_coordinator_address;
    let accounts = harness.accounts().to_vec();
    let task_generator_address = harness.task_generator_account();
    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ");
    warn!("Private key: {}", private_key);
    let signer_wallet = get_provider_from_signer(&private_key, &http_endpoint);
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(&http_endpoint, wallet.clone());

    let registry_coordinator =
        SlashingRegistryCoordinator::new(registry_coordinator_address, signer_wallet.clone());
    let registry_owner = registry_coordinator.owner().call().await.unwrap();
    error!("Registry owner: {}", registry_owner._0);
    // assert_eq!(registry_owner._0, harness.owner_account());

    // Check if the AVS is properly set
    let avs_address = registry_coordinator.avs().call().await.unwrap();
    error!("AVS address: {}", avs_address._0);

    // Check if the strategy is valid
    let strategy_addr = STRATEGY_ADDR;
    let strategy_registry = registry_coordinator.stakeRegistry().call().await.unwrap();
    error!("Stake Registry address: {}", strategy_registry._0);

    // Check if there's already a quorum
    let quorum_count = registry_coordinator.quorumCount().call().await.unwrap();
    error!("Current quorum count: {}", quorum_count._0);

    // Ensure the AVS is set correctly before creating the quorum
    if avs_address._0 == Address::ZERO {
        error!("AVS address is not set, setting it now");
        let set_avs_result = registry_coordinator
            .setAVS(service_manager_address)
            .send()
            .await;
        match set_avs_result {
            Ok(receipt) => {
                let tx_receipt = receipt.get_receipt().await.unwrap();
                error!("Set AVS transaction: {:?}", tx_receipt.transaction_hash);
            }
            Err(e) => error!("Failed to set AVS: {}", e),
        }
    }

    // Try with simpler parameters
    let operator_set_param = OperatorSetParam {
        maxOperatorCount: 10,
        kickBIPsOfOperatorStake: 150,
        kickBIPsOfTotalStake: 100,
    };
    let minimum_stake: U96 = U96::from(0);

    // Use a different strategy approach
    let strategy_params = vec![StrategyParams {
        strategy: strategy_addr,
        multiplier: U96::from(1),
    }];

    error!(
        "Attempting to create quorum with strategy: {}",
        strategy_addr
    );

    // Try to create the quorum with error handling
    let stake_quorum_result = registry_coordinator
        .createTotalDelegatedStakeQuorum(operator_set_param, minimum_stake, strategy_params)
        .send()
        .await;

    match stake_quorum_result {
        Ok(receipt) => {
            let tx_receipt = receipt.get_receipt().await.unwrap();
            info!(
                "Total Delegated Stake Quorum created: {:?}",
                tx_receipt.transaction_hash
            );
        }
        Err(e) => {
            error!("Failed to create quorum: {}", e);

            // If there's already a quorum, we can skip this step and continue
            if quorum_count._0 > 0 {
                info!("Quorum already exists, continuing with existing quorum");
            } else {
                panic!("Failed to create quorum and no existing quorum found");
            }
        }
    }

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

    // Create task producer
    let client = Arc::new(provider);
    let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig {
            poll_interval: Duration::from_secs(1),
            start_block: 235,
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
    _registry_coordinator_address: Address,
    task_generator_address: Address,
    _accounts: Vec<Address>,
    task_manager_address: Address,
) -> impl std::future::Future<Output = ()> {
    setup_log();
    info!("Setting up task spawner...");
    let provider = get_provider_http(&http_endpoint);
    let task_manager = SquaringTask::new(task_manager_address, provider.clone());

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
