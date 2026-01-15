use alloy_sol_types::SolType;
use blueprint_core::warn;
use hex;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::error::{Error, Result};
use crate::protocol::tangle_evm::client::TangleEvmProtocolClient;
use crate::protocol::tangle_evm::event_handler::{
    BlueprintMetadata as ManagerBlueprintMetadata, BlueprintMetadataProvider,
    REGISTRATION_SERVICE_ID,
};
use crate::sources::types::{
    BlueprintBinary, BlueprintSource as ManagerBlueprintSource,
    GithubFetcher as ManagerGithubFetcher, ImageRegistryFetcher, RemoteFetcher, TestFetcher,
};
use blueprint_client_tangle_evm::contracts::ITangleTypes;
use serde::Deserialize;
use serde_json;
type OnChainBlueprintSource = <ITangleTypes::BlueprintSource as SolType>::RustType;
type OnChainBlueprintBinary = <ITangleTypes::BlueprintBinary as SolType>::RustType;
type OnChainImageRegistrySource = <ITangleTypes::ImageRegistrySource as SolType>::RustType;
type OnChainTestingSource = <ITangleTypes::TestingSource as SolType>::RustType;
type OnChainNativeSource = <ITangleTypes::NativeSource as SolType>::RustType;

const SOURCE_KIND_CONTAINER: <ITangleTypes::BlueprintSourceKind as SolType>::RustType =
    ITangleTypes::BlueprintSourceKind::from_underlying(0).into_underlying();
const SOURCE_KIND_WASM: <ITangleTypes::BlueprintSourceKind as SolType>::RustType =
    ITangleTypes::BlueprintSourceKind::from_underlying(1).into_underlying();
const SOURCE_KIND_NATIVE: <ITangleTypes::BlueprintSourceKind as SolType>::RustType =
    ITangleTypes::BlueprintSourceKind::from_underlying(2).into_underlying();

const FETCHER_KIND_NONE: <ITangleTypes::BlueprintFetcherKind as SolType>::RustType =
    ITangleTypes::BlueprintFetcherKind::from_underlying(0).into_underlying();
const FETCHER_KIND_IPFS: <ITangleTypes::BlueprintFetcherKind as SolType>::RustType =
    ITangleTypes::BlueprintFetcherKind::from_underlying(1).into_underlying();
const FETCHER_KIND_HTTP: <ITangleTypes::BlueprintFetcherKind as SolType>::RustType =
    ITangleTypes::BlueprintFetcherKind::from_underlying(2).into_underlying();
const FETCHER_KIND_GITHUB: <ITangleTypes::BlueprintFetcherKind as SolType>::RustType =
    ITangleTypes::BlueprintFetcherKind::from_underlying(3).into_underlying();

/// Provider that fetches blueprint metadata and sources directly from the Tangle contracts.
pub struct OnChainMetadataProvider;

impl OnChainMetadataProvider {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    async fn build_metadata(
        client: &TangleEvmProtocolClient,
        blueprint_id: u64,
        service_id: u64,
        registration_mode: bool,
    ) -> Result<Option<ManagerBlueprintMetadata>> {
        let Some((blueprint_name, sources)) =
            Self::load_blueprint_sources(client, blueprint_id).await?
        else {
            return Ok(None);
        };

        Ok(Some(ManagerBlueprintMetadata {
            blueprint_id,
            service_id,
            name: blueprint_name,
            sources,
            registration_mode,
            registration_capture_only: false,
        }))
    }

    async fn load_blueprint_sources(
        client: &TangleEvmProtocolClient,
        blueprint_id: u64,
    ) -> Result<Option<(String, Vec<ManagerBlueprintSource>)>> {
        let inner = client.client();
        let raw_definition = match inner.get_raw_blueprint_definition(blueprint_id).await {
            Ok(bytes) => bytes,
            Err(err) => {
                warn!(
                    blueprint_id,
                    "Failed to fetch blueprint definition for metadata resolution: {err}"
                );
                return Ok(None);
            }
        };

        let (blueprint_name, onchain_sources) = match decode_blueprint_definition(&raw_definition) {
            Ok(result) => result,
            Err(err) => {
                warn!(
                    blueprint_id,
                    "Failed to decode blueprint definition payload: {err}"
                );
                return Ok(None);
            }
        };

        let sources = Self::convert_sources(&onchain_sources);

        if sources.is_empty() {
            warn!(
                blueprint_id,
                "Blueprint definition includes no supported sources"
            );
            return Ok(None);
        }

        Ok(Some((blueprint_name, sources)))
    }

