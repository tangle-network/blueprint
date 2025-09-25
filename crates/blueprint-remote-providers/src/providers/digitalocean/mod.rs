//! DigitalOcean infrastructure provisioning support
//!
//! Provides DigitalOcean resource provisioning capabilities including
//! Droplets and Kubernetes clusters.

pub mod adapter;

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::security::{ApiAuthentication, SecureHttpClient};
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::{info, warn};

/// DigitalOcean infrastructure provisioner
pub struct DigitalOceanProvisioner {
    client: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
}

impl DigitalOceanProvisioner {
    /// Create a new DigitalOcean provisioner
    pub async fn new(api_token: String, default_region: String) -> Result<Self> {
        let client = SecureHttpClient::new()?;
        let auth = ApiAuthentication::digitalocean(api_token);

        Ok(Self {
            client,
            auth,
            default_region,
        })
    }

    /// Create a Droplet
    pub async fn create_droplet(
        &self,
        name: &str,
        spec: &ResourceSpec,
        ssh_keys: Vec<String>,
    ) -> Result<Droplet> {
        let droplet_size = self.select_droplet_size(spec);

        let url = "https://api.digitalocean.com/v2/droplets";

        let droplet_request = serde_json::json!({
            "name": name,
            "region": self.default_region,
            "size": droplet_size,
            "image": "ubuntu-22-04-x64",
            "ssh_keys": ssh_keys,
            "backups": false,
            "ipv6": false,
            "monitoring": true,
            "tags": ["blueprint", "managed"],
            "user_data": self.generate_user_data(spec),
            "with_droplet_agent": true,
        });

        let response = self
            .client
            .post_json(url, &self.auth, droplet_request)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create droplet: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "DO API error: {}",
                error_text
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;

        let droplet_id = json["droplet"]["id"]
            .as_u64()
            .ok_or_else(|| Error::ConfigurationError("No droplet ID in response".into()))?;

        // Wait for droplet to be active
        self.wait_for_droplet_active(droplet_id).await?;

        // Get droplet details with IP addresses
        self.get_droplet_details(droplet_id).await
    }

