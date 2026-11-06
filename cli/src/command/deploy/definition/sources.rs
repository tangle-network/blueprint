use super::{
    BinaryArtifactSpec, BlueprintSource, ContainerSourceSpec, FetcherKind, GithubArtifactSpec,
    ImageRegistrySource, NativeSource, NativeSourceOverride, NativeSourceSpec,
    OnChainBlueprintBinary, RemoteArtifactSpec, SourceKind, SourceSpec, SourceSummary,
    SourceSummaryDetails, TestingSource, TestingSourceSpec, WasmSource, WasmSourceSpec,
    blueprint_source_kind,
};
use alloy_primitives::FixedBytes;
use color_eyre::eyre::{Result, eyre};
use std::collections::HashSet;
use url::Url;

impl NativeSourceOverride {
    #[must_use]
    pub(crate) fn github(entrypoint: String, spec: GithubArtifactSpec) -> Self {
        let binaries = spec.binaries.clone();
        Self {
            fetcher: FetcherKind::Github,
            entrypoint,
            github: Some(spec),
            remote: None,
            binaries,
        }
    }

    #[must_use]
    pub(crate) fn remote(
        entrypoint: String,
        fetcher: FetcherKind,
        spec: RemoteArtifactSpec,
    ) -> Self {
        let binaries = spec.binaries.clone();
        Self {
            fetcher,
            entrypoint,
            github: None,
            remote: Some(spec),
            binaries,
        }
    }

    pub(super) fn into_spec(&self) -> Result<NativeSourceSpec> {
        if self.fetcher == FetcherKind::None {
            return Err(eyre!(
                "cannot override native source with fetcher `none`, please select github/http/ipfs"
            ));
        }

        Ok(NativeSourceSpec {
            fetcher: self.fetcher,
            artifact_uri: String::new(),
            entrypoint: self.entrypoint.clone(),
            github: self.github.clone(),
            remote: self.remote.clone(),
            binaries: self.binaries.clone(),
            testing: None,
        })
    }
}

impl SourceSpec {
    pub(super) fn into_blueprint_source(self) -> Result<BlueprintSource> {
        let mut source = BlueprintSource {
            kind: 0,
            container: ImageRegistrySource {
                registry: String::new(),
                image: String::new(),
                tag: String::new(),
            },
            wasm: WasmSource {
                runtime: 0,
                fetcher: 0,
                artifactUri: String::new(),
                entrypoint: String::new(),
            },
            native: NativeSource {
                fetcher: 0,
                artifactUri: String::new(),
                entrypoint: String::new(),
            },
            testing: TestingSource {
                cargoPackage: String::new(),
                cargoBin: String::new(),
                basePath: String::new(),
            },
            binaries: Vec::new(),
        };
        match self {
            SourceSpec::Container(spec) => {
                let (container, binaries, testing) = spec.into_parts()?;
                source.kind = blueprint_source_kind(SourceKind::Container);
                source.container = container;
                source.testing = testing.unwrap_or_default().into_testing();
                source.binaries = binaries;
            }
            SourceSpec::Native(spec) => {
                let (native, binaries, testing) = spec.into_parts()?;
                source.kind = blueprint_source_kind(SourceKind::Native);
                source.native = native;
                source.testing = testing.unwrap_or_default().into_testing();
                source.binaries = binaries;
            }
            SourceSpec::Wasm(spec) => {
                let (wasm, binaries, testing) = spec.into_parts()?;
                source.kind = blueprint_source_kind(SourceKind::Wasm);
                source.wasm = wasm;
                source.testing = testing.unwrap_or_default().into_testing();
                source.binaries = binaries;
            }
        }
        Ok(source)
    }

