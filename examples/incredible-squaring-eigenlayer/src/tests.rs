use std::str::FromStr;

use crate::constants::AGGREGATOR_PRIVATE_KEY;
use crate::contexts::aggregator::AggregatorContext;
use crate::contexts::client::AggregatorClient;
use crate::contexts::combined::CombinedContext;
use crate::contexts::x_square::EigenSquareContext;
use crate::contracts::SquaringTask;
use crate::jobs::compute_x_square::{XSQUARE_JOB_ID, xsquare_eigen};
use crate::jobs::initialize_task::{INITIALIZE_TASK_JOB_ID, initialize_bls_task};
use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use alloy_provider::Provider;
use alloy_signer_local::{LocalSigner, PrivateKeySigner};
use alloy_sol_types::sol;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_provider_http;
use blueprint_sdk::evm::util::get_provider_ws;
use blueprint_sdk::evm::util::get_wallet_provider_http;
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
use blueprint_sdk::{Router, error, info};
use color_eyre::eyre;
use futures::StreamExt;
use tokio::sync::oneshot;

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    #[derive(Debug)]
    RegistryCoordinator,
    "./contracts/out/RegistryCoordinator.sol/RegistryCoordinator.json"
);

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

    // Deploy Task Manager
    let task_manager_address = deploy_task_manager(&harness).await;

    // Extract necessary data from harness before moving it
    let ws_endpoint = harness.ws_endpoint.to_string();
    let registry_coordinator_address = harness
        .eigenlayer_contract_addresses
        .registry_coordinator_address;
    let accounts = harness.accounts().to_vec();
    let task_generator_address = harness.task_generator_account();

    // let total_delegated_quorum_create_tx_hash = create_total_delegated_stake_quorum(
    //     erc20_mock_strategy_address,
    //     registry_coordinator_address,
    //     operator_pvt_key,
    //     ecdsa_keystore_path,
    //     ecdsa_keystore_password,
    //     &rpc_url,
    // )
    // .await.unwrap();

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
    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(&http_endpoint, wallet.clone());

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
            ..Default::default()
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
                    .route(XSQUARE_JOB_ID, xsquare_eigen)
                    .route(INITIALIZE_TASK_JOB_ID, initialize_bls_task)
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

pub async fn deploy_task_manager<Ctx>(harness: &EigenlayerTestHarness<Ctx>) -> Address
where
    Ctx: Clone + Send + Sync + 'static,
{
    let env = harness.env().clone();
    let http_endpoint = &env.http_rpc_endpoint;
    let registry_coordinator_address = harness
        .eigenlayer_contract_addresses
        .registry_coordinator_address;
    let owner_address = harness.owner_account();
    let aggregator_address = harness.aggregator_account();
    let task_generator_address = harness.task_generator_account();

    let provider = get_provider_http(http_endpoint);
    let deploy_call =
        SquaringTask::deploy_builder(provider.clone(), registry_coordinator_address, 10u32);
    info!("Deploying Incredible Squaring Task Manager");
    let task_manager_address = match get_receipt(deploy_call).await {
        Ok(receipt) => match receipt.contract_address {
            Some(address) => address,
            None => {
                error!("Failed to get contract address from receipt");
                panic!("Failed to get contract address from receipt");
            }
        },
        Err(e) => {
            error!("Failed to get receipt: {:?}", e);
            panic!("Failed to get contract address from receipt");
        }
    };
    info!(
        "Deployed Incredible Squaring Task Manager at {}",
        task_manager_address
    );
    // std::env::set_var("TASK_MANAGER_ADDRESS", task_manager_address.to_string());

    let task_manager = SquaringTask::new(task_manager_address, provider.clone());
    // Initialize the Incredible Squaring Task Manager
    info!("Initializing Incredible Squaring Task Manager");
    let init_call =
        task_manager.initialize(aggregator_address, task_generator_address, owner_address);
    let init_receipt = get_receipt(init_call).await.unwrap();
    assert!(init_receipt.status());
    info!("Initialized Incredible Squaring Task Manager");

    task_manager_address
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
            // Increased delay to allow for proper task initialization
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

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

// /// Creates Total Delegated stake
// #[allow(clippy::too_many_arguments)]
// pub async fn create_total_delegated_stake_quorum(
//     strategy_address: Address,
//     registry_coordinator_address: Address,
//     operator_pvt_key: Option<String>,
//     ecdsa_keystore_path: String,
//     ecdsa_keystore_password: String,
//     rpc_url: &str,
// ) -> eyre::Result<FixedBytes<32>> {
//     let signer;
//     if let Some(operator_key) = operator_pvt_key {
//         signer = PrivateKeySigner::from_str(&operator_key)?;
//     } else {
//         signer = LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?;
//     }
//     let s = signer.to_field_bytes();
//     let pvt_key = hex::encode(s).to_string();

//     let registry_coordinator_instance =
//         RegistryCoordinator::new(registry_coordinator_address, get_signer(&pvt_key, rpc_url));

//     let operator_set_param = OperatorSetParam {
//         maxOperatorCount: 3,
//         kickBIPsOfOperatorStake: 100,
//         kickBIPsOfTotalStake: 1000,
//     };
//     let minimum_stake: U96 = U96::from(0);
//     let strategy_params = vec![StrategyParams {
//         strategy: strategy_address,
//         multiplier: U96::from(1),
//     }];
//     let s = registry_coordinator_instance
//         .createTotalDelegatedStakeQuorum(operator_set_param, minimum_stake, strategy_params)
//         .send()
//         .await
//         .unwrap()
//         .get_receipt()
//         .await
//         .unwrap()
//         .transaction_hash;
//     Ok(s)
// }

// /// Deposit into strategy
// ///
// /// # Arguments
// ///
// /// * `strategy_address` - The address of the strategy
// /// * `amount` - The amount to deposit
// /// * `el_reader` - The EL chain reader
// /// * `el_writer` - The EL chain writer
// pub async fn deposit_into_strategy(
//     strategy_address: Address,
//     amount: U256,
//     el_writer: ELChainWriter,
// ) -> Result<(), ElContractsError> {
//     el_writer
//         .deposit_erc20_into_strategy(strategy_address, amount)
//         .await?;
//     Ok(())
// }
