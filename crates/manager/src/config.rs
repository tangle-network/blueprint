use crate::error::{Error, Result};
use clap::Parser;
use docktopus::bollard::system::Version;
use docktopus::bollard::{API_DEFAULT_VERSION, Docker};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::HeaderValue;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::LazyLock;
use tracing::error;
use url::Url;

pub static DEFAULT_DOCKER_HOST: LazyLock<Url> =
    LazyLock::new(|| Url::parse("unix:///var/run/docker.sock").unwrap());

#[derive(Debug, Parser)]
#[command(
    name = "Blueprint Manager",
    about = "An program executor that connects to the Tangle network and runs protocols dynamically on the fly"
)]
pub struct BlueprintManagerConfig {
    /// The path to the gadget configuration file
    #[arg(short = 's', long)]
    pub gadget_config: Option<PathBuf>,
    /// The path to the keystore
    #[arg(short = 'k', long)]
    pub keystore_uri: String,
    /// The directory in which all gadgets will store their data
    #[arg(long, short = 'd', default_value = "./data")]
    pub data_dir: PathBuf,
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
    /// The preferred way to run a blueprint.
    ///
    /// This is not a guarantee that the blueprint will use this method, as there may not be a source
    /// available of this type.
    #[arg(long, short, default_value_t)]
    pub preferred_source: SourceType,
    /// The location of the Podman-Docker socket
    #[arg(long, short, default_value_t = DEFAULT_DOCKER_HOST.clone())]
    pub podman_host: Url,
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
