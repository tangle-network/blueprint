//! Resource specification for cloud provisioning

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
};
use serde::{Deserialize, Serialize};

/// Essential resource specification for deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ResourceSpec {
    /// Minimal resources
    pub fn minimal() -> Self {
        Self {
            provider: None,
            cpu: 0.5,
            memory_gb: 1.0,
            storage_gb: 10.0,
            gpu_count: None,
            gpu_class: None,
            gpu_vram_gb: None,
            tee_required: false,
            ssh_public_key: None,
            image: None,
            lease_duration_secs: None,
            max_hourly_cost_usd: None,
            allow_spot: true,
            qos: QosParameters::default(),
        }
    }

    /// Basic resources
    pub fn basic() -> Self {
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
            lease_duration_secs: None,
            max_hourly_cost_usd: None,
            allow_spot: false,
            qos: QosParameters::default(),
        }
    }

    /// Production resources
    pub fn recommended() -> Self {
        Self {
            provider: None,
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: None,
            gpu_class: None,
            gpu_vram_gb: None,
            tee_required: false,
            ssh_public_key: None,
            image: None,
            lease_duration_secs: None,
            max_hourly_cost_usd: None,
            allow_spot: false,
            qos: QosParameters::default(),
        }
    }

    /// Performance resources
    pub fn performance() -> Self {
        Self {
            provider: None,
            cpu: 8.0,
            memory_gb: 32.0,
            storage_gb: 500.0,
            gpu_count: None,
            gpu_class: None,
            gpu_vram_gb: None,
            tee_required: false,
            ssh_public_key: None,
            image: None,
            lease_duration_secs: None,
            max_hourly_cost_usd: None,
            allow_spot: false,
            qos: QosParameters::default(),
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
        Self::basic()
    }
}

/// Cloud provider types for cost tracking and pricing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CloudProvider {
    /// AWS (Amazon Web Services)
    AWS,
    /// Google Cloud Platform
    GCP,
    /// Microsoft Azure
    Azure,
    /// DigitalOcean
    DigitalOcean,
    /// Vultr
    Vultr,
    /// Linode
    Linode,
    /// Lambda Labs — GPU cloud, on-demand A100/H100
    LambdaLabs,
    /// RunPod — pod-based GPU cloud
    RunPod,
    /// Vast.ai — bid-based spot GPU marketplace
    VastAi,
    /// CoreWeave — K8s-native GPU cloud
    CoreWeave,
    /// Paperspace — GPU cloud
    Paperspace,
    /// Fluidstack — GPU-focused cloud
    Fluidstack,
    /// TensorDock — GPU marketplace
    TensorDock,
    /// Akash Network — Cosmos-based decentralized compute
    Akash,
    /// io.net — decentralized GPU cloud aggregator
    IoNet,
    /// Prime Intellect — compute aggregator
    PrimeIntellect,
    /// Render (Dispersed) — decentralized AI compute
    Render,
    /// Bittensor Lium (subnet 51) — decentralized GPU rental
    BittensorLium,
    /// Hetzner Cloud — European cloud with dedicated GPU servers
    Hetzner,
    /// Crusoe Energy — clean-energy GPU cloud (L40S, A100, H100)
    Crusoe,
    /// Generic cloud provider
    Generic,
    /// Local Docker
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
        let key = key.trim().to_ascii_lowercase().replace('_', "-");
        [
            Self::AWS,
            Self::GCP,
            Self::Azure,
            Self::DigitalOcean,
            Self::Vultr,
            Self::Linode,
            Self::LambdaLabs,
            Self::RunPod,
            Self::VastAi,
            Self::CoreWeave,
            Self::Paperspace,
            Self::Fluidstack,
            Self::TensorDock,
            Self::Akash,
            Self::IoNet,
            Self::PrimeIntellect,
            Self::Render,
            Self::BittensorLium,
            Self::Hetzner,
            Self::Crusoe,
            Self::Generic,
        ]
        .into_iter()
        .find(|provider| provider.catalog_key() == key)
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
            _ => "generic",
        }
    }
}
