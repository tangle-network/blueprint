#![allow(unused_variables, unreachable_code)]

use std::path::PathBuf;

use crate::error::ConfigError;
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use blueprint_keystore::{Keystore, KeystoreConfig};
use clap::Parser;
use core::fmt::{Debug, Display};
use core::str::FromStr;
#[cfg(feature = "networking")]
pub use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use url::Url;

use blueprint_core::config::{
    BlueprintCliCoreSettings, BlueprintEnvironment as CoreBlueprintEnvironment, BlueprintSettings,
    ContextConfig as CoreContextConfig, Protocol, SupportedChains,
};

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
    /// For example, [`TangleProtocolSettings`] will return `"tangle"`.
    ///
    /// [`TangleProtocolSettings`]: crate::tangle::config::TangleProtocolSettings
    fn protocol(&self) -> &'static str;
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize)]
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

    fn protocol(&self) -> &'static str {
        match self {
            #[cfg(feature = "tangle")]
            ProtocolSettings::Tangle(val) => val.protocol(),
            #[cfg(feature = "eigenlayer")]
            ProtocolSettings::Eigenlayer(val) => val.protocol(),
            #[cfg(feature = "symbiotic")]
            ProtocolSettings::Symbiotic => "symbiotic",
            _ => unreachable!("should be exhaustive"),
        }
    }
}

impl ProtocolSettings {
    /// Attempt to extract the [`TangleProtocolSettings`]
    ///
    /// # Errors
    ///
    /// `self` is not [`ProtocolSettings::Tangle`]
    ///
    /// [`TangleProtocolSettings`]: crate::tangle::config::TangleProtocolSettings
    #[cfg(feature = "tangle")]
    #[allow(clippy::match_wildcard_for_single_variants)]
    pub fn tangle(&self) -> Result<&crate::tangle::config::TangleProtocolSettings, ConfigError> {
        match self {
            Self::Tangle(settings) => Ok(settings),
            _ => Err(ConfigError::UnexpectedProtocol("Tangle")),
        }
    }

    /// Attempt to extract the [`EigenlayerProtocolSettings`]
    ///
    /// # Errors
    ///
    /// `self` is not [`ProtocolSettings::Eigenlayer`]
    ///
    /// [`EigenlayerProtocolSettings`]: crate::eigenlayer::config::EigenlayerProtocolSettings
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
#[derive(Debug, Clone, Default)]
pub struct BlueprintEnvironment {
    pub core_env: blueprint_core::config::BlueprintEnvironment,
    pub protocol_settings: ProtocolSettings,
}

impl BlueprintEnvironment {
    /// Loads the [`BlueprintEnvironment`] from the current environment.
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the required environment variables are missing.
    pub fn load() -> Result<Self, ConfigError> {
        let cli_config = CoreContextConfig::parse(); // Use alias
        Self::load_with_config(cli_config)
    }

    /// Loads the [`BlueprintEnvironment`] from the given [`CoreContextConfig`]
    ///
    /// # Errors
    ///
    /// This function will return an error if any of the required environment variables are missing.
    pub fn load_with_config(cli_config: CoreContextConfig) -> Result<Self, ConfigError> {
        // Use alias
        let core_settings = match cli_config.blueprint_core_settings {
            BlueprintCliCoreSettings::Run(settings) => settings, // BlueprintCliCoreSettings is directly imported
            _ => {
                return Err(ConfigError::InvalidArgument(
                    "Expected Run subcommand for core settings".into(),
                ));
            }
        };
        load_inner(core_settings)
    }