    pub(super) fn summary(&self, index: usize) -> SourceSummary {
        match self {
            SourceSpec::Container(spec) => SourceSummary {
                index,
                kind: SourceKind::Container,
                fetcher: None,
                entrypoint: None,
                details: SourceSummaryDetails::Container {
                    registry: spec.registry.clone(),
                    image: spec.image.clone(),
                    tag: spec.tag.clone(),
                },
            },
            SourceSpec::Native(spec) => SourceSummary {
                index,
                kind: SourceKind::Native,
                fetcher: Some(spec.fetcher),
                entrypoint: Some(spec.entrypoint.clone()),
                details: SourceSummaryDetails::Native {
                    has_testing: spec.testing.is_some(),
                },
            },
            SourceSpec::Wasm(spec) => SourceSummary {
                index,
                kind: SourceKind::Wasm,
                fetcher: Some(spec.fetcher),
                entrypoint: Some(spec.entrypoint.clone()),
                details: SourceSummaryDetails::Wasm {
                    runtime: spec.runtime,
                },
            },
        }
    }
}

impl ContainerSourceSpec {
    fn into_parts(
        self,
    ) -> Result<(
        ImageRegistrySource,
        Vec<OnChainBlueprintBinary>,
        Option<TestingSourceSpec>,
    )> {
        let container = ImageRegistrySource {
            registry: self.registry,
            image: self.image,
            tag: self.tag,
        };
        let binaries = convert_binary_specs("container source", self.binaries)?;
        Ok((container, binaries, self.testing))
    }
}

impl NativeSourceSpec {
    fn into_parts(
        self,
    ) -> Result<(
        NativeSource,
        Vec<OnChainBlueprintBinary>,
        Option<TestingSourceSpec>,
    )> {
        if self.entrypoint.trim().is_empty() {
            return Err(eyre!("native source requires a non-empty entrypoint"));
        }

        let binaries = convert_binary_specs("native source", self.binaries)?;

        let artifact_uri = match self.fetcher {
            FetcherKind::Github => {
                let spec = self
                    .github
                    .ok_or_else(|| eyre!("github native source requires `github` metadata"))?;
                spec.validate()?;
                serde_json::to_string(&spec)
                    .map_err(|err| eyre!("failed to serialize GitHub artifact: {err}"))?
            }
            FetcherKind::Http | FetcherKind::Ipfs => {
                let spec = self
                    .remote
                    .ok_or_else(|| eyre!("http/ipfs native source requires `remote` metadata"))?;
                spec.validate()?;
                serde_json::to_string(&spec)
                    .map_err(|err| eyre!("failed to serialize remote artifact metadata: {err}"))?
            }
            FetcherKind::None => {
                if self.artifact_uri.trim().is_empty() {
                    return Err(eyre!(
                        "native fetcher `none` requires `artifact_uri` to be configured"
                    ));
                }
                self.artifact_uri
            }
        };

        let native = NativeSource {
            fetcher: self.fetcher.into_fetcher(),
            artifactUri: artifact_uri,
            entrypoint: self.entrypoint,
        };
        Ok((native, binaries, self.testing))
    }
}

impl WasmSourceSpec {
    fn into_parts(
        self,
    ) -> Result<(
        WasmSource,
        Vec<OnChainBlueprintBinary>,
        Option<TestingSourceSpec>,
    )> {
        let wasm = WasmSource {
            runtime: self.runtime.into_runtime(),
            fetcher: self.fetcher.into_fetcher(),
            artifactUri: self.artifact_uri,
            entrypoint: self.entrypoint,
        };
        let binaries = convert_binary_specs("wasm source", self.binaries)?;
        Ok((wasm, binaries, self.testing))
    }
}

impl TestingSourceSpec {
    fn into_testing(self) -> TestingSource {
        TestingSource {
            cargoPackage: self.cargo_package,
            cargoBin: self.cargo_bin,
            basePath: self.base_path,
        }
    }
}

impl GithubArtifactSpec {
    fn validate(&self) -> Result<()> {
        if self.owner.trim().is_empty() {
            return Err(eyre!("github native source requires a non-empty owner"));
        }
        if self.repo.trim().is_empty() {
            return Err(eyre!("github native source requires a non-empty repo"));
        }
        if self.tag.trim().is_empty() {
            return Err(eyre!("github native source requires a non-empty tag"));
        }
        if self.binaries.is_empty() {
            return Err(eyre!(
                "github native source binaries list must not be empty"
            ));
        }
        for binary in &self.binaries {
            binary.validate()?;
        }
        Ok(())
    }
}

