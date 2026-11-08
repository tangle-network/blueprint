//! One native provider primitive for SDK leases and external orchestrators.

pub(crate) mod authorization;
pub mod bootstrap;
mod credentials;
mod leases;
mod metadata;
mod operations;
mod ownership;
mod quotes;
#[cfg(feature = "schema")]
mod schema;
mod types;
#[cfg(feature = "schema")]
pub use schema::{CredentialField, ProviderDescriptor, provider_catalog};

pub use blueprint_core::resources::ManagedLeaseIdentity;
pub use credentials::ProviderCredentials;
pub use leases::{Lease, LeaseAccess, RetireReport};
pub use metadata::ManagedMetadata;
pub use ownership::lock_lease;
pub use quotes::rank_quotes;
pub use types::*;

use crate::{ResourceSpec, Result};
use async_trait::async_trait;

/// All managed verbs are required; a registry cannot advertise a partial adapter.
#[async_trait]
pub trait ManagedProviderAdapter: Send + Sync {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>>;
    async fn create(
        &self,
        spec: &ResourceSpec,
        offer: &NativeOffer,
        metadata: &ManagedMetadata,
    ) -> Result<ManagedResource>;
    /// Finish recoverable setup without renting another instance.
    async fn complete(
        &self,
        spec: &ResourceSpec,
        resource: ManagedResource,
    ) -> Result<ManagedResource>;
    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        provider_id: &str,
    ) -> Result<Option<ManagedResource>>;
    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>>;
    async fn destroy(
        &self,
        identity: &ManagedLeaseIdentity,
        resource: &ManagedResource,
    ) -> Result<()>;
}

pub fn request_fingerprint(spec: &ResourceSpec) -> Result<String> {
    let invalid = || crate::Error::Provider(ProviderErrorCode::InvalidRequest);
    let mut value = serde_json::to_value(spec).map_err(|_| invalid())?;
    value["env"] = serde_json::to_value(&spec.env).map_err(|_| invalid())?;
    value
        .as_object_mut()
        .expect("resource object")
        .remove("ownership");
    // Unified `serde_json/preserve_order` must not change stored authority digests.
    value.sort_all_objects();
    let bytes = serde_json::to_vec(&value).map_err(|_| invalid())?;
    Ok(hex::encode(aws_lc_rs::digest::digest(
        &aws_lc_rs::digest::SHA256,
        &bytes,
    )))
}
