//! Render Network Dispersed `CloudProviderAdapter` implementation.
//!
//! **Stability note:** Render Dispersed is a young platform (public preview Q4 2025).
//! The endpoint shapes encoded here track the public docs at
//! <https://dispersed.com/docs> as of 2026-04. Operators should run a smoke
//! provision/teardown before relying on this adapter in production. The adapter
//! tolerates minor field renames via lenient JSON parsing, but breaking changes
//! to the resource model will require code updates.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    ClassifiedError, RetryPolicy, build_http_client, classify_response, deploy_via_ssh,
    generate_instance_name, poll_until, provision_with_cleanup, require_public_ip,
    retry_with_backoff,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://api.rendernetwork.com/v1/dispersed";
const NODE_READY_TIMEOUT: Duration = Duration::from_secs(600);
const NODE_POLL_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_IMAGE: &str = "dispersed/ubuntu-22.04-cuda12";

/// Adapter for Render Network's Dispersed AI compute platform.
pub struct RenderAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
    image: String,
    ssh_key_id: Option<String>,
}

impl RenderAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("RENDER_API_KEY")
            .map_err(|_| Error::Other("RENDER_API_KEY environment variable not set".into()))?;
        let default_region =
            std::env::var("RENDER_REGION").unwrap_or_else(|_| "na-east".to_string());
        let image = std::env::var("RENDER_IMAGE").unwrap_or_else(|_| DEFAULT_IMAGE.to_string());
        let ssh_key_id = std::env::var("RENDER_SSH_KEY_ID").ok();

        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::render(api_key),
            default_region,
            image,
            ssh_key_id,
        })
    }

    async fn fetch_node_json(&self, node_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/compute/nodes/{node_id}");
        retry_with_backoff("render.get_node", &RetryPolicy::default(), |_| {
            let url = url.clone();
            async move {
                let response = self.http.get(&url, &self.auth).await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!("Render GET {url}: {e}")))
                })?;
                if let Some(class) = classify_response(&response) {
                    let body = response.text().await.unwrap_or_default();
                    return Err(ClassifiedError {
                        class,
                        inner: Error::HttpError(format!("Render GET {url} failed: {body}")),
                    });
                }
                response.json::<serde_json::Value>().await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!(
                        "Render response parse: {e}"
                    )))
                })
            }
        })
        .await
    }

    /// Convert a Render Dispersed node JSON payload into a `ProvisionedInstance`.
    fn parse_instance(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value
            .get("node")
            .or_else(|| value.get("data"))
            .unwrap_or(value);
        let id = data
            .get("node_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Render response missing node_id".into()))?
            .to_string();
        let instance_type = data
            .get("gpu_tier")
            .or_else(|| data.get("tier"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = data
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = data
            .get("public_ip")
            .or_else(|| data.get("ip"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = data
            .get("private_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match data.get("state").and_then(|v| v.as_str()).unwrap_or("") {
            "ready" | "running" | "active" => InstanceStatus::Running,
            "provisioning" | "starting" | "queued" | "pending" => InstanceStatus::Starting,
            "stopping" | "draining" => InstanceStatus::Stopping,
            "terminated" | "failed" | "expired" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::Render,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, node_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "Render Dispersed node",
            NODE_POLL_INTERVAL,
            NODE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_node_json(node_id).await?;
                let instance = Self::parse_instance(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Render node terminated before becoming ready".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }

    async fn launch_node(&self, gpu_tier: &str, region: &str) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("render");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };

        let mut payload = serde_json::json!({
            "name": name,
            "gpu_tier": gpu_tier,
            "region": region_name,
            "container_image": self.image,
        });
        if let Some(key_id) = &self.ssh_key_id {
            payload["ssh_key_id"] = serde_json::json!(key_id);
        }

        let json: serde_json::Value =
            retry_with_backoff("render.launch", &RetryPolicy::default(), |_| {
                let payload = payload.clone();
                async move {
                    let response = self
                        .http
                        .authenticated_request(
                            reqwest::Method::POST,
                            &format!("{BASE_URL}/compute/nodes"),
                            &self.auth,
                            Some(payload),
                        )
                        .await
                        .map_err(|e| {
                            ClassifiedError::transient(Error::HttpError(format!(
                                "Render launch: {e}"
                            )))
                        })?;
                    if let Some(class) = classify_response(&response) {
                        let body = response.text().await.unwrap_or_default();
                        return Err(ClassifiedError {
                            class,
                            inner: Error::HttpError(format!("Render launch failed: {body}")),
                        });
                    }
                    response.json::<serde_json::Value>().await.map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Render launch parse: {e}"
                        )))
                    })
                }
            })
            .await?;

        let data = json
            .get("node")
            .or_else(|| json.get("data"))
            .unwrap_or(&json);
        let node_id = data
            .get("node_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Render launch: no node id returned".into()))?
            .to_string();

        info!(%node_id, %region_name, %gpu_tier, "Launched Render Dispersed node");

        self.wait_for_running(&node_id).await
    }

    async fn delete_node(&self, node_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/compute/nodes/{node_id}");
        retry_with_backoff("render.terminate", &RetryPolicy::default(), |_| {
            let url = url.clone();
            async move {
                let response = self
                    .http
                    .authenticated_request(reqwest::Method::DELETE, &url, &self.auth, None)
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Render terminate: {e}"
                        )))
                    })?;
                if let Some(class) = classify_response(&response) {
                    if response.status().as_u16() == 404 {
                        return Ok(());
                    }
                    let body = response.text().await.unwrap_or_default();
                    return Err(ClassifiedError {
                        class,
                        inner: Error::HttpError(format!("Render terminate failed: {body}")),
                    });
                }
                Ok(())
            }
        })
        .await?;
        info!(%node_id, "Terminated Render Dispersed node");
        Ok(())
    }
}

