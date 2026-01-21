use crate::command::tangle::parse_address;
use alloy_json_abi::Param;
use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use alloy_sol_types::{SolType, SolValue};
use blueprint_client_tangle_evm::contracts::ITangleTypes;
use color_eyre::eyre::{Context, Result, eyre};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;
use url::Url;

pub use ITangleTypes::BlueprintDefinition;
use ITangleTypes::{
    BlueprintArchitecture, BlueprintBinary as OnChainBlueprintBinary, BlueprintConfig,
    BlueprintFetcherKind, BlueprintMetadata, BlueprintOperatingSystem, BlueprintSource,
    BlueprintSourceKind, ImageRegistrySource, JobDefinition, MembershipModel, NativeSource,
    PricingModel, TestingSource, WasmRuntime, WasmSource,
};

/// Blueprint definition payload ready to send to the contract.
#[derive(Debug, Clone)]
pub struct BlueprintDefinitionInput {
    pub encoded: Bytes,
    pub metadata_uri: String,
    pub manager: Address,
}

#[derive(Debug, Clone)]
pub struct BlueprintDefinitionLoadResult {
    pub definition: BlueprintDefinitionInput,
    pub summaries: Vec<SourceSummary>,
}

impl BlueprintDefinitionInput {
    #[must_use]
    pub fn encoded_bytes(&self) -> Bytes {
        self.encoded.clone()
    }
}

/// Decode a blueprint definition payload returned by the Tangle contract.
pub fn decode_blueprint_definition(bytes: &[u8]) -> Result<BlueprintDefinition> {
    <ITangleTypes::BlueprintDefinition as SolType>::abi_decode(bytes).map_err(|err| {
        eyre!(
            "failed to decode blueprint definition ({} bytes): {err}",
            bytes.len()
        )
    })
}

/// Load and encode a blueprint definition from disk.
pub fn load_blueprint_definition(
    path: &Path,
    overrides: Option<&DefinitionOverrides>,
) -> Result<BlueprintDefinitionLoadResult> {
    let bytes = fs::read(path).with_context(|| {
        format!(
            "failed to read blueprint definition file {}",
            path.display()
        )
    })?;

    let mut spec = parse_definition_spec(&bytes, path)?;
    if let Some(extra) = overrides {
        spec.apply_overrides(extra)?;
    }
    let summary = DefinitionSummary {
        metadata_uri: spec.metadata_uri.clone(),
        manager: parse_address(&spec.manager, "manager")?,
    };
    let summaries = spec.source_summaries();
    let definition = spec.into_blueprint_definition()?;
    let encoded = Bytes::from(definition.abi_encode());

    Ok(BlueprintDefinitionLoadResult {
        definition: BlueprintDefinitionInput {
            encoded,
            metadata_uri: summary.metadata_uri,
            manager: summary.manager,
        },
        summaries,
    })
}

fn parse_definition_spec(bytes: &[u8], path: &Path) -> Result<BlueprintDefinitionSpec> {
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();
    match ext {
        "json" => serde_json::from_slice(bytes)
            .with_context(|| format!("failed to parse JSON from {}", path.display())),
        "yaml" | "yml" => serde_yaml::from_slice(bytes)
            .with_context(|| format!("failed to parse YAML from {}", path.display())),
        "toml" => toml::from_slice(bytes)
            .with_context(|| format!("failed to parse TOML from {}", path.display())),
        _ => serde_json::from_slice(bytes)
            .or_else(|json_err| {
                toml::from_slice(bytes).map_err(|toml_err| {
                    eyre!(
                        "Failed to parse {} as JSON ({json_err}) or TOML ({toml_err})",
                        path.display()
                    )
                })
            })
            .with_context(|| format!("failed to parse definition file {}", path.display())),
    }
}

#[derive(Debug, Deserialize, Clone)]
struct BlueprintDefinitionSpec {
    metadata_uri: String,
    manager: String,
    #[serde(default)]
    master_manager_revision: u32,
    #[serde(default)]
    config: Option<BlueprintConfigSpec>,
    #[serde(default)]
    metadata: MetadataSpec,
    jobs: Vec<JobSpec>,
    #[serde(default)]
    registration_schema: Option<String>,
    #[serde(default)]
    request_schema: Option<String>,
    sources: Vec<SourceSpec>,
    #[serde(default = "default_memberships")]
    supported_memberships: Vec<MembershipModelSpec>,
}

impl BlueprintDefinitionSpec {
    fn into_blueprint_definition(self) -> Result<BlueprintDefinition> {
        if self.metadata_uri.trim().is_empty() {
            return Err(eyre!("metadata_uri must not be empty"));
        }

        if self.jobs.is_empty() {
            return Err(eyre!("definition must include at least one job"));
        }

        if self.sources.is_empty() {
            return Err(eyre!("definition must include at least one source"));
        }

        if self.supported_memberships.is_empty() {
            return Err(eyre!(
                "definition must include at least one supported membership model"
            ));
        }

        let (explicit_config, cfg_spec) = match self.config.clone() {
            Some(cfg) => (true, cfg),
            None => (false, BlueprintConfigSpec::default()),
        };

        // Determine effective membership and pricing values
        let effective_membership = cfg_spec.membership.unwrap_or(self.supported_memberships[0]);
        let effective_pricing = cfg_spec.pricing.unwrap_or_default();

        // Set hasConfig = true if we have an explicit config OR if we're using
        // non-default values. The contract defaults to Fixed/PayOnce when hasConfig
        // is false, so we must set it true to preserve Dynamic membership or other
        // non-default pricing models.
        let has_config = explicit_config
            || effective_membership != MembershipModelSpec::Fixed
            || effective_pricing != PricingModelSpec::PayOnce;

        let config = cfg_spec.into_blueprint_config(self.supported_memberships[0]);

        Ok(BlueprintDefinition {
            metadataUri: self.metadata_uri,
            manager: parse_address(&self.manager, "manager")?,
            masterManagerRevision: self.master_manager_revision,
            hasConfig: has_config,
            config,
            metadata: self.metadata.into_metadata(),
            jobs: self
                .jobs
                .into_iter()
                .map(JobSpec::into_job_definition)
                .collect::<Result<_>>()?,
            registrationSchema: hex_to_bytes(self.registration_schema.as_deref())?,
            requestSchema: hex_to_bytes(self.request_schema.as_deref())?,
            sources: self
                .sources
                .into_iter()
                .map(SourceSpec::into_blueprint_source)
                .collect::<Result<_>>()?,
            supportedMemberships: self
                .supported_memberships
                .into_iter()
                .map(MembershipModelSpec::into_membership)
                .collect(),
        })
    }

