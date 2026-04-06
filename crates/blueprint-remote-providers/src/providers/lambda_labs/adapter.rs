//! Lambda Labs `CloudProviderAdapter` implementation.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, generate_instance_name, poll_until, require_public_ip,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://cloud.lambdalabs.com/api/v1";
const INSTANCE_READY_TIMEOUT: Duration = Duration::from_secs(600);
const INSTANCE_POLL_INTERVAL: Duration = Duration::from_secs(10);

/// Adapter for Lambda Labs on-demand GPU cloud.
pub struct LambdaLabsAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
    ssh_key_name: Option<String>,
}

impl LambdaLabsAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("LAMBDA_LABS_API_KEY")
            .map_err(|_| Error::Other("LAMBDA_LABS_API_KEY environment variable not set".into()))?;
        let default_region =
            std::env::var("LAMBDA_LABS_REGION").unwrap_or_else(|_| "us-west-1".to_string());
        let ssh_key_name = std::env::var("LAMBDA_LABS_SSH_KEY_NAME").ok();

        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::lambda_labs(api_key),
            default_region,
            ssh_key_name,
        })
    }

    /// Fetch a single instance's JSON payload from the REST API.
    async fn fetch_instance_json(&self, instance_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/instances/{instance_id}");
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
                        .map_err(|e| Error::HttpError(format!("Lambda Labs response parse: {e}")));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Lambda Labs GET {url}: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Lambda Labs GET {url} failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Lambda Labs GET {url}: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    /// Convert the Lambda Labs instance JSON into a `ProvisionedInstance`.
    fn parse_instance(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value.get("data").unwrap_or(value);
        let id = data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Lambda Labs response missing id".into()))?
            .to_string();
        let instance_type = data
            .get("instance_type")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = data
            .get("region")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = data
            .get("ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = data
            .get("private_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match data.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "active" => InstanceStatus::Running,
            "booting" => InstanceStatus::Starting,
            "terminating" => InstanceStatus::Stopping,
            "terminated" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::LambdaLabs,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, instance_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "Lambda Labs instance",
            INSTANCE_POLL_INTERVAL,
            INSTANCE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_instance_json(instance_id).await?;
                let instance = Self::parse_instance(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Lambda Labs instance terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for LambdaLabsAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("lambda");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };

        let mut payload = serde_json::json!({
            "region_name": region_name,
            "instance_type_name": instance_type,
            "quantity": 1,
            "name": name,
        });
        if let Some(key) = &self.ssh_key_name {
            payload["ssh_key_names"] = serde_json::json!([key]);
        }

        let launch_url = format!("{BASE_URL}/instance-operations/launch");
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
                        &launch_url,
                        &self.auth,
                        Some(payload.clone()),
                    )
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        result = Some(response.json::<serde_json::Value>().await.map_err(|e| {
                            Error::HttpError(format!("Lambda Labs response parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "Lambda Labs launch: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!(
                            "Lambda Labs launch failed: {body}"
                        )));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("Lambda Labs launch: {e}")));
                        continue;
                    }
                }
            }
            match result {
                Some(v) => v,
                None => return Err(last_err.unwrap()),
            }
        };

        let instance_id = json
            .get("data")
            .and_then(|d| d.get("instance_ids"))
            .and_then(|ids| ids.get(0))
            .and_then(|id| id.as_str())
            .ok_or_else(|| Error::HttpError("Lambda Labs launch: no instance id returned".into()))?
            .to_string();

        info!(%instance_id, %region_name, %instance_type, "Launched Lambda Labs instance");

        self.wait_for_running(&instance_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let payload = serde_json::json!({ "instance_ids": [instance_id] });
        let terminate_url = format!("{BASE_URL}/instance-operations/terminate");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self
                .http
                .authenticated_request(
                    reqwest::Method::POST,
                    &terminate_url,
                    &self.auth,
                    Some(payload.clone()),
                )
                .await
            {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated Lambda Labs instance");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Lambda Labs instance already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Lambda Labs terminate: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Lambda Labs terminate failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Lambda Labs terminate: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_instance_json(instance_id).await {
            Ok(raw) => {
                let instance = Self::parse_instance(&raw, &self.default_region)?;
                Ok(instance.status)
            }
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Lambda Labs instance status");
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
                let selection =
                    crate::providers::lambda_labs::LambdaLabsInstanceMapper::map(resource_spec);
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
                            provider = "lambda_labs",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "lambda_labs",
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
                "Lambda Labs does not offer managed Kubernetes".into(),
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
                "Lambda Labs does not offer serverless deployment".into(),
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
            SshDeploymentConfig::lambda_labs(),
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
    fn parses_running_instance_json() {
        let json = serde_json::json!({
            "data": {
                "id": "0920482c7a",
                "ip": "192.0.2.10",
                "private_ip": "10.0.0.5",
                "status": "active",
                "instance_type": { "name": "gpu_1x_a100" },
                "region": { "name": "us-west-1" }
            }
        });
        let instance = LambdaLabsAdapter::parse_instance(&json, "us-east-1").unwrap();
        assert_eq!(instance.id, "0920482c7a");
        assert_eq!(instance.public_ip.as_deref(), Some("192.0.2.10"));
        assert_eq!(instance.private_ip.as_deref(), Some("10.0.0.5"));
        assert_eq!(instance.region, "us-west-1");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::LambdaLabs);
    }

    #[test]
    fn parses_booting_instance_as_starting() {
        let json = serde_json::json!({
            "data": {
                "id": "pending",
                "status": "booting",
                "instance_type": { "name": "gpu_1x_a10" },
                "region": { "name": "us-west-1" }
            }
        });
        let instance = LambdaLabsAdapter::parse_instance(&json, "us-west-1").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
        assert!(instance.public_ip.is_none());
    }

    #[test]
    fn parses_terminated_instance() {
        let json = serde_json::json!({
            "data": {
                "id": "dead",
                "status": "terminated",
                "instance_type": { "name": "gpu_1x_a10" },
                "region": { "name": "us-west-1" }
            }
        });
        let instance = LambdaLabsAdapter::parse_instance(&json, "us-west-1").unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    #[test]
    fn parse_fails_without_id() {
        let json = serde_json::json!({
            "data": { "status": "active" }
        });
        assert!(LambdaLabsAdapter::parse_instance(&json, "us-west-1").is_err());
    }

    /// `deploy_blueprint_with_target` inlines the same cleanup-on-failure shape
    /// proven by `providers::common::gpu_adapter::tests::provision_with_cleanup_*`:
    /// when `deploy_blueprint` returns `Err`, `terminate_instance` is invoked with
    /// the provisioned id and the original deploy error is propagated. This test
    /// exercises the helper directly to lock that contract.
    #[tokio::test]
    async fn cleanup_on_failed_deploy_terminates_instance() {
        use crate::providers::common::gpu_adapter::provision_with_cleanup;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};

        let cleaned = Arc::new(AtomicBool::new(false));
        let cleaned_clone = cleaned.clone();
        let result = provision_with_cleanup(
            "lambda_labs",
            || async {
                Ok(ProvisionedInstance {
                    id: "ll-test".into(),
                    provider: CloudProvider::LambdaLabs,
                    instance_type: "gpu_1x_a10".into(),
                    region: "us-west-1".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("simulated deploy failure".into())) },
            |id| async move {
                assert_eq!(id, "ll-test");
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::SeqCst));
    }
}
