//! TensorDock `CloudProviderAdapter`.
//!
//! TensorDock's API authenticates each call with `api_key` + `api_token` passed
//! in the JSON body. We represent this using `ApiAuthentication::None` at the
//! transport layer and inject both values in each request payload.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, generate_instance_name, poll_until, require_public_ip,
};
use crate::providers::tensordock::TensorDockInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://marketplace.tensordock.com/api/v0";
const INSTANCE_READY_TIMEOUT: Duration = Duration::from_secs(600);
const INSTANCE_POLL_INTERVAL: Duration = Duration::from_secs(10);

pub struct TensorDockAdapter {
    http: SecureHttpClient,
    api_key: String,
    api_token: String,
    default_region: String,
}

impl TensorDockAdapter {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("TENSORDOCK_API_KEY")
            .map_err(|_| Error::Other("TENSORDOCK_API_KEY environment variable not set".into()))?;
        let api_token = std::env::var("TENSORDOCK_API_TOKEN").map_err(|_| {
            Error::Other("TENSORDOCK_API_TOKEN environment variable not set".into())
        })?;
        let default_region =
            std::env::var("TENSORDOCK_REGION").unwrap_or_else(|_| "any".to_string());
        Ok(Self {
            http: build_http_client()?,
            api_key,
            api_token,
            default_region,
        })
    }

    /// Inject api_key + api_token into a payload object.
    fn authed_payload(&self, mut payload: serde_json::Value) -> serde_json::Value {
        if let Some(map) = payload.as_object_mut() {
            map.insert("api_key".to_string(), serde_json::json!(self.api_key));
            map.insert("api_token".to_string(), serde_json::json!(self.api_token));
        }
        payload
    }

    async fn fetch_instance(&self, id: &str) -> Result<serde_json::Value> {
        let payload = self.authed_payload(serde_json::json!({ "server": id }));
        let fetch_url = format!("{BASE_URL}/client/get/single");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self
                .http
                .authenticated_request(
                    reqwest::Method::POST,
                    &fetch_url,
                    &ApiAuthentication::None,
                    Some(payload.clone()),
                )
                .await
            {
                Ok(response) if response.status().is_success() => {
                    return response
                        .json::<serde_json::Value>()
                        .await
                        .map_err(|e| Error::HttpError(format!("TensorDock response parse: {e}")));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "TensorDock get instance: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "TensorDock get instance failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("TensorDock get instance: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn parse_instance(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let server = value.get("server").unwrap_or(value);
        let id = server
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("TensorDock response missing server id".into()))?
            .to_string();
        let instance_type = server
            .get("gpu_model")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = server
            .get("location")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = server
            .get("ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match server.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "running" | "active" => InstanceStatus::Running,
            "provisioning" | "starting" => InstanceStatus::Starting,
            "stopped" | "off" => InstanceStatus::Stopped,
            "terminated" | "destroyed" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::TensorDock,
            instance_type,
            region,
            public_ip,
            private_ip: None,
            status,
        })
    }

    async fn wait_for_running(&self, instance_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "TensorDock server",
            INSTANCE_POLL_INTERVAL,
            INSTANCE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_instance(instance_id).await?;
                let instance = Self::parse_instance(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "TensorDock server terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for TensorDockAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("tensordock");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };
        let payload = self.authed_payload(serde_json::json!({
            "name": name,
            "gpu_model": instance_type,
            "gpu_count": 1,
            "vcpus": 8,
            "ram": 32,
            "storage": 100,
            "operating_system": "Ubuntu 22.04",
            "location": region_name,
            "external_ports": "22,9615,9944",
            "internal_ports": "22,9615,9944",
        }));
        let deploy_url = format!("{BASE_URL}/client/deploy/single");
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
                        &deploy_url,
                        &ApiAuthentication::None,
                        Some(payload.clone()),
                    )
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        result = Some(response.json::<serde_json::Value>().await.map_err(|e| {
                            Error::HttpError(format!("TensorDock deploy parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "TensorDock deploy: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!(
                            "TensorDock deploy failed: {body}"
                        )));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("TensorDock deploy: {e}")));
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
            .get("server")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("TensorDock deploy: no server id".into()))?
            .to_string();
        info!(%instance_id, %instance_type, "Created TensorDock server");
        self.wait_for_running(&instance_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let payload = self.authed_payload(serde_json::json!({ "server": instance_id }));
        let delete_url = format!("{BASE_URL}/client/delete/single");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self
                .http
                .authenticated_request(
                    reqwest::Method::POST,
                    &delete_url,
                    &ApiAuthentication::None,
                    Some(payload.clone()),
                )
                .await
            {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated TensorDock server");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "TensorDock server already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "TensorDock delete: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "TensorDock delete failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("TensorDock delete: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_instance(instance_id).await {
            Ok(raw) => Ok(Self::parse_instance(&raw, &self.default_region)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get TensorDock server status");
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
                let selection = TensorDockInstanceMapper::map(resource_spec);
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
                            provider = "tensordock",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "tensordock",
                                instance_id = %instance_id_for_cleanup,
                                cleanup_error = %cleanup_err,
                                "Cleanup after failed deploy also failed — instance may be orphaned"
                            );
                        }
                        Err(deploy_err)
                    }
                }
            }
            _ => Err(Error::ConfigurationError(
                "TensorDock only supports VirtualMachine deployment targets".into(),
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
            SshDeploymentConfig::tensordock(),
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
    fn parses_running_server_from_nested_response() {
        let json = serde_json::json!({
            "server": {
                "id": "td-123",
                "status": "running",
                "ip": "198.51.100.2",
                "gpu_model": "a100-80gb",
                "location": "us-west"
            }
        });
        let instance = TensorDockAdapter::parse_instance(&json, "us-west").unwrap();
        assert_eq!(instance.id, "td-123");
        assert_eq!(instance.instance_type, "a100-80gb");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::TensorDock);
    }

    #[test]
    fn parses_provisioning_as_starting() {
        let json = serde_json::json!({
            "server": {
                "id": "td-new",
                "status": "provisioning",
                "gpu_model": "rtx4090-24gb",
                "location": "us-west"
            }
        });
        let instance = TensorDockAdapter::parse_instance(&json, "us-west").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
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
            "tensordock",
            || async {
                Ok(ProvisionedInstance {
                    id: "td-test".into(),
                    provider: CloudProvider::TensorDock,
                    instance_type: "a100-80gb".into(),
                    region: "us-west".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("simulated deploy failure".into())) },
            |id| async move {
                assert_eq!(id, "td-test");
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::SeqCst));
    }

    #[test]
    fn authed_payload_includes_credentials() {
        // Construct the adapter directly to avoid env var requirements.
        let adapter = TensorDockAdapter {
            http: build_http_client().unwrap(),
            api_key: "key123".into(),
            api_token: "tok456".into(),
            default_region: "any".into(),
        };
        let authed = adapter.authed_payload(serde_json::json!({ "foo": "bar" }));
        assert_eq!(authed["api_key"].as_str(), Some("key123"));
        assert_eq!(authed["api_token"].as_str(), Some("tok456"));
        assert_eq!(authed["foo"].as_str(), Some("bar"));
    }
}
