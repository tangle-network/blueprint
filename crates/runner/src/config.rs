#![allow(unused_variables, unreachable_code)]

#[cfg(feature = "tls")]
use std::path::Path;
use std::path::PathBuf;

use crate::error::ConfigError;
use alloc::string::String;
#[cfg(feature = "tls")]
use blueprint_auth::models::TlsProfile;
#[cfg(feature = "tls")]
use blueprint_auth::tls_envelope::{TlsEnvelope, TlsEnvelopeKey};
#[cfg(feature = "std")]
use blueprint_keystore::{Keystore, KeystoreConfig};
use blueprint_manager_bridge::client::Bridge;
use clap::Parser;
use core::fmt::{Debug, Display};
use core::str::FromStr;
#[cfg(feature = "networking")]
pub use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

pub trait ProtocolSettingsT: Sized + 'static {
    /// Load the protocol-specific settings from the given [`BlueprintSettings`].
    ///
    /// # Errors
    ///
    /// Failure is dependent on the protocol being used. The most likely culprit will be missing
    /// certain variables.
    fn load(settings: BlueprintSettings)
    -> Result<Self, Box<dyn core::error::Error + Send + Sync>>;

    /// Get the protocol name as a `str`
    ///
    /// For example, [`TangleProtocolSettings`](crate::tangle::config::TangleProtocolSettings) will return `"tangle"`.
    fn protocol_name(&self) -> &'static str;

    fn protocol(&self) -> Protocol;
}

/// The protocol on which a blueprint will be executed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(
    all(
        feature = "std",
        any(feature = "tangle", feature = "eigenlayer", feature = "symbiotic")
    ),
    derive(clap::ValueEnum),
    clap(rename_all = "lowercase")
)]
#[non_exhaustive]
pub enum Protocol {
    #[cfg(feature = "tangle")]
    #[cfg_attr(feature = "std", value(alias = "tangle-evm"))]
    Tangle,
    #[cfg(feature = "eigenlayer")]
    Eigenlayer,
    #[cfg(feature = "symbiotic")]
    Symbiotic,
}

impl Protocol {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match self {
            #[cfg(feature = "tangle")]
            Self::Tangle => "tangle",
            #[cfg(feature = "eigenlayer")]
            Self::Eigenlayer => "eigenlayer",
            #[cfg(feature = "symbiotic")]
            Self::Symbiotic => "symbiotic",
            _ => unreachable!("should be exhaustive"),
        }
    }
}

impl core::fmt::Display for Protocol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolSettings {
    #[default]
    None,
    #[cfg(feature = "tangle")]
    Tangle(crate::tangle::config::TangleProtocolSettings),
    #[cfg(feature = "eigenlayer")]
    Eigenlayer(crate::eigenlayer::config::EigenlayerProtocolSettings),
    #[cfg(feature = "symbiotic")]
    Symbiotic,
}

impl ProtocolSettingsT for ProtocolSettings {
    fn load(
        settings: BlueprintSettings,
    ) -> Result<Self, Box<dyn core::error::Error + Send + Sync>> {
        #[allow(unreachable_patterns)]
        let protocol_settings = match settings.protocol {
            #[cfg(feature = "tangle")]
            Some(Protocol::Tangle) => {
                use crate::tangle::config::TangleProtocolSettings;
                let settings = TangleProtocolSettings::load(settings)?;
                ProtocolSettings::Tangle(settings)
            }
            #[cfg(feature = "eigenlayer")]
            Some(Protocol::Eigenlayer) => {
                use crate::eigenlayer::config::EigenlayerProtocolSettings;
                let settings = EigenlayerProtocolSettings::load(settings)?;
                ProtocolSettings::Eigenlayer(settings)
            }
            #[cfg(feature = "symbiotic")]
            Some(Protocol::Symbiotic) => {
                todo!()
            }
            None => ProtocolSettings::None,
            _ => unreachable!("should be exhaustive"),
        };

        Ok(protocol_settings)
    }

    fn protocol_name(&self) -> &'static str {
        match self {
            #[cfg(feature = "tangle")]
            ProtocolSettings::Tangle(val) => val.protocol_name(),
            #[cfg(feature = "eigenlayer")]
            ProtocolSettings::Eigenlayer(val) => val.protocol_name(),
            #[cfg(feature = "symbiotic")]
            ProtocolSettings::Symbiotic => "symbiotic",
            _ => unreachable!("should be exhaustive"),
        }
    }

    fn protocol(&self) -> Protocol {
        match self {
            ProtocolSettings::None => unreachable!(),
            #[cfg(feature = "tangle")]
            ProtocolSettings::Tangle(_) => Protocol::Tangle,
            #[cfg(feature = "eigenlayer")]
            ProtocolSettings::Eigenlayer(_) => Protocol::Eigenlayer,
            #[cfg(feature = "symbiotic")]
            ProtocolSettings::Symbiotic => Protocol::Symbiotic,
        }
    }
}

impl ProtocolSettings {
    /// Attempt to extract the [`TangleProtocolSettings`](crate::tangle::config::TangleProtocolSettings)
    ///
    /// # Errors
    ///
    /// `self` is not [`ProtocolSettings::Tangle`]
    #[cfg(feature = "tangle")]
    #[allow(clippy::match_wildcard_for_single_variants)]
    pub fn tangle(
        &self,
    ) -> Result<&crate::tangle::config::TangleProtocolSettings, ConfigError> {
        match self {
            Self::Tangle(settings) => Ok(settings),
            _ => Err(ConfigError::UnexpectedProtocol("Tangle")),
        }
    }

