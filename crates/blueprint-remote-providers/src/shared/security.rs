//! Shared security configuration across cloud providers
//!
//! This module provides unified security group/firewall abstractions
//! that work consistently across all cloud providers.

use crate::core::error::{Error, Result};
use blueprint_core::info;

/// Standard Blueprint security configuration
#[derive(Debug, Clone)]
pub struct BlueprintSecurityConfig {
    pub ssh_access: bool,
    pub qos_ports: bool,
    pub https_outbound: bool,
    pub custom_rules: Vec<SecurityRule>,
}

impl Default for BlueprintSecurityConfig {
    fn default() -> Self {
        Self {
            ssh_access: true,
            qos_ports: true,
            https_outbound: true,
            custom_rules: Vec::new(),
        }
    }
}

/// Generic security rule that can be translated to any provider
#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub name: String,
    pub direction: Direction,
    pub protocol: Protocol,
    pub ports: Vec<u16>,
    pub source_cidrs: Vec<String>,
    pub destination_cidrs: Vec<String>,
    pub priority: u16,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Ingress,
    Egress,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
}

impl BlueprintSecurityConfig {
    /// Get standard Blueprint security rules
    pub fn standard_rules(&self) -> Vec<SecurityRule> {
        let mut rules = Vec::new();

        if self.ssh_access {
            rules.push(SecurityRule {
                name: "blueprint-ssh".to_string(),
                direction: Direction::Ingress,
                protocol: Protocol::Tcp,
                ports: vec![22],
                source_cidrs: vec!["0.0.0.0/0".to_string()], // TODO: Restrict in production
                destination_cidrs: vec![],
                priority: 1000,
            });
        }

        if self.qos_ports {
            rules.push(SecurityRule {
                name: "blueprint-qos".to_string(),
                direction: Direction::Ingress,
                protocol: Protocol::Tcp,
                ports: vec![8080, 9615, 9944],
                source_cidrs: vec!["0.0.0.0/0".to_string()], // TODO: Restrict in production
                destination_cidrs: vec![],
                priority: 1000,
            });
        }

        if self.https_outbound {
            rules.push(SecurityRule {
                name: "blueprint-https-outbound".to_string(),
                direction: Direction::Egress,
                protocol: Protocol::Tcp,
                ports: vec![443, 80],
                source_cidrs: vec![],
                destination_cidrs: vec!["0.0.0.0/0".to_string()],
                priority: 1000,
            });
        }

        rules.extend(self.custom_rules.clone());
        rules
    }
}

/// Provider-specific security group manager
pub trait SecurityGroupManager {
    /// Create or update security group with Blueprint rules
    fn ensure_security_group(
        &self,
        name: &str,
        config: &BlueprintSecurityConfig,
    ) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Delete security group
    fn delete_security_group(&self, group_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Azure Network Security Group implementation
#[derive(Debug)]
pub struct AzureNsgManager {
    subscription_id: String,
    resource_group: String,
}

impl AzureNsgManager {
    pub fn new(subscription_id: String, resource_group: String) -> Self {
        Self {
            subscription_id,
            resource_group,
        }
    }
}

impl SecurityGroupManager for AzureNsgManager {
    fn ensure_security_group(
        &self,
        name: &str,
        config: &BlueprintSecurityConfig,
    ) -> impl std::future::Future<Output = Result<String>> + Send {
        let name = name.to_string();
        let config = config.clone();
        let subscription_id = self.subscription_id.clone();
        let resource_group = self.resource_group.clone();

        async move {
        let access_token = std::env::var("AZURE_ACCESS_TOKEN")
            .map_err(|_| Error::ConfigurationError("AZURE_ACCESS_TOKEN not set".into()))?;

        let client = reqwest::Client::new();
        let url = format!(
            "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/networkSecurityGroups/{name}?api-version=2023-09-01"
        );

        let rules = config.standard_rules();
        let mut security_rules = Vec::new();

        for (index, rule) in rules.iter().enumerate() {
            let direction = match rule.direction {
                Direction::Ingress => "Inbound",
                Direction::Egress => "Outbound",
            };

            let protocol = match rule.protocol {
                Protocol::Tcp => "Tcp",
                Protocol::Udp => "Udp",
                Protocol::Icmp => "Icmp",
            };

            let port_ranges = if rule.ports.len() == 1 {
                rule.ports[0].to_string()
            } else {
                rule.ports
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            };

            security_rules.push(serde_json::json!({
                "name": format!("{}-{}", rule.name, index),
                "properties": {
                    "protocol": protocol,
                    "sourcePortRange": "*",
                    "destinationPortRange": port_ranges,
                    "sourceAddressPrefix": rule.source_cidrs.first().unwrap_or(&"*".to_string()),
                    "destinationAddressPrefix": "*",
                    "access": "Allow",
                    "priority": rule.priority + index as u16,
                    "direction": direction
                }
            }));
        }

        let nsg_body = serde_json::json!({
            "location": "eastus",
            "properties": {
                "securityRules": security_rules
            }
        });

        match client
            .put(&url)
            .bearer_auth(&access_token)
            .json(&nsg_body)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                info!("Created Azure NSG: {}", name);
                Ok(name.to_string())
            }
            Ok(response) => {
                let error_text = response.text().await.unwrap_or_default();
                Err(Error::ConfigurationError(format!(
                    "Failed to create Azure NSG: {error_text}"
                )))
            }
            Err(e) => Err(Error::ConfigurationError(format!(
                "Failed to create Azure NSG: {e}"
            ))),
        }
        }
    }

