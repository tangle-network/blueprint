//! Service ID extractor

use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The service ID from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceId(pub u64);

impl ServiceId {
    /// Metadata key for service ID
    pub const METADATA_KEY: &'static str = "tangle_evm.service_id";
}

/// Error when service ID is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing service ID")]
pub struct InvalidServiceId;

impl FromJobCallParts for ServiceId {
    type Error = InvalidServiceId;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidServiceId)?;

        let service_id: u64 = value.try_into().map_err(|_| InvalidServiceId)?;
        Ok(ServiceId(service_id))
    }
}

impl FromJobCall for ServiceId {
    type Error = InvalidServiceId;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<ServiceId> for u64 {
    fn from(val: ServiceId) -> Self {
        val.0
    }
}
