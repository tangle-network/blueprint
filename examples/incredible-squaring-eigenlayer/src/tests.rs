use crate::constants::AGGREGATOR_PRIVATE_KEY;
use crate::contexts::aggregator::AggregatorContext;
use crate::contexts::client::AggregatorClient;
use crate::contexts::combined::CombinedContext;
use crate::contexts::x_square::EigenSquareContext;
use crate::contracts::{IServiceManager, ProxyAdmin, SquaringServiceManager, SquaringTask};
use crate::jobs::compute_x_square::{XSQUARE_JOB_ID, xsquare_eigen};
use crate::jobs::initialize_task::{INITIALIZE_TASK_JOB_ID, initialize_bls_task};
use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes, FixedBytes, U256, address};
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
use blueprint_sdk::{Router, debug, error, info, warn};
use color_eyre::eyre;
use eigensdk::utils::slashing::core::allocationmanager::IAllocationManagerTypes::SlashingParams;
use eigensdk::utils::slashing::core::avsdirectory::AVSDirectory;
use eigensdk::utils::slashing::middleware::registrycoordinator::IRegistryCoordinatorTypes::{
    RegistryCoordinatorParams, SlashingRegistryParams,
};
use eigensdk::utils::slashing::middleware::registrycoordinator::RegistryCoordinator;
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

    // Deploy Task Manager
    let avs_contracts = deploy_avs_contracts(&harness).await;
    let task_manager_address = avs_contracts.task_manager_address;
    let service_manager_address = avs_contracts.service_manager_address;

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

pub struct SquaringAvsContracts {
    pub task_manager_address: Address,
    pub service_manager_address: Address,
}

// pub async fn deploy_avs_contracts_with_proxy<Ctx>(harness: &EigenlayerTestHarness<Ctx>) -> SquaringAvsContracts
// where
//     Ctx: Clone + Send + Sync + 'static,
// {
//     use alloy_primitives::U256;
//     use alloy_primitives::bytes::Bytes;

//     let env = harness.env().clone();
//     let http_endpoint = &env.http_rpc_endpoint;
//     let contract_addresses = harness.eigenlayer_contract_addresses;
//     let avs_directory_address = contract_addresses.avs_directory_address;
//     let registry_coordinator_address = contract_addresses.registry_coordinator_address;
//     let rewards_coordinator_address = contract_addresses.rewards_coordinator_address;
//     let stake_registry_address = contract_addresses.stake_registry_address;

//     let owner_address = harness.owner_account();
//     let aggregator_address = harness.aggregator_account();
//     let task_generator_address = harness.task_generator_account();

//     let provider = get_provider_http(http_endpoint);

//     // Step 1: Deploy ProxyAdmin
//     info!("Deploying ProxyAdmin");
//     let proxy_admin_deploy_call = ProxyAdmin::deploy_builder(
//         provider.clone(),
//         owner_address, // owner
//     );

//     let proxy_admin_address = match get_receipt(proxy_admin_deploy_call).await {
//         Ok(receipt) => match receipt.contract_address {
//             Some(address) => address,
//             None => {
//                 error!("Failed to get contract address from receipt");
//                 panic!("Failed to get contract address from receipt");
//             }
//         },
//         Err(e) => {
//             error!("Failed to get receipt: {:?}", e);
//             panic!("Failed to get contract address from receipt");
//         }
//     };
//     info!("Deployed ProxyAdmin at {}", proxy_admin_address);

//     // Step 2: Deploy SquaringTask (TaskManager) implementation
//     info!("Deploying Incredible Squaring Task Manager implementation");
//     let task_manager_deploy_call =
//         SquaringTask::deploy_builder(provider.clone(), registry_coordinator_address, 10u32);

//     let task_manager_impl_address = match get_receipt(task_manager_deploy_call).await {
//         Ok(receipt) => match receipt.contract_address {
//             Some(address) => address,
//             None => {
//                 error!("Failed to get contract address from receipt");
//                 panic!("Failed to get contract address from receipt");
//             }
//         },
//         Err(e) => {
//             error!("Failed to get receipt: {:?}", e);
//             panic!("Failed to get contract address from receipt");
//         }
//     };
//     info!(
//         "Deployed Incredible Squaring Task Manager implementation at {}",
//         task_manager_impl_address
//     );

