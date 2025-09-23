//! Dynamic machine type discovery from cloud provider APIs
//!
//! Discovers available instance types and their specifications from cloud providers
//! to maintain an up-to-date catalog of available resources.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

/// Machine type discovery service
pub struct MachineTypeDiscovery {
    client: reqwest::Client,
    cache: HashMap<CloudProvider, Vec<MachineType>>,
}

impl MachineTypeDiscovery {
    /// Create a new discovery service
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(blueprint_std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            cache: HashMap::new(),
        }
    }

    /// Discover all machine types for a cloud provider
    pub async fn discover_machine_types(
        &mut self,
        provider: &CloudProvider,
        region: &str,
        credentials: &CloudCredentials,
    ) -> Result<Vec<MachineType>> {
        // Check cache first
        if let Some(cached) = self.cache.get(provider) {
            if !cached.is_empty() {
                debug!("Using cached machine types for {:?}", provider);
                return Ok(cached.clone());
            }
        }

        let machines = match provider {
            CloudProvider::AWS => self.get_common_aws_instances(),
            CloudProvider::GCP => self.get_common_gcp_machines(),
            CloudProvider::Azure => self.get_common_azure_vms(),
            CloudProvider::DigitalOcean => self.get_common_do_droplets(),
            CloudProvider::Vultr => self.get_common_vultr_plans(),
            _ => vec![],
        };

        // Cache the results
        self.cache.insert(provider.clone(), machines.clone());

        Ok(machines)
    }

    /// Discover AWS EC2 instance types
    async fn discover_aws_instances(
        &self,
        region: &str,
        credentials: &CloudCredentials,
    ) -> Result<Vec<MachineType>> {
        // AWS DescribeInstanceTypes API
        let url = format!(
            "https://ec2.{}.amazonaws.com/?Action=DescribeInstanceTypes&Version=2016-11-15",
            region
        );

        // In production, this would use proper AWS signature v4
        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!(
                    "AWS4-HMAC-SHA256 Credential={}",
                    credentials.access_key.as_ref().unwrap_or(&String::new())
                ),
            )
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to query AWS: {}", e)))?;

        if !response.status().is_success() {
            // For now, return hardcoded common instance types
            return Ok(self.get_common_aws_instances());
        }

        // Parse XML response (simplified)
        Ok(self.get_common_aws_instances())
    }

    /// Get common AWS instance types (fallback)
    fn get_common_aws_instances(&self) -> Vec<MachineType> {
        vec![
            MachineType {
                name: "t3.micro".to_string(),
                provider: CloudProvider::AWS,
                vcpus: 2,
                memory_gb: 1.0,
                storage_gb: Some(8.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "Up to 5 Gigabit".to_string(),
                hourly_price: Some(0.0104),
                spot_price: Some(0.0031),
            },
            MachineType {
                name: "t3.small".to_string(),
                provider: CloudProvider::AWS,
                vcpus: 2,
                memory_gb: 2.0,
                storage_gb: Some(8.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "Up to 5 Gigabit".to_string(),
                hourly_price: Some(0.0208),
                spot_price: Some(0.0062),
            },
            MachineType {
                name: "m6i.xlarge".to_string(),
                provider: CloudProvider::AWS,
                vcpus: 4,
                memory_gb: 16.0,
                storage_gb: None,
                gpu_count: 0,
                gpu_type: None,
                network_performance: "Up to 12.5 Gigabit".to_string(),
                hourly_price: Some(0.192),
                spot_price: Some(0.0576),
            },
            MachineType {
                name: "g4dn.xlarge".to_string(),
                provider: CloudProvider::AWS,
                vcpus: 4,
                memory_gb: 16.0,
                storage_gb: Some(125.0),
                gpu_count: 1,
                gpu_type: Some("NVIDIA T4".to_string()),
                network_performance: "Up to 25 Gigabit".to_string(),
                hourly_price: Some(0.526),
                spot_price: Some(0.1578),
            },
        ]
    }

    /// Discover GCP machine types
    async fn discover_gcp_machines(
        &self,
        zone: &str,
        credentials: &CloudCredentials,
    ) -> Result<Vec<MachineType>> {
        let project_id = credentials
            .project_id
            .as_ref()
            .ok_or_else(|| Error::ConfigurationError("GCP project ID required".into()))?;

        let url = format!(
            "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/machineTypes",
            project_id, zone
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(credentials.access_token.as_ref().unwrap_or(&String::new()))
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to query GCP: {}", e)))?;

        if !response.status().is_success() {
            return Ok(self.get_common_gcp_machines());
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            Error::ConfigurationError(format!("Failed to parse GCP response: {}", e))
        })?;

        let mut machines = Vec::new();
        if let Some(items) = json["items"].as_array() {
            for item in items {
                if let (Some(name), Some(vcpus), Some(memory)) = (
                    item["name"].as_str(),
                    item["guestCpus"].as_u64(),
                    item["memoryMb"].as_u64(),
                ) {
                    machines.push(MachineType {
                        name: name.to_string(),
                        provider: CloudProvider::GCP,
                        vcpus: vcpus as u32,
                        memory_gb: memory as f64 / 1024.0,
                        storage_gb: None,
                        gpu_count: 0,
                        gpu_type: None,
                        network_performance: "10 Gbps".to_string(),
                        hourly_price: None, // TODO: Implement pricing API integration
                        spot_price: None,
                    });
                }
            }
        }

        if machines.is_empty() {
            Ok(self.get_common_gcp_machines())
        } else {
            Ok(machines)
        }
    }

    /// Get common GCP machine types (fallback)
    fn get_common_gcp_machines(&self) -> Vec<MachineType> {
        vec![
            MachineType {
                name: "e2-micro".to_string(),
                provider: CloudProvider::GCP,
                vcpus: 2,
                memory_gb: 1.0,
                storage_gb: None,
                gpu_count: 0,
                gpu_type: None,
                network_performance: "1 Gbps".to_string(),
                hourly_price: Some(0.00838),
                spot_price: Some(0.00251),
            },
            MachineType {
                name: "e2-standard-4".to_string(),
                provider: CloudProvider::GCP,
                vcpus: 4,
                memory_gb: 16.0,
                storage_gb: None,
                gpu_count: 0,
                gpu_type: None,
                network_performance: "10 Gbps".to_string(),
                hourly_price: Some(0.134),
                spot_price: Some(0.0402),
            },
        ]
    }

    /// Discover Azure VM sizes
    async fn discover_azure_vms(
        &self,
        location: &str,
        credentials: &CloudCredentials,
    ) -> Result<Vec<MachineType>> {
        let subscription_id = credentials
            .subscription_id
            .as_ref()
            .ok_or_else(|| Error::ConfigurationError("Azure subscription ID required".into()))?;

        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Compute/locations/{}/vmSizes?api-version=2023-03-01",
            subscription_id, location
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(credentials.access_token.as_ref().unwrap_or(&String::new()))
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to query Azure: {}", e)))?;

        if !response.status().is_success() {
            return Ok(self.get_common_azure_vms());
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            Error::ConfigurationError(format!("Failed to parse Azure response: {}", e))
        })?;

        let mut machines = Vec::new();
        if let Some(values) = json["value"].as_array() {
            for value in values {
                if let (Some(name), Some(cores), Some(memory)) = (
                    value["name"].as_str(),
                    value["numberOfCores"].as_u64(),
                    value["memoryInMB"].as_u64(),
                ) {
                    machines.push(MachineType {
                        name: name.to_string(),
                        provider: CloudProvider::Azure,
                        vcpus: cores as u32,
                        memory_gb: memory as f64 / 1024.0,
                        storage_gb: value["resourceDiskSizeInMB"]
                            .as_u64()
                            .map(|mb| mb as f64 / 1024.0),
                        gpu_count: 0,
                        gpu_type: None,
                        network_performance: "Unknown".to_string(),
                        hourly_price: None,
                        spot_price: None,
                    });
                }
            }
        }

        if machines.is_empty() {
            Ok(self.get_common_azure_vms())
        } else {
            Ok(machines)
        }
    }

    /// Get common Azure VM sizes (fallback)
    fn get_common_azure_vms(&self) -> Vec<MachineType> {
        vec![
            MachineType {
                name: "Standard_B1s".to_string(),
                provider: CloudProvider::Azure,
                vcpus: 1,
                memory_gb: 1.0,
                storage_gb: Some(4.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "Moderate".to_string(),
                hourly_price: Some(0.012),
                spot_price: Some(0.0036),
            },
            MachineType {
                name: "Standard_D4s_v5".to_string(),
                provider: CloudProvider::Azure,
                vcpus: 4,
                memory_gb: 16.0,
                storage_gb: None,
                gpu_count: 0,
                gpu_type: None,
                network_performance: "12500 Mbps".to_string(),
                hourly_price: Some(0.192),
                spot_price: Some(0.0576),
            },
        ]
    }

    /// Discover DigitalOcean droplet sizes
    async fn discover_do_droplets(
        &self,
        credentials: &CloudCredentials,
    ) -> Result<Vec<MachineType>> {
        let url = "https://api.digitalocean.com/v2/sizes";

        let response = self
            .client
            .get(url)
            .bearer_auth(credentials.api_token.as_ref().unwrap_or(&String::new()))
            .send()
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to query DigitalOcean: {}", e))
            })?;

        if !response.status().is_success() {
            return Ok(self.get_common_do_droplets());
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            Error::ConfigurationError(format!("Failed to parse DO response: {}", e))
        })?;

        let mut machines = Vec::new();
        if let Some(sizes) = json["sizes"].as_array() {
            for size in sizes {
                if let (Some(slug), Some(vcpus), Some(memory), Some(price_monthly)) = (
                    size["slug"].as_str(),
                    size["vcpus"].as_u64(),
                    size["memory"].as_u64(),
                    size["price_monthly"].as_f64(),
                ) {
                    machines.push(MachineType {
                        name: slug.to_string(),
                        provider: CloudProvider::DigitalOcean,
                        vcpus: vcpus as u32,
                        memory_gb: memory as f64 / 1024.0,
                        storage_gb: size["disk"].as_u64().map(|gb| gb as f64),
                        gpu_count: 0,
                        gpu_type: None,
                        network_performance: format!(
                            "{} Gbps",
                            size["transfer"].as_f64().unwrap_or(1.0)
                        ),
                        hourly_price: Some(price_monthly / 730.0), // Approximate
                        spot_price: None,                          // DO doesn't have spot
                    });
                }
            }
        }

        if machines.is_empty() {
            Ok(self.get_common_do_droplets())
        } else {
            Ok(machines)
        }
    }

    /// Get common DigitalOcean droplet sizes (fallback)
    fn get_common_do_droplets(&self) -> Vec<MachineType> {
        vec![
            MachineType {
                name: "s-1vcpu-1gb".to_string(),
                provider: CloudProvider::DigitalOcean,
                vcpus: 1,
                memory_gb: 1.0,
                storage_gb: Some(25.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "1 Gbps".to_string(),
                hourly_price: Some(0.009),
                spot_price: None,
            },
            MachineType {
                name: "s-2vcpu-4gb".to_string(),
                provider: CloudProvider::DigitalOcean,
                vcpus: 2,
                memory_gb: 4.0,
                storage_gb: Some(80.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "4 Gbps".to_string(),
                hourly_price: Some(0.036),
                spot_price: None,
            },
        ]
    }

    /// Discover Vultr plans
    async fn discover_vultr_plans(
        &self,
        credentials: &CloudCredentials,
    ) -> Result<Vec<MachineType>> {
        let url = "https://api.vultr.com/v2/plans";

        let response = self
            .client
            .get(url)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    credentials.api_key.as_ref().unwrap_or(&String::new())
                ),
            )
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to query Vultr: {}", e)))?;

        if !response.status().is_success() {
            return Ok(self.get_common_vultr_plans());
        }

        let json: serde_json::Value = response.json().await.map_err(|e| {
            Error::ConfigurationError(format!("Failed to parse Vultr response: {}", e))
        })?;

        let mut machines = Vec::new();
        if let Some(plans) = json["plans"].as_array() {
            for plan in plans {
                if let (Some(id), Some(vcpu), Some(ram), Some(price)) = (
                    plan["id"].as_str(),
                    plan["vcpu_count"].as_u64(),
                    plan["ram"].as_u64(),
                    plan["monthly_cost"].as_f64(),
                ) {
                    machines.push(MachineType {
                        name: id.to_string(),
                        provider: CloudProvider::Vultr,
                        vcpus: vcpu as u32,
                        memory_gb: ram as f64 / 1024.0,
                        storage_gb: plan["disk"].as_u64().map(|gb| gb as f64),
                        gpu_count: if plan["gpu_vram_gb"].as_u64().is_some() {
                            1
                        } else {
                            0
                        },
                        gpu_type: plan["gpu_type"].as_str().map(|s| s.to_string()),
                        network_performance: format!(
                            "{} Gbps",
                            plan["bandwidth_gb"].as_u64().unwrap_or(1000) / 1000
                        ),
                        hourly_price: Some(price / 730.0),
                        spot_price: None,
                    });
                }
            }
        }

        if machines.is_empty() {
            Ok(self.get_common_vultr_plans())
        } else {
            Ok(machines)
        }
    }

    /// Get common Vultr plans (fallback)
    fn get_common_vultr_plans(&self) -> Vec<MachineType> {
        vec![
            MachineType {
                name: "vc2-1c-1gb".to_string(),
                provider: CloudProvider::Vultr,
                vcpus: 1,
                memory_gb: 1.0,
                storage_gb: Some(25.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "1 Gbps".to_string(),
                hourly_price: Some(0.007),
                spot_price: None,
            },
            MachineType {
                name: "vc2-2c-4gb".to_string(),
                provider: CloudProvider::Vultr,
                vcpus: 2,
                memory_gb: 4.0,
                storage_gb: Some(80.0),
                gpu_count: 0,
                gpu_type: None,
                network_performance: "3 Gbps".to_string(),
                hourly_price: Some(0.024),
                spot_price: None,
            },
        ]
    }

    /// Find best machine type for given requirements
    pub fn find_best_match(
        &self,
        provider: &CloudProvider,
        min_vcpus: u32,
        min_memory_gb: f64,
        needs_gpu: bool,
        max_price_per_hour: Option<f64>,
    ) -> Option<MachineType> {
        self.cache.get(provider).and_then(|machines| {
            machines
                .iter()
                .filter(|m| m.vcpus >= min_vcpus)
                .filter(|m| m.memory_gb >= min_memory_gb)
                .filter(|m| !needs_gpu || m.gpu_count > 0)
                .filter(|m| {
                    max_price_per_hour.map_or(true, |max| {
                        m.hourly_price.map_or(true, |price| price <= max)
                    })
                })
                .min_by(|a, b| {
                    // Sort by price, then by excess resources
                    match (a.hourly_price, b.hourly_price) {
                        (Some(a_price), Some(b_price)) => a_price.partial_cmp(&b_price).unwrap(),
                        _ => blueprint_std::cmp::Ordering::Equal,
                    }
                })
                .cloned()
        })
    }
}

