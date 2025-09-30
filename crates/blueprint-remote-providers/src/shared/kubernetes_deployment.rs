//! Shared Kubernetes deployment patterns across providers
//!
//! This module consolidates Kubernetes deployment logic that's
//! duplicated across all cloud provider adapters.

#![cfg(feature = "kubernetes")]

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::kubernetes::KubernetesDeploymentClient;
use crate::infra::traits::BlueprintDeploymentResult;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use std::collections::HashMap;
use tracing::info;

/// Shared Kubernetes deployment implementation
pub struct SharedKubernetesDeployment;

impl SharedKubernetesDeployment {
    /// Deploy to managed Kubernetes service (EKS/GKE/AKS/DOKS/VKE)
    pub async fn deploy_to_managed_k8s(
        cluster_id: &str,
        namespace: &str,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        provider_config: ManagedK8sConfig,
    ) -> Result<BlueprintDeploymentResult> {
        info!("Deploying to {} cluster: {}", provider_config.service_name, cluster_id);

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
    pub fn aks(region: &str) -> Self {
        Self {
            service_name: "AKS",
            provider_identifier: "azure-aks".to_string(),
            instance_prefix: "aks",
            cloud_provider: crate::core::remote::CloudProvider::Azure,
            default_region: region.to_string(),
            additional_metadata: HashMap::new(),
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