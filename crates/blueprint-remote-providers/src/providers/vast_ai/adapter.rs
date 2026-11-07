//! Vast native offers and one identity-scoped container lifecycle.
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode,
};
use crate::security::{
    ApiAuthentication, SecureHttpClient, canonical_public_key, parse_public_key,
};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::HashSet;

const BASE_URL: &str = "https://console.vast.ai/api/v0";
pub struct VastAiAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    min_reliability: f64,
}
impl VastAiAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("VAST_AI_API_KEY")
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderNotConfigured))?;
        let mut adapter = Self::with_credentials(key)?;
        if let Ok(value) = std::env::var("VAST_AI_MIN_RELIABILITY") {
            adapter.min_reliability = value
                .parse()
                .ok()
                .filter(|n: &f64| n.is_finite() && (0.0..=1.0).contains(n))
                .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        }
        Ok(adapter)
    }
    pub(crate) fn credential_binding(&self) -> Result<(String, Option<String>)> {
        Ok((
            crate::managed::authorization::credential_digest(
                &self.auth,
                &(&self.base_url, self.min_reliability),
            )?,
            None,
        ))
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Self::with_endpoint(api_key, BASE_URL)
    }
    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        Ok(Self {
            http: SecureHttpClient::with_endpoint(endpoint)?,
            auth: ApiAuthentication::vast_ai(api_key),
            base_url: endpoint.trim_end_matches('/').into(),
            min_reliability: 0.95,
        })
    }
    fn constraints(spec: &ResourceSpec) -> Result<()> {
        crate::managed::bootstrap::workload_environment(spec)?;
        if let Some(key) = &spec.ssh_public_key {
            parse_public_key(key)?;
            if spec.command.as_ref().is_none_or(|command| {
                command.is_empty()
                    || command[0].is_empty()
                    || command[0].starts_with('-')
                    || command.iter().any(|arg| arg.contains('\0'))
            }) {
                return Err(Error::Provider(ProviderErrorCode::Unsupported));
            }
        }
        if spec.tee_required
            || spec.source_network.is_some()
            || spec.volume_gb.is_some()
            || spec.volume_id.is_some()
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        Ok(())
    }
    async fn search(
        &self,
        spec: &ResourceSpec,
        spot: bool,
        id: Option<&str>,
    ) -> Result<Vec<Value>> {
        let mut query = json!({"verified":{"eq":true},"rentable":{"eq":true},"rented":{"eq":false},
            "num_gpus":{"eq":spec.gpu_count},"cpu_cores":{"gte":spec.cpu},"cpu_ram":{"gte":f64::from(spec.memory_gb)*1000.0},
            "disk_space":{"gte":spec.storage_gb},"allocated_storage":spec.storage_gb,
            "reliability2":{"gte":self.min_reliability},"type":if spot {"bid"} else {"on-demand"},"limit":65535});
        if let Some(class) = spec
            .gpu_class
            .as_deref()
            .and_then(blueprint_core::gpu::find)
        {
            let sku = class
                .sku("vast-ai", spec.gpu_count.unwrap_or(0))
                .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
            query["gpu_name"] = json!({"eq":sku});
        }
        if let Some(id) = id {
            query["id"] = json!({"eq":id.parse::<u64>().map_err(|_|Error::Provider(ProviderErrorCode::InvalidRequest))?});
        }
        let response = self
            .http
            .post_json(&format!("{}/bundles/", self.base_url), &self.auth, query)
            .await?;
        let document: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        document["offers"]
            .as_array()
            .cloned()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))
    }
    fn price(raw: &Value, spot: bool) -> Option<f64> {
        let price = if spot {
            let bid = raw["min_bid"]
                .as_f64()
                .filter(|bid| bid.is_finite() && *bid > 0.0)?;
            let disk = raw["storage_total_cost"]
                .as_f64()
                .filter(|disk| disk.is_finite() && *disk >= 0.0)?;
            bid + disk
        } else {
            raw["dph_total"].as_f64()?
        };
        (price.is_finite() && price > 0.0).then_some(price)
    }
    fn instance(raw: &Value) -> Result<ProvisionedInstance> {
        Ok(ProvisionedInstance {
            id: raw["id"]
                .as_u64()
                .filter(|id| *id > 0)
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
                .to_string(),
            provider: CloudProvider::VastAi,
            instance_type: format!(
                "{}x {}",
                raw["num_gpus"].as_u64().unwrap_or(0),
                raw["gpu_name"].as_str().unwrap_or("")
            ),
            region: raw["geolocation"].as_str().unwrap_or("").into(),
            public_ip: raw["public_ipaddr"].as_str().map(str::to_owned),
            private_ip: None,
            status: match raw["actual_status"].as_str() {
                Some("running") => InstanceStatus::Running,
                Some("loading" | "scheduling" | "creating") => InstanceStatus::Starting,
                Some("stopped" | "offline") => InstanceStatus::Stopped,
                Some("exited") => InstanceStatus::Terminated,
                _ => InstanceStatus::Unknown,
            },
        })
    }
    fn managed(raw: &Value) -> Result<Option<ManagedResource>> {
        let Some(metadata) = ManagedMetadata::from_label(raw["label"].as_str().unwrap_or(""))?
        else {
            return Ok(None);
        };
        let count = raw["num_gpus"].as_u64().and_then(|n| u32::try_from(n).ok());
        let class = blueprint_core::gpu::catalog().into_iter().find(|class| {
            count.is_some_and(|count| {
                class
                    .sku("vast-ai", count)
                    .is_some_and(|sku| Some(sku) == raw["gpu_name"].as_str())
            })
        });
        let mut resource = metadata.bind(Self::instance(raw)?, ManagedResourceKind::Instance);
        resource.gpu_class = class.as_ref().map(|class| class.id.clone());
        resource.gpu_vram_gb = class.map(|class| class.vram_gb);
        resource.gpu_count = count;
        resource.hourly_cost_usd = raw["dph_total"]
            .as_f64()
            .filter(|price| price.is_finite() && *price > 0.0);
        resource.created_at = raw["start_date"]
            .as_f64()
            .filter(|time| time.is_finite() && *time > 0.0)
            .and_then(|time| chrono::DateTime::from_timestamp(time as i64, 0));
        resource.preempted = raw["is_bid"].as_bool() == Some(true)
            && raw["actual_status"] == "stopped"
            && raw["intended_status"] == "running";
        if let (Some(host), Some(port)) = (
            raw["ssh_host"].as_str(),
            raw["ssh_port"]
                .as_u64()
                .filter(|port| (1..=65535).contains(port)),
        ) {
            let candidate = format!("ssh://root@{host}:{port}");
            if let Ok(url) = url::Url::parse(&candidate) {
                if url.host_str() == Some(host)
                    && url.password().is_none()
                    && url.path().is_empty()
                    && url.query().is_none()
                    && url.fragment().is_none()
                {
                    resource.ssh_endpoint = Some(candidate);
                }
            }
        }
        Ok(Some(resource))
    }
    async fn has_instance_key(&self, path: &str, expected: &str) -> Result<bool> {
        let response = self.http.get(path, &self.auth).await?;
        let document: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        if document["success"].as_bool() != Some(true) {
            return Err(Error::Provider(ProviderErrorCode::Indeterminate));
        }
        let keys: Value = match &document["ssh_keys"] {
            Value::String(encoded) => serde_json::from_str(encoded)
                .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?,
            value => value.clone(),
        };
        let keys = keys
            .as_array()
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let mut present = false;
        for key in keys {
            let key = key["public_key"]
                .as_str()
                .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
            present |= canonical_public_key(key)? == expected;
        }
        Ok(present)
    }

    async fn raw_instance(&self, id: &str) -> Result<Option<Value>> {
        let id = id
            .parse::<u64>()
            .ok()
            .filter(|id| *id > 0)
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let response = self
            .http
            .get(&format!("{}/instances/{id}/", self.base_url), &self.auth)
            .await?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let document: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        document["instances"]
            .as_object()
            .map(|_| Some(document["instances"].clone()))
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))
    }
}
#[async_trait]
impl ManagedProviderAdapter for VastAiAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        Self::constraints(spec)?;
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let classes = blueprint_core::gpu::catalog();
        let mut offers = Vec::new();
        for spot in [true, false] {
            if spot && !spec.allow_spot {
                continue;
            }
            for raw in self.search(spec, spot, None).await? {
                let Some(id) = raw["id"].as_u64().filter(|id| *id > 0) else {
                    continue;
                };
                if raw["num_gpus"].as_u64() != Some(u64::from(count))
                    || raw["rentable"].as_bool() != Some(true)
                    || raw["rented"].as_bool() != Some(false)
                    || (raw["verified"].as_bool() != Some(true)
                        && raw["verification"].as_str() != Some("verified"))
                    || raw["reliability2"]
                        .as_f64()
                        .is_none_or(|r| !r.is_finite() || r < self.min_reliability)
                    || raw["cpu_cores_effective"]
                        .as_f64()
                        .or_else(|| raw["cpu_cores"].as_f64())
                        .is_none_or(|cpu| cpu < f64::from(spec.cpu))
                    || raw["cpu_ram"]
                        .as_f64()
                        .is_none_or(|ram| ram / 1000.0 < f64::from(spec.memory_gb))
                    || raw["disk_space"]
                        .as_f64()
                        .is_none_or(|disk| disk < f64::from(spec.storage_gb))
                {
                    continue;
                }
                let Some(class) = classes.iter().find(|class| {
                    class
                        .sku("vast-ai", count)
                        .is_some_and(|sku| Some(sku) == raw["gpu_name"].as_str())
                }) else {
                    continue;
                };
                if spec.gpu_class.as_ref().is_some_and(|id| id != &class.id)
                    || spec.gpu_vram_gb.is_some_and(|vram| class.vram_gb < vram)
                {
                    continue;
                }
                let Some(location) = raw["geolocation"]
                    .as_str()
                    .filter(|location| !location.is_empty())
                else {
                    continue;
                };
                if !region.is_empty() && location != region {
                    continue;
                }
                let Some(price) = Self::price(&raw, spot) else {
                    continue;
                };
                offers.push(NativeOffer {
                    provider: CloudProvider::VastAi,
                    native_id: id.to_string(),
                    instance_type: format!("{count}x {}", raw["gpu_name"].as_str().unwrap_or("")),
                    region: location.into(),
                    gpu_class: class.id.clone(),
                    gpu_count: count,
                    hourly_cost_usd: price,
                    spot,
                    pricing_source: format!("{}/bundles/", self.base_url),
                    underlying: None,
                });
            }
        }
        Ok(offers)
    }
    async fn create(
        &self,
        spec: &ResourceSpec,
        offer: &NativeOffer,
        metadata: &ManagedMetadata,
    ) -> Result<ManagedResource> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        Self::constraints(spec)?;
        let image = spec
            .image
            .as_ref()
            .filter(|image| {
                !image.is_empty()
                    && !image.starts_with('-')
                    && !image.chars().any(|c| c.is_whitespace() || c.is_control())
            })
            .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
        metadata.check_deadline()?;
        let mut native_env = crate::managed::bootstrap::workload_environment(spec)?;
        for port in &spec.ports {
            native_env.insert(format!("-p {port}:{port}"), "1".into());
        }
        let mut body = json!({"client_id":"me","image":image,"disk":spec.storage_gb,"label":metadata.label()?,"runtype":"args","target_state":"running",
            "env":native_env});
        if spec.ssh_public_key.is_some() {
            body["runtype"] = json!("ssh_direc ssh_proxy");
            let command = spec
                .command
                .as_ref()
                .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
            let arguments = command
                .iter()
                .map(|arg| shell_escape::escape(arg.as_str().into()).into_owned())
                .collect::<Vec<_>>()
                .join(" ");
            body["onstart"] = json!(format!("exec {arguments}"));
        } else if let Some(command) = &spec.command {
            body["args"] = json!(command);
        }
        if offer.spot {
            let raw = self
                .search(spec, true, Some(offer.native_id.as_str()))
                .await?
                .into_iter()
                .find(|raw| {
                    raw["id"].as_u64().map(|id| id.to_string()).as_deref()
                        == Some(offer.native_id.as_str())
                })
                .ok_or(Error::Provider(ProviderErrorCode::NoStock))?;
            if Self::price(&raw, true).is_none_or(|price| price > offer.hourly_cost_usd) {
                return Err(Error::Provider(ProviderErrorCode::NoStock));
            }
            body["price"] = raw["min_bid"].clone();
        }
        metadata.check_deadline()?;
        let response = self
            .http
            .authenticated_request(
                reqwest::Method::PUT,
                &format!(
                    "{}/asks/{}/",
                    self.base_url,
                    urlencoding::encode(&offer.native_id)
                ),
                &self.auth,
                Some(body),
            )
            .await?;
        let document: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        if document["success"].as_bool() != Some(true) {
            return Err(Error::Provider(ProviderErrorCode::Indeterminate));
        }
        let id = document["new_contract"]
            .as_u64()
            .filter(|id| *id > 0)
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let mut resource = metadata.clone().bind(
            ProvisionedInstance {
                id: id.to_string(),
                provider: CloudProvider::VastAi,
                instance_type: offer.instance_type.clone(),
                region: offer.region.clone(),
                public_ip: None,
                private_ip: None,
                status: InstanceStatus::Starting,
            },
            ManagedResourceKind::Instance,
        );
        resource.gpu_class = Some(offer.gpu_class.clone());
        resource.gpu_count = Some(offer.gpu_count);
        resource.gpu_vram_gb =
            blueprint_core::gpu::find(&offer.gpu_class).map(|class| class.vram_gb);
        resource.hourly_cost_usd = Some(offer.hourly_cost_usd);
        Ok(resource)
    }
    async fn complete(
        &self,
        spec: &ResourceSpec,
        resource: ManagedResource,
    ) -> Result<ManagedResource> {
        let Some(key) = spec.ssh_public_key.as_deref() else {
            return Ok(resource);
        };
        let key = canonical_public_key(key)?;
        let raw = self
            .raw_instance(&resource.instance.id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let observed = Self::managed(&raw)?.ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if observed.instance.id != resource.instance.id
            || observed.namespace != resource.namespace
            || observed.lease_id != resource.lease_id
            || observed.request_fingerprint != resource.request_fingerprint
        {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        if !matches!(
            raw["image_runtype"].as_str(),
            Some("ssh" | "ssh_proxy" | "ssh_direc ssh_proxy")
        ) {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let path = format!(
            "{}/instances/{}/ssh/",
            self.base_url,
            urlencoding::encode(&resource.instance.id)
        );
        if !self.has_instance_key(&path, &key).await? {
            let response = self
                .http
                .post_json(&path, &self.auth, json!({"ssh_key":key}))
                .await?;
            let document: Value = response
                .json()
                .await
                .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
            if document["success"].as_bool() != Some(true)
                || !self.has_instance_key(&path, &key).await?
            {
                return Err(Error::Provider(ProviderErrorCode::Indeterminate));
            }
        }
        Ok(observed)
    }

    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        let Some(raw) = self.raw_instance(id).await? else {
            return Ok(None);
        };
        let resource = Self::managed(&raw)?.ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }
    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let base = self.base_url.strip_suffix("/v0").unwrap_or(&self.base_url);
        let mut url = format!("{base}/v1/instances?limit=25");
        let mut seen = HashSet::new();
        let mut resources = Vec::new();
        loop {
            let response = self.http.get(&url, &self.auth).await?;
            let document: Value = response
                .json()
                .await
                .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
            for raw in document["instances"]
                .as_array()
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
            {
                if let Some(resource) = Self::managed(raw)? {
                    if resource.namespace == namespace {
                        resources.push(resource);
                    }
                }
            }
            let Some(token) = document["next_token"]
                .as_str()
                .filter(|token| !token.is_empty())
            else {
                break;
            };
            if !seen.insert(token.to_owned()) {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            url = format!(
                "{base}/v1/instances?limit=25&after_token={}",
                urlencoding::encode(token)
            );
        }
        Ok(resources)
    }
    async fn destroy(
        &self,
        identity: &ManagedLeaseIdentity,
        resource: &ManagedResource,
    ) -> Result<()> {
        if resource.kind != ManagedResourceKind::Instance {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let id = resource.instance.id.as_str();
        if <Self as ManagedProviderAdapter>::status(self, identity, id)
            .await?
            .is_none()
        {
            return Ok(());
        }
        self.http
            .delete(
                &format!("{}/instances/{}/", self.base_url, urlencoding::encode(id)),
                &self.auth,
            )
            .await?;
        Ok(())
    }
}
#[async_trait]
impl CloudProviderAdapter for VastAiAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::instance(
            &self
                .raw_instance(id)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::NotFound))?,
        )
    }
}
