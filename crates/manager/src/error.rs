use tangle_subxt::subxt::ext::jsonrpsee::core::__reexports::serde_json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No fetchers found for blueprint")]
    NoFetchers,
    #[error("No testing fetcher found for blueprint, despite operating in test mode")]
    NoTestFetcher,
    #[error("Blueprint does not contain a supported fetcher")]
    UnsupportedBlueprint,

    #[error("Unable to find matching binary")]
    NoMatchingBinary,
    #[error("Binary hash {expected} mismatched expected hash of {actual}")]
    HashMismatch { expected: String, actual: String },
    #[error("Failed to build binary: {0:?}")]
    BuildBinary(std::process::Output),
    #[error("Failed to fetch git root: {0:?}")]
    FetchGitRoot(std::process::Output),
    #[error("Failed to verify attestation for GitHub release")]
    AttestationFailed,
    #[error("No GitHub CLI found, is it installed?")]
    NoGithubCli,
    #[error("Bridge error: {0}")]
    Bridge(#[from] blueprint_manager_bridge::error::Error),
    #[error("Hypervisor error: {0}")]
    Hypervisor(String),
    #[error("Networking error: {0}")]
    Net(#[from] rtnetlink::Error),
    #[error("Capabilities error: {0}")]
    Caps(#[from] capctl::Error),
    #[error("nftables error: {0}")]
    Nftables(#[from] nftables::helper::NftablesError),

    #[error("Failed to get initial block hash")]
    InitialBlock,
    #[error("Finality Notification stream died")]
    ClientDied,
    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Errno(#[from] nix::errno::Errno),
    #[error(transparent)]
    WalkDir(#[from] walkdir::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    TangleClient(#[from] blueprint_clients::tangle::error::Error),
    #[error(transparent)]
    Auth(#[from] blueprint_auth::Error),
}
