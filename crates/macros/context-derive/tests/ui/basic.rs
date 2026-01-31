use blueprint_context_derive::{EVMProviderContext, KeystoreContext, TangleClientContext};
use blueprint_sdk::contexts::instrumented_evm_client::EvmInstrumentedClientContext as _;
use blueprint_sdk::contexts::keystore::KeystoreContext as _;
use blueprint_sdk::contexts::tangle::TangleClientContext as _;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::std::sync::Arc;
use blueprint_sdk::stores::local_database::LocalDatabase;

#[derive(KeystoreContext, EVMProviderContext, TangleClientContext)]
#[allow(dead_code)]
struct MyContext {
    foo: String,
    #[config]
    config: BlueprintEnvironment,
    store: Arc<LocalDatabase<u64>>,
    #[call_id]
    call_id: Option<u64>,
}

#[allow(dead_code)]
fn main() {
    let body = async {
        let ctx = MyContext {
            foo: "bar".to_string(),
            config: BlueprintEnvironment::default(),
            store: Arc::new(LocalDatabase::open("test.json").unwrap()),
            call_id: None,
        };

        // Test existing context functions
        let _keystore = ctx.keystore();
        let _evm_provider = ctx.evm_client();
        let _tangle_client = ctx.tangle_client().await.unwrap();
    };

    drop(body);
}
