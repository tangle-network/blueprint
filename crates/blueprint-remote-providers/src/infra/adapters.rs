//! Cloud provider adapter implementations for Blueprint infrastructure provisioning
//!
//! This module contains all the cloud provider-specific adapter implementations that enable
//! provisioning and management of infrastructure across different cloud providers including
//! AWS, GCP, Azure, DigitalOcean, and Vultr.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::infra::traits::CloudProviderAdapter;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// AWS adapter implementation
#[cfg(feature = "aws")]
pub struct AwsAdapter {
    client: aws_sdk_ec2::Client,
}

#[cfg(feature = "aws")]
impl AwsAdapter {
    pub async fn new() -> Result<Self> {
        let config = aws_config::load_from_env().await;
        Ok(Self {
            client: aws_sdk_ec2::Client::new(&config),
        })
    }

    async fn get_latest_ami(&self, _region: &str) -> Result<String> {
        // For production, query for the latest Amazon Linux 2023 AMI
        // For now, use a known good AMI ID
        Ok("ami-0c02fb55731490381".to_string()) // Amazon Linux 2023 in us-east-1
    }
}

#[cfg(feature = "aws")]
#[async_trait]
impl CloudProviderAdapter for AwsAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        // Use the latest Amazon Linux 2023 AMI
        let ami_id = self.get_latest_ami(region).await?;

        let result = self
            .client
            .run_instances()
            .image_id(ami_id)
            .instance_type(
                instance_type
                    .parse()
                    .map_err(|_| Error::Other("Invalid instance type".into()))?,
            )
            .min_count(1)
            .max_count(1)
            .tag_specifications(
                aws_sdk_ec2::types::TagSpecification::builder()
                    .resource_type(aws_sdk_ec2::types::ResourceType::Instance)
                    .tags(
                        aws_sdk_ec2::types::Tag::builder()
                            .key("ManagedBy")
                            .value("BlueprintManager")
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await
            .map_err(|e| Error::Other(format!("AWS provisioning failed: {}", e)))?;

        let instance = result
            .instances()
            .first()
            .ok_or_else(|| Error::Other("No instance created".into()))?;

        Ok(ProvisionedInstance {
            id: instance.instance_id().unwrap_or_default().to_string(),
            provider: CloudProvider::AWS,
            instance_type: instance_type.to_string(),
            region: region.to_string(),
            public_ip: None, // Will be assigned after launch
            private_ip: instance.private_ip_address().map(|s| s.to_string()),
            status: InstanceStatus::Starting,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.client
            .terminate_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(|e| Error::Other(format!("AWS termination failed: {}", e)))?;

        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let result = self
            .client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .map_err(|e| Error::Other(format!("AWS describe failed: {}", e)))?;

        let state = result
            .reservations()
            .first()
            .and_then(|r| r.instances().first())
            .and_then(|i| i.state())
            .and_then(|s| s.name())
            .ok_or_else(|| Error::Other("Instance not found".into()))?;

        Ok(match state {
            aws_sdk_ec2::types::InstanceStateName::Pending => InstanceStatus::Starting,
            aws_sdk_ec2::types::InstanceStateName::Running => InstanceStatus::Running,
            aws_sdk_ec2::types::InstanceStateName::Stopping => InstanceStatus::Stopping,
            aws_sdk_ec2::types::InstanceStateName::Stopped => InstanceStatus::Stopped,
            aws_sdk_ec2::types::InstanceStateName::Terminated => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        })
    }
}

/// GCP adapter implementation using REST API
pub struct GcpAdapter {
    client: reqwest::Client,
    project_id: String,
    access_token: Arc<RwLock<String>>,
}

impl GcpAdapter {
    pub async fn new() -> Result<Self> {
        let project_id = blueprint_std::env::var("GCP_PROJECT_ID")
            .map_err(|_| Error::Other("GCP_PROJECT_ID not set".into()))?;

        let client = reqwest::Client::new();
        let access_token = Arc::new(RwLock::new(String::new()));

        Ok(Self {
            client,
            project_id,
            access_token,
        })
    }

    async fn refresh_token(&self) -> Result<String> {
        let output = tokio::process::Command::new("gcloud")
            .args(&["auth", "print-access-token"])
            .output()
            .await
            .map_err(|e| Error::Other(format!("Failed to get GCP token: {}", e)))?;

        if !output.status.success() {
            return Err(Error::Other("Failed to get GCP access token".into()));
        }

        let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
        *self.access_token.write().await = token.clone();
        Ok(token)
    }
}

#[async_trait]
impl CloudProviderAdapter for GcpAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let token = self.refresh_token().await?;
        let zone = format!("{}-a", region); // Default to zone 'a'

        let instance_name = format!("blueprint-{}", uuid::Uuid::new_v4());

        let body = serde_json::json!({
            "name": instance_name,
            "machineType": format!("zones/{}/machineTypes/{}", zone, instance_type),
            "disks": [{
                "boot": true,
                "autoDelete": true,
                "initializeParams": {
                    "sourceImage": "projects/debian-cloud/global/images/family/debian-12",
                    "diskSizeGb": "10"
                }
            }],
            "networkInterfaces": [{
                "network": "global/networks/default",
                "accessConfigs": [{
                    "type": "ONE_TO_ONE_NAT",
                    "name": "External NAT"
                }]
            }],
            "labels": {
                "managed-by": "blueprint-manager"
            }
        });

        let response = self
            .client
            .post(format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances",
                self.project_id, zone
            ))
            .bearer_auth(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Other(format!("GCP request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Other(format!(
                "GCP provisioning failed: {}",
                error_text
            )));
        }

        Ok(ProvisionedInstance {
            id: instance_name,
            provider: CloudProvider::GCP,
            instance_type: instance_type.to_string(),
            region: region.to_string(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let token = self.refresh_token().await?;

        // Assume instance is in zone-a for simplicity
        // In production, we'd need to query to find the actual zone
        let zone = "us-central1-a";

        let response = self
            .client
            .delete(format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
                self.project_id, zone, instance_id
            ))
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| Error::Other(format!("GCP delete failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Other(format!(
                "GCP termination failed: {}",
                error_text
            )));
        }

        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let token = self.refresh_token().await?;
        let zone = "us-central1-a";

        let response = self
            .client
            .get(format!(
                "https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
                self.project_id, zone, instance_id
            ))
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| Error::Other(format!("GCP get failed: {}", e)))?;

        if response.status() == 404 {
            return Ok(InstanceStatus::Terminated);
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("GCP response parse failed: {}", e)))?;

        let status = data["status"].as_str().unwrap_or("UNKNOWN");

        Ok(match status {
            "PROVISIONING" | "STAGING" => InstanceStatus::Starting,
            "RUNNING" => InstanceStatus::Running,
            "STOPPING" => InstanceStatus::Stopping,
            "STOPPED" | "SUSPENDED" => InstanceStatus::Stopped,
            "TERMINATED" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        })
    }
}

