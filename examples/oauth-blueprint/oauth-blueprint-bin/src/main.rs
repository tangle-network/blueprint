use blueprint_sdk::contexts::tangle_evm::TangleEvmClientContext;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::tangle_evm::config::TangleEvmConfig;
use blueprint_sdk::tangle_evm::{TangleEvmConsumer, TangleEvmLayer, TangleEvmProducer};
use blueprint_sdk::{Job, Router, error, info};
use oauth_blueprint_lib::{
    ADMIN_PURGE_JOB_ID, OAuthProtectedApiService, WRITE_DOC_JOB_ID, admin_purge, write_doc,
};

fn setup_log() {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{EnvFilter, fmt};
    if tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .is_err()
    {
        // logging already initialized
    }
}

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    let env = BlueprintEnvironment::load()?;
    let tangle_client = env
        .tangle_evm_client()
        .await
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    let service_id = env
        .protocol_settings
        .tangle_evm()
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?
        .service_id
        .ok_or_else(|| blueprint_sdk::Error::Other("SERVICE_ID not configured".into()))?;

    info!("Starting OAuth blueprint for service {service_id}");

    let tangle_producer = TangleEvmProducer::new(tangle_client.clone(), service_id);
    let tangle_consumer = TangleEvmConsumer::new(tangle_client);
    let tangle_config = TangleEvmConfig::default();

    let result = BlueprintRunner::builder(tangle_config, env)
        .router(
            Router::new()
                .route(WRITE_DOC_JOB_ID, write_doc.layer(TangleEvmLayer))
                .route(ADMIN_PURGE_JOB_ID, admin_purge.layer(TangleEvmLayer)),
        )
        .background_service(OAuthProtectedApiService)
        .producer(tangle_producer)
        .consumer(tangle_consumer)
        .with_shutdown_handler(async {
            info!("Shutting down OAuth blueprint");
        })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed: {e:?}");
    }

    Ok(())
}
