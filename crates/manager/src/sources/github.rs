use std::fs::File;
use super::BlueprintSourceHandler;
use crate::error::{Error, Result};
use crate::blueprint::native::get_blueprint_binary;
use crate::sdk::utils::{make_executable, valid_file_exists};
use blueprint_core::info;
use std::path::{Path, PathBuf};
use std::process::Command;
use cargo_dist_schema::{ArtifactKind, AssetKind, DistManifest};
use tangle_subxt::subxt::ext::jsonrpsee::core::__reexports::serde_json;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::{BlueprintBinary, GithubFetcher};
use tar::Archive;
use tokio::io::AsyncWriteExt;
use tracing::{error, warn};
use xz::read::XzDecoder;

pub struct GithubBinaryFetcher {
    pub fetcher: GithubFetcher,
    pub blueprint_id: u64,
    pub blueprint_name: String,
    allow_unchecked_attestations: bool,
    target_binary_name: Option<String>,
    resolved_binary_path: Option<PathBuf>,
}

impl GithubBinaryFetcher {
    #[must_use]
    pub fn new(
        fetcher: GithubFetcher,
        blueprint_id: u64,
        blueprint_name: String,
        allow_unchecked_attestations: bool,
    ) -> Self {
        GithubBinaryFetcher {
            fetcher,
            blueprint_id,
            blueprint_name,
            allow_unchecked_attestations,
            target_binary_name: None,
            resolved_binary_path: None,
        }
    }

    async fn get_binary(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        let relevant_binary =
            get_blueprint_binary(&self.fetcher.binaries.0).ok_or(Error::NoMatchingBinary)?;

        let tag_str = std::str::from_utf8(&self.fetcher.tag.0.0).map_or_else(
            |_| self.fetcher.tag.0.0.escape_ascii().to_string(),
            ToString::to_string,
        );

        const DIST_MANIFEST_NAME: &str = "dist.json";

        let relevant_binary_name = String::from_utf8(relevant_binary.name.0.0.clone())?;

        let archive_file_name = format!("archive-{tag_str}");
        let archive_download_path = cache_dir.join(archive_file_name);
        let dist_manifest_path = cache_dir.join(DIST_MANIFEST_NAME);

        let has_archive = valid_file_exists(&archive_download_path).await;
        let has_manifest = valid_file_exists(&dist_manifest_path).await;

        // Check if the binary exists, if not download it
        if has_archive && has_manifest {
            info!(
                "Archive already exists at: {}",
                archive_download_path.display()
            );

            self.target_binary_name = Some(relevant_binary_name);
            return Ok(archive_download_path);
        }

        if has_archive || has_manifest {
            warn!("Missing archive or manifest, re-downloading...");
            let _ = tokio::fs::remove_file(&archive_download_path).await;
            let _ = tokio::fs::remove_file(&dist_manifest_path).await;
        }

        let urls = DownloadUrls::new(relevant_binary, &self.fetcher);
        info!("Downloading dist manifest from {}", urls.dist_manifest);

        let Ok(manifest) = reqwest::get(urls.dist_manifest).await else {
            error!(
                "No dist manifest found for blueprint {} (id: {}, tag: {tag_str})",
                self.blueprint_id, self.blueprint_id
            );
            return Err(Error::NoMatchingBinary);
        };

        let manifest_contents = manifest.bytes().await?;
        std::fs::write(&dist_manifest_path, &manifest_contents)?;

        let manifest: DistManifest = serde_json::from_slice(manifest_contents.as_ref())?;

        let mut found_asset = false;
        for (_, artifact) in manifest.artifacts {
            if !matches!(artifact.kind, ArtifactKind::ExecutableZip) {
                continue;
            }

            for asset in artifact.assets {
                if !matches!(asset.kind, AssetKind::Executable(_)) {
                    continue;
                }

                if asset.name.is_some_and(|s| s == relevant_binary_name) {
                    found_asset = true;
                }
            }
        }

        if !found_asset {
            error!(
                "Didn't find binary asset `{relevant_binary_name}` in manifest, malformed blueprint?"
            );
            return Err(Error::NoMatchingBinary);
        }

        self.target_binary_name = Some(relevant_binary_name);

        info!(
            "Downloading binary from {} to {}",
            urls.binary_archive_url,
            archive_download_path.display()
        );

        let archive = reqwest::get(&urls.binary_archive_url)
            .await?
            .bytes()
            .await?;

        // Write the archive to disk
        let mut file = tokio::fs::File::create(&archive_download_path).await?;
        file.write_all(&archive).await?;
        file.flush().await?;

        Ok(archive_download_path)
    }
}