//     // Step 3: Create initialization data for SquaringTask
//     let task_manager_init_data = SquaringTask::initialize_call_builder(
//         aggregator_address,
//         task_generator_address,
//         owner_address
//     ).calldata().unwrap();

//     // Step 4: Deploy TransparentUpgradeableProxy for SquaringTask
//     info!("Deploying TransparentUpgradeableProxy for SquaringTask");
//     let task_manager_proxy_deploy_call = TransparentUpgradeableProxy::deploy_builder(
//         provider.clone(),
//         task_manager_impl_address, // logic implementation address
//         proxy_admin_address,       // admin address
//         Bytes::from(task_manager_init_data), // initialization data
//     );

//     let task_manager_address = match get_receipt(task_manager_proxy_deploy_call).await {
//         Ok(receipt) => match receipt.contract_address {
//             Some(address) => address,
//             None => {
//                 error!("Failed to get contract address from receipt");
//                 panic!("Failed to get contract address from receipt");
//             }
//         },
//         Err(e) => {
//             error!("Failed to get receipt: {:?}", e);
//             panic!("Failed to get contract address from receipt");
//         }
//     };
//     info!(
//         "Deployed TransparentUpgradeableProxy for SquaringTask at {}",
//         task_manager_address
//     );

//     // Step 5: Deploy SquaringServiceManager implementation
//     info!("Deploying Incredible Squaring Service Manager implementation");
//     let service_manager_deploy_call = SquaringServiceManager::deploy_builder(
//         provider.clone(),
//         avs_directory_address,
//         registry_coordinator_address,
//         stake_registry_address,
//         rewards_coordinator_address,
//         task_manager_address,  // Using the proxy address here
//     );

//     let service_manager_impl_address = match get_receipt(service_manager_deploy_call).await {
//         Ok(receipt) => match receipt.contract_address {
//             Some(address) => address,
//             None => {
//                 error!("Failed to get contract address from receipt");
//                 panic!("Failed to get contract address from receipt");
//             }
//         },
//         Err(e) => {
//             error!("Failed to get receipt: {:?}", e);
//             panic!("Failed to get contract address from receipt");
//         }
//     };
//     info!(
//         "Deployed Incredible Squaring Service Manager implementation at {}",
//         service_manager_impl_address
//     );

//     // Step 6: Create initialization data for SquaringServiceManager
//     // Now we need to create initialization data since we're no longer initializing in the constructor
//     let service_manager_init_data = SquaringServiceManager::initializeCall {
//         initialOwner: owner_address,
//         rewardsInitiator: owner_address
//     };

//     // Step 7: Deploy TransparentUpgradeableProxy for SquaringServiceManager
//     info!("Deploying TransparentUpgradeableProxy for SquaringServiceManager");
//     let service_manager_proxy_deploy_call = TransparentUpgradeableProxy::deploy_builder(
//         provider.clone(),
//         service_manager_impl_address, // logic implementation address
//         proxy_admin_address,          // admin address
//         service_manager_init_data,    // initialization data with owner and rewards initiator
//     );

//     let service_manager_address = match get_receipt(service_manager_proxy_deploy_call).await {
//         Ok(receipt) => match receipt.contract_address {
//             Some(address) => address,
//             None => {
//                 error!("Failed to get contract address from receipt");
//                 panic!("Failed to get contract address from receipt");
//             }
//         },
//         Err(e) => {
//             error!("Failed to get receipt: {:?}", e);
//             panic!("Failed to get contract address from receipt");
//         }
//     };
//     info!(
//         "Deployed TransparentUpgradeableProxy for SquaringServiceManager at {}",
//         service_manager_address
//     );

//     // Step 8: Verify deployment
//     info!("Verifying deployment");

//     // Verify SquaringTask through proxy
//     let task_manager = SquaringTask::new(task_manager_address, provider.clone());
//     let task_generator = task_manager.generator().call().await.unwrap();
//     let aggregator = task_manager.aggregator().call().await.unwrap();

