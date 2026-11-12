//! Compute Engine accelerator VMs: Billing Catalog prices, metadata ownership, zonal lifecycle.
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode, bootstrap,
};
use crate::providers::common::gpu_adapter::poll_until;
use crate::security::{
    ApiAuthentication, SecureHttpClient, canonical_public_key, parse_public_key,
};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use gcp_auth::TokenProvider;
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

const COMPUTE: &str = "https://compute.googleapis.com";
const BILLING: &str = "https://cloudbilling.googleapis.com";
const SCOPE: &str = "https://www.googleapis.com/auth/cloud-platform";
const USER: &str = "tangle";
const DEFAULT_IMAGE: &str = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580";

pub struct GcpManagedAdapter {
    compute: SecureHttpClient,
    billing: SecureHttpClient,
    compute_url: String,
    billing_url: String,
    project_id: String,
    image: String,
    key: Arc<gcp_auth::CustomServiceAccount>,
    key_digest: String,
    billing_key: Option<String>,
    skus: tokio::sync::OnceCell<Vec<Value>>,
}

impl GcpManagedAdapter {
    pub const PROJECT_ID_ENV: &'static str = "GCP_PROJECT_ID";
    pub const SERVICE_ACCOUNT_KEY_ENV: &'static str = "GCP_SERVICE_ACCOUNT_KEY";
    pub const IMAGE_ENV: &'static str = "GCP_GPU_SOURCE_IMAGE";
    pub const REGION_ENV: &'static str = "GCP_REGION";
    pub const BILLING_API_KEY_ENV: &'static str = "GCP_BILLING_API_KEY";