    fn convert_sources(sources: &[OnChainBlueprintSource]) -> Vec<ManagerBlueprintSource> {
        let mut resolved_sources: Vec<ManagerBlueprintSource> = Vec::new();

        for source in sources {
            if source.kind == SOURCE_KIND_CONTAINER {
                if let Some(fetcher) = Self::convert_container_source(&source.container) {
                    resolved_sources.push(ManagerBlueprintSource::Container(fetcher));
                }
            } else if source.kind == SOURCE_KIND_NATIVE {
                let binaries = Self::convert_binaries(&source.binaries);
                if let Some(fetcher) = Self::convert_native_source(&source.native, binaries) {
                    resolved_sources.push(fetcher);
                }
            } else if source.kind == SOURCE_KIND_WASM {
                warn!("Ignoring WASM blueprint source; not supported by manager yet");
            } else {
                warn!("Encountered unknown blueprint source kind {}", source.kind);
            }

            if let Some(fetcher) = Self::convert_testing_source(&source.testing) {
                resolved_sources.push(ManagerBlueprintSource::Testing(fetcher));
            }
        }

        resolved_sources
    }

    fn convert_container_source(
        source: &OnChainImageRegistrySource,
    ) -> Option<ImageRegistryFetcher> {
        let registry = source.registry.clone().to_string();
        let image = source.image.clone().to_string();
        let tag = source.tag.clone().to_string();
        if registry.is_empty() && image.is_empty() && tag.is_empty() {
            return None;
        }

        Some(ImageRegistryFetcher {
            registry,
            image,
            tag,
        })
    }

    fn convert_testing_source(source: &OnChainTestingSource) -> Option<TestFetcher> {
        let cargo_package = source.cargoPackage.clone().to_string();
        let cargo_bin = source.cargoBin.clone().to_string();
        let base_path = source.basePath.clone().to_string();

        if cargo_package.is_empty() && cargo_bin.is_empty() && base_path.is_empty() {
            return None;
        }

        Some(TestFetcher {
            cargo_package,
            cargo_bin,
            base_path,
        })
    }

    fn convert_native_source(
        source: &OnChainNativeSource,
        binaries: Vec<BlueprintBinary>,
    ) -> Option<ManagerBlueprintSource> {
        if source.fetcher == FETCHER_KIND_GITHUB {
            return Self::build_github_fetcher(source.artifactUri.clone().to_string(), binaries)
                .map(ManagerBlueprintSource::Github);
        }

        if source.fetcher == FETCHER_KIND_HTTP || source.fetcher == FETCHER_KIND_IPFS {
            return Self::build_remote_fetcher(source.artifactUri.clone().to_string(), binaries)
                .map(ManagerBlueprintSource::Remote);
        }

        if source.fetcher == FETCHER_KIND_NONE {
            warn!("Native source provided without a fetcher");
        }

        None
    }

    fn build_github_fetcher(
        payload: String,
        onchain_binaries: Vec<BlueprintBinary>,
    ) -> Option<ManagerGithubFetcher> {
        if payload.trim().is_empty() {
            warn!("Github native source missing artifact metadata");
            return None;
        }

        let metadata: GithubArtifactMetadata = match serde_json::from_str(&payload) {
            Ok(data) => data,
            Err(err) => {
                warn!("Failed to parse GitHub artifact metadata: {err}");
                return None;
            }
        };

        let binaries = Self::select_binaries("GitHub", onchain_binaries, metadata.binaries);
        if binaries.is_empty() {
            warn!("No usable binaries found for GitHub artifact metadata");
            return None;
        }

        Some(ManagerGithubFetcher {
            owner: metadata.owner,
            repo: metadata.repo,
            tag: metadata.tag,
            binaries,
        })
    }

    fn build_remote_fetcher(
        payload: String,
        onchain_binaries: Vec<BlueprintBinary>,
    ) -> Option<RemoteFetcher> {
        if payload.trim().is_empty() {
            warn!("Remote native source missing artifact metadata");
            return None;
        }

        let metadata: RemoteArtifactMetadata = match serde_json::from_str(&payload) {
            Ok(data) => data,
            Err(err) => {
                warn!("Failed to parse remote artifact metadata: {err}");
                return None;
            }
        };

        let binaries = Self::select_binaries("remote", onchain_binaries, metadata.binaries);
        if binaries.is_empty() {
            warn!("No usable binaries found in remote artifact metadata");
            return None;
        }

        Some(RemoteFetcher {
            dist_url: metadata.dist_url,
            archive_url: metadata.archive_url,
            binaries,
        })
    }

    fn select_binaries(
        source_label: &str,
        onchain_binaries: Vec<BlueprintBinary>,
        metadata_binaries: Vec<GithubArtifactBinary>,
    ) -> Vec<BlueprintBinary> {
        if !onchain_binaries.is_empty() {
            return onchain_binaries;
        }

        warn!(
            "No on-chain binaries found for {source_label} source; falling back to metadata payload"
        );
        metadata_binaries
            .into_iter()
            .filter_map(|binary| match binary.try_into() {
                Ok(value) => Some(value),
                Err(err) => {
                    warn!("Skipping malformed binary entry: {err}");
                    None
                }
            })
            .collect()
    }

