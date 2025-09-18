//! Resource specification for cloud provisioning

use serde::{Deserialize, Serialize};

/// Essential resource specification for deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// CPU cores (fractional allowed, e.g. 0.5, 1.5)
    pub cpu: f32,
    /// Memory in GB
    pub memory_gb: f32,
    /// Storage in GB
    pub storage_gb: f32,
    /// Optional GPU count
    pub gpu_count: Option<u32>,
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
            cpu: 0.5,
            memory_gb: 1.0,
            storage_gb: 10.0,
            gpu_count: None,
            allow_spot: true,
            qos: QosParameters::default(),
        }
    }

    /// Basic resources
    pub fn basic() -> Self {
        Self {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 20.0,
            gpu_count: None,
            allow_spot: false,
            qos: QosParameters::default(),
        }
    }

    /// Production resources
    pub fn recommended() -> Self {
        Self {
            cpu: 4.0,
            memory_gb: 16.0,
            storage_gb: 100.0,
            gpu_count: None,
            allow_spot: false,
            qos: QosParameters::default(),
        }
    }

    /// Performance resources
    pub fn performance() -> Self {
        Self {
            cpu: 8.0,
            memory_gb: 32.0,
            storage_gb: 500.0,
            gpu_count: None,
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
        if self.cpu < 0.1 {
            return Err("CPU must be at least 0.1 cores".into());
        }
        if self.memory_gb < 0.5 {
            return Err("Memory must be at least 0.5 GB".into());
        }
        if self.storage_gb < 1.0 {
            return Err("Storage must be at least 1 GB".into());
        }
        if let Some(gpu) = self.gpu_count {
            if gpu == 0 || gpu > 8 {
                return Err("GPU count must be between 1 and 8".into());
            }
        }
        Ok(())
    }

    /// Convert to Kubernetes resource requirements
    #[cfg(feature = "kubernetes")]
    pub fn to_k8s_resources(&self) -> k8s_openapi::api::core::v1::ResourceRequirements {
        use blueprint_std::collections::BTreeMap;
        use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

        let mut limits = BTreeMap::new();
        let mut requests = BTreeMap::new();

        // CPU in millicores or cores
        limits.insert("cpu".to_string(), Quantity(format!("{}", self.cpu)));
        requests.insert("cpu".to_string(), Quantity(format!("{}", self.cpu * 0.8)));

        // Memory in Gi
        limits.insert(
            "memory".to_string(),
            Quantity(format!("{}Gi", self.memory_gb)),
        );
        requests.insert(
            "memory".to_string(),
            Quantity(format!("{}Gi", self.memory_gb * 0.9)),
        );

        // GPU if requested
        if let Some(gpu_count) = self.gpu_count {
            limits.insert(
                "nvidia.com/gpu".to_string(),
                Quantity(gpu_count.to_string()),
            );
        }

        k8s_openapi::api::core::v1::ResourceRequirements {
            limits: Some(limits),
            requests: Some(requests),
            claims: None,
        }
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

    /// Estimate hourly cost in USD
    pub fn estimate_hourly_cost(&self) -> f64 {
        let base_cost = self.cpu * 0.04 + self.memory_gb * 0.01;
        let storage_cost = self.storage_gb * 0.0001;
        let gpu_cost = self.gpu_count.unwrap_or(0) as f32 * 0.90;

        let total = base_cost + storage_cost + gpu_cost;

        let final_cost = if self.allow_spot {
            total * 0.7 // 30% discount for spot instances
        } else {
            total
        };
        final_cost as f64
    }

    /// Convert to pricing units for pricing engine integration
    pub fn to_pricing_units(&self) -> blueprint_std::collections::HashMap<String, f64> {
        let mut units = blueprint_std::collections::HashMap::new();
        units.insert("CPU".to_string(), self.cpu as f64);
        units.insert("MemoryMB".to_string(), (self.memory_gb * 1024.0) as f64);
        units.insert("StorageMB".to_string(), (self.storage_gb * 1024.0) as f64);
        if let Some(gpu) = self.gpu_count {
            units.insert("GPU".to_string(), gpu as f64);
        }
        units
    }
}

/// Convert resource spec to pricing units
pub fn to_pricing_units(spec: &ResourceSpec) -> blueprint_std::collections::HashMap<String, f64> {
    spec.to_pricing_units()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_validation() {
        let valid = ResourceSpec::basic();
        assert!(valid.validate().is_ok());

        let invalid_cpu = ResourceSpec {
            cpu: 0.05,
            ..Default::default()
        };
        assert!(invalid_cpu.validate().is_err());

        let invalid_memory = ResourceSpec {
            memory_gb: 0.25,
            ..Default::default()
        };
        assert!(invalid_memory.validate().is_err());
    }

    #[test]
    fn test_cost_estimation() {
        let basic = ResourceSpec::basic();
        let cost = basic.estimate_hourly_cost();
        assert!(cost > 0.0);
        assert!(cost < 1.0); // Basic should be under $1/hour

        let with_gpu = ResourceSpec::basic().with_gpu(1);
        let gpu_cost = with_gpu.estimate_hourly_cost();
        assert!(gpu_cost > cost); // GPU should increase cost

        let spot = ResourceSpec {
            allow_spot: true,
            ..basic
        };
        let spot_cost = spot.estimate_hourly_cost();
        assert!(spot_cost < cost); // Spot should be cheaper
    }

    #[cfg(feature = "kubernetes")]
    #[test]
    fn test_k8s_conversion() {
        let spec = ResourceSpec::recommended();
        let k8s = spec.to_k8s_resources();

        assert!(k8s.limits.is_some());
        assert!(k8s.requests.is_some());

        let limits = k8s.limits.unwrap();
        assert!(limits.contains_key("cpu"));
        assert!(limits.contains_key("memory"));

        let requests = k8s.requests.unwrap();
        assert!(requests.contains_key("cpu"));
        assert!(requests.contains_key("memory"));
    }

    #[test]
    fn test_docker_conversion() {
        let spec = ResourceSpec::basic();
        let docker = spec.to_docker_resources();

        assert_eq!(docker["NanoCPUs"], 2_000_000_000i64);
        assert_eq!(docker["Memory"], 4 * 1024 * 1024 * 1024i64);
        assert_eq!(docker["StorageOpt"]["size"], "20G");
    }
}
