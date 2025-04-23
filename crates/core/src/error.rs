use core::fmt;

/// Alias for a type-erased error type.
pub type BoxError = alloc::boxed::Box<dyn core::error::Error + Send + Sync>;
pub type CloneableError = alloc::sync::Arc<dyn core::error::Error + Send + Sync>;

/// Errors that can happen when using `blueprint-sdk` job routing.
#[derive(Debug, Clone)]
pub struct Error {
    inner: CloneableError,
}

impl Error {
    /// Create a new `Error` from a boxable error.
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: CloneableError::from(error.into()),
        }
    }

    /// Convert an `Error` back into the underlying trait object.
    pub fn into_inner(self) -> CloneableError {
        self.inner
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&*self.inner)
    }
}
