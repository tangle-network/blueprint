//! Incredible Squaring EigenLayer Service
//! 
//! This service monitors an EigenLayer TaskManager contract for squaring tasks,
//! processes them, and submits responses back to the chain. It demonstrates how to:
//! 
//! - Set up and configure an EigenLayer BLS service
//! - Monitor smart contract events using a polling producer
//! - Handle task processing and response submission
//! - Implement proper error handling and logging
//! - Manage graceful shutdown

use ::std::{str::FromStr, sync::Arc, time::Duration};
use blueprint_sdk::alloy::primitives::Address;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::runner::{config::BlueprintEnvironment, BlueprintRunner};
use blueprint_sdk::evm::util::get_provider_http;
use blueprint_sdk::*;
use incredible_squaring_eigenlayer_lib::contexts::ExampleContext;
// use incredible_squaring_eigenlayer_lib::{create_contract_router, ExampleContext};
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;

/// The main entry point for the Incredible Squaring EigenLayer service.
/// 
/// # Environment Variables
/// 
/// - `TASK_MANAGER_ADDRESS`: The address of the deployed TaskManager contract
/// - `RPC_URL`: URL of the Ethereum RPC endpoint to connect to
/// 
/// # Error Handling
/// 
/// Returns a Result that propagates any errors encountered during setup or execution.
#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    // Initialize structured logging
    setup_log();
    info!("Starting Incredible Squaring EigenLayer service");

    // Load and validate required environment variables
    let task_manager = match std::env::var("TASK_MANAGER_ADDRESS")
        .map(|addr| Address::from_str(&addr)) {
        Ok(Ok(address)) => address,
        _ => {
            warn!("TASK_MANAGER_ADDRESS environment variable not set or invalid");
            return Err(blueprint_sdk::Error::Custom("Missing or invalid TASK_MANAGER_ADDRESS".into()));
        }
    };

    let rpc_url = std::env::var("RPC_URL")
        .map_err(|_| blueprint_sdk::Error::Custom("RPC_URL environment variable not set".into()))?;

    // Initialize Ethereum RPC client
    let client = Arc::new(get_provider_http(&rpc_url));
    info!("Connected to Ethereum node at {}", rpc_url);

    // Configure event monitoring
    let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig {
            poll_interval: Duration::from_secs(1),
            ..Default::default()
        },
    );
    info!("Configured task event monitoring");

    // Initialize EigenLayer BLS configuration
    // TODO: Make these addresses configurable via environment variables
    let eigenlayer_bls_config = EigenlayerBLSConfig::new(
        Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
        Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
    );

    // Create service context and start the runner
    let ctx = ExampleContext {};
    info!("Starting BlueprintRunner with TaskManager at {}", task_manager);
    
    BlueprintRunner::builder(eigenlayer_bls_config, BlueprintEnvironment::default())
        .router(create_contract_router(ctx, task_manager))
        .producer(task_producer)
        .with_shutdown_handler(async {
            info!("Initiating graceful shutdown of task manager service");
            // Add any cleanup logic here
        })
        .run()
        .await?;

    info!("Service terminated successfully");
    Ok(())
}

/// Configures structured logging with appropriate filters and formatting.
pub fn setup_log() {
    use tracing_subscriber::util::SubscriberInitExt;

    let _ = tracing_subscriber::fmt::SubscriberBuilder::default()
        .without_time()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE)
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish()
        .try_init();
}