    /// Wait for droplet to be active
    async fn wait_for_droplet_active(&self, droplet_id: u64) -> Result<()> {
        let mut attempts = 0;
        loop {
            if attempts > 60 {
                return Err(Error::ConfigurationError(
                    "Timeout waiting for droplet".into(),
                ));
            }

            let droplet = self.get_droplet_details(droplet_id).await?;
            if droplet.status == "active" {
                return Ok(());
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }

    /// Get droplet details
    async fn get_droplet_details(&self, droplet_id: u64) -> Result<Droplet> {
        let url = format!("https://api.digitalocean.com/v2/droplets/{}", droplet_id);

        let response = self
            .client
            .get(&url, &self.auth)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get droplet: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ConfigurationError(
                "Failed to get droplet details".into(),
            ));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;

        let droplet = &json["droplet"];

        let public_ipv4 = droplet["networks"]["v4"]
            .as_array()
            .and_then(|nets| nets.iter().find(|n| n["type"] == "public"))
            .and_then(|n| n["ip_address"].as_str())
            .map(|s| s.to_string());

        let private_ipv4 = droplet["networks"]["v4"]
            .as_array()
            .and_then(|nets| nets.iter().find(|n| n["type"] == "private"))
            .and_then(|n| n["ip_address"].as_str())
            .map(|s| s.to_string());

        let public_ipv6 = droplet["networks"]["v6"]
            .as_array()
            .and_then(|nets| nets.first())
            .and_then(|n| n["ip_address"].as_str())
            .map(|s| s.to_string());

        Ok(Droplet {
            id: droplet_id,
            name: droplet["name"].as_str().unwrap_or("").to_string(),
            size: droplet["size"]["slug"].as_str().unwrap_or("").to_string(),
            region: droplet["region"]["slug"].as_str().unwrap_or("").to_string(),
            status: droplet["status"].as_str().unwrap_or("unknown").to_string(),
            public_ipv4,
            private_ipv4,
            public_ipv6,
        })
    }

    /// Create a Kubernetes cluster
    pub async fn create_kubernetes_cluster(
        &self,
        name: &str,
        spec: &ResourceSpec,
        node_count: u32,
    ) -> Result<DOKSCluster> {
        let _node_size = self.select_droplet_size(spec);

        let url = "https://api.digitalocean.com/v2/kubernetes/clusters";

        // Get latest Kubernetes version
        let version = self.get_latest_k8s_version().await?;
        let node_size = self.select_droplet_size(spec);

        let cluster_request = serde_json::json!({
            "name": name,
            "region": self.default_region,
            "version": version,
            "node_pools": [{
                "size": node_size,
                "count": node_count,
                "name": format!("{}-pool", name),
                "auto_scale": node_count > 1,
                "min_nodes": 1,
                "max_nodes": node_count * 2,
                "tags": ["blueprint"],
            }],
            "maintenance_policy": {
                "start_time": "03:00",
                "day": "sunday",
            },
            "auto_upgrade": false,
            "surge_upgrade": true,
        });

        let response = self
            .client
            .post(url, &self.auth, Some(cluster_request))
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to create DOKS cluster: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "DOKS API error: {}",
                error_text
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;

        let cluster_id = json["kubernetes_cluster"]["id"]
            .as_str()
            .ok_or_else(|| Error::ConfigurationError("No cluster ID in response".into()))?;

        // Wait for cluster to be running
        self.wait_for_cluster_running(cluster_id).await?;

        // Get cluster details
        self.get_cluster_details(cluster_id).await
    }

    /// Get latest Kubernetes version
    async fn get_latest_k8s_version(&self) -> Result<String> {
        let url = format!("https://api.digitalocean.com/v2/kubernetes/options");

        let response = self
            .client
            .get(&url, &self.auth)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get K8s versions: {}", e)))?;

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;

        json["options"]["versions"]
            .as_array()
            .and_then(|versions| versions.first())
            .and_then(|v| v["slug"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::ConfigurationError("No K8s versions available".into()))
    }

    /// Wait for cluster to be running
    async fn wait_for_cluster_running(&self, cluster_id: &str) -> Result<()> {
        let mut attempts = 0;
        loop {
            if attempts > 120 {
                // 10 minutes
                return Err(Error::ConfigurationError(
                    "Timeout waiting for cluster".into(),
                ));
            }

            let cluster = self.get_cluster_details(cluster_id).await?;
            if cluster.status == "running" {
                return Ok(());
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }

    /// Get cluster details
    async fn get_cluster_details(&self, cluster_id: &str) -> Result<DOKSCluster> {
        let url = format!(
            "https://api.digitalocean.com/v2/kubernetes/clusters/{}",
            cluster_id
        );

        let response = self
            .client
            .get(&url, &self.auth)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get cluster: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ConfigurationError(
                "Failed to get cluster details".into(),
            ));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {}", e)))?;

        let cluster = &json["kubernetes_cluster"];

        Ok(DOKSCluster {
            id: cluster_id.to_string(),
            name: cluster["name"].as_str().unwrap_or("").to_string(),
            region: cluster["region"].as_str().unwrap_or("").to_string(),
            version: cluster["version"].as_str().unwrap_or("").to_string(),
            status: cluster["status"]["state"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            endpoint: cluster["endpoint"].as_str().unwrap_or("").to_string(),
            node_count: cluster["node_pools"][0]["count"].as_u64().unwrap_or(0) as u32,
        })
    }

    /// Get kubeconfig for a cluster
    pub async fn get_kubeconfig(&self, cluster_id: &str) -> Result<String> {
        let url = format!(
            "https://api.digitalocean.com/v2/kubernetes/clusters/{}/kubeconfig",
            cluster_id
        );

        let response = self
            .client
            .get(&url, &self.auth)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get kubeconfig: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::ConfigurationError("Failed to get kubeconfig".into()));
        }

        response
            .text()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to read kubeconfig: {}", e)))
    }

    /// Select appropriate droplet size based on resource requirements
    fn select_droplet_size(&self, spec: &ResourceSpec) -> String {
        // Check for GPU (DO doesn't have GPU instances yet)
        if spec.gpu_count.is_some() {
            warn!("DigitalOcean doesn't support GPU instances, using CPU instance");
        }

        // Map to droplet sizes
        match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 1.0 && mem <= 0.5 => "s-1vcpu-512mb",
            (cpu, mem) if cpu <= 1.0 && mem <= 1.0 => "s-1vcpu-1gb",
            (cpu, mem) if cpu <= 1.0 && mem <= 2.0 => "s-1vcpu-2gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 2.0 => "s-2vcpu-2gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "s-2vcpu-4gb",
            (cpu, mem) if cpu <= 4.0 && mem <= 8.0 => "s-4vcpu-8gb",
            (cpu, mem) if cpu <= 6.0 && mem <= 16.0 => "s-6vcpu-16gb",
            (cpu, mem) if cpu <= 8.0 && mem <= 16.0 => "s-8vcpu-16gb",
            (cpu, mem) if cpu <= 16.0 && mem <= 32.0 => "s-16vcpu-32gb",
            (cpu, mem) if cpu <= 32.0 && mem <= 64.0 => "s-32vcpu-64gb",
            // CPU optimized
            (cpu, _) if cpu > 32.0 => "c-48",
            // Memory optimized
            (_, mem) if mem > 64.0 => "m-32vcpu-256gb",
            _ => "s-2vcpu-4gb",
        }
        .to_string()
    }

    /// Generate cloud-init user data
    fn generate_user_data(&self, spec: &ResourceSpec) -> String {
        let mut user_data = String::from("#cloud-config\n");

        // Install Docker
        user_data.push_str("packages:\n");
        user_data.push_str("  - docker.io\n");
        user_data.push_str("  - docker-compose\n\n");

        // Configure resource limits via systemd
        if spec.cpu > 0.0 || spec.memory_gb > 0.0 {
            user_data.push_str("write_files:\n");
            user_data.push_str("  - path: /etc/systemd/system/blueprint.service\n");
            user_data.push_str("    content: |\n");
            user_data.push_str("      [Unit]\n");
            user_data.push_str("      Description=Blueprint Service\n");
            user_data.push_str("      After=docker.service\n");
            user_data.push_str("      [Service]\n");
            user_data.push_str(&format!("      CPUQuota={}%\n", (spec.cpu * 100.0) as u32));
            user_data.push_str(&format!(
                "      MemoryMax={}M\n",
                (spec.memory_gb * 1024.0) as u32
            ));
            user_data.push_str("      Restart=always\n");
            user_data.push_str("      [Install]\n");
            user_data.push_str("      WantedBy=multi-user.target\n");
        }

        // Enable monitoring
        user_data.push_str("\nruncmd:\n");
        user_data.push_str("  - systemctl enable docker\n");
        user_data.push_str("  - systemctl start docker\n");

        user_data
    }

    /// Delete a droplet
    pub async fn delete_droplet(&self, droplet_id: u64) -> Result<()> {
        let url = format!("https://api.digitalocean.com/v2/droplets/{}", droplet_id);

        let response = self
            .client
            .delete(&url, &self.auth)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to delete droplet: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Failed to delete droplet: {}",
                error_text
            )));
        }

        info!("Deleted droplet: {}", droplet_id);
        Ok(())
    }

    /// Delete a Kubernetes cluster
    pub async fn delete_kubernetes_cluster(&self, cluster_id: &str) -> Result<()> {
        let url = format!(
            "https://api.digitalocean.com/v2/kubernetes/clusters/{}",
            cluster_id
        );

        let response = self
            .client
            .delete(&url, &self.auth)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to delete cluster: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Failed to delete cluster: {}",
                error_text
            )));
        }

        info!("Deleted Kubernetes cluster: {}", cluster_id);
        Ok(())
    }
}

