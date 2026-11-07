//! CoreWeave `CloudProviderAdapter` implementation.
//!
//! CoreWeave is Kubernetes-native: rather than provisioning bare instances we
//! deploy workloads directly into the tenant's CoreWeave namespace. Raw VM
//! lifecycle calls are unsupported; Kubernetes deployment uses the caller's
//! configured Kubernetes access through the generic K8s path.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::ProvisionedInstance;
use crate::providers::common::gpu_adapter::build_http_client;
use crate::security::ApiAuthentication;
use async_trait::async_trait;
use blueprint_std::collections::HashMap;

pub struct CoreWeaveAdapter {
    #[cfg(feature = "kubernetes")]
    namespace: String,
}

impl CoreWeaveAdapter {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            #[cfg(feature = "kubernetes")]
            namespace: std::env::var("COREWEAVE_NAMESPACE")
                .unwrap_or_else(|_| "tenant-blueprint".to_string()),
        })
    }
}

#[async_trait]
impl CloudProviderAdapter for CoreWeaveAdapter {
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
            // CoreWeave is a Kubernetes-backed adapter and holds no pooled HTTP
            // client; build a short-lived one for the occasional health probe.
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
}
