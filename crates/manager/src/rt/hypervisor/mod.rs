pub mod images;
pub mod net;

use net::NetworkManager;

use crate::error::{Error, Result};
use crate::rt::hypervisor::images::CloudImage;
use crate::rt::hypervisor::net::Lease;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_core::{error, info, warn};
use cloud_hypervisor_client::apis::DefaultApi;
use cloud_hypervisor_client::models::console_config::Mode;
use cloud_hypervisor_client::models::{
    ConsoleConfig, DiskConfig, MemoryConfig, NetConfig, PayloadConfig, VmConfig, VsockConfig,
};
use cloud_hypervisor_client::{SocketBasedApiClient, socket_based_api_client};
use fatfs::{Dir, FileSystem, FormatVolumeOptions, FsOptions};
use hyper::StatusCode;
use ipnet::Ipv4Net;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, io};
use tokio::process::{Child, Command};
use tokio::time::sleep;
use url::{Host, Url};

const VM_DATA_DIR: &str = "/mnt/data";

pub struct ServiceVmConfig {
    pub id: u32,
    pub pty: bool,
    /// Allocated storage space in bytes
    pub storage_space: u64,
    /// Allocated memory space in bytes
    pub memory_size: u64,
}

impl Default for ServiceVmConfig {
    fn default() -> Self {
        Self {
            id: 0,
            pty: false,
            // 20GB
            storage_space: 1024 * 1024 * 1024 * 20,
            // 4GB
            memory_size: 4_294_967_296,
        }
    }
}

pub struct HypervisorInstance {
    config: ServiceVmConfig,
    sock_path: PathBuf,
    guest_logs_path: PathBuf,
    binary_image_path: PathBuf,
    cloud_init_image_path: PathBuf,
    hypervisor: Child,
    lease: Option<net::Lease>,
}

