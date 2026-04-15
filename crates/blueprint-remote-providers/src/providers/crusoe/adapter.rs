//! Crusoe Energy `CloudProviderAdapter` implementation.
//!
//! Uses the REST API (`https://api.crusoecloud.com/v1alpha5`) to manage VMs.

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

const BASE_URL: &str = "https://api.crusoecloud.com/v1alpha5";
const VM_READY_TIMEOUT: Duration = Duration::from_secs(600);
const VM_POLL_INTERVAL: Duration = Duration::from_secs(10);

pub struct CrusoeAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    project_id: String,
    default_location: String,
    ssh_key: Option<String>,
}

impl CrusoeAdapter {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("CRUSOE_API_KEY")
            .map_err(|_| Error::Other("CRUSOE_API_KEY environment variable not set".into()))?;
        let api_secret = std::env::var("CRUSOE_API_SECRET")
            .map_err(|_| Error::Other("CRUSOE_API_SECRET environment variable not set".into()))?;
        let project_id = std::env::var("CRUSOE_PROJECT_ID")
            .map_err(|_| Error::Other("CRUSOE_PROJECT_ID environment variable not set".into()))?;
        let default_location =
            std::env::var("CRUSOE_REGION").unwrap_or_else(|_| "us-east1".to_string());
        let ssh_key = std::env::var("CRUSOE_SSH_PUBLIC_KEY").ok();
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::crusoe(api_key, api_secret),
            project_id,
            default_location,
            ssh_key,
        })
    }

    async fn fetch_vm(&self, vm_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/projects/{}/vms/{vm_id}", self.project_id);
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
                        .map_err(|e| Error::HttpError(format!("Crusoe response parse: {e}")));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Crusoe GET {url}: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!("Crusoe GET {url} failed: {body}")));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Crusoe GET {url}: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn parse_vm(value: &serde_json::Value) -> Result<ProvisionedInstance> {
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Crusoe response missing id".into()))?
            .to_string();
        let instance_type = value
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = value
            .get("location")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let public_ip = value
            .get("network_interfaces")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|iface| iface.get("ips"))
            .and_then(|v| v.as_array())
            .and_then(|arr| {
                arr.iter()
                    .find(|ip| {
                        ip.get("public_ipv4")
                            .and_then(|v| v.get("address"))
                            .and_then(|v| v.as_str())
                            .is_some()
                    })
                    .and_then(|ip| {
                        ip.get("public_ipv4")
                            .and_then(|v| v.get("address"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                    })
            });
        let private_ip = value
            .get("network_interfaces")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|iface| iface.get("ips"))
            .and_then(|v| v.as_array())
            .and_then(|arr| {
                arr.iter().find_map(|ip| {
                    ip.get("private_ipv4")
                        .and_then(|v| v.get("address"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
            });
        let status = match value.get("state").and_then(|v| v.as_str()).unwrap_or("") {
            "STATE_RUNNING" => InstanceStatus::Running,
            "STATE_CREATING" | "STATE_STARTING" => InstanceStatus::Starting,
            "STATE_STOPPING" => InstanceStatus::Stopping,
            "STATE_STOPPED" => InstanceStatus::Stopped,
            "STATE_DELETING" | "STATE_DELETED" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::Crusoe,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, vm_id: &str) -> Result<ProvisionedInstance> {
        poll_until("Crusoe VM", VM_POLL_INTERVAL, VM_READY_TIMEOUT, || async {
            let raw = self.fetch_vm(vm_id).await?;
            let instance = Self::parse_vm(&raw)?;
            match instance.status {
                InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                InstanceStatus::Terminated => Err(Error::HttpError(
                    "Crusoe VM terminated before running".into(),
                )),
                _ => Ok(None),
            }
        })
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for CrusoeAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("crusoe");
        let location = if region.is_empty() {
            &self.default_location
        } else {
            region
        };

        let mut payload = serde_json::json!({
            "name": name,
            "type": instance_type,
            "location": location,
            "image": "ubuntu22.04-nvidia-pcie-docker",
            "network_interfaces": [{
                "ips": [{ "public_ipv4": {} }],
            }],
        });

        if let Some(ssh_key) = &self.ssh_key {
            payload["ssh_public_key"] = serde_json::Value::String(ssh_key.clone());
        }

        let create_url = format!("{BASE_URL}/projects/{}/vms", self.project_id);
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
                            Error::HttpError(format!("Crusoe response parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "Crusoe create VM: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!("Crusoe create VM failed: {body}")));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("Crusoe create VM: {e}")));
                        continue;
                    }
                }
            }
            match result {
                Some(v) => v,
                None => return Err(last_err.unwrap()),
            }
        };

        let vm_id = json
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Crusoe create VM: no id in response".into()))?
            .to_string();

        info!(%vm_id, %instance_type, location, "Created Crusoe VM");

        self.wait_for_running(&vm_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/projects/{}/vms/{instance_id}", self.project_id);
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.delete(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated Crusoe VM");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Crusoe VM already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Crusoe delete VM: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!("Crusoe delete VM failed: {body}")));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Crusoe delete VM: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_vm(instance_id).await {
            Ok(raw) => Ok(Self::parse_vm(&raw)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Crusoe VM status");
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
                let selection = crate::providers::crusoe::CrusoeInstanceMapper::map(resource_spec);
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
                            provider = "crusoe",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "crusoe",
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
                "Crusoe does not expose managed Kubernetes via this adapter".into(),
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
                "Crusoe does not offer serverless compute".into(),
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
            SshDeploymentConfig::crusoe(),
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
    fn parses_running_vm() {
        let json = serde_json::json!({
            "id": "vm-abc123",
            "state": "STATE_RUNNING",
            "type": "a100.1x",
            "location": "us-east1",
            "network_interfaces": [{
                "ips": [{
                    "public_ipv4": { "address": "203.0.113.20" },
                    "private_ipv4": { "address": "10.0.0.5" }
                }]
            }]
        });
        let instance = CrusoeAdapter::parse_vm(&json).unwrap();
        assert_eq!(instance.id, "vm-abc123");
        assert_eq!(instance.instance_type, "a100.1x");
        assert_eq!(instance.public_ip.as_deref(), Some("203.0.113.20"));
        assert_eq!(instance.private_ip.as_deref(), Some("10.0.0.5"));
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::Crusoe);
    }

    #[test]
    fn parses_creating_vm_as_starting() {
        let json = serde_json::json!({
            "id": "vm-new",
            "state": "STATE_CREATING",
            "type": "a100.2x",
            "location": "us-central1",
            "network_interfaces": []
        });
        let instance = CrusoeAdapter::parse_vm(&json).unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
    }

    #[test]
    fn parses_deleted_vm_as_terminated() {
        let json = serde_json::json!({
            "id": "vm-dead",
            "state": "STATE_DELETED",
            "type": "a100.1x",
            "location": "us-northwest1",
            "network_interfaces": []
        });
        let instance = CrusoeAdapter::parse_vm(&json).unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }
}
