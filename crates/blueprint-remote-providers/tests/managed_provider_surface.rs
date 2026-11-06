//! Compile-only downstream consumer contract for the managed provider boundary.
//!
//! This target establishes the missing public surface before native wire tests.
//! It has no provider behavior cases and must not be reported as provider parity.

use blueprint_remote_providers::{CloudProvisioner, ManagedResource, ProviderCredentials};

/// Public caller configuration and provider observations cross the SDK boundary.
pub fn managed_consumer_contract(
    credentials: Option<ProviderCredentials>,
    observation: Option<ManagedResource>,
) {
    let _ = (credentials, observation);
    let _ = CloudProvisioner::from_credentials;
    let _ = CloudProvisioner::quotes;
    let _ = CloudProvisioner::create_managed;
    let _ = CloudProvisioner::status_managed;
    let _ = CloudProvisioner::list_managed;
    let _ = CloudProvisioner::destroy_managed;
}
