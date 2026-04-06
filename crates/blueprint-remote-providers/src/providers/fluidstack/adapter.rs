//! Fluidstack `CloudProviderAdapter`.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, generate_instance_name, poll_until, require_public_ip,
};
use crate::providers::fluidstack::FluidstackInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://api.fluidstack.io/v1";
const INSTANCE_READY_TIMEOUT: Duration = Duration::from_secs(600);
const INSTANCE_POLL_INTERVAL: Duration = Duration::from_secs(10);

pub struct FluidstackAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
}

impl FluidstackAdapter {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("FLUIDSTACK_API_KEY")
            .map_err(|_| Error::Other("FLUIDSTACK_API_KEY environment variable not set".into()))?;
        let default_region =
            std::env::var("FLUIDSTACK_REGION").unwrap_or_else(|_| "us_east".to_string());
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::fluidstack(api_key),
            default_region,
        })
    }

    async fn fetch_instance(&self, id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/servers/{id}");
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
                        .map_err(|e| Error::HttpError(format!("Fluidstack response parse: {e}")));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Fluidstack GET server: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Fluidstack GET server failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Fluidstack GET server: {e}")));
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
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Fluidstack response missing id".into()))?
            .to_string();
        let instance_type = value
            .get("plan")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = value
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = value
            .get("ip_address")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match value.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "running" => InstanceStatus::Running,
            "provisioning" | "pending" => InstanceStatus::Starting,
            "stopped" => InstanceStatus::Stopped,
            "terminated" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::Fluidstack,
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
            "Fluidstack server",
            INSTANCE_POLL_INTERVAL,
            INSTANCE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_instance(instance_id).await?;
                let instance = Self::parse_instance(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Fluidstack server terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for FluidstackAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("fluidstack");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };
        let payload = serde_json::json!({
            "plan": instance_type,
            "region": region_name,
            "hostname": name,
            "operating_system": "ubuntu_22_04",
            "ssh_key_ids": std::env::var("FLUIDSTACK_SSH_KEY_IDS")
                .ok()
                .map(|s| s.split(',').map(|k| k.trim().to_string()).collect::<Vec<_>>())
                .unwrap_or_default(),
        });
        let create_url = format!("{BASE_URL}/servers");
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
                            Error::HttpError(format!("Fluidstack create parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "Fluidstack create server: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!(
                            "Fluidstack create server failed: {body}"
                        )));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("Fluidstack create server: {e}")));
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
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Fluidstack create: no id returned".into()))?
            .to_string();
        info!(%instance_id, %instance_type, "Created Fluidstack server");
        self.wait_for_running(&instance_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/servers/{instance_id}");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.delete(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated Fluidstack server");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Fluidstack server already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Fluidstack delete: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Fluidstack delete failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Fluidstack delete: {e}")));
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
                warn!(%instance_id, error = %e, "Failed to get Fluidstack server status");
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
                let selection = FluidstackInstanceMapper::map(resource_spec);
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
                            provider = "fluidstack",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "fluidstack",
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
                "Fluidstack only supports VirtualMachine deployment targets".into(),
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
            SshDeploymentConfig::fluidstack(),
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
    fn parses_running_server() {
        let json = serde_json::json!({
            "id": "fs-abc",
            "status": "running",
            "ip_address": "203.0.113.9",
            "plan": "a100_80gb_pcie",
            "region": "us_east"
        });
        let instance = FluidstackAdapter::parse_instance(&json, "us_east").unwrap();
        assert_eq!(instance.id, "fs-abc");
        assert_eq!(instance.instance_type, "a100_80gb_pcie");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::Fluidstack);
    }

    #[test]
    fn parses_pending_as_starting() {
        let json = serde_json::json!({
            "id": "fs-new",
            "status": "pending",
            "plan": "rtx_a4000",
            "region": "us_east"
        });
        let instance = FluidstackAdapter::parse_instance(&json, "us_east").unwrap();
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
            "fluidstack",
            || async {
                Ok(ProvisionedInstance {
                    id: "fs-test".into(),
                    provider: CloudProvider::Fluidstack,
                    instance_type: "a100_80gb_pcie".into(),
                    region: "us_east".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("simulated deploy failure".into())) },
            |id| async move {
                assert_eq!(id, "fs-test");
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::SeqCst));
    }
}