    fn apply_overrides(&mut self, overrides: &DefinitionOverrides) -> Result<()> {
        if overrides.is_empty() {
            return Ok(());
        }

        let mut retained: Vec<SourceSpec> = self
            .sources
            .drain(..)
            .filter(|spec| !matches!(spec, SourceSpec::Native(_)))
            .collect();

        for override_spec in overrides.native_sources() {
            retained.push(SourceSpec::Native(override_spec.clone().into_spec()?));
        }

        self.sources = retained;
        Ok(())
    }

    fn source_summaries(&self) -> Vec<SourceSummary> {
        self.sources
            .iter()
            .enumerate()
            .map(|(idx, source)| source.summary(idx))
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct DefinitionOverrides {
    native_sources: Vec<NativeSourceOverride>,
}

impl DefinitionOverrides {
    #[must_use]
    pub fn new(native_sources: Vec<NativeSourceOverride>) -> Self {
        Self { native_sources }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.native_sources.is_empty()
    }

    pub fn push_native(&mut self, override_spec: NativeSourceOverride) {
        self.native_sources.push(override_spec);
    }

    #[must_use]
    pub fn native_sources(&self) -> &[NativeSourceOverride] {
        &self.native_sources
    }
}

#[derive(Debug, Clone)]
pub struct NativeSourceOverride {
    fetcher: FetcherKind,
    entrypoint: String,
    github: Option<GithubArtifactSpec>,
    remote: Option<RemoteArtifactSpec>,
    binaries: Vec<BinaryArtifactSpec>,
}

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

    fn into_spec(&self) -> Result<NativeSourceSpec> {
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

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(default)]
struct BlueprintConfigSpec {
    membership: Option<MembershipModelSpec>,
    pricing: Option<PricingModelSpec>,
    min_operators: Option<u32>,
    max_operators: Option<u32>,
    subscription_rate: Option<u128>,
    subscription_interval: Option<u64>,
    event_rate: Option<u128>,
}

impl BlueprintConfigSpec {
    fn into_blueprint_config(self, default_membership: MembershipModelSpec) -> BlueprintConfig {
        BlueprintConfig {
            membership: self
                .membership
                .unwrap_or(default_membership)
                .into_membership(),
            pricing: self.pricing.unwrap_or_default().into_pricing(),
            minOperators: self.min_operators.unwrap_or_default(),
            maxOperators: self.max_operators.unwrap_or_default(),
            subscriptionRate: U256::from(self.subscription_rate.unwrap_or_default()),
            subscriptionInterval: self.subscription_interval.unwrap_or_default(),
            eventRate: U256::from(self.event_rate.unwrap_or_default()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct MetadataSpec {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default = "default_author")]
    author: String,
    #[serde(default = "default_category")]
    category: String,
    #[serde(default)]
    code_repository: String,
    #[serde(default)]
    logo: String,
    #[serde(default = "default_website")]
    website: String,
    #[serde(default = "default_license")]
    license: String,
    #[serde(default)]
    profiling_data: String,
}

impl Default for MetadataSpec {
    fn default() -> Self {
        Self {
            name: default_name(),
            description: String::new(),
            author: default_author(),
            category: default_category(),
            code_repository: String::new(),
            logo: String::new(),
            website: default_website(),
            license: default_license(),
            profiling_data: String::new(),
        }
    }
}

impl MetadataSpec {
    fn into_metadata(self) -> BlueprintMetadata {
        BlueprintMetadata {
            name: self.name,
            description: self.description,
            author: self.author,
            category: self.category,
            codeRepository: self.code_repository,
            logo: self.logo,
            website: self.website,
            license: self.license,
            profilingData: self.profiling_data,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct JobSpec {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    metadata_uri: String,
    #[serde(default)]
    params_schema: Option<String>,
    #[serde(default)]
    result_schema: Option<String>,
}

impl JobSpec {
    fn into_job_definition(self) -> Result<JobDefinition> {
        Ok(JobDefinition {
            name: self.name,
            description: self.description,
            metadataUri: self.metadata_uri,
            paramsSchema: hex_to_bytes(self.params_schema.as_deref())?,
            resultSchema: hex_to_bytes(self.result_schema.as_deref())?,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum SourceSpec {
    Container(ContainerSourceSpec),
    Native(NativeSourceSpec),
    Wasm(WasmSourceSpec),
}

impl SourceSpec {
    fn into_blueprint_source(self) -> Result<BlueprintSource> {
        let mut source = BlueprintSource::default();
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

    fn summary(&self, index: usize) -> SourceSummary {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Container,
    Native,
    Wasm,
}

#[derive(Debug, Clone)]
pub struct SourceSummary {
    pub index: usize,
    pub kind: SourceKind,
    pub fetcher: Option<FetcherKind>,
    pub entrypoint: Option<String>,
    pub details: SourceSummaryDetails,
}

#[derive(Debug, Clone)]
pub enum SourceSummaryDetails {
    Container {
        registry: String,
        image: String,
        tag: String,
    },
    Native {
        has_testing: bool,
    },
    Wasm {
        runtime: WasmRuntimeKind,
    },
}

#[derive(Debug, Deserialize, Clone)]
struct ContainerSourceSpec {
    registry: String,
    image: String,
    tag: String,
    #[serde(default)]
    testing: Option<TestingSourceSpec>,
    #[serde(default)]
    binaries: Vec<BinaryArtifactSpec>,
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

#[derive(Debug, Deserialize, Clone)]
struct NativeSourceSpec {
    #[serde(default)]
    fetcher: FetcherKind,
    #[serde(default)]
    artifact_uri: String,
    #[serde(default)]
    entrypoint: String,
    #[serde(default)]
    github: Option<GithubArtifactSpec>,
    #[serde(default)]
    remote: Option<RemoteArtifactSpec>,
    #[serde(default)]
    binaries: Vec<BinaryArtifactSpec>,
    #[serde(default)]
    testing: Option<TestingSourceSpec>,
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

#[derive(Debug, Deserialize, Clone)]
struct WasmSourceSpec {
    #[serde(default)]
    runtime: WasmRuntimeKind,
    #[serde(default)]
    fetcher: FetcherKind,
    #[serde(default)]
    artifact_uri: String,
    #[serde(default)]
    entrypoint: String,
    #[serde(default)]
    testing: Option<TestingSourceSpec>,
    #[serde(default)]
    binaries: Vec<BinaryArtifactSpec>,
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

#[derive(Debug, Deserialize, Clone)]
struct TestingSourceSpec {
    cargo_package: String,
    cargo_bin: String,
    base_path: String,
}

impl Default for TestingSourceSpec {
    fn default() -> Self {
        Self {
            cargo_package: String::new(),
            cargo_bin: String::new(),
            base_path: String::new(),
        }
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubArtifactSpec {
    pub owner: String,
    pub repo: String,
    pub tag: String,
    #[serde(default)]
    pub binaries: Vec<BinaryArtifactSpec>,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinaryArtifactSpec {
    pub name: String,
    pub arch: String,
    pub os: String,
    pub sha256: String,
    #[serde(default)]
    pub blake3: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoteArtifactSpec {
    pub dist_url: String,
    pub archive_url: String,
    #[serde(default)]
    pub binaries: Vec<BinaryArtifactSpec>,
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

    fn parse_digest(value: &str, label: &str) -> Result<FixedBytes<32>> {
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

fn parse_architecture(value: &str) -> Result<<BlueprintArchitecture as SolType>::RustType> {
    let normalized = value.trim().to_lowercase();
    let variant = match normalized.as_str() {
        "wasm32" | "wasm-32" => BlueprintArchitecture::from_underlying(0),
        "wasm64" | "wasm-64" => BlueprintArchitecture::from_underlying(1),
        "wasi32" | "wasi-32" => BlueprintArchitecture::from_underlying(2),
        "wasi64" | "wasi-64" => BlueprintArchitecture::from_underlying(3),
        "amd32" | "x86" | "i386" | "ia32" | "x86_32" => BlueprintArchitecture::from_underlying(4),
        "amd64" | "x86_64" | "x64" => BlueprintArchitecture::from_underlying(5),
        "arm32" | "armv7" | "armv6" | "arm" => BlueprintArchitecture::from_underlying(6),
        "arm64" | "aarch64" | "armv8" => BlueprintArchitecture::from_underlying(7),
        "riscv32" | "risc-v32" | "riscv-32" => BlueprintArchitecture::from_underlying(8),
        "riscv64" | "risc-v64" | "riscv-64" => BlueprintArchitecture::from_underlying(9),
        other => {
            return Err(eyre!(
                "unsupported binary architecture `{other}`, expected wasm32/64, wasi32/64, amd32/64, arm32/64, or riscv32/64"
            ));
        }
    };
    Ok(variant.into_underlying())
}

fn parse_operating_system(value: &str) -> Result<<BlueprintOperatingSystem as SolType>::RustType> {
    let normalized = value.trim().to_lowercase();
    let variant = match normalized.as_str() {
        "" => return Err(eyre!("binary os must not be empty")),
        "unknown" => BlueprintOperatingSystem::from_underlying(0),
        "linux" => BlueprintOperatingSystem::from_underlying(1),
        "windows" | "win32" | "win64" => BlueprintOperatingSystem::from_underlying(2),
        "macos" | "mac" | "osx" | "darwin" => BlueprintOperatingSystem::from_underlying(3),
        "bsd" | "freebsd" | "openbsd" | "netbsd" => BlueprintOperatingSystem::from_underlying(4),
        other => {
            return Err(eyre!(
                "unsupported binary operating system `{other}`, expected linux/windows/macos/bsd/unknown"
            ));
        }
    };
    Ok(variant.into_underlying())
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum MembershipModelSpec {
    Fixed,
    Dynamic,
}

impl MembershipModelSpec {
    fn into_membership(self) -> <MembershipModel as SolType>::RustType {
        let value = match self {
            MembershipModelSpec::Fixed => MembershipModel::from_underlying(0),
            MembershipModelSpec::Dynamic => MembershipModel::from_underlying(1),
        };
        value.into_underlying()
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PricingModelSpec {
    PayOnce,
    Subscription,
    EventDriven,
}

impl Default for PricingModelSpec {
    fn default() -> Self {
        Self::PayOnce
    }
}

impl PricingModelSpec {
    fn into_pricing(self) -> <PricingModel as SolType>::RustType {
        let value = match self {
            PricingModelSpec::PayOnce => PricingModel::from_underlying(0),
            PricingModelSpec::Subscription => PricingModel::from_underlying(1),
            PricingModelSpec::EventDriven => PricingModel::from_underlying(2),
        };
        value.into_underlying()
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FetcherKind {
    None,
    Ipfs,
    Http,
    Github,
}

impl Default for FetcherKind {
    fn default() -> Self {
        Self::None
    }
}

impl FetcherKind {
    fn into_fetcher(self) -> <BlueprintFetcherKind as SolType>::RustType {
        let value = match self {
            FetcherKind::None => BlueprintFetcherKind::from_underlying(0),
            FetcherKind::Ipfs => BlueprintFetcherKind::from_underlying(1),
            FetcherKind::Http => BlueprintFetcherKind::from_underlying(2),
            FetcherKind::Github => BlueprintFetcherKind::from_underlying(3),
        };
        value.into_underlying()
    }
}

impl fmt::Display for FetcherKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetcherKind::None => write!(f, "none"),
            FetcherKind::Ipfs => write!(f, "ipfs"),
            FetcherKind::Http => write!(f, "http"),
            FetcherKind::Github => write!(f, "github"),
        }
    }
}

impl fmt::Display for SourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceKind::Container => write!(f, "container"),
            SourceKind::Native => write!(f, "native"),
            SourceKind::Wasm => write!(f, "wasm"),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WasmRuntimeKind {
    Unknown,
    Wasmtime,
    Wasmer,
}

impl Default for WasmRuntimeKind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl WasmRuntimeKind {
    fn into_runtime(self) -> <WasmRuntime as SolType>::RustType {
        let value = match self {
            WasmRuntimeKind::Unknown => WasmRuntime::from_underlying(0),
            WasmRuntimeKind::Wasmtime => WasmRuntime::from_underlying(1),
            WasmRuntimeKind::Wasmer => WasmRuntime::from_underlying(2),
        };
        value.into_underlying()
    }
}

fn blueprint_source_kind(kind: SourceKind) -> <BlueprintSourceKind as SolType>::RustType {
    let value = match kind {
        SourceKind::Container => BlueprintSourceKind::from_underlying(0),
        SourceKind::Wasm => BlueprintSourceKind::from_underlying(1),
        SourceKind::Native => BlueprintSourceKind::from_underlying(2),
    };
    value.into_underlying()
}

fn hex_to_bytes(value: Option<&str>) -> Result<Bytes> {
    if let Some(raw) = value {
        if raw.trim().is_empty() {
            return Ok(Bytes::new());
        }
        let trimmed = raw.trim();

        // Check if this looks like JSON schema (starts with '[')
        if trimmed.starts_with('[') {
            return encode_json_schema_to_tlv(trimmed);
        }

        // Check if hex-encoded data might be JSON (0x5b = '[')
        let hex_trimmed = trimmed.strip_prefix("0x").unwrap_or(trimmed);
        if hex_trimmed.starts_with("5b") || hex_trimmed.starts_with("5B") {
            // Decode hex first, then check if it's JSON
            if let Ok(decoded) = hex::decode(hex_trimmed) {
                if let Ok(json_str) = std::str::from_utf8(&decoded) {
                    if json_str.trim().starts_with('[') {
                        return encode_json_schema_to_tlv(json_str);
                    }
                }
            }
        }

        // Otherwise treat as raw hex bytes (pre-encoded TLV or other binary format)
        let decoded = hex::decode(hex_trimmed)
            .map_err(|e| eyre!("failed to decode hex schema {raw}: {e}"))?;
        Ok(Bytes::from(decoded))
    } else {
        Ok(Bytes::new())
    }
}

/// Encode a JSON ABI schema to the TLV binary format expected by the contract.
///
/// The TLV format is:
/// - 2 bytes: uint16 field count (big-endian)
/// - For each field (5 bytes header + children recursively):
///   - 1 byte: BlueprintFieldKind enum (0-22)
///   - 2 bytes: uint16 arrayLength (big-endian)
///   - 2 bytes: uint16 childCount (big-endian)
fn encode_json_schema_to_tlv(json_str: &str) -> Result<Bytes> {
    let params: Vec<Param> =
        serde_json::from_str(json_str).map_err(|e| eyre!("failed to parse JSON schema: {e}"))?;

    if params.is_empty() {
        return Ok(Bytes::new());
    }

    let field_count = params.len();
    if field_count > u16::MAX as usize {
        return Err(eyre!("schema has too many fields (max {})", u16::MAX));
    }

    // Count total nodes needed
    let total_nodes: usize = params.iter().map(count_nodes).sum();

    // Allocate buffer: 2 bytes for field count + 5 bytes per node
    let mut buffer = vec![0u8; 2 + total_nodes * 5];

    // Write field count (big-endian)
    buffer[0] = (field_count >> 8) as u8;
    buffer[1] = (field_count & 0xFF) as u8;

    // Write each field
    let mut cursor = 2;
    for param in &params {
        cursor = write_field(&mut buffer, cursor, param)?;
    }

    Ok(Bytes::from(buffer))
}

/// Count total nodes in a schema field (including nested children).
fn count_nodes(param: &Param) -> usize {
    // For array types (type[] or type[n]), we need to count the element type as a child
    if param.ty.ends_with(']') && param.components.is_empty() {
        if let Some(bracket_pos) = param.ty.rfind('[') {
            let base_type = &param.ty[..bracket_pos];
            if !base_type.is_empty() {
                // Create synthetic child for element type
                let element_param = Param {
                    name: String::new(),
                    ty: base_type.to_string(),
                    components: vec![],
                    internal_type: None,
                };
                return 1 + count_nodes(&element_param);
            }
        }
    }
    1 + param.components.iter().map(count_nodes).sum::<usize>()
}

/// Write a single field to the buffer and return the new cursor position.
fn write_field(buffer: &mut [u8], cursor: usize, param: &Param) -> Result<usize> {
    let (kind, array_length) = parse_solidity_type(&param.ty)?;

    // For array types (kind 20 or 21) without explicit components, create synthetic child
    let (child_count, synthetic_child) =
        if (kind == 20 || kind == 21) && param.components.is_empty() && param.ty.ends_with(']') {
            if let Some(bracket_pos) = param.ty.rfind('[') {
                let base_type = &param.ty[..bracket_pos];
                if !base_type.is_empty() {
                    let element_param = Param {
                        name: String::new(),
                        ty: base_type.to_string(),
                        components: vec![],
                        internal_type: None,
                    };
                    (1, Some(element_param))
                } else {
                    (param.components.len(), None)
                }
            } else {
                (param.components.len(), None)
            }
        } else {
            (param.components.len(), None)
        };

    if child_count > u16::MAX as usize {
        return Err(eyre!("field has too many children (max {})", u16::MAX));
    }

    // Write 5-byte header
    buffer[cursor] = kind;
    buffer[cursor + 1] = (array_length >> 8) as u8;
    buffer[cursor + 2] = (array_length & 0xFF) as u8;
    buffer[cursor + 3] = (child_count >> 8) as u8;
    buffer[cursor + 4] = (child_count & 0xFF) as u8;

    // Write children recursively
    let mut next_cursor = cursor + 5;

    // If we have a synthetic child for array element type, write it
    if let Some(ref element) = synthetic_child {
        next_cursor = write_field(buffer, next_cursor, element)?;
    }

    // Write explicit children (for tuples, etc.)
    for child in &param.components {
        next_cursor = write_field(buffer, next_cursor, child)?;
    }

    Ok(next_cursor)
}

/// Parse a Solidity type string and return (BlueprintFieldKind, arrayLength).
///
/// BlueprintFieldKind enum values:
/// - Void=0, Bool=1, Uint8=2, Int8=3, Uint16=4, Int16=5, Uint32=6, Int32=7
/// - Uint64=8, Int64=9, Uint128=10, Int128=11, Uint256=12, Int256=13
/// - Address=14, Bytes32=15, FixedBytes=16, String=17, Bytes=18
/// - Optional=19, Array=20, List=21, Struct=22
fn parse_solidity_type(ty: &str) -> Result<(u8, u16)> {
    let ty = ty.trim();

    // Handle arrays: type[] or type[n]
    if ty.ends_with(']') {
        if let Some(bracket_pos) = ty.rfind('[') {
            let base_type = &ty[..bracket_pos];
            let size_str = &ty[bracket_pos + 1..ty.len() - 1];

            if size_str.is_empty() {
                // Dynamic array: type[] → List (21) with child being the element type
                // For List, arrayLength=0 (dynamic), the actual element type goes in children
                return Ok((21, 0)); // List
            } else {
                // Fixed array: type[n] → Array (20) with arrayLength=n
                let size: u16 = size_str
                    .parse()
                    .map_err(|_| eyre!("invalid array size in type '{ty}'"))?;
                return Ok((20, size)); // Array
            }
        }
    }

    // Handle tuple (struct)
    if ty == "tuple" {
        return Ok((22, 0)); // Struct
    }

    // Handle basic types
    let kind = match ty {
        "bool" => 1,
        "uint8" => 2,
        "int8" => 3,
        "uint16" => 4,
        "int16" => 5,
        "uint32" => 6,
        "int32" => 7,
        "uint64" => 8,
        "int64" => 9,
        "uint128" => 10,
        "int128" => 11,
        "uint256" | "uint" => 12,
        "int256" | "int" => 13,
        "address" => 14,
        "bytes32" => 15,
        "string" => 17,
        "bytes" => 18,
        _ => {
            // Handle fixed bytes (bytes1 to bytes32)
            if let Some(size_str) = ty.strip_prefix("bytes") {
                if let Ok(size) = size_str.parse::<u16>() {
                    if size >= 1 && size <= 32 {
                        if size == 32 {
                            return Ok((15, 0)); // Bytes32
                        }
                        return Ok((16, size)); // FixedBytes with arrayLength=size
                    }
                }
            }
            // Handle uint with explicit size
            if let Some(size_str) = ty.strip_prefix("uint") {
                if let Ok(bits) = size_str.parse::<u16>() {
                    return Ok((
                        match bits {
                            8 => 2,
                            16 => 4,
                            32 => 6,
                            64 => 8,
                            128 => 10,
                            256 => 12,
                            _ => return Err(eyre!("unsupported uint size: uint{bits}")),
                        },
                        0,
                    ));
                }
            }
            // Handle int with explicit size
            if let Some(size_str) = ty.strip_prefix("int") {
                if let Ok(bits) = size_str.parse::<u16>() {
                    return Ok((
                        match bits {
                            8 => 3,
                            16 => 5,
                            32 => 7,
                            64 => 9,
                            128 => 11,
                            256 => 13,
                            _ => return Err(eyre!("unsupported int size: int{bits}")),
                        },
                        0,
                    ));
                }
            }
            return Err(eyre!("unsupported Solidity type: '{ty}'"));
        }
    };

    Ok((kind, 0))
}

fn default_memberships() -> Vec<MembershipModelSpec> {
    vec![MembershipModelSpec::Fixed]
}

fn default_name() -> String {
    "Blueprint".into()
}

fn default_author() -> String {
    "Unknown".into()
}

fn default_category() -> String {
    "General".into()
}

fn default_website() -> String {
    "https://tangle.network".into()
}

fn default_license() -> String {
    "MIT".into()
}

struct DefinitionSummary {
    metadata_uri: String,
    manager: Address,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parses_minimal_definition() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                { "name": "square" }
            ],
            "sources": [
                {
                    "kind": "container",
                    "registry": "ghcr.io",
                    "image": "org/blueprint",
                    "tag": "v0.1.0",
                    "binaries": [
                        {
                            "name": "blueprint",
                            "arch": "x86_64",
                            "os": "linux",
                            "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        }
                    ]
                }
            ]
        });
        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let loaded = load_blueprint_definition(&path, None).unwrap();
        assert_eq!(loaded.definition.metadata_uri, "ipfs://cid");
        assert_eq!(
            loaded.definition.manager,
            Address::from_str("0x0000000000000000000000000000000000000001").unwrap()
        );
        assert!(
            !loaded.definition.encoded.is_empty(),
            "encoded definition should not be empty"
        );
    }

    #[test]
    fn round_trip_github_source_matches_metadata() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                {"name": "square"}
            ],
            "sources": [
                {
                    "kind": "native",
                    "fetcher": "github",
                    "entrypoint": "./bin/cli",
                    "binaries": [
                        {
                            "name": "cli",
                            "arch": "x86_64",
                            "os": "linux",
                            "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                            "blake3": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        }
                    ],
                    "github": {
                        "owner": "tangle",
                        "repo": "blueprint",
                        "tag": "v1.2.3",
                        "binaries": [
                            {
                                "name": "cli",
                                "arch": "x86_64",
                                "os": "linux",
                                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                                "blake3": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                            }
                        ]
                    }
                }
            ]
        });

        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let loaded = load_blueprint_definition(&path, None).unwrap();
        assert_eq!(loaded.summaries.len(), 1);
        assert_eq!(loaded.summaries[0].kind, SourceKind::Native);
        assert_eq!(loaded.summaries[0].fetcher.unwrap(), FetcherKind::Github);

        let decoded =
            decode_blueprint_definition(loaded.definition.encoded.as_ref()).expect("decode");
        assert_eq!(decoded.sources.len(), 1);
        let native_source = &decoded.sources[0];
        assert_eq!(
            native_source.kind,
            blueprint_source_kind(SourceKind::Native)
        );
        assert_eq!(
            native_source.native.fetcher,
            FetcherKind::Github.into_fetcher()
        );
        let metadata_json = native_source.native.artifactUri.clone().to_string();
        let metadata: GithubArtifactSpec = serde_json::from_str(&metadata_json).unwrap();
        assert_eq!(metadata.owner, "tangle");
        assert_eq!(metadata.repo, "blueprint");
        assert_eq!(metadata.tag, "v1.2.3");
        assert_eq!(metadata.binaries.len(), 1);
        assert_eq!(metadata.binaries[0].name, "cli");
    }

    #[test]
    fn round_trip_http_and_ipfs_sources() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                {"name": "square"}
            ],
            "sources": [
                {
                    "kind": "native",
                    "fetcher": "http",
                    "entrypoint": "./srv",
                    "binaries": [
                        {
                            "name": "srv",
                            "arch": "x86_64",
                            "os": "linux",
                            "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        }
                    ],
                    "remote": {
                        "dist_url": "https://example.com/dist.json",
                        "archive_url": "https://example.com/archive.tar.xz",
                        "binaries": [
                            {
                                "name": "srv",
                                "arch": "x86_64",
                                "os": "linux",
                                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                            }
                        ]
                    }
                },
                {
                    "kind": "native",
                    "fetcher": "ipfs",
                    "entrypoint": "./ipfs",
                    "binaries": [
                        {
                            "name": "ipfs",
                            "arch": "aarch64",
                            "os": "linux",
                            "sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        }
                    ],
                    "remote": {
                        "dist_url": "ipfs://manifest-cid",
                        "archive_url": "ipfs://archive-cid",
                        "binaries": [
                            {
                                "name": "ipfs",
                                "arch": "aarch64",
                                "os": "linux",
                                "sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                            }
                        ]
                    }
                }
            ]
        });

        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let loaded = load_blueprint_definition(&path, None).unwrap();
        assert_eq!(loaded.summaries.len(), 2);
        assert_eq!(loaded.summaries[0].fetcher.unwrap(), FetcherKind::Http);
        assert_eq!(loaded.summaries[1].fetcher.unwrap(), FetcherKind::Ipfs);

        let decoded =
            decode_blueprint_definition(loaded.definition.encoded.as_ref()).expect("decode");
        assert_eq!(decoded.sources.len(), 2);
        for (idx, fetcher) in [FetcherKind::Http, FetcherKind::Ipfs]
            .into_iter()
            .enumerate()
        {
            assert_eq!(decoded.sources[idx].native.fetcher, fetcher.into_fetcher());
        }
    }

    #[test]
    fn native_entrypoint_must_not_be_empty() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                {"name": "square"}
            ],
            "sources": [
                {
                    "kind": "native",
                    "fetcher": "github",
                    "entrypoint": "",
                    "github": {
                        "owner": "tangle",
                        "repo": "blueprint",
                        "tag": "v0.1.0",
                        "binaries": [
                            {
                                "name": "cli",
                                "arch": "x86_64",
                                "os": "linux",
                                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                            }
                        ]
                    }
                }
            ]
        });
        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let err = load_blueprint_definition(&path, None).unwrap_err();
        assert!(
            err.to_string()
                .contains("native source requires a non-empty entrypoint")
        );
    }

    #[test]
    fn remote_urls_and_binaries_are_validated() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                {"name": "square"}
            ],
            "sources": [
                {
                    "kind": "native",
                    "fetcher": "http",
                    "entrypoint": "./srv",
                    "binaries": [
                        {
                            "name": "srv",
                            "arch": "x86_64",
                            "os": "linux",
                            "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        }
                    ],
                    "remote": {
                        "dist_url": "not-a-url",
                        "archive_url": "https://example.com/archive.tar.xz",
                        "binaries": [
                            {
                                "name": "srv",
                                "arch": "x86_64",
                                "os": "linux",
                                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                            }
                        ]
                    }
                }
            ]
        });
        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let err = load_blueprint_definition(&path, None).unwrap_err();
        assert!(
            err.to_string().contains("invalid dist_url"),
            "expected 'invalid dist_url' error but got: {}",
            err
        );
    }

    #[test]
    fn remote_requires_binary_entries() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        // Test that empty top-level binaries array is rejected
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                {"name": "square"}
            ],
            "sources": [
                {
                    "kind": "native",
                    "fetcher": "http",
                    "entrypoint": "./srv",
                    "binaries": [],
                    "remote": {
                        "dist_url": "https://example.com/dist.json",
                        "archive_url": "https://example.com/archive.tar.xz",
                        "binaries": []
                    }
                }
            ]
        });
        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let err = load_blueprint_definition(&path, None).unwrap_err();
        assert!(
            err.to_string()
                .contains("native source requires at least one binary descriptor"),
            "expected 'native source requires at least one binary descriptor' error but got: {}",
            err
        );
    }

    #[test]
    fn binary_hashes_must_be_hex() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("definition.json");
        // Test that invalid hash at the native source level is rejected
        let manifest = serde_json::json!({
            "metadata_uri": "ipfs://cid",
            "manager": "0x0000000000000000000000000000000000000001",
            "jobs": [
                {"name": "square"}
            ],
            "sources": [
                {
                    "kind": "native",
                    "fetcher": "github",
                    "entrypoint": "./srv",
                    "binaries": [
                        {
                            "name": "srv",
                            "arch": "x86_64",
                            "os": "linux",
                            "sha256": "not-hex"
                        }
                    ],
                    "github": {
                        "owner": "tangle",
                        "repo": "blueprint",
                        "tag": "v0.1.0",
                        "binaries": [
                            {
                                "name": "srv",
                                "arch": "x86_64",
                                "os": "linux",
                                "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                            }
                        ]
                    }
                }
            ]
        });
        fs::write(&path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let err = load_blueprint_definition(&path, None).unwrap_err();
        assert!(
            err.to_string().contains("invalid sha256 digest"),
            "expected 'invalid sha256 digest' error but got: {}",
            err
        );
    }

    // Tests for schema TLV encoding

    #[test]
    fn encodes_simple_string_schema_to_tlv() {
        // JSON schema for a single string parameter
        let json = r#"[{"name": "greeting", "type": "string"}]"#;
        let result = encode_json_schema_to_tlv(json).unwrap();

        // Expected TLV format:
        // - 2 bytes: field count = 1 (0x00, 0x01)
        // - 5 bytes: header for string field (kind=17, arrayLength=0, childCount=0)
        assert_eq!(result.len(), 7); // 2 + 5
        assert_eq!(result[0], 0x00); // field count high byte
        assert_eq!(result[1], 0x01); // field count low byte
        assert_eq!(result[2], 17); // String kind
        assert_eq!(result[3], 0x00); // arrayLength high byte
        assert_eq!(result[4], 0x00); // arrayLength low byte
        assert_eq!(result[5], 0x00); // childCount high byte
        assert_eq!(result[6], 0x00); // childCount low byte
    }

    #[test]
    fn encodes_multiple_fields_schema_to_tlv() {
        // JSON schema for (string, uint256)
        let json = r#"[{"name": "name", "type": "string"}, {"name": "value", "type": "uint256"}]"#;
        let result = encode_json_schema_to_tlv(json).unwrap();

        // Expected TLV format:
        // - 2 bytes: field count = 2
        // - 5 bytes: string field
        // - 5 bytes: uint256 field
        assert_eq!(result.len(), 12); // 2 + 5 + 5
        assert_eq!(result[0], 0x00); // field count high byte
        assert_eq!(result[1], 0x02); // field count low byte
        assert_eq!(result[2], 17); // String kind
        assert_eq!(result[7], 12); // Uint256 kind
    }

    #[test]
    fn encodes_tuple_with_children_to_tlv() {
        // JSON schema for a tuple with nested fields
        let json = r#"[{"name": "data", "type": "tuple", "components": [{"name": "x", "type": "uint64"}, {"name": "y", "type": "bool"}]}]"#;
        let result = encode_json_schema_to_tlv(json).unwrap();

        // Expected TLV format:
        // - 2 bytes: field count = 1
        // - 5 bytes: tuple header (kind=22, arrayLength=0, childCount=2)
        // - 5 bytes: uint64 field
        // - 5 bytes: bool field
        assert_eq!(result.len(), 17); // 2 + 5 + 5 + 5
        assert_eq!(result[0], 0x00); // field count high byte
        assert_eq!(result[1], 0x01); // field count low byte
        assert_eq!(result[2], 22); // Struct kind
        assert_eq!(result[5], 0x00); // childCount high byte
        assert_eq!(result[6], 0x02); // childCount low byte = 2 children
        assert_eq!(result[7], 8); // Uint64 kind
        assert_eq!(result[12], 1); // Bool kind
    }

    #[test]
    fn encodes_dynamic_array_to_tlv() {
        // JSON schema for a dynamic array: uint256[]
        let json = r#"[{"name": "values", "type": "uint256[]"}]"#;
        let result = encode_json_schema_to_tlv(json).unwrap();

        // Expected TLV format:
        // - 2 bytes: field count = 1
        // - 5 bytes: List header (kind=21, arrayLength=0, childCount=1)
        // - 5 bytes: element type header (kind=12 for uint256)
        assert_eq!(result.len(), 12);
        assert_eq!(result[2], 21); // List kind
        assert_eq!(result[5], 0x00); // childCount high byte
        assert_eq!(result[6], 0x01); // childCount low byte = 1
        assert_eq!(result[7], 12); // element type = uint256
    }

    #[test]
    fn encodes_fixed_array_to_tlv() {
        // JSON schema for a fixed array: uint256[3]
        let json = r#"[{"name": "triple", "type": "uint256[3]"}]"#;
        let result = encode_json_schema_to_tlv(json).unwrap();

        // Expected TLV format:
        // - 2 bytes: field count = 1
        // - 5 bytes: Array header (kind=20, arrayLength=3, childCount=1)
        // - 5 bytes: element type header (kind=12 for uint256)
        assert_eq!(result.len(), 12);
        assert_eq!(result[2], 20); // Array kind
        assert_eq!(result[3], 0x00); // arrayLength high byte
        assert_eq!(result[4], 0x03); // arrayLength low byte = 3
        assert_eq!(result[5], 0x00); // childCount high byte
        assert_eq!(result[6], 0x01); // childCount low byte = 1
        assert_eq!(result[7], 12); // element type = uint256
    }

    #[test]
    fn hex_to_bytes_converts_json_schema() {
        // Test that hex_to_bytes correctly identifies and converts JSON schema
        let json = r#"[{"name": "name", "type": "string"}]"#;
        let result = hex_to_bytes(Some(json)).unwrap();

        // Should be TLV format, not raw JSON bytes
        assert_eq!(result.len(), 7); // TLV format
        assert_eq!(result[2], 17); // String kind
    }

    #[test]
    fn hex_to_bytes_converts_hex_encoded_json_schema() {
        // Test that hex_to_bytes correctly identifies hex-encoded JSON schema
        // JSON: [{"name": "name", "type": "string"}]
        // The hex starts with 0x5b which is '['
        let json = r#"[{"name": "name", "type": "string"}]"#;
        let hex_encoded = format!("0x{}", hex::encode(json.as_bytes()));
        let result = hex_to_bytes(Some(&hex_encoded)).unwrap();

        // Should be TLV format, not raw JSON bytes
        assert_eq!(result.len(), 7); // TLV format
        assert_eq!(result[2], 17); // String kind
    }

    #[test]
    fn hex_to_bytes_passes_through_tlv_bytes() {
        // Test that hex_to_bytes passes through pre-encoded TLV bytes
        // TLV for a single string field: field_count=1, kind=17, arrayLength=0, childCount=0
        let tlv_bytes = "0x0001110000000000";
        let result = hex_to_bytes(Some(tlv_bytes)).unwrap();

        // Should be passed through as-is
        assert_eq!(result.len(), 8);
        assert_eq!(result[2], 17); // String kind at offset 2
    }

    #[test]
    fn parses_all_basic_solidity_types() {
        assert_eq!(parse_solidity_type("bool").unwrap(), (1, 0));
        assert_eq!(parse_solidity_type("uint8").unwrap(), (2, 0));
        assert_eq!(parse_solidity_type("int8").unwrap(), (3, 0));
        assert_eq!(parse_solidity_type("uint16").unwrap(), (4, 0));
        assert_eq!(parse_solidity_type("int16").unwrap(), (5, 0));
        assert_eq!(parse_solidity_type("uint32").unwrap(), (6, 0));
        assert_eq!(parse_solidity_type("int32").unwrap(), (7, 0));
        assert_eq!(parse_solidity_type("uint64").unwrap(), (8, 0));
        assert_eq!(parse_solidity_type("int64").unwrap(), (9, 0));
        assert_eq!(parse_solidity_type("uint128").unwrap(), (10, 0));
        assert_eq!(parse_solidity_type("int128").unwrap(), (11, 0));
        assert_eq!(parse_solidity_type("uint256").unwrap(), (12, 0));
        assert_eq!(parse_solidity_type("int256").unwrap(), (13, 0));
        assert_eq!(parse_solidity_type("uint").unwrap(), (12, 0));
        assert_eq!(parse_solidity_type("int").unwrap(), (13, 0));
        assert_eq!(parse_solidity_type("address").unwrap(), (14, 0));
        assert_eq!(parse_solidity_type("bytes32").unwrap(), (15, 0));
        assert_eq!(parse_solidity_type("string").unwrap(), (17, 0));
        assert_eq!(parse_solidity_type("bytes").unwrap(), (18, 0));
        assert_eq!(parse_solidity_type("tuple").unwrap(), (22, 0));
    }

    #[test]
    fn parses_fixed_bytes_types() {
        assert_eq!(parse_solidity_type("bytes1").unwrap(), (16, 1));
        assert_eq!(parse_solidity_type("bytes16").unwrap(), (16, 16));
        assert_eq!(parse_solidity_type("bytes31").unwrap(), (16, 31));
        assert_eq!(parse_solidity_type("bytes32").unwrap(), (15, 0)); // bytes32 is special
    }

    #[test]
    fn parses_array_types() {
        // Dynamic array
        assert_eq!(parse_solidity_type("uint256[]").unwrap(), (21, 0));
        // Fixed array
        assert_eq!(parse_solidity_type("uint256[10]").unwrap(), (20, 10));
        assert_eq!(parse_solidity_type("address[5]").unwrap(), (20, 5));
    }
}