    /// Attempt to extract the [`EigenlayerProtocolSettings`](crate::eigenlayer::config::EigenlayerProtocolSettings)
    ///
    /// # Errors
    ///
    /// `self` is not [`ProtocolSettings::Eigenlayer`]
    #[cfg(feature = "eigenlayer")]
    #[allow(clippy::match_wildcard_for_single_variants)]
    pub fn eigenlayer(
        &self,
    ) -> Result<&crate::eigenlayer::config::EigenlayerProtocolSettings, ConfigError> {
        match self {
            Self::Eigenlayer(settings) => Ok(settings),
            _ => Err(ConfigError::UnexpectedProtocol("Eigenlayer")),
        }
    }

    // TODO
    // /// Attempt to extract the [`SymbioticContractAddresses`]
    // ///
    // /// # Errors
    // ///
    // /// `self` is not [`ProtocolSettings::Symbiotic`]
    // ///
    // /// [`SymbioticContractAddresses`]: crate::symbiotic::config::SymbioticContractAddresses
    // #[cfg(feature = "symbiotic")]
    // #[allow(clippy::match_wildcard_for_single_variants)]
    // pub fn symbiotic(&self) -> Result<(), ConfigError> {
    //     todo!()
    // }
}

/// Description of the environment in which the blueprint is running
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintEnvironment {
    /// HTTP RPC endpoint for host restaking network (Tangle / Ethereum (Eigenlayer or Symbiotic)).
    pub http_rpc_endpoint: Url,
    /// WS RPC endpoint for host restaking network (Tangle / Ethereum (Eigenlayer or Symbiotic)).
    pub ws_rpc_endpoint: Url,
    /// The keystore URI for the blueprint
    pub keystore_uri: String,
    /// Data directory exclusively for this blueprint
    pub data_dir: PathBuf,
    /// Protocol-specific settings
    pub protocol_settings: ProtocolSettings,
    /// Whether the blueprint is in test mode
    pub test_mode: bool,
    /// When true, avoid on-chain submissions from the runtime (dry run).
    #[serde(default)]
    pub dry_run: bool,
    pub bridge_socket_path: Option<PathBuf>,
    #[serde(skip)]
    bridge: Arc<Mutex<Option<Arc<Bridge>>>>,
    /// Indicates that the blueprint is running in preregistration mode.
    pub registration_mode: bool,
    /// When true, the runner will only capture registration inputs and exit without registering.
    pub registration_capture_only: bool,
    registration_output_path: Option<PathBuf>,

    /// KMS HTTP endpoint
    #[cfg(feature = "tee")]
    pub kms_url: Url,

    #[cfg(feature = "networking")]
    pub bootnodes: Vec<Multiaddr>,
    /// The port to bind the network to
    #[cfg(feature = "networking")]
    pub network_bind_port: u16,
    #[cfg(feature = "networking")]
    pub enable_mdns: bool,
    /// Whether to enable Kademlia
    #[cfg(feature = "networking")]
    pub enable_kademlia: bool,
    /// The target number of peers to connect to
    #[cfg(feature = "networking")]
    pub target_peer_count: u32,

    // TLS configuration
    #[cfg(feature = "tls")]
    pub tls_profile: Option<TlsProfile>,
}

impl Default for BlueprintEnvironment {
    fn default() -> Self {
        Self {
            http_rpc_endpoint: default_http_rpc_url(),
            ws_rpc_endpoint: default_ws_rpc_url(),
            keystore_uri: String::default(),
            data_dir: PathBuf::default(),
            protocol_settings: ProtocolSettings::default(),
            test_mode: false,
            dry_run: false,
            bridge_socket_path: None,
            bridge: Arc::new(Mutex::new(None)),
            registration_mode: false,
            registration_capture_only: false,
            registration_output_path: None,

            #[cfg(feature = "tee")]
            kms_url: default_kms_url(),

            #[cfg(feature = "networking")]
            bootnodes: Vec::new(),
            #[cfg(feature = "networking")]
            network_bind_port: 0,
            #[cfg(feature = "networking")]
            enable_mdns: false,
            #[cfg(feature = "networking")]
            enable_kademlia: false,
            #[cfg(feature = "networking")]
            target_peer_count: 0,

            #[cfg(feature = "tls")]
            tls_profile: None,
        }
    }
}

impl BlueprintEnvironment {
    /// Loads the [`BlueprintEnvironment`] from the current environment.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the required environment variables are missing.
    pub fn load() -> Result<BlueprintEnvironment, ConfigError> {
        let config = ContextConfig::parse();
        Self::load_with_config(config)
    }

    /// Loads the [`BlueprintEnvironment`] from the given [`ContextConfig`]
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the required environment variables are missing.
    pub fn load_with_config(config: ContextConfig) -> Result<BlueprintEnvironment, ConfigError> {
        load_inner(config)
    }

    // TODO: this shouldn't be exclusive to the std feature
    #[cfg(feature = "std")]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // TODO: Should return errors
    pub fn keystore(&self) -> Keystore {
        let config = KeystoreConfig::new().fs_root(self.keystore_uri.clone());
        Keystore::new(config).expect("Failed to create keystore")
    }

    /// Returns true if preregistration mode is enabled.
    #[must_use]
    pub fn registration_mode(&self) -> bool {
        self.registration_mode
    }

    /// Returns true if the runner should only capture inputs and exit.
    #[must_use]
    pub fn registration_capture_only(&self) -> bool {
        self.registration_capture_only
    }

    /// Returns the expected registration output path.
    #[must_use]
    pub fn registration_output_path(&self) -> PathBuf {
        self.registration_output_path
            .clone()
            .unwrap_or_else(|| self.data_dir.join("registration_inputs.bin"))
    }
}

