use crate::keystore::backend::GenericKeyStore;
#[cfg(any(feature = "std", feature = "wasm"))]
use crate::keystore::BackendExt;
#[cfg(any(feature = "std", feature = "wasm"))]
use crate::keystore::TanglePairSigner;
use alloc::string::{String, ToString};
use alloy_primitives::Address;
use core::fmt::Debug;
use core::net::IpAddr;
use eigensdk::crypto_bls;
use gadget_io::SupportedChains;
use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

/// The protocol on which a gadget will be executed.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(
    feature = "std",
    derive(clap::ValueEnum),
    clap(rename_all = "lowercase")
)]
pub enum Protocol {
    #[default]
    Tangle,
    Eigenlayer,
}

impl Protocol {
    /// Returns the protocol from the environment variable `PROTOCOL`.
    ///
    /// If the environment variable is not set, it defaults to `Protocol::Tangle`.
    ///
    /// # Errors
    ///
    /// * [`Error::UnsupportedProtocol`] if the protocol is unknown. See [`Protocol`].
    #[cfg(feature = "std")]
    pub fn from_env() -> Result<Self, Error> {
        if let Ok(protocol) = std::env::var("PROTOCOL") {
            return protocol.to_ascii_lowercase().parse::<Protocol>();
        }

        Ok(Protocol::default())
    }

    /// Returns the protocol from the environment variable `PROTOCOL`.
    #[cfg(not(feature = "std"))]
    pub fn from_env() -> Result<Self, Error> {
        Ok(Protocol::default())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Tangle => "tangle",
            Self::Eigenlayer => "eigenlayer",
        }
    }
}

impl core::fmt::Display for Protocol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::str::FromStr for Protocol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tangle" => Ok(Self::Tangle),
            "eigenlayer" => Ok(Self::Eigenlayer),
            _ => Err(Error::UnsupportedProtocol(s.to_string())),
        }
    }
}

/// Gadget environment using the `parking_lot` RwLock.
#[cfg(feature = "std")]
pub type StdGadgetConfiguration = GadgetConfiguration<parking_lot::RawRwLock>;

/// Gadget environment.
#[non_exhaustive]
pub struct GadgetConfiguration<RwLock: lock_api::RawRwLock> {
    /// Tangle HTTP RPC endpoint.
    pub http_rpc_endpoint: String,
    /// Tangle WS RPC endpoint.
    pub ws_rpc_endpoint: String,
    /// Keystore URI
    ///
    /// * In Memory: `file::memory:` or `:memory:`
    /// * Filesystem: `file:/path/to/keystore` or `file:///path/to/keystore`
    pub keystore_uri: String,
    /// Data directory exclusively for this gadget
    ///
    /// This will be `None` if the blueprint manager was not provided a base directory.
    pub data_dir: Option<PathBuf>,
    /// The list of bootnodes to connect to
    pub bootnodes: Vec<Multiaddr>,
    /// Blueprint ID for this gadget.
    pub blueprint_id: u64,
    /// Service ID for this gadget.
    ///
    /// This is only set to `None` when the gadget is in the registration mode.
    /// Always check for is `is_registration` flag before using this.
    pub service_id: Option<u64>,
    /// The Current Environment is for the `PreRegisteration` of the Gadget
    ///
    /// The gadget will now start in the Registration mode and will try to register the current operator on that blueprint
    /// There is no Service ID for this mode, since we need to register the operator first on the blueprint.
    ///
    /// If this is set to true, the gadget should do some work and register the operator on the blueprint.
    pub is_registration: bool,
    /// The type of protocol the gadget is executing on.
    pub protocol: Protocol,
    /// The Port of the Network that will be interacted with
    pub bind_port: u16,
    /// The Address of the Network that will be interacted with
    pub bind_addr: IpAddr,
    /// Specifies custom tracing span for the gadget
    pub span: tracing::Span,
    /// Whether the gadget is in test mode
    pub test_mode: bool,
    /// Basic Eigenlayer contract system
    pub eigenlayer_contract_addrs: EigenlayerContractAddresses,
    _lock: core::marker::PhantomData<RwLock>,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EigenlayerContractAddresses {
    /// The address of the registry coordinator contract
    pub registry_coordinator_addr: Address,
    /// The address of the operator state retriever contract
    pub operator_state_retriever_addr: Address,
    /// The address of the operator registry contract
    pub delegation_manager_addr: Address,
    /// The address of the strategy manager contract
    pub strategy_manager_addr: Address,
}

impl<RwLock: lock_api::RawRwLock> Debug for GadgetConfiguration<RwLock> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GadgetConfiguration")
            .field("http_rpc_endpoint", &self.http_rpc_endpoint)
            .field("ws_rpc_endpoint", &self.ws_rpc_endpoint)
            .field("keystore_uri", &self.keystore_uri)
            .field("data_dir", &self.data_dir)
            .field("bootnodes", &self.bootnodes)
            .field("blueprint_id", &self.blueprint_id)
            .field("service_id", &self.service_id)
            .field("is_registration", &self.is_registration)
            .field("protocol", &self.protocol)
            .field("bind_port", &self.bind_port)
            .field("bind_addr", &self.bind_addr)
            .field("test_mode", &self.test_mode)
            .field("eigenlayer_contract_addrs", &self.eigenlayer_contract_addrs)
            .finish()
    }
}