impl BlueprintSourceHandler for GithubBinaryFetcher {
    async fn fetch(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        if let Some(resolved_binary_path) = &self.resolved_binary_path {
            return Ok(resolved_binary_path.clone());
        }

        let archive_path = self.get_binary(cache_dir).await?;

        let owner =
            String::from_utf8(self.fetcher.owner.0.0.clone()).expect("Should be a valid owner");
        let repo =
            String::from_utf8(self.fetcher.repo.0.0.clone()).expect("Should be a valid repo");

        match verify_attestation(&owner, &repo, &archive_path) {
            AttestationResult::Ok => {}
            AttestationResult::NotMatching | AttestationResult::NoGithubCli
                if self.allow_unchecked_attestations => {}
            AttestationResult::NotMatching => return Err(Error::AttestationFailed),
            AttestationResult::NoGithubCli => {
                error!("No GitHub CLI found, unable to verify attestation.");
                return Err(Error::NoGithubCli);
            }
        }

        let tar_xz = File::open(&archive_path)?;
        let tar = XzDecoder::new(tar_xz);
        let mut archive = Archive::new(tar);

        archive.unpack(cache_dir)?;

        // sanity check that the binary actually there
        let mut binary_path = None;
        for entry in walkdir::WalkDir::new(cache_dir) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.file_name().to_str() != self.target_binary_name.as_deref() {
                continue;
            }

            binary_path = Some(entry.path().to_path_buf());
            break;
        }

        let Some(mut binary_path) = binary_path else {
            error!("Expected binary not found in the archive, bad manifest?");
            return Err(Error::NoMatchingBinary);
        };

        // Ensure the binary is executable
        binary_path = make_executable(&binary_path)?;
        self.resolved_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    fn name(&self) -> String {
        self.blueprint_name.clone()
    }
}

struct DownloadUrls {
    binary_archive_url: String,
    dist_manifest: String,
}

impl DownloadUrls {
    fn new(binary: &BlueprintBinary, fetcher: &GithubFetcher) -> Self {
        let owner = String::from_utf8(fetcher.owner.0.0.clone()).expect("Should be a valid owner");
        let repo = String::from_utf8(fetcher.repo.0.0.clone()).expect("Should be a valid repo");
        let tag = String::from_utf8(fetcher.tag.0.0.clone()).expect("Should be a valid tag");
        let binary_name =
            String::from_utf8(binary.name.0.0.clone()).expect("Should be a valid binary name");
        let os_name = format!("{:?}", binary.os).to_lowercase();
        let arch_name = format!("{:?}", binary.arch).to_lowercase();
        // let binary_archive_url = format!(
        //     "https://github.com/{owner}/{repo}/releases/download/{tag}/{binary_name}-{os_name}-{arch_name}.tar.xz"
        // );

        let binary_archive_url = String::from(
            "https://github.com/tangle-network/hyperlane-validator-blueprint/releases/download/0.1.0/hyperlane-validator-blueprint-bin-x86_64-unknown-linux-gnu.tar.xz",
        );

        let dist_manifest =
            format!("https://github.com/{owner}/{repo}/releases/download/{tag}/dist-manifest.json");
        Self {
            binary_archive_url,
            dist_manifest,
        }
    }
}

enum AttestationResult {
    Ok,
    NotMatching,
    NoGithubCli,
}

fn verify_attestation(owner: &str, repo: &str, binary: impl AsRef<Path>) -> AttestationResult {
    match Command::new("which").arg("gh").output() {
        Ok(output) if output.status.success() => {}
        Ok(_) | Err(_) => return AttestationResult::NoGithubCli,
    }

    let repo = format!("{owner}/{repo}");
    match Command::new("gh")
        .args(["attestation", "verify"])
        .arg(binary.as_ref())
        .arg("--repo")
        .arg(repo)
        .output()
    {
        Ok(output) if output.status.success() => AttestationResult::Ok,
        Ok(_) => AttestationResult::NotMatching,
        Err(_) => AttestationResult::NoGithubCli,
    }
}
