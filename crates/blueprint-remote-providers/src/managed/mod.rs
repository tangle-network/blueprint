//! One native provider primitive for SDK leases and external orchestrators.

pub mod bootstrap;
mod credentials;
mod metadata;
mod operations;
mod ownership;
mod quotes;
mod types;

pub use blueprint_core::resources::ManagedLeaseIdentity;
pub use credentials::ProviderCredentials;
pub use metadata::ManagedMetadata;
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
        fingerprint: &str,
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
    let mut value = serde_json::to_value(spec)
        .map_err(|_| crate::Error::Provider(ProviderErrorCode::InvalidRequest))?;
    value["env"] = serde_json::to_value(&spec.env)
        .map_err(|_| crate::Error::Provider(ProviderErrorCode::InvalidRequest))?;
    value
        .as_object_mut()
        .expect("resource object")
        .remove("ownership");
    let bytes = serde_json::to_vec(&value)
        .map_err(|_| crate::Error::Provider(ProviderErrorCode::InvalidRequest))?;
    Ok(hex::encode(aws_lc_rs::digest::digest(
        &aws_lc_rs::digest::SHA256,
        &bytes,
    )))
}