//     info!("SquaringTask task_generator: {:?}", task_generator._0);
//     info!("SquaringTask aggregator: {:?}", aggregator._0);

//     assert_eq!(task_generator._0, task_generator_address, "Task generator address mismatch");
//     assert_eq!(aggregator._0, aggregator_address, "Aggregator address mismatch");

//     // Verify SquaringServiceManager through proxy
//     let service_manager = SquaringServiceManager::new(service_manager_address, provider.clone());
//     let task_manager_from_service = service_manager.squaringTaskManager().call().await.unwrap();

//     info!("SquaringServiceManager task_manager: {:?}", task_manager_from_service._0);

//     assert_eq!(task_manager_from_service._0, task_manager_address, "Task manager address mismatch");

//     SquaringAvsContracts {
//         task_manager_address,
//         service_manager_address,
//     }
// }

pub async fn deploy_avs_contracts<Ctx>(harness: &EigenlayerTestHarness<Ctx>) -> SquaringAvsContracts
where
    Ctx: Clone + Send + Sync + 'static,
{
    let env = harness.env().clone();
    let http_endpoint = &env.http_rpc_endpoint;
    let contract_addresses = harness.eigenlayer_contract_addresses;
    // let avs_directory_address = contract_addresses.avs_directory_address;
    let registry_coordinator_address = contract_addresses.registry_coordinator_address;
    // let rewards_coordinator_address = contract_addresses.rewards_coordinator_address;
    // let stake_registry_address = contract_addresses.stake_registry_address;
    let delegation_manager_address = contract_addresses.delegation_manager_address;

    let owner_address = harness.owner_account();
    let aggregator_address = harness.aggregator_account();
    let task_generator_address = harness.task_generator_account();

    let provider = get_provider_http(http_endpoint);

    info!("Deploying new AVS Directory");
    let pauser_registry_address = address!("9a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae");
    let deploy_call = AVSDirectory::deploy_builder(
        provider.clone(),
        delegation_manager_address,
        pauser_registry_address,
        "0.1".into(),
    );
    let avs_directory_address = match get_receipt(deploy_call).await {
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
    info!("Deployed new AVS Directory at {}", avs_directory_address);

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

    let proxy_admin_address = address!("5eb3bc0a489c5a8288765d2336659ebca68fcd00");
    let existing_service_manager_proxy = address!("b7278a61aa25c888815afc32ad3cc52ff24fe575");
    let service_manager_address = upgrade_service_manager(
        harness,
        existing_service_manager_proxy,
        task_manager_address,
        proxy_admin_address,
    )
    .await;

    SquaringAvsContracts {
        task_manager_address,
        service_manager_address,
    }
}

pub async fn upgrade_service_manager<Ctx>(
    harness: &EigenlayerTestHarness<Ctx>,
    existing_service_manager_proxy: Address,
    task_manager_address: Address,
    proxy_admin_address: Address,
) -> Address
where
    Ctx: Clone + Send + Sync + 'static,
{
    let env = harness.env().clone();
    let http_endpoint = &env.http_rpc_endpoint;
    let contract_addresses = harness.eigenlayer_contract_addresses;
    let avs_directory_address = contract_addresses.avs_directory_address;
    let registry_coordinator_address = contract_addresses.registry_coordinator_address;
    let rewards_coordinator_address = contract_addresses.rewards_coordinator_address;
    let stake_registry_address = contract_addresses.stake_registry_address;
    let owner_address = harness.owner_account();
    let allocation_manager_address = contract_addresses.allocation_manager_address;
    let permission_controller_address = contract_addresses.permission_controller_address;

    let provider = get_provider_http(http_endpoint);

    let proxy_admin = ProxyAdmin::new(proxy_admin_address, provider.clone());

    info!("Deploying new SquaringServiceManager implementation");
    let service_manager_deploy_call = SquaringServiceManager::deploy_builder(
        provider.clone(),
        avs_directory_address,
        registry_coordinator_address,
        stake_registry_address,
        rewards_coordinator_address,
        task_manager_address,
        permission_controller_address,
        allocation_manager_address,
    );

    let service_manager_impl_address = match get_receipt(service_manager_deploy_call).await {
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
        "Deployed new SquaringServiceManager implementation at {}",
        service_manager_impl_address
    );

    let slashing_params = SlashingRegistryParams {
        allocationManager: allocation_manager_address,
        blsApkRegistry: address!("c351628eb244ec633d5f21fbd6621e1a683b1181"),
        indexRegistry: address!("cbeaf3bde82155f56486fb5a1072cb8baaf547cc"),
        socketRegistry: address!("82e01223d51eb87e16a03e24687edf0f294da6f1"),
        stakeRegistry: address!("82e01223d51eb87e16a03e24687edf0f294da6f1"),
        pauserRegistry: address!("9a9f2ccfde556a7e9ff0848998aa4a0cfd8863ae"),
    };
    let registry_coordinator_params = RegistryCoordinatorParams {
        serviceManager: existing_service_manager_proxy,
        slashingParams: slashing_params,
    };
    let new_registry_coordinator =
        RegistryCoordinator::deploy_builder(provider.clone(), registry_coordinator_params);
    let new_registry_coordinator_address = match get_receipt(new_registry_coordinator).await {
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

    info!("Upgrading Registry Coordinator to use new implementation");
    let upgrade_call = proxy_admin.upgrade(
        registry_coordinator_address,
        new_registry_coordinator_address,
    );

    let upgrade_receipt = get_receipt(upgrade_call).await.unwrap();
    assert!(upgrade_receipt.status(), "Upgrade transaction failed");

    info!(
        "Successfully upgraded RegistryCoordinator proxy at {} to use implementation at {}",
        registry_coordinator_address, new_registry_coordinator_address
    );

    let registry_coordinator =
        RegistryCoordinator::new(new_registry_coordinator_address, provider.clone());

    let init_call = registry_coordinator.initialize(
        owner_address,
        owner_address,
        owner_address,
        U256::from(0),
        existing_service_manager_proxy,
    );

    match get_receipt(init_call).await {
        Ok(receipt) => {
            if receipt.status() {
                info!("Successfully initialized upgraded RegistryCoordinator");
            } else {
                warn!("Failed to initialize RegistryCoordinator - it may already be initialized");
            }
        }
        Err(e) => {
            warn!(
                "Failed to initialize RegistryCoordinator - it may already be initialized: {:?}",
                e
            );
        }
    }

    let test_sm = registry_coordinator
        .serviceManager()
        .call()
        .await
        .unwrap()
        ._0;
    // registry_coordinator.
    assert_eq!(test_sm, existing_service_manager_proxy);

    info!("Upgrading Service Manager proxy to use new implementation");
    let upgrade_call =
        proxy_admin.upgrade(existing_service_manager_proxy, service_manager_impl_address);

    let upgrade_receipt = get_receipt(upgrade_call).await.unwrap();
    assert!(upgrade_receipt.status(), "Upgrade transaction failed");

    info!(
        "Successfully upgraded ServiceManager proxy at {} to use implementation at {}",
        existing_service_manager_proxy, service_manager_impl_address
    );

    let service_manager =
        SquaringServiceManager::new(existing_service_manager_proxy, provider.clone());

    let init_call = service_manager.initialize(owner_address, owner_address);

    match get_receipt(init_call).await {
        Ok(receipt) => {
            if receipt.status() {
                info!("Successfully initialized upgraded ServiceManager");
            } else {
                warn!("Failed to initialize ServiceManager - it may already be initialized");
            }
        }
        Err(e) => {
            warn!(
                "Failed to initialize ServiceManager - it may already be initialized: {:?}",
                e
            );
        }
    }

    let task_manager_from_service = service_manager.squaringTaskManager().call().await.unwrap();
    debug!(
        "SquaringServiceManager task_manager: {:?}",
        task_manager_from_service._0
    );

    let owner = service_manager.owner().call().await.unwrap();
    debug!("SquaringServiceManager owner: {:?}", owner._0);

    let rewards_initiator = service_manager.rewardsInitiator().call().await.unwrap();
    debug!(
        "SquaringServiceManager rewardsInitiator: {:?}",
        rewards_initiator._0
    );

    existing_service_manager_proxy
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
            continue;
            // Delay to allow for proper task initialization
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
