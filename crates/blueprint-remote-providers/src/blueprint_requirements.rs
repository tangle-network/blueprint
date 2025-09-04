//! Blueprint resource requirements specification
//!
//! Allows blueprint developers to specify minimum and recommended resources
//! for their services, enabling proper pricing and deployment decisions.

use crate::resources::{ComputeResources, ResourceSpec, StorageResources};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Resource requirements specified by blueprint developers
///
/// Developers can optionally define these in their Cargo.toml [package.metadata.blueprint] to indicate
/// what resources their service needs to function properly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintResourceRequirements {
    /// Minimum resources required for the service to function
    pub minimum: ResourceSpec,

    /// Recommended resources for optimal performance
    pub recommended: ResourceSpec,

    /// Optional description of resource usage
    pub description: Option<String>,

    /// Scaling characteristics
    pub scaling: ScalingInfo,
}

impl Default for BlueprintResourceRequirements {
    fn default() -> Self {
        Self {
            minimum: ResourceSpec {
                compute: ComputeResources {
                    cpu_cores: 0.5,
                    ..Default::default()
                },
                storage: StorageResources {
                    memory_gb: 1.0,
                    disk_gb: 5.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            recommended: ResourceSpec {
                compute: ComputeResources {
                    cpu_cores: 2.0,
                    ..Default::default()
                },
                storage: StorageResources {
                    memory_gb: 4.0,
                    disk_gb: 20.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            description: None,
            scaling: ScalingInfo::default(),
        }
    }
}

/// Information about how the service scales with resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingInfo {
    /// Whether the service can utilize multiple CPU cores effectively
    pub cpu_scalable: bool,

    /// Whether the service benefits from additional memory
    pub memory_scalable: bool,

    /// Whether the service can scale horizontally (multiple instances)
    pub horizontal_scalable: bool,

    /// Maximum useful CPU cores (None = unlimited)
    pub max_useful_cpu: Option<f64>,

    /// Maximum useful memory in GB (None = unlimited)
    pub max_useful_memory_gb: Option<f64>,
}

impl Default for ScalingInfo {
    fn default() -> Self {
        Self {
            cpu_scalable: true,
            memory_scalable: true,
            horizontal_scalable: false,
            max_useful_cpu: None,
            max_useful_memory_gb: None,
        }
    }
}

impl BlueprintResourceRequirements {
    /// Load requirements from Cargo.toml [package.metadata.blueprint] section
    pub fn from_toml_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse requirements: {}", e))
    }

    /// Validate that customer-requested resources meet minimum requirements
    pub fn validate_request(&self, requested: &ResourceSpec) -> ValidationResult {
        let mut issues = Vec::new();

        // Check CPU
        if requested.compute.cpu_cores < self.minimum.compute.cpu_cores {
            issues.push(format!(
                "CPU cores {} below minimum {}",
                requested.compute.cpu_cores, self.minimum.compute.cpu_cores
            ));
        }

        // Check memory
        if requested.storage.memory_gb < self.minimum.storage.memory_gb {
            issues.push(format!(
                "Memory {}GB below minimum {}GB",
                requested.storage.memory_gb, self.minimum.storage.memory_gb
            ));
        }

        // Check storage
        if requested.storage.disk_gb < self.minimum.storage.disk_gb {
            issues.push(format!(
                "Storage {}GB below minimum {}GB",
                requested.storage.disk_gb, self.minimum.storage.disk_gb
            ));
        }

        if issues.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::BelowMinimum { issues }
        }
    }

    /// Check if requested resources exceed recommended (might waste money)
    pub fn check_overprovisioning(&self, requested: &ResourceSpec) -> Option<String> {
        let mut warnings = Vec::new();

        // Check if CPU exceeds max useful
        if let Some(max_cpu) = self.scaling.max_useful_cpu {
            if requested.compute.cpu_cores > max_cpu {
                warnings.push(format!(
                    "CPU cores {} exceed maximum useful {}",
                    requested.compute.cpu_cores, max_cpu
                ));
            }
        }

        // Check if memory exceeds max useful
        if let Some(max_mem) = self.scaling.max_useful_memory_gb {
            if requested.storage.memory_gb > max_mem {
                warnings.push(format!(
                    "Memory {}GB exceeds maximum useful {}GB",
                    requested.storage.memory_gb, max_mem
                ));
            }
        }

        // Warn if significantly over recommended without being scalable
        if !self.scaling.cpu_scalable
            && requested.compute.cpu_cores > self.recommended.compute.cpu_cores * 2.0
        {
            warnings.push(format!(
                "Service doesn't scale well with CPU; {} cores may be wasteful",
                requested.compute.cpu_cores
            ));
        }

        if warnings.is_empty() {
            None
        } else {
            Some(warnings.join("; "))
        }
    }

    /// Get a resource spec that's midway between minimum and recommended
    pub fn balanced(&self) -> ResourceSpec {
        ResourceSpec {
            compute: ComputeResources {
                cpu_cores: (self.minimum.compute.cpu_cores + self.recommended.compute.cpu_cores)
                    / 2.0,
                ..self.minimum.compute.clone()
            },
            storage: StorageResources {
                memory_gb: (self.minimum.storage.memory_gb + self.recommended.storage.memory_gb)
                    / 2.0,
                disk_gb: (self.minimum.storage.disk_gb + self.recommended.storage.disk_gb) / 2.0,
                ..self.minimum.storage.clone()
            },
            ..self.minimum.clone()
        }
    }
}

/// Result of validating requested resources against requirements
#[derive(Debug, Clone)]
pub enum ValidationResult {
    /// Resources meet requirements
    Valid,
    /// Resources below minimum
    BelowMinimum { issues: Vec<String> },
}

/// Example Cargo.toml [package.metadata.blueprint] configuration
///
/// ```toml
/// [metadata]
/// name = "my-service"
/// version = "1.0.0"
///
/// [requirements.minimum]
/// [requirements.minimum.compute]
/// cpu_cores = 1.0
///
/// [requirements.minimum.storage]
/// memory_gb = 2.0
/// disk_gb = 10.0
///
/// [requirements.recommended]
/// [requirements.recommended.compute]
/// cpu_cores = 4.0
///
/// [requirements.recommended.storage]
/// memory_gb = 8.0
/// disk_gb = 50.0
///
/// [requirements.scaling]
/// cpu_scalable = true
/// memory_scalable = true
/// horizontal_scalable = false
/// max_useful_cpu = 16.0
/// max_useful_memory_gb = 64.0
///
/// [requirements]
/// description = "CPU scales linearly up to 16 cores. Memory improves cache performance."
/// ```
pub fn example_toml() -> &'static str {
    r#"[metadata]
name = "my-service"
version = "1.0.0"

[requirements.minimum.compute]
cpu_cores = 1.0

[requirements.minimum.storage]
memory_gb = 2.0
disk_gb = 10.0

[requirements.recommended.compute]
cpu_cores = 4.0

[requirements.recommended.storage]
memory_gb = 8.0
disk_gb = 50.0

[requirements.scaling]
cpu_scalable = true
memory_scalable = true
horizontal_scalable = false
max_useful_cpu = 16.0
max_useful_memory_gb = 64.0

[requirements]
description = "CPU scales linearly up to 16 cores. Memory improves cache performance."
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requirement_validation() {
        let requirements = BlueprintResourceRequirements::default();

        // Test below minimum
        let insufficient = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 0.25,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 0.5,
                disk_gb: 2.0,
                ..Default::default()
            },
            ..Default::default()
        };

        match requirements.validate_request(&insufficient) {
            ValidationResult::BelowMinimum { issues } => {
                assert!(issues.len() > 0);
            }
            _ => panic!("Should have failed validation"),
        }

        // Test meeting minimum
        let sufficient = requirements.minimum.clone();
        match requirements.validate_request(&sufficient) {
            ValidationResult::Valid => {}
            _ => panic!("Should have passed validation"),
        }
    }

    #[test]
    fn test_overprovisioning_warning() {
        let mut requirements = BlueprintResourceRequirements::default();
        requirements.scaling.cpu_scalable = false;
        requirements.scaling.max_useful_cpu = Some(4.0);

        let overprovisioned = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 8.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let warning = requirements.check_overprovisioning(&overprovisioned);
        assert!(warning.is_some());
    }

    #[test]
    fn test_balanced_spec() {
        let requirements = BlueprintResourceRequirements::default();
        let balanced = requirements.balanced();

        assert!(balanced.compute.cpu_cores > requirements.minimum.compute.cpu_cores);
        assert!(balanced.compute.cpu_cores < requirements.recommended.compute.cpu_cores);
    }
}
