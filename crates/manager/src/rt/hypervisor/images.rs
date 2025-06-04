use crate::error::{Error, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::warn;

const CLOUD_IMG_URL: &str =
    "https://cloud-images.ubuntu.com/releases/24.04/release/ubuntu-24.04-server-cloudimg-amd64.img";
const VMLINUZ_URL: &str = "https://cloud-images.ubuntu.com/releases/releases/24.04/release/unpacked/ubuntu-24.04-server-cloudimg-amd64-vmlinuz-generic";
const INITRD_URL: &str = "https://cloud-images.ubuntu.com/releases/releases/24.04/release/unpacked/ubuntu-24.04-server-cloudimg-amd64-initrd-generic";

const CLOUD_IMG_FILE_NAME: &str = "ubuntu.img";
const VMLINUZ_IMG_FILE_NAME: &str = "vmlinuz";
const INITRD_IMG_FILE_NAME: &str = "initrd.img";

const FILES: &[(&str, &str)] = &[
    (CLOUD_IMG_FILE_NAME, CLOUD_IMG_URL),
    (VMLINUZ_IMG_FILE_NAME, VMLINUZ_URL),
    (INITRD_IMG_FILE_NAME, INITRD_URL),
];

// Give each VM a 20GB rootfs
const ALLOCATED_IMAGE_SIZE: u64 = 1024 * 1024 * 1024 * 20;

async fn download_image_if_needed(cache_dir: impl AsRef<Path>) -> Result<()> {
    let cache = cache_dir.as_ref();

    for (dest_file_name, url) in FILES.iter().copied() {
        let dest = cache.join(dest_file_name);
        if !dest.exists() {
            warn!("{} not found, downloading...", dest.display());

            let response = reqwest::get(url).await?.bytes().await?;
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&dest)?;
            f.write_all(&response)?;
        }
    }

    Ok(())
}

pub struct CloudImage {
    pub image: PathBuf,
    pub vmlinuz: PathBuf,
    pub initrd: PathBuf,
}

impl CloudImage {
    pub async fn fetch(data_dir: impl AsRef<Path>, cache_dir: impl AsRef<Path>) -> Result<Self> {
        let cache = cache_dir.as_ref();
        download_image_if_needed(cache).await?;

        let base_image_src = cache.join(CLOUD_IMG_FILE_NAME);
        let image_dest = data_dir.as_ref().join(CLOUD_IMG_FILE_NAME);

        // The Ubuntu cloud images are compressed QCOW2, which cloud-hypervisor doesn't support
        let out = Command::new("qemu-img")
            .args(["convert", "-O", "raw"])
            .arg(base_image_src)
            .arg(&image_dest)
            .output()?;

        if !out.status.success() {
            return Err(Error::Other(format!(
                "Failed to convert Ubuntu cloud image to raw: {}",
                String::from_utf8_lossy(&out.stderr)
            )));
        }

        let out = Command::new("qemu-img")
            .arg("resize")
            .arg(&image_dest)
            .arg(ALLOCATED_IMAGE_SIZE.to_string())
            .output()?;

        if !out.status.success() {
            return Err(Error::Other(format!(
                "Failed to resize image: {}",
                String::from_utf8_lossy(&out.stderr)
            )));
        }

        Ok(Self {
            image: image_dest,
            vmlinuz: cache.join(VMLINUZ_IMG_FILE_NAME),
            initrd: cache.join(INITRD_IMG_FILE_NAME),
        })
    }
}
