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
use incredible_squaring_blueprint_lib::{FooBackgroundService, XSQUARE_JOB_ID, XSQUARE_FAAS_JOB_ID, square, square_faas};
use tower::filter::FilterLayer;

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    // Initialize logging - can be configured via RUST_LOG environment variable

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

    // FaaS Executor Configuration
    // For testing: Use custom HTTP FaaS executor (no AWS credentials needed)
    // For production: Replace with LambdaExecutor::new("us-east-1", role_arn).await?
    #[cfg(feature = "faas")]
    let faas_executor = {
        use blueprint_faas::custom::HttpFaasExecutor;

        // In production, this would be your FaaS endpoint
        // For local testing: run a test server on localhost:8080
        HttpFaasExecutor::new("http://localhost:8080")
            .with_job_endpoint(XSQUARE_FAAS_JOB_ID, "http://localhost:8080/square")
    };

    let mut runner_builder = BlueprintRunner::builder(tangle_config, env)
        .router(
            Router::new()
                // Job 0: LOCAL execution - runs on this machine
                .route(XSQUARE_JOB_ID, square.layer(TangleLayer))
                // Job 1: FAAS execution - delegated to serverless
                // CRITICAL: Also has TangleLayer so results go to TangleConsumer → onchain
                .route(XSQUARE_FAAS_JOB_ID, square_faas.layer(TangleLayer))
                .layer(FilterLayer::new(MatchesServiceId(service_id))),
        )
        .background_service(FooBackgroundService)
        .producer(tangle_producer)
        .consumer(tangle_consumer);

    // Register FaaS executor for job 1
    // This is THE critical line: job 1 will be delegated to FaaS instead of running locally
    #[cfg(feature = "faas")]
    {
        runner_builder = runner_builder.with_faas_executor(XSQUARE_FAAS_JOB_ID, faas_executor);
        info!("✅ Job {} registered for FaaS execution", XSQUARE_FAAS_JOB_ID);
        info!("📊 Job {} will execute locally", XSQUARE_JOB_ID);
    }

    let result = runner_builder
        .with_shutdown_handler(async { println!("Shutting down!") })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed! {e:?}");
    }

    Ok(())
}
