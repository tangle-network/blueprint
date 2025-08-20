use crate::error::{Error, Result};
use blueprint_auth::proxy::DEFAULT_AUTH_PROXY_PORT;
use blueprint_core::{error, info};
use clap::{Args, Parser};
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

mod ctx;
pub use ctx::*;

#[cfg(feature = "vm-sandbox")]
pub static DEFAULT_ADDRESS_POOL: LazyLock<ipnet::Ipv4Net> =
    LazyLock::new(|| "172.30.0.0/16".parse().unwrap());

#[derive(Debug, Parser)]
#[command(
    name = "Blueprint Manager",
    about = "A program executor that connects to the Tangle network and runs blueprints dynamically on the fly"
)]
pub struct BlueprintManagerCli {
    #[command(flatten)]
    pub config: BlueprintManagerConfig,
}

#[derive(Debug, Args, Default)]
pub struct BlueprintManagerConfig {
    #[command(flatten)]
    pub paths: Paths,
    /// The verbosity level, can be used multiple times to increase verbosity
    #[arg(long, short = 'v', action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Whether to use pretty logging
    #[arg(long)]
    pub pretty: bool,
    /// An optional unique string identifier for the blueprint manager to differentiate between multiple
    /// running instances of a `BlueprintManager` (mostly for debugging purposes)
    #[arg(long, alias = "id")]
    pub instance_id: Option<String>,
    #[arg(long, short = 't')]
    pub test_mode: bool,
    /// Whether to allow invalid GitHub attestations (binary integrity checks)
    ///
    /// This will also allow for running the manager without the GitHub CLI installed.
    #[arg(long)]
    pub allow_unchecked_attestations: bool,
    /// The preferred way to run a blueprint.
    ///
    /// This is not a guarantee that the blueprint will use this method, as there may not be a source
    /// available of this type.
    #[arg(long, short = 's', default_value_t)]
    pub preferred_source: SourceType,

    /// Options to configure the VM sandbox for native blueprints
    #[cfg(feature = "vm-sandbox")]
    #[command(flatten)]
    pub vm_sandbox_options: VmSandboxOptions,

    /// Options to configure the container sandbox for containerized blueprints
    #[cfg(feature = "containers")]
    #[command(flatten)]
    pub container_options: ContainerOptions,

    /// Authentication proxy options
    #[command(flatten)]
    pub auth_proxy_opts: AuthProxyOpts,
}

impl BlueprintManagerConfig {
    #[inline]
    #[must_use]
    pub fn blueprint_config_path(&self) -> Option<&Path> {
        self.paths.blueprint_config.as_deref()
    }

    #[inline]
    #[must_use]
    pub fn keystore_uri(&self) -> &str {
        &self.paths.keystore_uri
    }

    #[inline]
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        &self.paths.data_dir
    }

    #[inline]
    #[must_use]
    pub fn cache_dir(&self) -> &Path {
        &self.paths.cache_dir
    }

    #[inline]
    #[must_use]
    pub fn runtime_dir(&self) -> &Path {
        &self.paths.runtime_dir
    }

    #[inline]
    #[must_use]
    #[cfg(feature = "containers")]
    pub fn kube_service_port(&self) -> u16 {
        self.container_options.kube_service_port
    }
}

#[derive(Args, Debug, Clone)]
pub struct Paths {
    /// The path to the blueprint configuration file
    #[arg(short = 'c', long)]
    pub blueprint_config: Option<PathBuf>,
    /// The path to the keystore
    #[arg(short = 'k', long, default_value = "./keystore")]
    pub keystore_uri: String,
    /// The directory in which all blueprints will store their data
    #[arg(long, short = 'd', default_value = "./data")]
    pub data_dir: PathBuf,
    /// The cache directory for blueprint manager downloads
    #[arg(long, short = 'z', default_value_os_t = default_cache_dir())]
    pub cache_dir: PathBuf,
    /// The runtime directory for manager-to-blueprint sockets
    #[arg(long, short, default_value_os_t = default_runtime_dir())]
    pub runtime_dir: PathBuf,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            blueprint_config: None,
            keystore_uri: "./keystore".into(),
            data_dir: PathBuf::from("./data"),
            cache_dir: default_cache_dir(),
            runtime_dir: default_runtime_dir(),
        }
    }
}