impl<RwLock: lock_api::RawRwLock> Clone for GadgetConfiguration<RwLock> {
    fn clone(&self) -> Self {
        Self {
            http_rpc_endpoint: self.http_rpc_endpoint.clone(),
            ws_rpc_endpoint: self.ws_rpc_endpoint.clone(),
            keystore_uri: self.keystore_uri.clone(),
            data_dir: self.data_dir.clone(),
            bootnodes: self.bootnodes.clone(),
            blueprint_id: self.blueprint_id,
            service_id: self.service_id,
            eigenlayer_contract_addrs: self.eigenlayer_contract_addrs,
            is_registration: self.is_registration,
            protocol: self.protocol,
            bind_port: self.bind_port,
            bind_addr: self.bind_addr,
            span: self.span.clone(),
            test_mode: self.test_mode,
            _lock: core::marker::PhantomData,
        }
    }
}

// Useful for quick testing
impl<RwLock: lock_api::RawRwLock> Default for GadgetConfiguration<RwLock> {
    fn default() -> Self {
        Self {
            http_rpc_endpoint: "http://localhost:9944".to_string(),
            ws_rpc_endpoint: "ws://localhost:9944".to_string(),
            keystore_uri: "file::memory:".to_string(),
            data_dir: None,
            bootnodes: Vec::new(),
            blueprint_id: 0,
            service_id: Some(0),
            eigenlayer_contract_addrs: Default::default(),
            is_registration: false,
            protocol: Protocol::Tangle,
            bind_port: 0,
            bind_addr: core::net::IpAddr::V4(core::net::Ipv4Addr::new(127, 0, 0, 1)),
            span: tracing::Span::current(),
            test_mode: true,
            _lock: core::marker::PhantomData,
        }
    }
}

/// Errors that can occur while loading and using the gadget configuration.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
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
    /// Error opening the filesystem keystore.
    #[error(transparent)]
    Keystore(#[from] crate::keystore::Error),
    /// Subxt error.
    #[error(transparent)]
    #[cfg(any(feature = "std", feature = "wasm"))]
    Subxt(#[from] subxt::Error),
    /// Error parsing the protocol, from the `PROTOCOL` environment variable.
    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),
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
}

#[derive(Debug, Clone, clap::Parser, Serialize, Deserialize)]
#[command(name = "General CLI Context")]
#[cfg(feature = "std")]
pub struct ContextConfig {
    /// Pass through arguments to another command
    #[command(subcommand)]
    pub gadget_core_settings: GadgetCLICoreSettings,
}

