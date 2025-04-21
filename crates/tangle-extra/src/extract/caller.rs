use blueprint_core::{
    __composite_rejection as composite_rejection, __define_rejection as define_rejection,
};
use blueprint_core::{FromJobCallParts, job::call::Parts as JobCallParts};
use tangle_subxt::subxt::utils::AccountId32;

/// Extracts the current caller from the job call.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Caller(pub AccountId32);

impl Caller {
    pub const METADATA_KEY: &'static str = "X-TANGLE-CALLER-ID";
}

blueprint_core::__impl_deref!(Caller: AccountId32);
blueprint_core::__impl_from!(AccountId32, Caller);

define_rejection! {
  #[body = "No Caller found in the metadata"]
  /// A Rejection type for [`Caller`] when it is missing from the Metadata.
  pub struct MissingCaller;
}

define_rejection! {
  #[body = "The caller id in the metadata is not a valid AccountId32"]
  /// A Rejection type for [`Caller`] when it is not a valid AccountId32.
  pub struct InvalidCaller;
}

composite_rejection! {
    /// Rejection used for [`Caller`].
    ///
    /// Contains one variant for each way the [`Caller`] extractor
    /// can fail.
    pub enum CallerRejection {
        MissingCaller,
        InvalidCaller,
    }
}

impl TryFrom<&mut JobCallParts> for Caller {
    type Error = CallerRejection;

    fn try_from(parts: &mut JobCallParts) -> Result<Self, Self::Error> {
        let caller_id_raw = parts
            .metadata
            .get(Self::METADATA_KEY)
            .ok_or(MissingCaller)?;
        let caller_id_bytes: [u8; 32] = caller_id_raw
            .as_bytes()
            .try_into()
            .map_err(|_| InvalidCaller)?;
        let caller_id = AccountId32::from(caller_id_bytes);
        Ok(Caller(caller_id))
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
