use crate::command::tangle::parse_address;
use alloy_primitives::{Address, Bytes, FixedBytes, U256, keccak256};
use alloy_sol_types::{SolCall, SolType, SolValue, sol};
use blueprint_client_tangle::{ExecutionProfile, inject_execution_profile};
use color_eyre::eyre::{Context, Result, eyre};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

mod schema;
mod sources;

use self::schema::hex_to_bytes;

sol! {
    struct BlueprintConfig {
        uint8 membership;
        uint8 pricing;
        uint32 minOperators;
        uint32 maxOperators;
        uint256 subscriptionRate;
        uint64 subscriptionInterval;
        uint256 eventRate;
    }

    struct BlueprintMetadata {
        string name;
        string description;
        string author;
        string category;
        string codeRepository;
        string logo;
        string website;
        string license;
        string profilingData;
    }

    struct JobDefinition {
        string name;
        string description;
        string metadataUri;
        bytes paramsSchema;
        bytes resultSchema;
    }

    struct BlueprintBinary {
        uint8 arch;
        uint8 os;
        string name;
        bytes32 sha256;
    }

    struct ImageRegistrySource {
        string registry;
        string image;
        string tag;
    }

    struct WasmSource {
        uint8 runtime;
        uint8 fetcher;
        string artifactUri;
        string entrypoint;
    }

    struct NativeSource {
        uint8 fetcher;
        string artifactUri;
        string entrypoint;
    }

    struct TestingSource {
        string cargoPackage;
        string cargoBin;
        string basePath;
    }

    struct BlueprintSource {
        uint8 kind;
        ImageRegistrySource container;
        WasmSource wasm;
        NativeSource native;
        TestingSource testing;
        BlueprintBinary[] binaries;
    }

    struct BlueprintDefinition {
        string metadataUri;
        bytes32 metadataHash;
        address manager;
        uint32 masterManagerRevision;
        bool hasConfig;
        BlueprintConfig config;
        BlueprintMetadata metadata;
        JobDefinition[] jobs;
        bytes registrationSchema;
        bytes requestSchema;
        BlueprintSource[] sources;
        uint8[] supportedMemberships;
    }

    function createBlueprint(BlueprintDefinition definition) external returns (uint64 blueprintId);
}

type OnChainBlueprintBinary = BlueprintBinary;

/// Blueprint definition payload ready to send to the contract.
#[derive(Debug, Clone)]
pub struct BlueprintDefinitionInput {
    pub encoded: Bytes,
    pub call_data: Bytes,
    pub metadata_uri: String,
    pub metadata_hash: FixedBytes<32>,
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

    #[must_use]
    pub fn call_data_bytes(&self) -> Bytes {
        self.call_data.clone()
    }
}