    fn delete_security_group(&self, group_id: &str) -> impl std::future::Future<Output = Result<()>> + Send {
        let group_id = group_id.to_string();
        let subscription_id = self.subscription_id.clone();
        let resource_group = self.resource_group.clone();

        async move {
        let access_token = std::env::var("AZURE_ACCESS_TOKEN")
            .map_err(|_| Error::ConfigurationError("AZURE_ACCESS_TOKEN not set".into()))?;

        let client = reqwest::Client::new();
        let url = format!(
            "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/networkSecurityGroups/{group_id}?api-version=2023-09-01"
        );

        match client.delete(&url).bearer_auth(&access_token).send().await {
            Ok(response) if response.status().is_success() => {
                info!("Deleted Azure NSG: {}", group_id);
                Ok(())
            }
            Ok(_) => Ok(()), // NSG already deleted
            Err(e) => Err(Error::ConfigurationError(format!(
                "Failed to delete Azure NSG: {e}"
            ))),
        }
        }
    }
}

/// DigitalOcean Cloud Firewall implementation
#[derive(Debug)]
pub struct DigitalOceanFirewallManager {
    api_token: String,
}

impl DigitalOceanFirewallManager {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }
}

impl SecurityGroupManager for DigitalOceanFirewallManager {
    async fn ensure_security_group(
        &self,
        name: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<String> {
        let client = reqwest::Client::new();
        let url = "https://api.digitalocean.com/v2/firewalls";

        let rules = config.standard_rules();
        let mut inbound_rules = Vec::new();
        let mut outbound_rules = Vec::new();

        for rule in rules {
            let ports = rule
                .ports
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let protocol = match rule.protocol {
                Protocol::Tcp => "tcp",
                Protocol::Udp => "udp",
                Protocol::Icmp => "icmp",
            };

            let rule_json = serde_json::json!({
                "protocol": protocol,
                "ports": ports,
                "sources": {
                    "addresses": rule.source_cidrs
                }
            });

            match rule.direction {
                Direction::Ingress => inbound_rules.push(rule_json),
                Direction::Egress => {
                    let mut egress_rule = rule_json;
                    egress_rule["destinations"] =
                        serde_json::json!({"addresses": rule.destination_cidrs});
                    egress_rule.as_object_mut().unwrap().remove("sources");
                    outbound_rules.push(egress_rule);
                }
            }
        }

        let firewall_body = serde_json::json!({
            "name": name,
            "inbound_rules": inbound_rules,
            "outbound_rules": outbound_rules,
            "tags": ["blueprint"]
        });

        match client
            .post(url)
            .bearer_auth(&self.api_token)
            .json(&firewall_body)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                let json: serde_json::Value = response.json().await.map_err(|e| {
                    Error::ConfigurationError(format!("Failed to parse response: {e}"))
                })?;

                let firewall_id = json["firewall"]["id"].as_str().ok_or_else(|| {
                    Error::ConfigurationError("No firewall ID in response".into())
                })?;

                info!("Created DigitalOcean firewall: {} ({})", name, firewall_id);
                Ok(firewall_id.to_string())
            }
            Ok(response) => {
                let error_text = response.text().await.unwrap_or_default();
                Err(Error::ConfigurationError(format!(
                    "Failed to create DO firewall: {error_text}"
                )))
            }
            Err(e) => Err(Error::ConfigurationError(format!(
                "Failed to create DO firewall: {e}"
            ))),
        }
    }

    async fn delete_security_group(&self, group_id: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("https://api.digitalocean.com/v2/firewalls/{group_id}");

        match client
            .delete(&url)
            .bearer_auth(&self.api_token)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                info!("Deleted DigitalOcean firewall: {}", group_id);
                Ok(())
            }
            Ok(_) => Ok(()), // Firewall already deleted
            Err(e) => Err(Error::ConfigurationError(format!(
                "Failed to delete DO firewall: {e}"
            ))),
        }
    }
}

