use crate::error::{Error, Result};
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::models::{DiskConfig, VmConfig};
use cloud_hypervisor_client::{SocketBasedApiClient, socket_based_api_client};
use ext2::Ext2;
use hyper::StatusCode;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct CHVmConfig;

pub(super) struct HypervisorInstance {
    sock_path: PathBuf,
    binary_image_path: PathBuf,
    handle: Child,
    vm_conf: Option<CHVmConfig>,
}

impl HypervisorInstance {
    pub fn new(
        conf: CHVmConfig,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
    ) -> Result<HypervisorInstance> {
        let binary_image_path = cache_dir
            .as_ref()
            .join(&format!("{}-bin.img", service_name));
        let sock_path = runtime_dir.as_ref().join("ch-api.sock");

        let handle = Command::new("cloud-hypervisor")
            .arg("--api-socket")
            .arg(&sock_path)
            .spawn()?;

        Ok(HypervisorInstance {
            sock_path,
            binary_image_path,
            handle,
            vm_conf: Some(conf),
        })
    }

    async fn create_binary_image(
        &self,
        binary_path: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: Vec<String>,
    ) -> Result<()> {
        const IMG_SIZE: u64 = 16 * 1024 * 1024;

        let mut img = File::create(&self.binary_image_path)?;
        img.set_len(IMG_SIZE)?;

        Command::new("mkfs.ext2")
            .args(["-q", "-F", &self.binary_image_path.display().to_string()])
            .status()
            .await?;

        let mut fs = Ext2::new(&mut img)?;

        fs.create_dir("/srv")?;
        {
            let mut host_bin = File::open(binary_path.as_ref())?;
            let mut guest_bin = fs.create("/srv/service")?;
            std::io::copy(&mut host_bin, &mut guest_bin)?;

            let mut launch_script = fs.create("/srv/launch")?;
            let launcher_script_content = Self::build_launcher_script(env_vars, arguments);
            write!(&mut launch_script, "{launcher_script_content}")?;
        }

        Ok(())
    }

    fn build_launcher_script(env_vars: Vec<(String, String)>, arguments: Vec<String>) -> String {
        const LAUNCHER_SCRIPT_HEADER: &str = r"#!/bin/sh
        set -e
        ";

        let mut launcher_script = LAUNCHER_SCRIPT_HEADER.to_string();
        for (key, val) in env_vars {
            launcher_script.push_str(&format!("export {key}=\"{val}\"\n"));
        }

        launcher_script.push_str("exec /srv/service\n");

        for arg in arguments {
            launcher_script.push(' ');
            launcher_script.push_str(&arg);
        }

        launcher_script
    }

    pub async fn prepare(
        &mut self,
        binary_path: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: Vec<String>,
    ) -> Result<()> {
        let Some(_conf) = self.vm_conf.take() else {
            error!("Service already created!");
            return Ok(());
        };

        self.create_binary_image(binary_path, env_vars, arguments)
            .await?;

        let vm_conf = VmConfig {
            cpus: None,
            memory: None,
            payload: Default::default(),
            rate_limit_groups: None,
            disks: Some(vec![DiskConfig {
                path: Some(self.binary_image_path.display().to_string()),
                readonly: Some(true),
                direct: Some(true),
                ..DiskConfig::default()
            }]),
            net: None,
            rng: None,
            balloon: None,
            fs: None,
            pmem: None,
            serial: None,
            console: None,
            debug_console: None,
            devices: None,
            vdpa: None,
            vsock: None,
            sgx_epc: None,
            numa: None,
            iommu: None,
            watchdog: None,
            pvpanic: None,
            pci_segments: None,
            platform: None,
            tpm: None,
            landlock_enable: None,
            landlock_rules: None,
        };

        let client = self.client().await?;
        client
            .create_vm(vm_conf)
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;

        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        let client = self
            .client()
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;
        client
            .boot_vm()
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;

        Ok(())
    }

    pub async fn client(&self) -> Result<SocketBasedApiClient> {
        let client = socket_based_api_client(&self.sock_path);

        if let Err(e) = client.vmm_ping_get().await {
            return Err(Error::Hypervisor(format!("{e:?}")));
        }

        Ok(client)
    }

    pub async fn shutdown(mut self) -> Result<()> {
        const VM_SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_secs(10);

        let client = self
            .client()
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;

        // Wait for the VM to power down (sends SIGINT to the blueprint)
        client
            .power_button_vm()
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;

        let shutdown_task = async {
            loop {
                let r = client.vm_info_get().await;
                match r {
                    Ok(_info) => sleep(Duration::from_millis(500)).await,
                    Err(cloud_hypervisor_client::apis::Error::Api(
                        cloud_hypervisor_client::apis::ApiError {
                            code: StatusCode::NOT_FOUND,
                            ..
                        },
                    )) => return Ok(()),
                    Err(e) => return Err(Error::Hypervisor(format!("{e:?}"))),
                }
            }
        };

        info!("Attempting to shutdown VM...");
        if tokio::time::timeout(VM_SHUTDOWN_GRACE_PERIOD, shutdown_task)
            .await
            .is_err()
        {
            warn!("Unable to shutdown VM");
        }

        let _ = std::fs::remove_file(self.sock_path);

        if let Err(e) = client.shutdown_vmm().await {
            error!("Unable to gracefully shutdown VM manager, killing process: {e:?}");
            self.handle.kill().await?;
            return Ok(());
        }

        // VM manager shutting down, process will exit with it
        self.handle.wait().await?;

        Ok(())
    }
}