fn load_inner(config: ContextConfig) -> Result<BlueprintEnvironment, ConfigError> {
    let ContextConfig {
        blueprint_core_settings: BlueprintCliCoreSettings::Run(settings),
        ..
    } = config;

    let data_dir = settings.data_dir.clone();
    let test_mode = settings.test_mode;
    let dry_run = settings.dry_run;
    let http_rpc_url = settings.http_rpc_url.clone();
    let ws_rpc_url = settings.ws_rpc_url.clone();
    let keystore_uri = settings.keystore_uri.clone();
    let bridge_socket_path = settings.bridge_socket_path.clone();
    let registration_mode = env_flag("REGISTRATION_MODE_ON");
    let registration_capture_only = env_flag("REGISTRATION_CAPTURE_ONLY");
    let registration_output_path = std::env::var("REGISTRATION_OUTPUT_PATH")
        .ok()
        .map(PathBuf::from)
        .or_else(|| registration_mode.then(|| data_dir.join("registration_inputs.bin")));

    #[cfg(feature = "tee")]
    let kms_url = settings.kms_url.clone();

    #[cfg(feature = "networking")]
    let bootnodes = settings.bootnodes.clone().unwrap_or_default();
    #[cfg(feature = "networking")]
    let network_bind_port = settings.network_bind_port.unwrap_or_default();
    #[cfg(feature = "networking")]
    let enable_mdns = settings.enable_mdns;
    #[cfg(feature = "networking")]
    let enable_kademlia = settings.enable_kademlia;
    #[cfg(feature = "networking")]
    let target_peer_count = settings.target_peer_count.unwrap_or(24);

    // Create TLS profile before settings is moved
    #[cfg(feature = "tls")]
    let tls_profile = create_tls_profile(&settings)?;

    let protocol_settings = ProtocolSettings::load(settings)?;

    Ok(BlueprintEnvironment {
        test_mode,
        dry_run,
        http_rpc_endpoint: http_rpc_url,
        ws_rpc_endpoint: ws_rpc_url,
        keystore_uri,
        data_dir,
        protocol_settings,
        bridge_socket_path,
        bridge: Arc::new(Mutex::new(None)),
        registration_mode,
        registration_capture_only,
        registration_output_path,

        #[cfg(feature = "tee")]
        kms_url,

        #[cfg(feature = "networking")]
        bootnodes,
        #[cfg(feature = "networking")]
        network_bind_port,
        #[cfg(feature = "networking")]
        enable_mdns,
        #[cfg(feature = "networking")]
        enable_kademlia,
        #[cfg(feature = "networking")]
        target_peer_count,
        #[cfg(feature = "tls")]
        tls_profile,
    })
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            matches!(normalized.as_str(), "1" | "true" | "yes")
        })
        .unwrap_or(false)
}

#[cfg(feature = "tls")]
fn load_tls_envelope(settings: &BlueprintSettings) -> Result<TlsEnvelope, ConfigError> {
    if let Ok(key_hex) = std::env::var("TLS_ENVELOPE_KEY") {
        let key = TlsEnvelopeKey::from_hex(&key_hex).map_err(|e| {
            ConfigError::InvalidTlsConfig(format!("Invalid TLS_ENVELOPE_KEY value: {e}"))
        })?;
        return Ok(TlsEnvelope::with_key(key));
    }

    let key_path = if let Ok(path) = std::env::var("TLS_ENVELOPE_KEY_PATH") {
        Some(PathBuf::from(path))
    } else {
        let default_path = settings.data_dir.join(".tls_envelope_key");
        if default_path.exists() {
            Some(default_path)
        } else {
            None
        }
    };

    if let Some(path) = key_path {
        let key_bytes = std::fs::read(&path).map_err(|e| ConfigError::IoError(e.to_string()))?;
        if key_bytes.len() != 32 {
            return Err(ConfigError::InvalidTlsConfig(format!(
                "TLS envelope key at {} must be 32 bytes but was {}",
                path.display(),
                key_bytes.len()
            )));
        }
        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_bytes);
        return Ok(TlsEnvelope::with_key(TlsEnvelopeKey::from_bytes(key_array)));
    }

    Err(ConfigError::MissingTlsConfig(
        "TLS_ENVELOPE_KEY or TLS_ENVELOPE_KEY_PATH must be set when TLS is enabled".into(),
    ))
}

#[cfg(feature = "tls")]
fn encrypt_file(path: &Path, envelope: &TlsEnvelope) -> Result<Vec<u8>, ConfigError> {
    let data = std::fs::read(path).map_err(|e| ConfigError::IoError(e.to_string()))?;
    envelope
        .encrypt(&data)
        .map_err(|e| ConfigError::Other(Box::new(e)))
}

#[cfg(feature = "tls")]
fn encrypt_optional_file(
    path: Option<&PathBuf>,
    envelope: &TlsEnvelope,
) -> Result<Vec<u8>, ConfigError> {
    if let Some(path) = path {
        encrypt_file(path.as_path(), envelope)
    } else {
        Ok(Vec::new())
    }
}

