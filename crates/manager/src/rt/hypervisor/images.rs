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
const CLOUD_IMG_RAW_FILE_NAME: &str = "ubuntu-raw.img";
const VMLINUZ_IMG_FILE_NAME: &str = "vmlinuz";
const INITRD_IMG_FILE_NAME: &str = "initrd.img";

const FILES: &[(&str, &str)] = &[
    (CLOUD_IMG_FILE_NAME, CLOUD_IMG_URL),
    (VMLINUZ_IMG_FILE_NAME, VMLINUZ_URL),
    (INITRD_IMG_FILE_NAME, INITRD_URL),
];

async fn download_image_if_needed(cache_dir: impl AsRef<Path>) -> Result<()> {
    let cache = cache_dir.as_ref();

    let mut downloaded_cloud_image = false;
    for (dest_file_name, url) in FILES {
        let dest = cache.join(dest_file_name);
        if !dest.exists() {
            warn!("{} not found, downloading...", dest.display());

            let response = reqwest::get(*url).await?.bytes().await?;
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(dest)?;
            f.write_all(&response)?;

            if *dest_file_name == CLOUD_IMG_FILE_NAME {
                downloaded_cloud_image = true;
            }
        }
    }

    // cloud-hypervisor doesn't support compressed QCOW2, need to convert to raw
    if downloaded_cloud_image {
        let compressed_img_path = cache.join(CLOUD_IMG_FILE_NAME);
        let raw_img_path = cache.join(CLOUD_IMG_RAW_FILE_NAME);
        let out = Command::new("qemu-img")
            .args(["convert", "-O", "raw"])
            .arg(compressed_img_path)
            .arg(raw_img_path)
            .output()?;

        if !out.status.success() {
            return Err(Error::Other(format!(
                "Failed to convert Ubuntu cloud image to raw: {}",
                String::from_utf8_lossy(&out.stderr)
            )));
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

        let image_dest = data_dir.as_ref().join(CLOUD_IMG_RAW_FILE_NAME);
        fs::copy(cache.join(CLOUD_IMG_RAW_FILE_NAME), &image_dest)?;

        Ok(Self {
            image: image_dest,
            vmlinuz: cache.join(VMLINUZ_IMG_FILE_NAME),
            initrd: cache.join(INITRD_IMG_FILE_NAME),
        })
    }
}
