use super::bridge::{Bridge, BridgeHandle};
use super::hypervisor::{CHVmConfig, HypervisorInstance};
use crate::error::{Error, Result};
use std::path::Path;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time;
use tracing::error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Running,
    Finished,
    Error,
}

pub struct Service {
    manager_id: u32,
    hypervisor: HypervisorInstance,
    bridge: BridgeHandle,
    alive_rx: Option<oneshot::Receiver<()>>,
}

impl Service {
    pub async fn new(
        id: u32,
        vm_conf: CHVmConfig,
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
        binary_path: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: Vec<String>,
    ) -> Result<Service> {
        let bridge = Bridge::new(runtime_dir.as_ref().to_path_buf(), service_name.to_string());
        let bridge_base_socket = bridge.base_socket_path();

        let (bridge_handle, alive_rx) = bridge.spawn().map_err(|e| {
            error!("Failed to spawn manager <-> service bridge: {e}");
            e
        })?;

        let mut hypervisor = HypervisorInstance::new(
            vm_conf,
            data_dir,
            keystore,
            cache_dir.as_ref(),
            runtime_dir.as_ref(),
            service_name,
        )?;

        hypervisor
            .prepare(
                id,
                bridge_base_socket,
                binary_path.as_ref(),
                env_vars,
                arguments,
                service_name,
            )
            .await?;

        Ok(Self {
            manager_id: id,
            hypervisor,
            bridge: bridge_handle,
            alive_rx: Some(alive_rx),
        })
    }

    pub fn status(&self) -> Result<Status> {
        // TODO: A way to actually check the status
        Ok(Status::Running)
    }

    pub async fn start(&mut self) -> Result<()> {
        let Some(alive_rx) = self.alive_rx.take() else {
            error!("Service already started!");
            return Ok(());
        };

        self.hypervisor.start().await.map_err(|e| {
            error!("Failed to start hypervisor: {e}");
            e
        })?;

        if time::timeout(Duration::from_secs(30), alive_rx)
            .await
            .is_err()
        {
            error!("Service never connected to bridge (network error?)");
            return Err(Error::Other("Bridge connection timeout".into()));
        }

        Ok(())
    }

    pub async fn shutdown(mut self) -> Result<()> {
        self.hypervisor.shutdown().await.map_err(|e| {
            error!("Failed to shut down hypervisor: {e}");
            e
        })?;
        self.bridge.shutdown();

        Ok(())
    }
}