#[cfg(feature = "tls")]
/// Create a TLS profile from the given settings
///
/// # Errors
/// Returns [`ConfigError::MissingTlsConfig`] when mandatory TLS files or keys are absent, and
/// propagates IO or encryption failures encountered while loading and envelope-encrypting TLS assets.
pub fn create_tls_profile(settings: &BlueprintSettings) -> Result<Option<TlsProfile>, ConfigError> {
    if !settings.tls_enabled {
        return Ok(None);
    }

    let envelope = load_tls_envelope(settings)?;

    let server_cert_path = settings.tls_server_cert_path.as_ref().ok_or_else(|| {
        ConfigError::MissingTlsConfig(
            "Server certificate path is required when TLS is enabled".to_string(),
        )
    })?;
    let encrypted_server_cert = encrypt_file(server_cert_path.as_path(), &envelope)?;

    let server_key_path = settings.tls_server_key_path.as_ref().ok_or_else(|| {
        ConfigError::MissingTlsConfig("Server key path is required when TLS is enabled".to_string())
    })?;
    let encrypted_server_key = encrypt_file(server_key_path.as_path(), &envelope)?;

    let client_ca_path = settings.tls_client_ca_path.as_ref().ok_or_else(|| {
        ConfigError::MissingTlsConfig(
            "Client CA bundle path is required when TLS is enabled".to_string(),
        )
    })?;
    let encrypted_client_ca_bundle = encrypt_file(client_ca_path.as_path(), &envelope)?;

    let encrypted_upstream_ca_bundle =
        encrypt_optional_file(settings.tls_upstream_ca_path.as_ref(), &envelope)?;

    let encrypted_upstream_client_cert =
        encrypt_optional_file(settings.tls_upstream_client_cert_path.as_ref(), &envelope)?;

    let encrypted_upstream_client_key =
        encrypt_optional_file(settings.tls_upstream_client_key_path.as_ref(), &envelope)?;

    Ok(Some(TlsProfile {
        tls_enabled: true,
        require_client_mtls: settings.tls_require_client_mtls,
        encrypted_server_cert,
        encrypted_server_key,
        encrypted_client_ca_bundle,
        encrypted_upstream_ca_bundle,
        encrypted_upstream_client_cert,
        encrypted_upstream_client_key,
        client_cert_ttl_hours: settings.tls_client_cert_ttl_hours,
        sni: settings.tls_sni.clone(),
        subject_alt_name_template: settings.tls_subject_alt_name_template.clone(),
        allowed_dns_names: settings.tls_allowed_dns_names.clone().unwrap_or_default(),
    }))
}

impl BlueprintEnvironment {
    /// Returns the bridge to the blueprint manager.
    ///
    /// NOTE: Only the first call will attempt a connection. Future calls will be cached, so this
    ///       is cheap to call repeatedly.
    ///
    /// # Errors
    ///
    /// - See [`Bridge::connect()`]
    pub async fn bridge(&self) -> Result<Arc<Bridge>, blueprint_manager_bridge::Error> {
        let mut guard = self.bridge.lock().await;
        if let Some(bridge) = &*guard {
            return Ok(bridge.clone());
        }

        let bridge = Arc::new(Bridge::connect(self.bridge_socket_path.as_deref()).await?);
        *guard = Some(bridge.clone());
        Ok(bridge)
    }
}

#[cfg(feature = "networking")]
impl BlueprintEnvironment {
    /// Start a p2p network with the given `network_config`
    ///
    /// # Errors
    ///
    /// See [`NetworkService::new()`]
    ///
    /// [`NetworkService::new()`]: blueprint_networking::NetworkService::new
    #[cfg(feature = "networking")]
    pub fn libp2p_start_network<K: blueprint_crypto::KeyType>(
        &self,
        network_config: blueprint_networking::NetworkConfig<K>,
        allowed_keys: blueprint_networking::service::AllowedKeys<K>,
        allowed_keys_rx: crossbeam_channel::Receiver<blueprint_networking::AllowedKeys<K>>,
    ) -> Result<
        blueprint_networking::service_handle::NetworkServiceHandle<K>,
        crate::error::RunnerError,
    > {
        let networking_service = blueprint_networking::NetworkService::new(
            network_config,
            allowed_keys,
            allowed_keys_rx,
        )?;

        let handle = networking_service.start();

        Ok(handle)
    }

    /// Returns a new `NetworkConfig` for the current environment.
    ///
    /// # Errors
    ///
    /// Missing the following keys in the keystore:
    ///
    /// * `Ed25519`
    /// * `ECDSA`
    #[cfg(feature = "networking")]
    #[allow(clippy::missing_panics_doc)] // Known good Multiaddr
    pub fn libp2p_network_config<K: blueprint_crypto::KeyType>(
        &self,
        network_name: impl Into<String>,
        using_evm_address_for_handshake_verification: bool,
    ) -> Result<blueprint_networking::NetworkConfig<K>, crate::error::RunnerError> {
        use blueprint_keystore::backends::Backend;
        use blueprint_keystore::crypto::ed25519::Ed25519Zebra as LibP2PKeyType;

        let keystore_config = blueprint_keystore::KeystoreConfig::new().fs_root(&self.keystore_uri);
        let keystore = blueprint_keystore::Keystore::new(keystore_config)?;
        let ed25519_pub_key = keystore.first_local::<LibP2PKeyType>()?;
        let ed25519_pair = keystore.get_secret::<LibP2PKeyType>(&ed25519_pub_key)?;

        let network_identity = {
            // `ed25519_from_bytes` takes an `AsMut<[u8]>` for seemingly no reason??
            let bytes = ed25519_pair.0.as_ref().to_vec();
            libp2p::identity::Keypair::ed25519_from_bytes(bytes).expect("should be valid")
        };

        let ecdsa_pub_key = keystore.first_local::<K>()?;
        let ecdsa_pair = keystore.get_secret::<K>(&ecdsa_pub_key)?;

        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", self.network_bind_port)
            .parse()
            .expect("valid multiaddr; qed");

        let network_name: String = network_name.into();
        let network_config = blueprint_networking::NetworkConfig {
            instance_id: network_name.clone(),
            network_name,
            instance_key_pair: ecdsa_pair,
            local_key: network_identity,
            listen_addr,
            target_peer_count: self.target_peer_count,
            bootstrap_peers: self.bootnodes.clone(),
            enable_mdns: self.enable_mdns,
            enable_kademlia: self.enable_kademlia,
            using_evm_address_for_handshake_verification,
        };

        Ok(network_config)
    }
}

