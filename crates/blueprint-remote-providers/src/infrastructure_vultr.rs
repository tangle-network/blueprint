//! Vultr infrastructure provisioning support
//! 
//! Provides Vultr resource provisioning capabilities including
//! Cloud Compute instances and Kubernetes clusters.

use crate::error::{Error, Result};
use crate::resources::ResourceSpec;
use crate::provisioning::InstanceSelection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};

/// Vultr infrastructure provisioner
pub struct VultrProvisioner {
    #[cfg(feature = "api-clients")]
    client: reqwest::Client,
    api_key: String,
    default_region: String,
}

impl VultrProvisioner {
    /// Create a new Vultr provisioner
    pub async fn new(api_key: String, default_region: String) -> Result<Self> {
        #[cfg(feature = "api-clients")]
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::ConfigurationError(e.to_string()))?;
        
        Ok(Self {
            #[cfg(feature = "api-clients")]
            client,
            api_key,
            default_region,
        })
    }
    
    /// Create a Vultr instance
    pub async fn create_instance(
        &self,
        label: &str,
        spec: &ResourceSpec,
        ssh_keys: Vec<String>,
    ) -> Result<VultrInstance> {
        let plan_id = self.select_plan(spec);
        
        #[cfg(feature = "api-clients")]
        {
            let url = "https://api.vultr.com/v2/instances";
            
            // Get OS ID for Ubuntu 22.04
            let os_id = self.get_ubuntu_os_id().await?;
            
            let instance_request = serde_json::json!({
                "region": self.default_region,
                "plan": plan_id,
                "os_id": os_id,
                "label": label,
                "hostname": label,
                "sshkey_id": ssh_keys,
                "backups": if spec.qos.backup_config.enabled { "enabled" } else { "disabled" },
                "enable_ipv6": spec.network.ipv6_enabled,
                "ddos_protection": spec.network.ddos_protection,
                "activation_email": false,
                "tag": "blueprint",
                "user_data": base64::encode(self.generate_user_data(spec)),
            });
            
            let response = self.client
                .post(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&instance_request)
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to create Vultr instance: {}", e)))?;
            
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(Error::ConfigurationError(format!("Vultr API error: {}", error_text)));
            }
            
            let json: serde_json::Value = response.json().await
                .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
            
            let instance_id = json["instance"]["id"].as_str()
                .ok_or_else(|| Error::ConfigurationError("No instance ID in response".into()))?;
            
            // Wait for instance to be active
            self.wait_for_instance_active(instance_id).await?;
            
            // Get instance details with IP addresses
            return self.get_instance_details(instance_id).await;
        }
        
        #[cfg(not(feature = "api-clients"))]
        Ok(VultrInstance {
            id: "abc123".to_string(),
            label: label.to_string(),
            plan: plan_id,
            region: self.default_region.clone(),
            status: "active".to_string(),
            main_ip: Some("45.63.1.2".to_string()),
            internal_ip: Some("10.1.0.2".to_string()),
            v6_main_ip: spec.network.ipv6_enabled.then(|| "2001:19f0:1:2::3".to_string()),
            vcpu_count: spec.compute.cpu_cores as u32,
            ram: (spec.storage.memory_gb * 1024.0) as u32,
        })
    }
    
    /// Get Ubuntu OS ID
    #[cfg(feature = "api-clients")]
    async fn get_ubuntu_os_id(&self) -> Result<u32> {
        let url = "https://api.vultr.com/v2/os";
        
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get OS list: {}", e)))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
        
        // Find Ubuntu 22.04
        json["os"]
            .as_array()
            .and_then(|oses| oses.iter().find(|os| {
                os["name"].as_str()
                    .map(|name| name.contains("Ubuntu 22.04"))
                    .unwrap_or(false)
            }))
            .and_then(|os| os["id"].as_u64())
            .map(|id| id as u32)
            .ok_or_else(|| Error::ConfigurationError("Ubuntu 22.04 not found".into()))
    }
    
    /// Wait for instance to be active
    #[cfg(feature = "api-clients")]
    async fn wait_for_instance_active(&self, instance_id: &str) -> Result<()> {
        let mut attempts = 0;
        loop {
            if attempts > 60 {
                return Err(Error::ConfigurationError("Timeout waiting for instance".into()));
            }
            
            let instance = self.get_instance_details(instance_id).await?;
            if instance.status == "active" {
                return Ok(());
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }
    
    /// Get instance details
    #[cfg(feature = "api-clients")]
    async fn get_instance_details(&self, instance_id: &str) -> Result<VultrInstance> {
        let url = format!("https://api.vultr.com/v2/instances/{}", instance_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get instance: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ConfigurationError("Failed to get instance details".into()));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
        
        let instance = &json["instance"];
        
        Ok(VultrInstance {
            id: instance_id.to_string(),
            label: instance["label"].as_str().unwrap_or("").to_string(),
            plan: instance["plan"].as_str().unwrap_or("").to_string(),
            region: instance["region"].as_str().unwrap_or("").to_string(),
            status: instance["status"].as_str().unwrap_or("unknown").to_string(),
            main_ip: instance["main_ip"].as_str().map(|s| s.to_string()),
            internal_ip: instance["internal_ip"].as_str().map(|s| s.to_string()),
            v6_main_ip: instance["v6_main_ip"].as_str().map(|s| s.to_string()),
            vcpu_count: instance["vcpu_count"].as_u64().unwrap_or(0) as u32,
            ram: instance["ram"].as_u64().unwrap_or(0) as u32,
        })
    }
    
    /// Create a Vultr Kubernetes Engine (VKE) cluster
    pub async fn create_vke_cluster(
        &self,
        label: &str,
        spec: &ResourceSpec,
        node_count: u32,
    ) -> Result<VkeCluster> {
        let node_plan = self.select_plan(spec);
        
        #[cfg(feature = "api-clients")]
        {
            let url = "https://api.vultr.com/v2/kubernetes/clusters";
            
            // Get latest Kubernetes version
            let version = self.get_latest_k8s_version().await?;
            
            let cluster_request = serde_json::json!({
                "label": label,
                "region": self.default_region,
                "version": version,
                "ha_controlplanes": spec.qos.availability_sla > 99.5,
                "enable_firewall": true,
                "node_pools": [{
                    "node_quantity": node_count,
                    "plan": node_plan,
                    "label": format!("{}-pool", label),
                    "auto_scaler": node_count > 1,
                    "min_nodes": 1,
                    "max_nodes": node_count * 2,
                    "tag": "blueprint",
                }],
            });
            
            let response = self.client
                .post(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&cluster_request)
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to create VKE cluster: {}", e)))?;
            
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(Error::ConfigurationError(format!("VKE API error: {}", error_text)));
            }
            
            let json: serde_json::Value = response.json().await
                .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
            
            let cluster_id = json["vke_cluster"]["id"].as_str()
                .ok_or_else(|| Error::ConfigurationError("No cluster ID in response".into()))?;
            
            // Wait for cluster to be ready
            self.wait_for_cluster_ready(cluster_id).await?;
            
            // Get cluster details
            return self.get_cluster_details(cluster_id).await;
        }
        
        #[cfg(not(feature = "api-clients"))]
        Ok(VkeCluster {
            id: "vke-123".to_string(),
            label: label.to_string(),
            region: self.default_region.clone(),
            version: "v1.28.2".to_string(),
            status: "active".to_string(),
            endpoint: format!("https://{}.vke.vultr.com", label),
            node_count,
            ip: "192.0.2.1".to_string(),
        })
    }
    
    /// Get latest Kubernetes version for VKE
    #[cfg(feature = "api-clients")]
    async fn get_latest_k8s_version(&self) -> Result<String> {
        let url = "https://api.vultr.com/v2/kubernetes/versions";
        
        let response = self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get K8s versions: {}", e)))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
        
        json["versions"]
            .as_array()
            .and_then(|versions| versions.first())
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::ConfigurationError("No K8s versions available".into()))
    }
    
    /// Wait for cluster to be ready
    #[cfg(feature = "api-clients")]
    async fn wait_for_cluster_ready(&self, cluster_id: &str) -> Result<()> {
        let mut attempts = 0;
        loop {
            if attempts > 120 { // 10 minutes
                return Err(Error::ConfigurationError("Timeout waiting for cluster".into()));
            }
            
            let cluster = self.get_cluster_details(cluster_id).await?;
            if cluster.status == "active" {
                return Ok(());
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }
    
    /// Get cluster details
    #[cfg(feature = "api-clients")]
    async fn get_cluster_details(&self, cluster_id: &str) -> Result<VkeCluster> {
        let url = format!("https://api.vultr.com/v2/kubernetes/clusters/{}", cluster_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get cluster: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::ConfigurationError("Failed to get cluster details".into()));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
        
        let cluster = &json["vke_cluster"];
        
        Ok(VkeCluster {
            id: cluster_id.to_string(),
            label: cluster["label"].as_str().unwrap_or("").to_string(),
            region: cluster["region"].as_str().unwrap_or("").to_string(),
            version: cluster["version"].as_str().unwrap_or("").to_string(),
            status: cluster["status"].as_str().unwrap_or("unknown").to_string(),
            endpoint: cluster["endpoint"].as_str().unwrap_or("").to_string(),
            node_count: cluster["node_pools"][0]["node_quantity"].as_u64().unwrap_or(0) as u32,
            ip: cluster["ip"].as_str().unwrap_or("").to_string(),
        })
    }
    
    /// Get kubeconfig for a VKE cluster
    pub async fn get_kubeconfig(&self, cluster_id: &str) -> Result<String> {
        #[cfg(feature = "api-clients")]
        {
            let url = format!("https://api.vultr.com/v2/kubernetes/clusters/{}/config", cluster_id);
            
            let response = self.client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to get kubeconfig: {}", e)))?;
            
            if !response.status().is_success() {
                return Err(Error::ConfigurationError("Failed to get kubeconfig".into()));
            }
            
            let json: serde_json::Value = response.json().await
                .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;
            
            // Kubeconfig is base64 encoded
            let kubeconfig_b64 = json["kube_config"].as_str()
                .ok_or_else(|| Error::ConfigurationError("No kubeconfig in response".into()))?;
            
            let kubeconfig = base64::decode(kubeconfig_b64)
                .map_err(|e| Error::ConfigurationError(format!("Failed to decode kubeconfig: {}", e)))?;
            
            return Ok(String::from_utf8_lossy(&kubeconfig).to_string());
        }
        
        #[cfg(not(feature = "api-clients"))]
        Ok("mock-kubeconfig".to_string())
    }
    
    /// Select appropriate Vultr plan based on resource requirements
    fn select_plan(&self, spec: &ResourceSpec) -> String {
        // Check for GPU (Vultr has limited GPU support)
        if spec.accelerators.is_some() {
            warn!("Vultr GPU support is limited, using high-performance CPU instance");
            return "vhp-8c-32gb".to_string(); // High performance instance
        }
        
        // Regular cloud compute plans
        match (spec.compute.cpu_cores, spec.storage.memory_gb) {
            (cpu, mem) if cpu <= 1.0 && mem <= 1.0 => "vc2-1c-1gb",
            (cpu, mem) if cpu <= 1.0 && mem <= 2.0 => "vc2-1c-2gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "vc2-2c-4gb",
            (cpu, mem) if cpu <= 4.0 && mem <= 8.0 => "vc2-4c-8gb",
            (cpu, mem) if cpu <= 6.0 && mem <= 16.0 => "vc2-6c-16gb",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "vc2-8c-32gb",
            (cpu, mem) if cpu <= 16.0 && mem <= 64.0 => "vc2-16c-64gb",
            (cpu, mem) if cpu <= 24.0 && mem <= 96.0 => "vc2-24c-96gb",
            // High frequency plans for CPU-intensive
            (cpu, _) if cpu > 24.0 => "vhf-12c-48gb",
            // High memory plans
            (_, mem) if mem > 96.0 => "vhm-8c-128gb",
            _ => "vc2-2c-4gb",
        }.to_string()
    }
    
    /// Generate startup script for the instance
    fn generate_user_data(&self, spec: &ResourceSpec) -> String {
        let mut script = String::from("#!/bin/bash\n\n");
        
        // Update system
        script.push_str("apt-get update\n");
        script.push_str("apt-get upgrade -y\n\n");
        
        // Install Docker
        script.push_str("curl -fsSL https://get.docker.com | sh\n");
        script.push_str("systemctl enable docker\n");
        script.push_str("systemctl start docker\n\n");
        
        // Configure resource limits via cgroups
        if spec.compute.cpu_cores > 0.0 {
            script.push_str(&format!(
                "echo 'docker run --cpus=\"{}\"' >> /etc/profile.d/blueprint.sh\n",
                spec.compute.cpu_cores
            ));
        }
        
        if spec.storage.memory_gb > 0.0 {
            script.push_str(&format!(
                "echo 'docker run --memory=\"{}g\"' >> /etc/profile.d/blueprint.sh\n",
                spec.storage.memory_gb
            ));
        }
        
        // Setup monitoring
        script.push_str("# Install monitoring tools\n");
        script.push_str("apt-get install -y prometheus-node-exporter\n");
        script.push_str("systemctl enable prometheus-node-exporter\n");
        script.push_str("systemctl start prometheus-node-exporter\n");
        
        script
    }
    
    /// Delete a Vultr instance
    pub async fn delete_instance(&self, instance_id: &str) -> Result<()> {
        #[cfg(feature = "api-clients")]
        {
            let url = format!("https://api.vultr.com/v2/instances/{}", instance_id);
            
            let response = self.client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to delete instance: {}", e)))?;
            
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(Error::ConfigurationError(format!("Failed to delete instance: {}", error_text)));
            }
        }
        
        info!("Deleted instance: {}", instance_id);
        Ok(())
    }
    
    /// Delete a VKE cluster
    pub async fn delete_vke_cluster(&self, cluster_id: &str) -> Result<()> {
        #[cfg(feature = "api-clients")]
        {
            let url = format!("https://api.vultr.com/v2/kubernetes/clusters/{}", cluster_id);
            
            let response = self.client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to delete cluster: {}", e)))?;
            
            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(Error::ConfigurationError(format!("Failed to delete cluster: {}", error_text)));
            }
        }
        
        info!("Deleted VKE cluster: {}", cluster_id);
        Ok(())
    }
}