/// Options for the VM sandbox
#[cfg(feature = "vm-sandbox")]
#[derive(Args, Debug, Clone)]
pub struct VmSandboxOptions {
    /// Disables the VM sandbox for native blueprints
    ///
    /// This should only be used for testing and never for production setups.
    #[arg(long)]
    pub no_vm: bool,
    /// The default address pool for VM TAP interfaces
    #[arg(long, default_value_t = *DEFAULT_ADDRESS_POOL)]
    pub default_address_pool: ipnet::Ipv4Net,
    /// The network interface to funnel blueprint VM traffic through
    #[arg(long)]
    pub network_interface: Option<String>,
}

#[cfg(feature = "vm-sandbox")]
impl Default for VmSandboxOptions {
    fn default() -> Self {
        Self {
            no_vm: false,
            default_address_pool: *DEFAULT_ADDRESS_POOL,
            network_interface: None,
        }
    }
}

#[cfg(feature = "containers")]
#[derive(Args, Debug, Clone, Default)]
pub struct ContainerOptions {
    #[arg(long, default_value_t = 0)]
    pub kube_service_port: u16,
}

/// The options for the auth proxy
#[derive(Debug, Parser, Clone)]
pub struct AuthProxyOpts {
    /// The host on which the auth proxy will listen
    #[arg(long, default_value = "0.0.0.0")]
    pub auth_proxy_host: IpAddr,
    /// The port on which the auth proxy will listen
    #[arg(long, default_value_t = DEFAULT_AUTH_PROXY_PORT)]
    pub auth_proxy_port: u16,
}

impl Default for AuthProxyOpts {
    fn default() -> Self {
        Self {
            auth_proxy_host: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            auth_proxy_port: DEFAULT_AUTH_PROXY_PORT,
        }
    }
}

impl BlueprintManagerConfig {
    /// Check if all configured directories exist, and if not, create them
    ///
    /// # Errors
    ///
    /// This will error if it fails to create any of the directories.
    fn verify_directories_exist(&self) -> Result<()> {
        if !self.cache_dir().exists() {
            info!(
                "Cache directory does not exist, creating it at `{}`",
                self.cache_dir().display()
            );
            std::fs::create_dir_all(self.cache_dir())?;
        }

        if !self.runtime_dir().exists() {
            info!(
                "Runtime directory does not exist, creating it at `{}`",
                self.runtime_dir().display()
            );
            std::fs::create_dir_all(self.runtime_dir())?;
        }

        if !self.data_dir().exists() {
            info!(
                "Data directory does not exist, creating it at `{}`",
                self.data_dir().display()
            );
            std::fs::create_dir_all(self.data_dir())?;
        }

        let keystore = Path::new(self.keystore_uri());
        if !keystore.exists() {
            info!(
                "Keystore directory does not exist, creating it at `{}`",
                keystore.display()
            );
            std::fs::create_dir_all(keystore)?;
        }

        Ok(())
    }

    /// Checks if a network interface was provided, and if not, attempts to determine the host's default
    ///
    /// # Errors
    ///
    /// This will error if it is unable to determine the default network interface
    #[cfg(feature = "vm-sandbox")]
    fn verify_network_interface(&mut self) -> Result<String> {
        if let Some(interface) = self.vm_sandbox_options.network_interface.clone() {
            return Ok(interface);
        }

        let Ok(interface) = netdev::interface::get_default_interface().map(|i| i.name) else {
            error!(
                "Unable to determine the default network interface, you must specify it manually with --network-interface"
            );
            return Err(Error::Other(String::from(
                "Failed to determine default network interface",
            )));
        };

        self.vm_sandbox_options.network_interface = Some(interface.clone());
        Ok(interface)
    }
}

fn default_cache_dir() -> PathBuf {
    match dirs::cache_dir() {
        Some(dir) => dir.join("blueprint-manager"),
        None => PathBuf::from("./blueprint-manager-cache"),
    }
}

fn default_runtime_dir() -> PathBuf {
    match dirs::runtime_dir() {
        Some(dir) => dir.join("blueprint-manager"),
        None => PathBuf::from("/run/blueprint-manager"),
    }
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum SourceType {
    Container,
    #[default]
    Native,
    Wasm,
}

impl Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceType::Container => write!(f, "container"),
            SourceType::Native => write!(f, "native"),
            SourceType::Wasm => write!(f, "wasm"),
        }
    }
}
