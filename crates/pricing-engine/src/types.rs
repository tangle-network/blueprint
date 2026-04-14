use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Resource units for various types of cloud resources
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceUnit {
    /// CPU cores or vCPUs
    CPU,
    /// Memory in megabytes
    MemoryMB,
    /// Storage in megabytes
    StorageMB,
    /// Network egress in megabytes
    NetworkEgressMB,
    /// Network ingress in megabytes
    NetworkIngressMB,
    /// GPU units
    GPU,
    /// Request count (for FaaS/API services)
    Request,
    /// Invocation count (for FaaS)
    Invocation,
    /// Execution time in milliseconds
    ExecutionTimeMS,
    /// Storage IO operations per second
    StorageIOPS,
    /// Custom unit with a name
    Custom(String),
}

impl core::fmt::Display for ResourceUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ResourceUnit::CPU => write!(f, "CPU"),
            ResourceUnit::MemoryMB => write!(f, "MemoryMB"),
            ResourceUnit::StorageMB => write!(f, "StorageMB"),
            ResourceUnit::NetworkEgressMB => write!(f, "NetworkEgressMB"),
            ResourceUnit::NetworkIngressMB => write!(f, "NetworkIngressMB"),
            ResourceUnit::GPU => write!(f, "GPU"),
            ResourceUnit::Request => write!(f, "Request"),
            ResourceUnit::Invocation => write!(f, "Invocation"),
            ResourceUnit::ExecutionTimeMS => write!(f, "ExecutionTimeMS"),
            ResourceUnit::StorageIOPS => write!(f, "StorageIOPS"),
            ResourceUnit::Custom(name) => write!(f, "{name}"),
        }
    }
}

/// Error type for parsing resource units
#[derive(Debug, thiserror::Error)]
#[error("Failed to parse resource unit")]
pub struct ParseResourceUnitError;

impl core::str::FromStr for ResourceUnit {
    type Err = ParseResourceUnitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CPU" => Ok(ResourceUnit::CPU),
            "MEMORYMB" => Ok(ResourceUnit::MemoryMB),
            "STORAGEMB" => Ok(ResourceUnit::StorageMB),
            "NETWORKEGRESSMB" => Ok(ResourceUnit::NetworkEgressMB),
            "NETWORKINGRESSMB" => Ok(ResourceUnit::NetworkIngressMB),
            "GPU" => Ok(ResourceUnit::GPU),
            "REQUEST" => Ok(ResourceUnit::Request),
            "INVOCATION" => Ok(ResourceUnit::Invocation),
            "EXECUTIONTIMEMS" => Ok(ResourceUnit::ExecutionTimeMS),
            "STORAGEIOPS" => Ok(ResourceUnit::StorageIOPS),
            _ => Err(ParseResourceUnitError),
        }
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
    BareMetal(Vec<String>),
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