#[derive(Debug, Clone, clap::Parser, Serialize, Deserialize)]
#[cfg(feature = "std")]
pub enum GadgetCLICoreSettings {
    #[command(name = "run")]
    Run {
        #[arg(long, short = 'b', env)]
        bind_addr: IpAddr,
        #[arg(long, short = 'p', env)]
        bind_port: u16,
        #[arg(long, short = 't', env)]
        test_mode: bool,
        #[arg(long, short = 'l', env)]
        log_id: Option<String>,
        #[arg(long, env)]
        #[serde(default = "gadget_io::defaults::http_rpc_url")]
        http_rpc_url: Url,
        #[arg(long, env)]
        #[serde(default = "gadget_io::defaults::ws_rpc_url")]
        ws_rpc_url: Url,
        #[arg(long, value_parser = <Multiaddr as std::str::FromStr>::from_str, action = clap::ArgAction::Append, env)]
        #[serde(default)]
        bootnodes: Option<Vec<Multiaddr>>,
        #[arg(long, short = 'd', env)]
        keystore_uri: String,
        #[arg(long, value_enum, env)]
        chain: SupportedChains,
        #[arg(long, short = 'v', action = clap::ArgAction::Count, env)]
        verbose: u8,
        /// Whether to use pretty logging
        #[arg(long, env)]
        pretty: bool,
        #[arg(long, env)]
        keystore_password: Option<String>,
        #[arg(long, env)]
        blueprint_id: u64,
        #[arg(long, env)]
        service_id: Option<u64>,
        /// The protocol to use
        #[arg(long, value_enum, env)]
        protocol: Protocol,
        /// The address of the registry coordinator
        #[arg(
            long,
            value_name = "ADDR",
            env = "REGISTRY_COORDINATOR_ADDR",
            required_if_eq("protocol", Protocol::Eigenlayer.as_str()),
        )]
        registry_coordinator: Option<Address>,
        /// The address of the operator state retriever
        #[arg(
            long,
            value_name = "ADDR",
            env = "OPERATOR_STATE_RETRIEVER_ADDR",
            required_if_eq("protocol", Protocol::Eigenlayer.as_str())
        )]
        operator_state_retriever: Option<Address>,
        /// The address of the delegation manager
        #[arg(
            long,
            value_name = "ADDR",
            env = "DELEGATION_MANAGER_ADDR",
            required_if_eq("protocol", Protocol::Eigenlayer.as_str())
        )]
        delegation_manager: Option<Address>,
        /// The address of the strategy manager
        #[arg(
            long,
            value_name = "ADDR",
            env = "STRATEGY_MANAGER_ADDR",
            required_if_eq("protocol", Protocol::Eigenlayer.as_str())
        )]
        strategy_manager: Option<Address>,
    },
}

/// Loads the [`GadgetConfiguration`] from the current environment.
/// # Errors
///
/// This function will return an error if any of the required environment variables are missing.
#[cfg(feature = "std")]
pub fn load(config: ContextConfig) -> Result<GadgetConfiguration<parking_lot::RawRwLock>, Error> {
    load_with_lock::<parking_lot::RawRwLock>(config)
}

/// Loads the [`GadgetConfiguration`] from the current environment.
///
/// This allows callers to specify the `RwLock` implementation to use.
///
/// # Errors
///
/// This function will return an error if any of the required environment variables are missing.
// TODO: Add no_std support
#[cfg(feature = "std")]
pub fn load_with_lock<RwLock: lock_api::RawRwLock>(
    config: ContextConfig,
) -> Result<GadgetConfiguration<RwLock>, Error> {
    load_inner::<RwLock>(config)
}

#[cfg(feature = "std")]
fn load_inner<RwLock: lock_api::RawRwLock>(
    config: ContextConfig,
) -> Result<GadgetConfiguration<RwLock>, Error> {
    let is_registration = std::env::var("REGISTRATION_MODE_ON").is_ok();
    let ContextConfig {
        gadget_core_settings:
            GadgetCLICoreSettings::Run {
                bind_addr,
                bind_port,
                test_mode,
                log_id,
                http_rpc_url,
                ws_rpc_url,
                bootnodes,
                keystore_uri,
                blueprint_id,
                service_id,
                protocol,
                registry_coordinator,
                operator_state_retriever,
                delegation_manager,
                strategy_manager,
                ..
            },
        ..
    } = config;

    let span = match log_id {
        Some(id) => tracing::info_span!("gadget", id = id),
        None => tracing::info_span!("gadget"),
    };

    Ok(GadgetConfiguration {
        bind_addr,
        bind_port,
        test_mode,
        span,
        http_rpc_endpoint: http_rpc_url.to_string(),
        ws_rpc_endpoint: ws_rpc_url.to_string(),
        keystore_uri,
        data_dir: std::env::var("DATA_DIR").ok().map(PathBuf::from),
        bootnodes: bootnodes.unwrap_or_default(),
        blueprint_id,
        // If the registration mode is on, we don't need the service ID
        service_id: if is_registration {
            None
        } else {
            Some(service_id.ok_or_else(|| Error::MissingServiceId)?)
        },
        is_registration,
        protocol,
        // Eigenlayer contract addresses will be None if the protocol is not Eigenlayer
        // otherwise, they will be the values provided by the user
        eigenlayer_contract_addrs: EigenlayerContractAddresses {
            registry_coordinator_addr: registry_coordinator.unwrap_or_default(),
            operator_state_retriever_addr: operator_state_retriever.unwrap_or_default(),
            delegation_manager_addr: delegation_manager.unwrap_or_default(),
            strategy_manager_addr: strategy_manager.unwrap_or_default(),
        },
        _lock: core::marker::PhantomData,
    })
}

