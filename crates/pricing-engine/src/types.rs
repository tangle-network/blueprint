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
            _ => Ok(ResourceUnit::Custom(s.to_string())),
        }
    }
}

/// Represents a price with a value and currency/token
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    /// Numerical value of the price (in the smallest unit of the token, e.g., microtoken)
    pub value: u128,
    /// Token or currency used for pricing (e.g., "TGL")
    pub token: String,
}

impl Price {
    /// Create a new price
    pub fn new(value: u128, token: impl Into<String>) -> Self {
        Self {
            value,
            token: token.into(),
        }
    }

    /// Add another price to this one, assuming same token
    pub fn add(&self, other: &Price) -> Result<Price, &'static str> {
        if self.token != other.token {
            return Err("Cannot add prices with different tokens");
        }

        Ok(Price {
            value: self.value.saturating_add(other.value),
            token: self.token.clone(),
        })
    }

    /// Scale this price by a factor
    pub fn scale(&self, factor: u128) -> Price {
        Price {
            value: self.value.saturating_mul(factor),
            token: self.token.clone(),
        }
    }
}

impl core::fmt::Display for Price {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Format to show as a decimal number with 6 decimal places (assuming microtoken)
        let whole = self.value / 1_000_000;
        let fractional = self.value % 1_000_000;
        write!(f, "{}.{:06} {}", whole, fractional, self.token)
    }
}

/// Resource requirement for a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirement {
    /// Type of resource
    pub unit: ResourceUnit,
    /// Quantity of the resource (in the smallest measurable unit)
    pub quantity: u128,
}

impl ResourceRequirement {
    /// Create a new resource requirement
    pub fn new(unit: ResourceUnit, quantity: u128) -> Self {
        Self { unit, quantity }
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
