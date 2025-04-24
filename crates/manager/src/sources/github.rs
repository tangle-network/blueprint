use super::BlueprintSourceHandler;
use super::ProcessHandle;
use super::binary::{BinarySourceFetcher, generate_running_process_status_handle};
use crate::error::{Error, Result};
use crate::gadget::native::get_blueprint_binary;
use crate::sdk;
use crate::sdk::utils::{get_download_url, hash_bytes_to_hex, make_executable, valid_file_exists};
use blueprint_core::info;
use std::path::PathBuf;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::GithubFetcher;
use tokio::io::AsyncWriteExt;
use blueprint_runner::config::BlueprintEnvironment;
use crate::config::SourceCandidates;

pub struct GithubBinaryFetcher {
    pub fetcher: GithubFetcher,
    pub blueprint_id: u64,
    pub gadget_name: String,
    resolved_binary_path: Option<PathBuf>,
}

impl GithubBinaryFetcher {
    #[must_use]
    pub fn new(fetcher: GithubFetcher, blueprint_id: u64, gadget_name: String) -> Self {
        GithubBinaryFetcher {
            fetcher,
            blueprint_id,
            gadget_name,
            resolved_binary_path: None,
        }
    }
}

impl BinarySourceFetcher for GithubBinaryFetcher {
    async fn get_binary(&self) -> Result<PathBuf> {
        let relevant_binary =
            get_blueprint_binary(&self.fetcher.binaries.0).ok_or(Error::NoMatchingBinary)?;
        let expected_hash = sdk::utils::slice_32_to_sha_hex_string(relevant_binary.sha256);
        let current_dir = std::env::current_dir()?;

        let tag_str = std::str::from_utf8(&self.fetcher.tag.0.0).map_or_else(
            |_| self.fetcher.tag.0.0.escape_ascii().to_string(),
            ToString::to_string,
        );

        // TODO: !!! This is not going to work for multiple blueprints. There *will* be collisions.
        let mut binary_download_path = format!("{}/protocol-{tag_str}", current_dir.display());

        if cfg!(target_family = "windows") {
            binary_download_path += ".exe";
        }

        // Check if the binary exists, if not download it
        if valid_file_exists(&binary_download_path, &expected_hash).await {
            info!("Binary already exists at: {binary_download_path}");
            return Ok(PathBuf::from(binary_download_path));
        }

        let url = get_download_url(relevant_binary, &self.fetcher);
        info!("Downloading binary from {url} to {binary_download_path}");

        let download = reqwest::get(&url).await?.bytes().await?;
        // let retrieved_hash = hash_bytes_to_hex(&download);

        // Write the binary to disk
        let mut file = tokio::fs::File::create(&binary_download_path).await?;
        file.write_all(&download).await?;
        file.flush().await?;

        // TODO(HACK)
        // if retrieved_hash.trim() != expected_hash.trim() {
        //     return Err(Error::HashMismatch {
        //         expected: expected_hash,
        //         actual: retrieved_hash,
        //     });
        // }

        Ok(PathBuf::from(binary_download_path))
    }
}

impl BlueprintSourceHandler for GithubBinaryFetcher {
    async fn fetch(&mut self) -> Result<()> {
        if self.resolved_binary_path.is_some() {
            return Ok(());
        }

        let mut binary_path = self.get_binary().await?;

        // Ensure the binary is executable
        binary_path = make_executable(&binary_path)?;
        self.resolved_binary_path = Some(binary_path);
        Ok(())
    }

    async fn spawn(
        &mut self,
        _source_candidates: &SourceCandidates,
        _env: &BlueprintEnvironment,
        service: &str,
        args: Vec<String>,
        env_vars: Vec<(String, String)>,
    ) -> Result<ProcessHandle> {
        let binary = self.resolved_binary_path.as_ref().expect("should be set");
        let process_handle = tokio::process::Command::new(binary)
            .kill_on_drop(true)
            .stdin(std::process::Stdio::null())
            .current_dir(&std::env::current_dir()?)
            .envs(env_vars)
            .args(args)
            .spawn()?;

        let (status, abort_handle) =
            generate_running_process_status_handle(process_handle, service);

        Ok(ProcessHandle::new(status, abort_handle))
    }

    fn blueprint_id(&self) -> u64 {
        self.blueprint_id
    }

    fn name(&self) -> String {
        self.gadget_name.clone()
    }
}
