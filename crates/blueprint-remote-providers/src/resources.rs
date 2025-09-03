//! Unified resource model that bridges pricing-engine, manager, and remote-providers
//! 
//! This module provides the foundation for resource management across local and remote
//! deployments, ensuring consistent resource definitions and pricing calculations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified resource specification that works across all deployment targets
/// 
/// This replaces manager's basic ResourceLimits and provides a consistent model
/// for both local resource enforcement and remote instance selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedResourceSpec {
    /// Core compute resources
    pub compute: ComputeResources,
    
    /// Memory and storage
    pub storage: StorageResources,
    
    /// Network requirements
    pub network: NetworkResources,
    
    /// Optional accelerators (GPUs, TPUs, etc)
    pub accelerators: Option<AcceleratorResources>,
    
    /// Quality of service parameters
    pub qos: QosParameters,
}

impl Default for UnifiedResourceSpec {
    fn default() -> Self {
        Self {
            compute: ComputeResources::default(),
            storage: StorageResources::default(),
            network: NetworkResources::default(),
            accelerators: None,
            qos: QosParameters::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    /// CPU cores (can be fractional, e.g., 0.5 for half a core)
    pub cpu_cores: f64,
    
    /// CPU architecture preference (x86_64, arm64, etc)
    pub cpu_arch: Option<String>,
    
    /// Minimum CPU frequency in GHz (optional)
    pub min_cpu_frequency_ghz: Option<f64>,
}

impl Default for ComputeResources {
    fn default() -> Self {
        Self {
            cpu_cores: 1.0,
            cpu_arch: None,
            min_cpu_frequency_ghz: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResources {
    /// RAM in GB
    pub memory_gb: f64,
    
    /// Persistent storage in GB
    pub disk_gb: f64,
    
    /// Storage type (ssd, nvme, hdd)
    pub disk_type: StorageType,
    
    /// IOPS requirement (optional)
    pub iops: Option<u32>,
}

impl Default for StorageResources {
    fn default() -> Self {
        Self {
            memory_gb: 2.0,
            disk_gb: 10.0,
            disk_type: StorageType::SSD,
            iops: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    HDD,
    SSD, 
    NVME,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResources {
    /// Bandwidth tier
    pub bandwidth_tier: BandwidthTier,
    
    /// Guaranteed bandwidth in Mbps (optional)
    pub guaranteed_bandwidth_mbps: Option<u32>,
    
    /// Static IP requirement
    pub static_ip: bool,
    
    /// Public IP requirement
    pub public_ip: bool,
}

impl Default for NetworkResources {
    fn default() -> Self {
        Self {
            bandwidth_tier: BandwidthTier::Standard,
            guaranteed_bandwidth_mbps: None,
            static_ip: false,
            public_ip: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BandwidthTier {
    Low,      // Up to 1 Gbps
    Standard, // Up to 10 Gbps
    High,     // Up to 25 Gbps
    Ultra,    // 50+ Gbps
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceleratorResources {
    /// Number of accelerators
    pub count: u32,
    
    /// Type of accelerator
    pub accelerator_type: AcceleratorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcceleratorType {
    GPU(GpuSpec),
    TPU(String),
    FPGA(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSpec {
    /// GPU vendor (nvidia, amd, intel)
    pub vendor: String,
    
    /// GPU model (a100, v100, t4, etc)
    pub model: String,
    
    /// Minimum VRAM in GB
    pub min_vram_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QosParameters {
    /// Priority level (0-100, higher is more important)
    pub priority: u8,
    
    /// Whether spot/preemptible instances are acceptable
    pub allow_spot: bool,
    
    /// Whether burstable instances are acceptable
    pub allow_burstable: bool,
    
    /// Minimum availability SLA (99.9, 99.99, etc)
    pub min_availability_sla: Option<f64>,
}

impl Default for QosParameters {
    fn default() -> Self {
        Self {
            priority: 50,
            allow_spot: false,
            allow_burstable: true,
            min_availability_sla: None,
        }
    }
}

/// Converts unified spec to pricing engine resource units for cost calculation
pub fn to_pricing_units(spec: &UnifiedResourceSpec) -> HashMap<String, f64> {
    let mut units = HashMap::new();
    
    // Map to pricing engine ResourceUnit equivalents
    units.insert("CPU".to_string(), spec.compute.cpu_cores);
    units.insert("MemoryMB".to_string(), spec.storage.memory_gb * 1024.0);
    units.insert("StorageMB".to_string(), spec.storage.disk_gb * 1024.0);
    
    // Network units based on tier
    let network_multiplier = match spec.network.bandwidth_tier {
        BandwidthTier::Low => 1.0,
        BandwidthTier::Standard => 2.0,
        BandwidthTier::High => 4.0,
        BandwidthTier::Ultra => 8.0,
    };
    units.insert("NetworkEgressMB".to_string(), 1024.0 * network_multiplier);
    units.insert("NetworkIngressMB".to_string(), 1024.0 * network_multiplier);
    
    // GPU units if present
    if let Some(ref accel) = spec.accelerators {
        if let AcceleratorType::GPU(_) = accel.accelerator_type {
            units.insert("GPU".to_string(), accel.count as f64);
        }
    }
    
    units
}

/// Converts unified spec to Kubernetes resource limits
#[cfg(feature = "kubernetes")]
pub fn to_k8s_resources(spec: &UnifiedResourceSpec) -> (k8s_openapi::api::core::v1::ResourceRequirements, Option<k8s_openapi::api::core::v1::PersistentVolumeClaimSpec>) {
    use k8s_openapi::api::core::v1::{ResourceRequirements, PersistentVolumeClaimSpec};
    use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
    
    let mut limits = std::collections::BTreeMap::new();
    let mut requests = std::collections::BTreeMap::new();
    
    // CPU (in cores or millicores)
    let cpu_str = if spec.compute.cpu_cores < 1.0 {
        format!("{}m", (spec.compute.cpu_cores * 1000.0) as i32)
    } else {
        format!("{}", spec.compute.cpu_cores)
    };
    limits.insert("cpu".to_string(), Quantity(cpu_str.clone()));
    requests.insert("cpu".to_string(), Quantity(cpu_str));
    
    // Memory
    let memory_str = format!("{}Gi", spec.storage.memory_gb);
    limits.insert("memory".to_string(), Quantity(memory_str.clone()));
    requests.insert("memory".to_string(), Quantity(memory_str));
    
    // GPU if present
    if let Some(ref accel) = spec.accelerators {
        if let AcceleratorType::GPU(ref gpu_spec) = accel.accelerator_type {
            let gpu_key = match gpu_spec.vendor.as_str() {
                "nvidia" => "nvidia.com/gpu",
                "amd" => "amd.com/gpu",
                _ => "gpu",
            };
            limits.insert(gpu_key.to_string(), Quantity(accel.count.to_string()));
            requests.insert(gpu_key.to_string(), Quantity(accel.count.to_string()));
        }
    }
    
    let resource_req = ResourceRequirements {
        limits: Some(limits),
        requests: Some(requests),
        claims: None,
    };
    
    // Storage as PVC if needed
    let pvc_spec = if spec.storage.disk_gb > 0.0 {
        let mut pvc_requests = std::collections::BTreeMap::new();
        pvc_requests.insert("storage".to_string(), Quantity(format!("{}Gi", spec.storage.disk_gb)));
        
        let storage_class = match spec.storage.disk_type {
            StorageType::HDD => Some("standard".to_string()),
            StorageType::SSD => Some("ssd".to_string()),
            StorageType::NVME => Some("nvme".to_string()),
        };
        
        Some(PersistentVolumeClaimSpec {
            access_modes: Some(vec!["ReadWriteOnce".to_string()]),
            resources: Some(ResourceRequirements {
                requests: Some(pvc_requests),
                limits: None,
                claims: None,
            }),
            storage_class_name: storage_class,
            ..Default::default()
        })
    } else {
        None
    };
    
    (resource_req, pvc_spec)
}

/// Converts unified spec to Docker resource limits
pub fn to_docker_resources(spec: &UnifiedResourceSpec) -> serde_json::Value {
    let mut host_config = serde_json::json!({});
    
    // CPU limits (Docker uses nano CPUs)
    let nano_cpus = (spec.compute.cpu_cores * 1_000_000_000.0) as i64;
    host_config["NanoCPUs"] = nano_cpus.into();
    
    // Memory limits (in bytes)
    let memory_bytes = (spec.storage.memory_gb * 1024.0 * 1024.0 * 1024.0) as i64;
    host_config["Memory"] = memory_bytes.into();
    
    // Storage limits if available
    if spec.storage.disk_gb > 0.0 {
        host_config["StorageOpt"] = serde_json::json!({
            "size": format!("{}G", spec.storage.disk_gb)
        });
    }
    
    // GPU support via device requests
    if let Some(ref accel) = spec.accelerators {
        if let AcceleratorType::GPU(_) = accel.accelerator_type {
            host_config["DeviceRequests"] = serde_json::json!([
                {
                    "Driver": "nvidia",
                    "Count": accel.count,
                    "Capabilities": [["gpu"]]
                }
            ]);
        }
    }
    
    host_config
}

/// Converts from legacy manager ResourceLimits
pub fn from_legacy_limits(memory_mb: Option<u64>, storage_mb: Option<u64>) -> UnifiedResourceSpec {
    UnifiedResourceSpec {
        compute: ComputeResources::default(),
        storage: StorageResources {
            memory_gb: memory_mb.map(|mb| mb as f64 / 1024.0).unwrap_or(2.0),
            disk_gb: storage_mb.map(|mb| mb as f64 / 1024.0).unwrap_or(10.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Converts from remote-providers ResourceRequirements
pub fn from_resource_requirements(req: &crate::provisioning::ResourceRequirements) -> UnifiedResourceSpec {
    use crate::provisioning::NetworkTier;
    
    UnifiedResourceSpec {
        compute: ComputeResources {
            cpu_cores: req.cpu_cores,
            ..Default::default()
        },
        storage: StorageResources {
            memory_gb: req.memory_gb,
            disk_gb: req.storage_gb,
            ..Default::default()
        },
        network: NetworkResources {
            bandwidth_tier: match req.network_tier {
                NetworkTier::Low => BandwidthTier::Low,
                NetworkTier::Standard => BandwidthTier::Standard,
                NetworkTier::High => BandwidthTier::High,
                NetworkTier::Ultra => BandwidthTier::Ultra,
            },
            ..Default::default()
        },
        accelerators: req.gpu_count.map(|count| AcceleratorResources {
            count,
            accelerator_type: AcceleratorType::GPU(GpuSpec {
                vendor: "nvidia".to_string(),
                model: req.gpu_type.clone().unwrap_or_else(|| "t4".to_string()),
                min_vram_gb: 16.0,
            }),
        }),
        qos: QosParameters {
            allow_spot: req.allow_spot,
            ..Default::default()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unified_spec_to_pricing_units() {
        let spec = UnifiedResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let units = to_pricing_units(&spec);
        
        assert_eq!(units.get("CPU"), Some(&4.0));
        assert_eq!(units.get("MemoryMB"), Some(&(16.0 * 1024.0)));
        assert_eq!(units.get("StorageMB"), Some(&(100.0 * 1024.0)));
    }
    
    #[test]
    #[cfg(feature = "kubernetes")]
    fn test_k8s_resource_conversion() {
        let spec = UnifiedResourceSpec {
            compute: ComputeResources {
                cpu_cores: 0.5,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 2.0,
                disk_gb: 10.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let (resources, pvc) = to_k8s_resources(&spec);
        
        assert!(resources.limits.is_some());
        let limits = resources.limits.unwrap();
        assert!(limits.contains_key("cpu"));
        assert!(limits.contains_key("memory"));
        
        assert!(pvc.is_some());
    }
    
    #[test]
    fn test_docker_resource_conversion() {
        let spec = UnifiedResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 4.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let docker_config = to_docker_resources(&spec);
        
        assert_eq!(docker_config["NanoCPUs"], 2_000_000_000i64);
        assert_eq!(docker_config["Memory"], 4 * 1024 * 1024 * 1024i64);
    }
}