    fn convert_binaries(entries: &[OnChainBlueprintBinary]) -> Vec<BlueprintBinary> {
        entries
            .iter()
            .filter_map(|entry| Self::convert_binary(entry))
            .collect()
    }

    fn convert_binary(entry: &OnChainBlueprintBinary) -> Option<BlueprintBinary> {
        let arch = match Self::architecture_label(entry.arch) {
            Some(label) => label,
            None => {
                warn!(
                    "Unknown blueprint architecture discriminator {}",
                    entry.arch
                );
                return None;
            }
        };
        let os = match Self::operating_system_label(entry.os) {
            Some(label) => label,
            None => {
                warn!(
                    "Unknown blueprint operating system discriminator {}",
                    entry.os
                );
                return None;
            }
        };
        let sha256 = *entry.sha256.as_ref();

        Some(BlueprintBinary {
            arch: arch.to_string(),
            os: os.to_string(),
            name: entry.name.clone(),
            sha256,
            blake3: None,
        })
    }

    fn architecture_label(value: u8) -> Option<&'static str> {
        match value {
            0 => Some("wasm32"),
            1 => Some("wasm64"),
            2 => Some("wasi32"),
            3 => Some("wasi64"),
            4 => Some("amd32"),
            5 => Some("amd64"),
            6 => Some("arm32"),
            7 => Some("arm64"),
            8 => Some("riscv32"),
            9 => Some("riscv64"),
            _ => None,
        }
    }

    fn operating_system_label(value: u8) -> Option<&'static str> {
        match value {
            0 => Some("unknown"),
            1 => Some("linux"),
            2 => Some("windows"),
            3 => Some("macos"),
            4 => Some("bsd"),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct GithubArtifactMetadata {
    owner: String,
    repo: String,
    tag: String,
    binaries: Vec<GithubArtifactBinary>,
}

#[derive(Debug, Deserialize)]
struct GithubArtifactBinary {
    name: String,
    arch: String,
    os: String,
    sha256: String,
    #[serde(default)]
    blake3: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RemoteArtifactMetadata {
    dist_url: String,
    archive_url: String,
    binaries: Vec<GithubArtifactBinary>,
}

impl TryFrom<GithubArtifactBinary> for BlueprintBinary {
    type Error = String;

    fn try_from(value: GithubArtifactBinary) -> std::result::Result<Self, Self::Error> {
        if value.name.trim().is_empty() {
            return Err("binary entry missing name".into());
        }
        if value.arch.trim().is_empty() || value.os.trim().is_empty() {
            return Err("binary entry missing arch/os".into());
        }
        let sha256 = parse_digest(&value.sha256, "sha256")?;
        let blake3 = if let Some(digest) = value.blake3 {
            Some(parse_digest(&digest, "blake3")?)
        } else {
            None
        };

        Ok(BlueprintBinary {
            arch: value.arch,
            os: value.os,
            name: value.name,
            sha256,
            blake3,
        })
    }
}

fn parse_digest(value: &str, label: &str) -> std::result::Result<[u8; 32], String> {
    let bytes = hex::decode(value.trim().trim_start_matches("0x"))
        .map_err(|err| format!("invalid {label}: {err}"))?;
    if bytes.len() != 32 {
        return Err(format!("{label} digest must be 32 bytes"));
    }
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&bytes);
    Ok(buf)
}

impl Default for OnChainMetadataProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl BlueprintMetadataProvider for OnChainMetadataProvider {
    async fn resolve_service(
        &self,
        client: &TangleEvmProtocolClient,
        service_id: u64,
    ) -> Result<Option<ManagerBlueprintMetadata>> {
        let inner = client.client();
        let service = match inner.get_service(service_id).await {
            Ok(svc) => svc,
            Err(err) => return Err(Error::from(err)),
        };

        Self::build_metadata(client, service.blueprintId, service_id, false).await
    }

    async fn resolve_registration(
        &self,
        client: &TangleEvmProtocolClient,
        blueprint_id: u64,
    ) -> Result<Option<ManagerBlueprintMetadata>> {
        Self::build_metadata(client, blueprint_id, REGISTRATION_SERVICE_ID, true).await
    }
}

fn decode_blueprint_definition(data: &[u8]) -> Result<(String, Vec<OnChainBlueprintSource>)> {
    // Use proper ABI decoding for the entire BlueprintDefinition struct
    let definition =
        <ITangleTypes::BlueprintDefinition as SolType>::abi_decode(data).map_err(|e| {
            Error::Other(format!(
                "Failed to decode blueprint definition ({} bytes): {e}",
                data.len()
            ))
        })?;

    let blueprint_name = definition.metadata.name.clone();
    let sources = definition.sources;

    Ok((blueprint_name, sources))
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::FixedBytes;
    use serde_json::json;

    #[test]
    fn convert_sources_keeps_container_and_remote() {
        let mut container: OnChainBlueprintSource = Default::default();
        container.kind = SOURCE_KIND_CONTAINER;
        container.container = ITangleTypes::ImageRegistrySource {
            registry: "ghcr.io".into(),
            image: "demo/app".into(),
            tag: "v1.0.0".into(),
        };
        container.testing = ITangleTypes::TestingSource {
            cargoPackage: "pkg".into(),
            cargoBin: "bin".into(),
            basePath: "/tmp/tests".into(),
        };

        let metadata = json!({
            "dist_url": "https://example.com/dist.json",
            "archive_url": "https://example.com/archive.tar.xz",
            "binaries": [{
                "name": "demo",
                "arch": "x86_64",
                "os": "linux",
                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }]
        })
        .to_string();

        let mut native: OnChainBlueprintSource = Default::default();
        native.kind = SOURCE_KIND_NATIVE;
        native.native.fetcher = FETCHER_KIND_HTTP;
        native.native.artifactUri = metadata.into();
        native.native.entrypoint = "./demo".into();
        native.testing = ITangleTypes::TestingSource {
            cargoPackage: "pkg-native".into(),
            cargoBin: "bin-native".into(),
            basePath: "/tmp/native".into(),
        };

        let sources = vec![container, native];
        let converted = OnChainMetadataProvider::convert_sources(&sources);
        assert_eq!(converted.len(), 4);

        match &converted[0] {
            ManagerBlueprintSource::Container(fetcher) => {
                assert_eq!(fetcher.image, "demo/app");
            }
            _ => panic!("expected container source"),
        }

        assert!(matches!(converted[1], ManagerBlueprintSource::Testing(_)));

        match &converted[2] {
            ManagerBlueprintSource::Remote(fetcher) => {
                assert_eq!(fetcher.binaries.len(), 1);
                assert_eq!(fetcher.binaries[0].name, "demo");
            }
            other => panic!("expected remote source, got {other:?}"),
        }

        assert!(matches!(converted[3], ManagerBlueprintSource::Testing(_)));
    }

    #[test]
    fn uses_onchain_binaries_when_available() {
        let mut native: OnChainBlueprintSource = Default::default();
        native.kind = SOURCE_KIND_NATIVE;
        native.native.fetcher = FETCHER_KIND_HTTP;
        native.native.entrypoint = "./demo".into();
        native.native.artifactUri = json!({
            "dist_url": "https://example.com/dist.json",
            "archive_url": "https://example.com/archive.tar.xz",
            "binaries": []
        })
        .to_string()
        .into();
        native.binaries = vec![ITangleTypes::BlueprintBinary {
            arch: ITangleTypes::BlueprintArchitecture::from_underlying(5).into_underlying(),
            os: ITangleTypes::BlueprintOperatingSystem::from_underlying(1).into_underlying(),
            name: "demo".into(),
            sha256: FixedBytes::<32>::from([0x11; 32]),
        }];

        let converted = OnChainMetadataProvider::convert_sources(&[native]);
        assert_eq!(converted.len(), 1);
        match &converted[0] {
            ManagerBlueprintSource::Remote(fetcher) => {
                assert_eq!(fetcher.binaries.len(), 1);
                assert_eq!(fetcher.binaries[0].arch, "amd64");
                assert_eq!(fetcher.binaries[0].os, "linux");
                assert_eq!(
                    fetcher.binaries[0].sha256, [0x11u8; 32],
                    "should use on-chain digest"
                );
            }
            other => panic!("expected remote source, got {other:?}"),
        }
    }

    #[test]
    fn falls_back_to_metadata_when_onchain_missing() {
        let mut native: OnChainBlueprintSource = Default::default();
        native.kind = SOURCE_KIND_NATIVE;
        native.native.fetcher = FETCHER_KIND_HTTP;
        native.native.entrypoint = "./demo".into();
        native.native.artifactUri = json!({
            "dist_url": "https://example.com/dist.json",
            "archive_url": "https://example.com/archive.tar.xz",
            "binaries": [{
                "name": "demo",
                "arch": "amd64",
                "os": "linux",
                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }]
        })
        .to_string()
        .into();

        let converted = OnChainMetadataProvider::convert_sources(&[native]);
        assert_eq!(converted.len(), 1);
        match &converted[0] {
            ManagerBlueprintSource::Remote(fetcher) => {
                assert_eq!(fetcher.binaries.len(), 1);
                assert_eq!(fetcher.binaries[0].name, "demo");
                assert_eq!(fetcher.binaries[0].sha256[0], 0xaa);
            }
            other => panic!("expected remote source, got {other:?}"),
        }
    }
}
