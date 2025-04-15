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
use tracing::{info, log, warn};
use blueprint_runner::config::BlueprintEnvironment;
use std::future::Future;

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

// TODO(serial): Stop using `Error::Other` everywhere
impl BlueprintSourceHandler for ContainerSource {
    async fn fetch(&mut self) -> Result<()> {
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
        env: &BlueprintEnvironment,
        service: &str,
        _args: Vec<String>,
        env_vars: Vec<(String, String)>,
    ) -> Result<ProcessHandle> {
        let image = match &self.resolved_image {
            Some(img) => img.clone(),
            None => return Err(Error::Other("Image not resolved".to_string())),
        };

        let builder = DockerBuilder::new()
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
            env_vars,
            status_tx,
            stop_rx,
            service_name,
        )?;
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

fn create_container_task(
    client: Arc<Docker>,
    image: String,
    keystore_uri: String,
    env_vars: Vec<(String, String)>,
    status_tx: mpsc::UnboundedSender<Status>,
    stop_rx: oneshot::Receiver<()>,
    service_name: String,
) -> Result<impl Future<Output = ()> + Send> {
    let mut container = Container::new(client, image);
    container.env(env_vars.into_iter().map(|(k, v)| format!("{k}={v}")));

    let keystore_uri_absolute = std::path::absolute(&keystore_uri)?;
    container.binds([format!(
        "{}:{keystore_uri}",
        keystore_uri_absolute.display()
    )]);

    Ok(async move {
        let container_future = async {
            info!("Starting process execution for {service_name}");
            let _ = status_tx.send(Status::Running);
            let output = container.start(true).await;
            if output.is_ok() {
                let _ = status_tx.send(Status::Finished);
            } else {
                let _ = status_tx.send(Status::Error);
            }
            dbg!(output)
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
