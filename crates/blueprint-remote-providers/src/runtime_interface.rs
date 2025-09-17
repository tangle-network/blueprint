//! Runtime interface for decoupled integration with Blueprint Manager
//!
//! This module provides traits that Blueprint Manager implements,
//! avoiding cyclic dependencies while enabling remote deployments.

use crate::error::Result;
use crate::resources::ResourceSpec;
use async_trait::async_trait;
use blueprint_std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Runtime provider interface that Blueprint Manager implements
#[async_trait]
pub trait RuntimeProvider: Send + Sync {
    /// Deploy a service using the specified runtime
    async fn deploy(&self, deployment: DeploymentRequest) -> Result<DeploymentHandle>;

    /// Check deployment status
    async fn status(&self, handle: &DeploymentHandle) -> Result<DeploymentStatus>;

    /// Stop a deployment
    async fn stop(&self, handle: &DeploymentHandle) -> Result<()>;

    /// Get deployment logs
    async fn logs(&self, handle: &DeploymentHandle, lines: usize) -> Result<Vec<String>>;
}

/// Request to deploy a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRequest {
    pub blueprint_id: u64,
    pub service_id: u64,
    pub deployment_type: DeploymentTarget,
    pub resources: ResourceSpec,
    pub image: Option<String>,
    pub binary_path: Option<PathBuf>,
    pub env_vars: Vec<(String, String)>,
    pub args: Vec<String>,
}

/// Deployment target type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentTarget {
    /// Local Kubernetes with optional Kata containers
    LocalKubernetes { kata: bool },
    /// Remote Kubernetes cluster
    RemoteKubernetes {
        kubeconfig: PathBuf,
        namespace: String,
        context: Option<String>,
    },
    /// Docker container
    Docker,
    /// Hypervisor VM
    Hypervisor,
    /// Native process
    Native,
    /// SSH to remote host
    Ssh { host: String, user: String },
}

/// Handle to a running deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentHandle {
    pub deployment_id: String,
    pub deployment_type: DeploymentTarget,
    pub resource_ids: Vec<(String, String)>,
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed(String),
}

/// Kubernetes runtime provider for remote clusters
#[cfg(feature = "kubernetes")]
pub struct KubernetesRuntimeProvider {
    client: kube::Client,
    namespace: String,
}

#[cfg(feature = "kubernetes")]
impl KubernetesRuntimeProvider {
    pub async fn new(
        kubeconfig_path: Option<&std::path::Path>,
        context: Option<&str>,
        namespace: String,
    ) -> Result<Self> {
        use kube::{Client, Config};

        let config = if let Some(path) = kubeconfig_path {
            let kubeconfig_yaml = tokio::fs::read_to_string(path).await?;
            let kubeconfig: kube::config::Kubeconfig = serde_yaml::from_str(&kubeconfig_yaml)?;

            let mut config_options = kube::config::KubeConfigOptions::default();
            if let Some(ctx) = context {
                config_options.context = Some(ctx.to_string());
            }

            Config::from_custom_kubeconfig(kubeconfig, &config_options).await?
        } else {
            Config::infer().await?
        };

        let client = Client::try_from(config)?;

        Ok(Self { client, namespace })
    }

    async fn deploy_pod(&self, request: &DeploymentRequest) -> Result<DeploymentHandle> {
        use k8s_openapi::api::core::v1::{Container, Pod, PodSpec};
        use kube::api::{Api, PostParams};
        use kube::core::ObjectMeta;

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);

        let container = Container {
            name: format!("blueprint-{}-{}", request.blueprint_id, request.service_id),
            image: request.image.clone(),
            env: Some(
                request
                    .env_vars
                    .iter()
                    .map(|(k, v)| k8s_openapi::api::core::v1::EnvVar {
                        name: k.clone(),
                        value: Some(v.clone()),
                        ..Default::default()
                    })
                    .collect(),
            ),
            args: Some(request.args.clone()),
            resources: Some(request.resources.to_k8s_resources()),
            ..Default::default()
        };

        let pod = Pod {
            metadata: ObjectMeta {
                name: Some(format!(
                    "blueprint-{}-{}",
                    request.blueprint_id, request.service_id
                )),
                namespace: Some(self.namespace.clone()),
                labels: Some(
                    [
                        ("blueprint-id".to_string(), request.blueprint_id.to_string()),
                        ("service-id".to_string(), request.service_id.to_string()),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: vec![container],
                ..Default::default()
            }),
            ..Default::default()
        };

        let created_pod = pod_api.create(&PostParams::default(), &pod).await?;
        let pod_name = created_pod.metadata.name.unwrap_or_default();

        Ok(DeploymentHandle {
            deployment_id: format!("{}/{}", self.namespace, pod_name),
            deployment_type: request.deployment_type.clone(),
            resource_ids: vec![
                ("namespace".to_string(), self.namespace.clone()),
                ("pod".to_string(), pod_name),
            ],
        })
    }
}

#[cfg(feature = "kubernetes")]
#[async_trait]
impl RuntimeProvider for KubernetesRuntimeProvider {
    async fn deploy(&self, request: DeploymentRequest) -> Result<DeploymentHandle> {
        self.deploy_pod(&request).await
    }

    async fn status(&self, handle: &DeploymentHandle) -> Result<DeploymentStatus> {
        use k8s_openapi::api::core::v1::Pod;
        use kube::api::Api;

        let pod_name = handle
            .resource_ids
            .iter()
            .find(|(k, _)| k == "pod")
            .map(|(_, v)| v.as_str())
            .ok_or_else(|| crate::error::Error::ConfigurationError("No pod name".into()))?;

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let pod = pod_api.get(pod_name).await?;

        let status = pod.status.and_then(|s| s.phase).unwrap_or_default();

        Ok(match status.as_str() {
            "Pending" => DeploymentStatus::Starting,
            "Running" => DeploymentStatus::Running,
            "Succeeded" => DeploymentStatus::Stopped,
            "Failed" => DeploymentStatus::Failed("Pod failed".into()),
            _ => DeploymentStatus::Failed(format!("Unknown status: {}", status)),
        })
    }

    async fn stop(&self, handle: &DeploymentHandle) -> Result<()> {
        use k8s_openapi::api::core::v1::Pod;
        use kube::api::{Api, DeleteParams};

        let pod_name = handle
            .resource_ids
            .iter()
            .find(|(k, _)| k == "pod")
            .map(|(_, v)| v.as_str())
            .ok_or_else(|| crate::error::Error::ConfigurationError("No pod name".into()))?;

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        pod_api.delete(pod_name, &DeleteParams::default()).await?;

        Ok(())
    }

    async fn logs(&self, handle: &DeploymentHandle, lines: usize) -> Result<Vec<String>> {
        use k8s_openapi::api::core::v1::Pod;
        use kube::api::{Api, LogParams};

        let pod_name = handle
            .resource_ids
            .iter()
            .find(|(k, _)| k == "pod")
            .map(|(_, v)| v.as_str())
            .ok_or_else(|| crate::error::Error::ConfigurationError("No pod name".into()))?;

        let pod_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);

        let log_params = LogParams {
            tail_lines: Some(lines as i64),
            ..Default::default()
        };

        let logs = pod_api.logs(pod_name, &log_params).await?;
        Ok(logs.lines().map(String::from).collect())
    }
}