#[derive(Debug, Default, Clone, clap::Parser, Serialize, Deserialize)]
#[command(name = "General CLI Context")]
pub struct ContextConfig {
    /// Pass through arguments to another command
    #[command(subcommand)]
    pub blueprint_core_settings: BlueprintCliCoreSettings,
}

impl ContextConfig {
    /// Creates a new context config with the given parameters
    ///
    /// # Arguments
    /// - `http_rpc_url`: The HTTP RPC URL of the target chain
    /// - `ws_rpc_url`: The WebSocket RPC URL of the target chain
    /// - `use_secure_url`: Whether to use a secure URL (ws/wss and http/https)
    /// - `keystore_uri`: The keystore URI as a string
    /// - `chain`: The [`chain`](SupportedChains)
    /// - `protocol`: The [`Protocol`]
    /// - `protocol_settings`: The protocol-specific settings
    #[allow(
        clippy::too_many_arguments,
        clippy::match_wildcard_for_single_variants,
        clippy::needless_pass_by_value
    )]
    #[must_use]
    pub fn create_config(
        http_rpc_url: Url,
        ws_rpc_url: Url,
        keystore_uri: String,
        keystore_password: Option<String>,
        data_dir: PathBuf,
        bridge_socket_path: Option<PathBuf>,
        chain: SupportedChains,
        protocol: Protocol,
        protocol_settings: ProtocolSettings,
    ) -> Self {
        // Eigenlayer addresses
        #[cfg(feature = "eigenlayer")]
        let eigenlayer_settings = match &protocol_settings {
            ProtocolSettings::Eigenlayer(settings) => Some(settings),
            _ => None,
        };
        #[cfg(feature = "eigenlayer")]
        let allocation_manager = eigenlayer_settings
            .as_ref()
            .map(|s| s.allocation_manager_address);
        #[cfg(feature = "eigenlayer")]
        let registry_coordinator = eigenlayer_settings
            .as_ref()
            .map(|s| s.registry_coordinator_address);
        #[cfg(feature = "eigenlayer")]
        let operator_state_retriever = eigenlayer_settings
            .as_ref()
            .map(|s| s.operator_state_retriever_address);
        #[cfg(feature = "eigenlayer")]
        let delegation_manager = eigenlayer_settings
            .as_ref()
            .map(|s| s.delegation_manager_address);
        #[cfg(feature = "eigenlayer")]
        let service_manager = eigenlayer_settings
            .as_ref()
            .map(|s| s.service_manager_address);
        #[cfg(feature = "eigenlayer")]
        let stake_registry = eigenlayer_settings
            .as_ref()
            .map(|s| s.stake_registry_address);
        #[cfg(feature = "eigenlayer")]
        let strategy_manager = eigenlayer_settings
            .as_ref()
            .map(|s| s.strategy_manager_address);
        #[cfg(feature = "eigenlayer")]
        let avs_directory = eigenlayer_settings
            .as_ref()
            .map(|s| s.avs_directory_address);
        #[cfg(feature = "eigenlayer")]
        let rewards_coordinator = eigenlayer_settings
            .as_ref()
            .map(|s| s.rewards_coordinator_address);
        #[cfg(feature = "eigenlayer")]
        let permission_controller = eigenlayer_settings
            .as_ref()
            .map(|s| s.permission_controller_address);
        #[cfg(feature = "eigenlayer")]
        let strategy = eigenlayer_settings.as_ref().map(|s| s.strategy_address);

        #[cfg(feature = "networking")]
        let enable_mdns = cfg!(debug_assertions);
        #[cfg(feature = "networking")]
        let enable_kademlia = !cfg!(debug_assertions);

        ContextConfig {
            blueprint_core_settings: BlueprintCliCoreSettings::Run(BlueprintSettings {
                test_mode: false,
                dry_run: false,
                http_rpc_url,
                #[cfg(feature = "networking")]
                bootnodes: None,
                #[cfg(feature = "networking")]
                network_bind_port: None,
                #[cfg(feature = "networking")]
                enable_mdns,
                #[cfg(feature = "networking")]
                enable_kademlia,
                #[cfg(feature = "networking")]
                target_peer_count: None,
                #[cfg(feature = "tee")]
                kms_url: default_kms_url(),
                keystore_uri,
                data_dir,
                chain,
                verbose: 3,
                pretty: true,
                keystore_password,
                protocol: Some(protocol),
                bridge_socket_path,
                ws_rpc_url,
                #[cfg(feature = "eigenlayer")]
                allocation_manager,
                #[cfg(feature = "eigenlayer")]
                registry_coordinator,
                #[cfg(feature = "eigenlayer")]
                operator_state_retriever,
                #[cfg(feature = "eigenlayer")]
                delegation_manager,
                #[cfg(feature = "eigenlayer")]
                stake_registry,
                #[cfg(feature = "eigenlayer")]
                service_manager,
                #[cfg(feature = "eigenlayer")]
                strategy_manager,
                #[cfg(feature = "eigenlayer")]
                avs_directory,
                #[cfg(feature = "eigenlayer")]
                rewards_coordinator,
                #[cfg(feature = "eigenlayer")]
                permission_controller,
                #[cfg(feature = "eigenlayer")]
                strategy,
                #[cfg(feature = "eigenlayer")]
                eigenlayer_allocation_delay: None,
                #[cfg(feature = "eigenlayer")]
                eigenlayer_deposit_amount: None,
                #[cfg(feature = "eigenlayer")]
                eigenlayer_stake_amount: None,
                #[cfg(feature = "eigenlayer")]
                eigenlayer_operator_sets: None,
                #[cfg(feature = "eigenlayer")]
                eigenlayer_staker_opt_out_window_blocks: None,
                #[cfg(feature = "eigenlayer")]
                eigenlayer_metadata_url: None,
                #[cfg(feature = "tls")]
                tls_enabled: false,
                #[cfg(feature = "tls")]
                tls_server_cert_path: None,
                #[cfg(feature = "tls")]
                tls_server_key_path: None,
                #[cfg(feature = "tls")]
                tls_client_ca_path: None,
                #[cfg(feature = "tls")]
                tls_require_client_mtls: false,
                #[cfg(feature = "tls")]
                tls_upstream_ca_path: None,
                #[cfg(feature = "tls")]
                tls_upstream_client_cert_path: None,
                #[cfg(feature = "tls")]
                tls_upstream_client_key_path: None,
                #[cfg(feature = "tls")]
                tls_client_cert_ttl_hours: 24,
                #[cfg(feature = "tls")]
                tls_sni: None,
                #[cfg(feature = "tls")]
                tls_subject_alt_name_template: None,
                #[cfg(feature = "tls")]
                tls_allowed_dns_names: None,
            }),
        }
    }

    /// Creates a new context config with the given parameters
    ///
    /// # Defaults
    /// - `target_addr`: The same host address as the given `http_rpc_url`, defaulting to 127.0.0.1 if an error occurs
    /// - `target_port`: The same port as the given `http_rpc_url`, defaulting to 0 if an error occurs
    /// - `skip_registration`: false
    /// - `keystore_password`: None
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn create_config_with_defaults(
        http_rpc_url: Url,
        ws_rpc_url: Url,
        keystore_uri: String,
        keystore_password: Option<String>,
        data_dir: PathBuf,
        bridge_socket_path: Option<PathBuf>,
        chain: SupportedChains,
        protocol: Protocol,
        protocol_settings: ProtocolSettings,
    ) -> Self {
        ContextConfig::create_config(
            http_rpc_url,
            ws_rpc_url,
            keystore_uri,
            keystore_password,
            data_dir,
            bridge_socket_path,
            chain,
            protocol,
            protocol_settings,
        )
    }

    /// Creates a new context config with defaults for Eigenlayer
    #[cfg(feature = "eigenlayer")]
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_arguments)]
    pub fn create_eigenlayer_config(
        http_rpc_url: Url,
        ws_rpc_url: Url,
        keystore_uri: String,
        keystore_password: Option<String>,
        data_dir: PathBuf,
        bridge_socket_path: Option<PathBuf>,
        chain: SupportedChains,
        eigenlayer_contract_addresses: crate::eigenlayer::config::EigenlayerProtocolSettings,
    ) -> Self {
        Self::create_config_with_defaults(
            http_rpc_url,
            ws_rpc_url,
            keystore_uri,
            keystore_password,
            data_dir,
            bridge_socket_path,
            chain,
            Protocol::Eigenlayer,
            ProtocolSettings::Eigenlayer(eigenlayer_contract_addresses),
        )
    }
}

