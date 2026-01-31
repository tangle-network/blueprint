//! Incredible Squaring Blueprint Binary
//!
//! This is the main entry point for the incredible squaring blueprint.
//! It sets up the EVM-based producer/consumer pattern for Tangle.
//!
//! ## Job Matrix (Execution × Aggregation)
//!
//! | Execution | Aggregation | Job ID | Function |
//! |-----------|-------------|--------|----------|
//! | Local     | Single (1)  | 0      | `square` |
//! | Local     | Multi (2)   | 1      | `verified_square` |
//! | Local     | Multi (3)   | 2      | `consensus_square` |
//! | FaaS      | Single (1)  | 3      | `square_faas` |
//! | FaaS      | Multi (2)   | 4      | `verified_square_faas` |

use blueprint_sdk::contexts::tangle::TangleClientContext;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::tangle::config::TangleConfig;
use blueprint_sdk::tangle::{TangleConsumer, TangleProducer};
use blueprint_sdk::{error, info};
use incredible_squaring_blueprint_lib::{
    CONSENSUS_XSQUARE_JOB_ID, FooBackgroundService, VERIFIED_XSQUARE_FAAS_JOB_ID,
    VERIFIED_XSQUARE_JOB_ID, XSQUARE_FAAS_JOB_ID, XSQUARE_JOB_ID, router,
};

/// Initialize logging
fn setup_log() {
    use tracing_subscriber::{EnvFilter, fmt};
    let filter = EnvFilter::from_default_env();
    fmt().with_env_filter(filter).init();
}

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    // Initialize logging - can be configured via RUST_LOG environment variable
    setup_log();

    info!("Starting the incredible squaring blueprint!");

    // Load the blueprint environment
    let env = BlueprintEnvironment::load()?;

    // Get Tangle client from context
    let tangle_client = env
        .tangle_client()
        .await
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    // Get service ID from protocol settings
    let service_id = env
        .protocol_settings
        .tangle()
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?
        .service_id
        .ok_or_else(|| blueprint_sdk::Error::Other("No service ID configured".to_string()))?;

    // Create the producer to listen for JobSubmitted events
    let tangle_producer = TangleProducer::new(tangle_client.clone(), service_id);

    // Create the consumer to submit results back to the contract
    let tangle_consumer = TangleConsumer::new(tangle_client.clone());

    // Use the Tangle config for the runner
    let tangle_config = TangleConfig::default();

    info!("Connected to Tangle. Service ID: {}", service_id);
    info!("Registered jobs (Execution × Aggregation matrix):");
    info!("  Local execution:");
    info!("    - Job {}: square (1 result)", XSQUARE_JOB_ID);
    info!(
        "    - Job {}: verified_square (2 results)",
        VERIFIED_XSQUARE_JOB_ID
    );
    info!(
        "    - Job {}: consensus_square (3 results)",
        CONSENSUS_XSQUARE_JOB_ID
    );
    info!("  FaaS execution:");
    info!("    - Job {}: square_faas (1 result)", XSQUARE_FAAS_JOB_ID);
    info!(
        "    - Job {}: verified_square_faas (2 results)",
        VERIFIED_XSQUARE_FAAS_JOB_ID
    );

    let result = BlueprintRunner::builder(tangle_config, env)
        .router(router())
        .background_service(FooBackgroundService)
        // Add the producer
        //
        // The TangleProducer polls for JobSubmitted events from the Tangle Jobs contract
        // and converts them to JobCall streams for processing.
        .producer(tangle_producer)
        // Add the consumer
        //
        // The TangleConsumer receives JobResults and submits them back to the
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
