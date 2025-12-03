//! Block hash extractor

use blueprint_core::extract::{FromJobCall, FromJobCallParts};
use blueprint_core::job::call::Parts;
use blueprint_core::metadata::MetadataValue;

/// The block hash from a Tangle EVM job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockHash(pub [u8; 32]);

impl BlockHash {
    /// Metadata key for block hash
    pub const METADATA_KEY: &'static str = "tangle_evm.block_hash";
}

/// Error when block hash is missing or invalid
#[derive(Debug, thiserror::Error)]
#[error("Invalid or missing block hash")]
pub struct InvalidBlockHash;

impl FromJobCallParts for BlockHash {
    type Error = InvalidBlockHash;

    fn from_job_call_parts(parts: &Parts) -> Result<Self, Self::Error> {
        let value = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(InvalidBlockHash)?;

        let hash: [u8; 32] = value.try_into().map_err(|_| InvalidBlockHash)?;
        Ok(BlockHash(hash))
    }
}

impl FromJobCall for BlockHash {
    type Error = InvalidBlockHash;

    fn from_job_call(call: &blueprint_core::JobCall) -> Result<Self, Self::Error> {
        Self::from_job_call_parts(call.parts())
    }
}

impl From<BlockHash> for [u8; 32] {
    fn from(val: BlockHash) -> Self {
        val.0
    }
}
