//! Job index extractor

use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The job index from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobIndex(pub u8);

impl JobIndex {
    /// Metadata key for job index
    pub const METADATA_KEY: &'static str = "tangle_evm.job_index";
}

/// Error when job index is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing job index")]
pub struct InvalidJobIndex;

impl FromJobCallParts for JobIndex {
    type Error = InvalidJobIndex;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidJobIndex)?;

        let job_index: u8 = value.try_into().map_err(|_| InvalidJobIndex)?;
        Ok(JobIndex(job_index))
    }
}

impl FromJobCall for JobIndex {
    type Error = InvalidJobIndex;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<JobIndex> for u8 {
    fn from(val: JobIndex) -> Self {
        val.0
    }
}