/// Decode a blueprint definition payload returned by the Tangle contract.
pub fn decode_blueprint_definition(bytes: &[u8]) -> Result<BlueprintDefinition> {
    <BlueprintDefinition as SolType>::abi_decode(bytes).map_err(|err| {
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
    let metadata_hash = spec.resolve_metadata_hash(path)?;
    let summary = DefinitionSummary {
        metadata_uri: spec.metadata_uri.clone(),
        metadata_hash,
        manager: parse_address(&spec.manager, "manager")?,
    };
    let summaries = spec.source_summaries();
    let definition = spec.into_blueprint_definition(metadata_hash)?;
    let encoded = Bytes::from(definition.abi_encode());
    let call_data = Bytes::from(createBlueprintCall { definition }.abi_encode());

    Ok(BlueprintDefinitionLoadResult {
        definition: BlueprintDefinitionInput {
            encoded,
            call_data,
            metadata_uri: summary.metadata_uri,
            metadata_hash: summary.metadata_hash,
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

fn compute_metadata_hash_from_file(path: &Path) -> Result<FixedBytes<32>> {
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read metadata file {}", path.display()))?;
    let value: Value = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse metadata JSON from {}", path.display()))?;
    compute_metadata_hash_from_value(value)
}

fn compute_metadata_hash_from_value(value: Value) -> Result<FixedBytes<32>> {
    let mut object = value
        .as_object()
        .cloned()
        .ok_or_else(|| eyre!("metadata_file must contain a top-level JSON object"))?;
    object.remove("integrity");
    let canonical = canonicalize_json_value(Value::Object(object));
    Ok(FixedBytes::from(
        keccak256(canonical.to_string().as_bytes()).0,
    ))
}

fn canonicalize_json_value(value: Value) -> Value {
    match value {
        Value::Array(values) => {
            Value::Array(values.into_iter().map(canonicalize_json_value).collect())
        }
        Value::Object(map) => {
            let mut entries = map.into_iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            let mut sorted = serde_json::Map::new();
            for (key, nested) in entries {
                sorted.insert(key, canonicalize_json_value(nested));
            }
            Value::Object(sorted)
        }
        other => other,
    }
}

#[derive(Debug, Deserialize, Clone)]
struct BlueprintDefinitionSpec {
    metadata_uri: String,
    #[serde(default)]
    metadata_hash: Option<String>,
    #[serde(default)]
    metadata_file: Option<PathBuf>,
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
    fn into_blueprint_definition(
        self,
        metadata_hash: FixedBytes<32>,
    ) -> Result<BlueprintDefinition> {
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
            metadataHash: metadata_hash,
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

    fn resolve_metadata_hash(&self, definition_path: &Path) -> Result<FixedBytes<32>> {
        self.resolve_metadata_hash_inner(definition_path.parent())
    }

    fn resolve_metadata_hash_inner(&self, definition_dir: Option<&Path>) -> Result<FixedBytes<32>> {
        if let Some(metadata_hash) = &self.metadata_hash {
            return BinaryArtifactSpec::parse_digest(metadata_hash, "metadata_hash");
        }

        let metadata_file = self.metadata_file.as_ref().ok_or_else(|| {
            eyre!(
                "metadata_hash is required unless metadata_file is provided for canonical hash computation"
            )
        })?;
        let resolved_path = if metadata_file.is_absolute() {
            metadata_file.clone()
        } else if let Some(base_dir) = definition_dir {
            base_dir.join(metadata_file)
        } else {
            metadata_file.clone()
        };

        compute_metadata_hash_from_file(&resolved_path)
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
    #[serde(default)]
    execution_profile: Option<ExecutionProfileSpec>,
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
            execution_profile: None,
        }
    }
}

impl MetadataSpec {
    fn into_metadata(self) -> BlueprintMetadata {
        let execution_profile = self
            .execution_profile
            .as_ref()
            .map(ExecutionProfileSpec::to_profile);
        let profiling_data = execution_profile
            .map(|profile| inject_execution_profile(&self.profiling_data, profile))
            .unwrap_or(self.profiling_data);

        BlueprintMetadata {
            name: self.name,
            description: self.description,
            author: self.author,
            category: self.category,
            codeRepository: self.code_repository,
            logo: self.logo,
            website: self.website,
            license: self.license,
            profilingData: profiling_data,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct ExecutionProfileSpec {
    #[serde(default)]
    confidentiality: blueprint_client_tangle::ConfidentialityPolicy,
    #[serde(default)]
    gpu: blueprint_client_tangle::GpuRequirements,
}

impl ExecutionProfileSpec {
    fn to_profile(&self) -> ExecutionProfile {
        ExecutionProfile {
            confidentiality: self.confidentiality,
            gpu: self.gpu.clone(),
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubArtifactSpec {
    pub owner: String,
    pub repo: String,
    pub tag: String,
    #[serde(default)]
    pub binaries: Vec<BinaryArtifactSpec>,
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

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum MembershipModelSpec {
    Fixed,
    Dynamic,
}

impl MembershipModelSpec {
    fn into_membership(self) -> u8 {
        match self {
            MembershipModelSpec::Fixed => 0,
            MembershipModelSpec::Dynamic => 1,
        }
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
    fn into_pricing(self) -> u8 {
        match self {
            PricingModelSpec::PayOnce => 0,
            PricingModelSpec::Subscription => 1,
            PricingModelSpec::EventDriven => 2,
        }
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
    fn into_fetcher(self) -> u8 {
        match self {
            FetcherKind::None => 0,
            FetcherKind::Ipfs => 1,
            FetcherKind::Http => 2,
            FetcherKind::Github => 3,
        }
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
    fn into_runtime(self) -> u8 {
        match self {
            WasmRuntimeKind::Unknown => 0,
            WasmRuntimeKind::Wasmtime => 1,
            WasmRuntimeKind::Wasmer => 2,
        }
    }
}

fn blueprint_source_kind(kind: SourceKind) -> u8 {
    match kind {
        SourceKind::Container => 0,
        SourceKind::Wasm => 1,
        SourceKind::Native => 2,
    }
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
    metadata_hash: FixedBytes<32>,
    manager: Address,
}
