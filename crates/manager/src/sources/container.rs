use super::ProcessHandle;
use super::{BlueprintSource, Status};
use crate::error::{Error, Result};
use dockworker::container::Container;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::gadget::ImageRegistryFetcher;
use tokio::process::Command;
use tracing::{info, log, warn};

pub struct ContainerSource {
    pub fetcher: ImageRegistryFetcher,
    pub blueprint_id: u64,
    pub gadget_name: String,
    resolved_image: Option<String>,
}

// TODO(serial): Stop using `Error::Other` everywhere
impl BlueprintSource for ContainerSource {
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
            .arg(full)
            .status()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        self.resolved_image = Some(format!("{image}:{tag}"));

        Ok(())
    }

    async fn spawn(
        &mut self,
        service: &str,
        _args: Vec<String>,
        env: Vec<(String, String)>,
    ) -> Result<ProcessHandle> {
        let Some(image) = &self.resolved_image else {
            return Err(Error::Other("Image not resolved".to_string()));
        };

        let builder = dockworker::DockerBuilder::new()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let mut container = Container::new(builder.get_client(), image).env(env);

        let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
        let (status_tx, status_rx) = tokio::sync::mpsc::unbounded_channel::<Status>();

        let task = async move {
            info!("Starting process execution for {service}");
            let _ = status_tx.send(Status::Running).ok();
            let output = container.start(true).await;
            if output.is_ok() {
                let _ = status_tx.send(Status::Finished).ok();
            } else {
                let _ = status_tx.send(Status::Error).ok();
            }

            warn!("Process for {service} exited: {output:?}");
        };

        let task = async move {
            tokio::select! {
                _ = stop_rx => {},
                () = task => {},
            }
        };

        Ok(ProcessHandle::new(status_rx, stop_tx))
    }

    fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    fn name(&self) -> String {
        self.gadget_name.clone()
    }
}
