use crate::error::Error;
use crate::metadata::{MetadataMap, MetadataValue};
use bytes::Bytes;

mod into_job_result;
mod into_job_result_parts;

pub use into_job_result::IntoJobResult;
pub use into_job_result_parts::IntoJobResultParts;
pub use into_job_result_parts::JobResultParts;

/// A special result type that indicates a job produced no result
///
/// This is **not** the same as returning `None` or `()` from your [`Job`].
///
/// It is useful for the following situations:
///
/// * Multiple parties are running a [`Job`], but only one should submit the result. All other parties
///   should return [`Void`].
/// * The [`Job`] doesn't produce anything to submit
///
/// This can also be used in a [`Result`] to return nothing in the success case but still allow for
/// reporting errors:
///
/// ```rust
/// use blueprint_core::error::BoxError;
/// use blueprint_sdk::job::result::Void;
///
/// async fn my_job() -> Result<Void, BoxError> {
///     Ok(Void)
/// }
/// ```
///
/// [`Job`]: crate::Job
pub struct Void;

/// The result of a [`Job`] call
///
/// This type is rarely used directly. It is produced by the [`IntoJobResult`] trait and given to
/// [consumers].
///
/// See the [module docs](crate::job::result) for more details.
///
/// [`Job`]: crate::job::Job
/// [consumers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/consumers/index.html
#[derive(Debug, Clone)]
pub enum JobResult<T = Bytes, E = Error> {
    Ok { head: Parts, body: T },
    Err(E),
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct Parts {
    /// Any Metadata that were included in this result.
    pub metadata: MetadataMap<MetadataValue>,
}

impl Parts {
    pub fn new() -> Self {
        Self {
            metadata: MetadataMap::new(),
        }
    }
}

impl<T, E> JobResult<T, E> {
    pub fn empty() -> Self
    where
        T: Default,
    {
        Self::Ok {
            head: Parts::new(),
            body: Default::default(),
        }
    }

    pub fn new(body: T) -> Self {
        Self::Ok {
            head: Parts::new(),
            body,
        }
    }

    pub fn from_parts(parts: Parts, body: T) -> Self {
        Self::Ok { head: parts, body }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, JobResult::Ok { .. })
    }

    pub fn is_err(&self) -> bool {
        matches!(self, JobResult::Err { .. })
    }

    pub fn metadata(&self) -> Option<&MetadataMap<MetadataValue>> {
        match self {
            JobResult::Ok { head, .. } => Some(&head.metadata),
            JobResult::Err(_) => None,
        }
    }

    pub fn metadata_mut(&mut self) -> Option<&mut MetadataMap<MetadataValue>> {
        match self {
            JobResult::Ok { head, .. } => Some(&mut head.metadata),
            JobResult::Err(_) => None,
        }
    }

    pub fn body_mut(&mut self) -> Result<&mut T, &E> {
        match self {
            JobResult::Ok { body, .. } => Ok(body),
            JobResult::Err(e) => Err(e),
        }
    }

    pub fn body(&self) -> Result<&T, &E> {
        match self {
            JobResult::Ok { body, .. } => Ok(body),
            JobResult::Err(err) => Err(err),
        }
    }

    pub fn into_parts(self) -> Result<(Parts, T), E> {
        match self {
            JobResult::Ok { head, body } => Ok((head, body)),
            JobResult::Err(err) => Err(err),
        }
    }

    pub fn map<F, U>(self, f: F) -> JobResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            JobResult::Ok { head, body } => JobResult::Ok {
                head,
                body: f(body),
            },
            JobResult::Err(err) => JobResult::Err(err),
        }
    }

    pub fn map_err<F, U>(self, f: F) -> JobResult<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            JobResult::Ok { head, body } => JobResult::Ok { head, body },
            JobResult::Err(err) => JobResult::Err(f(err)),
        }
    }
}
