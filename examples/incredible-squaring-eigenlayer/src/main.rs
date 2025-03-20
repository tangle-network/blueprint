use std::sync::Arc;
use std::time::Duration;

use alloy_network::EthereumWallet;
use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::{Router, info};
use incredible_squaring_blueprint_eigenlayer::constants::AGGREGATOR_PRIVATE_KEY;
use incredible_squaring_blueprint_eigenlayer::contexts::client::AggregatorClient;
use incredible_squaring_blueprint_eigenlayer::contexts::x_square::EigenSquareContext;
use incredible_squaring_blueprint_eigenlayer::jobs::compute_x_square::{
    XSQUARE_JOB_ID, xsquare_eigen,
};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    let env = BlueprintEnvironment::load()?;

    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(&env.http_rpc_endpoint, wallet.clone());

    let server_address = format!("{}:{}", "127.0.0.1", 8081);
    let eigen_client_context = EigenSquareContext {
        client: AggregatorClient::new(&server_address)
            .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?,
        std_config: env.clone(),
    };
    // let aggregator_context =
    //     AggregatorContext::new(server_address, *TASK_MANAGER_ADDRESS, wallet, env.clone())
    //         .await
    //         .unwrap();

    // let contract = IncredibleSquaringTaskManager::new(
    //     *TASK_MANAGER_ADDRESS,
    //     provider,
    // );

    let client = Arc::new(provider);
    // Create producer for task events
    let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig {
            poll_interval: Duration::from_secs(1),
            ..Default::default()
        },
    );

    // let initialize_task =
    //     InitializeBlsTaskEventHandler::new(contract.clone(), aggregator_context.clone());

    // let x_square_eigen = XsquareEigenEventHandler::new(contract.clone(), eigen_client_context);

    info!("~~~ Executing the incredible squaring blueprint ~~~");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::builder(eigen_config, BlueprintEnvironment::default())
        .router(
            Router::new()
                .route(XSQUARE_JOB_ID, xsquare_eigen)
                .with_context(eigen_client_context),
        )
        .producer(task_producer)
        .with_shutdown_handler(async {
            tracing::info!("Shutting down task manager service");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