/// Simplified Azure adapter
pub struct AzureAdapter;

impl AzureAdapter {
    pub async fn new() -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl CloudProviderAdapter for AzureAdapter {
    async fn provision_instance(
        &self,
        _instance_type: &str,
        _region: &str,
    ) -> Result<ProvisionedInstance> {
        Err(Error::Other(
            "Azure provisioning not yet implemented".into(),
        ))
    }

    async fn terminate_instance(&self, _instance_id: &str) -> Result<()> {
        Err(Error::Other("Azure termination not yet implemented".into()))
    }

    async fn get_instance_status(&self, _instance_id: &str) -> Result<InstanceStatus> {
        Err(Error::Other("Azure status not yet implemented".into()))
    }
}

/// Simplified DigitalOcean adapter
pub struct DigitalOceanAdapter {
    client: reqwest::Client,
    token: String,
}

impl DigitalOceanAdapter {
    pub fn new() -> Result<Self> {
        let token = blueprint_std::env::var("DIGITALOCEAN_TOKEN")
            .map_err(|_| Error::Other("DIGITALOCEAN_TOKEN not set".into()))?;

        Ok(Self {
            client: reqwest::Client::new(),
            token,
        })
    }
}

#[async_trait]
impl CloudProviderAdapter for DigitalOceanAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let body = serde_json::json!({
            "name": format!("blueprint-{}", uuid::Uuid::new_v4()),
            "region": region,
            "size": instance_type,
            "image": "ubuntu-22-04-x64",
            "tags": ["blueprint-manager"]
        });

