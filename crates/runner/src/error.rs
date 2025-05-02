use blueprint_core::error::BoxError;
use thiserror::Error;
use tokio::task::JoinError;

/// Errors that can occur when constructing or operating a [`BlueprintRunner`](crate::BlueprintRunner)
#[derive(Error, Debug)]
pub enum RunnerError {
    /// Unable to open/interact with the provided [`Keystore`](blueprint_keystore::Keystore)
    #[error("Keystore error: {0}")]
    Keystore(#[from] blueprint_keystore::Error),

    #[cfg(feature = "networking")]
    #[error("Networking error: {0}")]
    Networking(#[from] blueprint_networking::error::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),

    #[error("Pricing engine error: {0}")]
    Pricing(#[from] blueprint_pricing_engine_lib::PricingError),

    /// Unable to read/use the provided configuration values
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// The [`BlueprintRunner`] was configured without a [`Router`]
    ///
    /// [`BlueprintRunner`]: crate::BlueprintRunner
    /// [`Router`]: blueprint_router::Router
    #[error("Blueprint runner configured without a router")]
    NoRouter,
    /// The [`BlueprintRunner`] was configured without any [producers]
    ///
    /// [`BlueprintRunner`]: crate::BlueprintRunner
    /// [producers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/producers/index.html
    #[error("Blueprint runner configured without any producers")]
    NoProducers,

    /// An error occurred in a [`BackgroundService`]
    ///
    /// [`BackgroundService`]: crate::BackgroundService
    #[error("A background service failed: {0}")]
    BackgroundService(String),

    /// Errors that occur during a [`Job`] call
    ///
    /// [`Job`]: blueprint_core::Job
    #[error("A job call failed: {0}")]
    JobCall(#[from] JobCallError),

    /// Errors that come from [producers]
    ///
    /// [producers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/producers/index.html
    #[error("A producer failed: {0}")]
    Producer(#[from] ProducerError),

    /// Errors that come from [consumers]
    ///
    /// [consumers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/consumers/index.html
    #[error("A consumer failed: {0}")]
    Consumer(BoxError),

    // Protocols
    /// [Tangle] protocol errors
    ///
    /// [Tangle]: https://tangle.tools
    #[cfg(feature = "tangle")]
    #[error("Tangle error: {0}")]
    Tangle(#[from] crate::tangle::error::TangleError),

    /// [Eigenlayer] protocol errors
    ///
    /// [Eigenlayer]: https://eigenlayer.xyz
    #[cfg(feature = "eigenlayer")]
    #[error("EigenLayer error: {0}")]
    Eigenlayer(#[from] crate::eigenlayer::error::EigenlayerError),

    #[error("{0}")]
    Other(#[from] Box<dyn core::error::Error + Send + Sync>),
}

/// Errors that can occur while loading and using the blueprint configuration.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// Missing `RPC_URL` environment variable.
    #[error("Missing Tangle RPC endpoint")]
    MissingTangleRpcEndpoint,
    /// Missing `KEYSTORE_URI` environment
    #[error("Missing keystore URI")]
    MissingKeystoreUri,
    /// Missing `BLUEPRINT_ID` environment variable
    #[error("Missing blueprint ID")]
    MissingBlueprintId,
    /// Missing `SERVICE_ID` environment variable
    #[error("Missing service ID")]
    MissingServiceId,
    /// Error parsing the blueprint ID.
    #[error(transparent)]
    MalformedBlueprintId(core::num::ParseIntError),
    /// Error parsing the service ID.
    #[error(transparent)]
    MalformedServiceId(core::num::ParseIntError),
    /// Unsupported keystore URI.
    #[error("Unsupported keystore URI: {0}")]
    UnsupportedKeystoreUri(String),
    /// Error parsing the protocol, from the `PROTOCOL` environment variable.
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
    /// Attempting to load the [`ProtocolSettings`] of a protocol differing from the target
    ///
    /// [`ProtocolSettings`]: crate::config::ProtocolSettings
    #[error("Unexpect protocol, expected {0}")]
    UnexpectedProtocol(&'static str),
    /// No Sr25519 keypair found in the keystore.
    #[error("No Sr25519 keypair found in the keystore")]
    NoSr25519Keypair,
    /// Invalid Sr25519 keypair found in the keystore.
    #[error("Invalid Sr25519 keypair found in the keystore")]
    InvalidSr25519Keypair,
    /// No ECDSA keypair found in the keystore.
    #[error("No ECDSA keypair found in the keystore")]
    NoEcdsaKeypair,
    /// Invalid ECDSA keypair found in the keystore.
    #[error("Invalid ECDSA keypair found in the keystore")]
    InvalidEcdsaKeypair,
    /// Test setup error
    #[error("Test setup error: {0}")]
    TestSetup(String),
    /// Missing `EigenlayerContractAddresses`
    #[error("Missing EigenlayerContractAddresses")]
    MissingEigenlayerContractAddresses,
    /// Missing `SymbioticContractAddresses`
    #[error("Missing SymbioticContractAddresses")]
    MissingSymbioticContractAddresses,

    #[error("{0}")]
    Other(#[from] Box<dyn core::error::Error + Send + Sync>),
}

/// Errors that occur during a [`Job`] call
///
/// [`Job`]: blueprint_core::Job
#[derive(Error, Debug)]
pub enum JobCallError {
    /// The job call completed and returned an error
    #[error("Job call failed: {0}")]
    JobFailed(Box<dyn core::error::Error + Send + Sync>),

    /// The job call did not complete (canceled or panicked)
    #[error("Job failed to finish: {0}")]
    JobDidntFinish(JoinError),
}

/// Errors that come from [producers]
///
/// [producers]: https://docs.rs/blueprint_sdk/latest/blueprint_sdk/producers/index.html
#[derive(Error, Debug)]
pub enum ProducerError {
    /// The producer, when polled, produced an error
    #[error("A producer failed to produce a value: {0}")]
    Failed(Box<dyn core::error::Error + Send + Sync>),

    /// The producer stream ended prematurely
    #[error("A producer stream ended unexpectedly")]
    StreamEnded,
}
