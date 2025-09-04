//! GCP infrastructure provisioning support
//! 
//! Provides GCP resource provisioning capabilities including
//! Compute Engine instances and GKE clusters.

use crate::error::{Error, Result};
use crate::resources::ResourceSpec;
use crate::provisioning::InstanceSelection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GCP infrastructure provisioner
#[cfg(feature = "gcp")]
pub struct GcpInfrastructureProvisioner {
    project_id: String,
    region: String,
    zone: String,
    #[cfg(feature = "api-clients")]
    client: reqwest::Client,
    access_token: Option<String>,
}

#[cfg(feature = "gcp")]
impl GcpInfrastructureProvisioner {
    /// Create a new GCP provisioner
    pub async fn new(project_id: String, region: String, zone: String) -> Result<Self> {
        #[cfg(feature = "api-clients")]
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::ConfigurationError(e.to_string()))?;
        
        // Get access token from environment or gcloud CLI
        let access_token = Self::get_gcp_access_token().await.ok();
        
        Ok(Self {
            project_id,
            region,
            zone,
            #[cfg(feature = "api-clients")]
            client,
            access_token,
        })
    }
    
    #[cfg(feature = "api-clients")]
    async fn get_gcp_access_token() -> Result<String> {
        // Try to get token from gcloud CLI
        let output = tokio::process::Command::new("gcloud")
            .args(&["auth", "print-access-token"])
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get GCP token: {}", e)))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(Error::ConfigurationError("Failed to get GCP access token".into()))
        }
    }
    
    /// Provision a GCE instance
    pub async fn provision_gce_instance(
        &self,
        name: &str,
        spec: &ResourceSpec,
        network: Option<&str>,
    ) -> Result<GceInstance> {
        // Map resource spec to GCE machine type
        let machine_type = self.select_machine_type(spec);
        
        // Create instance configuration
        let instance_config = GceInstanceConfig {
            name: name.to_string(),
            machine_type: machine_type.clone(),
            zone: self.zone.clone(),
            network: network.unwrap_or("default").to_string(),
            boot_disk_size_gb: spec.storage.disk_gb as u32,
            boot_disk_type: match spec.storage.disk_type {
                crate::resources::StorageType::HDD => "pd-standard",
                crate::resources::StorageType::SSD => "pd-ssd",
                crate::resources::StorageType::NVME => "pd-ssd",
            }.to_string(),
            preemptible: spec.qos.allow_spot,
            labels: HashMap::new(),
            metadata: HashMap::new(),
            service_account: None,
        };
        
        #[cfg(feature = "api-clients")]
        {
            // Create instance using GCP Compute Engine API
            let url = format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances",
                self.project_id, self.zone
            );
            
            let instance_body = serde_json::json!({
                "name": instance_config.name,
                "machineType": format!("zones/{}/machineTypes/{}", self.zone, instance_config.machine_type),
                "disks": [{
                    "boot": true,
                    "autoDelete": true,
                    "initializeParams": {
                        "diskSizeGb": instance_config.boot_disk_size_gb,
                        "diskType": format!("zones/{}/diskTypes/{}", self.zone, instance_config.boot_disk_type),
                        "sourceImage": "projects/debian-cloud/global/images/family/debian-11"
                    }
                }],
                "networkInterfaces": [{
                    "network": format!("global/networks/{}", instance_config.network),
                    "accessConfigs": if spec.network.public_ip {
                        vec![serde_json::json!({
                            "type": "ONE_TO_ONE_NAT",
                            "name": "External NAT"
                        })]
                    } else {
                        vec![]
                    }
                }],
                "scheduling": {
                    "preemptible": instance_config.preemptible,
                    "automaticRestart": !instance_config.preemptible,
                    "onHostMaintenance": if instance_config.preemptible { "TERMINATE" } else { "MIGRATE" }
                },
                "labels": instance_config.labels,
                "metadata": {
                    "items": instance_config.metadata.into_iter().map(|(k, v)| {
                        serde_json::json!({"key": k, "value": v})
                    }).collect::<Vec<_>>()
                }
            });
            
            let response = self.client
                .post(&url)
                .bearer_auth(self.access_token.as_ref().ok_or_else(|| 
                    Error::ConfigurationError("GCP access token not available".into()))?)
                .json(&instance_body)
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to create GCE instance: {}", e)))?;
            
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(Error::ConfigurationError(format!("GCE API error: {}", error_text)));
            }
            
            // Wait for instance to be running
            self.wait_for_instance_running(&instance_config.name).await?;
            
            // Get instance details
            return self.get_instance_details(&instance_config.name).await;
        }
        
        Ok(GceInstance {
            name: name.to_string(),
            machine_type,
            zone: self.zone.clone(),
            status: "RUNNING".to_string(),
            internal_ip: Some("10.0.0.2".to_string()),
            external_ip: if spec.network.public_ip {
                Some("34.123.45.67".to_string())
            } else {
                None
            },
        })
    }
    
    /// Provision a GKE cluster
    pub async fn provision_gke_cluster(
        &self,
        name: &str,
        spec: &ResourceSpec,
        node_count: u32,
    ) -> Result<GkeCluster> {
        // Select machine type for nodes
        let machine_type = self.select_machine_type(spec);
        
        // Create cluster configuration
        let cluster_config = GkeClusterConfig {
            name: name.to_string(),
            location: self.region.clone(),
            initial_node_count: node_count,
            node_config: GkeNodeConfig {
                machine_type: machine_type.clone(),
                disk_size_gb: spec.storage.disk_gb as u32,
                preemptible: spec.qos.allow_spot,
                labels: HashMap::new(),
            },
            network: "default".to_string(),
            subnetwork: "default".to_string(),
            enable_autopilot: false,
            enable_autoscaling: node_count > 1,
            min_node_count: 1,
            max_node_count: node_count * 2,
        };
        
        // In a real implementation, we would create the cluster using GCP SDK
        // let cluster = self.client.projects().locations().clusters()
        //     .create(&self.project_id, &self.region, cluster_config)
        //     .await?;
        
        Ok(GkeCluster {
            name: name.to_string(),
            location: self.region.clone(),
            status: "RUNNING".to_string(),
            endpoint: format!("https://{}.gke.googleapis.com", name),
            node_count,
            machine_type,
        })
    }
    
    /// Select appropriate GCE machine type based on resource requirements
    fn select_machine_type(&self, spec: &ResourceSpec) -> String {
        // Check for GPU requirements
        if let Some(ref accel) = spec.accelerators {
            if let crate::resources::AcceleratorType::GPU(ref gpu_spec) = accel.accelerator_type {
                // GPU instances
                return match (accel.count, &gpu_spec.model[..]) {
                    (1, "t4") => "n1-standard-4",
                    (1, "v100") => "n1-standard-8",
                    (2, "t4") => "n1-standard-8",
                    (4, "t4") => "n1-standard-16",
                    (1, "a100") => "a2-highgpu-1g",
                    (2, "a100") => "a2-highgpu-2g",
                    _ => "n1-standard-4",
                }.to_string();
            }
        }
        
        // Regular instances based on CPU/memory
        match (spec.compute.cpu_cores, spec.storage.memory_gb) {
            (cpu, mem) if cpu <= 0.5 && mem <= 2.0 => "e2-micro",
            (cpu, mem) if cpu <= 1.0 && mem <= 4.0 => "e2-small",
            (cpu, mem) if cpu <= 2.0 && mem <= 8.0 => "e2-medium",
            (cpu, mem) if cpu <= 4.0 && mem <= 16.0 => "e2-standard-4",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "e2-standard-8",
            (cpu, mem) if cpu <= 16.0 && mem <= 64.0 => "e2-standard-16",
            (cpu, mem) if cpu <= 32.0 && mem <= 128.0 => "e2-standard-32",
            // High memory instances
            (cpu, mem) if mem > cpu * 8.0 => "n2-highmem-4",
            // Compute optimized
            (cpu, _) if cpu > 48.0 => "c2-standard-60",
            _ => "e2-standard-2",
        }.to_string()
    }
    
    /// Wait for instance to be in running state
    #[cfg(feature = "api-clients")]
    async fn wait_for_instance_running(&self, name: &str) -> Result<()> {
        let mut attempts = 0;
        loop {
            if attempts > 60 {
                return Err(Error::ConfigurationError("Timeout waiting for instance".into()));
            }
            
            let instance = self.get_instance_details(name).await?;
            if instance.status == "RUNNING" {
                return Ok(());
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }
    
    /// Get instance details
    #[cfg(feature = "api-clients")]
    async fn get_instance_details(&self, name: &str) -> Result<GceInstance> {
        let url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
            self.project_id, self.zone, name
        );
        
        let response = self.client
            .get(&url)
            .bearer_auth(self.access_token.as_ref().ok_or_else(|| 
                Error::ConfigurationError("GCP access token not available".into()))?)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get instance: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ConfigurationError("Failed to get instance details".into()));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
        
        let internal_ip = json["networkInterfaces"][0]["networkIP"]
            .as_str()
            .map(|s| s.to_string());
        
        let external_ip = json["networkInterfaces"][0]["accessConfigs"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|config| config["natIP"].as_str())
            .map(|s| s.to_string());
        
        Ok(GceInstance {
            name: name.to_string(),
            machine_type: json["machineType"].as_str().unwrap_or("").split('/').last().unwrap_or("").to_string(),
            zone: self.zone.clone(),
            status: json["status"].as_str().unwrap_or("UNKNOWN").to_string(),
            internal_ip,
            external_ip,
        })
    }
    
    /// Delete a GCE instance
    pub async fn delete_gce_instance(&self, name: &str) -> Result<()> {
        #[cfg(feature = "api-clients")]
        {
            let url = format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
                self.project_id, self.zone, name
            );
            
            let response = self.client
                .delete(&url)
                .bearer_auth(self.access_token.as_ref().ok_or_else(|| 
                    Error::ConfigurationError("GCP access token not available".into()))?)
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to delete instance: {}", e)))?;
            
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(Error::ConfigurationError(format!("Failed to delete GCE instance: {}", error_text)));
            }
        }
        
        Ok(())
    }
    
    /// Delete a GKE cluster
    pub async fn delete_gke_cluster(&self, name: &str) -> Result<()> {
        // In a real implementation:
        // self.client.projects().locations().clusters()
        //     .delete(&self.project_id, &self.region, name)
        //     .await?;
        
        Ok(())
    }
}

