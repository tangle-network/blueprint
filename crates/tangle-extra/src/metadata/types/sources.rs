use alloc::borrow::Cow;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::{Architecture, OperatingSystem, WasmRuntime};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::BlueprintSource as SubxtBlueprintSource;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::WasmFetcher as SubxtWasmFetcher;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::NativeFetcher as SubxtNativeFetcher;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::GithubFetcher as SubxtGithubFetcher;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::TestFetcher as SubxtTestFetcher;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::BlueprintBinary as SubxtBlueprintBinary;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::sources::ImageRegistryFetcher as SubxtImageRegistryFetcher;

use crate::serde::new_bounded_string;

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

impl From<BlueprintSource<'_>> for SubxtBlueprintSource {
    fn from(source: BlueprintSource<'_>) -> Self {
        match source {
            BlueprintSource::Wasm { runtime, fetcher } => SubxtBlueprintSource::Wasm {
                runtime,
                fetcher: fetcher.into(),
            },
            BlueprintSource::Native(native) => SubxtBlueprintSource::Native(native.into()),
            BlueprintSource::Container(container) => {
                SubxtBlueprintSource::Container(container.into())
            }
            BlueprintSource::Testing(testing) => SubxtBlueprintSource::Testing(testing.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum WasmFetcher<'a> {
    /// A WASM binary that will be fetched from the IPFS.
    IPFS(Vec<u8>),
    /// A WASM binary that will be fetched from a GitHub release.
    Github(GithubFetcher<'a>),
}

impl From<WasmFetcher<'_>> for SubxtWasmFetcher {
    fn from(source: WasmFetcher<'_>) -> Self {
        match source {
            WasmFetcher::IPFS(ipfs) => SubxtWasmFetcher::IPFS(BoundedVec(ipfs)),
            WasmFetcher::Github(github) => SubxtWasmFetcher::Github(github.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum NativeFetcher<'a> {
    /// A blueprint that will be fetched from the IPFS.
    IPFS(Vec<u8>),
    /// A blueprint that will be fetched from a GitHub release.
    Github(GithubFetcher<'a>),
}

impl From<NativeFetcher<'_>> for SubxtNativeFetcher {
    fn from(source: NativeFetcher<'_>) -> Self {
        match source {
            NativeFetcher::IPFS(ipfs) => SubxtNativeFetcher::IPFS(BoundedVec(ipfs)),
            NativeFetcher::Github(github) => SubxtNativeFetcher::Github(github.into()),
        }
    }
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

impl From<GithubFetcher<'_>> for SubxtGithubFetcher {
    fn from(source: GithubFetcher<'_>) -> Self {
        let GithubFetcher {
            owner,
            repo,
            tag,
            binaries,
        } = source;

        SubxtGithubFetcher {
            owner: new_bounded_string(owner),
            repo: new_bounded_string(repo),
            tag: new_bounded_string(tag),
            binaries: BoundedVec(binaries.into_iter().map(Into::into).collect()),
        }
    }
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

impl From<ImageRegistryFetcher<'_>> for SubxtImageRegistryFetcher {
    fn from(source: ImageRegistryFetcher<'_>) -> Self {
        let ImageRegistryFetcher {
            registry,
            image,
            tag,
        } = source;

        SubxtImageRegistryFetcher {
            registry: new_bounded_string(registry),
            image: new_bounded_string(image),
            tag: new_bounded_string(tag),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TestFetcher<'a> {
    pub cargo_package: Cow<'a, str>,
    pub cargo_bin: Cow<'a, str>,
    pub base_path: Cow<'a, str>,
}

impl From<TestFetcher<'_>> for SubxtTestFetcher {
    fn from(source: TestFetcher<'_>) -> Self {
        let TestFetcher {
            cargo_package,
            cargo_bin,
            base_path,
        } = source;

        SubxtTestFetcher {
            cargo_package: new_bounded_string(cargo_package),
            cargo_bin: new_bounded_string(cargo_bin),
            base_path: new_bounded_string(base_path),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlueprintBinary<'a> {
    /// CPU or System architecture.
    pub arch: Architecture,
    /// Operating System that the binary is compiled for.
    pub os: OperatingSystem,
    /// The name of the binary.
    pub name: Cow<'a, str>,
    /// The sha256 hash of the binary, used for verification.
    #[serde(default)]
    pub sha256: [u8; 32],
}

impl From<BlueprintBinary<'_>> for SubxtBlueprintBinary {
    fn from(source: BlueprintBinary<'_>) -> Self {
        let BlueprintBinary {
            arch,
            os,
            name,
            sha256,
        } = source;

        SubxtBlueprintBinary {
            arch,
            os,
            name: new_bounded_string(name),
            sha256,
        }
    }
}
