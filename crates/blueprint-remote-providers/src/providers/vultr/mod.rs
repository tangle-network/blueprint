//! Vultr provider implementation

use crate::error::{Error, Result};
use crate::providers::common::{InstanceSelection, ProvisionedInfrastructure, ProvisioningConfig};
use crate::resources::ResourceSpec;

/// Vultr instance provisioner
pub struct VultrProvisioner;

impl VultrProvisioner {
    pub async fn new() -> Result<Self> {
        // TODO: Initialize Vultr client
        Ok(Self)
    }

    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        _config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        let _instance_selection = Self::map_instance(spec);

        // TODO: Implement Vultr API calls
        Err(Error::ConfigurationError(
            "Vultr provisioning not yet implemented".into(),
        ))
    }

    fn map_instance(spec: &ResourceSpec) -> InstanceSelection {
        let instance_type = match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 1.0 && mem <= 1.0 => "vc2-1c-1gb",
            (cpu, mem) if cpu <= 1.0 && mem <= 2.0 => "vc2-1c-2gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "vc2-2c-4gb",
            (cpu, mem) if cpu <= 4.0 && mem <= 8.0 => "vc2-4c-8gb",
            (cpu, mem) if cpu <= 8.0 && mem <= 16.0 => "vc2-8c-16gb",
            _ => "vc2-2c-4gb",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: false, // Vultr doesn't have spot instances
            estimated_hourly_cost: Self::estimate_cost(instance_type),
        }
    }

    fn estimate_cost(instance_type: &str) -> Option<f64> {
        Some(match instance_type {
            "vc2-1c-1gb" => 0.007,
            "vc2-1c-2gb" => 0.012,
            "vc2-2c-4gb" => 0.024,
            "vc2-4c-8gb" => 0.048,
            "vc2-8c-16gb" => 0.096,
            _ => 0.024,
        })
    }
}
