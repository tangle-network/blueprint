//! Block number extractor

use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The block number from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockNumber(pub u64);

impl BlockNumber {
    /// Metadata key for block number
    pub const METADATA_KEY: &'static str = "tangle_evm.block_number";
}

/// Error when block number is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing block number")]
pub struct InvalidBlockNumber;

impl FromJobCallParts for BlockNumber {
    type Error = InvalidBlockNumber;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidBlockNumber)?;

        let block_number: u64 = value.try_into().map_err(|_| InvalidBlockNumber)?;
        Ok(BlockNumber(block_number))
    }
}

impl FromJobCall for BlockNumber {
    type Error = InvalidBlockNumber;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<BlockNumber> for u64 {
    fn from(val: BlockNumber) -> Self {
        val.0
    }
}
