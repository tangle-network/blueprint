use blueprint_sdk::Job;
use blueprint_sdk::Router;
use blueprint_sdk::{info, error};
use blueprint_sdk::contexts::tangle::TangleClientContext;
use blueprint_sdk::crypto::sp_core::SpSr25519;
use blueprint_sdk::crypto::tangle_pair_signer::TanglePairSigner;
use blueprint_sdk::keystore::backends::Backend;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::tangle::config::TangleConfig;
use blueprint_sdk::tangle::consumer::TangleConsumer;
use blueprint_sdk::tangle::filters::MatchesServiceId;
use blueprint_sdk::tangle::layers::TangleLayer;
use blueprint_sdk::tangle::producer::TangleProducer;
use incredible_squaring_blueprint_lib::{FooBackgroundService, XSQUARE_JOB_ID, square};
use tower::filter::FilterLayer;

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    info!("Starting the incredible squaring blueprint!");

    let env = BlueprintEnvironment::load()?;
    let keystore = env.keystore();
    let sr25519_signer = keystore.first_local::<SpSr25519>()?;
    let sr25519_pair = keystore.get_secret::<SpSr25519>(&sr25519_signer)?;
    let st25519_signer = TanglePairSigner::new(sr25519_pair.0);

    let tangle_client = env.tangle_client().await?;
    let tangle_producer =
        TangleProducer::finalized_blocks(tangle_client.rpc_client.clone()).await?;
    let tangle_consumer = TangleConsumer::new(tangle_client.rpc_client.clone(), st25519_signer);

    let tangle_config = TangleConfig::default();

    let service_id = env.protocol_settings.tangle()?.service_id.unwrap();
    let result = BlueprintRunner::builder(tangle_config, env)
        .router(
            // A router
            //
            // Each "route" is a job ID and the job function. We can also support arbitrary `Service`s from `tower`,
            // which may make it easier for people to port over existing services to a blueprint.
            Router::new()
                // The route defined here has a `TangleLayer`, which adds metadata to the
                // produced `JobResult`s, making it visible to a `TangleConsumer`.
                .route(XSQUARE_JOB_ID, square.layer(TangleLayer))
                // Add the `FilterLayer` to filter out job calls that don't match the service ID
                .layer(FilterLayer::new(MatchesServiceId(service_id))),
        )
        .background_service(FooBackgroundService)
        // Add potentially many producers
        //
        // A producer is simply a `Stream` that outputs `JobCall`s, which are passed down to the intended
        // job functions.
        .producer(tangle_producer)
        // Add potentially many consumers
        //
        // A consumer is simply a `Sink` that consumes `JobResult`s, which are the output of the job functions.
        // Every result will be passed to every consumer. It is the responsibility of the consumer
        // to determine whether or not to process a result.
        .consumer(tangle_consumer)
        // Custom shutdown handlers
        //
        // Now users can specify what to do when an error occurs and the runner is shutting down.
        // That can be cleanup logic, finalizing database transactions, etc.
        .with_shutdown_handler(async { println!("Shutting down!") })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed! {e:?}");
    }

    Ok(())
}
