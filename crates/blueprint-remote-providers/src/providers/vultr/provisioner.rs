//! Vultr instance provisioning

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use blueprint_core::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vultr API instance representation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VultrInstance {
    id: String,
    main_ip: String,
    v6_main_ip: Option<String>,
    internal_ip: Option<String>,
    hostname: String,
    os: String,
    region: String,
    plan: String,
    status: String,
    power_status: String,
}

/// Vultr API provisioner
pub struct VultrProvisioner {
    api_key: String,
    client: reqwest::Client,
}

impl VultrProvisioner {
    /// Create a new Vultr provisioner
    pub async fn new(api_key: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::ConfigurationError(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self { api_key, client })
    }

    /// Provision a Vultr instance
    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        let plan = self.select_plan(spec);
        let region = if config.region.is_empty() {
            "ewr" // Newark default
        } else {
            &config.region
        };

        info!(
            "Provisioning Vultr instance with plan {} in region {}",
            plan, region
        );

        // Create instance via Vultr API
        let create_payload = serde_json::json!({
            "region": region,
            "plan": plan,
            "label": config.name,
            "hostname": config.name,
            "os_id": 1743, // Ubuntu 22.04 LTS
            "backups": "disabled",
            "enable_ipv6": false,
            "ddos_protection": false,
            "activation_email": false,
            "ssh_keys": config.ssh_key_name.as_ref().map(|k| vec![k]).unwrap_or_default(),
            "user_data": self.generate_user_data(spec),
        });

        let response = self
            .client
            .post("https://api.vultr.com/v2/instances")
            .bearer_auth(&self.api_key)
            .json(&create_payload)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create instance: {e}")))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Vultr API error: {error_text}"
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {e}")))?;

        let instance_id = response_json["instance"]["id"]
            .as_str()
            .ok_or_else(|| Error::ConfigurationError("No instance ID in response".into()))?
            .to_string();

        // Wait for instance to be active
        let instance = self.wait_for_instance(&instance_id).await?;

        let mut metadata = HashMap::new();
        metadata.insert("plan".to_string(), plan.to_string());
        metadata.insert("os".to_string(), instance.os);
        metadata.insert("hostname".to_string(), instance.hostname);

        Ok(ProvisionedInfrastructure {
            provider: crate::core::remote::CloudProvider::Vultr,
            instance_id,
            public_ip: Some(instance.main_ip),
            private_ip: instance.internal_ip,
            region: instance.region,
            instance_type: plan.to_string(),
            metadata,
        })
    }

    /// Wait for instance to become active
    async fn wait_for_instance(&self, instance_id: &str) -> Result<VultrInstance> {
        let mut attempts = 0;
        let max_attempts = 60;

        loop {
            if attempts >= max_attempts {
                return Err(Error::ConfigurationError(
                    "Instance provisioning timeout".into(),
                ));
            }

            let instance = self.get_instance(instance_id).await?;

            if instance.status == "active" && instance.power_status == "running" {
                info!("Vultr instance {} is active", instance_id);
                return Ok(instance);
            }

            debug!(
                "Instance {} status: {}, power: {}",
                instance_id, instance.status, instance.power_status
            );

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }

    /// Get instance details
    async fn get_instance(&self, instance_id: &str) -> Result<VultrInstance> {
        let url = format!("https://api.vultr.com/v2/instances/{instance_id}");

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to get instance: {e}")))?;

        if !response.status().is_success() {
            return Err(Error::ConfigurationError(
                "Failed to get instance details".into(),
            ));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {e}")))?;

        let instance = &json["instance"];

        Ok(VultrInstance {
            id: instance["id"].as_str().unwrap_or("").to_string(),
            main_ip: instance["main_ip"].as_str().unwrap_or("").to_string(),
            v6_main_ip: instance["v6_main_ip"].as_str().map(|s| s.to_string()),
            internal_ip: instance["internal_ip"].as_str().map(|s| s.to_string()),
            hostname: instance["hostname"].as_str().unwrap_or("").to_string(),
            os: instance["os"].as_str().unwrap_or("").to_string(),
            region: instance["region"].as_str().unwrap_or("").to_string(),
            plan: instance["plan"].as_str().unwrap_or("").to_string(),
            status: instance["status"].as_str().unwrap_or("").to_string(),
            power_status: instance["power_status"].as_str().unwrap_or("").to_string(),
        })
    }

    /// Terminate instance
    pub async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("https://api.vultr.com/v2/instances/{instance_id}");

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to terminate instance: {e}")))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Failed to terminate: {error_text}"
            )));
        }

        info!("Terminated Vultr instance: {}", instance_id);
        Ok(())
    }

    /// Get instance status
    pub async fn get_instance_status(
        &self,
        instance_id: &str,
    ) -> Result<crate::infra::types::InstanceStatus> {
        match self.get_instance(instance_id).await {
            Ok(instance) => match (instance.status.as_str(), instance.power_status.as_str()) {
                ("active", "running") => Ok(crate::infra::types::InstanceStatus::Running),
                ("active", "stopped") => Ok(crate::infra::types::InstanceStatus::Stopped),
                ("pending", _) => Ok(crate::infra::types::InstanceStatus::Starting),
                _ => Ok(crate::infra::types::InstanceStatus::Unknown),
            },
            Err(_) => Ok(crate::infra::types::InstanceStatus::Terminated),
        }
    }

    /// Select Vultr plan based on resource requirements
    fn select_plan(&self, spec: &ResourceSpec) -> &'static str {
        // Vultr plan IDs (vc2 = regular cloud compute, vhf = high frequency)
        match (spec.cpu, spec.memory_gb, spec.gpu_count) {
            // GPU instances not available on Vultr
            (_, _, Some(_)) => "vc2-8c-32gb", // Largest available

            // High memory
            (cpu, mem, _) if mem > cpu * 4.0 => {
                if mem <= 2.0 {
                    "vc2-1c-2gb"
                } else if mem <= 4.0 {
                    "vc2-2c-4gb"
                } else if mem <= 8.0 {
                    "vc2-4c-8gb"
                } else if mem <= 16.0 {
                    "vc2-6c-16gb"
                } else {
                    "vc2-8c-32gb"
                }
            }

            // High CPU
            (cpu, _, _) if cpu >= 6.0 => "vhf-6c-24gb",
            (cpu, _, _) if cpu >= 4.0 => "vhf-4c-16gb",
            (cpu, _, _) if cpu >= 2.0 => "vhf-2c-8gb",

            // Standard
            (cpu, mem, _) if cpu <= 1.0 && mem <= 1.0 => "vc2-1c-1gb",
            (cpu, mem, _) if cpu <= 1.0 && mem <= 2.0 => "vc2-1c-2gb",
            (cpu, mem, _) if cpu <= 2.0 && mem <= 4.0 => "vc2-2c-4gb",
            _ => "vc2-2c-4gb",
        }
    }

    /// Generate cloud-init user data
    fn generate_user_data(&self, _spec: &ResourceSpec) -> String {
        // Base64 encoded cloud-init script
        let script = r#"#!/bin/bash
# Update system
apt-get update
apt-get upgrade -y

# Install Docker
curl -fsSL https://get.docker.com | sh
systemctl enable docker
systemctl start docker

# Install monitoring tools
apt-get install -y htop iotop sysstat

# Configure firewall
ufw allow 22/tcp
ufw allow 8080/tcp
ufw allow 9615/tcp
ufw allow 9944/tcp
ufw --force enable
"#;

        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(script)
    }
}
