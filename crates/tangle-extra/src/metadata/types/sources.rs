use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase", tag = "type")]
pub enum BlueprintSource<'a> {
    /// A blueprint that is a WASM binary that will be executed with the specified runtime.
    Wasm {
        runtime: WasmRuntime,
        #[cfg_attr(feature = "std", serde(flatten))]
        fetcher: WasmFetcher<'a>,
    },
    /// A blueprint that is a native binary that will be executed.
    Native(NativeFetcher<'a>),
    /// A blueprint contained in a container image.
    Container(ImageRegistryFetcher<'a>),
    /// A binary source used for testing the blueprint.
    Testing(TestFetcher<'a>),
}

impl Default for BlueprintSource<'_> {
    fn default() -> Self {
        BlueprintSource::Wasm {
            runtime: WasmRuntime::Wasmtime,
            fetcher: WasmFetcher::Github(GithubFetcher::default()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WasmFetcher<'a> {
    /// A WASM binary that will be fetched from the IPFS.
    IPFS(Vec<u8>),
    /// A WASM binary that will be fetched from a GitHub release.
    Github(GithubFetcher<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NativeFetcher<'a> {
    /// A blueprint that will be fetched from the IPFS.
    IPFS(Vec<u8>),
    /// A blueprint that will be fetched from a GitHub release.
    Github(GithubFetcher<'a>),
}

/// A binary that is stored in the GitHub release.
///
/// This will construct the URL to the release and download the binary.
/// The URL will be in the following format:
///
/// `https://github.com/<owner>/<repo>/releases/download/v<tag>/<path>`
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GithubFetcher<'a> {
    /// The owner of the repository.
    pub owner: Cow<'a, str>,
    /// The repository name.
    pub repo: Cow<'a, str>,
    /// The release tag of the repository.
    /// NOTE: The tag should be a valid semver tag.
    pub tag: Cow<'a, str>,
    /// The names of the binary in the release by the arch and the os.
    pub binaries: Vec<BlueprintBinary<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TestFetcher<'a> {
    pub cargo_package: Cow<'a, str>,
    pub cargo_bin: Cow<'a, str>,
    pub base_path: Cow<'a, str>,
}

/// The CPU or System architecture.
#[derive(
    Default,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Architecture {
    /// WebAssembly architecture (32-bit).
    #[default]
    Wasm,
    /// WebAssembly architecture (64-bit).
    Wasm64,
    /// WASI architecture (32-bit).
    Wasi,
    /// WASI architecture (64-bit).
    Wasi64,
    /// Amd architecture (32-bit).
    Amd,
    /// Amd64 architecture (`x86_64`).
    Amd64,
    /// Arm architecture (32-bit).
    Arm,
    /// Arm64 architecture (64-bit).
    Arm64,
    /// Risc-V architecture (32-bit).
    RiscV,
    /// Risc-V architecture (64-bit).
    RiscV64,
}

/// Operating System that the binary is compiled for.
#[derive(
    Default,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum OperatingSystem {
    /// Unknown operating system.
    /// This is used when the operating system is not known
    /// for example, for WASM, where the OS is not relevant.
    #[default]
    Unknown,
    /// Linux operating system.
    Linux,
    /// Windows operating system.
    Windows,
    /// `MacOS` operating system.
    MacOS,
    /// BSD operating system.
    BSD,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlueprintBinary<'a> {
    /// CPU or System architecture.
    pub arch: Architecture,
    /// Operating System that the binary is compiled for.
    pub os: OperatingSystem,
    /// The name of the binary.
    pub name: Cow<'a, str>,
    /// The sha256 hash of the binary.
    /// used to verify the downloaded binary.
    #[serde(default)]
    pub sha256: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ImageRegistryFetcher<'a> {
    /// The URL of the container registry.
    registry: Cow<'a, str>,
    /// The name of the image.
    image: Cow<'a, str>,
    /// The tag of the image.
    tag: Cow<'a, str>,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WasmRuntime {
    /// The WASM binary will be executed using the `WASMtime` runtime.
    Wasmtime,
    /// The WASM binary will be executed using the Wasmer runtime.
    Wasmer,
}
