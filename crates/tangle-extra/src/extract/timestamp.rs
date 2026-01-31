//! Timestamp extractor

use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The block timestamp from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Metadata key for timestamp
    pub const METADATA_KEY: &'static str = "tangle.timestamp";
}

/// Error when timestamp is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing timestamp")]
pub struct InvalidTimestamp;

impl FromJobCallParts for Timestamp {
    type Error = InvalidTimestamp;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidTimestamp)?;

        let timestamp: u64 = value.try_into().map_err(|_| InvalidTimestamp)?;
        Ok(Timestamp(timestamp))
    }
}

impl FromJobCall for Timestamp {
    type Error = InvalidTimestamp;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<Timestamp> for u64 {
    fn from(val: Timestamp) -> Self {
        val.0
    }
}
