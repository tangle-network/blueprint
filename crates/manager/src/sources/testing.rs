use super::BlueprintSourceHandler;
use crate::error::{Error, Result};
use crate::sdk::utils::make_executable;
use blueprint_core::trace;
use std::path::{Path, PathBuf};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::TestFetcher;

pub struct TestSourceFetcher {
    pub fetcher: TestFetcher,
    pub blueprint_id: u64,
    pub blueprint_name: String,
    resolved_binary_path: Option<PathBuf>,
}

impl TestSourceFetcher {
    #[must_use]
    pub fn new(fetcher: TestFetcher, blueprint_id: u64, blueprint_name: String) -> Self {
        Self {
            fetcher,
            blueprint_id,
            blueprint_name,
            resolved_binary_path: None,
        }
    }

    async fn get_binary(&mut self, _cache_dir: &Path) -> Result<PathBuf> {
        let TestFetcher {
            cargo_package,
            base_path,
            ..
        } = &self.fetcher;
        let cargo_bin = String::from_utf8(cargo_package.0.0.clone())
            .map_err(|err| Error::Other(format!("Failed to parse `cargo_bin`: {:?}", err)))?;
        let base_path_str = String::from_utf8(base_path.0.0.clone())
            .map_err(|err| Error::Other(format!("Failed to parse `base_path`: {:?}", err)))?;
        let git_repo_root = get_git_repo_root_path().await?;

        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        let base_path = std::path::absolute(git_repo_root.join(&base_path_str))?;

        let target_dir = match std::env::var("CARGO_TARGET_DIR") {
            Ok(target) => PathBuf::from(target),
            Err(_) => git_repo_root.join(&base_path).join("target"),
        };

        let binary_path = target_dir.join(profile).join(&cargo_bin);
        let binary_path = std::path::absolute(&binary_path)?;

        trace!("Base Path: {}", base_path.display());
        trace!("Binary Path: {}", binary_path.display());

        // Run cargo build on the cargo_bin and ensure it build to the binary_path
        let mut command = tokio::process::Command::new("cargo");
        command
            .arg("build")
            .arg(format!("--target-dir={}", target_dir.display()))
            .arg("--bin")
            .arg(&cargo_bin);

        if !cfg!(debug_assertions) {
            command.arg("--release");
        }

        trace!("Running build command in {}", base_path.display());
        let output = command.current_dir(&base_path).output().await.unwrap();
        trace!("Build command run");
        if !output.status.success() {
            blueprint_core::warn!("Failed to build binary");
            return Err(Error::BuildBinary(output));
        }
        unsafe {
            // Set the environment variable to indicate that the binary was built for testing
            std::env::set_var("BLUEPRINT_BINARY_TEST_BUILD", "true");
        }

        trace!("Successfully built binary");

        Ok(binary_path)
    }
}

async fn get_git_repo_root_path() -> Result<PathBuf> {
    // Run a process to determine the root directory for this repo
    let output = tokio::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .await?;

    if !output.status.success() {
        return Err(Error::FetchGitRoot(output));
    }

    Ok(PathBuf::from(String::from_utf8(output.stdout)?.trim()))
}

impl BlueprintSourceHandler for TestSourceFetcher {
    async fn fetch(&mut self, cache_dir: &Path) -> Result<PathBuf> {
        if let Some(binary_path) = &self.resolved_binary_path {
            return Ok(binary_path.clone());
        }

        let mut binary_path = self.get_binary(cache_dir).await?;

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
