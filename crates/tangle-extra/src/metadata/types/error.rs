#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The blueprint.json defines no sources. There must be at least 1")]
    NoSources,
    #[error("Encountered a malformed source in the blueprint.json")]
    BadSource,

    #[error("Failed to parse the blueprint service manager")]
    BadServiceManager,
}
