#![allow(clippy::result_large_err)]

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
use incredible_squaring_blueprint_eigenlayer::AGGREGATOR_PRIVATE_KEY;
use incredible_squaring_blueprint_eigenlayer::TASK_MANAGER_ADDRESS;
use incredible_squaring_blueprint_eigenlayer::contexts::aggregator::AggregatorContext;
use incredible_squaring_blueprint_eigenlayer::contexts::client::AggregatorClient;
use incredible_squaring_blueprint_eigenlayer::contexts::combined::CombinedContext;
use incredible_squaring_blueprint_eigenlayer::contexts::x_square::EigenSquareContext;
use incredible_squaring_blueprint_eigenlayer::jobs::compute_x_square::{
    XSQUARE_JOB_ID, xsquare_eigen,
};
use incredible_squaring_blueprint_eigenlayer::jobs::initialize_task::{
    INITIALIZE_TASK_JOB_ID, initialize_bls_task,
};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    let env = BlueprintEnvironment::load()?;

    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());

    let server_address = format!("{}:{}", "127.0.0.1", 8081);
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

    info!("~~~ Executing the incredible squaring blueprint ~~~");

    // Configure EigenLayer operator addresses:
    // - delegation_approver: Address::ZERO is acceptable for test/development (matches eigensdk-rs test patterns)
    //   For production, use operator wallet address to enable delegation approval
    // - earnings_receiver: Deprecated in eigensdk-rs v2.0.0, internally set to None
    let earnings_receiver_address = Address::ZERO; // Ignored - deprecated field
    let delegation_approver_address = Address::ZERO; // For tests; use wallet.address() in production

    let eigen_config =
        EigenlayerBLSConfig::new(earnings_receiver_address, delegation_approver_address);
    BlueprintRunner::builder(eigen_config, env)
        .router(
            Router::new()
                .route(XSQUARE_JOB_ID, xsquare_eigen)
                .route(INITIALIZE_TASK_JOB_ID, initialize_bls_task)
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
