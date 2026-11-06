//! Lium native executor rent, private templates and namespace-scoped recovery.
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::BTreeMap;

const BASE_URL: &str = "https://lium.io/api";
pub struct BittensorLiumAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    default_region: Option<String>,
}
impl BittensorLiumAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("LIUM_API_KEY")
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderNotConfigured))?;
        let mut adapter = Self::with_credentials(key)?;
        adapter.default_region = std::env::var("LIUM_REGION").ok();
        Ok(adapter)
    }
    pub fn with_credentials(api_key: String) -> Result<Self> {
        Self::with_endpoint(api_key, BASE_URL)
    }
    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        Ok(Self {
            http: SecureHttpClient::with_endpoint(endpoint)?,
            auth: ApiAuthentication::bittensor_lium(api_key),
            base_url: endpoint.trim_end_matches('/').into(),
            default_region: None,
        })
    }
    fn constraints(spec: &ResourceSpec) -> Result<()> {
        crate::managed::bootstrap::workload_environment(spec)?;
        if spec
            .max_hourly_cost_usd
            .is_some_and(|cap| cap / f64::from(spec.gpu_count.unwrap_or(1)) <= 0.01)
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        if spec.tee_required
            || spec.source_network.is_some()
            || spec.volume_gb.is_some()
            || spec.command.is_some()
            || spec.volume_id.is_some()
                && spec
                    .volume_mount_path
                    .as_deref()
                    .is_some_and(|path| path != "/mnt")
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        if spec
            .lease_duration_secs
            .is_none_or(|seconds| seconds == 0 || seconds % 3600 != 0 || seconds / 3600 > 720)
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        Ok(())
    }
    async fn get(&self, path: &str) -> Result<Option<Value>> {
        let response = self
            .http
            .get(&format!("{}/{path}", self.base_url), &self.auth)
            .await?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        response
            .json()
            .await
            .map(Some)
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))
    }
    fn id(raw: &Value) -> Result<&str> {
        raw["id"]
            .as_str()
            .filter(|id| uuid::Uuid::parse_str(id).is_ok())
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))
    }
    fn class(model: &str, count: u32) -> Option<blueprint_core::gpu::GpuClass> {
        let normalize = |name: &str| {
            name.trim()
                .to_ascii_lowercase()
                .trim_start_matches("nvidia ")
                .to_owned()
        };
        let model = normalize(model);
        blueprint_core::gpu::catalog().into_iter().find(|class| {
            class
                .sku("bittensor-lium", count)
                .is_some_and(|sku| normalize(sku) == model)
                || class.aliases.iter().any(|alias| normalize(alias) == model)
        })
    }
    fn metadata(raw: &Value) -> Result<Option<ManagedMetadata>> {
        let env = match raw["environment"].as_object() {
            Some(_) => &raw["environment"],
            None => &raw["template"]["environment"],
        };
        if env.is_null() {
            return Ok(None);
        }
        let env: BTreeMap<String, String> = serde_json::from_value(env.clone())
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        ManagedMetadata::from_env(&env)
    }
    fn instance(raw: &Value) -> Result<ProvisionedInstance> {
        Ok(ProvisionedInstance {
            id: Self::id(raw)?.into(),
            provider: CloudProvider::BittensorLium,
            instance_type: raw["gpu_name"].as_str().unwrap_or("").into(),
            region: raw["executor"]["location"]["region"]
                .as_str()
                .unwrap_or("")
                .into(),
            public_ip: raw["executor"]["executor_ip_address"]
                .as_str()
                .and_then(|ip| ip.parse::<std::net::IpAddr>().ok())
                .map(|ip| ip.to_string()),
            private_ip: None,
            status: match raw["status"].as_str() {
                Some("RUNNING") => InstanceStatus::Running,
                Some("PENDING" | "START_PENDING" | "REBOOT_PENDING") => InstanceStatus::Starting,
                Some("DELETING" | "STOP_PENDING") => InstanceStatus::Stopping,
                Some("STOPPED" | "FAILED" | "CREATION_FAILED" | "BROKEN" | "REBOOT_FAILED") => {
                    InstanceStatus::Stopped
                }
                _ => InstanceStatus::Unknown,
            },
        })
    }
    fn managed_pod(raw: &Value) -> Result<Option<ManagedResource>> {
        let Some(metadata) = Self::metadata(raw)? else {
            return Ok(None);
        };
        let count = raw["gpu_count"]
            .as_str()
            .and_then(|n| n.parse::<u32>().ok())
            .or_else(|| {
                raw["gpu_count"]
                    .as_u64()
                    .and_then(|n| u32::try_from(n).ok())
            });
        let class =
            count.and_then(|count| Self::class(raw["gpu_name"].as_str().unwrap_or(""), count));
        let mut resource = metadata.bind(Self::instance(raw)?, ManagedResourceKind::Instance);
        resource.gpu_class = class.as_ref().map(|class| class.id.clone());
        resource.gpu_vram_gb = class.map(|class| class.vram_gb);
        resource.gpu_count = count;
        resource.hourly_cost_usd = raw["price"]
            .as_f64()
            .filter(|price| price.is_finite() && *price > 0.0);
        resource.created_at = raw["created_at"]
            .as_str()
            .and_then(|date| date.parse().ok());
        resource.ssh_endpoint = raw["ssh_connect_cmd"].as_str().and_then(Self::ssh_endpoint);
        Ok(Some(resource))
    }
    fn ssh_endpoint(command: &str) -> Option<String> {
        let words = command.split_whitespace().collect::<Vec<_>>();
        if words.first() != Some(&"ssh") {
            return None;
        }
        let mut port = 22u16;
        let mut target = None;
        let mut index = 1;
        while index < words.len() {
            if words[index] == "-p" {
                index += 1;
                port = words.get(index)?.parse().ok().filter(|port| *port > 0)?;
            } else if target.replace(words[index]).is_some() {
                return None;
            }
            index += 1;
        }
        let (user, host) = target?.split_once('@')?;
        if user.is_empty()
            || !user
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return None;
        }
        let candidate = format!("ssh://{user}@{host}:{port}");
        let url = url::Url::parse(&candidate).ok()?;
        (url.host_str() == Some(host)
            && url.password().is_none()
            && url.path().is_empty()
            && url.query().is_none()
            && url.fragment().is_none())
        .then_some(candidate)
    }
    fn managed_template(raw: &Value) -> Result<Option<ManagedResource>> {
        let Some(metadata) = Self::metadata(raw)? else {
            return Ok(None);
        };
        if raw["is_private"].as_bool() != Some(true) {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(metadata.bind(
            ProvisionedInstance {
                id: Self::id(raw)?.into(),
                provider: CloudProvider::BittensorLium,
                instance_type: "template".into(),
                region: String::new(),
                public_ip: None,
                private_ip: None,
                status: InstanceStatus::Running,
            },
            ManagedResourceKind::Template,
        )))
    }
    async fn templates(&self) -> Result<Vec<Value>> {
        self.get("templates")
            .await?
            .and_then(|raw| raw.as_array().cloned())
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))
    }
}
#[async_trait]
impl ManagedProviderAdapter for BittensorLiumAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        Self::constraints(spec)?;
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let document = self
            .get("public/v1/nodes")
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let nodes = document["nodes"]
            .as_array()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let region = if region.is_empty() {
            self.default_region.as_deref().unwrap_or("")
        } else {
            region
        };
        let mut offers = Vec::new();
        for node in nodes {
            if node["available_now"].as_bool() != Some(true)
                || node["available_gpu_count"]
                    .as_u64()
                    .is_none_or(|free| free < u64::from(count))
                || node["min_rentable_gpu_count"].as_u64().map_or_else(
                    || node["gpu_count"].as_u64() != Some(u64::from(count)),
                    |minimum| u64::from(count) < minimum,
                )
                || node["cpu_count"]
                    .as_f64()
                    .is_none_or(|cpu| cpu < f64::from(spec.cpu))
                || node["ram_gb"]
                    .as_f64()
                    .is_none_or(|ram| ram < f64::from(spec.memory_gb))
                || node["disk_free_gb"]
                    .as_f64()
                    .is_none_or(|disk| disk < f64::from(spec.storage_gb))
            {
                continue;
            }
            let spot = match node["tier"].as_str() {
                Some("secure") => false,
                Some("spot") if spec.allow_spot => true,
                _ => continue,
            };
            let Some(class) = node["gpu_model"]
                .as_str()
                .and_then(|model| Self::class(model, count))
            else {
                continue;
            };
            if spec.gpu_class.as_ref().is_some_and(|id| id != &class.id)
                || spec
                    .gpu_vram_gb
                    .is_some_and(|minimum| class.vram_gb < minimum)
            {
                continue;
            }
            let Some(location) = node["region"]
                .as_str()
                .filter(|location| !location.is_empty())
            else {
                continue;
            };
            if !region.is_empty() && location != region {
                continue;
            }
            let Some(price) = node["price_per_gpu_hour"]
                .as_f64()
                .map(|price| price * f64::from(count))
                .filter(|price| price.is_finite() && *price > 0.0)
            else {
                continue;
            };
            offers.push(NativeOffer {
                provider: CloudProvider::BittensorLium,
                native_id: Self::id(node)?.into(),
                instance_type: node["gpu_model"].as_str().unwrap_or("").into(),
                region: location.into(),
                gpu_class: class.id,
                gpu_count: count,
                hourly_cost_usd: price,
                spot,
                pricing_source: format!("{}/public/v1/nodes", self.base_url),
                underlying: None,
            });
        }
        Ok(offers)
    }
    async fn create(
        &self,
        spec: &ResourceSpec,
        offer: &NativeOffer,
        fingerprint: &str,
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
        let key = spec
            .ssh_public_key
            .as_ref()
            .filter(|key| !key.trim().is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
        let mut metadata = ManagedMetadata::for_spec(spec, fingerprint, chrono::Utc::now())?;
        let mut original_offer = offer.clone();
        let mut owned = Vec::new();
        for raw in self.templates().await? {
            if let Some(existing) = Self::metadata(&raw)? {
                if existing.identity == metadata.identity {
                    if existing.request_fingerprint != fingerprint
                        || existing.expires_at <= chrono::Utc::now()
                        || raw["is_private"].as_bool() != Some(true)
                    {
                        return Err(Error::Provider(ProviderErrorCode::Conflict));
                    }
                    owned.push((raw, existing));
                }
            }
        }
        if owned.len() > 1 {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        let template = if let Some((raw, existing)) = owned.pop() {
            metadata = existing;
            original_offer = raw["description"]
                .as_str()
                .and_then(|description| serde_json::from_str::<NativeOffer>(description).ok())
                .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
            if original_offer.provider != offer.provider
                || original_offer.native_id != offer.native_id
                || original_offer.instance_type != offer.instance_type
                || original_offer.region != offer.region
                || original_offer.gpu_class != offer.gpu_class
                || original_offer.gpu_count != offer.gpu_count
                || original_offer.spot != offer.spot
                || !original_offer.hourly_cost_usd.is_finite()
                || original_offer.hourly_cost_usd <= 0.0
                || spec
                    .max_hourly_cost_usd
                    .is_some_and(|cap| original_offer.hourly_cost_usd > cap)
            {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
            Self::id(&raw)?.to_owned()
        } else {
            let mut env = crate::managed::bootstrap::workload_environment(spec)?;
            env.extend(metadata.env());
            let mut ports = spec.ports.clone();
            ports.push(22);
            ports.sort_unstable();
            ports.dedup();
            let response=self.http.post_json(&format!("{}/templates",self.base_url),&self.auth,json!({"name":metadata.name(),"description":serde_json::to_string(&original_offer).map_err(|_|Error::Provider(ProviderErrorCode::InvalidRequest))?,"docker_image":image,"docker_image_tag":"","environment":env,"internal_ports":ports,"is_private":true,"one_time_template":true,"category":"DOCKER","volumes":["/workspace"]})).await?;
            let raw: Value = response
                .json()
                .await
                .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
            Self::id(&raw)
                .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?
                .to_owned()
        };
        let native_ceiling = spec
            .max_hourly_cost_usd
            .map(|cap| (cap / f64::from(offer.gpu_count) - 0.01).next_down());
        let expected_price = native_ceiling.map_or(
            original_offer.hourly_cost_usd / f64::from(offer.gpu_count),
            |ceiling| ceiling.min(original_offer.hourly_cost_usd / f64::from(offer.gpu_count)),
        );
        if !expected_price.is_finite() || expected_price <= 0.0 {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let body = json!({"pod_name":metadata.name(),"template_id":template,"gpu_count":offer.gpu_count,"volume_id":spec.volume_id,"user_public_key":key,
            "termination_hours":spec.lease_duration_secs.unwrap_or(0)/3600,"initial_port_count":spec.ports.len()+usize::from(!spec.ports.contains(&22)),"enable_jupyter":false,"enable_backups":false,
            "expected_price_per_gpu_hour":expected_price});
        let idempotency_key = metadata.idempotency_key();
        let response = self
            .http
            .post_json_idempotent(
                &format!(
                    "{}/executors/{}/rent",
                    self.base_url,
                    urlencoding::encode(&offer.native_id)
                ),
                &self.auth,
                body,
                &idempotency_key,
            )
            .await?;
        let raw: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        if raw["success"].as_bool() != Some(true) {
            return Err(Error::Provider(ProviderErrorCode::Indeterminate));
        }
        let id = raw["pod_id"]
            .as_str()
            .filter(|id| uuid::Uuid::parse_str(id).is_ok())
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let mut resource = metadata.bind(
            ProvisionedInstance {
                id: id.into(),
                provider: CloudProvider::BittensorLium,
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
    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        if uuid::Uuid::parse_str(id).is_err() {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let Some(raw) = self
            .get(&format!("pods/{}", urlencoding::encode(id)))
            .await?
        else {
            return Ok(None);
        };
        let resource =
            Self::managed_pod(&raw)?.ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }
    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let pods = self
            .get("pods")
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let mut resources = Vec::new();
        for raw in pods
            .as_array()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
        {
            if let Some(resource) = Self::managed_pod(raw)? {
                if resource.namespace == namespace {
                    resources.push(resource);
                }
            }
        }
        for raw in self.templates().await? {
            if let Some(resource) = Self::managed_template(&raw)? {
                if resource.namespace == namespace {
                    resources.push(resource);
                }
            }
        }
        Ok(resources)
    }
    async fn destroy(
        &self,
        identity: &ManagedLeaseIdentity,
        resource: &ManagedResource,
    ) -> Result<()> {
        let id = &resource.instance.id;
        if uuid::Uuid::parse_str(id).is_err() {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let path = match resource.kind {
            ManagedResourceKind::Instance => "pods",
            ManagedResourceKind::Template => "templates",
            _ => return Err(Error::Provider(ProviderErrorCode::Unsupported)),
        };
        let Some(raw) = self.get(&format!("{path}/{id}")).await? else {
            return Ok(());
        };
        let observed = if resource.kind == ManagedResourceKind::Instance {
            Self::managed_pod(&raw)?
        } else {
            Self::managed_template(&raw)?
        }
        .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if observed.namespace != identity.namespace || observed.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        self.http
            .delete(&format!("{}/{path}/{id}", self.base_url), &self.auth)
            .await?;
        Ok(())
    }
}
#[async_trait]
impl CloudProviderAdapter for BittensorLiumAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::instance(
            &self
                .get(&format!("pods/{}", urlencoding::encode(id)))
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::NotFound))?,
        )
    }
}