impl RemoteArtifactSpec {
    fn validate(&self) -> Result<()> {
        Url::parse(&self.dist_url)
            .map_err(|err| eyre!("invalid dist_url `{}`: {err}", self.dist_url))?;
        Url::parse(&self.archive_url)
            .map_err(|err| eyre!("invalid archive_url `{}`: {err}", self.archive_url))?;
        Ok(())
    }
}

impl BinaryArtifactSpec {
    fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(eyre!("binary name must not be empty"));
        }
        if self.arch.trim().is_empty() {
            return Err(eyre!("binary arch must not be empty"));
        }
        if self.os.trim().is_empty() {
            return Err(eyre!("binary os must not be empty"));
        }
        let _ = Self::parse_digest(&self.sha256, "sha256")?;
        if let Some(blake3) = &self.blake3 {
            let _ = Self::parse_digest(blake3, "blake3")?;
        }
        Ok(())
    }

    pub(super) fn parse_digest(value: &str, label: &str) -> Result<FixedBytes<32>> {
        let trimmed = value.trim().trim_start_matches("0x");
        let bytes =
            hex::decode(trimmed).map_err(|err| eyre!("invalid {label} digest `{value}`: {err}"))?;
        if bytes.len() != 32 {
            return Err(eyre!(
                "{label} digest must be 32 bytes (got {} bytes)",
                bytes.len()
            ));
        }
        let mut buf = [0u8; 32];
        buf.copy_from_slice(&bytes);
        Ok(FixedBytes::from(buf))
    }

    fn into_blueprint_binary(self) -> Result<OnChainBlueprintBinary> {
        let sha256 = Self::parse_digest(&self.sha256, "sha256")?;
        let arch = parse_architecture(&self.arch)?;
        let os = parse_operating_system(&self.os)?;
        Ok(OnChainBlueprintBinary {
            arch,
            os,
            name: self.name,
            sha256,
        })
    }
}

fn convert_binary_specs(
    label: &str,
    specs: Vec<BinaryArtifactSpec>,
) -> Result<Vec<OnChainBlueprintBinary>> {
    if specs.is_empty() {
        return Err(eyre!("{label} requires at least one binary descriptor"));
    }
    let mut dedup = HashSet::new();
    let mut converted = Vec::new();
    for spec in specs {
        spec.validate()?;
        let binary = spec.into_blueprint_binary()?;
        let digest = binary.sha256;
        if dedup.insert(digest) {
            converted.push(binary);
        }
    }
    Ok(converted)
}

fn parse_architecture(value: &str) -> Result<u8> {
    let normalized = value.trim().to_lowercase();
    let variant = match normalized.as_str() {
        "wasm32" | "wasm-32" => 0,
        "wasm64" | "wasm-64" => 1,
        "wasi32" | "wasi-32" => 2,
        "wasi64" | "wasi-64" => 3,
        "amd32" | "x86" | "i386" | "ia32" | "x86_32" => 4,
        "amd64" | "x86_64" | "x64" => 5,
        "arm32" | "armv7" | "armv6" | "arm" => 6,
        "arm64" | "aarch64" | "armv8" => 7,
        "riscv32" | "risc-v32" | "riscv-32" => 8,
        "riscv64" | "risc-v64" | "riscv-64" => 9,
        other => {
            return Err(eyre!(
                "unsupported binary architecture `{other}`, expected wasm32/64, wasi32/64, amd32/64, arm32/64, or riscv32/64"
            ));
        }
    };
    Ok(variant)
}

fn parse_operating_system(value: &str) -> Result<u8> {
    let normalized = value.trim().to_lowercase();
    let variant = match normalized.as_str() {
        "" => return Err(eyre!("binary os must not be empty")),
        "unknown" => 0,
        "linux" => 1,
        "windows" | "win32" | "win64" => 2,
        "macos" | "mac" | "osx" | "darwin" => 3,
        "bsd" | "freebsd" | "openbsd" | "netbsd" => 4,
        other => {
            return Err(eyre!(
                "unsupported binary operating system `{other}`, expected linux/windows/macos/bsd/unknown"
            ));
        }
    };
    Ok(variant)
}
