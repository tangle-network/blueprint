use blueprint_sdk::Job;
use blueprint_sdk::Router;
use blueprint_sdk::{info, error};
use blueprint_sdk::contexts::tangle::TangleClientContext;
use std::sync::Arc;
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
use oauth_blueprint_lib::{
    OAuthBlueprintContext,
    AuthEchoBackgroundService,
    WHOAMI_JOB_ID, whoami,
    CHECK_SCOPE_JOB_ID, check_scope,
    WRITE_DOC_JOB_ID, write_doc,
    READ_DOC_JOB_ID, read_doc,
    LIST_DOCS_JOB_ID, list_docs,
    ADMIN_PURGE_JOB_ID, admin_purge,
    ECHO_JOB_ID, echo,
};
use tower::filter::FilterLayer;

fn setup_log() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();

    info!("Starting the OAuth Blueprint!");

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

    // Create the context with tangle client
    let context = OAuthBlueprintContext {
        tangle_client: Arc::new(tangle_client.clone()),
    };

    let result = BlueprintRunner::builder(tangle_config, env)
        .router(
            Router::new()
                .route(WHOAMI_JOB_ID, whoami.layer(TangleLayer))
                .route(CHECK_SCOPE_JOB_ID, check_scope.layer(TangleLayer))
                .route(WRITE_DOC_JOB_ID, write_doc.layer(TangleLayer))
                .route(READ_DOC_JOB_ID, read_doc.layer(TangleLayer))
                .route(LIST_DOCS_JOB_ID, list_docs.layer(TangleLayer))
                .route(ADMIN_PURGE_JOB_ID, admin_purge.layer(TangleLayer))
                .route(ECHO_JOB_ID, echo.layer(TangleLayer))
                .layer(FilterLayer::new(MatchesServiceId(service_id)))
                .with_context(context),
        )
        .background_service(AuthEchoBackgroundService)
        .producer(tangle_producer)
        .consumer(tangle_consumer)
        .with_shutdown_handler(async { println!("Shutting down OAuth Blueprint!") })
        .run()
        .await;

    if let Err(e) = result {
        error!("Runner failed! {e:?}");
    }

    Ok(())
}