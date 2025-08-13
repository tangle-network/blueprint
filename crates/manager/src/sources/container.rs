use super::{BlueprintArgs, BlueprintEnvVars};
use super::BlueprintSourceHandler;
use crate::error::{Error, Result};
use docktopus::bollard::Docker;
use docktopus::container::Container;
use std::sync::Arc;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::ImageRegistryFetcher;
use tokio::process::Command;
use tokio::sync::{mpsc, oneshot};
use tracing::{info, log, warn};
use blueprint_runner::config::BlueprintEnvironment;
use std::future::Future;
use url::Url;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use tokio::net::lookup_host;
use crate::config::BlueprintManagerContext;
use crate::rt::ResourceLimits;
use crate::rt::service::{Service, Status};

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
async fn is_local_endpoint(url: &mut Url) -> bool {
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

/// Convert any local endpoints to `host.docker.internal`
async fn adjust_url_for_container(url: &mut Url) {
    if is_local_endpoint(url).await {
        url.set_host(Some("172.17.0.1"))
            .expect("Failed to set host in URL");
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
        log::info!("Pulling image {full}");

        Command::new("docker")
            .arg("pull")
            .arg(&full)
            .status()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        self.resolved_image = Some(full);
        Ok(PathBuf::from(self.resolved_image.as_ref().unwrap()))
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
