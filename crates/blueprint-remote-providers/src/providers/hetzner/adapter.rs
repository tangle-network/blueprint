//! Hetzner Cloud `CloudProviderAdapter` implementation.
//!
//! Uses the REST API (`https://api.hetzner.cloud/v1`) to manage servers.

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

const BASE_URL: &str = "https://api.hetzner.cloud/v1";
const SERVER_READY_TIMEOUT: Duration = Duration::from_secs(300);
const SERVER_POLL_INTERVAL: Duration = Duration::from_secs(5);

pub struct HetznerAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_location: String,
    ssh_key_name: Option<String>,
}

impl HetznerAdapter {
    pub async fn new() -> Result<Self> {
        let api_token = std::env::var("HETZNER_API_TOKEN")
            .map_err(|_| Error::Other("HETZNER_API_TOKEN environment variable not set".into()))?;
        let default_location =
            std::env::var("HETZNER_REGION").unwrap_or_else(|_| "fsn1".to_string());
        let ssh_key_name = std::env::var("HETZNER_SSH_KEY_NAME").ok();
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::hetzner(api_token),
            default_location,
            ssh_key_name,
        })
    }

    async fn fetch_server(&self, server_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/servers/{server_id}");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.get(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    let body: serde_json::Value = response
                        .json()
                        .await
                        .map_err(|e| Error::HttpError(format!("Hetzner response parse: {e}")))?;
                    return body
                        .get("server")
                        .cloned()
                        .ok_or_else(|| Error::HttpError("Hetzner response missing server".into()));
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Hetzner GET {url}: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!("Hetzner GET {url} failed: {body}")));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Hetzner GET {url}: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn parse_server(value: &serde_json::Value) -> Result<ProvisionedInstance> {
        let id = value
            .get("id")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| Error::HttpError("Hetzner response missing id".into()))?
            .to_string();
        let instance_type = value
            .get("server_type")
            .and_then(|t| t.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = value
            .get("datacenter")
            .and_then(|d| d.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let public_ip = value
            .get("public_net")
            .and_then(|n| n.get("ipv4"))
            .and_then(|v4| v4.get("ip"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = value
            .get("private_net")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|n| n.get("ip"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match value
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("")
        {
            "running" => InstanceStatus::Running,
            "initializing" | "starting" | "rebuilding" => InstanceStatus::Starting,
            "stopping" => InstanceStatus::Stopping,
            "off" => InstanceStatus::Stopped,
            "deleting" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::Hetzner,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, server_id: &str) -> Result<ProvisionedInstance> {
        poll_until(
            "Hetzner server",
            SERVER_POLL_INTERVAL,
            SERVER_READY_TIMEOUT,
            || async {
                let raw = self.fetch_server(server_id).await?;
                let instance = Self::parse_server(&raw)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Hetzner server terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }

    async fn resolve_ssh_key_ids(&self) -> Result<Vec<u64>> {
        let Some(key_name) = &self.ssh_key_name else {
            return Ok(vec![]);
        };
        let url = format!("{BASE_URL}/ssh_keys?name={key_name}");
        let response = self
            .http
            .get(&url, &self.auth)
            .await
            .map_err(|e| Error::HttpError(format!("Hetzner SSH key lookup: {e}")))?;
        if !response.status().is_success() {
            return Ok(vec![]);
        }
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::HttpError(format!("Hetzner SSH key parse: {e}")))?;
        let ids: Vec<u64> = body
            .get("ssh_keys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|k| k.get("id").and_then(|v| v.as_u64()))
                    .collect()
            })
            .unwrap_or_default();
        Ok(ids)
    }
}

#[async_trait]
impl CloudProviderAdapter for HetznerAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("hetzner");
        let location = if region.is_empty() {
            &self.default_location
        } else {
            region
        };
        let ssh_keys = self.resolve_ssh_key_ids().await?;
        let payload = serde_json::json!({
            "name": name,
            "server_type": instance_type,
            "location": location,
            "image": "ubuntu-22.04",
            "ssh_keys": ssh_keys,
            "start_after_create": true,
            "public_net": {
                "enable_ipv4": true,
                "enable_ipv6": true,
            },
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
                            Error::HttpError(format!("Hetzner response parse: {e}"))
                        })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "Hetzner create server: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!(
                            "Hetzner create server failed: {body}"
                        )));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("Hetzner create server: {e}")));
                        continue;
                    }
                }
            }
            match result {
                Some(v) => v,
                None => return Err(last_err.unwrap()),
            }
        };

        let server_id = json
            .get("server")
            .and_then(|s| s.get("id"))
            .and_then(|v| v.as_u64())
            .ok_or_else(|| Error::HttpError("Hetzner create server: no id in response".into()))?
            .to_string();

        info!(%server_id, %instance_type, location, "Created Hetzner server");

        self.wait_for_running(&server_id).await
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
                    info!(%instance_id, "Terminated Hetzner server");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Hetzner server already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Hetzner delete server: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Hetzner delete server failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Hetzner delete server: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_server(instance_id).await {
            Ok(raw) => Ok(Self::parse_server(&raw)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Hetzner server status");
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
                let selection = crate::providers::hetzner::HetznerInstanceMapper::map(resource_spec);
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
                            provider = "hetzner",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "hetzner",
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
                "Hetzner managed Kubernetes (HKE) is not yet supported via this adapter".into(),
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
                "Hetzner does not offer serverless compute".into(),
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
            SshDeploymentConfig::hetzner(),
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
            "id": 42,
            "status": "running",
            "public_net": {
                "ipv4": { "ip": "203.0.113.10" },
                "ipv6": { "ip": "2001:db8::1" }
            },
            "server_type": { "name": "cpx31" },
            "datacenter": { "name": "fsn1-dc14" }
        });
        let instance = HetznerAdapter::parse_server(&json).unwrap();
        assert_eq!(instance.id, "42");
        assert_eq!(instance.instance_type, "cpx31");
        assert_eq!(instance.public_ip.as_deref(), Some("203.0.113.10"));
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::Hetzner);
    }

    #[test]
    fn parses_initializing_as_starting() {
        let json = serde_json::json!({
            "id": 43,
            "status": "initializing",
            "public_net": { "ipv4": { "ip": "1.2.3.4" } },
            "server_type": { "name": "cpx11" },
            "datacenter": { "name": "nbg1-dc3" }
        });
        let instance = HetznerAdapter::parse_server(&json).unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
    }

    #[test]
    fn parses_off_as_stopped() {
        let json = serde_json::json!({
            "id": 44,
            "status": "off",
            "public_net": { "ipv4": { "ip": "1.2.3.4" } },
            "server_type": { "name": "cpx11" },
            "datacenter": { "name": "hel1-dc2" }
        });
        let instance = HetznerAdapter::parse_server(&json).unwrap();
        assert_eq!(instance.status, InstanceStatus::Stopped);
    }
}
