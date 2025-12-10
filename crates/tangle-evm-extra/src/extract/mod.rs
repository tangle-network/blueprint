//! Tangle EVM Extractors
//!
//! Extractors for job call metadata from Tangle EVM events.
//! These mirror the extractors in `blueprint-tangle-extra` but for EVM.
//!
//! ## Input/Output Extractors
//!
//! For ABI-encoded inputs and outputs, use:
//! - [`TangleEvmArg<T>`] - Extracts and ABI-decodes a single argument from the job call body
//! - [`TangleEvmResult<T>`] - Wraps a result and ABI-encodes it for return
//!
//! ## Example
//!
//! ```rust,ignore
//! use blueprint_tangle_evm_extra::extract::{TangleEvmArg, TangleEvmResult};
//!
//! // Job function that takes a u64 and returns its square
//! async fn square(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
//!     TangleEvmResult(x * x)
//! }
//! ```

use alloy_primitives::Address;
use alloy_sol_types::SolValue;
use blueprint_core::{
    __composite_rejection as composite_rejection, __define_rejection as define_rejection,
};
use blueprint_core::{FromJobCall, FromJobCallParts, JobCall, job::call::Parts as JobCallParts};
use blueprint_std::string::{String, ToString};
use bytes::Bytes;

// ═══════════════════════════════════════════════════════════════════════════════
// CALL ID
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the current call id from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallId(pub u64);

impl CallId {
    /// Metadata key for call ID
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-CALL-ID";
}

blueprint_core::__impl_deref!(CallId: u64);
blueprint_core::__impl_from!(u64, CallId);

define_rejection! {
    #[body = "No CallId found in the metadata"]
    /// A Rejection type for [`CallId`] when it is missing from the Metadata.
    pub struct MissingCallId;
}

define_rejection! {
    #[body = "The call id in the metadata is not a valid integer"]
    /// A Rejection type for [`CallId`] when it is not a valid u64.
    pub struct InvalidCallId;
}

composite_rejection! {
    /// Rejection used for [`CallId`].
    pub enum CallIdRejection {
        MissingCallId,
        InvalidCallId,
    }
}

impl TryFrom<&mut JobCallParts> for CallId {
    type Error = CallIdRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let call_id_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingCallId)?;
        let call_id = call_id_raw.try_into().map_err(|_| InvalidCallId)?;
        Ok(CallId(call_id))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for CallId
