//! Tangle Extractors
//!
//! Extractors for job call metadata from Tangle events.
//!
//! ## Input/Output Extractors
//!
//! For ABI-encoded inputs and outputs, use:
//! - [`TangleArg<T>`] - Extracts and ABI-decodes a single argument from the job call body
//! - [`TangleResult<T>`] - Wraps a result and ABI-encodes it for return
//!
//! ## Example
//!
//! ```rust,ignore
//! use blueprint_tangle_extra::extract::{TangleArg, TangleResult};
//!
//! // Job function that takes a u64 and returns its square
//! async fn square(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
//!     TangleResult(x * x)
//! }
//! ```

use alloy_primitives::Address;
use alloy_sol_types::SolValue;
use blueprint_core::{
    __composite_rejection as composite_rejection, __define_rejection as define_rejection,
    extract::OptionalFromJobCallParts,
};
use blueprint_core::{FromJobCall, FromJobCallParts, JobCall, job::call::Parts as JobCallParts};
use blueprint_std::string::{String, ToString};
use bytes::Bytes;

// ═══════════════════════════════════════════════════════════════════════════════
// COMPACT BINARY DECODING
// ═══════════════════════════════════════════════════════════════════════════════

/// Decode a compact-encoded length value.
/// Returns (length, bytes_consumed) or None if invalid.
fn decode_compact_length(data: &[u8]) -> Option<(usize, usize)> {
    if data.is_empty() {
        return None;
    }

    let first = data[0];
    if first < 0x80 {
        // Single byte: 0x00-0x7F
        Some((first as usize, 1))
    } else if first < 0xC0 {
        // Two bytes: 0x80-0xBF + 1 byte
        if data.len() < 2 {
            return None;
        }
        let len = ((first as usize & 0x3F) << 8) | (data[1] as usize);
        Some((len, 2))
    } else if first < 0xE0 {
        // Three bytes: 0xC0-0xDF + 2 bytes
        if data.len() < 3 {
            return None;
        }
        let len = ((first as usize & 0x1F) << 16) | ((data[1] as usize) << 8) | (data[2] as usize);
        Some((len, 3))
    } else if first < 0xF0 {
        // Four bytes: 0xE0-0xEF + 3 bytes
        if data.len() < 4 {
            return None;
        }
        let len = ((first as usize & 0x0F) << 24)
            | ((data[1] as usize) << 16)
            | ((data[2] as usize) << 8)
            | (data[3] as usize);
        Some((len, 4))
    } else {
        // Five bytes: 0xF0 + 4 bytes
        if data.len() < 5 {
            return None;
        }
        let len = ((data[1] as usize) << 24)
            | ((data[2] as usize) << 16)
            | ((data[3] as usize) << 8)
            | (data[4] as usize);
        Some((len, 5))
    }
}

/// Try to decode compact binary format for a string type.
/// Returns the decoded string or None if decoding fails.
fn try_decode_compact_string(data: &[u8]) -> Option<String> {
    let (len, header_size) = decode_compact_length(data)?;
    let string_start = header_size;
    let string_end = string_start + len;

    if string_end > data.len() {
        return None;
    }

    let string_bytes = &data[string_start..string_end];
    core::str::from_utf8(string_bytes).ok().map(String::from)
}

/// Try to decode compact binary format for a tuple/struct with a single string field.
/// This matches the HelloRequest { name: string } pattern.
/// Returns the decoded string or None if decoding fails.
fn try_decode_compact_single_string_struct(data: &[u8]) -> Option<String> {
    // Compact struct format: field_count (compact length) + fields
    // For a single-field struct: 0x01 + string
    let (field_count, header_size) = decode_compact_length(data)?;

    if field_count != 1 {
        return None;
    }

    // The rest is the string field
    try_decode_compact_string(&data[header_size..])
}

/// Check if data appears to be ABI-encoded (heuristic).
/// ABI-encoded dynamic types typically have 32-byte alignment and offset patterns.
fn looks_like_abi_encoded(data: &[u8]) -> bool {
    // ABI-encoded single dynamic type (like string) typically:
    // - Has at least 64 bytes (32-byte offset + 32-byte length minimum)
    // - First 32 bytes contain an offset pointer (typically 0x20 = 32)
    if data.len() < 64 {
        return false;
    }

    // Check if first 32 bytes look like an offset (should be 0x20 for single param)
    // The offset is stored as big-endian u256, so for offset=32, bytes 0-30 are 0, byte 31 is 32
    let first_31_zeros = data[..31].iter().all(|&b| b == 0);
    let offset_is_32 = data[31] == 32;

    first_31_zeros && offset_is_32
}

