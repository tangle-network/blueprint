/// Errors that can occur within blueprint storage providers
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Errors from the [`LocalDatabase`]
    ///
    /// [`LocalDatabase`]: blueprint_store_local_database::LocalDatabase
    #[cfg(feature = "local")]
    #[error("Local database error: {0}")]
    LocalDatabase(#[from] blueprint_store_local_database::Error),
}
