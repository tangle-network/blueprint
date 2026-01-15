//! Blueprint runner for {{project-name}}.

use {{crate_name}}_lib::router;
use blueprint_sdk::contexts::tangle_evm::TangleEvmClientContext;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::tangle_evm::config::TangleEvmConfig;
use blueprint_sdk::tangle_evm::{TangleEvmConsumer, TangleEvmProducer};
use blueprint_sdk::{error, info};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    // Load configuration from environment variables
    let env = BlueprintEnvironment::load()?;

    // Connect to the Tangle EVM network
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
        .ok_or_else(|| blueprint_sdk::Error::Other("SERVICE_ID missing".into()))?;

    info!("Starting {{project-name}} blueprint for service {service_id}");

    // Create producer (listens for JobSubmitted events) and consumer (submits results)
    let tangle_producer = TangleEvmProducer::new(tangle_client.clone(), service_id);
    let tangle_consumer = TangleEvmConsumer::new(tangle_client);
    let tangle_config = TangleEvmConfig::default();

    // Build and run the blueprint
    let result = BlueprintRunner::builder(tangle_config, env)
        .router(router())
        .producer(tangle_producer)
        .consumer(tangle_consumer)
        .with_shutdown_handler(async {
            info!("Shutting down {{project-name}} blueprint");
        })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed: {e:?}");
    }

    Ok(())
}

fn setup_log() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{EnvFilter, fmt};
    if tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .is_err()
    {}
}
