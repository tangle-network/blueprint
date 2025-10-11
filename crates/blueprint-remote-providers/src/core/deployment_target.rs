//! Deployment target abstraction for cloud providers
//!
//! Defines where blueprints are deployed within a cloud provider's ecosystem

use serde::{Deserialize, Serialize};

/// Deployment target within a cloud provider
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentTarget {
    /// Deploy to virtual machines via SSH + Docker/Podman
    VirtualMachine {
        /// Container runtime to use
        runtime: ContainerRuntime,
    },

    /// Deploy to managed Kubernetes service
    ManagedKubernetes {
        /// Cluster identifier or name
        cluster_id: String,
        /// Kubernetes namespace
        namespace: String,
    },

    /// Deploy to existing generic Kubernetes cluster
    GenericKubernetes {
        /// Kubeconfig context name
        context: Option<String>,
        /// Kubernetes namespace
        namespace: String,
    },

    /// Deploy to serverless container platform
    Serverless {
        /// Platform-specific configuration
        config: std::collections::HashMap<String, String>,
    },
}

/// Container runtime for VM deployments
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerRuntime {
    Docker,
    Podman,
    Containerd,
}

impl Default for DeploymentTarget {
    fn default() -> Self {
        Self::VirtualMachine {
            runtime: ContainerRuntime::Docker,
        }
    }
}

impl DeploymentTarget {
    /// Check if this target requires VM provisioning
    pub fn requires_vm_provisioning(&self) -> bool {
        matches!(self, Self::VirtualMachine { .. })
    }

    /// Check if this target uses Kubernetes
    pub fn uses_kubernetes(&self) -> bool {
        matches!(
            self,
            Self::ManagedKubernetes { .. } | Self::GenericKubernetes { .. }
        )
    }

    /// Get the container runtime for VM targets
    pub fn container_runtime(&self) -> Option<&ContainerRuntime> {
        match self {
            Self::VirtualMachine { runtime } => Some(runtime),
            _ => None,
        }
    }

    /// Get Kubernetes namespace
    pub fn kubernetes_namespace(&self) -> Option<&str> {
        match self {
            Self::ManagedKubernetes { namespace, .. }
            | Self::GenericKubernetes { namespace, .. } => Some(namespace),
            _ => None,
        }
    }
}

/// Deployment configuration combining provider and target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Cloud provider
    pub provider: crate::core::remote::CloudProvider,
    /// Deployment target within the provider
    pub target: DeploymentTarget,
    /// Region or availability zone
    pub region: String,
    /// Additional configuration
    pub metadata: std::collections::HashMap<String, String>,
}

impl DeploymentConfig {
    /// Create VM deployment config
    pub fn vm(
        provider: crate::core::remote::CloudProvider,
        region: String,
        runtime: ContainerRuntime,
    ) -> Self {
        Self {
            provider,
            target: DeploymentTarget::VirtualMachine { runtime },
            region,
            metadata: Default::default(),
        }
    }

    /// Create managed Kubernetes deployment config
    pub fn managed_k8s(
        provider: crate::core::remote::CloudProvider,
        region: String,
        cluster_id: String,
        namespace: String,
    ) -> Self {
        Self {
            provider,
            target: DeploymentTarget::ManagedKubernetes {
                cluster_id,
                namespace,
            },
            region,
            metadata: Default::default(),
        }
    }

    /// Create generic Kubernetes deployment config
    pub fn generic_k8s(context: Option<String>, namespace: String) -> Self {
        Self {
            provider: crate::core::remote::CloudProvider::Generic,
            target: DeploymentTarget::GenericKubernetes { context, namespace },
            region: "generic".to_string(),
            metadata: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_target_properties() {
        let vm_target = DeploymentTarget::VirtualMachine {
            runtime: ContainerRuntime::Docker,
        };
        assert!(vm_target.requires_vm_provisioning());
        assert!(!vm_target.uses_kubernetes());

        let k8s_target = DeploymentTarget::ManagedKubernetes {
            cluster_id: "my-cluster".to_string(),
            namespace: "default".to_string(),
        };
        assert!(!k8s_target.requires_vm_provisioning());
        assert!(k8s_target.uses_kubernetes());
        assert_eq!(k8s_target.kubernetes_namespace(), Some("default"));
    }

    #[test]
    fn test_deployment_config_builders() {
        let aws_vm = DeploymentConfig::vm(
            crate::core::remote::CloudProvider::AWS,
            "us-east-1".to_string(),
            ContainerRuntime::Docker,
        );
        assert!(aws_vm.target.requires_vm_provisioning());

        let aws_eks = DeploymentConfig::managed_k8s(
            crate::core::remote::CloudProvider::AWS,
            "us-east-1".to_string(),
            "my-eks-cluster".to_string(),
            "blueprint-ns".to_string(),
        );
        assert!(aws_eks.target.uses_kubernetes());
    }
}