// ═══════════════════════════════════════════════════════════════════════════════
// CALL ID
// ═══════════════════════════════════════════════════════════════════════════════

/// Extracts the current call id from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallId(pub u64);

impl CallId {
    /// Metadata key for call ID
    pub const METADATA_KEY: &'static str = "X-TANGLE-CALL-ID";
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

impl<Ctx> OptionalFromJobCallParts<Ctx> for CallId
where
    Ctx: Send + Sync,
{
    type Rejection = CallIdRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(CallIdRejection::MissingCallId(_)) => Ok(None),
            Err(err) => Err(err),
        }
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
    pub const METADATA_KEY: &'static str = "X-TANGLE-SERVICE-ID";
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

impl<Ctx> OptionalFromJobCallParts<Ctx> for ServiceId
where
    Ctx: Send + Sync,
{
    type Rejection = ServiceIdRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(ServiceIdRejection::MissingServiceId(_)) => Ok(None),
            Err(err) => Err(err),
        }
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
    pub const METADATA_KEY: &'static str = "X-TANGLE-JOB-INDEX";
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

impl<Ctx> OptionalFromJobCallParts<Ctx> for JobIndex
where
    Ctx: Send + Sync,
{
    type Rejection = JobIndexRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(JobIndexRejection::MissingJobIndex(_)) => Ok(None),
            Err(err) => Err(err),
        }
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
    pub const METADATA_KEY: &'static str = "X-TANGLE-BLOCK-NUMBER";
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

impl<Ctx> OptionalFromJobCallParts<Ctx> for BlockNumber
where
    Ctx: Send + Sync,
{
    type Rejection = BlockNumberRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(BlockNumberRejection::MissingBlockNumber(_)) => Ok(None),
            Err(err) => Err(err),
        }
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
    pub const METADATA_KEY: &'static str = "X-TANGLE-BLOCK-HASH";
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

impl<Ctx> OptionalFromJobCallParts<Ctx> for BlockHash
where
    Ctx: Send + Sync,
{
    type Rejection = BlockHashRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(BlockHashRejection::MissingBlockHash(_)) => Ok(None),
            Err(err) => Err(err),
        }
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
    pub const METADATA_KEY: &'static str = "X-TANGLE-TIMESTAMP";
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

impl<Ctx> OptionalFromJobCallParts<Ctx> for Timestamp
where
    Ctx: Send + Sync,
{
    type Rejection = TimestampRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(TimestampRejection::MissingTimestamp(_)) => Ok(None),
            Err(err) => Err(err),
        }
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
    pub const METADATA_KEY: &'static str = "X-TANGLE-CALLER";

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

impl<Ctx> OptionalFromJobCallParts<Ctx> for Caller
where
    Ctx: Send + Sync,
{
    type Rejection = CallerRejection;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(CallerRejection::MissingCaller(_)) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TANGLE EVM ARG - ABI-Encoded Input Extractor
// ═══════════════════════════════════════════════════════════════════════════════

define_rejection! {
    #[body = "Failed to decode the job input (tried both compact binary and ABI formats)"]
    /// A Rejection type for [`TangleArg`] when decoding fails.
    pub struct AbiDecodeError;
}

/// Extracts and decodes a single argument from the job call body.
///
/// This extractor supports two encoding formats:
/// 1. **Compact binary** - Tangle's native format used by the CLI with `--params-file`
/// 2. **ABI encoding** - Standard Ethereum format used with `--payload-hex`
///
/// The extractor uses heuristics to detect the format:
/// - If data looks like ABI-encoded (64+ bytes with offset patterns), try ABI first
/// - Otherwise, try compact binary first, then fall back to ABI
///
/// # Type Parameters
///
/// * `T` - The type to decode into. Must implement `SolValue`.
///
/// # Example
///
/// ```rust,ignore
/// use blueprint_tangle_extra::extract::TangleArg;
///
/// async fn my_job(TangleArg(value): TangleArg<u64>) -> TangleResult<u64> {
///     TangleResult(value * 2)
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TangleArg<T>(pub T);

impl<T> core::ops::Deref for TangleArg<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for TangleArg<T> {
    fn from(value: T) -> Self {
        TangleArg(value)
    }
}

impl<T, Ctx> FromJobCall<Ctx> for TangleArg<T>
where
    T: SolValue + Send + From<<T::SolType as alloy_sol_types::SolType>::RustType>,
    for<'a> <<T as SolValue>::SolType as alloy_sol_types::SolType>::Token<'a>:
        alloy_sol_types::abi::TokenSeq<'a>,
    Ctx: Send + Sync,
{
    type Rejection = AbiDecodeError;

    async fn from_job_call(call: JobCall, _ctx: &Ctx) -> Result<Self, Self::Rejection> {
        let (_, body) = call.into_parts();

        // Strategy: Use heuristics to determine encoding format
        // 1. If data looks like ABI (64+ bytes with offset pattern), try ABI first
        // 2. Otherwise, try compact binary first, then fall back to ABI
        //
        // This ensures backwards compatibility with --payload-hex (ABI)
        // while supporting the new --params-file (compact binary) format.

        if looks_like_abi_encoded(&body) {
            // Data looks like ABI format, try abi_decode (with outer offset) first
            if let Ok(value) = T::abi_decode(&body) {
                return Ok(TangleArg(value));
            }
            // Try abi_decode_sequence (flat tuple encoding, no outer offset)
            if let Ok(value) = T::abi_decode_sequence(&body) {
                return Ok(TangleArg(value));
            }
            // ABI failed, try compact as fallback
            if let Ok(value) = try_decode_compact::<T>(&body) {
                return Ok(TangleArg(value));
            }
        } else {
            // Data doesn't look like ABI, try compact first
            if let Ok(value) = try_decode_compact::<T>(&body) {
                return Ok(TangleArg(value));
            }
            // Compact failed, try ABI as fallback (with outer offset)
            if let Ok(value) = T::abi_decode(&body) {
                return Ok(TangleArg(value));
            }
            // Try abi_decode_sequence (flat tuple encoding, no outer offset)
            if let Ok(value) = T::abi_decode_sequence(&body) {
                return Ok(TangleArg(value));
            }
        }

        Err(AbiDecodeError)
    }
}

/// Try to decode compact binary format into the target type.
///
/// This function attempts to decode compact binary encoded data by:
/// 1. First trying to ABI-decode the extracted compact data (for struct types)
/// 2. Using type-specific decoding for common patterns
///
/// The compact format used by the CLI encodes:
/// - Strings: compact_length + UTF-8 bytes
/// - Structs/tuples: field_count + fields (each field encoded recursively)
fn try_decode_compact<T>(data: &[u8]) -> Result<T, ()>
where
    T: SolValue + From<<T::SolType as alloy_sol_types::SolType>::RustType>,
{
    // Get the Solidity type name to determine decoding strategy
    use alloy_sol_types::SolType;
    let type_name_str = <T::SolType>::SOL_NAME;

    // Handle tuple/struct types (e.g., "(string)" or "HelloRequest")
    // These are encoded as: field_count + field_1 + field_2 + ...
    if type_name_str.starts_with('(') || !type_name_str.contains('(') {
        // Try to decode as a single-field struct with string
        // This handles the common HelloRequest { name: string } pattern
        if let Some(decoded_string) = try_decode_compact_single_string_struct(data) {
            // Re-encode as ABI and decode to get the proper type
            // This is a workaround since we can't directly construct T
            let abi_encoded = alloy_sol_types::SolValue::abi_encode(&(decoded_string,));
            if let Ok(value) = T::abi_decode(&abi_encoded) {
                return Ok(value);
            }
        }

        // Try direct string decode (for bare string type)
        if type_name_str == "string" {
            if let Some(decoded_string) = try_decode_compact_string(data) {
                let abi_encoded = alloy_sol_types::SolValue::abi_encode(&decoded_string);
                if let Ok(value) = T::abi_decode(&abi_encoded) {
                    return Ok(value);
                }
            }
        }
    }

    Err(())
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
/// use blueprint_tangle_extra::extract::{TangleArg, TangleResult};
///
/// async fn square(TangleArg(x): TangleArg<u64>) -> TangleResult<u64> {
///     TangleResult(x * x)
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TangleResult<T>(pub T);

impl<T> core::ops::Deref for TangleResult<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for TangleResult<T> {
    fn from(value: T) -> Self {
        TangleResult(value)
    }
}

impl<T: SolValue> blueprint_core::IntoJobResult for TangleResult<T> {
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
    use blueprint_core::FromJobCallParts;

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
        assert_eq!(JobIndex::METADATA_KEY, "X-TANGLE-JOB-INDEX");
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Compact binary decoding tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_decode_compact_length_single_byte() {
        // Values 0-127 are encoded as single byte
        assert_eq!(decode_compact_length(&[0x00]), Some((0, 1)));
        assert_eq!(decode_compact_length(&[0x05]), Some((5, 1)));
        assert_eq!(decode_compact_length(&[0x7F]), Some((127, 1)));
    }

    #[test]
    fn test_decode_compact_length_two_bytes() {
        // Values 128-16383 are encoded as 0x80-0xBF + 1 byte
        // 128 = 0x80 | (128 >> 8), (128 & 0xFF) = 0x80, 0x80
        assert_eq!(decode_compact_length(&[0x80, 0x80]), Some((128, 2)));
        // 255 = 0x80 | 0, 0xFF
        assert_eq!(decode_compact_length(&[0x80, 0xFF]), Some((255, 2)));
    }

    #[test]
    fn test_decode_compact_length_empty() {
        assert_eq!(decode_compact_length(&[]), None);
    }

    #[test]
    fn test_try_decode_compact_string() {
        // "TestUser" has length 8, encoded as 0x08 + "TestUser"
        let data = b"\x08TestUser";
        assert_eq!(
            try_decode_compact_string(data),
            Some("TestUser".to_string())
        );
    }

    #[test]
    fn test_try_decode_compact_string_empty() {
        // Empty string: length 0
        let data = b"\x00";
        assert_eq!(try_decode_compact_string(data), Some(String::new()));
    }

    #[test]
    fn test_try_decode_compact_string_hello() {
        // "Hello" has length 5
        let data = b"\x05Hello";
        assert_eq!(try_decode_compact_string(data), Some("Hello".to_string()));
    }

    #[test]
    fn test_try_decode_compact_single_string_struct() {
        // Struct with 1 field (string "TestUser")
        // Format: field_count (1) + string (length 8 + "TestUser")
        let data = b"\x01\x08TestUser";
        assert_eq!(
            try_decode_compact_single_string_struct(data),
            Some("TestUser".to_string())
        );
    }

    #[test]
    fn test_looks_like_abi_encoded() {
        // ABI-encoded string "Hello" would have:
        // - 32 bytes offset (0x20 = 32 for single param)
        // - 32 bytes length
        // - padded string data
        let mut abi_data = vec![0u8; 96];
        abi_data[31] = 32; // offset = 32
        abi_data[63] = 5; // length = 5
        abi_data[64..69].copy_from_slice(b"Hello");

        assert!(looks_like_abi_encoded(&abi_data));
    }

    #[test]
    fn test_looks_like_abi_encoded_false_for_short_data() {
        // Short data should not look like ABI
        let short_data = b"\x08TestUser";
        assert!(!looks_like_abi_encoded(short_data));
    }

    #[test]
    fn test_looks_like_abi_encoded_false_for_compact() {
        // Compact encoded struct should not look like ABI
        let compact_data = b"\x01\x08TestUser";
        assert!(!looks_like_abi_encoded(compact_data));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Optional extractor tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn optional_call_id_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<CallId>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_call_id_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(CallId::METADATA_KEY, [1u8]);

        let extracted = Option::<CallId>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(extracted, Err(CallIdRejection::InvalidCallId(_))));
    }

    #[tokio::test]
    async fn optional_service_id_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<ServiceId>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_service_id_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(ServiceId::METADATA_KEY, [1u8]);

        let extracted = Option::<ServiceId>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(
            extracted,
            Err(ServiceIdRejection::InvalidServiceId(_))
        ));
    }

