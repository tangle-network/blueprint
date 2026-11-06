//! Resource specification for cloud provisioning

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Essential resource specification for deployments
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceSpec {
    /// Optional provider constraint; absent means compare configured providers.
    #[serde(default)]
    pub provider: Option<CloudProvider>,
    /// CPU cores (fractional allowed, e.g. 0.5, 1.5)
    pub cpu: f32,
    /// Memory in GB
    pub memory_gb: f32,
    /// Storage in GB
    pub storage_gb: f32,
    /// Optional GPU count
    pub gpu_count: Option<u32>,
    /// Canonical GPU class id from the catalog.
    #[serde(default)]
    pub gpu_class: Option<String>,
    /// Minimum VRAM per GPU, independent of host memory.
    #[serde(default)]
    pub gpu_vram_gb: Option<u32>,
    /// Require confidential compute.
    #[serde(default)]
    pub tee_required: bool,
    /// Per-allocation SSH public key installed by the provider.
    #[serde(default)]
    pub ssh_public_key: Option<String>,
    /// Workload image for providers that allocate containers.
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub volume_gb: Option<f32>,
    #[serde(default)]
    pub volume_mount_path: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    /// Caller workload metadata; no provider credentials belong here.
    #[serde(default)]
    pub manifest: Option<serde_json::Value>,
    /// Workload environment. Secret values are never serialized into lease state.
    #[serde(default, skip_serializing)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub ports: Vec<u16>,
    /// Optional argv override; absent preserves the image entrypoint.
    #[serde(default)]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    pub source_network: Option<String>,
    #[serde(default)]
    pub ownership: Option<ManagedLeaseIdentity>,
    #[serde(default)]
    pub endpoint_port: Option<u16>,
    /// Allocation lifetime supplied to providers with a native expiry.
    #[serde(default)]
    pub lease_duration_secs: Option<u64>,
    /// Maximum total node price in USD/hour, enforced during live selection.
    #[serde(default)]
    pub max_hourly_cost_usd: Option<f64>,
    /// Allow spot/preemptible instances
    pub allow_spot: bool,
    /// QoS parameters
    #[serde(default)]
    pub qos: QosParameters,
}

/// Caller-owned identity used for deterministic provider tags and recovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManagedLeaseIdentity {
    pub namespace: String,
    pub lease_id: String,
}

impl ManagedLeaseIdentity {
    pub fn validate(&self) -> Result<(), String> {
        if self.namespace.is_empty()
            || self.namespace.len() > 128
            || self.namespace.chars().any(char::is_control)
            || self.lease_id.is_empty()
            || self.lease_id.len() > 40
            || !self
                .lease_id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
        {
            return Err("Invalid managed lease identity".into());
        }
        Ok(())
    }
}

