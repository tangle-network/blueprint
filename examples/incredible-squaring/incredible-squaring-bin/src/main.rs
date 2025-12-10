//! Incredible Squaring Blueprint Binary
//!
//! This is the main entry point for the incredible squaring blueprint.
//! It sets up the EVM-based producer/consumer pattern for Tangle v2.
//!
//! ## Jobs
//!
//! This blueprint provides three job variants with different aggregation requirements:
//!
//! - **Job 0 (square)**: Basic squaring, requires 1 operator result
//! - **Job 1 (verified_square)**: Requires 2 operator results for redundancy
//! - **Job 2 (consensus_square)**: Requires 3 operator results for Byzantine fault tolerance

use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::tangle_evm::config::TangleEvmConfig;
use blueprint_sdk::tangle_evm::{TangleEvmConsumer, TangleEvmProducer};
use blueprint_sdk::{error, info};
use incredible_squaring_blueprint_lib::{
    CONSENSUS_XSQUARE_JOB_ID, FooBackgroundService, VERIFIED_XSQUARE_JOB_ID, XSQUARE_JOB_ID, router,
};

/// Initialize logging
fn setup_log() {
    use tracing_subscriber::{EnvFilter, fmt};
    let filter = EnvFilter::from_default_env();
    fmt().with_env_filter(filter).init();
}

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    info!("Starting the incredible squaring blueprint (EVM v2)!");

    // Load the blueprint environment
    let env = BlueprintEnvironment::load()?;

    // Get Tangle EVM client from context
    let tangle_client = env
        .tangle_evm_client()
        .await
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    // Get service ID from protocol settings
    let service_id = env
        .protocol_settings
        .tangle_evm()
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?
        .service_id
        .ok_or_else(|| blueprint_sdk::Error::Other("No service ID configured".to_string()))?;

    // Create the EVM producer to listen for JobSubmitted events
    let tangle_producer = TangleEvmProducer::new(tangle_client.clone(), service_id);

    // Create the EVM consumer to submit results back to the contract
    let tangle_consumer = TangleEvmConsumer::new(tangle_client.clone());

    // Use the EVM config for the runner
    let tangle_config = TangleEvmConfig::default();

    info!("Connected to Tangle EVM. Service ID: {}", service_id);
    info!("Registered jobs:");
    info!("  - Job {}: square (1 operator required)", XSQUARE_JOB_ID);
    info!(
        "  - Job {}: verified_square (2 operators required)",
        VERIFIED_XSQUARE_JOB_ID
    );
    info!(
        "  - Job {}: consensus_square (3 operators required)",
        CONSENSUS_XSQUARE_JOB_ID
    );

    let result = BlueprintRunner::builder(tangle_config, env)
        .router(router())
        .background_service(FooBackgroundService)
        // Add the producer
        //
        // The TangleEvmProducer polls for JobSubmitted events from the Tangle Jobs contract
        // and converts them to JobCall streams for processing.
        .producer(tangle_producer)
        // Add the consumer
        //
        // The TangleEvmConsumer receives JobResults and submits them back to the
        // Tangle contract via the submitResult function.
        .consumer(tangle_consumer)
        // Custom shutdown handlers
        .with_shutdown_handler(async { println!("Shutting down!") })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed! {e:?}");
    }

    Ok(())
}