    /// `service_account_key` is the JSON key; a loopback `endpoint` serves compute and billing routes.
    pub fn with_endpoint(
        project_id: String,
        service_account_key: &str,
        image: Option<String>,
        billing_key: Option<String>,
        endpoint: Option<&str>,
    ) -> Result<Self> {
        let invalid = || Error::Provider(ProviderErrorCode::InvalidRequest);
        let image = image.unwrap_or_else(|| DEFAULT_IMAGE.into());
        if project_id.is_empty()
            || !project_id.bytes().all(|b| {
                b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'-' | b':' | b'.')
            })
            || !image.starts_with("projects/")
            || image
                .chars()
                .any(|c| c.is_whitespace() || c.is_control() || c == '?')
        {
            return Err(invalid());
        }
        let key = gcp_auth::CustomServiceAccount::from_json(service_account_key)
            .map_err(|_| invalid())?;
        let key_digest = hex::encode(aws_lc_rs::digest::digest(
            &aws_lc_rs::digest::SHA256,
            service_account_key.as_bytes(),
        ));
        let compute = endpoint.unwrap_or(COMPUTE).trim_end_matches('/');
        let billing = endpoint.unwrap_or(BILLING).trim_end_matches('/');
        Ok(Self {
            compute: SecureHttpClient::with_endpoint(compute)?,
            billing: SecureHttpClient::with_endpoint(billing)?,
            compute_url: format!("{compute}/compute/v1/projects/{project_id}"),
            billing_url: format!("{billing}/v1/services/6F81-5844-456A/skus"),
            project_id,
            image,
            key: Arc::new(key),
            key_digest,
            billing_key,
            skus: tokio::sync::OnceCell::new(),
        })
    }

    pub(crate) fn credential_binding(&self) -> Result<(String, Option<String>)> {
        Ok((
            crate::managed::authorization::credential_digest(
                &ApiAuthentication::Bearer {
                    token: self.key_digest.clone(),
                },
                &(
                    &self.compute_url,
                    &self.billing_url,
                    &self.project_id,
                    &self.image,
                ),
            )?,
            None,
        ))
    }

    async fn auth(&self) -> Result<ApiAuthentication> {
        let token = self
            .key
            .token(&[SCOPE])
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::AuthRejected))?;
        Ok(ApiAuthentication::Bearer {
            token: token.as_str().to_owned(),
        })
    }

    /// `None` is a 404.
    async fn call(&self, method: Method, url: &str, body: Option<Value>) -> Result<Option<Value>> {
        let auth = self.auth().await?;
        Self::json(
            self.compute
                .authenticated_request(method, url, &auth, body)
                .await?,
        )
        .await
    }

    async fn json(response: reqwest::Response) -> Result<Option<Value>> {
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        response
            .json()
            .await
            .map(Some)
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))
    }

    /// On-demand Compute Engine SKUs, fetched once per adapter.
    async fn skus(&self) -> Result<&Vec<Value>> {
        self.skus
            .get_or_try_init(|| async {
                let mut skus = Vec::new();
                let mut token = String::new();
                for _ in 0..100 {
                    let url = format!(
                        "{}?currencyCode=USD&pageSize=2000&pageToken={}",
                        self.billing_url,
                        urlencoding::encode(&token)
                    );
                    // Google documents an API key for the Catalog API; a token is the fallback.
                    let auth = match &self.billing_key {
                        Some(key) => ApiAuthentication::ApiKey {
                            key: key.clone(),
                            header_name: "X-goog-api-key".into(),
                        },
                        None => self.auth().await?,
                    };
                    let page = Self::json(self.billing.get(&url, &auth).await?)
                        .await?
                        .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                    skus.extend(
                        page["skus"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter(|sku| sku["category"]["usageType"] == "OnDemand")
                            .cloned(),
                    );
                    match page["nextPageToken"]
                        .as_str()
                        .filter(|next| !next.is_empty())
                    {
                        Some(next) => token = next.to_owned(),
                        None => return Ok(skus),
                    }
                }
                Err(Error::Provider(ProviderErrorCode::ProviderError))
            })
            .await
    }

    /// Live hourly price: the family's core, RAM and bundled local SSD SKUs plus its GPU SKU.
    async fn price(&self, region: &str, machine: &Value) -> Result<Option<f64>> {
        let family = machine["name"]
            .as_str()
            .and_then(|name| name.split('-').next())
            .unwrap_or("")
            .to_ascii_uppercase();
        let accelerator = &machine["accelerators"][0];
        let gpu = match accelerator["guestAcceleratorType"].as_str() {
            Some("nvidia-tesla-a100") => "Nvidia Tesla A100 GPU",
            Some("nvidia-a100-80gb") => "Nvidia Tesla A100 80GB GPU",
            Some("nvidia-h100-80gb") => "Nvidia H100 80GB GPU",
            Some("nvidia-l4") => "Nvidia L4 GPU",
            _ => return Ok(None),
        };
        let (Some(cpus), Some(memory_mb), Some(gpus)) = (
            machine["guestCpus"].as_f64(),
            machine["memoryMb"].as_f64(),
            accelerator["guestAcceleratorCount"].as_f64(),
        ) else {
            return Ok(None);
        };
        let ssd_gib = machine["bundledLocalSsds"]["partitionCount"]
            .as_f64()
            .unwrap_or(0.0)
            * 375.0;
        let skus = self.skus().await?;
        let rate = |stem: &str, unit: &str| {
            let sku = skus.iter().find(|sku| {
                sku["description"]
                    .as_str()
                    .and_then(|text| text.strip_prefix(stem))
                    .is_some_and(|rest| {
                        // Americas local SSD carries no location suffix.
                        rest.is_empty()
                            || rest.starts_with(" running in ")
                            || rest.starts_with(" in ")
                    })
                    && sku["serviceRegions"]
                        .as_array()
                        .is_some_and(|regions| regions.iter().any(|value| value == region))
            })?;
            let expression = &sku["pricingInfo"][0]["pricingExpression"];
            if expression["usageUnit"] != unit {
                return None;
            }
            let price = &expression["tieredRates"].as_array()?.last()?["unitPrice"];
            // Proto3 JSON omits zero fields.
            let units = price["units"].as_str().unwrap_or("0").parse::<f64>().ok()?;
            Some(units + price["nanos"].as_f64().unwrap_or(0.0) / 1e9)
        };
        let ssd = if ssd_gib > 0.0 {
            // Google bills local SSD per GiB-month of 730 hours.
            rate("SSD backed Local Storage", "GiBy.mo").map(|month| ssd_gib * month / 730.0)
        } else {
            Some(0.0)
        };
        Ok(
            match (
                rate(&format!("{family} Instance Core"), "h"),
                rate(&format!("{family} Instance Ram"), "GiBy.h"),
                rate(gpu, "h"),
                ssd,
            ) {
                (Some(core), Some(ram), Some(device), Some(ssd)) => {
                    Some(cpus * core + memory_mb / 1024.0 * ram + gpus * device + ssd)
                        .filter(|price| price.is_finite() && *price > 0.0)
                }
                _ => None,
            },
        )
    }

    async fn get(&self, path: &str) -> Result<Option<Value>> {
        let url = format!("{}{path}", self.compute_url);
        self.call(Method::GET, &url, None).await
    }

    fn last(url: &str) -> &str {
        url.rsplit('/').next().unwrap_or("")
    }

    fn region_of(zone: &str) -> &str {
        zone.rsplit_once('-').map_or(zone, |(region, _)| region)
    }

    fn class(machine: &str) -> Option<(blueprint_core::gpu::GpuClass, u32)> {
        blueprint_core::gpu::catalog()
            .into_iter()
            .find_map(|class| {
                let count = (1..=class.max_per_node)
                    .find(|count| class.sku("gcp", *count) == Some(machine))?;
                Some((class, count))
            })
    }

    /// Instance id is `zone/name`: Compute Engine addresses instances by zone.
    fn split(id: &str) -> Result<(&str, &str)> {
        id.split_once('/')
            .filter(|(zone, name)| {
                [zone, name].iter().all(|part| {
                    !part.is_empty()
                        && part
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                })
            })
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))
    }

    fn state(raw: &Value) -> InstanceStatus {
        match raw["status"].as_str() {
            Some("PROVISIONING" | "STAGING") => InstanceStatus::Starting,
            Some("RUNNING") => InstanceStatus::Running,
            Some("STOPPING" | "SUSPENDING") => InstanceStatus::Stopping,
            Some("STOPPED" | "SUSPENDED" | "TERMINATED") => InstanceStatus::Stopped,
            _ => InstanceStatus::Unknown,
        }
    }

    async fn resource(
        &self,
        raw: &Value,
        prices: &mut HashMap<(String, String), Option<f64>>,
    ) -> Result<Option<ManagedResource>> {
        let mut items = BTreeMap::new();
        for item in raw["metadata"]["items"].as_array().into_iter().flatten() {
            if let (Some(key), Some(value)) = (item["key"].as_str(), item["value"].as_str()) {
                items.insert(key.to_owned(), value.to_owned());
            }
        }
        let Some(metadata) = ManagedMetadata::from_tags(&items)? else {
            return Ok(None);
        };
        let name = raw["name"]
            .as_str()
            .filter(|name| !name.is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let zone = Self::last(raw["zone"].as_str().unwrap_or(""));
        let machine = Self::last(raw["machineType"].as_str().unwrap_or("")).to_owned();
        let region = Self::region_of(zone).to_owned();
        let public_ip = raw["networkInterfaces"][0]["accessConfigs"][0]["natIP"]
            .as_str()
            .map(str::to_owned);
        let instance = ProvisionedInstance {
            id: format!("{zone}/{name}"),
            provider: CloudProvider::GCP,
            instance_type: machine.clone(),
            region: region.clone(),
            public_ip,
            private_ip: raw["networkInterfaces"][0]["networkIP"]
                .as_str()
                .map(str::to_owned),
            status: Self::state(raw),
        };
        let mut resource = metadata.bind(instance, ManagedResourceKind::Instance);
        if let Some((class, count)) = Self::class(&machine) {
            resource.gpu_class = Some(class.id.clone());
            resource.gpu_count = Some(count);
            resource.gpu_vram_gb = Some(class.vram_gb);
            let key = (region.clone(), machine.clone());
            resource.hourly_cost_usd = match prices.get(&key) {
                Some(price) => *price,
                None => {
                    let details = self
                        .get(&format!("/zones/{zone}/machineTypes/{machine}"))
                        .await?
                        .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                    let price = self.price(&region, &details).await?;
                    prices.insert(key, price);
                    price
                }
            };
        }
        resource.created_at = raw["creationTimestamp"]
            .as_str()
            .and_then(|time| time.parse().ok());
        resource.ssh_endpoint = resource
            .instance
            .public_ip
            .as_ref()
            .and_then(|ip| ip.parse::<std::net::Ipv4Addr>().ok())
            .map(|ip| format!("ssh://{USER}@{ip}:22"));
        Ok(Some(resource))
    }

    async fn instance(&self, id: &str) -> Result<Option<Value>> {
        let (zone, name) = Self::split(id)?;
        self.get(&format!("/zones/{zone}/instances/{name}")).await
    }

    fn unsupported(spec: &ResourceSpec) -> bool {
        spec.tee_required
            || spec.volume_gb.is_some()
            || spec.volume_id.is_some()
            || spec.source_network.is_some()
    }

    /// A deterministic UUID-shaped request id: Compute Engine deduplicates retried inserts by it.
    fn request_id(metadata: &ManagedMetadata) -> String {
        let key = metadata.idempotency_key();
        format!(
            "{}-{}-{}-{}-{}",
            &key[0..8],
            &key[8..12],
            &key[12..16],
            &key[16..20],
            &key[20..32]
        )
    }
}

