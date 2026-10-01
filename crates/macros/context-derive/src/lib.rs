#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    unsafe_code,
    unstable_features,
    unused_qualifications,
    missing_docs,
    unused_results,
    clippy::exhaustive_enums
)]
//! Proc-macros for deriving Context Extensions from [`blueprint-sdk`](https://crates.io/crates/blueprint-sdk) crate.
use proc_macro::TokenStream;

/// Field information for the configuration field.
mod cfg;
/// Resolution of the SDK crate path used by generated code.
mod crate_path;
/// EVM Provider context extension implementation.
#[cfg(all(feature = "std", feature = "evm"))]
mod evm;
/// Keystore context extension implementation.
mod keystore;
/// Tangle context extensions.
#[cfg(feature = "tangle")]
mod tangle;

const CONFIG_TAG_NAME: &str = "config";
const CONFIG_TAG_TYPE: &str = "blueprint_runner::config::BlueprintEnvironment";

/// Derive macro for generating Context Extensions trait implementation for `KeystoreContext`.
///
/// Accepts `#[context(crate = "...")]` to point the generated code at an SDK
/// dependency that is not named `blueprint_sdk`. Defaults to `::blueprint_sdk`.
#[proc_macro_derive(KeystoreContext, attributes(config, call_id, context))]
pub fn derive_keystore_context(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let result = crate_path::parse_crate_path(&input.attrs).and_then(|sdk| {
        cfg::find_config_field(&input.ident, &input.data, CONFIG_TAG_NAME, CONFIG_TAG_TYPE)
            .map(|config_field| keystore::generate_context_impl(input, config_field, &sdk))
    });

    match result {
        Ok(expanded) => TokenStream::from(expanded),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// Derive macro for generating Context Extensions trait implementation for `EVMProviderContext`.
///
/// Accepts `#[context(crate = "...")]` to point the generated code at an SDK
/// dependency that is not named `blueprint_sdk`. Defaults to `::blueprint_sdk`.
#[cfg(all(feature = "std", feature = "evm"))]
#[proc_macro_derive(EVMProviderContext, attributes(config, call_id, context))]
pub fn derive_evm_provider_context(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let result = crate_path::parse_crate_path(&input.attrs).and_then(|sdk| {
        cfg::find_config_field(&input.ident, &input.data, CONFIG_TAG_NAME, CONFIG_TAG_TYPE)
            .map(|config_field| evm::generate_context_impl(input, config_field, &sdk))
    });

    match result {
        Ok(expanded) => TokenStream::from(expanded),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// Derive macro for generating Context Extensions trait implementation for `TangleClientContext`.
///
/// Accepts `#[context(crate = "...")]` to point the generated code at an SDK
/// dependency that is not named `blueprint_sdk`. Defaults to `::blueprint_sdk`.
#[cfg(feature = "tangle")]
#[proc_macro_derive(TangleClientContext, attributes(config, call_id, context))]
pub fn derive_tangle_client_context(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let result = crate_path::parse_crate_path(&input.attrs).and_then(|sdk| {
        cfg::find_config_field(&input.ident, &input.data, CONFIG_TAG_NAME, CONFIG_TAG_TYPE)
            .map(|config_field| tangle::client::generate_context_impl(input, config_field, &sdk))
    });

    match result {
        Ok(expanded) => TokenStream::from(expanded),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
