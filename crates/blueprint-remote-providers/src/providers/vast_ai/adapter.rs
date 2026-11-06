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
    verified_only: bool,
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
            verified_only: true,
        })
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        Ok(adapter)
    }

    /// Caller-owned host policy; the default requires verified hosts and reliability >=0.95.
    pub fn with_offer_policy(mut self, min_reliability: f64, verified_only: bool) -> Result<Self> {
        if !min_reliability.is_finite() || !(0.0..=1.0).contains(&min_reliability) {
            return Err(Error::ConfigurationError(
                "Invalid Vast reliability policy".into(),
            ));
        }
        self.min_reliability = min_reliability;
        self.verified_only = verified_only;
        Ok(self)
    }

    fn query(&self, spec: &ResourceSpec, region: &str) -> Result<serde_json::Value> {
        if spec.source_network.is_some() {
            return Err(Error::ConfigurationError(
                "Vast cannot enforce caller source-network restrictions".into(),
            ));
        }
        crate::providers::common::gpu_adapter::catalog_sku(spec, "vast-ai")?;
        if spec.volume_gb.is_some_and(|size| size > 0.0) {
            return Err(Error::ConfigurationError(
                "Vast attached volumes need an existing native volume offer".into(),
            ));
        }
        let cap = spec
            .max_hourly_cost_usd
            .unwrap_or(self.max_price_per_hour)
            .min(self.max_price_per_hour);
        let mut query =
            VastAiInstanceMapper::build_query(spec, Some(cap), Some(self.min_reliability));
        if !self.verified_only {
            query.as_object_mut().unwrap().remove("verified");
        }
        if !region.is_empty() {
            query["geolocation"] = serde_json::json!({"eq":region});
        }
        query["direct_port_count"] = serde_json::json!({"gte":crate::providers::common::gpu_adapter::workload_ports(spec,false).len().max(1)});
        Ok(query)
    }

    async fn instances(&self) -> Result<Vec<serde_json::Value>> {
        let mut instances = Vec::new();
        let mut cursor: Option<String> = None;
        let mut seen = std::collections::HashSet::new();
        loop {
            let path = cursor
                .as_ref()
                .map(|c| {
                    format!(
                        "{}/instances/?after_token={}",
                        self.base_url,
                        urlencoding::encode(c)
                    )
                })
                .unwrap_or_else(|| format!("{}/instances/", self.base_url));
            let value = crate::providers::common::gpu_adapter::api_json(
                self.http.get(&path, &self.auth).await?,
            )
            .await?;
            instances.extend(
                value["instances"]
                    .as_array()
                    .ok_or_else(|| Error::HttpError("Vast instance list missing".into()))?
                    .iter()
                    .cloned(),
            );
            cursor = value["next_token"]
                .as_str()
                .filter(|c| !c.is_empty())
                .map(String::from);
            let Some(token) = &cursor else { break };
            if !seen.insert(token.clone()) {
                return Err(Error::HttpError(
                    "Vast instance listing repeated its cursor".into(),
                ));
            }
        }
        Ok(instances)
    }

    /// Search for cheapest eligible offer matching the query.
    async fn find_cheapest_offer(
        &self,
        query: serde_json::Value,
    ) -> Result<(serde_json::Value, f64)> {
        // Vast.ai's `/bundles/` search endpoint is POST per the official REST API
        // documentation (https://docs.vast.ai/api-reference/search/search-offers).
        // Older third-party docs mistakenly listed PUT; using PUT returns 4xx.
        let response = self
            .http
            .authenticated_request(
                reqwest::Method::POST,
                &format!("{}/bundles/", self.base_url),
                &self.auth,
                Some(query.clone()),
            )
            .await
            .map_err(|e| Error::HttpError(format!("Vast.ai search: {e}")))?;
        let json = crate::providers::common::gpu_adapter::api_json(response).await?;
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
                let gpu = o["gpu_name"].as_str();
                let matching_gpu = query["gpu_name"]["eq"]
                    .as_str()
                    .is_some_and(|wanted| Some(wanted) == gpu)
                    || query["gpu_name"]["in"]
                        .as_array()
                        .is_some_and(|wanted| wanted.iter().any(|wanted| wanted.as_str() == gpu));
                matching_gpu
                    && o["gpu_ram"].as_f64().is_some_and(|memory| {
                        memory >= query["gpu_ram"]["gte"].as_f64().unwrap_or(0.0)
                    })
                    && query["id"]["eq"]
                        .as_i64()
                        .is_none_or(|id| o["id"].as_i64() == Some(id))
                    && o["num_gpus"].as_u64() == query["num_gpus"]["eq"].as_u64()
                    && o["rentable"].as_bool() == Some(true)
                    && o["rented"].as_bool() == Some(false)
                    && (!self.verified_only
                        || o["verification"].as_str() == Some("verified")
                        || o["verified"].as_bool() == Some(true))
                    && price.is_finite()
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
        if offer["id"].as_i64().is_none() {
            return Err(Error::HttpError("Vast offer missing id".into()));
        }
        Ok((
            offer.clone(),
            offer["dph_total"].as_f64().unwrap_or(f64::NAN),
        ))
    }

    async fn fetch_instance(&self, id: i64) -> Result<serde_json::Value> {
        self.instances()
            .await?
            .into_iter()
            .find(|instance| instance["id"].as_i64() == Some(id))
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

fn vast_endpoint(raw: &serde_json::Value, port: u16) -> Option<String> {
    let mappings = raw["ports"][format!("{port}/tcp")].as_array()?;
    let external = mappings.iter().find_map(|m| {
        m["HostPort"]
            .as_str()
            .and_then(|p| p.parse::<u16>().ok())
            .or_else(|| m["HostPort"].as_u64().and_then(|p| u16::try_from(p).ok()))
    })?;
    let host = raw["public_ipaddr"]
        .as_str()
        .or_else(|| raw["public_ip"].as_str())?;
    let host = if host.parse::<std::net::Ipv6Addr>().is_ok() {
        format!("[{host}]")
    } else {
        host.to_string()
    };
    let url = url::Url::parse(&format!("https://{host}:{external}")).ok()?;
    (external > 0 && url.username().is_empty() && url.password().is_none())
        .then(|| url.to_string().trim_end_matches('/').to_string())
}

#[async_trait]
impl CloudProviderAdapter for VastAiAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        let query = self.query(spec, region)?;
        let (offer, price) = self.find_cheapest_offer(query).await?;
        let sku = offer["gpu_name"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Vast offer has no GPU model".into()))?;
        let class = crate::providers::common::gpu_adapter::catalog_class_for_memory(
            spec,
            "vast-ai",
            sku,
            offer["gpu_ram"].as_f64().unwrap_or(f64::NAN),
        )?;
        crate::providers::common::gpu_adapter::quoted_gpu_selection(
            serde_json::json!({"offer_id":offer["id"],"gpu_name":sku,"quoted_price":price})
                .to_string(),
            price,
            class,
        )
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
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let sku = crate::providers::common::gpu_adapter::catalog_sku(spec, "vast-ai")?;
        if spec.source_network.is_some() {
            return Err(Error::ConfigurationError(
                "Vast cannot enforce caller source-network restrictions".into(),
            ));
        }
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
        let tls = crate::providers::common::gpu_adapter::worker_tls_enabled(spec)?;
        if spec.env.contains_key("TANGLE_GPU_WORKER_TOKEN") && !tls {
            return Err(Error::ConfigurationError(
                "Vast worker tokens require configured TLS".into(),
            ));
        }
        let label = if let Some(ownership) = &spec.ownership {
            let label = crate::providers::common::gpu_adapter::managed_resource_name(
                &ownership.namespace,
                &ownership.lease_id,
            )?;
            let existing: Vec<_> = self
                .instances()
                .await?
                .into_iter()
                .filter(|instance| instance["label"].as_str() == Some(label.as_str()))
                .collect();
            if existing.len() > 1 {
                return Err(Error::ConfigurationError(
                    "Multiple Vast instances claim this lease identity".into(),
                ));
            }
            if let Some(existing) = existing.first() {
                if existing["env"]["TANGLE_GPU_REQUEST_FINGERPRINT"].as_str()
                    != Some(crate::managed::request_fingerprint(spec)?.as_str())
                {
                    return Err(Error::ConfigurationError(
                        "Managed Vast resource conflicts with requested allocation".into(),
                    ));
                }
                return Self::parse_instance(existing);
            }
            label
        } else {
            format!("blueprint-{}", uuid::Uuid::new_v4())
        };
        let mut query = self.query(spec, region)?;
        let selected: serde_json::Value = serde_json::from_str(instance_type).unwrap_or_default();
        if let Some(offer) = selected["offer_id"].as_i64() {
            query["id"] = serde_json::json!({"eq":offer});
        }
        let (offer, price) = self.find_cheapest_offer(query).await?;
        let offer_id = offer["id"]
            .as_i64()
            .ok_or_else(|| Error::HttpError("Vast offer has no id".into()))?;
        if selected["quoted_price"]
            .as_f64()
            .is_some_and(|quoted| price > quoted)
        {
            return Err(Error::ConfigurationError(
                "Vast offer price increased after quote".into(),
            ));
        }
        let mut environment = crate::providers::common::gpu_adapter::workload_environment(
            spec,
            tls.then_some("pinned"),
        )?;
        let ports = crate::providers::common::gpu_adapter::workload_ports(spec, false);
        for port in ports {
            environment.insert(format!("-p {port}:{port}"), "1".into());
        }
        let mut rent_body = serde_json::json!({
            "client_id":"me","image":spec.image.as_deref().unwrap_or(crate::providers::common::gpu_adapter::DEFAULT_GPU_IMAGE),
            "disk":spec.storage_gb,"label":label,"env":environment,
            "runtype":if spec.image.is_some() || spec.command.is_some() || spec.endpoint_port.is_some(){"args"}else{"ssh_direct"},
            "target_state":"running","cancel_unavail":true,
        });
        if let Some(command) = &spec.command {
            rent_body["args"] = serde_json::json!(command);
        }
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
            region.into(),
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
        match self
            .instances()
            .await?
            .iter()
            .find(|instance| instance["id"].as_i64() == Some(id))
        {
            Some(raw) => Ok(Self::parse_instance(raw)?.status),
            None => Ok(InstanceStatus::Terminated),
        }
    }
    fn application_tls_mode(&self, spec: &ResourceSpec) -> Option<crate::infra::types::TlsMode> {
        crate::providers::common::gpu_adapter::worker_tls_enabled(spec)
            .ok()
            .filter(|tls| *tls)
            .map(|_| crate::infra::types::TlsMode::Pinned)
    }
    async fn application_endpoint(
        &self,
        instance: &ProvisionedInstance,
        spec: &ResourceSpec,
    ) -> Result<Option<String>> {
        let Some(port) = spec.endpoint_port else {
            return Ok(None);
        };
        if !crate::providers::common::gpu_adapter::worker_tls_enabled(spec)? {
            return Ok(None);
        }
        let id = instance
            .id
            .parse::<i64>()
            .map_err(|_| Error::ConfigurationError("Invalid Vast instance id".into()))?;
        Ok(vast_endpoint(&self.fetch_instance(id).await?, port))
    }
    async fn list_managed(
        &self,
        namespace: &str,
    ) -> Result<Vec<crate::infra::types::ManagedResource>> {
        use crate::providers::common::gpu_adapter::{
            managed_lease_id, managed_resource_prefix, provider_created_at, provider_hourly_price,
        };
        managed_resource_prefix(namespace)?;
        let mut resources = Vec::new();
        for raw in self.instances().await? {
            let Some(lease_id) = raw["label"]
                .as_str()
                .and_then(|name| managed_lease_id(name, namespace))
            else {
                continue;
            };
            let mut spec = ResourceSpec::default();
            spec.env = serde_json::from_value(raw["env"].clone()).unwrap_or_default();
            let tls =
                crate::providers::common::gpu_adapter::worker_tls_enabled(&spec).unwrap_or(false);
            let port = spec
                .env
                .get("TANGLE_GPU_LEASE_ENDPOINT_PORT")
                .and_then(|p| p.parse().ok());
            resources.push(crate::infra::types::ManagedResource {
                instance: Self::parse_instance(&raw)?,
                namespace: namespace.into(),
                lease_id,
                kind: crate::infra::types::ManagedResourceKind::Instance,
                endpoint: port
                    .filter(|_| tls)
                    .and_then(|port| vast_endpoint(&raw, port)),
                tls_mode: tls.then_some(crate::infra::types::TlsMode::Pinned),
                request_fingerprint: raw["env"]["TANGLE_GPU_REQUEST_FINGERPRINT"]
                    .as_str()
                    .map(String::from),
                expires_at: None,
                expiry_source: None,
                hourly_cost_usd: provider_hourly_price(&raw["dph_total"]),
                created_at: provider_created_at(&raw["start_date"]),
            });
        }
        Ok(resources)
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