impl core::fmt::Debug for ResourceSpec {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ResourceSpec")
            .field("provider", &self.provider)
            .field("gpu_class", &self.gpu_class)
            .field("gpu_count", &self.gpu_count)
            .field("ownership", &self.ownership)
            .field("env", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl ResourceSpec {
    pub fn minimal() -> Self {
        Self {
            cpu: 0.5,
            memory_gb: 1.0,
            storage_gb: 10.0,
            allow_spot: true,
            ..Self::default()
        }
    }

    pub fn basic() -> Self {
        Self::default()
    }

    pub fn recommended() -> Self {
        Self {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            ..Self::default()
        }
    }

    pub fn performance() -> Self {
        Self {
            cpu: 8.0,
            memory_gb: 32.0,
            storage_gb: 500.0,
            ..Self::default()
        }
    }

    /// GPU-enabled resources
    pub fn with_gpu(mut self, count: u32) -> Self {
        self.gpu_count = Some(count);
        self
    }

    /// Validate the resource specification
    pub fn validate(&self) -> Result<(), String> {
        if !self.cpu.is_finite() || self.cpu < 0.1 {
            return Err("CPU must be at least 0.1 cores".into());
        }
        if !self.memory_gb.is_finite() || self.memory_gb < 0.5 {
            return Err("Memory must be at least 0.5 GB".into());
        }
        if !self.storage_gb.is_finite() || self.storage_gb < 1.0 {
            return Err("Storage must be at least 1 GB".into());
        }
        if self
            .max_hourly_cost_usd
            .is_some_and(|price| !price.is_finite() || price <= 0.0)
        {
            return Err("Maximum hourly cost must be finite and positive".into());
        }
        if let Some(count) = self.gpu_count {
            if count == 0 {
                return Err("GPU count must be positive".into());
            }
            if let Some(id) = &self.gpu_class {
                let gpu = crate::gpu::find(id).ok_or_else(|| format!("Unknown GPU class: {id}"))?;
                if count > gpu.max_per_node {
                    return Err(format!(
                        "GPU count exceeds {} max per node",
                        gpu.max_per_node
                    ));
                }
                if self.gpu_vram_gb.is_some_and(|vram| vram > gpu.vram_gb) {
                    return Err("GPU class cannot satisfy per-device VRAM".into());
                }
            } else if !crate::gpu::catalog().iter().any(|gpu| {
                count <= gpu.max_per_node && gpu.vram_gb >= self.gpu_vram_gb.unwrap_or(0)
            }) {
                return Err("No GPU class satisfies count and per-device VRAM".into());
            }
        } else if self.gpu_class.is_some() || self.gpu_vram_gb.is_some() {
            return Err("GPU class and VRAM require a GPU count".into());
        }
        if self
            .volume_gb
            .is_some_and(|gb| !gb.is_finite() || gb <= 0.0)
            || self
                .volume_mount_path
                .as_ref()
                .is_some_and(|path| !path.starts_with('/') || path.contains('\0'))
        {
            return Err("Invalid volume size or mount path".into());
        }
        if let Some(cidr) = &self.source_network {
            let (address, prefix) = cidr
                .split_once('/')
                .ok_or("Source network requires a CIDR prefix")?;
            let address: core::net::IpAddr = address
                .parse()
                .map_err(|_| "Invalid source network address")?;
            let prefix: u8 = prefix
                .parse()
                .map_err(|_| "Invalid source network prefix")?;
            let valid = match address {
                core::net::IpAddr::V4(_) => (1..=32).contains(&prefix),
                core::net::IpAddr::V6(ip) => {
                    (1..=128).contains(&prefix) && (ip.to_ipv4_mapped().is_none() || prefix > 96)
                }
            };
            if !valid {
                return Err("Source network must restrict access to a bounded CIDR".into());
            }
        }
        if self.ports.contains(&0) || self.endpoint_port == Some(0) {
            return Err("Exposed ports must be positive".into());
        }
        if self
            .endpoint_port
            .is_some_and(|port| !self.ports.contains(&port))
        {
            return Err("Application endpoint must name an exposed port".into());
        }
        if self.env.iter().any(|(key, value)| {
            key.is_empty()
                || !key.bytes().enumerate().all(|(i, c)| {
                    c == b'_' || c.is_ascii_alphabetic() || (i > 0 && c.is_ascii_digit())
                })
                || value.contains('\0')
        }) || self
            .command
            .as_ref()
            .is_some_and(|args| args.is_empty() || args.iter().any(|arg| arg.contains('\0')))
        {
            return Err("Invalid workload environment or command".into());
        }
        if let Some(identity) = &self.ownership {
            identity.validate()?;
        }
        Ok(())
    }

    /// Convert to Docker resource configuration
    pub fn to_docker_resources(&self) -> serde_json::Value {
        serde_json::json!({
            "NanoCPUs": (self.cpu * 1_000_000_000.0) as i64,
            "Memory": (self.memory_gb * 1024.0 * 1024.0 * 1024.0) as i64,
            "MemorySwap": -1, // Unlimited swap
            "CpuShares": 1024, // Default shares
            "StorageOpt": {
                "size": format!("{}G", self.storage_gb)
            }
        })
    }

    /// Convert to pricing units for pricing engine integration
    pub fn to_pricing_units(&self) -> BTreeMap<String, f64> {
        let mut units = BTreeMap::new();
        units.insert("CPU".to_string(), self.cpu as f64);
        units.insert("MemoryMB".to_string(), (self.memory_gb * 1024.0) as f64);
        units.insert("StorageMB".to_string(), (self.storage_gb * 1024.0) as f64);
        if let Some(gpu) = self.gpu_count {
            units.insert("GPU".to_string(), gpu as f64);
        }
        units
    }
}

/// QoS parameters for pricing calculations
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct QosParameters {
    pub priority: f32,
    pub sla_target: f32,
    pub reliability_multiplier: f32,
}

impl Default for ResourceSpec {
    fn default() -> Self {
        Self {
            provider: None,
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            gpu_class: None,
            gpu_vram_gb: None,
            tee_required: false,
            ssh_public_key: None,
            image: None,
            volume_gb: None,
            volume_mount_path: None,
            region: None,
            manifest: None,
            env: BTreeMap::new(),
            ports: Vec::new(),
            command: None,
            source_network: None,
            ownership: None,
            endpoint_port: None,
            lease_duration_secs: None,
            max_hourly_cost_usd: None,
            allow_spot: false,
            qos: QosParameters::default(),
        }
    }
}

/// Cloud provider types for cost tracking and pricing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CloudProvider {
    /// AWS (Amazon Web Services)
    #[serde(alias = "aws")]
    AWS,
    /// Google Cloud Platform
    #[serde(alias = "gcp", alias = "google")]
    GCP,
    /// Microsoft Azure
    #[serde(alias = "azure")]
    Azure,
    /// DigitalOcean
    #[serde(alias = "digitalocean", alias = "do")]
    DigitalOcean,
    /// Vultr
    #[serde(alias = "vultr")]
    Vultr,
    /// Linode
    #[serde(alias = "linode")]
    Linode,
    /// Lambda Labs — GPU cloud, on-demand A100/H100
    #[serde(alias = "lambda-labs", alias = "lambda", alias = "lambdalabs")]
    LambdaLabs,
    /// RunPod — pod-based GPU cloud
    #[serde(alias = "runpod")]
    RunPod,
    /// Vast.ai — bid-based spot GPU marketplace
    #[serde(alias = "vast-ai", alias = "vast", alias = "vastai")]
    VastAi,
    /// CoreWeave — K8s-native GPU cloud
    #[serde(alias = "coreweave")]
    CoreWeave,
    /// Paperspace — GPU cloud
    #[serde(alias = "paperspace")]
    Paperspace,
    /// Fluidstack — GPU-focused cloud
    #[serde(alias = "fluidstack")]
    Fluidstack,
    /// TensorDock — GPU marketplace
    #[serde(alias = "tensordock")]
    TensorDock,
    /// Akash Network — Cosmos-based decentralized compute
    #[serde(alias = "akash")]
    Akash,
    /// io.net — decentralized GPU cloud aggregator
    #[serde(alias = "io-net", alias = "ionet")]
    IoNet,
    /// Prime Intellect — compute aggregator
    #[serde(alias = "prime-intellect", alias = "primeintellect", alias = "pi")]
    PrimeIntellect,
    /// Render (Dispersed) — decentralized AI compute
    #[serde(alias = "render")]
    Render,
    /// Bittensor Lium (subnet 51) — decentralized GPU rental
    #[serde(alias = "bittensor-lium", alias = "bittensorlium", alias = "lium")]
    BittensorLium,
    /// Hetzner Cloud — European cloud with dedicated GPU servers
    #[serde(alias = "hetzner")]
    Hetzner,
    /// Crusoe Energy — clean-energy GPU cloud (L40S, A100, H100)
    #[serde(alias = "crusoe")]
    Crusoe,
    #[serde(alias = "modal")]
    Modal,
    /// Generic cloud provider
    #[serde(alias = "generic")]
    Generic,
    /// Local Docker
    #[serde(alias = "docker-local", alias = "dockerlocal")]
    DockerLocal,
    /// Remote Docker host
    DockerRemote(String),
    /// Bare metal with SSH
    BareMetal(alloc::vec::Vec<String>),
}

impl core::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CloudProvider::AWS => write!(f, "AWS"),
            CloudProvider::GCP => write!(f, "GCP"),
            CloudProvider::Azure => write!(f, "Azure"),
            CloudProvider::DigitalOcean => write!(f, "DigitalOcean"),
            CloudProvider::Vultr => write!(f, "Vultr"),
            CloudProvider::Linode => write!(f, "Linode"),
            CloudProvider::LambdaLabs => write!(f, "Lambda Labs"),
            CloudProvider::RunPod => write!(f, "RunPod"),
            CloudProvider::VastAi => write!(f, "Vast.ai"),
            CloudProvider::CoreWeave => write!(f, "CoreWeave"),
            CloudProvider::Paperspace => write!(f, "Paperspace"),
            CloudProvider::Fluidstack => write!(f, "Fluidstack"),
            CloudProvider::TensorDock => write!(f, "TensorDock"),
            CloudProvider::Akash => write!(f, "Akash"),
            CloudProvider::IoNet => write!(f, "io.net"),
            CloudProvider::PrimeIntellect => write!(f, "Prime Intellect"),
            CloudProvider::Render => write!(f, "Render"),
            CloudProvider::BittensorLium => write!(f, "Bittensor Lium"),
            CloudProvider::Hetzner => write!(f, "Hetzner"),
            CloudProvider::Crusoe => write!(f, "Crusoe"),
            CloudProvider::Modal => write!(f, "Modal"),
            CloudProvider::Generic => write!(f, "Generic"),
            CloudProvider::DockerLocal => write!(f, "Docker (Local)"),
            CloudProvider::DockerRemote(host) => write!(f, "Docker (Remote: {host})"),
            CloudProvider::BareMetal(hosts) => write!(f, "Bare Metal ({} hosts)", hosts.len()),
        }
    }
}

