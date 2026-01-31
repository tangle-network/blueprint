use apikey_blueprint_lib::{
    ApiKeyProtectedService, PURCHASE_API_KEY_JOB_ID, WRITE_RESOURCE_JOB_ID, purchase_api_key,
    write_resource,
};
use blueprint_sdk::contexts::tangle::TangleClientContext;
use blueprint_sdk::registration;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::tangle::config::TangleConfig;
use blueprint_sdk::tangle::{TangleConsumer, TangleLayer, TangleProducer};
use blueprint_sdk::{Job, Router, error, info};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    let env = BlueprintEnvironment::load()?;
    let tangle_client = env
        .tangle_client()
        .await
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    if env.registration_mode() {
        let payload = apikey_blueprint_lib::registration_payload();
        let output_path = registration::write_registration_inputs(&env, payload)
            .await
            .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;
        info!(
            "API key blueprint registration payload saved to {}",
            output_path.display()
        );
        return Ok(());
    }

    let service_id = env
        .protocol_settings
        .tangle()
        .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?
        .service_id
        .ok_or_else(|| blueprint_sdk::Error::Other("SERVICE_ID missing".into()))?;

    info!("Starting API key blueprint for service {service_id}");

    let tangle_producer = TangleProducer::new(tangle_client.clone(), service_id);
    let tangle_consumer = TangleConsumer::new(tangle_client);
    let tangle_config = TangleConfig::default();

    let result = BlueprintRunner::builder(tangle_config, env)
        .router(
            Router::new()
                .route(WRITE_RESOURCE_JOB_ID, write_resource.layer(TangleLayer))
                .route(
                    PURCHASE_API_KEY_JOB_ID,
                    purchase_api_key.layer(TangleLayer),
                ),
        )
        .background_service(ApiKeyProtectedService)
        .producer(tangle_producer)
        .consumer(tangle_consumer)
        .with_shutdown_handler(async {
            info!("Shutting down API key blueprint");
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
