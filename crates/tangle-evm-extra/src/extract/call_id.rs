//! Call ID extractor

use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The call ID from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallId(pub u64);

impl CallId {
    /// Metadata key for call ID
    pub const METADATA_KEY: &'static str = "tangle_evm.call_id";
}

/// Error when call ID is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing call ID")]
pub struct InvalidCallId;

impl FromJobCallParts for CallId {
    type Error = InvalidCallId;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidCallId)?;

        let call_id: u64 = value.try_into().map_err(|_| InvalidCallId)?;
        Ok(CallId(call_id))
    }
}

impl FromJobCall for CallId {
    type Error = InvalidCallId;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<CallId> for u64 {
    fn from(val: CallId) -> Self {
        val.0
    }
}
