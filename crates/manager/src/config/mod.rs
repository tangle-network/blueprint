use crate::error::{Error, Result};
use blueprint_auth::proxy::DEFAULT_AUTH_PROXY_PORT;
use blueprint_core::{error, info};
use clap::{Args, Parser};
use docktopus::bollard::system::Version;
use docktopus::bollard::{API_DEFAULT_VERSION, Docker};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::HeaderValue;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use url::Url;

pub static DEFAULT_DOCKER_HOST: LazyLock<Url> =
    LazyLock::new(|| Url::parse("unix:///var/run/docker.sock").unwrap());

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

#[derive(Debug, Args)]
pub struct BlueprintManagerConfig {
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
    /// Disables the VM sandbox for native blueprints
    ///
    /// This should only be used for testing and never for production setups.
    #[arg(long)]
    #[cfg(feature = "vm-sandbox")]
    pub no_vm: bool,
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
    /// The location of the Podman-Docker socket
    #[arg(long, default_value_t = DEFAULT_DOCKER_HOST.clone())]
    pub podman_host: Url,
    /// The default address pool for VM TAP interfaces
    #[arg(long, default_value_t = *DEFAULT_ADDRESS_POOL)]
    #[cfg(feature = "vm-sandbox")]
    pub default_address_pool: ipnet::Ipv4Net,
    /// The network interface to funnel blueprint VM traffic through
    #[arg(long)]
    #[cfg(feature = "vm-sandbox")]
    pub network_interface: Option<String>,

    /// Authentication proxy options
    #[command(flatten)]
    pub auth_proxy_opts: AuthProxyOpts,
}

impl BlueprintManagerConfig {
    /// Check if all configured directories exist, and if not, create them
    ///
    /// # Errors
    ///
    /// This will error if it fails to create any of the directories.
    pub fn verify_directories_exist(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            info!(
                "Cache directory does not exist, creating it at `{}`",
                self.cache_dir.display()
            );
            std::fs::create_dir_all(&self.cache_dir)?;
        }

        if !self.runtime_dir.exists() {
            info!(
                "Runtime directory does not exist, creating it at `{}`",
                self.runtime_dir.display()
            );
            std::fs::create_dir_all(&self.runtime_dir)?;
        }

        if !self.data_dir.exists() {
            info!(
                "Data directory does not exist, creating it at `{}`",
                self.data_dir.display()
            );
            std::fs::create_dir_all(&self.data_dir)?;
        }

        let keystore = Path::new(&self.keystore_uri);
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
    pub fn verify_network_interface(&mut self) -> Result<()> {
        if self.network_interface.is_some() {
            return Ok(());
        }

        let Ok(interface) = netdev::interface::get_default_interface().map(|i| i.name) else {
            error!(
                "Unable to determine the default network interface, you must specify it manually with --network-interface"
            );
            return Err(Error::Other(String::from(
                "Failed to determine default network interface",
            )));
        };

        self.network_interface = Some(interface);
        Ok(())
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

impl Default for BlueprintManagerConfig {
    fn default() -> Self {
        Self {
            blueprint_config: None,
            keystore_uri: "./keystore".into(),
            data_dir: PathBuf::from("./data"),
            cache_dir: default_cache_dir(),
            runtime_dir: default_runtime_dir(),
            verbose: 0,
            pretty: false,
            instance_id: None,
            test_mode: false,
            #[cfg(feature = "vm-sandbox")]
            no_vm: false,
            allow_unchecked_attestations: false,
            preferred_source: SourceType::default(),
            podman_host: DEFAULT_DOCKER_HOST.clone(),
            #[cfg(feature = "vm-sandbox")]
            default_address_pool: *DEFAULT_ADDRESS_POOL,
            #[cfg(feature = "vm-sandbox")]
            network_interface: None,
            auth_proxy_opts: AuthProxyOpts::default(),
        }
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

pub struct SourceCandidates {
    pub container: Option<Url>,
    pub wasm_runtime: Option<String>,
    pub preferred_source: SourceType,
}

impl SourceCandidates {
    /// Determine all runtime sources available on this system
    ///
    /// # Errors
    ///
    /// If the `preferred_source` is not available on this system, or there is an error during its
    /// detection. Errors that occur while searching for non-preferred sources are silently ignored.
    pub async fn load(preferred_source: SourceType, podman_host: Url) -> Result<SourceCandidates> {
        let mut ret = SourceCandidates {
            container: None,
            wasm_runtime: None,
            preferred_source,
        };

        if let Err(e) = ret.determine_podman(podman_host).await {
            if preferred_source == SourceType::Container {
                error!("Podman not found, cannot use container source type as default: {e}");
                return Err(e);
            }
        }

        if let Err(e) = ret.determine_wasm().await {
            if preferred_source == SourceType::Wasm {
                error!("No WASM runtime found, cannot use WASM source type as default: {e}");
                return Err(e);
            }
        }

        Ok(ret)
    }

    async fn determine_podman(&mut self, host: Url) -> Result<()> {
        fn check_server_header(server: Option<&HeaderValue>) -> bool {
            if let Some(server) = server {
                if let Ok(server) = server.to_str() {
                    return server.to_lowercase().contains("libpod");
                }
            }
            false
        }

        let client = Docker::connect_with_local(host.as_str(), 20, API_DEFAULT_VERSION)
            .map_err(|e| Error::Other(e.to_string()))?;

        // Check the version, cheapest route
        let ver: Version = client
            .version()
            .await
            .map_err(|e| Error::Other(format!("Unable to determine the Podman version: {e}")))?;
        if let Some(comps) = &ver.components {
            if comps
                .iter()
                .any(|c| c.name.to_lowercase().contains("podman"))
            {
                self.container = Some(host);
                return Ok(());
            }
        }
        if let Some(platform) = &ver.platform {
            if platform.name.to_lowercase().contains("podman") {
                self.container = Some(host);
                return Ok(());
            }
        }

        // Fallback, read the HTTP "Server" header from a /_ping
        let res = if let Some(socket) = host.as_str().strip_prefix("unix://") {
            let client = Client::builder(TokioExecutor::new())
                .build::<_, Full<Bytes>>(hyperlocal::UnixConnector);
            client
                .get(hyperlocal::Uri::new(socket, "/_ping").into())
                .await
                .map_err(|e| Error::Other(format!("Unable to reach specified Podman host: {e}")))?
        } else {
            let client = Client::builder(TokioExecutor::new()).build_http::<Full<Bytes>>();
            client
                .get(
                    format!("{host}/_ping")
                        .parse()
                        .map_err(|e| Error::Other(format!("Unable to parse provided URI: {e}")))?,
                )
                .await
                .map_err(|e| Error::Other(format!("Unable to reach specified Podman host: {e}")))?
        };

        if check_server_header(res.headers().get("Server")) {
            self.container = Some(host);
            return Ok(());
        }

        Err(Error::Other(String::from("No Podman-Docker socket found")))
    }

    #[expect(clippy::unused_async, reason = "TBD")]
    async fn determine_wasm(&mut self) -> Result<bool> {
        // TODO: Verify WASM runtime installations
        Ok(true)
    }
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
