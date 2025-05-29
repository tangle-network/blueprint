pub mod net;
use net::NetworkManager;

use crate::error::{Error, Result};
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::models::console_config::Mode;
use cloud_hypervisor_client::models::{
    ConsoleConfig, DiskConfig, FsConfig, MemoryConfig, NetConfig, PayloadConfig, VmConfig,
    VsockConfig,
};
use cloud_hypervisor_client::{SocketBasedApiClient, socket_based_api_client};
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};
use hyper::StatusCode;
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::fmt::Write as _;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use tracing::{error, info, warn};

const DATA_DIR_VIRTIO: &str = "blueprint-data";
const KEYSTORE_VIRTIO: &str = "blueprint-keystore";

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
    cloud_init_image_path: PathBuf,
    virtio: Virtio,
    handles: Handles,
    lease: Option<net::Lease>,
}

impl HypervisorInstance {
    pub fn new(
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
        let binary_image_path = cache_dir.as_ref().join(format!("{}-bin.img", service_name));
        let cloud_init_image_path = cache_dir
            .as_ref()
            .join(format!("{}-cloud-init.img", service_name));
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
        let hypervisor_handle = Command::new("cloud-hypervisor")
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
            cloud_init_image_path,
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
            lease: None,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn create_binary_image(
        &self,
        data_dir: impl AsRef<Path>,
        keystore: impl AsRef<Path>,
        binary_path: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: &[String],
    ) -> Result<()> {
        const LAUNCHER_SCRIPT_TEMPLATE: &str = r"#!/bin/sh
        set -e

        mkdir -p {{DATA_DIR}}
        mount -t virtiofs {{DATA_DIR_VIRTIO}} {{DATA_DIR}}

        mkdir -p {{KEYSTORE_DIR}}
        mount -t virtiofs {{KEYSTORE_VIRTIO}} {{KEYSTORE_DIR}}

        {{ENV_VARS}}

        exec /srv/service {{SERVICE_ARGS}}
        ";

        let mut env_vars_str = String::new();
        for (key, val) in env_vars {
            writeln!(&mut env_vars_str, "export {key}=\"{val}\"").unwrap();
        }

        let args = arguments.join(" ");

        let launcher_script = LAUNCHER_SCRIPT_TEMPLATE
            .replace("{{DATA_DIR}}", &data_dir.as_ref().to_string_lossy())
            .replace("{{DATA_DIR_VIRTIO}}", DATA_DIR_VIRTIO)
            .replace("{{KEYSTORE_DIR}}", &keystore.as_ref().to_string_lossy())
            .replace("{{KEYSTORE_VIRTIO}}", KEYSTORE_VIRTIO)
            .replace("{{ENV_VARS}}", &env_vars_str)
            .replace("{{SERVICE_ARGS}}", &args);

        let binary_meta = fs::metadata(binary_path.as_ref())?;

        new_fat_fs(FatFsConfig {
            starting_length: binary_meta.len() as usize,
            volume_label: *b"SERVICEDISK",
            files: vec![
                CopiedFile {
                    target_name: String::from("service"),
                    source: FileSource::Fs(binary_path.as_ref().to_path_buf()),
                },
                CopiedFile {
                    target_name: String::from("launch"),
                    source: FileSource::Raw(launcher_script.into_bytes()),
                },
            ],
            image_path: self.binary_image_path.clone(),
        })?;

        Ok(())
    }

    fn create_cloud_init_image(&self, manager_service_id: u32) -> Result<()> {
        const CLOUD_INIT_USER_DATA: &str = include_str!("assets/user-data");
        const CLOUD_INIT_META_DATA: &str = include_str!("assets/meta-data");

        let meta = CLOUD_INIT_META_DATA
            .replace("{{BLUEPRINT_INSTANCE_ID}}", &manager_service_id.to_string());

        new_fat_fs(FatFsConfig {
            starting_length: CLOUD_INIT_USER_DATA.len() + CLOUD_INIT_META_DATA.len(),
            volume_label: *b"CIDATA     ",
            files: vec![
                CopiedFile {
                    target_name: String::from("user-data"),
                    source: FileSource::Raw(CLOUD_INIT_USER_DATA.as_bytes().to_vec()),
                },
                CopiedFile {
                    target_name: String::from("meta-data"),
                    source: FileSource::Raw(meta.into_bytes()),
                },
            ],
            image_path: self.cloud_init_image_path.clone(),
        })
    }

    pub async fn prepare(
        &mut self,
        manager_service_id: u32,
        network_manager: NetworkManager,
        bridge_socket_path: impl AsRef<Path>,
        binary_path: impl AsRef<Path>,
        env_vars: Vec<(String, String)>,
        arguments: Vec<String>,
    ) -> Result<()> {
        self.create_binary_image(
            &self.virtio.data_dir,
            &self.virtio.keystore,
            binary_path,
            env_vars,
            &arguments,
        )
        .map_err(|e| {
            error!("Error creating binary image: {e}");
            e
        })?;

        self.create_cloud_init_image(manager_service_id)
            .map_err(|e| {
                error!("Error creating cloud-init image: {e}");
                e
            })?;

        let (serial, console, cmdline_console_target) = self.logging_configs();

        let (lease, tap_interface) = network_manager
            .new_tap_interface(manager_service_id)
            .await
            .map_err(|e| {
                error!("Error creating TAP interface: {e}");
                e
            })?;

        let tap_interface_addr = lease.addr();
        self.lease = Some(lease);

        let vm_conf = VmConfig {
            memory: Some(MemoryConfig {
                size: 4096,
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
            disks: Some(vec![
                DiskConfig {
                    // TODO
                    path: Some(String::from("/home/alex/Downloads/ubuntu-base.raw")),
                    readonly: Some(false),
                    direct: Some(true),
                    ..DiskConfig::default()
                },
                DiskConfig {
                    path: Some(self.cloud_init_image_path.to_string_lossy().to_string()),
                    readonly: Some(true),
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
            serial: Some(serial),
            console: Some(console),
            vsock: Some(VsockConfig {
                // + 3 since 0 = hypervisor, 1 = loopback, 2 = host
                cid: i64::from(manager_service_id) + 3,
                socket: bridge_socket_path.as_ref().to_string_lossy().into(),
                ..Default::default()
            }),
            net: Some(vec![NetConfig {
                tap: Some(tap_interface),
                ip: Some(tap_interface_addr.to_string()),
                ..Default::default()
            }]),
            ..Default::default()
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

    #[allow(clippy::cast_possible_wrap)]
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

struct CopiedFile {
    target_name: String,
    source: FileSource,
}

enum FileSource {
    Fs(PathBuf),
    Raw(Vec<u8>),
}

struct FatFsConfig {
    starting_length: usize,
    volume_label: [u8; 11],
    files: Vec<CopiedFile>,
    image_path: PathBuf,
}

fn new_fat_fs(config: FatFsConfig) -> Result<()> {
    // Leave 64 KiB for FAT overhead
    const IMG_OVERHEAD: u64 = 64 * 1024;
    // Make at *least* 1 MiB images
    const MIN_IMG_SIZE: u64 = 1024 * 1024;

    let file_data_len = (config.starting_length as u64) + IMG_OVERHEAD;
    let img_len = file_data_len.next_power_of_two().max(MIN_IMG_SIZE);

    let mut img = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(config.image_path)?;
    img.set_len(img_len)?;

    fatfs::format_volume(
        &mut img,
        FormatVolumeOptions::new().volume_label(config.volume_label),
    )?;

    let fs = FileSystem::new(&mut img, FsOptions::new())?;
    let root = fs.root_dir();

    for file in config.files {
        match file.source {
            FileSource::Fs(path_on_host) => {
                std::io::copy(
                    &mut File::open(path_on_host)?,
                    &mut root.create_file(&file.target_name)?,
                )?;
            }
            FileSource::Raw(raw_file_content) => {
                let mut f = root.create_file(&file.target_name)?;
                f.write_all(&raw_file_content)?;
            }
        }
    }

    Ok(())
}
