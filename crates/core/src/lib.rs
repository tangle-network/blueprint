#![doc(
    html_logo_url = "https://cdn.prod.website-files.com/6494562b44a28080aafcbad4/65aaf8b0818b1d504cbdf81b_Tnt%20Logo.png"
)]
#![no_std]

extern crate alloc;

#[macro_use]
pub(crate) mod macros;

#[doc(hidden)]
pub mod __private {
    pub use tracing;
}

pub mod error;
pub mod ext_traits;
pub mod extensions;
pub mod extract;
pub mod job;
pub mod metadata;

pub use bytes::Bytes;
pub use error::Error;
pub use ext_traits::{job_call::JobCallExt, job_call_parts::JobCallPartsExt};
pub use extract::{FromJobCall, FromJobCallParts};
pub use job::Job;
pub use job::JobId;
pub use job::result::{IntoJobResult, IntoJobResultParts};

/// A type representing a job result with a body of type `bytes::Bytes`.
pub type JobResult<T = Bytes> = job::result::JobResult<T>;
/// A type representing a job call with a body of type `bytes::Bytes`.
pub type JobCall<T = Bytes> = job::call::JobCall<T>;

// Feature-gated tracing macros, used by the entire SDK
macro_rules! tracing_macros {
    ($d:tt $($name:ident),*) => {
        $(
            #[doc(hidden)]
            #[cfg(feature = "tracing")]
            pub use tracing::$name;

            #[doc(hidden)]
            #[cfg(not(feature = "tracing"))]
            #[macro_export]
            macro_rules! $name {
                ($d($d tt:tt)*) => {
                    if false {
                        let _ = $crate::__private::tracing::$name!($d($d tt)*);
                    }
                };
            }
        )*
    }
}

tracing_macros!($
    info,
    warn,
    error,
    debug,
    trace
);
