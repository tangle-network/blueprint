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
use crate::providers::common::gpu_adapter::{build_http_client, deploy_via_ssh, require_public_ip};
use crate::providers::vast_ai::VastAiInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://console.vast.ai/api/v0";

pub struct VastAiAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    max_price_per_hour: f64,
    min_reliability: f64,
}

impl VastAiAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("VAST_AI_API_KEY")
            .map_err(|_| Error::ConfigurationError("VAST_AI_API_KEY is required".into()))?;
        let mut adapter = Self::with_credentials(key)?;
        adapter.max_price_per_hour = std::env::var("VAST_AI_MAX_PRICE_PER_HOUR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(f64::MAX);
        Ok(adapter)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::vast_ai(api_key),
            base_url: BASE_URL.into(),
            max_price_per_hour: f64::MAX,
            min_reliability: 0.95,
        })
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        Ok(adapter)
    }

    /// Search for cheapest eligible offer matching the query.
    async fn find_cheapest_offer(&self, query: serde_json::Value) -> Result<(i64, f64)> {
        // Vast.ai's `/bundles/` search endpoint is POST per the official REST API
        // documentation (https://docs.vast.ai/api-reference/search/search-offers).
        // Older third-party docs mistakenly listed PUT; using PUT returns 4xx.
        let response = self
            .http
            .authenticated_request(
                reqwest::Method::POST,
                &format!("{}/bundles/", self.base_url),
                &self.auth,
                Some(query),
            )
            .await
            .map_err(|e| Error::HttpError(format!("Vast.ai search: {e}")))?;
        if !response.status().is_success() {
            return Err(Error::HttpError(format!(
                "Provider HTTP {}",
                response.status()
            )));
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
            .filter(|o| {
                let price = o
                    .get("dph_total")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(f64::MAX);
                let reliability = o
                    .get("reliability2")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                price.is_finite()
                    && price > 0.0
                    && price <= self.max_price_per_hour
                    && reliability >= self.min_reliability
            })
            .min_by(|a, b| {
                a["dph_total"]
                    .as_f64()
                    .unwrap_or(f64::MAX)
                    .total_cmp(&b["dph_total"].as_f64().unwrap_or(f64::MAX))
            })
            .ok_or_else(|| {
                Error::HttpError(format!(
                    "Vast.ai: no offers within price={:.2}/hr reliability>={:.2}",
                    self.max_price_per_hour, self.min_reliability
                ))
            })?;
        let id = offer["id"]
            .as_i64()
            .ok_or_else(|| Error::HttpError("Vast.ai offer missing id".into()))?;
        Ok((id, offer["dph_total"].as_f64().unwrap_or(f64::NAN)))
    }

    async fn fetch_instance(&self, id: i64) -> Result<serde_json::Value> {
        let value = crate::providers::common::gpu_adapter::api_json(
            self.http
                .get(&format!("{}/instances/", self.base_url), &self.auth)
                .await?,
        )
        .await?;
        value["instances"]
            .as_array()
            .and_then(|instances| {
                instances
                    .iter()
                    .find(|instance| instance["id"].as_i64() == Some(id))
            })
            .cloned()
            .ok_or_else(|| Error::HttpError("Vast instance not found".into()))
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
}

#[async_trait]
impl CloudProviderAdapter for VastAiAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        _region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        crate::providers::common::gpu_adapter::catalog_sku(spec, "vast-ai")?;
        let query = VastAiInstanceMapper::build_query(
            spec,
            Some(self.max_price_per_hour),
            Some(self.min_reliability),
        );
        let (_, price) = self.find_cheapest_offer(query.clone()).await?;
        crate::providers::common::gpu_adapter::quoted_selection(query.to_string(), price)
    }

    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        let instance = self
            .provision_resources(
                instance_type,
                region,
                &ResourceSpec::default().with_gpu(1),
                tee,
            )
            .await?;
        crate::providers::common::gpu_adapter::wait_for_instance(self, &instance.id).await
    }

    async fn provision_resources(
        &self,
        _instance_type: &str,
        _region: &str,
        spec: &ResourceSpec,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let sku = crate::providers::common::gpu_adapter::catalog_sku(spec, "vast-ai")?;
        // Fail closed at the trait boundary: this provider has no
        // confidential-compute / TEE attestation path. Refuse rather than
        // silently return a non-TEE instance for a require_tee request.
        if require_tee {
            return Err(Error::ConfigurationError(
                "Vast.ai does not support confidential-compute / TEE attestation; \
                 refusing require_tee"
                    .into(),
            ));
        }
        let query = VastAiInstanceMapper::build_query(
            spec,
            Some(self.max_price_per_hour),
            Some(self.min_reliability),
        );
        let (offer_id, _) = self.find_cheapest_offer(query).await?;
        info!(%offer_id, "Selected Vast.ai offer");

        let rent_body = serde_json::json!({
            "client_id": "me",
            "image": spec.image.as_deref().unwrap_or(crate::providers::common::gpu_adapter::DEFAULT_GPU_IMAGE),
            "disk": spec.storage_gb,
            "label": format!("blueprint-{}", uuid::Uuid::new_v4()),
            "runtype": "ssh_direct",
        });
        let rent_url = format!("{}/asks/{offer_id}/", self.base_url);
        let json = crate::providers::common::gpu_adapter::api_json(
            self.http
                .authenticated_request(reqwest::Method::PUT, &rent_url, &self.auth, Some(rent_body))
                .await?,
        )
        .await?;
        let instance_id = json
            .get("new_contract")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::HttpError("Vast.ai rent: no new_contract".into()))?;

        info!(%instance_id, %offer_id, "Rented Vast.ai instance");
        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::VastAi,
            instance_id.to_string(),
            sku,
            String::new(),
        ))
    }

    async fn prepare_access(
        &self,
        instance: &ProvisionedInstance,
        spec: &ResourceSpec,
    ) -> Result<()> {
        if let Some(public_key) = &spec.ssh_public_key {
            crate::providers::common::gpu_adapter::api_json(
                self.http
                    .post_json(
                        &format!("{}/instances/{}/ssh/", self.base_url, instance.id),
                        &self.auth,
                        serde_json::json!({"ssh_key":public_key}),
                    )
                    .await?,
            )
            .await?;
        }
        Ok(())
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let id = id
            .parse::<i64>()
            .map_err(|_| Error::ConfigurationError("Invalid Vast instance id".into()))?;
        Self::parse_instance(&self.fetch_instance(id).await?)
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        let id = instance
            .id
            .parse::<i64>()
            .map_err(|_| Error::ConfigurationError("Invalid Vast instance id".into()))?;
        let raw = self.fetch_instance(id).await?;
        let host = raw["ssh_host"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Vast has no SSH host".into()))?;
        let port = raw["ssh_port"]
            .as_u64()
            .ok_or_else(|| Error::HttpError("Vast has no SSH port".into()))?;
        Ok(format!("ssh://root@{host}:{port}"))
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let id: i64 = instance_id
            .parse()
            .map_err(|_| Error::Other(format!("Invalid Vast.ai instance id: {instance_id}")))?;
        let url = format!("{}/instances/{id}/", self.base_url);
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
                    return Err(Error::HttpError(format!(
                        "Provider HTTP {}",
                        response.status()
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

    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        let id = id
            .parse::<i64>()
            .map_err(|_| Error::ConfigurationError("Invalid Vast instance id".into()))?;
        let value = crate::providers::common::gpu_adapter::api_json(
            self.http
                .get(&format!("{}/instances/", self.base_url), &self.auth)
                .await?,
        )
        .await?;
        let instances = value["instances"]
            .as_array()
            .ok_or_else(|| Error::HttpError("Vast instance list missing".into()))?;
        match instances
            .iter()
            .find(|instance| instance["id"].as_i64() == Some(id))
        {
            Some(raw) => Ok(Self::parse_instance(raw)?.status),
            None => Ok(InstanceStatus::Terminated),
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
                    .provision_resources(&selection.instance_type, "", resource_spec, false)
                    .await?;
                let instance_id_for_cleanup = instance.id.clone();
                match self
                    .deploy_blueprint(&instance, blueprint_image, resource_spec, env_vars)
                    .await
                {
                    Ok(result) => Ok(result),
                    Err(deploy_err) => {
                        warn!(
                            provider = "vast-ai",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "vast-ai",
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
        self.prepare_access(instance, resource_spec).await?;
        let ready = if instance.status != InstanceStatus::Running || instance.public_ip.is_none() {
            Some(
                crate::providers::common::gpu_adapter::wait_for_instance(self, &instance.id)
                    .await?,
            )
        } else {
            None
        };
        let instance = ready.as_ref().unwrap_or(instance);
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
            // Reuse the pre-built pooled client instead of rebuilding the
            // TLS stack on every (interval-driven) health probe.
            match self
                .http
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
