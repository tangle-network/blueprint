//! Job-specific extractors for EVM
//!
//! Provides extractors for job-related data from EVM events, including
//! `ServiceId`, `CallId`, `JobIndex`, and `Caller` (as EVM Address).

use alloy_primitives::Address;
use blueprint_core::{
    __composite_rejection as composite_rejection, __define_rejection as define_rejection,
    FromJobCallParts, extract::OptionalFromJobCallParts, job::call::Parts as JobCallParts,
};

// ═══════════════════════════════════════════════════════════════════════════
// SERVICE ID
// ═══════════════════════════════════════════════════════════════════════════

/// Extracts the current service id from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceId(pub u64);

impl ServiceId {
    /// Metadata key for service ID
    pub const METADATA_KEY: &'static str = "X-EVM-SERVICE-ID";
}

blueprint_core::__impl_deref!(ServiceId: u64);
blueprint_core::__impl_from!(u64, ServiceId);

define_rejection! {
    #[body = "No ServiceId found in the metadata"]
    /// A Rejection type for [`ServiceId`] when it is missing from the Metadata.
    pub struct MissingServiceId;
}

define_rejection! {
    #[body = "The service id in the metadata is not a valid integer"]
    /// A Rejection type for [`ServiceId`] when it is not a valid u64.
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
        Self::try_from(parts)
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

// ═══════════════════════════════════════════════════════════════════════════
// CALL ID
// ═══════════════════════════════════════════════════════════════════════════

/// Extracts the current call id from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallId(pub u64);

impl CallId {
    /// Metadata key for call ID
    pub const METADATA_KEY: &'static str = "X-EVM-CALL-ID";
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
        Self::try_from(parts)
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

// ═══════════════════════════════════════════════════════════════════════════
// JOB INDEX
// ═══════════════════════════════════════════════════════════════════════════

/// Extracts the job index from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JobIndex(pub u8);

impl JobIndex {
    /// Metadata key for job index
    pub const METADATA_KEY: &'static str = "X-EVM-JOB-INDEX";
}

blueprint_core::__impl_deref!(JobIndex: u8);
blueprint_core::__impl_from!(u8, JobIndex);

define_rejection! {
    #[body = "No JobIndex found in the metadata"]
    /// A Rejection type for [`JobIndex`] when it is missing from the Metadata.
    pub struct MissingJobIndex;
}

define_rejection! {
    #[body = "The job index in the metadata is not a valid integer"]
    /// A Rejection type for [`JobIndex`] when it is not a valid u8.
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
        let job_index: u64 = job_index_raw.try_into().map_err(|_| InvalidJobIndex)?;
        // Safe conversion from u64 to u8
        let job_index = u8::try_from(job_index).map_err(|_| InvalidJobIndex)?;
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
        Self::try_from(parts)
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

// ═══════════════════════════════════════════════════════════════════════════
// CALLER (EVM Address)
// ═══════════════════════════════════════════════════════════════════════════

/// Extracts the caller address from the job call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Caller(pub Address);

impl Caller {
    /// Metadata key for caller address
    pub const METADATA_KEY: &'static str = "X-EVM-CALLER";
}

blueprint_core::__impl_deref!(Caller: Address);
blueprint_core::__impl_from!(Address, Caller);

define_rejection! {
    #[body = "No Caller found in the metadata"]
    /// A Rejection type for [`Caller`] when it is missing from the Metadata.
    pub struct MissingCaller;
}

define_rejection! {
    #[body = "The caller in the metadata is not a valid EVM address"]
    /// A Rejection type for [`Caller`] when it is not a valid Address.
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
        let caller_bytes: [u8; 20] = caller_raw
            .as_bytes()
            .try_into()
            .map_err(|_| InvalidCaller)?;
        let caller = Address::from(caller_bytes);
        Ok(Caller(caller))
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
        Self::try_from(parts)
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

// ═══════════════════════════════════════════════════════════════════════════
// JOB INPUTS
// ═══════════════════════════════════════════════════════════════════════════

/// Extracts the job inputs (bytes) from the job call body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobInputs(pub bytes::Bytes);

impl core::ops::Deref for JobInputs {
    type Target = bytes::Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<bytes::Bytes> for JobInputs {
    fn from(value: bytes::Bytes) -> Self {
        Self(value)
    }
}

impl From<Vec<u8>> for JobInputs {
    fn from(value: Vec<u8>) -> Self {
        Self(bytes::Bytes::from(value))
    }
}

define_rejection! {
    #[body = "No job inputs found in the body"]
    /// A Rejection type for [`JobInputs`] when the body is empty or missing.
    pub struct MissingJobInputs;
}

impl TryFrom<&mut JobCallParts> for JobInputs {
    type Error = MissingJobInputs;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        // The inputs are stored in the extensions by the producer
        let inputs = parts
            .extensions
            .get::<bytes::Bytes>()
            .ok_or(MissingJobInputs)?;
        Ok(JobInputs(inputs.clone()))
    }
}

impl<Ctx> FromJobCallParts<Ctx> for JobInputs
where
    Ctx: Send + Sync,
{
    type Rejection = MissingJobInputs;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Self, Self::Rejection> {
        Self::try_from(parts)
    }
}

impl<Ctx> OptionalFromJobCallParts<Ctx> for JobInputs
where
    Ctx: Send + Sync,
{
    type Rejection = MissingJobInputs;

    async fn from_job_call_parts(
        parts: &mut JobCallParts,
        _: &Ctx,
    ) -> Result<Option<Self>, Self::Rejection> {
        match Self::try_from(parts) {
            Ok(value) => Ok(Some(value)),
            Err(MissingJobInputs) => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blueprint_core::FromJobCallParts;

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
            .insert(JobIndex::METADATA_KEY, 256u64.to_be_bytes());

        let extracted = Option::<JobIndex>::from_job_call_parts(&mut parts, &()).await;
        assert!(matches!(
            extracted,
            Err(JobIndexRejection::InvalidJobIndex(_))
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

    #[tokio::test]
    async fn optional_job_inputs_missing_returns_none() {
        let mut parts = JobCallParts::new(0);
        let extracted = Option::<JobInputs>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(extracted, None);
    }

    #[tokio::test]
    async fn optional_job_inputs_present_returns_some() {
        let mut parts = JobCallParts::new(0);
        parts.extensions.insert(bytes::Bytes::from_static(b"hello"));

        let extracted = Option::<JobInputs>::from_job_call_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(
            extracted,
            Some(JobInputs::from(bytes::Bytes::from_static(b"hello")))
        );
    }
}