where
    Ctx: Send + Sync,
{
    type Rejection = CallIdRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        CallId::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SERVICE ID
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the service id from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceId(pub u64);

impl ServiceId {
    /// Metadata key for service ID
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-SERVICE-ID";
}

blueprint_core::__impl_deref!(ServiceId: u64);
blueprint_core::__impl_from!(u64, ServiceId);

define_rejection! {
    #[body = "No ServiceId found in the metadata"]
    pub struct MissingServiceId;
}

define_rejection! {
    #[body = "The service id in the metadata is not a valid integer"]
    pub struct InvalidServiceId;
}

composite_rejection! {
    /// Rejection used for [`ServiceId`].
    pub enum ServiceIdRejection {
        MissingServiceId,
        InvalidServiceId,
    }
}

impl TryFrom<&mut JobCallParts> for ServiceId {
    type Error = ServiceIdRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let service_id_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingServiceId)?;
        let service_id = service_id_raw.try_into().map_err(|_| InvalidServiceId)?;
        Ok(ServiceId(service_id))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for ServiceId
where
    Ctx: Send + Sync,
{
    type Rejection = ServiceIdRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        ServiceId::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// JOB INDEX
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the job index from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JobIndex(pub u8);

impl JobIndex {
    /// Metadata key for job index
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-JOB-INDEX";
}

blueprint_core::__impl_deref!(JobIndex: u8);
blueprint_core::__impl_from!(u8, JobIndex);

define_rejection! {
    #[body = "No JobIndex found in the metadata"]
    pub struct MissingJobIndex;
}

define_rejection! {
    #[body = "The job index in the metadata is not a valid u8"]
    pub struct InvalidJobIndex;
}

composite_rejection! {
    /// Rejection used for [`JobIndex`].
    pub enum JobIndexRejection {
        MissingJobIndex,
        InvalidJobIndex,
    }
}

impl TryFrom<&mut JobCallParts> for JobIndex {
    type Error = JobIndexRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let job_index_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingJobIndex)?;
        // u8 doesn't have a TryFrom<&MetadataValue> impl, so we extract the first byte manually
        let bytes = job_index_raw.as_bytes();
        let job_index = *bytes.first().ok_or(InvalidJobIndex)?;
        Ok(JobIndex(job_index))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for JobIndex
where
    Ctx: Send + Sync,
{
    type Rejection = JobIndexRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        JobIndex::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK NUMBER
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the block number from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockNumber(pub u64);

impl BlockNumber {
    /// Metadata key for block number
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-BLOCK-NUMBER";
}

blueprint_core::__impl_deref!(BlockNumber: u64);
blueprint_core::__impl_from!(u64, BlockNumber);

define_rejection! {
    #[body = "No BlockNumber found in the metadata"]
    pub struct MissingBlockNumber;
}

define_rejection! {
    #[body = "The block number in the metadata is not a valid integer"]
    pub struct InvalidBlockNumber;
}

composite_rejection! {
    /// Rejection used for [`BlockNumber`].
    pub enum BlockNumberRejection {
        MissingBlockNumber,
        InvalidBlockNumber,
    }
}

impl TryFrom<&mut JobCallParts> for BlockNumber {
    type Error = BlockNumberRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let block_number_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingBlockNumber)?;
        let block_number = block_number_raw
            .try_into()
            .map_err(|_| InvalidBlockNumber)?;
        Ok(BlockNumber(block_number))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for BlockNumber
where
    Ctx: Send + Sync,
{
    type Rejection = BlockNumberRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        BlockNumber::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLOCK HASH
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the block hash from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockHash(pub [u8; 32]);

impl BlockHash {
    /// Metadata key for block hash
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-BLOCK-HASH";
}

impl core::ops::Deref for BlockHash {
    type Target = [u8; 32];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; 32]> for BlockHash {
    fn from(hash: [u8; 32]) -> Self {
        BlockHash(hash)
    }
}

define_rejection! {
    #[body = "No BlockHash found in the metadata"]
    pub struct MissingBlockHash;
}

define_rejection! {
    #[body = "The block hash in the metadata is not valid"]
    pub struct InvalidBlockHash;
}

composite_rejection! {
    /// Rejection used for [`BlockHash`].
    pub enum BlockHashRejection {
        MissingBlockHash,
        InvalidBlockHash,
    }
}

impl TryFrom<&mut JobCallParts> for BlockHash {
    type Error = BlockHashRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let block_hash_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingBlockHash)?;
        // [u8; 32] doesn't have a TryFrom<&MetadataValue> impl, so we convert manually
        let bytes = block_hash_raw.as_bytes();
        let hash: [u8; 32] = (&bytes[..]).try_into().map_err(|_| InvalidBlockHash)?;
        Ok(BlockHash(hash))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for BlockHash
where
    Ctx: Send + Sync,
{
    type Rejection = BlockHashRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        BlockHash::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TIMESTAMP
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the block timestamp from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Metadata key for timestamp
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-TIMESTAMP";
}

blueprint_core::__impl_deref!(Timestamp: u64);
blueprint_core::__impl_from!(u64, Timestamp);

define_rejection! {
    #[body = "No Timestamp found in the metadata"]
    pub struct MissingTimestamp;
}

define_rejection! {
    #[body = "The timestamp in the metadata is not a valid integer"]
    pub struct InvalidTimestamp;
}

composite_rejection! {
    /// Rejection used for [`Timestamp`].
    pub enum TimestampRejection {
        MissingTimestamp,
        InvalidTimestamp,
    }
}

impl TryFrom<&mut JobCallParts> for Timestamp {
    type Error = TimestampRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let timestamp_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingTimestamp)?;
        let timestamp = timestamp_raw.try_into().map_err(|_| InvalidTimestamp)?;
        Ok(Timestamp(timestamp))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for Timestamp
where
    Ctx: Send + Sync,
{
    type Rejection = TimestampRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        Timestamp::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CALLER
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the caller address from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Caller(pub [u8; 20]);

impl Caller {
    /// Metadata key for caller
    pub const METADATA_KEY: &'static str = "X-TANGLE-EVM-CALLER";

    /// Get as alloy Address
    #[must_use]
    pub fn as_address(&self) -> Address {
        Address::from_slice(&self.0)
    }
}

impl core::ops::Deref for Caller {
    type Target = [u8; 20];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; 20]> for Caller {
    fn from(addr: [u8; 20]) -> Self {
        Caller(addr)
    }
}

impl From<Address> for Caller {
    fn from(addr: Address) -> Self {
        Caller(addr.0.0)
    }
}

define_rejection! {
    #[body = "No Caller found in the metadata"]
    pub struct MissingCaller;
}

define_rejection! {
    #[body = "The caller in the metadata is not a valid address"]
    pub struct InvalidCaller;
}

composite_rejection! {
    /// Rejection used for [`Caller`].
    pub enum CallerRejection {
        MissingCaller,
        InvalidCaller,
    }
}

impl TryFrom<&mut JobCallParts> for Caller {
    type Error = CallerRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let caller_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingCaller)?;
        // [u8; 20] doesn't have a TryFrom<&MetadataValue> impl, so we convert manually
        let bytes = caller_raw.as_bytes();
        let addr: [u8; 20] = (&bytes[..]).try_into().map_err(|_| InvalidCaller)?;
        Ok(Caller(addr))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for Caller
where
    Ctx: Send + Sync,
{
    type Rejection = CallerRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        Caller::try_from(parts)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TANGLE EVM ARG - ABI-Encoded Input Extractor
// ═══════════════════════════════════════════════════════════════════════════════

define_rejection! {
    #[body = "Failed to ABI-decode the job input"]
    /// A Rejection type for [`TangleEvmArg`] when ABI decoding fails.
    pub struct AbiDecodeError;
}

/// Extracts and ABI-decodes a single argument from the job call body.
///
/// This extractor uses Alloy's `SolValue::abi_decode` to decode the raw bytes
/// from the job call into the specified type.
///
/// # Type Parameters
///
/// * `T` - The type to decode into. Must implement `SolValue`.
///
/// # Example
///
/// ```rust,ignore
/// use blueprint_tangle_evm_extra::extract::TangleEvmArg;
///
/// async fn my_job(TangleEvmArg(value): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
///     TangleEvmResult(value * 2)
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TangleEvmArg<T>(pub T);

impl<T> core::ops::Deref for TangleEvmArg<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for TangleEvmArg<T> {
    fn from(value: T) -> Self {
        TangleEvmArg(value)
    }
}

impl<T, Ctx> FromJobCall<Ctx> for TangleEvmArg<T>
where
    T: SolValue + Send + From<<T::SolType as alloy_sol_types::SolType>::RustType>,
    Ctx: Send + Sync,
{
    type Rejection = AbiDecodeError;

    async fn from_job_call(call: JobCall, _ctx: &Ctx) -> Result<Self, Self::Rejection> {
        let (_, body) = call.into_parts();
        let value = T::abi_decode(&body).map_err(|_| AbiDecodeError)?;
        Ok(TangleEvmArg(value))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TANGLE EVM RESULT - ABI-Encoded Output Wrapper
// ═══════════════════════════════════════════════════════════════════════════════

/// Wraps a job result and ABI-encodes it for return.
///
/// This wrapper encodes the inner value using Alloy's `SolValue::abi_encode`
/// so it can be submitted back to the Tangle contract.
///
/// # Type Parameters
///
/// * `T` - The type to encode. Must implement `SolValue`.
///
/// # Example
///
/// ```rust,ignore
/// use blueprint_tangle_evm_extra::extract::{TangleEvmArg, TangleEvmResult};
///
/// async fn square(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
///     TangleEvmResult(x * x)
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TangleEvmResult<T>(pub T);

impl<T> core::ops::Deref for TangleEvmResult<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for TangleEvmResult<T> {
    fn from(value: T) -> Self {
        TangleEvmResult(value)
    }
}

impl<T: SolValue> blueprint_core::IntoJobResult for TangleEvmResult<T> {
    fn into_job_result(self) -> Option<blueprint_core::JobResult> {
        let encoded = self.0.abi_encode();
        Some(blueprint_core::JobResult::Ok {
            head: blueprint_core::job::result::Parts::new(),
            body: Bytes::from(encoded),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════════
    // JobIndex extraction tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_job_index_extraction() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(JobIndex::METADATA_KEY, [5u8]);

        let job_index = JobIndex::try_from(&mut parts).expect("should extract job index");
        assert_eq!(*job_index, 5);
    }

    #[test]
    fn test_job_index_different_values() {
        // Test job index 0
        let mut parts0 = JobCallParts::new(0);
        parts0.metadata.insert(JobIndex::METADATA_KEY, [0u8]);
        let idx0 = JobIndex::try_from(&mut parts0).unwrap();
        assert_eq!(*idx0, 0);

        // Test job index 1
        let mut parts1 = JobCallParts::new(0);
        parts1.metadata.insert(JobIndex::METADATA_KEY, [1u8]);
        let idx1 = JobIndex::try_from(&mut parts1).unwrap();
        assert_eq!(*idx1, 1);

        // Test job index 255 (max u8)
        let mut parts255 = JobCallParts::new(0);
        parts255.metadata.insert(JobIndex::METADATA_KEY, [255u8]);
        let idx255 = JobIndex::try_from(&mut parts255).unwrap();
        assert_eq!(*idx255, 255);

        // Verify they're different
        assert_ne!(idx0, idx1);
        assert_ne!(idx1, idx255);
    }

    #[test]
    fn test_job_index_missing() {
        let mut parts = JobCallParts::new(0);
        let result = JobIndex::try_from(&mut parts);
        assert!(result.is_err());
    }

    #[test]
    fn test_job_index_into_u8() {
        let job_index = JobIndex(42);
        let val: u8 = *job_index;
        assert_eq!(val, 42);
    }

    #[test]
    fn test_job_index_metadata_key() {
        assert_eq!(JobIndex::METADATA_KEY, "X-TANGLE-EVM-JOB-INDEX");
    }

    #[test]
    fn test_per_job_routing_simulation() {
        // Simulate how AggregatingConsumer would differentiate jobs
        // Job 0: might not require aggregation
        // Job 1: might require aggregation

        let mut parts_job0 = JobCallParts::new(0);
        parts_job0.metadata.insert(JobIndex::METADATA_KEY, [0u8]);
        parts_job0
            .metadata
            .insert(ServiceId::METADATA_KEY, 1u64.to_be_bytes());

        let mut parts_job1 = JobCallParts::new(0);
        parts_job1.metadata.insert(JobIndex::METADATA_KEY, [1u8]);
        parts_job1
            .metadata
            .insert(ServiceId::METADATA_KEY, 1u64.to_be_bytes());

        // Extract both
        let job_idx0 = JobIndex::try_from(&mut parts_job0).unwrap();
        let job_idx1 = JobIndex::try_from(&mut parts_job1).unwrap();

        // They should have different indices despite same service_id
        assert_eq!(*job_idx0, 0);
        assert_eq!(*job_idx1, 1);
        assert_ne!(job_idx0, job_idx1);

        // Each would result in different aggregation config lookups
        // get_aggregation_config(service_id=1, job_index=0) vs
        // get_aggregation_config(service_id=1, job_index=1)
    }
}