/// Vultr instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VultrInstance {
    pub id: String,
    pub label: String,
    pub plan: String,
    pub region: String,
    pub status: String,
    pub main_ip: Option<String>,
    pub internal_ip: Option<String>,
    pub v6_main_ip: Option<String>,
    pub vcpu_count: u32,
    pub ram: u32,
}

/// Vultr Kubernetes Engine cluster information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VkeCluster {
    pub id: String,
    pub label: String,
    pub region: String,
    pub version: String,
    pub status: String,
    pub endpoint: String,
    pub node_count: u32,
    pub ip: String,
}

// Helper for base64 encoding
#[cfg(not(feature = "api-clients"))]
mod base64 {
    pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
        "mock-base64".to_string()
    }
    
    pub fn decode<T: AsRef<[u8]>>(input: T) -> std::result::Result<Vec<u8>, String> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{ResourceSpec, ComputeResources, StorageResources};
    
    #[test]
    fn test_plan_selection() {
        let provisioner = VultrProvisioner {
            #[cfg(feature = "api-clients")]
            client: reqwest::Client::new(),
            api_key: "test".to_string(),
            default_region: "ewr".to_string(),
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
        
        assert_eq!(provisioner.select_plan(&spec), "vc2-1c-2gb");
        
        // Test large instance
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 8.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 32.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        assert_eq!(provisioner.select_plan(&spec), "vc2-8c-32gb");
    }
    
    #[test]
    fn test_user_data_generation() {
        let provisioner = VultrProvisioner {
            #[cfg(feature = "api-clients")]
            client: reqwest::Client::new(),
            api_key: "test".to_string(),
            default_region: "ewr".to_string(),
        };
        
        let spec = ResourceSpec {
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
        
        let user_data = provisioner.generate_user_data(&spec);
        assert!(user_data.contains("#!/bin/bash"));
        assert!(user_data.contains("docker"));
        assert!(user_data.contains("--cpus=\"2\""));
        assert!(user_data.contains("--memory=\"4g\""));
    }
}