impl<RwLock: lock_api::RawRwLock> GadgetConfiguration<RwLock> {
    /// Loads the `KeyStore` from the current environment.
    ///
    /// # Errors
    ///
    /// This function will return an error if the keystore URI is unsupported.
    pub fn keystore(&self) -> Result<GenericKeyStore<RwLock>, Error> {
        #[cfg(feature = "std")]
        use crate::keystore::backend::fs::FilesystemKeystore;
        use crate::keystore::backend::{mem::InMemoryKeystore, GenericKeyStore};

        match self.keystore_uri.as_str() {
            uri if uri == "file::memory:" || uri == ":memory:" => {
                Ok(GenericKeyStore::Mem(InMemoryKeystore::new()))
            }
            #[cfg(feature = "std")]
            uri if uri.starts_with("file:") || uri.starts_with("file://") => {
                let path = uri
                    .trim_start_matches("file://")
                    .trim_start_matches("file:");
                Ok(GenericKeyStore::Fs(FilesystemKeystore::open(path)?))
            }
            otherwise => Err(Error::UnsupportedKeystoreUri(otherwise.to_string())),
        }
    }

    /// Returns the first Sr25519 signer keypair from the keystore.
    ///
    /// # Errors
    ///
    /// * No sr25519 keypair is found in the keystore.
    /// * The keypair seed is invalid.
    #[doc(alias = "sr25519_signer")]
    #[cfg(any(feature = "std", feature = "wasm"))]
    pub fn first_sr25519_signer(&self) -> Result<TanglePairSigner<sp_core::sr25519::Pair>, Error> {
        self.keystore()?.sr25519_key().map_err(Error::Keystore)
    }

    /// Returns the first ECDSA signer keypair from the keystore.
    ///
    /// # Errors
    ///
    /// * No ECDSA keypair is found in the keystore.
    /// * The keypair seed is invalid.
    #[doc(alias = "ecdsa_signer")]
    #[cfg(any(feature = "std", feature = "wasm"))]
    pub fn first_ecdsa_signer(&self) -> Result<TanglePairSigner<sp_core::ecdsa::Pair>, Error> {
        self.keystore()?.ecdsa_key().map_err(Error::Keystore)
    }

    /// Returns the first ED25519 signer keypair from the keystore.
    ///
    /// # Errors
    ///
    /// * No ED25519 keypair is found in the keystore.
    /// * The keypair seed is invalid.
    #[doc(alias = "ed25519_signer")]
    #[cfg(any(feature = "std", feature = "wasm"))]
    pub fn first_ed25519_signer(&self) -> Result<TanglePairSigner<sp_core::ed25519::Pair>, Error> {
        self.keystore()?.ed25519_key().map_err(Error::Keystore)
    }

    /// Returns the first BLS BN254 signer keypair from the keystore.
    ///
    /// # Errors
    ///
    /// This function will return an error if no BLS BN254 keypair is found in the keystore.
    #[doc(alias = "bls_bn254_signer")]
    #[cfg(any(feature = "std", feature = "wasm"))]
    pub fn first_bls_bn254_signer(&self) -> Result<crypto_bls::BlsKeyPair, Error> {
        self.keystore()?.bls_bn254_key().map_err(Error::Keystore)
    }

    /// Returns whether the gadget should run in registration mode.
    #[must_use]
    pub const fn should_run_registration(&self) -> bool {
        self.is_registration
    }

    /// Returns a new [`subxt::OnlineClient`] for the Tangle.
    ///
    /// # Errors
    /// This function will return an error if we are unable to connect to the Tangle RPC endpoint.
    #[cfg(any(feature = "std", feature = "wasm"))]
    pub async fn client(&self) -> Result<crate::clients::tangle::runtime::TangleClient, Error> {
        let client =
            subxt::OnlineClient::<crate::clients::tangle::runtime::TangleConfig>::from_url(
                self.ws_rpc_endpoint.clone(),
            )
            .await?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        ContextConfig::command().debug_assert();
    }
}
