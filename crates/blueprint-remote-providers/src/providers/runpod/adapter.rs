//! RunPod `CloudProviderAdapter` implementation.
//!
//! Uses the REST API (`https://rest.runpod.io/v1/pods`) to manage pods.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, generate_instance_name, gpu_count_or_one, poll_until,
    require_public_ip,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://rest.runpod.io/v1";
const POD_READY_TIMEOUT: Duration = Duration::from_secs(600);
const POD_POLL_INTERVAL: Duration = Duration::from_secs(8);

pub struct RunPodAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    cloud_type: String,
    default_region: String,
}

impl RunPodAdapter {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("RUNPOD_API_KEY")
            .map_err(|_| Error::Other("RUNPOD_API_KEY environment variable not set".into()))?;
        let cloud_type = std::env::var("RUNPOD_CLOUD_TYPE").unwrap_or_else(|_| "SECURE".into());
        let default_region = std::env::var("RUNPOD_REGION").unwrap_or_else(|_| "US".into());
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::runpod(api_key),
            cloud_type,
            default_region,
        })
    }

    async fn fetch_pod(&self, pod_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/pods/{pod_id}");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.get(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    return response
                        .json::<serde_json::Value>()
                        .await
                        .map_err(|e| Error::HttpError(format!("RunPod response parse: {e}")));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "RunPod GET {url}: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!("RunPod GET {url} failed: {body}")));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("RunPod GET {url}: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn parse_pod(value: &serde_json::Value, fallback_region: &str) -> Result<ProvisionedInstance> {
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("RunPod response missing id".into()))?
            .to_string();
        let instance_type = value
            .get("machine")
            .and_then(|m| m.get("gpuTypeId"))
            .or_else(|| value.get("gpuTypeId"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = value
            .get("machine")
            .and_then(|m| m.get("dataCenterId"))
            .or_else(|| value.get("dataCenterId"))
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = value
            .get("publicIp")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = value
            .get("privateIp")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match value
            .get("desiredStatus")
            .or_else(|| value.get("status"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
        {
            "RUNNING" => InstanceStatus::Running,
            "CREATED" | "PROVISIONING" | "STARTING" => InstanceStatus::Starting,
            "PAUSED" => InstanceStatus::Stopped,
            "EXITED" | "TERMINATED" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::RunPod,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, pod_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "RunPod pod",
            POD_POLL_INTERVAL,
            POD_READY_TIMEOUT,
            || async {
                let raw = self.fetch_pod(pod_id).await?;
                let instance = Self::parse_pod(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "RunPod pod terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for RunPodAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        _region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("runpod");
        let payload = serde_json::json!({
            "name": name,
            "gpuTypeIds": [instance_type],
            "cloudType": self.cloud_type,
            "gpuCount": 1,
            "containerDiskInGb": 50,
            "volumeInGb": 0,
            "imageName": "runpod/pytorch:2.1.0-py3.10-cuda11.8.0-devel-ubuntu22.04",
            "ports": "22/tcp,9615/tcp,9944/tcp",
            "supportPublicIp": true,
        });

        let create_url = format!("{BASE_URL}/pods");
        let json: serde_json::Value = {
            let mut last_err = None;
            let mut result = None;
            for attempt in 0..3u32 {
                if attempt > 0 {
                    tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
                }
                match self
                    .http
                    .authenticated_request(
                        reqwest::Method::POST,
                        &create_url,
                        &self.auth,
                        Some(payload.clone()),
                    )
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        result = Some(response.json::<serde_json::Value>().await.map_err(|e| {
                            Error::HttpError(format!("RunPod response parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "RunPod create pod: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!(
                            "RunPod create pod failed: {body}"
                        )));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("RunPod create pod: {e}")));
                        continue;
                    }
                }
            }
            match result {
                Some(v) => v,
                None => return Err(last_err.unwrap()),
            }
        };
        let pod_id = json
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("RunPod create pod: no id in response".into()))?
            .to_string();

        info!(%pod_id, %instance_type, "Created RunPod pod");

        self.wait_for_running(&pod_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/pods/{instance_id}");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.delete(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated RunPod pod");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "RunPod pod already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "RunPod delete pod: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "RunPod delete pod failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("RunPod delete pod: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_pod(instance_id).await {
            Ok(raw) => Ok(Self::parse_pod(&raw, &self.default_region)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get RunPod pod status");
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
                let selection = crate::providers::runpod::RunPodInstanceMapper::map(resource_spec);
                let _ = gpu_count_or_one(resource_spec);
                let instance = self
                    .provision_instance(&selection.instance_type, "", false)
                    .await?;
                let instance_id_for_cleanup = instance.id.clone();
                match self
                    .deploy_blueprint(&instance, blueprint_image, resource_spec, env_vars)
                    .await
                {
                    Ok(result) => Ok(result),
                    Err(deploy_err) => {
                        warn!(
                            provider = "runpod",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "runpod",
                                instance_id = %instance_id_for_cleanup,
                                cleanup_error = %cleanup_err,
                                "Cleanup after failed deploy also failed — instance may be orphaned"
                            );
                        }
                        Err(deploy_err)
                    }
                }
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "RunPod does not expose a managed Kubernetes product via this API".into(),
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
                "RunPod Serverless uses a distinct /v2/serverless API — not supported here".into(),
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
            SshDeploymentConfig::runpod(),
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
    fn parses_running_pod() {
        let json = serde_json::json!({
            "id": "pod-abc",
            "desiredStatus": "RUNNING",
            "publicIp": "203.0.113.5",
            "privateIp": "10.2.3.4",
            "machine": {
                "gpuTypeId": "NVIDIA A100 80GB PCIe",
                "dataCenterId": "US-EAST-1"
            }
        });
        let instance = RunPodAdapter::parse_pod(&json, "US").unwrap();
        assert_eq!(instance.id, "pod-abc");
        assert_eq!(instance.instance_type, "NVIDIA A100 80GB PCIe");
        assert_eq!(instance.public_ip.as_deref(), Some("203.0.113.5"));
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::RunPod);
    }

    #[test]
    fn parses_provisioning_pod_as_starting() {
        let json = serde_json::json!({
            "id": "pod-new",
            "desiredStatus": "PROVISIONING",
            "machine": { "gpuTypeId": "X", "dataCenterId": "X" }
        });
        let instance = RunPodAdapter::parse_pod(&json, "US").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
    }

    #[test]
    fn parses_exited_pod_as_terminated() {
        let json = serde_json::json!({
            "id": "pod-dead",
            "desiredStatus": "EXITED",
            "machine": { "gpuTypeId": "X", "dataCenterId": "X" }
        });
        let instance = RunPodAdapter::parse_pod(&json, "US").unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    /// `deploy_blueprint_with_target` inlines the same cleanup-on-failure shape
    /// proven by `providers::common::gpu_adapter::tests::provision_with_cleanup_*`.
    #[tokio::test]
    async fn cleanup_on_failed_deploy_terminates_instance() {
        use crate::providers::common::gpu_adapter::provision_with_cleanup;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let cleaned = Arc::new(AtomicBool::new(false));
        let cleaned_clone = cleaned.clone();
        let result = provision_with_cleanup(
            "runpod",
            || async {
                Ok(ProvisionedInstance {
                    id: "pod-test".into(),
                    provider: CloudProvider::RunPod,
                    instance_type: "A100".into(),
                    region: "US".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("simulated deploy failure".into())) },
            |id| async move {
                assert_eq!(id, "pod-test");
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::SeqCst));
    }
}
