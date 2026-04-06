//! io.net `CloudProviderAdapter` implementation.
//!
//! io.net rents GPU compute through *clusters* (Ray, Kubernetes, or BareMetal).
//! For blueprint deployment we model a cluster as a single-node provisioning unit
//! and SSH into the first node to run the blueprint container.
//!
//! All HTTP calls go through `retry_with_backoff` so transient 5xx / 429 / network
//! errors are retried with jittered exponential backoff. The full provision-then-
//! deploy flow is wrapped in `provision_with_cleanup` to guarantee the cluster is
//! terminated if SSH deployment fails (no orphaned billing).

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    ClassifiedError, ErrorClass, RetryPolicy, build_http_client, classify_response, deploy_via_ssh,
    generate_instance_name, poll_until, provision_with_cleanup, require_public_ip,
    retry_with_backoff,
};
use crate::providers::io_net::IoNetInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://api.io.net/v1";
const CLUSTER_READY_TIMEOUT: Duration = Duration::from_secs(600);
const CLUSTER_POLL_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_CLUSTER_TYPE: &str = "BareMetal";
const DEFAULT_DURATION_HOURS: u32 = 1;

/// Adapter for io.net (io.cloud) GPU clusters.
pub struct IoNetAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
    cluster_type: String,
    duration_hours: u32,
}

