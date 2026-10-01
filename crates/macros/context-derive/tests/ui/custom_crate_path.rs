//! Proves the derive macros actually emit the configured path.
//!
//! The SDK is a dev-dependency of this crate, so the default `::blueprint_sdk`
//! path always resolves here and a passing compile would prove nothing. Pointing
//! the override at a path that does not exist inverts that: the file only fails
//! to compile if the generated code honours `#[context(crate = "...")]`. If the
//! override were ever dropped, the derives would emit `::blueprint_sdk`, this
//! file would compile, and the `compile_fail` expectation would break.

use blueprint_context_derive::KeystoreContext;
use blueprint_sdk::runner::config::BlueprintEnvironment;

#[derive(KeystoreContext)]
#[context(crate = "sdk_path_that_does_not_exist")]
#[allow(dead_code)]
struct MyContext {
    #[config]
    config: BlueprintEnvironment,
}

fn main() {}
