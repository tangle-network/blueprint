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
            CloudProvider::Generic => write!(f, "Generic"),
            CloudProvider::DockerLocal => write!(f, "Docker (Local)"),
            CloudProvider::DockerRemote(host) => write!(f, "Docker (Remote: {host})"),
            CloudProvider::BareMetal(hosts) => write!(f, "Bare Metal ({} hosts)", hosts.len()),
        }
    }
}
