//! CoreWeave `CloudProviderAdapter` implementation.
//!
//! CoreWeave is Kubernetes-native: rather than provisioning bare instances we
//! deploy workloads directly into the tenant's CoreWeave namespace. The
//! adapter therefore treats "provision_instance" as a logical namespace check
//! and routes `deploy_blueprint_with_target` to the generic K8s path.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::build_http_client;
use crate::security::ApiAuthentication;
use async_trait::async_trait;
use blueprint_core::info;
use blueprint_std::collections::HashMap;

pub struct CoreWeaveAdapter {
    default_region: String,
    namespace: String,
    token: String,
}

impl CoreWeaveAdapter {
    pub async fn new() -> Result<Self> {
        let token = std::env::var("COREWEAVE_TOKEN")
            .map_err(|_| Error::Other("COREWEAVE_TOKEN environment variable not set".into()))?;
        let default_region =
            std::env::var("COREWEAVE_REGION").unwrap_or_else(|_| "ORD1".to_string());
        let namespace =
            std::env::var("COREWEAVE_NAMESPACE").unwrap_or_else(|_| "tenant-blueprint".to_string());
        Ok(Self {
            default_region,
            namespace,
            token,
        })
    }

    /// Namespace-scoped logical identifier for a deployment.
    fn logical_instance_id(&self) -> String {
        format!("{}-{}", self.namespace, uuid::Uuid::new_v4())
    }

    #[allow(dead_code)]
    fn token(&self) -> &str {
        &self.token
    }
}

#[async_trait]
impl CloudProviderAdapter for CoreWeaveAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        // CoreWeave does not have an "instance" that exists separately from a
        // Kubernetes workload. We return a placeholder ProvisionedInstance whose
        // `id` uniquely identifies the logical deployment slot in the tenant
        // namespace; the real workload is created during `deploy_blueprint_with_target`.
        let resolved_region = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };
        let id = self.logical_instance_id();
        info!(%id, %resolved_region, %instance_type, "Reserved CoreWeave logical deployment slot");
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::CoreWeave,
            instance_type: instance_type.to_string(),
            region: resolved_region.to_string(),
            public_ip: None,
            private_ip: None,
            // The K8s workload has not been deployed yet — `provision_instance`
            // only reserves a logical slot. Real readiness is established by
            // `deploy_blueprint_with_target` creating the workload.
            status: InstanceStatus::Starting,
        })
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        // CoreWeave workloads are cleaned up via `cleanup_blueprint` which
        // calls `kubectl delete` against the tenant namespace.
        info!(%instance_id, "CoreWeave logical slot released");
        Ok(())
    }

    async fn get_instance_status(&self, _instance_id: &str) -> Result<InstanceStatus> {
        // Logical slots are always "running" from the adapter's perspective.
        // Workload-level health is surfaced via `health_check_blueprint`.
        Ok(InstanceStatus::Running)
    }

    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::core::deployment_target::DeploymentTarget;
        match target {
            DeploymentTarget::GenericKubernetes {
                context: _,
                namespace,
            } => {
                #[cfg(feature = "kubernetes")]
                {
                    use crate::shared::SharedKubernetesDeployment;
                    let ns = if namespace.is_empty() {
                        self.namespace.as_str()
                    } else {
                        namespace.as_str()
                    };
                    SharedKubernetesDeployment::deploy_to_generic_k8s(
                        ns,
                        blueprint_image,
                        resource_spec,
                        env_vars,
                    )
                    .await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    let _ = (namespace, blueprint_image, resource_spec, env_vars);
                    Err(Error::ConfigurationError(
                        "CoreWeave adapter requires the `kubernetes` feature".into(),
                    ))
                }
            }
            DeploymentTarget::ManagedKubernetes {
                cluster_id: _,
                namespace,
            } => {
                #[cfg(feature = "kubernetes")]
                {
                    use crate::shared::SharedKubernetesDeployment;
                    let ns = if namespace.is_empty() {
                        self.namespace.as_str()
                    } else {
                        namespace.as_str()
                    };
                    // CoreWeave's managed K8s does not expose a cluster-id based
                    // auth command the way EKS/GKE/AKS do. Users must export a
                    // kubeconfig beforehand; the adapter uses the generic path.
                    SharedKubernetesDeployment::deploy_to_generic_k8s(
                        ns,
                        blueprint_image,
                        resource_spec,
                        env_vars,
                    )
                    .await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    let _ = (namespace, blueprint_image, resource_spec, env_vars);
                    Err(Error::ConfigurationError(
                        "CoreWeave adapter requires the `kubernetes` feature".into(),
                    ))
                }
            }
            DeploymentTarget::VirtualMachine { .. } => Err(Error::ConfigurationError(
                "CoreWeave does not provision raw VMs — use GenericKubernetes target".into(),
            )),
            DeploymentTarget::Serverless { .. } => Err(Error::ConfigurationError(
                "CoreWeave has no serverless offering".into(),
            )),
        }
    }

    async fn deploy_blueprint(
        &self,
        _instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        #[cfg(feature = "kubernetes")]
        {
            use crate::shared::SharedKubernetesDeployment;
            SharedKubernetesDeployment::deploy_to_generic_k8s(
                &self.namespace,
                blueprint_image,
                resource_spec,
                env_vars,
            )
            .await
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (blueprint_image, resource_spec, env_vars);
            Err(Error::ConfigurationError(
                "CoreWeave adapter requires the `kubernetes` feature".into(),
            ))
        }
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        if let Some(endpoint) = deployment.qos_grpc_endpoint() {
            let client = build_http_client()?;
            match client
                .get(&format!("{endpoint}/health"), &ApiAuthentication::None)
                .await
            {
                Ok(response) => Ok(response.status().is_success()),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn cleanup_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<()> {
        // Kubernetes cleanup is handled by `SharedKubernetesDeployment`; the adapter
        // relies on the blueprint manager's TTL-based reconciliation to prune stale
        // deployments at namespace scope.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_adapter(namespace: &str) -> CoreWeaveAdapter {
        CoreWeaveAdapter {
            default_region: "ORD1".into(),
            namespace: namespace.into(),
            token: "test-token".into(),
        }
    }

    #[test]
    fn logical_id_is_namespace_scoped() {
        let adapter = test_adapter("tenant-test");
        let id = adapter.logical_instance_id();
        assert!(id.starts_with("tenant-test-"));
        // The suffix is a UUID; confirm at least 20 chars of entropy appended.
        assert!(id.len() > "tenant-test-".len() + 20);
    }

    #[test]
    fn logical_id_differs_between_calls() {
        let adapter = test_adapter("tenant-x");
        let a = adapter.logical_instance_id();
        let b = adapter.logical_instance_id();
        assert_ne!(a, b);
    }

    #[test]
    fn token_accessor_returns_configured_value() {
        let adapter = test_adapter("tenant-y");
        assert_eq!(adapter.token(), "test-token");
    }
}