impl IoNetAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("IO_NET_API_KEY")
            .map_err(|_| Error::Other("IO_NET_API_KEY environment variable not set".into()))?;
        let default_region = std::env::var("IO_NET_REGION").unwrap_or_else(|_| "us".to_string());
        let cluster_type = std::env::var("IO_NET_CLUSTER_TYPE")
            .unwrap_or_else(|_| DEFAULT_CLUSTER_TYPE.to_string());
        let duration_hours = std::env::var("IO_NET_DURATION_HOURS")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(DEFAULT_DURATION_HOURS);

        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::io_net(api_key),
            default_region,
            cluster_type,
            duration_hours,
        })
    }

    /// Issue an HTTP request with retry/backoff and return the parsed JSON body.
    async fn request_json(
        &self,
        label: &'static str,
        method: reqwest::Method,
        url: String,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let policy = RetryPolicy::default();
        retry_with_backoff(label, &policy, |_attempt| {
            let method = method.clone();
            let url = url.clone();
            let body = body.clone();
            async move {
                let response = self
                    .http
                    .authenticated_request(method, &url, &self.auth, body)
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!("{label}: {e}")))
                    })?;

                if let Some(class) = classify_response(&response) {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    let err = Error::HttpError(format!("{label} HTTP {status}: {text}"));
                    return Err(match class {
                        ErrorClass::Permanent => ClassifiedError::permanent(err),
                        ErrorClass::Transient => ClassifiedError::transient(err),
                        ErrorClass::RateLimited { retry_after } => {
                            ClassifiedError::rate_limited(err, retry_after)
                        }
                    });
                }

                response.json::<serde_json::Value>().await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!("{label} parse: {e}")))
                })
            }
        })
        .await
    }

    /// Issue an HTTP request with retry/backoff that does not require a JSON body
    /// in the response (used for DELETE).
    async fn request_no_body(
        &self,
        label: &'static str,
        method: reqwest::Method,
        url: String,
    ) -> Result<()> {
        let policy = RetryPolicy::default();
        retry_with_backoff(label, &policy, |_attempt| {
            let method = method.clone();
            let url = url.clone();
            async move {
                let response = self
                    .http
                    .authenticated_request(method, &url, &self.auth, None)
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!("{label}: {e}")))
                    })?;
                if let Some(class) = classify_response(&response) {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    let err = Error::HttpError(format!("{label} HTTP {status}: {text}"));
                    return Err(match class {
                        ErrorClass::Permanent => ClassifiedError::permanent(err),
                        ErrorClass::Transient => ClassifiedError::transient(err),
                        ErrorClass::RateLimited { retry_after } => {
                            ClassifiedError::rate_limited(err, retry_after)
                        }
                    });
                }
                Ok(())
            }
        })
        .await
    }

    async fn fetch_cluster_json(&self, cluster_id: &str) -> Result<serde_json::Value> {
        self.request_json(
            "io.net GET cluster",
            reqwest::Method::GET,
            format!("{BASE_URL}/clusters/{cluster_id}"),
            None,
        )
        .await
    }

    /// Convert an io.net cluster JSON payload into a `ProvisionedInstance`.
    ///
    /// io.net wraps responses inconsistently: some endpoints return the cluster
    /// object directly, others wrap it under `data` or `cluster`. We try the
    /// common shapes in order.
    fn parse_cluster(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value
            .get("data")
            .or_else(|| value.get("cluster"))
            .unwrap_or(value);

        let id = data
            .get("cluster_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("io.net response missing cluster_id".into()))?
            .to_string();

        let instance_type = data
            .get("gpu_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let region = data
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();

        let status = match data.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "Running" | "Active" => InstanceStatus::Running,
            "Provisioning" | "Pending" | "Deploying" => InstanceStatus::Starting,
            "Terminating" | "Stopping" => InstanceStatus::Stopping,
            "Terminated" | "Failed" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };

        // Extract the first node's public_ip — io.net returns nodes as an array
        // of objects with `id` and `public_ip` (and sometimes `private_ip`).
        let first_node = data
            .get("nodes")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());
        let public_ip = first_node
            .and_then(|n| n.get("public_ip"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = first_node
            .and_then(|n| n.get("private_ip"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::IoNet,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, cluster_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "io.net cluster",
            CLUSTER_POLL_INTERVAL,
            CLUSTER_READY_TIMEOUT,
            || async {
                let raw = self.fetch_cluster_json(cluster_id).await?;
                let instance = Self::parse_cluster(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "io.net cluster terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }

    /// Provision a single-node io.net cluster of the requested GPU type.
    async fn launch_cluster(
        &self,
        gpu_type: &str,
        gpu_count: u32,
        region: &str,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("ionet");
        let payload = serde_json::json!({
            "name": name,
            "gpu_type": gpu_type,
            "gpu_count": gpu_count,
            "cluster_type": self.cluster_type,
            "region": region,
            "duration_hours": self.duration_hours,
        });
        let json = self
            .request_json(
                "io.net POST cluster",
                reqwest::Method::POST,
                format!("{BASE_URL}/clusters"),
                Some(payload),
            )
            .await?;

        let cluster_id = json
            .get("data")
            .and_then(|d| d.get("cluster_id"))
            .or_else(|| json.get("cluster_id"))
            .or_else(|| json.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("io.net launch: no cluster_id returned".into()))?
            .to_string();

        info!(%cluster_id, %region, %gpu_type, gpu_count, "Launched io.net cluster");
        self.wait_for_running(&cluster_id).await
    }
}

#[async_trait]
impl CloudProviderAdapter for IoNetAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };
        // `instance_type` here is a bare gpu_type identifier (e.g. "H100").
        // Multi-GPU sizing flows through `deploy_blueprint_with_target` via the
        // instance mapper; direct callers default to a single GPU.
        self.launch_cluster(instance_type, 1, region_name).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.request_no_body(
            "io.net DELETE cluster",
            reqwest::Method::DELETE,
            format!("{BASE_URL}/clusters/{instance_id}"),
        )
        .await?;
        info!(%instance_id, "Terminated io.net cluster");
        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_cluster_json(instance_id).await {
            Ok(raw) => Ok(Self::parse_cluster(&raw, &self.default_region)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get io.net cluster status");
                Ok(InstanceStatus::Unknown)
            }
        }
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
            DeploymentTarget::VirtualMachine { runtime: _ } => {
                let selection = IoNetInstanceMapper::select(resource_spec);
                let region = self.default_region.clone();
                provision_with_cleanup(
                    "io.net",
                    || async {
                        self.launch_cluster(&selection.gpu_type, selection.gpu_count, &region)
                            .await
                    },
                    |instance| async move {
                        self.deploy_blueprint(&instance, blueprint_image, resource_spec, env_vars)
                            .await
                    },
                    |id| async move { self.terminate_instance(&id).await },
                )
                .await
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "io.net does not offer managed Kubernetes".into(),
            )),
            DeploymentTarget::GenericKubernetes {
                context: _,
                namespace,
            } => {
                #[cfg(feature = "kubernetes")]
                {
                    use crate::shared::SharedKubernetesDeployment;
                    SharedKubernetesDeployment::deploy_to_generic_k8s(
                        namespace,
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
                        "Kubernetes feature not enabled".into(),
                    ))
                }
            }
            DeploymentTarget::Serverless { .. } => Err(Error::ConfigurationError(
                "io.net does not offer serverless deployment".into(),
            )),
        }
    }

    async fn deploy_blueprint(
        &self,
        instance: &ProvisionedInstance,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        require_public_ip(instance)?;
        deploy_via_ssh(
            instance,
            blueprint_image,
            resource_spec,
            env_vars,
            SshDeploymentConfig::io_net(),
        )
        .await
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

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        self.terminate_instance(&deployment.instance.id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_running_cluster_json() {
        let json = serde_json::json!({
            "cluster_id": "cl-abc123",
            "status": "Running",
            "gpu_type": "H100",
            "region": "us",
            "nodes": [
                { "id": "node-1", "public_ip": "203.0.113.10", "private_ip": "10.0.0.5" }
            ]
        });
        let instance = IoNetAdapter::parse_cluster(&json, "eu").unwrap();
        assert_eq!(instance.id, "cl-abc123");
        assert_eq!(instance.instance_type, "H100");
        assert_eq!(instance.region, "us");
        assert_eq!(instance.public_ip.as_deref(), Some("203.0.113.10"));
        assert_eq!(instance.private_ip.as_deref(), Some("10.0.0.5"));
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::IoNet);
    }

    #[test]
    fn parses_provisioning_cluster_as_starting() {
        let json = serde_json::json!({
            "cluster_id": "cl-pending",
            "status": "Provisioning",
            "gpu_type": "A100-80GB",
            "region": "us",
            "nodes": []
        });
        let instance = IoNetAdapter::parse_cluster(&json, "us").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
        assert!(instance.public_ip.is_none());
    }

    #[test]
    fn parses_terminated_cluster_json() {
        let json = serde_json::json!({
            "data": {
                "cluster_id": "cl-dead",
                "status": "Terminated",
                "gpu_type": "RTX_4090",
                "region": "us"
            }
        });
        let instance = IoNetAdapter::parse_cluster(&json, "us").unwrap();
        assert_eq!(instance.id, "cl-dead");
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    #[test]
    fn extracts_public_ip_from_first_node() {
        let json = serde_json::json!({
            "cluster_id": "cl-multi",
            "status": "Active",
            "gpu_type": "H100",
            "region": "us",
            "nodes": [
                { "id": "node-a", "public_ip": "198.51.100.1" },
                { "id": "node-b", "public_ip": "198.51.100.2" }
            ]
        });
        let instance = IoNetAdapter::parse_cluster(&json, "us").unwrap();
        assert_eq!(instance.public_ip.as_deref(), Some("198.51.100.1"));
        assert_eq!(instance.status, InstanceStatus::Running);
    }

    #[test]
    fn parse_fails_without_cluster_id() {
        let json = serde_json::json!({
            "status": "Running",
            "gpu_type": "H100"
        });
        assert!(IoNetAdapter::parse_cluster(&json, "us").is_err());
    }

    #[test]
    fn unknown_status_falls_back_to_unknown() {
        let json = serde_json::json!({
            "cluster_id": "cl-x",
            "status": "Frobnicating",
            "gpu_type": "H100",
            "region": "us"
        });
        let instance = IoNetAdapter::parse_cluster(&json, "us").unwrap();
        assert_eq!(instance.status, InstanceStatus::Unknown);
    }
}