    #[tokio::test]
    async fn optional_job_index_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<JobIndex>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_job_index_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts
            .metadata
            .insert(JobIndex::METADATA_KEY, Vec::<u8>::new());

        let extracted = Option::<JobIndex>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(
            extracted,
            Err(JobIndexRejection::InvalidJobIndex(_))
        ));
    }

    #[tokio::test]
    async fn optional_block_number_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<BlockNumber>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_block_number_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(BlockNumber::METADATA_KEY, [1u8]);

        let extracted = Option::<BlockNumber>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(
            extracted,
            Err(BlockNumberRejection::InvalidBlockNumber(_))
        ));
    }

    #[tokio::test]
    async fn optional_block_hash_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<BlockHash>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_block_hash_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(BlockHash::METADATA_KEY, [0u8; 31]);

        let extracted = Option::<BlockHash>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(
            extracted,
            Err(BlockHashRejection::InvalidBlockHash(_))
        ));
    }

    #[tokio::test]
    async fn optional_timestamp_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<Timestamp>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_timestamp_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(Timestamp::METADATA_KEY, [1u8]);

        let extracted = Option::<Timestamp>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(
            extracted,
            Err(TimestampRejection::InvalidTimestamp(_))
        ));
    }

    #[tokio::test]
    async fn optional_caller_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<Caller>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_caller_invalid_returns_err() {
        let mut parts = JobCallParts::new(0);
        parts.metadata.insert(Caller::METADATA_KEY, [0u8; 19]);

        let extracted = Option::<Caller>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(extracted, Err(CallerRejection::InvalidCaller(_))));
    }
}
