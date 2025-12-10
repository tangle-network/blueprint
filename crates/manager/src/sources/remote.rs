use super::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler};
use crate::blueprint::native::get_blueprint_binary;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use crate::sdk::utils::{make_executable, valid_file_exists};
use crate::sources::types::{BlueprintBinary, RemoteFetcher};
use blake3::Hasher;
use blueprint_core::{error, info, warn};
use blueprint_runner::config::BlueprintEnvironment;
use cargo_dist_schema::{ArtifactKind, AssetKind, DistManifest};
use hex;
use reqwest::Client;
use serde_json;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tar::Archive;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use url::Url;
use xz::read::XzDecoder;

const MANIFEST_FILE_NAME: &str = "dist.json";
const MAX_ARCHIVE_BYTES_ENV: &str = "MAX_ARCHIVE_BYTES";
const IPFS_GATEWAY_ENV: &str = "IPFS_GATEWAY_URL";
const DEFAULT_MAX_ARCHIVE_BYTES: u64 = 512 * 1024 * 1024; // 512 MiB
const DOWNLOAD_RETRIES: usize = 3;
const RETRY_BACKOFF_MS: u64 = 500;

pub struct RemoteBinaryFetcher {
    fetcher: RemoteFetcher,
    blueprint_id: u64,
    blueprint_name: String,
    http: Client,
    max_archive_bytes: u64,
    ipfs_gateway: Option<String>,
    target_binary_name: Option<String>,
    selected_binary: Option<BlueprintBinary>,
    resolved_binary_path: Option<PathBuf>,
}

impl RemoteBinaryFetcher {
    #[must_use]
    pub fn new(fetcher: RemoteFetcher, blueprint_id: u64, blueprint_name: String) -> Self {
        let max_archive_bytes = std::env::var(MAX_ARCHIVE_BYTES_ENV)
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_MAX_ARCHIVE_BYTES);
        let ipfs_gateway = std::env::var(IPFS_GATEWAY_ENV).ok();

