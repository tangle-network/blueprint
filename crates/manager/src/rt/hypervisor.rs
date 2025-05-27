use crate::error::{Error, Result};
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::models::console_config::Mode;
use cloud_hypervisor_client::models::{
    ConsoleConfig, DiskConfig, FsConfig, MemoryConfig, PayloadConfig, VmConfig, VsockConfig,
};
use cloud_hypervisor_client::{SocketBasedApiClient, socket_based_api_client};
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};
use hyper::StatusCode;
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use tracing::{error, info, warn};

const DATA_DIR_VIRTIO: &str = "blueprint-data";
const KEYSTORE_VIRTIO: &str = "blueprint-keystore";

pub struct CHVmConfig;

struct Handles {
    hypervisor: Child,
    data_dir_virtio: Child,
    keystore_virtio: Child,
}

struct Virtio {
    data_dir_socket: PathBuf,
    data_dir: PathBuf,
    keystore_socket: PathBuf,
    keystore: PathBuf,
}

pub(super) struct HypervisorInstance {
    sock_path: PathBuf,
    guest_logs_path: PathBuf,
    binary_image_path: PathBuf,
    virtio: Virtio,
    handles: Handles,
    vm_conf: Option<CHVmConfig>,
}

impl HypervisorInstance {
    pub fn new(
        conf: CHVmConfig,
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
    ) -> Result<HypervisorInstance> {
        let guest_logs_path = cache_dir
            .as_ref()
            .join(format!("{}-guest.log", service_name));
        let stdout_log_path = cache_dir
            .as_ref()
            .join(format!("{}.log.stdout", service_name));
        let stderr_log_path = cache_dir
            .as_ref()
            .join(format!("{}.log.stderr", service_name));
        let binary_image_path = cache_dir
            .as_ref()
            .join(&format!("{}-bin.img", service_name));
        let sock_path = runtime_dir.as_ref().join("ch-api.sock");

        let stdout = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&stdout_log_path)?;
        let stderr = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&stderr_log_path)?;
        let handle = Command::new("cloud-hypervisor")
            .arg("--api-socket")
            .arg(&sock_path)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()?;

        let data_dir_virtio_socket = runtime_dir.as_ref().join("data-dir.sock");
        let data_dir_virtio = Command::new("unshare")
            .args([
                "-r",
                "--map-auto",
                "--",
                "/usr/lib/virtiofsd",
                "--sandbox",
                "chroot",
            ])
            .arg(format!(
                "--socket-path={}",
                data_dir_virtio_socket.display()
            ))
            .arg(format!("--shared-dir={}", data_dir.as_ref().display()))
            .spawn()?;

        let keystore_virtio_socket = runtime_dir.as_ref().join("keystore.sock");
        let keystore_virtio = Command::new("unshare")
            .args([
                "-r",
                "--map-auto",
                "--",
                "/usr/lib/virtiofsd",
                "--sandbox",
                "chroot",
            ])
            .arg(format!(
                "--socket-path={}",
                keystore_virtio_socket.display()
            ))
            .arg(format!("--shared-dir={}", keystore.as_ref().display()))
            .spawn()?;

        Ok(HypervisorInstance {
            sock_path,
            guest_logs_path,
            binary_image_path,
            virtio: Virtio {
                data_dir_socket: data_dir_virtio_socket,
                data_dir: data_dir.as_ref().to_path_buf(),
                keystore_socket: keystore_virtio_socket,
                keystore: keystore.as_ref().to_path_buf(),
            },
            handles: Handles {
                hypervisor: hypervisor_handle,
                data_dir_virtio,
                keystore_virtio,
            },
            vm_conf: Some(conf),
        })
    }

    async fn create_binary_image(
        &self,
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
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

        fatfs::format_volume(
            &mut img,
            FormatVolumeOptions::new().volume_label(*b"SERVICEDISK"),
        )?;

        let fs = FileSystem::new(&mut img, FsOptions::new())?;
        let root = fs.root_dir();

        std::io::copy(
            &mut File::open(&binary_path)?,
            &mut root.create_file("service")?,
        )?;

        let launcher_script_content =
            Self::build_launcher_script(data_dir, keystore, env_vars, arguments);

        let mut l = root.create_file("launch")?;
        l.write_all(launcher_script_content.as_bytes())?;

        Ok(())
    }

    fn build_launcher_script(
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: Vec<String>,
    ) -> String {
        const LAUNCHER_SCRIPT_HEADER: &str = r"#!/bin/sh
        set -e
        ";

        let mut launcher_script = LAUNCHER_SCRIPT_HEADER.to_string();

        launcher_script.push_str(&format!("mkdir -p {}\n", data_dir.as_ref().display()));
        launcher_script.push_str(&format!(
            "mount -t virtiofs {DATA_DIR_VIRTIO} {}\n",
            data_dir.as_ref().display()
        ));

        launcher_script.push_str(&format!("mkdir -p {}\n", keystore.as_ref().display()));
        launcher_script.push_str(&format!(
            "mount -t virtiofs {KEYSTORE_VIRTIO} {}\n",
            keystore.as_ref().display()
        ));

        for (key, val) in env_vars {
            launcher_script.push_str(&format!("export {key}=\"{val}\"\n"));
        }

        let args = arguments.join(" ");
        launcher_script = format!("{launcher_script}exec /srv/service {args}");

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

        self.create_binary_image(
            &self.virtio.data_dir,
            &self.virtio.keystore,
            binary_path,
            env_vars,
            arguments,
        )
        .await
        .map_err(|e| {
            error!("Error creating binary image: {}", e);
            e
        })?;

        let (serial, console, cmdline_console_target) = self.logging_configs();

        let vm_conf = VmConfig {
            cpus: None,
            memory: Some(MemoryConfig {
                size: 1073741824,
                shared: Some(true),
                ..Default::default()
            }),
            payload: PayloadConfig {
                // TODO
                kernel: Some(String::from(
                    "/home/alex/Downloads/kernel-extracted/vmlinuz",
                )),
                initramfs: Some(String::from(
                    "/home/alex/Downloads/kernel-extracted/initrd.img",
                )),
                cmdline: Some(format!(
                    "root=/dev/vda1 rw console={cmdline_console_target} systemd.log_level=debug systemd.log_target=kmsg"
                )),
                ..Default::default()
            },
            rate_limit_groups: None,
            disks: Some(vec![
                DiskConfig {
                    // TODO
                    path: Some(String::from("/home/alex/Downloads/ubuntu-base.raw")),
                    readonly: Some(false),
                    direct: Some(true),
                    ..DiskConfig::default()
                },
                DiskConfig {
                    // TODO
                    path: Some(String::from("/home/alex/Downloads/cloud-init.img")),
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
            fs: Some(vec![
                FsConfig {
                    tag: String::from(DATA_DIR_VIRTIO),
                    socket: self.virtio.data_dir_socket.to_string_lossy().to_string(),
                    ..Default::default()
                },
                FsConfig {
                    tag: String::from(KEYSTORE_VIRTIO),
                    socket: self.virtio.keystore_socket.to_string_lossy().to_string(),
                    ..Default::default()
                },
            ]),
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
        let virtio_console = ConsoleConfig {
            mode: Mode::Off,
            ..Default::default()
        };
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
            self.handles.hypervisor.kill().await?;
            return Ok(());
        }

        // VM manager shutting down, process will exit with it
        self.handles.hypervisor.wait().await?;

        if let Some(id) = self.handles.data_dir_virtio.id() {
            let pid = Pid::from_raw(id as i32);
            let _ = signal::kill(pid, Signal::SIGINT).ok();
            self.handles.data_dir_virtio.wait().await?;
        }

        if let Some(id) = self.handles.keystore_virtio.id() {
            let pid = Pid::from_raw(id as i32);
            let _ = signal::kill(pid, Signal::SIGINT).ok();
            self.handles.keystore_virtio.wait().await?;
        }

        Ok(())
    }
}
