use super::ProcessHandle;
use super::{BlueprintSourceHandler, Status};
use crate::error::{Error, Result};
use docktopus::DockerBuilder;
use docktopus::bollard::Docker;
use docktopus::container::Container;
use std::sync::Arc;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::ImageRegistryFetcher;
use tokio::process::Command;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, log, warn};
use blueprint_runner::config::BlueprintEnvironment;
use std::future::Future;
use url::Url;
use std::net::IpAddr;
use std::path::Path;
use tokio::net::lookup_host;
use crate::config::SourceCandidates;

pub struct ContainerSource {
    pub fetcher: ImageRegistryFetcher,
    pub blueprint_id: u64,
    pub blueprint_name: String,
    resolved_image: Option<String>,
}

impl ContainerSource {
    #[must_use]
    pub fn new(fetcher: ImageRegistryFetcher, blueprint_id: u64, blueprint_name: String) -> Self {
        Self {
            fetcher,
            blueprint_id,
            blueprint_name,
            resolved_image: None,
        }
    }
}

/// Returns true if the given URL appears to refer to a local endpoint,
/// using the OS's resolver configuration.
async fn is_local_endpoint(raw_url: &str) -> bool {
    let Ok(url) = Url::parse(raw_url) else {
        return false;
    };

    let Some(host) = url.host_str() else {
        return false;
    };

    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }

    if let Ok(ip) = host.parse::<IpAddr>() {
        return ip.is_loopback();
    }

    // Default to 9944, since this is only ever used to determine the RPC endpoint for a Tangle node anyway.
    let port = url.port_or_known_default().unwrap_or(9944);
    if let Ok(mut addrs) = lookup_host((host, port)).await {
        return addrs.all(|addr| addr.ip().is_loopback());
    }

    false
}

/// Convert any local endpoints to `host.containers.internal`
async fn adjust_url_for_container(raw_url: &str) -> String {
    let Ok(mut url) = Url::parse(raw_url) else {
        return raw_url.to_owned();
    };

    if is_local_endpoint(raw_url).await {
        url.set_host(Some("host.containers.internal"))
            .expect("Failed to set host in URL");
    }
    url.to_string()
}

// TODO(serial): Stop using `Error::Other` everywhere.
impl BlueprintSourceHandler for ContainerSource {
    async fn fetch(&mut self, _cache_dir: &Path) -> Result<()> {
        let registry = String::from_utf8(self.fetcher.registry.0.0.clone())
            .map_err(|e| Error::Other(e.to_string()))?;
        let image = String::from_utf8(self.fetcher.image.0.0.clone())
            .map_err(|e| Error::Other(e.to_string()))?;
        let tag = String::from_utf8(self.fetcher.tag.0.0.clone())
            .map_err(|e| Error::Other(e.to_string()))?;

        let full = format!("{registry}/{image}:{tag}");
        log::info!("Pulling image {full}");

        Command::new("docker")
            .arg("pull")
            .arg(&full)
            .status()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        self.resolved_image = Some(format!("{image}:{tag}"));
        Ok(())
    }

    async fn spawn(
        &mut self,
        source_candidates: &SourceCandidates,
        env: &BlueprintEnvironment,
        service: &str,
        _args: Vec<String>,
        env_vars: Vec<(String, String)>,
    ) -> Result<ProcessHandle> {
        let Some(container_host) = &source_candidates.container else {
            return Err(Error::Other(String::from(
                "No container manager found, unable to use this container source.",
            )));
        };

        let image = match &self.resolved_image {
            Some(img) => img.clone(),
            None => return Err(Error::Other("Image not resolved".to_string())),
        };

        let mut adjusted_env_vars = Vec::with_capacity(env_vars.len());
        for (key, value) in env_vars {
            // The RPC endpoints need to be adjusted for containers, since they'll usually refer to
            // localhost when testing.
            if key == "HTTP_RPC_URL" || key == "WS_RPC_URL" {
                let adjusted_value = adjust_url_for_container(&value).await;
                adjusted_env_vars.push((key, adjusted_value));
            } else {
                adjusted_env_vars.push((key, value));
            }
        }

        let builder = DockerBuilder::with_address(container_host.as_str())
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let client = builder.client();

        let (stop_tx, stop_rx) = oneshot::channel::<()>();
        let (status_tx, status_rx) = mpsc::unbounded_channel::<Status>();

        let service_name = service.to_string();
        let task = create_container_task(
            client,
            image,
            env.keystore_uri.clone(),
            adjusted_env_vars,
            status_tx,
            stop_rx,
            service_name,
        )
        .await?;
        tokio::spawn(task);

        Ok(ProcessHandle::new(status_rx, stop_tx))
    }

    fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    fn name(&self) -> String {
        self.blueprint_name.clone()
    }
}

async fn detect_sysbox(client: &Docker) -> Result<bool> {
    let info = client
        .info()
        .await
        .map_err(|e| Error::Other(e.to_string()))?;
    if let Some(rts) = info.runtimes {
        if rts.contains_key("sysbox-runc") {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn create_container_task(
    client: Arc<Docker>,
    image: String,
    keystore_uri: String,
    env_vars: Vec<(String, String)>,
    status_tx: mpsc::UnboundedSender<Status>,
    stop_rx: oneshot::Receiver<()>,
    service_name: String,
) -> Result<impl Future<Output = ()> + Send> {
    let runtime = if matches!(detect_sysbox(&client).await, Ok(true)) {
        Some("sysbox-runc")
    } else {
        None
    };

    let keystore_uri_absolute = std::path::absolute(&keystore_uri)?;

    let binds = vec![format!(
        "{}:{keystore_uri}",
        keystore_uri_absolute.display()
    )];

    // TODO: Name the container `service_name`
    let mut container = Container::new(client, image)
        .env(env_vars.into_iter().map(|(k, v)| format!("{k}={v}")))
        .binds(binds)
        .extra_hosts(["host.docker.internal:host-gateway"]);

    if let Some(runtime) = runtime {
        container = container.runtime(runtime);
    }

    Ok(async move {
        let container_future = async {
            info!("Starting process execution for {service_name}");
            let output = container.start(false).await;
            if let Err(e) = output {
                error!("Failed to start container for {service_name}: {e}");
                let _ = status_tx.send(Status::Error);
            }

            let _ = status_tx.send(Status::Running);

            let output = container.wait().await;
            if output.is_ok() {
                let _ = status_tx.send(Status::Finished);
            } else {
                let _ = status_tx.send(Status::Error);
            }
            output
        };

        tokio::select! {
            _ = stop_rx => {
                if let Err(e) = container.stop().await {
                    let id = container.id();
                    warn!("Stop signal received but failed to stop container with id {id:?}: {e}");
                }
            },
            output = container_future => {
                if let Err(e) = container.stop().await {
                    let id = container.id();
                    warn!("Failed to stop container with id {id:?}: {e}");
                }
                warn!("Process for {service_name} exited: {output:?}");
            }
        }
    })
}
