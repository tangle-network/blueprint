use blueprint_context_derive::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;

#[derive(KeystoreContext)]
#[context(crate = "blueprint_sdk", crate = "also_blueprint_sdk")]
#[allow(dead_code)]
struct MyContext {
    #[config]
    config: BlueprintEnvironment,
}

fn main() {}