#[derive(Debug, Clone, clap::Parser, Serialize, Deserialize)]
pub enum BlueprintCliCoreSettings {
    #[command(name = "run")]
    Run(BlueprintSettings),
}

impl Default for BlueprintCliCoreSettings {
    fn default() -> Self {
        BlueprintCliCoreSettings::Run(BlueprintSettings::default())
    }
}

#[derive(Debug, Clone, clap::Parser, Serialize, Deserialize)]
pub struct BlueprintSettings {
    #[arg(long, short = 't', env)]
    pub test_mode: bool,
    #[arg(long)]
    #[serde(default)]
    pub dry_run: bool,
    #[arg(long, env, default_value_t = default_http_rpc_url())]
    #[serde(default = "default_http_rpc_url")]
    pub http_rpc_url: Url,
    #[arg(long, env, default_value_t = default_ws_rpc_url())]
    #[serde(default = "default_ws_rpc_url")]
    pub ws_rpc_url: Url,
    #[arg(long, short = 'k', env, default_value_t = String::from("./keystore"))]
    pub keystore_uri: String,
    #[arg(long, short, env)]
    pub data_dir: PathBuf,
    #[arg(long, value_enum, env, default_value_t)]
    pub chain: SupportedChains,
    #[arg(long, short = 'v', global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Whether to use pretty logging
    #[arg(long, env)]
    pub pretty: bool,
    #[arg(long, env)]
    pub keystore_password: Option<String>,
    /// The protocol to use
    #[cfg_attr(
        any(feature = "tangle", feature = "eigenlayer", feature = "symbiotic"),
        arg(long, value_enum, env)
    )]
    #[cfg_attr(
        not(any(feature = "tangle", feature = "eigenlayer", feature = "symbiotic")),
        arg(skip)
    )]
    pub protocol: Option<Protocol>,
    #[arg(long, env)]
    pub bridge_socket_path: Option<PathBuf>,

    // ========
    // NETWORKING
    // ========
    #[cfg(feature = "networking")]
    #[arg(long, value_parser = <Multiaddr as blueprint_std::str::FromStr>::from_str, action = clap::ArgAction::Append, env)]
    #[serde(default)]
    pub bootnodes: Option<Vec<Multiaddr>>,
    #[cfg(feature = "networking")]
    #[arg(long, env)]
    #[serde(default)]
    pub network_bind_port: Option<u16>,
    #[cfg(feature = "networking")]
    #[arg(long, env)]
    #[serde(default)]
    pub enable_mdns: bool,
    #[cfg(feature = "networking")]
    #[arg(long, env)]
    #[serde(default)]
    pub enable_kademlia: bool,
    #[cfg(feature = "networking")]
    #[arg(long, env)]
    #[serde(default)]
    pub target_peer_count: Option<u32>,

    // ========
    // TEE
    // ========
    /// URL of the Key Brokerage Service (KBS)
    ///
    /// This defaults to the central KBS hosted by Tangle
    #[cfg(feature = "tee")]
    #[arg(long, env, default_value_t = default_kms_url())]
    #[serde(default = "default_kms_url")]
    pub kms_url: Url,

    // ========
    // EIGENLAYER
    // ========
    /// The address of the allocation manager
    #[cfg(feature = "eigenlayer")]
    #[arg(
        long,
        value_name = "ADDR",
        env = "ALLOCATION_MANAGER_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str()),
    )]
    pub allocation_manager: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the registry coordinator
    #[arg(
        long,
        value_name = "ADDR",
        env = "REGISTRY_COORDINATOR_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str()),
    )]
    pub registry_coordinator: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the operator state retriever
    #[arg(
        long,
        value_name = "ADDR",
        env = "OPERATOR_STATE_RETRIEVER_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub operator_state_retriever: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the delegation manager
    #[arg(
        long,
        value_name = "ADDR",
        env = "DELEGATION_MANAGER_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub delegation_manager: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the strategy manager
    #[arg(
        long,
        value_name = "ADDR",
        env = "STRATEGY_MANAGER_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub strategy_manager: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the Service Manager
    #[arg(
        long,
        value_name = "ADDR",
        env = "SERVICE_MANAGER_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub service_manager: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the Stake Registry
    #[arg(
        long,
        value_name = "ADDR",
        env = "STAKE_REGISTRY_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub stake_registry: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the AVS directory
    #[arg(
        long,
        value_name = "ADDR",
        env = "AVS_DIRECTORY_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub avs_directory: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the rewards coordinator
    #[arg(
        long,
        value_name = "ADDR",
        env = "REWARDS_COORDINATOR_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str())
    )]
    pub rewards_coordinator: Option<alloy_primitives::Address>,
    /// The address of the permission controller
    #[cfg(feature = "eigenlayer")]
    #[arg(
        long,
        value_name = "ADDR",
        env = "PERMISSION_CONTROLLER_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str()),
    )]
    pub permission_controller: Option<alloy_primitives::Address>,
    #[cfg(feature = "eigenlayer")]
    /// The address of the strategy
    #[arg(
        long,
        value_name = "ADDR",
        env = "STRATEGY_ADDRESS",
        required_if_eq("protocol", Protocol::Eigenlayer.as_str()),
    )]
    pub strategy: Option<alloy_primitives::Address>,

    // ========
    // EIGENLAYER REGISTRATION PARAMETERS
    // ========
    #[cfg(feature = "eigenlayer")]
    /// Allocation delay in blocks (default: 0)
    #[arg(long, env = "EIGENLAYER_ALLOCATION_DELAY")]
    pub eigenlayer_allocation_delay: Option<u32>,
    #[cfg(feature = "eigenlayer")]
    /// Deposit amount in wei (default: 5000 ether)
    #[arg(long, env = "EIGENLAYER_DEPOSIT_AMOUNT")]
    pub eigenlayer_deposit_amount: Option<u128>,
    #[cfg(feature = "eigenlayer")]
    /// Stake amount in wei (default: 1 ether)
    #[arg(long, env = "EIGENLAYER_STAKE_AMOUNT")]
    pub eigenlayer_stake_amount: Option<u64>,
    #[cfg(feature = "eigenlayer")]
    /// Operator sets to register for (comma-separated, default: 0)
    #[arg(long, env = "EIGENLAYER_OPERATOR_SETS", value_delimiter = ',')]
    pub eigenlayer_operator_sets: Option<Vec<u32>>,
    #[cfg(feature = "eigenlayer")]
    /// Staker opt-out window in blocks (default: 50400)
    #[arg(long, env = "EIGENLAYER_STAKER_OPT_OUT_WINDOW_BLOCKS")]
    pub eigenlayer_staker_opt_out_window_blocks: Option<u32>,
    #[cfg(feature = "eigenlayer")]
    /// Operator metadata URL
    #[arg(long, env = "EIGENLAYER_METADATA_URL")]
    pub eigenlayer_metadata_url: Option<String>,

    // ========
    // TLS CONFIGURATION
    // ========
    /// Enable TLS for service registration
    #[cfg(feature = "tls")]
    #[arg(long, env)]
    pub tls_enabled: bool,
    #[cfg(feature = "tls")]
    /// Path to server certificate file (PEM format)
    #[arg(long, env)]
    pub tls_server_cert_path: Option<PathBuf>,
    #[cfg(feature = "tls")]
    /// Path to server private key file (PEM format)
    #[arg(long, env)]
    pub tls_server_key_path: Option<PathBuf>,
    #[cfg(feature = "tls")]
    /// Path to client CA bundle file (PEM format) for mTLS
    #[arg(long, env)]
    pub tls_client_ca_path: Option<PathBuf>,
    #[cfg(feature = "tls")]
    /// Require client mTLS authentication
    #[arg(long, env)]
    pub tls_require_client_mtls: bool,
    #[cfg(feature = "tls")]
    /// Path to upstream CA bundle file (PEM format)
    #[arg(long, env)]
    pub tls_upstream_ca_path: Option<PathBuf>,
    #[cfg(feature = "tls")]
    /// Path to upstream client certificate file (PEM format)
    #[arg(long, env)]
    pub tls_upstream_client_cert_path: Option<PathBuf>,
    #[cfg(feature = "tls")]
    /// Path to upstream client private key file (PEM format)
    #[arg(long, env)]
    pub tls_upstream_client_key_path: Option<PathBuf>,
    #[cfg(feature = "tls")]
    /// Client certificate TTL in hours
    #[arg(long, env, default_value_t = 24u32)]
    pub tls_client_cert_ttl_hours: u32,
    #[cfg(feature = "tls")]
    /// SNI hostname for TLS connections
    #[arg(long, env)]
    pub tls_sni: Option<String>,
    #[cfg(feature = "tls")]
    /// Subject alternative name template
    #[arg(long, env)]
    pub tls_subject_alt_name_template: Option<String>,
    #[cfg(feature = "tls")]
    /// Allowed DNS names for TLS connections
    #[arg(long, env, value_delimiter = ',')]
    pub tls_allowed_dns_names: Option<Vec<String>>,
}