/// DigitalOcean Droplet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Droplet {
    pub id: u64,
    pub name: String,
    pub size: String,
    pub region: String,
    pub status: String,
    pub public_ipv4: Option<String>,
    pub private_ipv4: Option<String>,
    pub public_ipv6: Option<String>,
}

/// DigitalOcean Kubernetes cluster information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DOKSCluster {
    pub id: String,
    pub name: String,
    pub region: String,
    pub version: String,
    pub status: String,
    pub endpoint: String,
    pub node_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::resources::ResourceSpec;

    #[tokio::test]
    async fn test_droplet_size_selection() {
        let provisioner = DigitalOceanProvisioner::new(
            "test_token".to_string(),
            "nyc3".to_string(),
        ).await.unwrap();

        // Test small instance
        let spec = ResourceSpec {
            cpu: 1.0,
            memory_gb: 1.0,
            storage_gb: 25.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        assert_eq!(provisioner.select_droplet_size(&spec), "s-1vcpu-1gb");

        // Test large instance
        let spec = ResourceSpec {
            cpu: 8.0,
            memory_gb: 16.0,
            storage_gb: 160.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        assert_eq!(provisioner.select_droplet_size(&spec), "s-8vcpu-16gb");
    }

    #[tokio::test]
    async fn test_user_data_generation() {
        let provisioner = DigitalOceanProvisioner::new(
            "test_token".to_string(),
            "nyc3".to_string(),
        ).await.unwrap();

        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 80.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let user_data = provisioner.generate_user_data(&spec);
        assert!(user_data.contains("#cloud-config"));
        assert!(user_data.contains("docker.io"));
        assert!(user_data.contains("CPUQuota=200%"));
        assert!(user_data.contains("MemoryMax=4096M"));
    }
}

impl fmt::Debug for DigitalOceanProvisioner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DigitalOceanProvisioner")
            .field("api_token", &"[REDACTED]")
            .field("default_region", &self.default_region)
            .finish()
    }
}
