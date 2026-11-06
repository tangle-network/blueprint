use super::{
    BlueprintArgs, BlueprintEnvVars, BlueprintSourceHandler, is_safe_archive_path,
    unpack_archive_safely,
};
use crate::blueprint::native::get_blueprint_binary;
use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Service;
use crate::sdk::utils::{get_formatted_os_string, hash_bytes_to_hex, make_executable};
use crate::sources::types::{BlueprintBinary, GithubFetcher};
use blueprint_core::info;
use blueprint_runner::config::BlueprintEnvironment;
use cargo_dist_schema::{ArtifactKind, AssetKind, DistManifest};
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

    async fn get_binary(
        &self,
        binary: &BlueprintBinary,
        cache_dir: &Path,
    ) -> Result<(PathBuf, PathBuf, String)> {
        let release_url = release_url(&self.release_base_url, &self.fetcher)?;
        let manifest_path = cache_dir.join("dist.json");
        let contents = if manifest_path.is_file() {
            tokio::fs::read(&manifest_path).await?
        } else {
            let url = release_url
                .join("dist-manifest.json")
                .map_err(|_| Error::NoMatchingBinary)?;
            let bytes = self
                .http
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?;
            tokio::fs::write(&manifest_path, &bytes).await?;
            bytes.to_vec()
        };
        let manifest: DistManifest = serde_json::from_slice(&contents)?;
        let platform = if cfg!(all(target_os = "linux", target_env = "musl")) {
            "unknown-linux-musl".into()
        } else {
            get_formatted_os_string()
        };
        let target = format!("{}-{platform}", std::env::consts::ARCH);
        let mut matches = Vec::new();
        for release in &manifest.releases {
            for id in &release.artifacts {
                let Some(artifact) = manifest.artifacts.get(id) else {
                    continue;
                };
                if !matches!(artifact.kind, ArtifactKind::ExecutableZip)
                    || !artifact
                        .target_triples
                        .iter()
                        .any(|triple| triple.as_str() == target)
                {
                    continue;
                }
                for asset in &artifact.assets {
                    if matches!(asset.kind, AssetKind::Executable(_))
                        && asset.name.as_deref() == Some(binary.name.as_str())
                    {
                        matches.push((release, artifact, asset));
                    }
                }
            }
        }
        let [(release, artifact, asset)] = matches.as_slice() else {
            return Err(Error::NoMatchingBinary);
        };
        let name = artifact.name.as_ref().ok_or(Error::NoMatchingBinary)?;
        let stem = name
            .strip_suffix(".tar.xz")
            .ok_or(Error::NoMatchingBinary)?;
        let url = artifact_url(&release_url, name.as_str())?;
        if let Some(host) = &release.hosting.github {
            let declared = Url::parse(&format!(
                "{}{}/",
                host.artifact_base_url.trim_end_matches('/'),
                host.artifact_download_path.trim_end_matches('/')
            ))
            .map_err(|_| Error::NoMatchingBinary)?;
            if host.owner != self.fetcher.owner
                || host.repo != self.fetcher.repo
                || declared != release_url
            {
                return Err(Error::NoMatchingBinary);
            }
        }
        let asset_path = PathBuf::from(
            asset
                .path
                .as_ref()
                .map_or(binary.name.as_str(), |path| path.as_str()),
        );
        if !is_safe_archive_path(&asset_path) {
            return Err(Error::NoMatchingBinary);
        }
        let archive_path = cache_dir.join(name.as_str());
        if !archive_path.is_file() {
            info!("Downloading binary archive from {url}");
            let archive = self
                .http
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?;
            tokio::fs::write(&archive_path, &archive).await?;
        }
        Ok((archive_path, asset_path, stem.into()))
    }
}

impl BlueprintSourceHandler for GithubBinaryFetcher {
    async fn fetch(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        let binary = get_blueprint_binary(&self.fetcher.binaries)
            .ok_or(Error::NoMatchingBinary)?
            .clone();
        let identity = serde_json::to_vec(&(
            &self.fetcher.owner,
            &self.fetcher.repo,
            &self.fetcher.tag,
            &binary.name,
            binary.sha256,
            binary.blake3,
        ))?;
        let cache_dir = cache_dir.join(format!("github-{}", hash_bytes_to_hex(identity)));
        tokio::fs::create_dir_all(&cache_dir).await?;
        let output = cache_dir.join(if cfg!(windows) {
            "binary.exe"
        } else {
            "binary"
        });
        if self.resolved_binary_path.as_ref() == Some(&output) && output.is_file() {
            return Ok(output);
        }
        self.resolved_binary_path = None;
        let (archive_path, asset_path, archive_root) = self.get_binary(&binary, &cache_dir).await?;
        if let Err(error) = verify_attestation(
            &self.github_cli,
            &self.fetcher.owner,
            &self.fetcher.repo,
            &archive_path,
        ) {
            if !self.allow_unchecked_attestations {
                return Err(error);
            }
        }
        let extracted = tempfile::tempdir_in(&cache_dir)?;
        let tar = XzDecoder::new(File::open(&archive_path)?);
        unpack_archive_safely(&mut Archive::new(tar), extracted.path())?;
        let paths = [
            extracted.path().join(&asset_path),
            extracted.path().join(archive_root).join(asset_path),
        ];
        let matches: Vec<_> = paths.iter().filter(|path| path.is_file()).collect();
        let [binary_path] = matches.as_slice() else {
            return Err(Error::NoMatchingBinary);
        };
        let binary_path = make_executable(binary_path)?;
        tokio::fs::rename(binary_path, &output).await?;
        self.resolved_binary_path = Some(output.clone());
        Ok(output)
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

fn release_url(base: &Url, fetcher: &GithubFetcher) -> Result<Url> {
    let mut url = base.clone();
    url.path_segments_mut()
        .map_err(|_| Error::NoMatchingBinary)?
        .pop_if_empty()
        .extend([
            fetcher.owner.as_str(),
            fetcher.repo.as_str(),
            "releases",
            "download",
            fetcher.tag.as_str(),
            "",
        ]);
    Ok(url)
}

fn artifact_url(release: &Url, name: &str) -> Result<Url> {
    let path = Path::new(name);
    if path.components().count() != 1 || !is_safe_archive_path(path) {
        return Err(Error::NoMatchingBinary);
    }
    let mut url = release.clone();
    url.path_segments_mut()
        .map_err(|_| Error::NoMatchingBinary)?
        .pop_if_empty()
        .push(name);
    Ok(url)
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
