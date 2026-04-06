//! Vast.ai `CloudProviderAdapter` — spot GPU marketplace.
//!
//! Provisioning flow:
//! 1. POST `/api/v0/bundles/` with a search query to find matching offers.
//! 2. Pick the cheapest offer within `max_price_per_hour` and above `min_reliability`.
//! 3. Call `/api/v0/asks/{offer_id}/` to rent the instance (PUT with config).
//! 4. Poll `/api/v0/instances/` until the instance is `running` with an SSH host/port.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, poll_until, require_public_ip,
};
use crate::providers::vast_ai::VastAiInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://console.vast.ai/api/v0";
const INSTANCE_READY_TIMEOUT: Duration = Duration::from_secs(900);
const INSTANCE_POLL_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_IMAGE: &str = "pytorch/pytorch:2.1.0-cuda11.8-cudnn8-runtime";

pub struct VastAiAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    max_price_per_hour: f64,
    min_reliability: f64,
}

impl VastAiAdapter {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("VAST_AI_API_KEY")
            .map_err(|_| Error::Other("VAST_AI_API_KEY environment variable not set".into()))?;
        let max_price_per_hour = std::env::var("VAST_AI_MAX_PRICE_PER_HOUR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2.0);
        let min_reliability = std::env::var("VAST_AI_MIN_RELIABILITY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.95);
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::vast_ai(api_key),
            max_price_per_hour,
            min_reliability,
        })
    }

    /// Search for cheapest eligible offer matching the query.
    async fn find_cheapest_offer(&self, query: serde_json::Value) -> Result<i64> {
        // Vast.ai's `/bundles/` search endpoint is POST per the official REST API
        // documentation (https://docs.vast.ai/api-reference/search/search-offers).
        // Older third-party docs mistakenly listed PUT; using PUT returns 4xx.
        let response = self
            .http
            .authenticated_request(
                reqwest::Method::POST,
                &format!("{BASE_URL}/bundles/"),
                &self.auth,
                Some(serde_json::json!({ "q": query })),
            )
            .await
            .map_err(|e| Error::HttpError(format!("Vast.ai search: {e}")))?;
        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::HttpError(format!("Vast.ai search failed: {body}")));
        }
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::HttpError(format!("Vast.ai search parse: {e}")))?;
        let offers = json
            .get("offers")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::HttpError("Vast.ai search: no offers field".into()))?;
        let offer = offers
            .iter()
            .find(|o| {
                let price = o
                    .get("dph_total")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(f64::MAX);
                let reliability = o
                    .get("reliability2")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                price <= self.max_price_per_hour && reliability >= self.min_reliability
            })
            .ok_or_else(|| {
                Error::HttpError(format!(
                    "Vast.ai: no offers within price={:.2}/hr reliability>={:.2}",
                    self.max_price_per_hour, self.min_reliability
                ))
            })?;
        offer
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::HttpError("Vast.ai offer missing id".into()))
    }

    async fn fetch_instance(&self, instance_id: i64) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/instances/");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.get(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    let json: serde_json::Value = response
                        .json()
                        .await
                        .map_err(|e| Error::HttpError(format!("Vast.ai instances parse: {e}")))?;
                    let instances = json
                        .get("instances")
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| Error::HttpError("Vast.ai: no instances array".into()))?;
                    return instances
                        .iter()
                        .find(|i| i.get("id").and_then(|v| v.as_i64()) == Some(instance_id))
                        .cloned()
                        .ok_or_else(|| {
                            Error::HttpError(format!("Vast.ai instance {instance_id} not found"))
                        });
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Vast.ai GET instances: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Vast.ai list instances failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Vast.ai GET instances: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn parse_instance(value: &serde_json::Value) -> Result<ProvisionedInstance> {
        let id = value
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::HttpError("Vast.ai instance missing id".into()))?
            .to_string();
        let gpu_name = value
            .get("gpu_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let num_gpus = value.get("num_gpus").and_then(|v| v.as_i64()).unwrap_or(1);
        let instance_type = format!("{num_gpus}x {gpu_name}");
        let region = value
            .get("geolocation")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let public_ip = value
            .get("public_ipaddr")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match value
            .get("actual_status")
            .or_else(|| value.get("cur_state"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
        {
            "running" => InstanceStatus::Running,
            "loading" | "scheduling" | "creating" => InstanceStatus::Starting,
            "stopped" | "offline" => InstanceStatus::Stopped,
            "exited" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::VastAi,
            instance_type,
            region,
            public_ip,
            private_ip: None,
            status,
        })
    }

    async fn wait_for_running(&self, instance_id: i64) -> Result<ProvisionedInstance> {
        poll_until(
            "Vast.ai instance",
            INSTANCE_POLL_INTERVAL,
            INSTANCE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_instance(instance_id).await?;
                let instance = Self::parse_instance(&raw)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Vast.ai instance terminated before running".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for VastAiAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        _region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        // `instance_type` carries the serialized search query from the mapper.
        let query: serde_json::Value = if instance_type.is_empty() {
            VastAiInstanceMapper::build_query(
                &ResourceSpec::basic(),
                Some(self.max_price_per_hour),
                Some(self.min_reliability),
            )
        } else {
            let mut base: serde_json::Value = serde_json::from_str(instance_type)
                .map_err(|e| Error::HttpError(format!("Vast.ai query parse: {e}")))?;
            base["dph_total"] = serde_json::json!({ "lte": self.max_price_per_hour });
            base["reliability2"] = serde_json::json!({ "gte": self.min_reliability });
            base
        };

        let offer_id = self.find_cheapest_offer(query).await?;
        info!(%offer_id, "Selected Vast.ai offer");

        let rent_body = serde_json::json!({
            "client_id": "me",
            "image": DEFAULT_IMAGE,
            "disk": 50,
            "label": format!("blueprint-{}", uuid::Uuid::new_v4()),
            "runtype": "ssh_direc",
        });
        let rent_url = format!("{BASE_URL}/asks/{offer_id}/");
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
                        reqwest::Method::PUT,
                        &rent_url,
                        &self.auth,
                        Some(rent_body.clone()),
                    )
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        result =
                            Some(response.json::<serde_json::Value>().await.map_err(|e| {
                                Error::HttpError(format!("Vast.ai rent parse: {e}"))
                            })?);
                        break;
                    }
                    Ok(response)
                        if response.status() == 429 || response.status().is_server_error() =>
                    {
                        last_err = Some(Error::HttpError(format!(
                            "Vast.ai rent: transient {}",
                            response.status()
                        )));
                        continue;
                    }
                    Ok(response) => {
                        let body = response.text().await.unwrap_or_default();
                        return Err(Error::HttpError(format!("Vast.ai rent failed: {body}")));
                    }
                    Err(e) => {
                        last_err = Some(Error::HttpError(format!("Vast.ai rent: {e}")));
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
            .get("new_contract")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::HttpError("Vast.ai rent: no new_contract".into()))?;

        info!(%instance_id, %offer_id, "Rented Vast.ai instance");
        self.wait_for_running(instance_id).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let id: i64 = instance_id
            .parse()
            .map_err(|_| Error::Other(format!("Invalid Vast.ai instance id: {instance_id}")))?;
        let url = format!("{BASE_URL}/instances/{id}/");
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.delete(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated Vast.ai instance");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Vast.ai instance already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Vast.ai terminate: transient {}",
                        response.status()
                    )));
                    continue;
                }
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::HttpError(format!(
                        "Vast.ai terminate failed: {body}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::HttpError(format!("Vast.ai terminate: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        let id: i64 = instance_id
            .parse()
            .map_err(|_| Error::Other(format!("Invalid Vast.ai instance id: {instance_id}")))?;
        match self.fetch_instance(id).await {
            Ok(raw) => Ok(Self::parse_instance(&raw)?.status),
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Vast.ai instance status");
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
                let selection = VastAiInstanceMapper::map(resource_spec);
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
                            provider = "vast_ai",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "vast_ai",
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
                "Vast.ai only supports VirtualMachine deployment targets".into(),
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
            SshDeploymentConfig::vast_ai(),
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
    fn parses_running_vast_instance() {
        let json = serde_json::json!({
            "id": 12345,
            "actual_status": "running",
            "public_ipaddr": "198.51.100.1",
            "gpu_name": "RTX 4090",
            "num_gpus": 2,
            "geolocation": "US-CA"
        });
        let instance = VastAiAdapter::parse_instance(&json).unwrap();
        assert_eq!(instance.id, "12345");
        assert_eq!(instance.instance_type, "2x RTX 4090");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::VastAi);
    }

    #[test]
    fn parses_scheduling_as_starting() {
        let json = serde_json::json!({
            "id": 1,
            "actual_status": "scheduling",
            "gpu_name": "A100",
            "num_gpus": 1
        });
        let instance = VastAiAdapter::parse_instance(&json).unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
    }

    #[test]
    fn parses_exited_as_terminated() {
        let json = serde_json::json!({
            "id": 1,
            "actual_status": "exited",
            "gpu_name": "A100",
            "num_gpus": 1
        });
        let instance = VastAiAdapter::parse_instance(&json).unwrap();
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
            "vast_ai",
            || async {
                Ok(ProvisionedInstance {
                    id: "12345".into(),
                    provider: CloudProvider::VastAi,
                    instance_type: "1x A100".into(),
                    region: "US-CA".into(),
                    public_ip: Some("1.2.3.4".into()),
                    private_ip: None,
                    status: InstanceStatus::Running,
                })
            },
            |_| async { Err(Error::Other("simulated deploy failure".into())) },
            |id| async move {
                assert_eq!(id, "12345");
                cleaned_clone.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;
        assert!(result.is_err());
        assert!(cleaned.load(Ordering::SeqCst));
    }
}
