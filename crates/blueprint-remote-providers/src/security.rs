//! Shared security configuration for Blueprint Manager remote communication
//!
//! Provides consistent security rules across all cloud providers for
//! secure communication between Blueprint Manager auth proxy and remote instances.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Blueprint Manager communication ports and security requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintSecurityConfig {
    /// SSH port for deployment and management
    pub ssh_port: u16,
    /// Blueprint service port (HTTP/gRPC)
    pub service_port: u16,
    /// Health check port
    pub health_port: u16,
    /// Auth proxy bridge port
    pub bridge_port: u16,
    /// Allow these CIDR blocks to connect
    pub allowed_cidrs: Vec<String>,
    /// Require TLS for service communication
    pub require_tls: bool,
}

impl Default for BlueprintSecurityConfig {
    fn default() -> Self {
        Self {
            ssh_port: 22,
            service_port: 8080,
            health_port: 8081,
            bridge_port: 8082,
            allowed_cidrs: vec!["0.0.0.0/0".to_string()], // Open for initial deployment
            require_tls: true,
        }
    }
}

impl BlueprintSecurityConfig {
    /// Create secure configuration with specific manager IP
    pub fn with_manager_ip(manager_ip: &str) -> Self {
        Self {
            allowed_cidrs: vec![format!("{}/32", manager_ip)],
            ..Default::default()
        }
    }

    /// Create configuration for VPC deployment
    pub fn for_vpc(vpc_cidr: &str) -> Self {
        Self {
            allowed_cidrs: vec![vpc_cidr.to_string()],
            ..Default::default()
        }
    }
}

/// Provider-specific security rule implementation
#[async_trait::async_trait]
pub trait SecurityRuleProvider {
    /// Create security group/firewall rules for Blueprint communication
    async fn create_security_rules(
        &self,
        config: &BlueprintSecurityConfig,
        name: &str,
    ) -> Result<String>;

    /// Update existing security rules
    async fn update_security_rules(
        &self,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()>;

    /// Delete security rules
    async fn delete_security_rules(&self, rule_id: &str) -> Result<()>;
}

/// AWS Security Groups implementation
#[derive(Debug)]
pub struct AwsSecurityRules {
    pub region: String,
    pub vpc_id: Option<String>,
}

impl AwsSecurityRules {
    pub fn new(region: String, vpc_id: Option<String>) -> Self {
        Self { region, vpc_id }
    }

