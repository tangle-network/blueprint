//! Shared security configuration across cloud providers
//!
//! This module provides unified security group/firewall abstractions
//! that work consistently across all cloud providers.

use crate::core::error::{Error, Result};
use crate::create_default_provider_client;
use crate::security::auth;
use blueprint_core::info;
use url::Url;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Ingress,
    Egress,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
}

impl BlueprintSecurityConfig {
    /// Get standard Blueprint security rules with configurable source CIDRs
    ///
    /// Source CIDRs can be restricted via environment variables:
    /// - BLUEPRINT_ALLOWED_SSH_CIDRS: Comma-separated CIDRs for SSH access
    /// - BLUEPRINT_ALLOWED_QOS_CIDRS: Comma-separated CIDRs for QoS metrics access
    ///
    /// Default: 0.0.0.0/0 (all internet) for development convenience
    pub fn standard_rules(&self) -> Vec<SecurityRule> {
        let mut rules = Vec::new();

        // Get allowed source CIDRs from environment or use default
        let ssh_cidrs = Self::get_allowed_cidrs("BLUEPRINT_ALLOWED_SSH_CIDRS");
        let qos_cidrs = Self::get_allowed_cidrs("BLUEPRINT_ALLOWED_QOS_CIDRS");

        if self.ssh_access {
            rules.push(SecurityRule {
                name: "blueprint-ssh".to_string(),
                direction: Direction::Ingress,
                protocol: Protocol::Tcp,
                ports: vec![22],
                source_cidrs: ssh_cidrs,
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
                source_cidrs: qos_cidrs,
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

    /// Get allowed CIDRs from environment variable or default to 0.0.0.0/0
    fn get_allowed_cidrs(env_var: &str) -> Vec<String> {
        std::env::var(env_var)
            .ok()
            .map(|cidrs| {
                cidrs
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .filter(|v: &Vec<String>| !v.is_empty())
            .unwrap_or_else(|| vec!["0.0.0.0/0".to_string()])
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
    fn delete_security_group(
        &self,
        group_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
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
            let access_token = auth::azure_access_token().await?;

            let client = create_default_provider_client()?;
            let url = format!(
                "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/networkSecurityGroups/{name}?api-version=2023-09-01"
            );
            validate_management_url(&url)?;

            let rules = config.standard_rules();
            let mut security_rules = Vec::new();

            for (index, rule) in rules.iter().enumerate() {
                security_rules.push(build_azure_rule_payload(rule, index));
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

    fn delete_security_group(
        &self,
        group_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        let group_id = group_id.to_string();
        let subscription_id = self.subscription_id.clone();
        let resource_group = self.resource_group.clone();

        async move {
            let access_token = auth::azure_access_token().await?;

            let client = create_default_provider_client()?;
            let url = format!(
                "https://management.azure.com/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.Network/networkSecurityGroups/{group_id}?api-version=2023-09-01"
            );
            validate_management_url(&url)?;

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
        let client = create_default_provider_client()?;
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
                    if let Some(obj) = egress_rule.as_object_mut() {
                        obj.remove("sources");
                    }
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
        let client = create_default_provider_client()?;
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

fn validate_management_url(url: &str) -> Result<()> {
    let parsed =
        Url::parse(url).map_err(|e| Error::ConfigurationError(format!("Invalid URL: {e}")))?;
    if parsed.scheme() != "https" {
        return Err(Error::ConfigurationError(
            "Azure management URLs must use HTTPS".into(),
        ));
    }
    let host = parsed
        .host_str()
        .ok_or_else(|| Error::ConfigurationError("Missing host in URL".into()))?;
    if host != "management.azure.com" {
        return Err(Error::ConfigurationError(format!(
            "Unexpected Azure management host: {host}"
        )));
    }
    Ok(())
}

fn build_azure_rule_payload(rule: &SecurityRule, index: usize) -> serde_json::Value {
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

    let mut properties = serde_json::json!({
        "protocol": protocol,
        "sourcePortRange": "*",
        "destinationPortRange": port_ranges,
        "destinationAddressPrefix": "*",
        "access": "Allow",
        "priority": rule.priority + index as u16,
        "direction": direction
    });

    if rule.source_cidrs.len() <= 1 {
        let source = rule
            .source_cidrs
            .first()
            .cloned()
            .unwrap_or_else(|| "*".to_string());
        properties["sourceAddressPrefix"] = serde_json::json!(source);
    } else {
        properties["sourceAddressPrefixes"] = serde_json::json!(rule.source_cidrs.clone());
    }

    serde_json::json!({
        "name": format!("{}-{}", rule.name, index),
        "properties": properties
    })
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
        let client = create_default_provider_client()?;
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
                    rule.ports.iter().min().unwrap_or(&0),
                    rule.ports.iter().max().unwrap_or(&0)
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
        let client = create_default_provider_client()?;
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

#[cfg(test)]
mod tests {
    use super::{
        BlueprintSecurityConfig, Direction, Protocol, SecurityRule, build_azure_rule_payload,
        validate_management_url,
    };
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn validates_management_url() {
        assert!(validate_management_url("https://management.azure.com/").is_ok());
        assert!(validate_management_url("http://management.azure.com/").is_err());
        assert!(validate_management_url("https://example.com/").is_err());
    }

    #[test]
    fn standard_rules_use_configured_cidrs() {
        let _guard = env_lock();
        unsafe {
            std::env::set_var("BLUEPRINT_ALLOWED_SSH_CIDRS", "10.0.0.0/8,192.168.0.0/16");
            std::env::set_var("BLUEPRINT_ALLOWED_QOS_CIDRS", "172.16.0.0/12");
        }

        let config = BlueprintSecurityConfig::default();
        let rules = config.standard_rules();
        let ssh_rule = rules
            .iter()
            .find(|rule| rule.name == "blueprint-ssh")
            .unwrap();
        let qos_rule = rules
            .iter()
            .find(|rule| rule.name == "blueprint-qos")
            .unwrap();

        assert_eq!(
            ssh_rule.source_cidrs,
            vec!["10.0.0.0/8".to_string(), "192.168.0.0/16".to_string()]
        );
        assert_eq!(qos_rule.source_cidrs, vec!["172.16.0.0/12".to_string()]);

        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_SSH_CIDRS");
            std::env::remove_var("BLUEPRINT_ALLOWED_QOS_CIDRS");
        }
    }

    #[test]
    fn azure_rule_payload_uses_source_prefixes_for_multiple_cidrs() {
        let rule = SecurityRule {
            name: "blueprint-ssh".to_string(),
            direction: Direction::Ingress,
            protocol: Protocol::Tcp,
            ports: vec![22],
            source_cidrs: vec!["10.0.0.0/8".to_string(), "192.168.0.0/16".to_string()],
            destination_cidrs: vec![],
            priority: 1000,
        };

        let payload = build_azure_rule_payload(&rule, 0);
        let props = payload["properties"].as_object().unwrap();
        assert!(props.contains_key("sourceAddressPrefixes"));
        assert!(!props.contains_key("sourceAddressPrefix"));
    }

    #[test]
    fn azure_rule_payload_uses_single_prefix_for_one_cidr() {
        let rule = SecurityRule {
            name: "blueprint-ssh".to_string(),
            direction: Direction::Ingress,
            protocol: Protocol::Tcp,
            ports: vec![22],
            source_cidrs: vec!["10.0.0.0/8".to_string()],
            destination_cidrs: vec![],
            priority: 1000,
        };

        let payload = build_azure_rule_payload(&rule, 0);
        let props = payload["properties"].as_object().unwrap();
        assert!(props.contains_key("sourceAddressPrefix"));
        assert!(!props.contains_key("sourceAddressPrefixes"));
    }

    #[test]
    fn test_default_cidr_configuration() {
        let _guard = env_lock();
        // Without environment variables, should default to 0.0.0.0/0
        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_SSH_CIDRS");
            std::env::remove_var("BLUEPRINT_ALLOWED_QOS_CIDRS");
        }

        let config = BlueprintSecurityConfig::default();
        let rules = config.standard_rules();

        // Should have SSH, QoS, and HTTPS rules
        assert_eq!(rules.len(), 3);

        // SSH rule should have default CIDR
        let ssh_rule = rules.iter().find(|r| r.name == "blueprint-ssh").unwrap();
        assert_eq!(ssh_rule.source_cidrs, vec!["0.0.0.0/0"]);
        assert_eq!(ssh_rule.ports, vec![22]);
        assert!(matches!(ssh_rule.direction, Direction::Ingress));
        assert!(matches!(ssh_rule.protocol, Protocol::Tcp));

        // QoS rule should have default CIDR
        let qos_rule = rules.iter().find(|r| r.name == "blueprint-qos").unwrap();
        assert_eq!(qos_rule.source_cidrs, vec!["0.0.0.0/0"]);
        assert_eq!(qos_rule.ports, vec![8080, 9615, 9944]);
    }

    #[test]
    fn test_custom_ssh_cidr_configuration() {
        let _guard = env_lock();
        // Set custom SSH CIDR
        unsafe {
            std::env::set_var("BLUEPRINT_ALLOWED_SSH_CIDRS", "10.0.0.0/8");
            std::env::remove_var("BLUEPRINT_ALLOWED_QOS_CIDRS");
        }

        let config = BlueprintSecurityConfig::default();
        let rules = config.standard_rules();

        let ssh_rule = rules.iter().find(|r| r.name == "blueprint-ssh").unwrap();
        assert_eq!(ssh_rule.source_cidrs, vec!["10.0.0.0/8"]);

        // QoS should still use default
        let qos_rule = rules.iter().find(|r| r.name == "blueprint-qos").unwrap();
        assert_eq!(qos_rule.source_cidrs, vec!["0.0.0.0/0"]);

        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_SSH_CIDRS");
        }
    }

    #[test]
    fn test_multiple_cidrs_configuration() {
        let _guard = env_lock();
        // Set multiple CIDRs comma-separated
        unsafe {
            std::env::set_var(
                "BLUEPRINT_ALLOWED_SSH_CIDRS",
                "10.0.0.0/8, 192.168.1.0/24, 172.16.0.0/12",
            );
        }

        let config = BlueprintSecurityConfig::default();
        let rules = config.standard_rules();

        let ssh_rule = rules.iter().find(|r| r.name == "blueprint-ssh").unwrap();
        assert_eq!(
            ssh_rule.source_cidrs,
            vec!["10.0.0.0/8", "192.168.1.0/24", "172.16.0.0/12"]
        );

        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_SSH_CIDRS");
        }
    }

    #[test]
    fn test_empty_cidr_fallback() {
        let _guard = env_lock();
        // Empty string should fall back to default
        unsafe {
            std::env::set_var("BLUEPRINT_ALLOWED_SSH_CIDRS", "");
        }

        let config = BlueprintSecurityConfig::default();
        let rules = config.standard_rules();

        let ssh_rule = rules.iter().find(|r| r.name == "blueprint-ssh").unwrap();
        assert_eq!(ssh_rule.source_cidrs, vec!["0.0.0.0/0"]);

        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_SSH_CIDRS");
        }
    }

    #[test]
    fn test_whitespace_trimming() {
        let _guard = env_lock();
        // Should trim whitespace from CIDRs
        unsafe {
            std::env::set_var(
                "BLUEPRINT_ALLOWED_QOS_CIDRS",
                "  10.0.0.0/8  ,   192.168.1.0/24  ",
            );
        }

        let config = BlueprintSecurityConfig::default();
        let rules = config.standard_rules();

        let qos_rule = rules.iter().find(|r| r.name == "blueprint-qos").unwrap();
        assert_eq!(qos_rule.source_cidrs, vec!["10.0.0.0/8", "192.168.1.0/24"]);

        unsafe {
            std::env::remove_var("BLUEPRINT_ALLOWED_QOS_CIDRS");
        }
    }

    #[test]
    fn test_custom_rules() {
        let mut config = BlueprintSecurityConfig::default();
        config.custom_rules.push(SecurityRule {
            name: "custom-app".to_string(),
            direction: Direction::Ingress,
            protocol: Protocol::Tcp,
            ports: vec![3000],
            source_cidrs: vec!["192.168.1.0/24".to_string()],
            destination_cidrs: vec![],
            priority: 2000,
        });

        let rules = config.standard_rules();

        // Should have SSH, QoS, HTTPS, and custom rule
        assert_eq!(rules.len(), 4);

        let custom_rule = rules.iter().find(|r| r.name == "custom-app").unwrap();
        assert_eq!(custom_rule.ports, vec![3000]);
        assert_eq!(custom_rule.source_cidrs, vec!["192.168.1.0/24"]);
    }

    #[test]
    fn test_disabled_rules() {
        let config = BlueprintSecurityConfig {
            ssh_access: false,
            qos_ports: false,
            https_outbound: true,
            custom_rules: Vec::new(),
        };

        let rules = config.standard_rules();

        // Should only have HTTPS rule
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "blueprint-https-outbound");
        assert!(matches!(rules[0].direction, Direction::Egress));
    }
}