        let response = self
            .client
            .post("https://api.digitalocean.com/v2/droplets")
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Other(format!("DO request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other("DigitalOcean provisioning failed".into()));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("DO response parse failed: {}", e)))?;

        let droplet = &data["droplet"];

        Ok(ProvisionedInstance {
            id: droplet["id"].as_u64().unwrap_or(0).to_string(),
            provider: CloudProvider::DigitalOcean,
            instance_type: instance_type.to_string(),
            region: region.to_string(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!(
                "https://api.digitalocean.com/v2/droplets/{}",
                instance_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| Error::Other(format!("DO delete failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other("DigitalOcean termination failed".into()));
        }

        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let response = self
            .client
            .get(format!(
                "https://api.digitalocean.com/v2/droplets/{}",
                instance_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| Error::Other(format!("DO get failed: {}", e)))?;

        if response.status() == 404 {
            return Ok(InstanceStatus::Terminated);
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("DO response parse failed: {}", e)))?;

        let status = data["droplet"]["status"].as_str().unwrap_or("unknown");

        Ok(match status {
            "new" => InstanceStatus::Starting,
            "active" => InstanceStatus::Running,
            "off" => InstanceStatus::Stopped,
            "archive" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        })
    }
}

/// Simplified Vultr adapter
pub struct VultrAdapter {
    client: reqwest::Client,
    api_key: String,
}

impl VultrAdapter {
    pub fn new() -> Result<Self> {
        let api_key = blueprint_std::env::var("VULTR_API_KEY")
            .map_err(|_| Error::Other("VULTR_API_KEY not set".into()))?;

        Ok(Self {
            client: reqwest::Client::new(),
            api_key,
        })
    }
}

#[async_trait]
impl CloudProviderAdapter for VultrAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let body = serde_json::json!({
            "region": region,
            "plan": instance_type,
            "os_id": 1743, // Ubuntu 22.04
            "label": format!("blueprint-{}", uuid::Uuid::new_v4()),
            "tags": ["blueprint-manager"]
        });

        let response = self
            .client
            .post("https://api.vultr.com/v2/instances")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Vultr request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other("Vultr provisioning failed".into()));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Vultr response parse failed: {}", e)))?;

        let instance = &data["instance"];

        Ok(ProvisionedInstance {
            id: instance["id"].as_str().unwrap_or("").to_string(),
            provider: CloudProvider::Vultr,
            instance_type: instance_type.to_string(),
            region: region.to_string(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let response = self
            .client
            .delete(format!(
                "https://api.vultr.com/v2/instances/{}",
                instance_id
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::Other(format!("Vultr delete failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other("Vultr termination failed".into()));
        }

        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let response = self
            .client
            .get(format!(
                "https://api.vultr.com/v2/instances/{}",
                instance_id
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::Other(format!("Vultr get failed: {}", e)))?;

        if response.status() == 404 {
            return Ok(InstanceStatus::Terminated);
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Vultr response parse failed: {}", e)))?;

        let status = data["instance"]["status"].as_str().unwrap_or("unknown");
        let power = data["instance"]["power_status"]
            .as_str()
            .unwrap_or("unknown");

        Ok(match (status, power) {
            ("pending", _) => InstanceStatus::Starting,
            ("active", "running") => InstanceStatus::Running,
            ("active", "stopped") => InstanceStatus::Stopped,
            _ => InstanceStatus::Unknown,
        })
    }
}