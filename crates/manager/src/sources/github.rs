use super::{BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, unpack_archive_safely};
use crate::blueprint::native::get_blueprint_binary;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use crate::sdk::utils::{make_executable, valid_file_exists};
use crate::sources::types::{BlueprintBinary, GithubFetcher};
use blueprint_core::{error, info, warn};
use blueprint_runner::config::BlueprintEnvironment;
use cargo_dist_schema::{ArtifactKind, AssetKind, DistManifest};
use serde_json;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use url::Url;
use xz::read::XzDecoder;

pub struct GithubBinaryFetcher {
    pub fetcher: GithubFetcher,
    pub blueprint_id: u64,
    pub blueprint_name: String,
    allow_unchecked_attestations: bool,
    target_binary_name: Option<String>,
    resolved_binary_path: Option<PathBuf>,
    http: reqwest::Client,
    release_base_url: Url,
    github_cli: PathBuf,
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
            http: reqwest::Client::new(),
            release_base_url: Url::parse("https://github.com/").expect("static GitHub URL"),
            github_cli: PathBuf::from("gh"),
        }
    }

    /// Configure the release transport host from application configuration.
    /// Repository metadata cannot change this host or the attestation policy.
    #[must_use]
    pub fn with_release_base_url(mut self, url: Url) -> Self {
        self.release_base_url = url;
        self
    }

    /// Supply application-owned HTTP transport, such as an enterprise proxy client.
    #[must_use]
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http = client;
        self
    }

    /// Select the installed GitHub CLI executable; repository metadata cannot set it.
    #[must_use]
    pub fn with_github_cli(mut self, path: impl Into<PathBuf>) -> Self {
        self.github_cli = path.into();
        self
    }

    async fn get_binary(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        let relevant_binary =
            get_blueprint_binary(&self.fetcher.binaries).ok_or(Error::NoMatchingBinary)?;

        let tag_str = &self.fetcher.tag;

        const DIST_MANIFEST_NAME: &str = "dist.json";

        let relevant_binary_name = relevant_binary.name.clone();

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

        let urls = DownloadUrls::new(relevant_binary, &self.fetcher, &self.release_base_url);
        info!("Downloading dist manifest from {}", urls.dist_manifest);

        let Ok(manifest) = self.http.get(urls.dist_manifest).send().await else {
            error!(
                "No dist manifest found for blueprint {} (id: {}, tag: {tag_str})",
                self.blueprint_id, self.blueprint_id
            );
            return Err(Error::NoMatchingBinary);
        };

        let manifest_contents = manifest.bytes().await?;
        std::fs::write(&dist_manifest_path, &manifest_contents)?;

        let manifest: DistManifest = serde_json::from_slice(manifest_contents.as_ref())?;

        let found_asset = manifest.artifacts.values().any(|artifact| {
            matches!(artifact.kind, ArtifactKind::ExecutableZip)
                && artifact.assets.iter().any(|asset| {
                    matches!(asset.kind, AssetKind::Executable(_))
                        && asset.name.as_deref() == Some(relevant_binary_name.as_str())
                })
        });

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

        let archive = self
            .http
            .get(&urls.binary_archive_url)
            .send()
            .await?
            .bytes()
            .await?;

        tokio::fs::write(&archive_download_path, &archive).await?;

        Ok(archive_download_path)
    }
}

impl BlueprintSourceHandler for GithubBinaryFetcher {
    async fn fetch(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        if let Some(resolved_binary_path) = &self.resolved_binary_path {
            if resolved_binary_path.exists() {
                return Ok(resolved_binary_path.clone());
            }

            // Re-resolve
            self.resolved_binary_path = None;
        }

        let archive_path = self.get_binary(cache_dir).await?;

        let owner = self.fetcher.owner.clone();
        let repo = self.fetcher.repo.clone();

        if let Err(error) = verify_attestation(&self.github_cli, &owner, &repo, &archive_path) {
            if !self.allow_unchecked_attestations {
                return Err(error);
            }
        }

        let tar_xz = File::open(&archive_path)?;
        let tar = XzDecoder::new(tar_xz);
        let mut archive = Archive::new(tar);
        unpack_archive_safely(&mut archive, cache_dir)?;

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

    async fn spawn(
        &mut self,
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        blueprint_config: &BlueprintEnvironment,
        id: u32,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
        _confidentiality_policy: blueprint_client_tangle::ConfidentialityPolicy,
        sub_service_str: &str,
        cache_dir: &Path,
        runtime_dir: &Path,
    ) -> Result<Service> {
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

struct DownloadUrls {
    binary_archive_url: String,
    dist_manifest: String,
}

impl DownloadUrls {
    fn new(binary: &BlueprintBinary, fetcher: &GithubFetcher, base_url: &Url) -> Self {
        let base_url = base_url.as_str().trim_end_matches('/');
        let owner = fetcher.owner.clone();
        let repo = fetcher.repo.clone();
        let tag = fetcher.tag.clone();
        let binary_name = binary.name.clone();
        let os_name = binary.os.to_lowercase();
        let arch_name = binary.arch.to_lowercase();

        // TODO: This is NOT the correct format for cargo-dist. Need the target triple onchain
        let binary_archive_url = format!(
            "{base_url}/{owner}/{repo}/releases/download/{tag}/{binary_name}-{os_name}-{arch_name}.tar.xz"
        );

        let dist_manifest =
            format!("{base_url}/{owner}/{repo}/releases/download/{tag}/dist-manifest.json");
        Self {
            binary_archive_url,
            dist_manifest,
        }
    }
}

fn verify_attestation(cli: &Path, owner: &str, repo: &str, binary: impl AsRef<Path>) -> Result<()> {
    let repo = format!("{owner}/{repo}");
    match Command::new(cli)
        .args(["attestation", "verify"])
        .arg(binary.as_ref())
        .arg("--repo")
        .arg(repo)
        .output()
    {
        Ok(output) if output.status.success() => Ok(()),
        Ok(_) => Err(Error::AttestationFailed),
        Err(_) => Err(Error::NoGithubCli),
    }
}
