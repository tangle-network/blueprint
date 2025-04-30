/// Errors that can occur while interacting with the [`LocalDatabase`]
///
/// [`LocalDatabase`]: crate::LocalDatabase
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Errors during I/O operations
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Errors serializing or deserializing the database
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}