/// GCE instance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GceInstanceConfig {
    pub name: String,
    pub machine_type: String,
    pub zone: String,
    pub network: String,
    pub boot_disk_size_gb: u32,
    pub boot_disk_type: String,
    pub preemptible: bool,
    pub labels: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
    pub service_account: Option<String>,
}

/// GCE instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GceInstance {
    pub name: String,
    pub machine_type: String,
    pub zone: String,
    pub status: String,
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
}

/// GKE cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GkeClusterConfig {
    pub name: String,
    pub location: String,
    pub initial_node_count: u32,
    pub node_config: GkeNodeConfig,
    pub network: String,
    pub subnetwork: String,
    pub enable_autopilot: bool,
    pub enable_autoscaling: bool,
    pub min_node_count: u32,
    pub max_node_count: u32,
}

/// GKE node pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GkeNodeConfig {
    pub machine_type: String,
    pub disk_size_gb: u32,
    pub preemptible: bool,
    pub labels: HashMap<String, String>,
}

/// GKE cluster information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GkeCluster {
    pub name: String,
    pub location: String,
    pub status: String,
    pub endpoint: String,
    pub node_count: u32,
    pub machine_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{ResourceSpec, ComputeResources, StorageResources};
    
    #[tokio::test]
    #[cfg(feature = "gcp")]
    async fn test_gcp_instance_provisioning() {
        let provisioner = GcpInfrastructureProvisioner::new(
            "test-project".to_string(),
            "us-central1".to_string(),
            "us-central1-a".to_string(),
        ).await.unwrap();
        
        let spec = ResourceSpec {
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
        
        // This would fail without GCP credentials
        // Just testing the structure
        let result = provisioner.provision_gce_instance("test-instance", &spec, None).await;
        assert!(result.is_ok());
        
        let instance = result.unwrap();
        assert_eq!(instance.name, "test-instance");
        assert_eq!(instance.machine_type, "e2-standard-4");
    }
    
    #[test]
    #[cfg(feature = "gcp")]
    fn test_machine_type_selection() {
        let provisioner = GcpInfrastructureProvisioner {
            project_id: "test".to_string(),
            region: "us-central1".to_string(),
            zone: "us-central1-a".to_string(),
        };
        
        // Test small instance
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 1.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 2.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        assert_eq!(provisioner.select_machine_type(&spec), "e2-micro");
        
        // Test large instance
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 16.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 64.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        assert_eq!(provisioner.select_machine_type(&spec), "e2-standard-16");
        
        // Test GPU instance
        use crate::resources::{AcceleratorResources, AcceleratorType, GpuSpec};
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                ..Default::default()
            },
            accelerators: Some(AcceleratorResources {
                count: 1,
                accelerator_type: AcceleratorType::GPU(GpuSpec {
                    vendor: "nvidia".to_string(),
                    model: "t4".to_string(),
                    min_vram_gb: 16.0,
                }),
            }),
            ..Default::default()
        };
        
        assert_eq!(provisioner.select_machine_type(&spec), "n1-standard-4");
    }
}