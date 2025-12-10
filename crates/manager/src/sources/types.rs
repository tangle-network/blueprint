#[derive(Debug, Clone)]
pub struct TestFetcher {
    pub cargo_package: String,
    pub cargo_bin: String,
    pub base_path: String,
}

#[derive(Debug, Clone)]
pub struct GithubFetcher {
    pub owner: String,
    pub repo: String,
    pub tag: String,
    pub binaries: Vec<BlueprintBinary>,
}

#[derive(Debug, Clone)]
pub struct ImageRegistryFetcher {
    pub registry: String,
    pub image: String,
    pub tag: String,
}

#[derive(Debug, Clone)]
pub struct RemoteFetcher {
    pub dist_url: String,
    pub archive_url: String,
    pub binaries: Vec<BlueprintBinary>,
}

#[derive(Debug, Clone)]
pub struct BlueprintBinary {
    pub arch: String,
    pub os: String,
    pub name: String,
    pub sha256: [u8; 32],
    pub blake3: Option<[u8; 32]>,
}

#[derive(Debug, Clone)]
pub enum BlueprintSource {
    Testing(TestFetcher),
    Github(GithubFetcher),
    Container(ImageRegistryFetcher),
    Remote(RemoteFetcher),
}

impl BlueprintBinary {
    /// Returns true if this binary matches the current OS/arch identifiers.
    #[must_use]
    pub fn matches(&self, target_os: &str, target_arch: &str) -> bool {
        self.os.to_lowercase().contains(&target_os.to_lowercase())
            && self
                .arch
                .to_lowercase()
                .contains(&target_arch.to_lowercase())
    }
}
