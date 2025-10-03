//! Shared Kubernetes deployment patterns across providers
//!
//! This module consolidates Kubernetes deployment logic that's
//! duplicated across all cloud provider adapters. Provides real
//! cluster authentication and provider-specific cluster setup.

#![cfg(feature = "kubernetes")]

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::kubernetes::KubernetesDeploymentClient;
use crate::infra::traits::BlueprintDeploymentResult;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use std::collections::HashMap;
use std::process::Command;
use tracing::{info, warn};

/// Shared Kubernetes deployment implementation
pub struct SharedKubernetesDeployment;

impl SharedKubernetesDeployment {
    /// Deploy to managed Kubernetes service (EKS/GKE/AKS/DOKS/VKE) with cluster authentication
    pub async fn deploy_to_managed_k8s(
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        provider_config: ManagedK8sConfig,
    ) -> Result<BlueprintDeploymentResult> {
        info!("Deploying to {} cluster: {}", provider_config.service_name, cluster_id);

        // Authenticate to the managed cluster
        Self::setup_cluster_authentication(cluster_id, &provider_config).await?;

        // Verify cluster connectivity
        Self::verify_cluster_health(cluster_id, &provider_config).await?;

        let k8s_client = KubernetesDeploymentClient::new(Some(namespace.to_string())).await?;
        let (deployment_id, exposed_ports) = k8s_client
            .deploy_blueprint("blueprint", blueprint_image, resource_spec, 1)
            .await?;

        let mut port_mappings = HashMap::new();
        for port in exposed_ports {
            port_mappings.insert(port, port);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), provider_config.provider_identifier.clone());
        metadata.insert("cluster_id".to_string(), cluster_id.to_string());
        metadata.insert("namespace".to_string(), namespace.to_string());

        // Add provider-specific metadata
        for (key, value) in provider_config.additional_metadata {
            metadata.insert(key, value);
        }

        let instance = ProvisionedInstance {
            id: format!("{}-{}", provider_config.instance_prefix, cluster_id),
            public_ip: None, // K8s service handles routing
            private_ip: None,
            status: InstanceStatus::Running,
            provider: provider_config.cloud_provider,
            region: provider_config.default_region,
            instance_type: format!("{}-cluster", provider_config.service_name),
        };

