//! Google Cloud Platform provider implementation

pub mod adapter;

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::providers::common::{InstanceSelection, ProvisionedInfrastructure, ProvisioningConfig};
use std::collections::HashMap;
use tracing::{info, warn};

/// GCP Compute Engine provisioner
pub struct GcpProvisioner {
    #[cfg(feature = "gcp")]
    project_id: String,
    client: reqwest::Client,
    #[cfg(feature = "gcp")]
    access_token: Option<String>,
}

impl GcpProvisioner {
    /// Create new GCP provisioner
    #[cfg(feature = "gcp")]
    pub async fn new(project_id: String) -> Result<Self> {
        // In production, would use google-cloud-auth crate
        // For now, use environment variable or metadata service
        let access_token = Self::get_access_token().await?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::ConfigurationError(e.to_string()))?;

        Ok(Self {
            project_id,
            client,
            access_token: Some(access_token),
        })
    }

    #[cfg(not(feature = "gcp"))]
    pub async fn new(_project_id: String) -> Result<Self> {
        Err(Error::ConfigurationError(
            "GCP support not enabled. Enable the 'gcp' feature".into(),
        ))
    }

    /// Get access token from environment or metadata service
    #[cfg(feature = "gcp")]
    async fn get_access_token() -> Result<String> {
        // Try environment variable first
        if let Ok(token) = std::env::var("GCP_ACCESS_TOKEN") {
            return Ok(token);
        }

        // Try metadata service (for GCE instances)
        {
            let metadata_url = "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
            let client = reqwest::Client::new();
            let response = client
                .get(metadata_url)
                .header("Metadata-Flavor", "Google")
                .send()
                .await;

            if let Ok(resp) = response {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(token) = json["access_token"].as_str() {
                        return Ok(token.to_string());
                    }
                }
            }
        }

        Err(Error::ConfigurationError(
            "No GCP credentials found. Set GCP_ACCESS_TOKEN or use service account".into(),
        ))
    }

    /// Provision a GCE instance
    #[cfg(feature = "gcp")]
    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        let instance_selection = Self::map_instance(spec);
        let zone = format!("{}-a", config.region); // e.g., us-central1-a

        info!(
            "Provisioning GCP instance type {} in {}",
            instance_selection.instance_type, zone
        );

        // Prepare instance configuration
        let instance_config = serde_json::json!({
            "name": config.name,
            "machineType": format!("zones/{}/machineTypes/{}", zone, instance_selection.instance_type),
            "disks": [{
                "boot": true,
                "autoDelete": true,
                "initializeParams": {
                    "sourceImage": config.machine_image.as_deref()
                        .unwrap_or("projects/ubuntu-os-cloud/global/images/family/ubuntu-2204-lts"),
                    "diskSizeGb": spec.storage_gb.to_string(),
                }
            }],
            "networkInterfaces": [{
                "network": "global/networks/default",
                "accessConfigs": [{
                    "type": "ONE_TO_ONE_NAT",
                    "name": "External NAT"
                }]
            }],
            "metadata": {
                "items": [
                    {
                        "key": "ssh-keys",
                        "value": config.custom_config.get("ssh_public_key")
                            .unwrap_or(&String::from(""))
                    },
                    {
                        "key": "startup-script",
                        "value": Self::generate_startup_script()
                    }
                ]
            },
            "tags": {
                "items": ["blueprint", "managed"]
            },
            "labels": {
                "environment": "production",
                "managed_by": "blueprint_remote_providers"
            }
        });

        // Create the instance
        let url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances",
            self.project_id, zone
        );

        let response = self
            .client
            .post(&url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .json(&instance_config)
            .send()
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to create GCE instance: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "GCP API error: {}",
                error_text
            )));
        }

        let operation: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;

        info!(
            "GCP operation started: {}",
            operation["name"].as_str().unwrap_or("unknown")
        );

        // Wait for operation to complete
        self.wait_for_operation(&operation["selfLink"].as_str().unwrap_or(""))
            .await?;

        // Get instance details
        let instance_url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
            self.project_id, zone, config.name
        );

        let instance_response = self
            .client
            .get(&instance_url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get instance: {}", e)))?;

        let instance: serde_json::Value = instance_response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse instance: {}", e)))?;

        // Extract IPs
        let network_interface = &instance["networkInterfaces"][0];
        let private_ip = network_interface["networkIP"]
            .as_str()
            .map(|s| s.to_string());
        let public_ip = network_interface["accessConfigs"][0]["natIP"]
            .as_str()
            .map(|s| s.to_string());

        Ok(ProvisionedInfrastructure {
            provider: CloudProvider::GCP,
            instance_id: instance["id"].to_string(),
            public_ip,
            private_ip,
            region: config.region.clone(),
            instance_type: instance_selection.instance_type,
            metadata: HashMap::new(),
        })
    }

    #[cfg(not(feature = "gcp"))]
    pub async fn provision_instance(
        &self,
        _spec: &ResourceSpec,
        _config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        Err(Error::ConfigurationError(
            "GCP provisioning requires 'gcp' feature".into(),
        ))
    }

    /// Wait for GCP operation to complete
    #[cfg(feature = "gcp")]
    async fn wait_for_operation(&self, operation_url: &str) -> Result<()> {
        let max_attempts = 60;
        let mut attempts = 0;

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let response = self
                .client
                .get(operation_url)
                .bearer_auth(self.access_token.as_ref().unwrap())
                .send()
                .await
                .map_err(|e| {
                    Error::ConfigurationError(format!("Failed to check operation: {}", e))
                })?;

            let operation: serde_json::Value = response.json().await.map_err(|e| {
                Error::ConfigurationError(format!("Failed to parse operation: {}", e))
            })?;

            if operation["status"].as_str() == Some("DONE") {
                if let Some(error) = operation.get("error") {
                    return Err(Error::ConfigurationError(format!(
                        "Operation failed: {:?}",
                        error
                    )));
                }
                return Ok(());
            }

            attempts += 1;
            if attempts >= max_attempts {
                return Err(Error::ConfigurationError("Operation timeout".into()));
            }
        }
    }

    /// Generate startup script for GCE instances
    fn generate_startup_script() -> &'static str {
        r#"#!/bin/bash
        # Update system
        apt-get update
        
        # Install Docker if not present
        if ! command -v docker &> /dev/null; then
            curl -fsSL https://get.docker.com | sh
            usermod -aG docker ubuntu
        fi
        
        # Install monitoring agent
        curl -sSO https://dl.google.com/cloudagents/add-monitoring-agent-repo.sh
        bash add-monitoring-agent-repo.sh --also-install
        
        # Enable metrics collection
        systemctl enable stackdriver-agent
        systemctl start stackdriver-agent
        "#
    }

    /// Map resource requirements to GCP instance type
    fn map_instance(spec: &ResourceSpec) -> InstanceSelection {
        let gpu_count = spec.gpu_count;
        let instance_type = match (spec.cpu, spec.memory_gb, gpu_count) {
            // GPU instances
            (_, _, Some(1)) => "n1-standard-4", // Add GPU via accelerator API
            (_, _, Some(_)) => "n1-standard-8", // Multiple GPUs

            // Memory optimized
            (cpu, mem, _) if mem > cpu * 8.0 => {
                if mem <= 32.0 {
                    "n2-highmem-4"
                } else if mem <= 64.0 {
                    "n2-highmem-8"
                } else {
                    "n2-highmem-16"
                }
            }

            // CPU optimized
            (cpu, mem, _) if cpu > mem / 2.0 => {
                if cpu <= 4.0 {
                    "n2-highcpu-4"
                } else if cpu <= 8.0 {
                    "n2-highcpu-8"
                } else {
                    "n2-highcpu-16"
                }
            }

            // Standard instances
            (cpu, mem, _) if cpu <= 0.5 && mem <= 2.0 => "e2-micro",
            (cpu, mem, _) if cpu <= 1.0 && mem <= 4.0 => "e2-small",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "e2-medium",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "n2-standard-4",
            (cpu, mem, _) if cpu <= 8.0 && mem <= 32.0 => "n2-standard-8",
            (cpu, mem, _) if cpu <= 16.0 && mem <= 64.0 => "n2-standard-16",
            _ => "e2-standard-2",
        };

        InstanceSelection {
            instance_type: instance_type.to_string(),
            spot_capable: spec.allow_spot && !instance_type.starts_with("e2"),
            estimated_hourly_cost: Self::estimate_cost(instance_type),
        }
    }

    fn estimate_cost(instance_type: &str) -> Option<f64> {
        Some(match instance_type {
            "e2-micro" => 0.008,
            "e2-small" => 0.021,
            "e2-medium" => 0.042,
            "e2-standard-2" => 0.084,
            "n2-standard-4" => 0.194,
            "n2-standard-8" => 0.388,
            "n2-standard-16" => 0.776,
            "n2-highmem-4" => 0.260,
            "n2-highmem-8" => 0.520,
            "n2-highmem-16" => 1.040,
            "n2-highcpu-4" => 0.143,
            "n2-highcpu-8" => 0.286,
            "n2-highcpu-16" => 0.572,
            "n1-standard-4" => 0.190,
            "n1-standard-8" => 0.380,
            _ => 0.10,
        })
    }

    /// Terminate a GCE instance
    #[cfg(feature = "gcp")]
    pub async fn terminate_instance(&self, instance_name: &str, zone: &str) -> Result<()> {
        let url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
            self.project_id, zone, instance_name
        );

        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.access_token.as_ref().unwrap())
            .send()
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to terminate instance: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            warn!("Failed to terminate GCE instance: {}", error_text);
        } else {
            info!("Terminated GCE instance: {}", instance_name);
        }

        Ok(())
    }

    #[cfg(not(feature = "gcp"))]
    pub async fn terminate_instance(&self, _instance_name: &str, _zone: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcp_instance_mapping() {
        // Test basic specs
        let spec = ResourceSpec::basic();
        let result = GcpProvisioner::map_instance(&spec);
        assert!(result.instance_type.starts_with("e2") || result.instance_type.starts_with("n2"));

        // Test performance specs
        let spec = ResourceSpec::performance();
        let result = GcpProvisioner::map_instance(&spec);
        assert!(
            result.instance_type.contains("standard") || result.instance_type.contains("highcpu")
        );

        // Test GPU specs
        let mut spec = ResourceSpec::performance();
        spec.gpu_count = Some(1);
        let result = GcpProvisioner::map_instance(&spec);
        assert!(result.instance_type.starts_with("n1"));
    }

    #[test]
    fn test_cost_estimation() {
        assert!(GcpProvisioner::estimate_cost("e2-micro").unwrap() < 0.01);
        assert!(GcpProvisioner::estimate_cost("n2-standard-16").unwrap() > 0.5);
    }
}

// Re-export both provisioner and adapter
pub use adapter::GcpAdapter;
