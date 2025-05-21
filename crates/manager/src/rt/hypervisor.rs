use crate::error::{Error, Result};
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::models::{ConsoleConfig, DiskConfig, PayloadConfig, VmConfig, VsockConfig};
use cloud_hypervisor_client::{SocketBasedApiClient, socket_based_api_client};
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};
use hyper::StatusCode;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use cloud_hypervisor_client::models::console_config::Mode;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct CHVmConfig;

pub(super) struct HypervisorInstance {
    sock_path: PathBuf,
    guest_logs_path: PathBuf,
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
        let guest_logs_path = cache_dir.as_ref().join(format!("{}-guest.log", service_name));
        let stdout_log_path = cache_dir.as_ref().join(format!("{}.log.stdout", service_name));
        let stderr_log_path = cache_dir.as_ref().join(format!("{}.log.stderr", service_name));
        let binary_image_path = cache_dir
            .as_ref()
            .join(&format!("{}-bin.img", service_name));
        let sock_path = runtime_dir.as_ref().join("ch-api.sock");

        let stdout = OpenOptions::new().create(true).read(true).append(true).open(&stdout_log_path)?;
        let stderr = OpenOptions::new().create(true).read(true).append(true).open(&stderr_log_path)?;
        let handle = Command::new("cloud-hypervisor")
            .arg("--api-socket")
            .arg(&sock_path)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()?;

        Ok(HypervisorInstance {
            sock_path,
            guest_logs_path,
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

        fatfs::format_volume(&mut img, FormatVolumeOptions::new().volume_label(*b"SERVICEDISK"))?;

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
        manager_service_id: u32,
        bridge_socket_path: impl AsRef<Path>,
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

        let (serial, console, cmdline_console_target) = self.logging_configs();

        let vm_conf = VmConfig {
            cpus: None,
            memory: None,
            payload: PayloadConfig {
                // TODO
                kernel: Some(String::from("/home/alex/Downloads/kernel-extracted/vmlinuz")),
                initramfs: Some(String::from("/home/alex/Downloads/kernel-extracted/initrd.img")),
                cmdline: Some(format!("root=/dev/vda2 rw console={cmdline_console_target} systemd.log_level=debug systemd.log_target=kmsg")),
                ..Default::default()
            },
            rate_limit_groups: None,
            disks: Some(vec![
                DiskConfig {
                    // TODO
                    path: Some(String::from("/home/alex/Downloads/debian-base.qcow2")),
                    readonly: Some(false),
                    direct: Some(true),
                    ..DiskConfig::default()
                },
                DiskConfig {
                    path: Some(self.binary_image_path.display().to_string()),
                    readonly: Some(true),
                    direct: Some(true),
                    ..DiskConfig::default()
                },
            ]),
            net: None,
            rng: None,
            balloon: None,
            fs: None,
            pmem: None,
            serial: Some(serial),
            console: Some(console),
            debug_console: None,
            devices: None,
            vdpa: None,
            vsock: Some(VsockConfig {
                cid: manager_service_id as i64,
                socket: bridge_socket_path.as_ref().to_string_lossy().into(),
                ..Default::default()
            }),
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

    // Disable serial port logging in release builds, too much noise for production
    //#[cfg(not(debug_assertions))]
    // fn logging_configs(&self) -> (ConsoleConfig, ConsoleConfig, &'static str) {
    //     let serial = ConsoleConfig { mode: Mode::Off, ..Default::default() };
    //     let virtio_console = ConsoleConfig {
    //         mode: Mode::File,
    //         file: Some(self.guest_logs_path.to_string_lossy().into()),
    //         ..Default::default()
    //     };
    //     (serial, virtio_console, "hvc0")
    // }

    //#[cfg(debug_assertions)]
    fn logging_configs(&self) -> (ConsoleConfig, ConsoleConfig, &'static str) {
        let serial = ConsoleConfig {
            mode: Mode::File,
            file: Some(self.guest_logs_path.to_string_lossy().into()),
            ..Default::default()
        };
        let virtio_console = ConsoleConfig { mode: Mode::Off, ..Default::default() };
        (serial, virtio_console, "ttyS0")
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
        const VM_SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_secs(30);

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

        let _ = fs::remove_file(self.sock_path);

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