        Self {
            fetcher,
            blueprint_id,
            blueprint_name,
            http: Client::new(),
            max_archive_bytes,
            ipfs_gateway,
            target_binary_name: None,
            selected_binary: None,
            resolved_binary_path: None,
        }
    }

    async fn get_binary(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        let relevant_binary =
            get_blueprint_binary(&self.fetcher.binaries).ok_or(Error::NoMatchingBinary)?;
        self.target_binary_name = Some(relevant_binary.name.clone());
        self.selected_binary = Some(relevant_binary.clone());

        let cache_key = self.cache_key();
        let archive_file_name = format!("remote-archive-{cache_key}");
        let archive_path = cache_dir.join(&archive_file_name);
        let manifest_path = cache_dir.join(format!("remote-{cache_key}-{MANIFEST_FILE_NAME}"));

        let has_archive = valid_file_exists(&archive_path).await;
        let has_manifest = valid_file_exists(&manifest_path).await;

        if has_archive && has_manifest {
            info!(
                "Remote archive already cached for blueprint {} at {}",
                self.blueprint_id,
                archive_path.display()
            );
        } else {
            if has_archive || has_manifest {
                warn!(
                    "Blueprint {} cache missing either manifest or archive, re-downloading",
                    self.blueprint_id
                );
                let _ = fs::remove_file(&archive_path).await;
                let _ = fs::remove_file(&manifest_path).await;
            }
            let dist_url = self.resolve_url(&self.fetcher.dist_url)?;
            let archive_url = self.resolve_url(&self.fetcher.archive_url)?;

            info!(
                "Downloading dist manifest for blueprint {} from {}",
                self.blueprint_id, dist_url
            );
            let manifest_bytes = self.download_manifest(&dist_url).await?;
            fs::write(&manifest_path, &manifest_bytes).await?;

            info!(
                "Downloading archive for blueprint {} from {}",
                self.blueprint_id, archive_url
            );
            self.download_archive(&archive_url, &archive_path).await?;
        }

        let manifest_bytes = fs::read(&manifest_path).await?;
        let manifest: DistManifest = serde_json::from_slice(&manifest_bytes)?;
        self.ensure_manifest_contains_binary(&manifest, &relevant_binary.name)?;

        Ok(archive_path)
    }

    fn ensure_manifest_contains_binary(
        &self,
        manifest: &DistManifest,
        binary_name: &str,
    ) -> Result<()> {
        for artifact in manifest.artifacts.values() {
            if !matches!(artifact.kind, ArtifactKind::ExecutableZip) {
                continue;
            }
            for asset in &artifact.assets {
                if !matches!(asset.kind, AssetKind::Executable(_)) {
                    continue;
                }
                if asset.name.as_deref() == Some(binary_name) {
                    return Ok(());
                }
            }
        }

        error!(
            "Binary `{binary_name}` not found in manifest for blueprint {}",
            self.blueprint_id
        );
        Err(Error::NoMatchingBinary)
    }

    fn cache_key(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(self.fetcher.dist_url.as_bytes());
        hasher.update(self.fetcher.archive_url.as_bytes());
        hex::encode(&hasher.finalize().as_bytes()[..8])
    }

    fn resolve_url(&self, raw: &str) -> Result<Url> {
        if raw.starts_with("ipfs://") {
            let gateway = self
                .ipfs_gateway
                .as_ref()
                .ok_or_else(|| Error::MissingIpfsGateway {
                    url: raw.to_string(),
                })?;
            let suffix = raw.trim_start_matches("ipfs://").trim_start_matches('/');
            let full = format!(
                "{}/{}",
                gateway.trim_end_matches('/'),
                suffix.trim_start_matches('/')
            );
            Url::parse(&full).map_err(|err| Error::DownloadFailed {
                url: raw.to_string(),
                reason: format!("failed to build gateway URL: {err}"),
            })
        } else {
            Url::parse(raw).map_err(|err| Error::DownloadFailed {
                url: raw.to_string(),
                reason: err.to_string(),
            })
        }
    }

    async fn download_manifest(&self, url: &Url) -> Result<Vec<u8>> {
        let response = self.send_request(url).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(|err| Error::DownloadFailed {
                url: url.to_string(),
                reason: err.to_string(),
            })?;
        Ok(bytes.to_vec())
    }

    async fn download_archive(&self, url: &Url, dest: &Path) -> Result<()> {
        let response = self.send_request(url).await?;
        let total_len = response.content_length().unwrap_or(0);
        if total_len > self.max_archive_bytes && total_len > 0 {
            return Err(Error::ArchiveTooLarge {
                url: url.to_string(),
                size: total_len,
                max: self.max_archive_bytes,
            });
        }

        let temp_path = dest.with_extension("part");
        let mut file = fs::File::create(&temp_path).await?;
        let mut downloaded: u64 = 0;
        let mut response = response;

        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|err| Error::DownloadFailed {
                url: url.to_string(),
                reason: err.to_string(),
            })?
        {
            downloaded += chunk.len() as u64;
            if downloaded > self.max_archive_bytes {
                let _ = fs::remove_file(&temp_path).await;
                return Err(Error::ArchiveTooLarge {
                    url: url.to_string(),
                    size: downloaded,
                    max: self.max_archive_bytes,
                });
            }
            file.write_all(&chunk).await?;
        }
        file.flush().await?;
        fs::rename(&temp_path, dest).await?;
        Ok(())
    }

    async fn send_request(&self, url: &Url) -> Result<reqwest::Response> {
        let mut last_error = String::new();
        for attempt in 0..=DOWNLOAD_RETRIES {
            match self.http.get(url.clone()).send().await {
                Ok(resp) if resp.status().is_success() => return Ok(resp),
                Ok(resp) => {
                    last_error = format!("HTTP {}", resp.status());
                }
                Err(err) => {
                    last_error = err.to_string();
                }
            }

            if attempt < DOWNLOAD_RETRIES {
                let delay = RETRY_BACKOFF_MS * (attempt as u64 + 1);
                warn!(
                    "Download attempt {} for {} failed ({last_error}), retrying in {}ms",
                    attempt + 1,
                    url,
                    delay
                );
                sleep(Duration::from_millis(delay)).await;
            }
        }

        Err(Error::DownloadFailed {
            url: url.to_string(),
            reason: last_error,
        })
    }

    async fn clear_cache(&self, archive_path: &Path, manifest_path: &Path) {
        let _ = fs::remove_file(archive_path).await;
        let _ = fs::remove_file(manifest_path).await;
    }

    fn unpack_archive(&self, archive_path: &Path, cache_dir: &Path) -> Result<PathBuf> {
        let tar_xz = File::open(&archive_path)?;
        let tar = XzDecoder::new(tar_xz);
        let mut archive = Archive::new(tar);
        archive.unpack(cache_dir)?;

        let binary_name = self
            .target_binary_name
            .as_deref()
            .ok_or(Error::NoMatchingBinary)?;

        let mut binary_path = None;
        for entry in walkdir::WalkDir::new(cache_dir) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            if entry.file_name().to_str() != Some(binary_name) {
                continue;
            }
            binary_path = Some(entry.path().to_path_buf());
            break;
        }

        let Some(mut binary_path) = binary_path else {
            error!(
                "Expected binary {binary_name} not found in archive for blueprint {}",
                self.blueprint_id
            );
            return Err(Error::NoMatchingBinary);
        };

        self.verify_binary_digest(&binary_path)?;
        binary_path = make_executable(&binary_path)?;
        Ok(binary_path)
    }

    fn verify_binary_digest(&self, binary_path: &Path) -> Result<()> {
        let binary = self
            .selected_binary
            .as_ref()
            .ok_or(Error::NoMatchingBinary)?;

        let mut file = File::open(binary_path)?;
        let mut buffer = [0u8; 8192];
        let mut blake3_hasher = Hasher::new();
        let mut sha256_hasher = Sha256::new();

        loop {
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            blake3_hasher.update(&buffer[..read]);
            sha256_hasher.update(&buffer[..read]);
        }

        let computed_blake3 = blake3_hasher.finalize();
        if let Some(expected) = &binary.blake3 {
            if computed_blake3.as_bytes() != expected {
                return Err(Error::HashMismatch {
                    expected: hex::encode(expected),
                    actual: hex::encode(computed_blake3.as_bytes()),
                });
            }
            return Ok(());
        }

        let computed_sha: [u8; 32] = sha256_hasher.finalize().into();
        if computed_sha != binary.sha256 {
            return Err(Error::HashMismatch {
                expected: hex::encode(binary.sha256),
                actual: hex::encode(computed_sha),
            });
        }

        Ok(())
    }
}