    // TODO: this shouldn't be exclusive to the std feature
    #[cfg(feature = "std")]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // TODO: Should return errors
    pub fn keystore(&self) -> Keystore {
        // Updated to use self.core_env
        let config = KeystoreConfig::new().fs_root(self.core_env.keystore_uri.clone());
        #[cfg(feature = "tangle")]
        let substrate_keystore =
            sc_keystore::LocalKeystore::open(self.core_env.keystore_uri.clone(), None)
                .expect("Failed to open keystore");
        #[cfg(feature = "tangle")]
        let config = config.substrate(blueprint_std::sync::Arc::new(substrate_keystore));
        Keystore::new(config).expect("Failed to create keystore")
    }
}

fn load_inner(core_settings: BlueprintSettings) -> Result<BlueprintEnvironment, ConfigError> {
    // BlueprintSettings is directly imported
    let core_env = CoreBlueprintEnvironment {
        // Use alias
        http_rpc_endpoint: core_settings.http_rpc_url.to_string(),
        ws_rpc_endpoint: core_settings.ws_rpc_url.to_string(),
        keystore_uri: core_settings.keystore_uri.clone(),
        test_mode: core_settings.test_mode,
        data_dir: std::env::var("BLUEPRINT_DATA_DIR").ok().map(PathBuf::from),

        #[cfg(feature = "networking")]
        bootnodes: core_settings.bootnodes.clone().unwrap_or_default(),
        #[cfg(feature = "networking")]
        network_bind_port: core_settings.network_bind_port.unwrap_or_default(),
        #[cfg(feature = "networking")]
        enable_mdns: core_settings.enable_mdns,
        #[cfg(feature = "networking")]
        enable_kademlia: core_settings.enable_kademlia,
        #[cfg(feature = "networking")]
        target_peer_count: core_settings.target_peer_count.unwrap_or(24),
    };

    let runner_protocol_settings = crate::ProtocolSettings::load(&core_settings)?;

    Ok(BlueprintEnvironment {
        core_env,
        protocol_settings: runner_protocol_settings,
    })
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
        #[cfg(not(feature = "tangle"))]
        use blueprint_keystore::crypto::ed25519::Ed25519Zebra as LibP2PKeyType;
        #[cfg(feature = "tangle")]
        use blueprint_keystore::crypto::sp_core::SpEd25519 as LibP2PKeyType;

        let keystore_config =
            blueprint_keystore::KeystoreConfig::new().fs_root(&self.core_env.keystore_uri);
        let keystore = blueprint_keystore::Keystore::new(keystore_config)?;
        let ed25519_pub_key = keystore.first_local::<LibP2PKeyType>()?;
        let ed25519_pair = keystore.get_secret::<LibP2PKeyType>(&ed25519_pub_key)?;

        #[cfg(feature = "tangle")]
        let network_identity = libp2p::identity::Keypair::ed25519_from_bytes(ed25519_pair.seed())
            .expect("should be valid");

        #[cfg(not(feature = "tangle"))]
        let network_identity = {
            // `ed25519_from_bytes` takes an `AsMut<[u8]>` for seemingly no reason??
            let bytes = ed25519_pair.0.as_ref().to_vec();
            libp2p::identity::Keypair::ed25519_from_bytes(bytes).expect("should be valid")
        };

        let ecdsa_pub_key = keystore.first_local::<K>()?;
        let ecdsa_pair = keystore.get_secret::<K>(&ecdsa_pub_key)?;

        let listen_addr: Multiaddr =
            format!("/ip4/0.0.0.0/tcp/{}", self.core_env.network_bind_port)
                .parse()
                .expect("valid multiaddr; qed");

        let network_name: String = network_name.into();
        let network_config = blueprint_networking::NetworkConfig {
            instance_id: network_name.clone(),
            network_name,
            instance_key_pair: ecdsa_pair,
            local_key: network_identity,
            listen_addr,
            target_peer_count: self.core_env.target_peer_count,
            bootstrap_peers: self.core_env.bootnodes.clone(),
            enable_mdns: self.core_env.enable_mdns,
            enable_kademlia: self.core_env.enable_kademlia,
            using_evm_address_for_handshake_verification,
        };

        Ok(network_config)
    }
}
