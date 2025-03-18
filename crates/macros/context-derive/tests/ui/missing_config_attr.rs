use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_context_derive::KeystoreContext;

#[derive(KeystoreContext)]
struct MyContext {
    foo: String,
    sdk_config: BlueprintEnvironment,
}

fn main() {}