impl HypervisorInstance {
    /// Create a new `HypervisorInstance`
    ///
    /// # Errors
    ///
    /// * Unable to create log files in `cache_dir`
    /// * Unable to start a `cloud-hypervisor` instance
    ///     * In this case, the issue may be logged in `<cache_dir>/<service_name>.log.stderr`
    pub fn new(
        config: ServiceVmConfig,
        cache_dir: impl AsRef<Path>,
        runtime_dir: impl AsRef<Path>,
        service_name: &str,
    ) -> Result<HypervisorInstance> {
        info!("Initializing hypervisor for service `{service_name}`...");

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
            .stderr(stderr.try_clone()?)
            .spawn()?;

        Ok(HypervisorInstance {
            config,
            sock_path,
            guest_logs_path,
            binary_image_path,
            cloud_init_image_path,
            hypervisor: hypervisor_handle,
            lease: None,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn create_binary_image(
        &self,
        keystore: impl AsRef<Path>,
        binary_path: impl AsRef<Path>,
        env_vars: &BlueprintEnvVars,
        arguments: &BlueprintArgs,
    ) -> Result<()> {
        const LAUNCHER_SCRIPT_TEMPLATE: &str = r"#!/bin/sh
        set -e

        {{ENV_VARS}}

        exec /srv/service {{SERVICE_ARGS}}
        ";

        let mut env_vars_str = String::new();
        for (key, val) in env_vars.encode() {
            writeln!(&mut env_vars_str, "export {key}=\"{val}\"").unwrap();
        }

        let args = arguments.encode().join(" ");

        let launcher_script = LAUNCHER_SCRIPT_TEMPLATE
            .replace("{{ENV_VARS}}", &env_vars_str)
            .replace("{{SERVICE_ARGS}}", &args);

        let mut entries = vec![
            CopiedEntry::File(CopiedFile {
                target_name: String::from("service"),
                source: FileSource::Fs(binary_path.as_ref().to_path_buf()),
            }),
            CopiedEntry::File(CopiedFile {
                target_name: String::from("launch"),
                source: FileSource::Raw(launcher_script.into_bytes()),
            }),
        ];

        let binary_meta = fs::metadata(binary_path.as_ref())?;
        let mut keystore_size = 0;

        let mut keystore_dir = Directory {
            name: String::from("keystore"),
            children: Vec::new(),
        };

        let mut current_dir = None;
        for entry in walkdir::WalkDir::new(keystore) {
            let entry = entry?;
            if entry.file_type().is_dir() {
                if let Some(dir) = current_dir.take() {
                    keystore_dir.children.push(CopiedEntry::Dir(dir));
                }

                current_dir = Some(Directory {
                    name: entry.file_name().to_string_lossy().into(),
                    children: Vec::new(),
                });
                continue;
            }

            let Some(current_dir) = current_dir.as_mut() else {
                // The keystore doesn't store any files in the root, so just ignore anything extra
                continue;
            };

            keystore_size += entry.metadata()?.len();

            current_dir.children.push(CopiedEntry::File(CopiedFile {
                target_name: entry.file_name().to_string_lossy().into_owned(),
                source: FileSource::Fs(entry.path().to_path_buf()),
            }));
        }

        entries.push(CopiedEntry::Dir(keystore_dir));

        new_fat_fs(FatFsConfig {
            starting_length: binary_meta.len() as usize + keystore_size as usize,
            volume_label: *b"SERVICEDISK",
            entries,
            image_path: self.binary_image_path.clone(),
        })?;

        Ok(())
    }

    fn create_cloud_init_image(
        &self,
        manager_service_id: u32,
        tap_interface_ip: Ipv4Addr,
    ) -> Result<()> {
        const CLOUD_INIT_USER_DATA: &str = include_str!("assets/user-data");
        const CLOUD_INIT_NETWORK_CONFIG: &str = include_str!("assets/network-config");
        const CLOUD_INIT_META_DATA: &str = include_str!("assets/meta-data");

        let net = Ipv4Net::new(tap_interface_ip, 24).unwrap();
        let guest_ip = net
            .hosts()
            .find(|ip| *ip != tap_interface_ip)
            .ok_or(Error::Other(String::from(
                "Unable to determine IP address for guest",
            )))?;

        let net = CLOUD_INIT_NETWORK_CONFIG
            .replace("{{TAP_IP_ADDRESS}}", &tap_interface_ip.to_string())
            .replace("{{GUEST_IP_ADDRESS}}", &guest_ip.to_string());

        let meta = CLOUD_INIT_META_DATA
            .replace("{{BLUEPRINT_INSTANCE_ID}}", &manager_service_id.to_string());

        new_fat_fs(FatFsConfig {
            starting_length: CLOUD_INIT_USER_DATA.len() + meta.len() + net.len(),
            volume_label: *b"CIDATA     ",
            entries: vec![
                CopiedEntry::File(CopiedFile {
                    target_name: String::from("user-data"),
                    source: FileSource::Raw(CLOUD_INIT_USER_DATA.as_bytes().to_vec()),
                }),
                CopiedEntry::File(CopiedFile {
                    target_name: String::from("meta-data"),
                    source: FileSource::Raw(meta.into_bytes()),
                }),
                CopiedEntry::File(CopiedFile {
                    target_name: String::from("network-config"),
                    source: FileSource::Raw(net.into_bytes()),
                }),
            ],
            image_path: self.cloud_init_image_path.clone(),
        })
    }

    async fn create_data_disk(&self, data_dir: impl AsRef<Path>) -> Result<PathBuf> {
        let image_path = data_dir.as_ref().join("data.qcow2");
        let out = Command::new("qemu-img")
            .args(["create", "-f", "qcow2"])
            .arg(&image_path)
            .arg(self.config.storage_space.to_string())
            .output()
            .await?;

        if !out.status.success() {
            return Err(Error::Other(format!(
                "Failed to create data disk for blueprint: {}",
                String::from_utf8_lossy(&out.stderr)
            )));
        }

        Ok(image_path)
    }

    /// Configure the VM for boot
    ///
    /// # Errors
    ///
    /// * Unable to create the service or cloud-init FAT fs disks
    /// * Unable to create the QCOW data disk (possible storage exhaustion)
    /// * Unable to allocate an IPv4 address (possible pool exhaustion), see [`NetworkManager`]
    /// * Unable to create the VM, possibly a bad configuration
    ///
    /// See also:
    /// * [`Self::client()`]
    /// * [`CloudImage::fetch()`]
    #[allow(clippy::too_many_arguments)]
    pub async fn prepare(
        &mut self,
        network_manager: NetworkManager,
        keystore: impl AsRef<Path>,
        data_dir: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        bridge_socket_path: impl AsRef<Path>,
        binary_path: impl AsRef<Path>,
        mut env_vars: BlueprintEnvVars,
        arguments: BlueprintArgs,
    ) -> Result<()> {
        /// TODO: actually resolve the hosts to see if they're loopback
        // For local testnets, we need to translate IPs to the host
        fn translate_local_ip(url: &mut Url, lease: &Lease) {
            match url.host() {
                Some(Host::Ipv4(ip)) if ip.is_loopback() => {
                    let _ = url.set_ip_host(lease.addr().into()).ok();
                }
                _ => {}
            }
        }

        let image = CloudImage::fetch(data_dir.as_ref(), cache_dir).await?;

        let data_disk_path = self.create_data_disk(data_dir).await?;

        env_vars.data_dir = PathBuf::from(VM_DATA_DIR);
        env_vars.keystore_uri = String::from("/srv/keystore");

        let Some(lease) = network_manager.allocate().await else {
            return Err(io::Error::new(io::ErrorKind::QuotaExceeded, "IP pool exhausted").into());
        };

        translate_local_ip(&mut env_vars.http_rpc_endpoint, &lease);
        translate_local_ip(&mut env_vars.ws_rpc_endpoint, &lease);

        self.create_binary_image(keystore, binary_path, &env_vars, &arguments)
            .map_err(|e| {
                error!("Error creating binary image: {e}");
                e
            })?;

        self.create_cloud_init_image(self.config.id, lease.addr())
            .map_err(|e| {
                error!("Error creating cloud-init image: {e}");
                e
            })?;

        let (serial, console, cmdline_console_target) = self.logging_configs(self.config.pty);

        let tap_interface_addr = lease.addr();
        self.lease = Some(lease);

        #[allow(clippy::cast_possible_wrap)]
        let vm_conf = VmConfig {
            memory: Some(MemoryConfig {
                size: self.config.memory_size as i64,
                shared: Some(true),
                ..Default::default()
            }),
            payload: PayloadConfig {
                kernel: Some(image.vmlinuz.to_string_lossy().into()),
                initramfs: Some(image.initrd.to_string_lossy().into()),
                cmdline: Some(format!(
                    "root=/dev/vda1 rw console={cmdline_console_target} systemd.log_level=debug systemd.log_target=kmsg"
                )),
                ..Default::default()
            },
            disks: Some(vec![
                DiskConfig {
                    path: Some(image.image.to_string_lossy().into()),
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
                    path: Some(data_disk_path.to_string_lossy().into()),
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
            serial: Some(serial),
            console: Some(console),
            vsock: Some(VsockConfig {
                // + 3 since 0 = hypervisor, 1 = loopback, 2 = host
                cid: i64::from(self.config.id) + 3,
                socket: bridge_socket_path.as_ref().to_string_lossy().into(),
                ..Default::default()
            }),
            net: Some(vec![NetConfig {
                tap: Some(format!("tap-tngl-{}", self.config.id)),
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
    fn logging_configs(&self, pty: bool) -> (ConsoleConfig, ConsoleConfig, &'static str) {
        let serial = if pty {
            ConsoleConfig {
                mode: Mode::Pty,
                ..Default::default()
            }
        } else {
            ConsoleConfig {
                mode: Mode::File,
                file: Some(self.guest_logs_path.to_string_lossy().into()),
                ..Default::default()
            }
        };
        let virtio_console = ConsoleConfig {
            mode: Mode::Off,
            ..Default::default()
        };
        (serial, virtio_console, "ttyS0")
    }

    /// Boot the virtual machine
    ///
    /// # Errors
    ///
    /// * This will error if the VM cannot be booted for any reason
    /// * See [`Self::client()`]
    pub async fn start(&mut self) -> Result<()> {
        info!("Booting VM...");

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

    /// Connect to the configured VMM socket
    ///
    /// # Errors
    ///
    /// * This will error if it is unable to ping the VMM
    pub async fn client(&self) -> Result<SocketBasedApiClient> {
        let client = socket_based_api_client(&self.sock_path);

        if let Err(e) = client.vmm_ping_get().await {
            return Err(Error::Hypervisor(format!("{e:?}")));
        }

        Ok(client)
    }

    /// Shutdown the VM
    ///
    /// If the VM fails to shut down within the grace period, it, along with the VMM, will be killed.
    ///
    /// # Errors
    ///
    /// * This will error if it unable to send a shutdown signal to the VM
    /// * See [`Self::client()`]
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
            self.hypervisor.kill().await?;
            return Ok(());
        }

        // VM manager shutting down, process will exit with it
        self.hypervisor.wait().await?;

        Ok(())
    }

    /// Get the pty path, if the VM is configured to output to one
    ///
    /// # Errors
    ///
    /// * Unable to fetch VM info
    /// * See [`Self::client()`]
    pub async fn pty(&self) -> Result<Option<PathBuf>> {
        let client = self.client().await?;
        let info = client
            .vm_info_get()
            .await
            .map_err(|e| Error::Hypervisor(format!("{e:?}")))?;
        Ok(info.config.serial.and_then(|c| c.file.map(PathBuf::from)))
    }
}

enum CopiedEntry {
    Dir(Directory),
    File(CopiedFile),
}

struct Directory {
    name: String,
    children: Vec<CopiedEntry>,
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
    entries: Vec<CopiedEntry>,
    image_path: PathBuf,
}

fn new_fat_fs(config: FatFsConfig) -> Result<()> {
    fn write_entry(root: &Dir<'_, &mut File>, entry: CopiedEntry) -> Result<()> {
        match entry {
            CopiedEntry::Dir(dir) => {
                let root = root.create_dir(&dir.name)?;
                for child in dir.children {
                    write_entry(&root, child)?;
                }
            }
            CopiedEntry::File(file) => match file.source {
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
            },
        }

        Ok(())
    }

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

    for entry in config.entries {
        write_entry(&root, entry)?;
    }

    Ok(())
}