impl Default for BlueprintSettings {
    fn default() -> Self {
        Self {
            test_mode: false,
            dry_run: false,
            http_rpc_url: default_http_rpc_url(),
            ws_rpc_url: default_ws_rpc_url(),
            keystore_uri: String::new(),
            data_dir: PathBuf::new(),
            chain: SupportedChains::default(),
            verbose: 0,
            pretty: false,
            keystore_password: None,
            protocol: None,
            bridge_socket_path: None,

            // Networking
            #[cfg(feature = "networking")]
            bootnodes: None,
            #[cfg(feature = "networking")]
            network_bind_port: None,
            #[cfg(feature = "networking")]
            enable_mdns: false,
            #[cfg(feature = "networking")]
            enable_kademlia: false,
            #[cfg(feature = "networking")]
            target_peer_count: None,

            // ========
            // TEE
            // ========
            #[cfg(feature = "tee")]
            kms_url: default_kms_url(),

            // ========
            // EIGENLAYER
            // ========
            #[cfg(feature = "eigenlayer")]
            allocation_manager: None,
            #[cfg(feature = "eigenlayer")]
            registry_coordinator: None,
            #[cfg(feature = "eigenlayer")]
            operator_state_retriever: None,
            #[cfg(feature = "eigenlayer")]
            delegation_manager: None,
            #[cfg(feature = "eigenlayer")]
            service_manager: None,
            #[cfg(feature = "eigenlayer")]
            stake_registry: None,
            #[cfg(feature = "eigenlayer")]
            strategy_manager: None,
            #[cfg(feature = "eigenlayer")]
            avs_directory: None,
            #[cfg(feature = "eigenlayer")]
            rewards_coordinator: None,
            #[cfg(feature = "eigenlayer")]
            permission_controller: None,
            #[cfg(feature = "eigenlayer")]
            strategy: None,

            // ========
            // EIGENLAYER REGISTRATION PARAMETERS
            // ========
            #[cfg(feature = "eigenlayer")]
            eigenlayer_allocation_delay: None,
            #[cfg(feature = "eigenlayer")]
            eigenlayer_deposit_amount: None,
            #[cfg(feature = "eigenlayer")]
            eigenlayer_stake_amount: None,
            #[cfg(feature = "eigenlayer")]
            eigenlayer_operator_sets: None,
            #[cfg(feature = "eigenlayer")]
            eigenlayer_staker_opt_out_window_blocks: None,
            #[cfg(feature = "eigenlayer")]
            eigenlayer_metadata_url: None,

            // ========
            // TLS CONFIGURATION
            // ========
            #[cfg(feature = "tls")]
            tls_enabled: false,
            #[cfg(feature = "tls")]
            tls_server_cert_path: None,
            #[cfg(feature = "tls")]
            tls_server_key_path: None,
            #[cfg(feature = "tls")]
            tls_client_ca_path: None,
            #[cfg(feature = "tls")]
            tls_require_client_mtls: false,
            #[cfg(feature = "tls")]
            tls_upstream_ca_path: None,
            #[cfg(feature = "tls")]
            tls_upstream_client_cert_path: None,
            #[cfg(feature = "tls")]
            tls_upstream_client_key_path: None,
            #[cfg(feature = "tls")]
            tls_client_cert_ttl_hours: 24,
            #[cfg(feature = "tls")]
            tls_sni: None,
            #[cfg(feature = "tls")]
            tls_subject_alt_name_template: None,
            #[cfg(feature = "tls")]
            tls_allowed_dns_names: None,
        }
    }
}

fn default_http_rpc_url() -> Url {
    Url::from_str("http://127.0.0.1:9944").unwrap()
}

fn default_ws_rpc_url() -> Url {
    Url::from_str("ws://127.0.0.1:9944").unwrap()
}

#[cfg(feature = "tee")]
fn default_kms_url() -> Url {
    Url::from_str("https://kms.tangle.tools").unwrap()
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SupportedChains {
    #[default]
    LocalTestnet,
    LocalMainnet,
    Testnet,
    Mainnet,
}

impl FromStr for SupportedChains {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s {
            "local_testnet" => Ok(SupportedChains::LocalTestnet),
            "local_mainnet" => Ok(SupportedChains::LocalMainnet),
            "testnet" => Ok(SupportedChains::Testnet),
            "mainnet" => Ok(SupportedChains::Mainnet),
            _ => Err(format!("Invalid chain: {}", s)),
        }
    }
}

impl Display for SupportedChains {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SupportedChains::LocalTestnet => write!(f, "local_testnet"),
            SupportedChains::LocalMainnet => write!(f, "local_mainnet"),
            SupportedChains::Testnet => write!(f, "testnet"),
            SupportedChains::Mainnet => write!(f, "mainnet"),
        }
    }
}