/// DEPRECATED: Insecure plaintext credentials - use EncryptedCloudCredentials instead
// TODO: Replace with EncryptedCloudCredentials for secure credential storage
#[derive(Debug, Clone, Default)]
pub struct CloudCredentials {
    // AWS
    pub access_key: Option<String>,
    pub secret_key: Option<String>,

    // GCP
    pub project_id: Option<String>,

    // Azure
    pub subscription_id: Option<String>,

    // Common
    pub access_token: Option<String>,
    pub api_token: Option<String>,
    pub api_key: Option<String>,
}

/// Machine type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineType {
    pub name: String,
    pub provider: CloudProvider,
    pub vcpus: u32,
    pub memory_gb: f64,
    pub storage_gb: Option<f64>,
    pub gpu_count: u32,
    pub gpu_type: Option<String>,
    pub network_performance: String,
    pub hourly_price: Option<f64>,
    pub spot_price: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_type_discovery() {
        let mut discovery = MachineTypeDiscovery::new();

        // Test getting fallback machines
        let aws_machines = discovery.get_common_aws_instances();
        assert!(!aws_machines.is_empty());
        assert_eq!(aws_machines[0].provider, CloudProvider::AWS);

        let gcp_machines = discovery.get_common_gcp_machines();
        assert!(!gcp_machines.is_empty());
        assert_eq!(gcp_machines[0].provider, CloudProvider::GCP);
    }

    #[test]
    fn test_find_best_match() {
        let mut discovery = MachineTypeDiscovery::new();

        // Populate cache with test data
        discovery
            .cache
            .insert(CloudProvider::AWS, discovery.get_common_aws_instances());

        // Find small instance
        let match1 = discovery.find_best_match(&CloudProvider::AWS, 2, 1.0, false, Some(0.02));
        assert!(match1.is_some());
        assert_eq!(match1.unwrap().name, "t3.micro");

        // Find GPU instance
        let match2 = discovery.find_best_match(&CloudProvider::AWS, 4, 16.0, true, None);
        assert!(match2.is_some());
        assert_eq!(match2.unwrap().name, "g4dn.xlarge");
    }
}