    /// Generate AWS security group rules JSON
    fn generate_ingress_rules(&self, config: &BlueprintSecurityConfig) -> serde_json::Value {
        let mut rules = Vec::new();

        // SSH access
        for cidr in &config.allowed_cidrs {
            rules.push(serde_json::json!({
                "IpProtocol": "tcp",
                "FromPort": config.ssh_port,
                "ToPort": config.ssh_port,
                "CidrIp": cidr
            }));
        }

        // Blueprint service ports
        for &port in &[config.service_port, config.health_port, config.bridge_port] {
            for cidr in &config.allowed_cidrs {
                rules.push(serde_json::json!({
                    "IpProtocol": "tcp",
                    "FromPort": port,
                    "ToPort": port,
                    "CidrIp": cidr
                }));
            }
        }

        serde_json::json!(rules)
    }
}

#[async_trait::async_trait]
impl SecurityRuleProvider for AwsSecurityRules {
    async fn create_security_rules(
        &self,
        config: &BlueprintSecurityConfig,
        name: &str,
    ) -> Result<String> {
        // AWS Security Group creation
        let group_config = serde_json::json!({
            "GroupName": name,
            "Description": "Blueprint Manager remote communication",
            "VpcId": self.vpc_id,
            "IngressRules": self.generate_ingress_rules(config)
        });

        // TODO: Implement actual AWS API call
        tracing::info!("Creating AWS security group: {}", name);
        tracing::debug!("Config: {}", group_config);

        Ok(format!(
            "sg-{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        ))
    }

    async fn update_security_rules(
        &self,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()> {
        let rules = self.generate_ingress_rules(config);
        tracing::info!("Updating AWS security group {}: {:?}", rule_id, rules);
        Ok(())
    }

    async fn delete_security_rules(&self, rule_id: &str) -> Result<()> {
        tracing::info!("Deleting AWS security group: {}", rule_id);
        Ok(())
    }
}

/// GCP Firewall Rules implementation
#[derive(Debug)]
pub struct GcpFirewallRules {
    pub project_id: String,
    pub network: String,
}

impl GcpFirewallRules {
    pub fn new(project_id: String, network: Option<String>) -> Self {
        Self {
            project_id,
            network: network.unwrap_or_else(|| "default".to_string()),
        }
    }

    fn generate_firewall_rules(&self, config: &BlueprintSecurityConfig) -> Vec<serde_json::Value> {
        let mut rules = Vec::new();

        // SSH rule
        rules.push(serde_json::json!({
            "name": "blueprint-ssh",
            "direction": "INGRESS",
            "sourceRanges": config.allowed_cidrs,
            "allowed": [{
                "IPProtocol": "tcp",
                "ports": [config.ssh_port.to_string()]
            }],
            "targetTags": ["blueprint"]
        }));

        // Service ports rule
        rules.push(serde_json::json!({
            "name": "blueprint-services",
            "direction": "INGRESS",
            "sourceRanges": config.allowed_cidrs,
            "allowed": [{
                "IPProtocol": "tcp",
                "ports": [
                    config.service_port.to_string(),
                    config.health_port.to_string(),
                    config.bridge_port.to_string()
                ]
            }],
            "targetTags": ["blueprint"]
        }));

        rules
    }
}

#[async_trait::async_trait]
impl SecurityRuleProvider for GcpFirewallRules {
    async fn create_security_rules(
        &self,
        config: &BlueprintSecurityConfig,
        name: &str,
    ) -> Result<String> {
        let rules = self.generate_firewall_rules(config);
        tracing::info!("Creating GCP firewall rules for {}: {:?}", name, rules);

        // TODO: Implement actual GCP API calls
        Ok(format!(
            "fw-{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        ))
    }

    async fn update_security_rules(
        &self,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()> {
        let rules = self.generate_firewall_rules(config);
        tracing::info!("Updating GCP firewall rules {}: {:?}", rule_id, rules);
        Ok(())
    }

    async fn delete_security_rules(&self, rule_id: &str) -> Result<()> {
        tracing::info!("Deleting GCP firewall rules: {}", rule_id);
        Ok(())
    }
}

/// DigitalOcean Firewall implementation
#[derive(Debug)]
pub struct DigitalOceanFirewall {
    pub api_token: String,
}

impl DigitalOceanFirewall {
    pub fn new(api_token: String) -> Self {
        Self { api_token }
    }

    fn generate_inbound_rules(&self, config: &BlueprintSecurityConfig) -> Vec<serde_json::Value> {
        let mut rules = Vec::new();

        // SSH rule
        rules.push(serde_json::json!({
            "protocol": "tcp",
            "ports": config.ssh_port.to_string(),
            "sources": {
                "addresses": config.allowed_cidrs
            }
        }));

        // Blueprint service rules
        for &port in &[config.service_port, config.health_port, config.bridge_port] {
            rules.push(serde_json::json!({
                "protocol": "tcp",
                "ports": port.to_string(),
                "sources": {
                    "addresses": config.allowed_cidrs
                }
            }));
        }

        rules
    }
}

#[async_trait::async_trait]
impl SecurityRuleProvider for DigitalOceanFirewall {
    async fn create_security_rules(
        &self,
        config: &BlueprintSecurityConfig,
        name: &str,
    ) -> Result<String> {
        let firewall_config = serde_json::json!({
            "name": name,
            "inbound_rules": self.generate_inbound_rules(config),
            "outbound_rules": [{
                "protocol": "tcp",
                "ports": "all",
                "destinations": {
                    "addresses": ["0.0.0.0/0", "::/0"]
                }
            }],
            "tags": ["blueprint"]
        });

        tracing::info!("Creating DigitalOcean firewall: {}", name);
        tracing::debug!("Config: {}", firewall_config);

        // TODO: Implement actual DO API call
        Ok(format!(
            "fw-{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        ))
    }

    async fn update_security_rules(
        &self,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()> {
        let rules = self.generate_inbound_rules(config);
        tracing::info!("Updating DigitalOcean firewall {}: {:?}", rule_id, rules);
        Ok(())
    }

    async fn delete_security_rules(&self, rule_id: &str) -> Result<()> {
        tracing::info!("Deleting DigitalOcean firewall: {}", rule_id);
        Ok(())
    }
}

/// Kubernetes NetworkPolicy implementation
#[derive(Debug)]
pub struct KubernetesNetworkPolicy {
    pub namespace: String,
}

impl KubernetesNetworkPolicy {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }

    fn generate_network_policy(&self, config: &BlueprintSecurityConfig) -> serde_json::Value {
        serde_json::json!({
            "apiVersion": "networking.k8s.io/v1",
            "kind": "NetworkPolicy",
            "metadata": {
                "name": "blueprint-communication",
                "namespace": self.namespace
            },
            "spec": {
                "podSelector": {
                    "matchLabels": {
                        "app": "blueprint"
                    }
                },
                "policyTypes": ["Ingress"],
                "ingress": [
                    {
                        "ports": [
                            {"protocol": "TCP", "port": config.service_port},
                            {"protocol": "TCP", "port": config.health_port},
                            {"protocol": "TCP", "port": config.bridge_port}
                        ],
                        "from": config.allowed_cidrs.iter().map(|cidr| {
                            serde_json::json!({"ipBlock": {"cidr": cidr}})
                        }).collect::<Vec<_>>()
                    }
                ]
            }
        })
    }
}

#[async_trait::async_trait]
impl SecurityRuleProvider for KubernetesNetworkPolicy {
    async fn create_security_rules(
        &self,
        config: &BlueprintSecurityConfig,
        _name: &str,
    ) -> Result<String> {
        let policy = self.generate_network_policy(config);
        tracing::info!(
            "Creating Kubernetes NetworkPolicy in {}: {}",
            self.namespace,
            policy
        );

        // TODO: Implement actual Kubernetes API call
        Ok(format!(
            "netpol-{}",
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        ))
    }

    async fn update_security_rules(
        &self,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()> {
        let policy = self.generate_network_policy(config);
        tracing::info!("Updating Kubernetes NetworkPolicy {}: {}", rule_id, policy);
        Ok(())
    }

    async fn delete_security_rules(&self, rule_id: &str) -> Result<()> {
        tracing::info!("Deleting Kubernetes NetworkPolicy: {}", rule_id);
        Ok(())
    }
}

/// Enum for different security rule providers
#[derive(Debug)]
pub enum SecurityProvider {
    Aws(AwsSecurityRules),
    Gcp(GcpFirewallRules),
    DigitalOcean(DigitalOceanFirewall),
    Kubernetes(KubernetesNetworkPolicy),
}

#[async_trait::async_trait]
impl SecurityRuleProvider for SecurityProvider {
    async fn create_security_rules(
        &self,
        config: &BlueprintSecurityConfig,
        name: &str,
    ) -> Result<String> {
        match self {
            SecurityProvider::Aws(provider) => provider.create_security_rules(config, name).await,
            SecurityProvider::Gcp(provider) => provider.create_security_rules(config, name).await,
            SecurityProvider::DigitalOcean(provider) => {
                provider.create_security_rules(config, name).await
            }
            SecurityProvider::Kubernetes(provider) => {
                provider.create_security_rules(config, name).await
            }
        }
    }

    async fn update_security_rules(
        &self,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()> {
        match self {
            SecurityProvider::Aws(provider) => {
                provider.update_security_rules(rule_id, config).await
            }
            SecurityProvider::Gcp(provider) => {
                provider.update_security_rules(rule_id, config).await
            }
            SecurityProvider::DigitalOcean(provider) => {
                provider.update_security_rules(rule_id, config).await
            }
            SecurityProvider::Kubernetes(provider) => {
                provider.update_security_rules(rule_id, config).await
            }
        }
    }

    async fn delete_security_rules(&self, rule_id: &str) -> Result<()> {
        match self {
            SecurityProvider::Aws(provider) => provider.delete_security_rules(rule_id).await,
            SecurityProvider::Gcp(provider) => provider.delete_security_rules(rule_id).await,
            SecurityProvider::DigitalOcean(provider) => {
                provider.delete_security_rules(rule_id).await
            }
            SecurityProvider::Kubernetes(provider) => provider.delete_security_rules(rule_id).await,
        }
    }
}

/// Security manager that provides consistent security across all providers
pub struct SecurityManager {
    providers: HashMap<CloudProvider, SecurityProvider>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register security provider for a cloud platform
    pub fn register_provider(
        &mut self,
        provider: CloudProvider,
        security_provider: SecurityProvider,
    ) {
        self.providers.insert(provider, security_provider);
    }

    /// Create security rules for a provider
    pub async fn create_security_rules(
        &self,
        provider: &CloudProvider,
        config: &BlueprintSecurityConfig,
        name: &str,
    ) -> Result<String> {
        let security_provider = self.providers.get(provider).ok_or_else(|| {
            Error::ConfigurationError(format!("No security provider for {:?}", provider))
        })?;

        security_provider.create_security_rules(config, name).await
    }

    /// Update security rules
    pub async fn update_security_rules(
        &self,
        provider: &CloudProvider,
        rule_id: &str,
        config: &BlueprintSecurityConfig,
    ) -> Result<()> {
        let security_provider = self.providers.get(provider).ok_or_else(|| {
            Error::ConfigurationError(format!("No security provider for {:?}", provider))
        })?;

        security_provider
            .update_security_rules(rule_id, config)
            .await
    }

    /// Delete security rules
    pub async fn delete_security_rules(
        &self,
        provider: &CloudProvider,
        rule_id: &str,
    ) -> Result<()> {
        let security_provider = self.providers.get(provider).ok_or_else(|| {
            Error::ConfigurationError(format!("No security provider for {:?}", provider))
        })?;

        security_provider.delete_security_rules(rule_id).await
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_config() {
        let config = BlueprintSecurityConfig::default();
        assert_eq!(config.ssh_port, 22);
        assert_eq!(config.service_port, 8080);
        assert!(config.require_tls);

        let vpc_config = BlueprintSecurityConfig::for_vpc("10.0.0.0/16");
        assert_eq!(vpc_config.allowed_cidrs, vec!["10.0.0.0/16"]);
    }

    #[tokio::test]
    async fn test_aws_security_rules() {
        let aws = AwsSecurityRules::new("us-east-1".to_string(), None);
        let config = BlueprintSecurityConfig::default();

        let rule_id = aws.create_security_rules(&config, "test-sg").await.unwrap();
        assert!(rule_id.starts_with("sg-"));

        aws.update_security_rules(&rule_id, &config).await.unwrap();
        aws.delete_security_rules(&rule_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_security_manager() {
        let mut manager = SecurityManager::new();

        let aws_provider =
            SecurityProvider::Aws(AwsSecurityRules::new("us-east-1".to_string(), None));
        manager.register_provider(CloudProvider::AWS, aws_provider);

        let config = BlueprintSecurityConfig::default();
        let rule_id = manager
            .create_security_rules(&CloudProvider::AWS, &config, "test")
            .await
            .unwrap();

        manager
            .update_security_rules(&CloudProvider::AWS, &rule_id, &config)
            .await
            .unwrap();
        manager
            .delete_security_rules(&CloudProvider::AWS, &rule_id)
            .await
            .unwrap();
    }
}