impl BlueprintSourceHandler for RemoteBinaryFetcher {
    async fn fetch(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        if let Some(resolved_binary_path) = &self.resolved_binary_path {
            if resolved_binary_path.exists() {
                return Ok(resolved_binary_path.clone());
            }
            self.resolved_binary_path = None;
        }

        let cache_key = self.cache_key();
        let archive_path = self.get_binary(cache_dir).await?;
        let manifest_path = cache_dir.join(format!("remote-{cache_key}-{MANIFEST_FILE_NAME}"));

        let binary_path = match self.unpack_archive(&archive_path, cache_dir) {
            Ok(path) => path,
            Err(err) => {
                warn!(
                    "Failed to unpack remote archive for blueprint {} ({err:?}). Clearing cache and retrying.",
                    self.blueprint_id
                );
                self.clear_cache(&archive_path, &manifest_path).await;
                let archive_path = self.get_binary(cache_dir).await?;
                self.unpack_archive(&archive_path, cache_dir)?
            }
        };

        self.resolved_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    async fn spawn(
        &mut self,
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        blueprint_config: &BlueprintEnvironment,
        id: u32,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
        sub_service_str: &str,
        cache_dir: &Path,
        runtime_dir: &Path,
    ) -> crate::error::Result<Service> {
        let resolved_binary_path = self.fetch(cache_dir).await?;
        Service::from_binary(
            ctx,
            limits,
            blueprint_config,
            id,
            env,
            args,
            &resolved_binary_path,
            sub_service_str,
            cache_dir,
            runtime_dir,
        )
        .await
    }

    fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    fn name(&self) -> String {
        self.blueprint_name.clone()
    }
}
