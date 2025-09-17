use super::{BlueprintArgs, BlueprintEnvVars};
use super::BlueprintSourceHandler;
use crate::error::{Error, Result};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::ImageRegistryFetcher;
use tokio::process::Command;
use blueprint_core::info;
use blueprint_runner::config::BlueprintEnvironment;
use std::path::{Path, PathBuf};
use crate::config::BlueprintManagerContext;
use crate::rt::ResourceLimits;
use crate::rt::service::Service;

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

// TODO(serial): Stop using `Error::Other` everywhere.
impl BlueprintSourceHandler for ContainerSource {
    async fn fetch(&mut self, _cache_dir: &Path) -> Result<PathBuf> {
        if let Some(resolved_image) = &self.resolved_image {
            return Ok(PathBuf::from(resolved_image));
        }

        let registry = String::from_utf8(self.fetcher.registry.0.0.clone())
            .map_err(|e| Error::Other(e.to_string()))?;
        let image = String::from_utf8(self.fetcher.image.0.0.clone())
            .map_err(|e| Error::Other(e.to_string()))?;
        let tag = String::from_utf8(self.fetcher.tag.0.0.clone())
            .map_err(|e| Error::Other(e.to_string()))?;

        let full = format!("{registry}/{image}:{tag}");
        info!("Pulling image {full}");

        Command::new("docker")
            .arg("pull")
            .arg(&full)
            .status()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        let ret = PathBuf::from(&full);
        self.resolved_image = Some(full);
        Ok(ret)
    }

    async fn spawn(
        &mut self,
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        _blueprint_config: &BlueprintEnvironment,
        _id: u32,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
        sub_service_str: &str,
        cache_dir: &Path,
        runtime_dir: &Path,
    ) -> Result<Service> {
        let image = self.fetch(cache_dir).await?;
        Service::new_container(
            ctx,
            limits,
            runtime_dir,
            sub_service_str,
            image.to_string_lossy().to_string(),
            env,
            args,
            false,
        )
        .await
    }

    fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    fn name(&self) -> String {
        self.blueprint_name.clone()
    }
}