/// Vultr Firewall Group implementation
#[derive(Debug)]
pub struct VultrFirewallManager {
    api_key: String,
}

impl VultrFirewallManager {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl SecurityGroupManager for VultrFirewallManager {
    async fn ensure_security_group(
        &self,
        name: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<String> {
        let client = reqwest::Client::new();
        let url = "https://api.vultr.com/v2/firewalls";

        // First create the firewall group
        let firewall_body = serde_json::json!({
            "description": name
        });

        let response = client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&firewall_body)
            .send()
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to create Vultr firewall: {e}"))
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Failed to create Vultr firewall: {error_text}"
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse response: {e}")))?;

        let firewall_id = json["firewall_group"]["id"]
            .as_str()
            .ok_or_else(|| Error::ConfigurationError("No firewall ID in response".into()))?;

        // Add rules to the firewall group
        let rules = config.standard_rules();
        let rules_url = format!("https://api.vultr.com/v2/firewalls/{firewall_id}/rules");

        for rule in rules {
            let port_range = if rule.ports.len() == 1 {
                format!("{}", rule.ports[0])
            } else {
                format!(
                    "{}:{}",
                    rule.ports.iter().min().unwrap(),
                    rule.ports.iter().max().unwrap()
                )
            };

            let protocol = match rule.protocol {
                Protocol::Tcp => "tcp",
                Protocol::Udp => "udp",
                Protocol::Icmp => "icmp",
            };

            let action = match rule.direction {
                Direction::Ingress => "accept",
                Direction::Egress => "accept",
            };

            let rule_body = serde_json::json!({
                "ip_type": "v4",
                "protocol": protocol,
                "subnet": rule.source_cidrs.first().unwrap_or(&"0.0.0.0/0".to_string()),
                "subnet_size": 0,
                "port": port_range,
                "action": action
            });

            let _ = client
                .post(&rules_url)
                .bearer_auth(&self.api_key)
                .json(&rule_body)
                .send()
                .await; // Ignore individual rule failures
        }

        info!("Created Vultr firewall: {} ({})", name, firewall_id);
        Ok(firewall_id.to_string())
    }

    async fn delete_security_group(&self, group_id: &str) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("https://api.vultr.com/v2/firewalls/{group_id}");

        match client.delete(&url).bearer_auth(&self.api_key).send().await {
            Ok(response) if response.status().is_success() => {
                info!("Deleted Vultr firewall: {}", group_id);
                Ok(())
            }
            Ok(_) => Ok(()), // Firewall already deleted
            Err(e) => Err(Error::ConfigurationError(format!(
                "Failed to delete Vultr firewall: {e}"
            ))),
        }
    }
}
