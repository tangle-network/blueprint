//! Caller address extractor

use alloy_primitives::Address;
use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The caller address from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caller(pub Address);

impl Caller {
    /// Metadata key for caller
    pub const METADATA_KEY: &'static str = "tangle_evm.caller";
}

/// Error when caller is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing caller address")]
pub struct InvalidCaller;

impl FromJobCallParts for Caller {
    type Error = InvalidCaller;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidCaller)?;

        let bytes: [u8; 20] = value.try_into().map_err(|_| InvalidCaller)?;
        Ok(Caller(Address::from(bytes)))
    }
}

impl FromJobCall for Caller {
    type Error = InvalidCaller;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<Caller> for Address {
    fn from(val: Caller) -> Self {
        val.0
    }
}
