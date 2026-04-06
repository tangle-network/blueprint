//! Paperspace `CloudProviderAdapter`.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, generate_instance_name, poll_until, require_public_ip,
};
use crate::providers::paperspace::PaperspaceInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://api.paperspace.io";
const MACHINE_READY_TIMEOUT: Duration = Duration::from_secs(600);
const MACHINE_POLL_INTERVAL: Duration = Duration::from_secs(10);

pub struct PaperspaceAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
}

impl PaperspaceAdapter {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("PAPERSPACE_API_KEY")
            .map_err(|_| Error::Other("PAPERSPACE_API_KEY environment variable not set".into()))?;
        let default_region =
            std::env::var("PAPERSPACE_REGION").unwrap_or_else(|_| "East Coast (NY2)".to_string());
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::paperspace(api_key),
            default_region,
        })
    }

    async fn fetch_machine(&self, id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/machines/getMachinePublic?machineId={id}");
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
                        .map_err(|e| Error::HttpError(format!("Paperspace machine parse: {e}")));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Paperspace GET machine: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Paperspace GET machine failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Paperspace GET machine: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn parse_machine(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Paperspace response missing id".into()))?
            .to_string();
        let instance_type = value
            .get("machineType")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = value
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = value
            .get("publicIpAddress")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = value
            .get("privateIpAddress")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match value.get("state").and_then(|v| v.as_str()).unwrap_or("") {
            "ready" | "running" => InstanceStatus::Running,
            "provisioning" | "starting" | "restarting" => InstanceStatus::Starting,
            "stopping" => InstanceStatus::Stopping,
            "off" | "stopped" => InstanceStatus::Stopped,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::Paperspace,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, machine_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "Paperspace machine",
            MACHINE_POLL_INTERVAL,
            MACHINE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_machine(machine_id).await?;
                let instance = Self::parse_machine(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    _ => Ok(None),
                }
            },
        )
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for PaperspaceAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("paperspace");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };
        let payload = serde_json::json!({
            "region": region_name,
            "machineType": instance_type,
            "size": 50,
            "billingType": "hourly",
            "machineName": name,
            "templateId": "tmpl-ubuntu-22-04",
            "assignPublicIp": true,
        });
        let create_url = format!("{BASE_URL}/machines/createSingleMachinePublic");
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
                            Error::HttpError(format!("Paperspace create parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "Paperspace create machine: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!(
                            "Paperspace create machine failed: {body}"
                        )));
                    }
                    Err(e) => {
                        last_err =
                            Some(Error::HttpError(format!("Paperspace create machine: {e}")));
                        continue;
                    }
                }
            }
            match result {
                Some(v) => v,
                None => return Err(last_err.unwrap()),
            }
        };
        let machine_id = json
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Paperspace create machine: no id returned".into()))?
            .to_string();
        info!(%machine_id, %instance_type, "Created Paperspace machine");
        self.wait_for_running(&machine_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/machines/{instance_id}/destroyMachine");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self
                .http
                .authenticated_request(reqwest::Method::POST, &url, &self.auth, None)
                .await
            {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated Paperspace machine");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Paperspace machine already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Paperspace destroy: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Paperspace destroy failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Paperspace destroy: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_machine(instance_id).await {
            Ok(raw) => Ok(Self::parse_machine(&raw, &self.default_region)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Paperspace machine status");
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
                let selection = PaperspaceInstanceMapper::map(resource_spec);
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
                            provider = "paperspace",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "paperspace",
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
                "Paperspace only supports VirtualMachine deployment targets".into(),
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
            SshDeploymentConfig::paperspace(),
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
    fn parses_ready_machine() {
        let json = serde_json::json!({
            "id": "ps123",
            "state": "ready",
            "publicIpAddress": "192.0.2.50",
            "privateIpAddress": "10.0.0.2",
            "machineType": "A100",
            "region": "East Coast (NY2)"
        });
        let instance = PaperspaceAdapter::parse_machine(&json, "East Coast (NY2)").unwrap();
        assert_eq!(instance.id, "ps123");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.public_ip.as_deref(), Some("192.0.2.50"));
        assert_eq!(instance.provider, CloudProvider::Paperspace);
    }

    #[test]
    fn parses_provisioning_machine_as_starting() {
        let json = serde_json::json!({
            "id": "ps-new",
            "state": "provisioning",
            "machineType": "P4000",
            "region": "East Coast (NY2)"
        });
        let instance = PaperspaceAdapter::parse_machine(&json, "East Coast (NY2)").unwrap();
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
            "paperspace",
            || async {
                Ok(ProvisionedInstance {
                    id: "ps-test".into(),
                    provider: CloudProvider::Paperspace,
                    instance_type: "A100".into(),
                    region: "East Coast (NY2)".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("simulated deploy failure".into())) },
            |id| async move {
                assert_eq!(id, "ps-test");
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::SeqCst));
    }
}