impl CloudProvider {
    pub fn supports_tee(&self) -> bool {
        matches!(self, Self::AWS | Self::GCP | Self::Azure)
    }

    /// Resolve operator configuration using the same provider keys as the catalog.
    pub fn from_catalog_key(key: &str) -> Option<Self> {
        key.parse().ok()
    }

    /// Provider key used by the shared GPU catalog.
    pub fn catalog_key(&self) -> &str {
        match self {
            Self::AWS => "aws",
            Self::GCP => "gcp",
            Self::Azure => "azure",
            Self::DigitalOcean => "digitalocean",
            Self::Vultr => "vultr",
            Self::Linode => "linode",
            Self::LambdaLabs => "lambda-labs",
            Self::RunPod => "runpod",
            Self::VastAi => "vast-ai",
            Self::CoreWeave => "coreweave",
            Self::Paperspace => "paperspace",
            Self::Fluidstack => "fluidstack",
            Self::TensorDock => "tensordock",
            Self::Akash => "akash",
            Self::IoNet => "io-net",
            Self::PrimeIntellect => "prime-intellect",
            Self::Render => "render",
            Self::BittensorLium => "bittensor-lium",
            Self::Hetzner => "hetzner",
            Self::Crusoe => "crusoe",
            Self::Modal => "modal",
            _ => "generic",
        }
    }
}

impl core::str::FromStr for CloudProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let key = value.trim().to_ascii_lowercase().replace('_', "-");
        serde_json::from_value(serde_json::Value::String(key))
            .map_err(|_| "Unknown cloud provider".into())
    }
}