#[async_trait]
impl ManagedProviderAdapter for GcpManagedAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if Self::unsupported(spec) {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if region.is_empty()
            || !region
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let zones: Vec<String> = self
            .get(&format!("/regions/{region}"))
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))?["zones"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|zone| zone.as_str().map(|zone| Self::last(zone).to_owned()))
            .collect();
        let mut offers = Vec::new();
        for class in blueprint_core::gpu::catalog() {
            let Some(machine) = class.sku("gcp", count) else {
                continue;
            };
            if spec.gpu_class.as_ref().is_some_and(|id| id != &class.id)
                || spec
                    .gpu_vram_gb
                    .is_some_and(|minimum| class.vram_gb < minimum)
            {
                continue;
            }
            let mut price = None;
            for zone in &zones {
                let Some(details) = self
                    .get(&format!("/zones/{zone}/machineTypes/{machine}"))
                    .await?
                else {
                    continue;
                };
                if details["guestCpus"]
                    .as_f64()
                    .is_none_or(|cpu| cpu < f64::from(spec.cpu))
                    || details["memoryMb"]
                        .as_f64()
                        .is_none_or(|mb| mb / 1024.0 < f64::from(spec.memory_gb))
                    || details["accelerators"][0]["guestAcceleratorCount"].as_u64()
                        != Some(u64::from(count))
                {
                    continue;
                }
                if price.is_none() {
                    price = Some(self.price(region, &details).await?);
                }
                let Some(Some(hourly)) = price else {
                    break;
                };
                offers.push(NativeOffer {
                    provider: CloudProvider::GCP,
                    native_id: zone.clone(),
                    instance_type: machine.into(),
                    region: region.into(),
                    gpu_class: class.id.clone(),
                    gpu_count: count,
                    hourly_cost_usd: hourly,
                    spot: false,
                    pricing_source: self.billing_url.clone(),
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
        if Self::unsupported(spec) || offer.spot || offer.provider != CloudProvider::GCP {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let zone = offer.native_id.as_str();
        Self::split(&format!("{zone}/x"))?;
        if Self::region_of(zone) != offer.region {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let key = canonical_public_key(
            spec.ssh_public_key
                .as_deref()
                .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?,
        )?;
        let mut items: Vec<Value> = metadata
            .tags()
            .into_iter()
            .map(|(key, value)| json!({"key":key,"value":value}))
            .collect();
        items.push(json!({"key":"ssh-keys","value":format!("{USER}:{key}")}));
        items.push(json!({"key":"install-nvidia-driver","value":"True"}));
        if let Some(image) = &spec.image {
            let mut env = bootstrap::workload_environment(spec)?;
            env.extend(metadata.env());
            let user_data = bootstrap::gpu_container_cloud_init(
                image,
                &env,
                &spec.ports,
                &[],
                spec.command.as_deref(),
            )?;
            items.push(json!({"key":"user-data","value":user_data}));
        } else if !spec.env.is_empty() || spec.command.is_some() || spec.manifest.is_some() {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let name = metadata.name();
        let body = json!({"name":name,"machineType":format!("zones/{zone}/machineTypes/{}", offer.instance_type),
            "labels":{"bpg-managed":"1"},"metadata":{"items":items},
            "disks":[{"boot":true,"autoDelete":true,"initializeParams":{"sourceImage":self.image,"diskSizeGb":(spec.storage_gb.ceil() as u64).max(50).to_string()}}],
            "networkInterfaces":[{"network":"global/networks/default","accessConfigs":[{"name":"External NAT","type":"ONE_TO_ONE_NAT"}]}],
            "scheduling":{"onHostMaintenance":"TERMINATE","automaticRestart":false}});
        metadata.check_deadline()?;
        let url = format!(
            "{}/zones/{zone}/instances?requestId={}",
            self.compute_url,
            Self::request_id(metadata)
        );
        let operation = self
            .call(Method::POST, &url, Some(body))
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        // Capacity and quota refusals arrive on the insert operation, not the POST.
        let poll = operation["name"]
            .as_str()
            .filter(|name| name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-'))
            .map(|name| format!("{}/zones/{zone}/operations/{name}", self.compute_url))
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let wait = std::time::Duration::from_secs(3);
        // After an accepted insert, any polling failure leaves the outcome unknown.
        let operation = poll_until("GCP insert", wait, wait * 60, || async {
            let operation = self.call(Method::GET, &poll, None).await?;
            Ok(operation.filter(|operation| operation["status"] == "DONE"))
        })
        .await
        .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        if let Some(errors) = operation["error"]["errors"].as_array() {
            let quota = errors.iter().any(|error| {
                error["code"]
                    .as_str()
                    .is_some_and(|code| code.contains("QUOTA"))
            });
            return Err(Error::Provider(if quota {
                ProviderErrorCode::Quota
            } else {
                ProviderErrorCode::NoStock
            }));
        }
        let instance = ProvisionedInstance {
            id: format!("{zone}/{name}"),
            provider: CloudProvider::GCP,
            instance_type: offer.instance_type.clone(),
            region: offer.region.clone(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        };
        let mut resource = metadata
            .clone()
            .bind(instance, ManagedResourceKind::Instance);
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
        let raw = self
            .instance(&resource.instance.id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        if let Some(expected) = spec.ssh_public_key.as_deref() {
            let expected = parse_public_key(expected)?;
            let installed = raw["metadata"]["items"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|item| item["key"] == "ssh-keys")
                .filter_map(|item| item["value"].as_str())
                .flat_map(str::lines)
                .filter_map(|line| line.strip_prefix(&format!("{USER}:")).map(str::to_owned))
                .any(|key| {
                    parse_public_key(&key).is_ok_and(|key| key.key_data() == expected.key_data())
                });
            if !installed {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
        }
        let price = resource.hourly_cost_usd;
        let mut observed = self
            .resource(&raw, &mut HashMap::new())
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        observed.hourly_cost_usd = price.or(observed.hourly_cost_usd);
        Ok(observed)
    }

    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        let Some(raw) = self.instance(id).await? else {
            return Ok(None);
        };
        let resource = self
            .resource(&raw, &mut HashMap::new())
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }

    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let mut resources = Vec::new();
        let mut prices = HashMap::new();
        let mut token = None::<String>;
        let mut seen = HashSet::new();
        loop {
            let mut url = url::Url::parse(&format!("{}/aggregated/instances", self.compute_url))
                .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
            url.query_pairs_mut()
                .append_pair("filter", "labels.bpg-managed=\"1\"")
                .append_pair("returnPartialSuccess", "false");
            if let Some(token) = &token {
                url.query_pairs_mut().append_pair("pageToken", token);
            }
            let page = self
                .call(Method::GET, url.as_str(), None)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
            for scope in page["items"]
                .as_object()
                .into_iter()
                .flatten()
                .map(|(_, scope)| scope)
            {
                for raw in scope["instances"].as_array().into_iter().flatten() {
                    if let Some(resource) = self.resource(raw, &mut prices).await? {
                        if resource.namespace == namespace {
                            resources.push(resource);
                        }
                    }
                }
            }
            match page["nextPageToken"]
                .as_str()
                .filter(|next| !next.is_empty())
            {
                Some(next) if seen.insert(next.to_owned()) => token = Some(next.into()),
                Some(_) => return Err(Error::Provider(ProviderErrorCode::ProviderError)),
                None => return Ok(resources),
            }
        }
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
        let (zone, name) = Self::split(id)?;
        let url = format!("{}/zones/{zone}/instances/{name}", self.compute_url);
        self.call(Method::DELETE, &url, None).await?;
        Ok(())
    }
}

#[async_trait]
impl CloudProviderAdapter for GcpManagedAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let raw = self
            .instance(id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))?;
        self.resource(&raw, &mut HashMap::new())
            .await?
            .map(|resource| resource.instance)
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))
    }
}
