use crate::error::{Error, Result};
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::models::{DiskConfig, VmConfig};
use cloud_hypervisor_client::{SocketBasedApiClient, socket_based_api_client};
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};
use hyper::StatusCode;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
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
        // Leave 64 KiB for FAT overhead
        const IMG_OVERHEAD: u64 = 64 * 1024;
        // Make at *least* 1 MiB images
        const MIN_IMG_SIZE: u64 = 1 * 1024 * 1024;

        let binary_meta = fs::metadata(&binary_path)?;

        let file_data_len = binary_meta.len() + IMG_OVERHEAD;
        let img_len = file_data_len.next_power_of_two().max(MIN_IMG_SIZE);

        let mut img = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&self.binary_image_path)?;
        img.set_len(img_len)?;

        fatfs::format_volume(&mut img, FormatVolumeOptions::new())?;

        let fs = FileSystem::new(&mut img, FsOptions::new())?;
        let root = fs.root_dir();

        std::io::copy(
            &mut File::open(&binary_path)?,
            &mut root.create_file("service")?,
        )?;

        let launcher_script_content = Self::build_launcher_script(env_vars, arguments);

        let mut l = root.create_file("launch")?;
        l.write_all(launcher_script_content.as_bytes())?;

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
        service_name: &str,
    ) -> Result<()> {
        let Some(_conf) = self.vm_conf.take() else {
            error!("Service already created!");
            return Ok(());
        };

        self.create_binary_image(binary_path, env_vars, arguments)
            .await
            .map_err(|e| {
                error!("Error creating binary image: {}", e);
                e
            })?;

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
        client.create_vm(vm_conf).await.map_err(|e| {
            error!("Failed to create VM: {e:?}");
            Error::Hypervisor(format!("{e:?}"))
        })?;

        Ok(())
    }

    pub async fn start(&mut self) -> Result<()> {
        let client = self
            .client()
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;
        client.boot_vm().await.map_err(|e| {
            error!("Failed to boot VM: {e:?}");
            Error::Hypervisor(format!("{e:?}"))
        })?;

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
