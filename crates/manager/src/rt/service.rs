use super::bridge::{Bridge, BridgeHandle};
use super::hypervisor::{CHVmConfig, HypervisorInstance};
use crate::error::Result;
use std::path::Path;
use tracing::error;

enum BridgeState {
    Inactive(Bridge),
    Started(BridgeHandle),
}

pub struct Service {
    hypervisor: HypervisorInstance,
    bridge: Option<BridgeState>,
}

impl Service {
    pub fn new(
        vm_conf: CHVmConfig,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
    ) -> Result<Service> {
        let hypervisor = HypervisorInstance::new(
            vm_conf,
            cache_dir.as_ref(),
            runtime_dir.as_ref(),
            service_name,
        )?;
        let bridge = Bridge::new(runtime_dir.as_ref().to_path_buf(), service_name.to_string());

        Ok(Self {
            hypervisor,
            bridge: Some(BridgeState::Inactive(bridge)),
        })
    }

    pub async fn prepare(
        &mut self,
        binary_path: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: Vec<String>,
    ) -> Result<()> {
        self.hypervisor
            .prepare(binary_path.as_ref(), env_vars, arguments)
            .await?;
        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        let Some(BridgeState::Inactive(bridge)) = self.bridge.take() else {
            error!("Service already started!");
            return Ok(());
        };

        let bridge_handle = bridge.spawn()?;
        self.bridge = Some(BridgeState::Started(bridge_handle));

        self.hypervisor.start().await?;

        Ok(())
    }

    pub async fn shutdown(mut self) -> Result<()> {
        let Some(BridgeState::Started(bridge)) = self.bridge.take() else {
            error!("Service not running!");
            return Ok(());
        };

        self.hypervisor.shutdown().await?;
        bridge.shutdown();

        Ok(())
    }
}
