#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "local")]
    #[error("Local database error: {0}")]
    LocalDatabase(#[from] gadget_store_local_database::Error),
}
