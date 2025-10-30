#![allow(clippy::result_large_err)]

use std::env;
use std::sync::Arc;
use std::time::Duration;

use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes, U256};
use alloy_signer_local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::keystore::backends::Backend;
use blueprint_sdk::keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_sdk::keystore::crypto::k256::K256Ecdsa;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::{Router, error, info};
use eigenlayer_contract_deployer::bindings::RegistryCoordinator;
use incredible_squaring_blueprint_eigenlayer::SquaringTask;
use incredible_squaring_blueprint_eigenlayer::TASK_MANAGER_ADDRESS;
use incredible_squaring_blueprint_eigenlayer::contexts::aggregator::AggregatorContext;
use incredible_squaring_blueprint_eigenlayer::contexts::client::AggregatorClient;
use incredible_squaring_blueprint_eigenlayer::contexts::combined::CombinedContext;
use incredible_squaring_blueprint_eigenlayer::contexts::x_square::EigenSquareContext;
#[allow(unused_imports)]
use incredible_squaring_blueprint_eigenlayer::jobs::compute_x_square::{
    XSQUARE_JOB_ID, xsquare_eigen,
};
#[allow(unused_imports)]
use incredible_squaring_blueprint_eigenlayer::jobs::initialize_task::{
    INITIALIZE_TASK_JOB_ID, initialize_bls_task,
};
use incredible_squaring_blueprint_eigenlayer::{
    AGGREGATOR_PRIVATE_KEY, TASK_GENERATOR_PRIVATE_KEY,
};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();
    let env = BlueprintEnvironment::load()?;

    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet Aggregator");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());

    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8081".to_string());
    let server_address = format!("{server_host}:{server_port}");
    let eigen_client_context = EigenSquareContext {
        client: AggregatorClient::new(&server_address)
            .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?,
        std_config: env.clone(),
    };

    // Create the aggregator context
    let aggregator_context = AggregatorContext::new(
        server_address,
        *TASK_MANAGER_ADDRESS,
        wallet.clone(),
        env.clone(),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    // Create the combined context for both tasks
    let combined_context = CombinedContext::new(
        eigen_client_context,
        Some(aggregator_context.clone()),
        env.clone(),
    );

    let client = Arc::new(provider);
    // Create producer for task events
    let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig::default().poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    if env.test_mode {
        // Create task spawner with graceful shutdown
        let ecdsa_public = env.keystore().first_local::<K256Ecdsa>()?;
        let ecdsa_secret = env
            .keystore()
            .expose_ecdsa_secret(&ecdsa_public)?
            .expect("No ECDSA secret found");
        let operator_address = ecdsa_secret
            .alloy_address()
            .expect("Failed to get operator address");
        let task_spawner = setup_task_spawner(
            env.protocol_settings
                .eigenlayer()?
                .registry_coordinator_address,
            env.http_rpc_endpoint.clone(),
            operator_address,
        );
        tokio::spawn(async move {
            task_spawner.await;
        });
    }

    info!("~~~ Executing the incredible squaring blueprint ~~~");

    // Configure EigenLayer operator addresses:
    // - delegation_approver: Address::ZERO is acceptable for test/development (matches eigensdk-rs test patterns)
    //   For production, use operator wallet address to enable delegation approval
    // - earnings_receiver: Deprecated in eigensdk-rs v2.0.0, internally set to None
    let earnings_receiver_address = Address::ZERO; // Ignored - deprecated field
    let delegation_approver_address = Address::ZERO; // For tests; use wallet.address() in production

    let eigen_config =
        EigenlayerBLSConfig::new(earnings_receiver_address, delegation_approver_address)
            .with_exit_after_register(false);

    BlueprintRunner::builder(eigen_config, env)
        .router(
            Router::new()
                // @dev Due to topic0 of event `emit NewTaskCreated(latestTaskNum, newTask);`
                // in  `examples/incredible-squaring-eigenlayer/contracts/src/TaskManager.sol:153`
                // is a sequence number, we need to use `always` to force handle all the tasks.
                .always(xsquare_eigen)
                .always(initialize_bls_task)
                // @dev
                // If the topic0 of event is a directional number
                // we can use `route` to handle the task by specified `id`.
                // .route(XSQUARE_JOB_ID, xsquare_eigen)
                // .route(INITIALIZE_TASK_JOB_ID, initialize_bls_task)
                .with_context(combined_context),
        )
        .producer(task_producer)
        .background_service(aggregator_context)
        .with_shutdown_handler(async {
            blueprint_sdk::info!("Shutting down task manager service");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}

fn setup_log() {
    let filter = tracing_subscriber::EnvFilter::new("trace,info");
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}

pub fn setup_task_spawner(
    registry_coordinator_addr: Address,
    http_endpoint: reqwest::Url,
    operator_addr: Address,
) -> impl std::future::Future<Output = ()> {
    let task_manager_address = *TASK_MANAGER_ADDRESS;
    let task_generator_signer = TASK_GENERATOR_PRIVATE_KEY
        .parse::<PrivateKeySigner>()
        .expect("failed to generate task generator wallet");
    let task_generator_address = task_generator_signer.address();
    let task_generator_wallet = EthereumWallet::from(task_generator_signer);
    let provider = get_wallet_provider_http(http_endpoint.clone(), task_generator_wallet);
    let task_manager = SquaringTask::new(task_manager_address, provider.clone());
    let registry_coordinator = RegistryCoordinator::new(registry_coordinator_addr, provider);
    info!("Operator address: {}", operator_addr);
    let operators = vec![vec![operator_addr]];
    let quorums = Bytes::from(vec![0]);

    info!(
        "Setting up task spawner for task manager: {} using task generator: {}",
        task_manager_address, task_generator_address
    );

    async move {
        loop {
            // Delay to allow for proper task initialization
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            info!("Creating a new task...");
            match task_manager
                .createSquaringTask(U256::from(2), 100u32, quorums.clone())
                .from(task_generator_address)
                .send()
                .await
                .expect("Failed to send task creation transaction")
                .get_receipt()
                .await
            {
                Ok(receipt) => {
                    info!("Created a new task!");
                    if !receipt.status() {
                        error!("Failed to create a new task: {:?}", receipt);
                    }
                }
                Err(e) => {
                    error!("Failed to create a new task: {:?}", e);
                }
            }

            match registry_coordinator
                .updateOperatorsForQuorum(operators.clone(), quorums.clone())
                .from(task_generator_address)
                .send()
                .await
                .expect("Failed to send operator update transaction")
                .get_receipt()
                .await
            {
                Ok(receipt) => {
                    info!("Updated operators for quorum!");
                    if !receipt.status() {
                        error!("Failed to update operators for quorum: {:?}", receipt);
                    }
                }
                Err(e) => {
                    error!("Failed to update operators for quorum: {:?}", e);
                }
            }

            // Wait for task initialization to complete
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "cast rpc anvil_mine 1 --rpc-url {http_endpoint} > /dev/null",
                ))
                .output()
                .await
                .expect("Failed to mine block");
            info!("Mined a block...");
        }
    }
}
