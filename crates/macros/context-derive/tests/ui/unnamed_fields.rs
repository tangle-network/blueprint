use blueprint_context_derive::{EVMProviderContext, KeystoreContext, TangleClientContext};
use blueprint_sdk::contexts::instrumented_evm_client::EvmInstrumentedClientContext as _;
use blueprint_sdk::contexts::keystore::KeystoreContext as _;
use blueprint_sdk::contexts::tangle::TangleClientContext as _;
use blueprint_sdk::runner::config::BlueprintEnvironment;

#[derive(KeystoreContext, EVMProviderContext, TangleClientContext)]
#[allow(dead_code)]
struct MyContext(
    String,
    #[config] BlueprintEnvironment,
    #[call_id] Option<u64>,
);

#[allow(dead_code)]
fn main() {
    let body = async {
        let ctx = MyContext("bar".to_string(), BlueprintEnvironment::default(), None);
        let _keystore = ctx.keystore();
        let _evm_provider = ctx.evm_client();
        let _tangle_client = ctx.tangle_client().await.unwrap();
    };
    drop(body);
}