        Ok(BlueprintDeploymentResult {
            instance,
            blueprint_id: deployment_id,
            port_mappings,
            metadata,
        })
    }

    /// Setup authentication to managed Kubernetes cluster
    async fn setup_cluster_authentication(cluster_id: &str, config: &ManagedK8sConfig) -> Result<()> {
        info!("Setting up {} cluster authentication for: {}", config.service_name, cluster_id);

        match config.cloud_provider {
            crate::core::remote::CloudProvider::AWS => {
                Self::setup_eks_auth(cluster_id, &config.default_region).await
            }
            crate::core::remote::CloudProvider::GCP => {
                Self::setup_gke_auth(cluster_id, &config.default_region, &config.additional_metadata).await
            }
            crate::core::remote::CloudProvider::Azure => {
                Self::setup_aks_auth(cluster_id, &config.additional_metadata).await
            }
            crate::core::remote::CloudProvider::DigitalOcean => {
                Self::setup_doks_auth(cluster_id).await
            }
            crate::core::remote::CloudProvider::Vultr => {
                Self::setup_vke_auth(cluster_id).await
            }
            _ => {
                warn!("No specific cluster authentication setup for provider: {:?}", config.cloud_provider);
                Ok(())
            }
        }
    }

    /// Setup AWS EKS cluster authentication
    async fn setup_eks_auth(cluster_id: &str, region: &str) -> Result<()> {
        info!("Configuring EKS cluster {} in region {}", cluster_id, region);

        let output = Command::new("aws")
            .args(&["eks", "update-kubeconfig", "--region", region, "--name", cluster_id])
            .output()
            .map_err(|e| Error::ConfigurationError(format!("Failed to run aws eks update-kubeconfig: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!("AWS EKS kubeconfig update failed: {}", stderr)));
        }

        info!("EKS cluster {} authentication configured successfully", cluster_id);
        Ok(())
    }

    /// Setup GCP GKE cluster authentication
    async fn setup_gke_auth(cluster_id: &str, region: &str, metadata: &HashMap<String, String>) -> Result<()> {
        let project_id = metadata.get("project_id")
            .ok_or_else(|| Error::ConfigurationError("GKE requires project_id in metadata".into()))?;

        info!("Configuring GKE cluster {} in project {} region {}", cluster_id, project_id, region);

        let output = Command::new("gcloud")
            .args(&[
                "container", "clusters", "get-credentials", cluster_id,
                "--region", region,
                "--project", project_id
            ])
            .output()
            .map_err(|e| Error::ConfigurationError(format!("Failed to run gcloud get-credentials: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!("GCP GKE kubeconfig update failed: {}", stderr)));
        }

        info!("GKE cluster {} authentication configured successfully", cluster_id);
        Ok(())
    }

    /// Setup Azure AKS cluster authentication
    async fn setup_aks_auth(cluster_id: &str, metadata: &HashMap<String, String>) -> Result<()> {
        let resource_group = metadata.get("resource_group")
            .ok_or_else(|| Error::ConfigurationError("AKS requires resource_group in metadata".into()))?;

        info!("Configuring AKS cluster {} in resource group {}", cluster_id, resource_group);

        let output = Command::new("az")
            .args(&[
                "aks", "get-credentials",
                "--resource-group", resource_group,
                "--name", cluster_id
            ])
            .output()
            .map_err(|e| Error::ConfigurationError(format!("Failed to run az aks get-credentials: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!("Azure AKS kubeconfig update failed: {}", stderr)));
        }

        info!("AKS cluster {} authentication configured successfully", cluster_id);
        Ok(())
    }

    /// Setup DigitalOcean DOKS cluster authentication
    async fn setup_doks_auth(cluster_id: &str) -> Result<()> {
        info!("Configuring DOKS cluster {}", cluster_id);

        let output = Command::new("doctl")
            .args(&["kubernetes", "cluster", "kubeconfig", "save", cluster_id])
            .output()
            .map_err(|e| Error::ConfigurationError(format!("Failed to run doctl kubeconfig save: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!("DigitalOcean DOKS kubeconfig update failed: {}", stderr)));
        }

        info!("DOKS cluster {} authentication configured successfully", cluster_id);
        Ok(())
    }

    /// Setup Vultr VKE cluster authentication
    async fn setup_vke_auth(cluster_id: &str) -> Result<()> {
        info!("Configuring VKE cluster {}", cluster_id);

        // Note: vultr-cli doesn't have direct kubeconfig download, would need API call
        warn!("VKE cluster authentication requires manual kubeconfig setup for cluster {}", cluster_id);

        // For now, assume kubeconfig is already configured
        // In production, would make Vultr API call to get kubeconfig
        Ok(())
    }

    /// Verify cluster health before deployment
    async fn verify_cluster_health(cluster_id: &str, config: &ManagedK8sConfig) -> Result<()> {
        info!("Verifying {} cluster health: {}", config.service_name, cluster_id);

        let output = Command::new("kubectl")
            .args(&["cluster-info", "--request-timeout=10s"])
            .output()
            .map_err(|e| Error::ConfigurationError(format!("Failed to run kubectl cluster-info: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!("Cluster {} health check failed: {}", cluster_id, stderr)));
        }

        info!("Cluster {} is healthy and ready for deployment", cluster_id);
        Ok(())
    }

    /// Deploy to generic Kubernetes cluster
    pub async fn deploy_to_generic_k8s(
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
    ) -> Result<BlueprintDeploymentResult> {
        info!("Deploying to generic Kubernetes namespace: {}", namespace);

        let k8s_client = KubernetesDeploymentClient::new(Some(namespace.to_string())).await?;
        let (deployment_id, exposed_ports) = k8s_client
            .deploy_blueprint("blueprint", blueprint_image, resource_spec, 1)
            .await?;

        let mut port_mappings = HashMap::new();
        for port in exposed_ports {
            port_mappings.insert(port, port);
        }

        let mut metadata = HashMap::new();
        metadata.insert("provider".to_string(), "generic-k8s".to_string());
        metadata.insert("namespace".to_string(), namespace.to_string());

        let instance = ProvisionedInstance {
            id: format!("k8s-{}", namespace),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Running,
            provider: crate::core::remote::CloudProvider::Generic,
            region: "generic".to_string(),
            instance_type: "kubernetes-cluster".to_string(),
        };

        Ok(BlueprintDeploymentResult {
            instance,
            blueprint_id: deployment_id,
            port_mappings,
            metadata,
        })
    }
}

/// Configuration for managed Kubernetes services
pub struct ManagedK8sConfig {
    pub service_name: &'static str,
    pub provider_identifier: String,
    pub instance_prefix: &'static str,
    pub cloud_provider: crate::core::remote::CloudProvider,
    pub default_region: String,
    pub additional_metadata: HashMap<String, String>,
}

impl ManagedK8sConfig {
    /// AWS EKS configuration
    pub fn eks(region: &str) -> Self {
        Self {
            service_name: "EKS",
            provider_identifier: "aws-eks".to_string(),
            instance_prefix: "eks",
            cloud_provider: crate::core::remote::CloudProvider::AWS,
            default_region: region.to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// GCP GKE configuration
    pub fn gke(project_id: &str, region: &str) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("project_id".to_string(), project_id.to_string());

        Self {
            service_name: "GKE",
            provider_identifier: "gcp-gke".to_string(),
            instance_prefix: "gke",
            cloud_provider: crate::core::remote::CloudProvider::GCP,
            default_region: region.to_string(),
            additional_metadata: metadata,
        }
    }

    /// Azure AKS configuration
    pub fn aks(region: &str, resource_group: &str) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("resource_group".to_string(), resource_group.to_string());

        Self {
            service_name: "AKS",
            provider_identifier: "azure-aks".to_string(),
            instance_prefix: "aks",
            cloud_provider: crate::core::remote::CloudProvider::Azure,
            default_region: region.to_string(),
            additional_metadata: metadata,
        }
    }

    /// DigitalOcean DOKS configuration
    pub fn doks(region: &str) -> Self {
        Self {
            service_name: "DOKS",
            provider_identifier: "digitalocean-doks".to_string(),
            instance_prefix: "doks",
            cloud_provider: crate::core::remote::CloudProvider::DigitalOcean,
            default_region: region.to_string(),
            additional_metadata: HashMap::new(),
        }
    }

    /// Vultr VKE configuration
    pub fn vke(region: &str) -> Self {
        Self {
            service_name: "VKE",
            provider_identifier: "vultr-vke".to_string(),
            instance_prefix: "vke",
            cloud_provider: crate::core::remote::CloudProvider::Vultr,
            default_region: region.to_string(),
            additional_metadata: HashMap::new(),
        }
    }
}