#[async_trait]
impl CloudProviderAdapter for RenderAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        self.launch_node(instance_type, region).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.delete_node(instance_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_node_json(instance_id).await {
            Ok(raw) => {
                let instance = Self::parse_instance(&raw, &self.default_region)?;
                Ok(instance.status)
            }
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Render node status");
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
                let plan = super::RenderInstanceMapper::map(resource_spec);
                provision_with_cleanup(
                    "render",
                    || self.launch_node(&plan.instance_type, ""),
                    |instance| {
                        let env_vars = env_vars.clone();
                        async move {
                            self.deploy_blueprint(
                                &instance,
                                blueprint_image,
                                resource_spec,
                                env_vars,
                            )
                            .await
                        }
                    },
                    |id| async move { self.delete_node(&id).await },
                )
                .await
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "Render Dispersed does not offer managed Kubernetes".into(),
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
                "Render Dispersed does not offer serverless deployment".into(),
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
            SshDeploymentConfig::render(),
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
    fn parses_running_json() {
        let json = serde_json::json!({
            "node": {
                "node_id": "rn-001",
                "state": "ready",
                "gpu_tier": "premium",
                "region": "na-east",
                "public_ip": "198.51.100.20",
                "private_ip": "10.10.0.5"
            }
        });
        let instance = RenderAdapter::parse_instance(&json, "na-west").unwrap();
        assert_eq!(instance.id, "rn-001");
        assert_eq!(instance.public_ip.as_deref(), Some("198.51.100.20"));
        assert_eq!(instance.region, "na-east");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::Render);
    }

    #[test]
    fn parses_starting_json() {
        let json = serde_json::json!({
            "node": {
                "node_id": "rn-pending",
                "state": "provisioning",
                "gpu_tier": "standard",
                "region": "na-east"
            }
        });
        let instance = RenderAdapter::parse_instance(&json, "na-east").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
        assert!(instance.public_ip.is_none());
    }

    #[test]
    fn parses_terminated_json() {
        let json = serde_json::json!({
            "node": {
                "node_id": "rn-dead",
                "state": "terminated",
                "gpu_tier": "entry",
                "region": "na-east"
            }
        });
        let instance = RenderAdapter::parse_instance(&json, "na-east").unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    #[test]
    fn parse_fails_without_id() {
        let json = serde_json::json!({ "node": { "state": "ready" } });
        assert!(RenderAdapter::parse_instance(&json, "na-east").is_err());
    }
}
