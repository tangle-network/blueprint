//! Azure Resource Manager GPU VMs: retail prices, VM tags and NIC/IP/disk deleted with the VM.
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode, bootstrap,
};
use crate::pricing::{PricingError, PricingFetcher};
use crate::security::{
    ApiAuthentication, SecureHttpClient, canonical_public_key, parse_public_key,
};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use base64::Engine;
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::{Duration, Instant};

const ARM: &str = "https://management.azure.com";
const LOGIN: &str = "https://login.microsoftonline.com";
const COMPUTE_API: &str = "2024-07-01";
const NETWORK_API: &str = "2024-05-01";
const SKUS_API: &str = "2021-07-01";
const USER: &str = "azureuser";
const DEFAULT_IMAGE: &str = "microsoft-dsvm:ubuntu-hpc:2204:latest";
const PRICES: &str = "https://prices.azure.com/api/retail/prices";

/// One subscription and resource group reached through a service principal.
pub struct AzureAccount {
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub subscription_id: String,
    pub resource_group: String,
    /// Existing subnet resource id the VM NIC joins.
    pub subnet_id: String,
    /// `publisher:offer:sku:version`; the image must provide NVIDIA drivers and Docker.
    pub image: Option<String>,
}

pub struct AzureManagedAdapter {
    http: SecureHttpClient,
    login: SecureHttpClient,
    pricing: PricingFetcher,
    base_url: String,
    login_url: String,
    account: AzureAccount,
    image: [String; 4],
    token: tokio::sync::Mutex<Option<(zeroize::Zeroizing<String>, Instant)>>,
}

impl AzureManagedAdapter {
    pub const TENANT_ID_ENV: &'static str = "AZURE_TENANT_ID";
    pub const CLIENT_ID_ENV: &'static str = "AZURE_CLIENT_ID";
    pub const CLIENT_SECRET_ENV: &'static str = "AZURE_CLIENT_SECRET";
    pub const SUBSCRIPTION_ID_ENV: &'static str = "AZURE_SUBSCRIPTION_ID";
    pub const RESOURCE_GROUP_ENV: &'static str = "AZURE_RESOURCE_GROUP";
    pub const SUBNET_ID_ENV: &'static str = "AZURE_SUBNET_ID";
    pub const IMAGE_ENV: &'static str = "AZURE_GPU_IMAGE_URN";
    pub const REGION_ENV: &'static str = "AZURE_LOCATION";

    /// A loopback `endpoint` serves ARM, token and retail-price routes for local peers.
    pub fn with_endpoint(account: AzureAccount, endpoint: Option<&str>) -> Result<Self> {
        let invalid = || Error::Provider(ProviderErrorCode::InvalidRequest);
        let image: Vec<String> = account
            .image
            .as_deref()
            .unwrap_or(DEFAULT_IMAGE)
            .split(':')
            .map(str::to_owned)
            .collect();
        let image: [String; 4] = image.try_into().map_err(|_| invalid())?;
        let ids = [
            &account.tenant_id,
            &account.client_id,
            &account.subscription_id,
            &account.resource_group,
        ];
        if image.iter().any(String::is_empty)
            || ids.iter().any(|id| {
                id.is_empty()
                    || !id
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
            })
            || !account.subnet_id.starts_with("/subscriptions/")
            || !account.subnet_id.contains("/subnets/")
            || account
                .subnet_id
                .chars()
                .any(|c| c.is_control() || c == '?')
        {
            return Err(invalid());
        }
        let mut pricing = PricingFetcher::new().map_err(|_| invalid())?;
        if let Some(endpoint) = endpoint {
            pricing = pricing
                .with_endpoint(CloudProvider::Azure, endpoint)
                .map_err(|_| invalid())?;
        }
        let base = endpoint.unwrap_or(ARM).trim_end_matches('/');
        let login = endpoint.unwrap_or(LOGIN).trim_end_matches('/');
        Ok(Self {
            http: SecureHttpClient::with_endpoint(base)?,
            login: SecureHttpClient::with_endpoint(login)?,
            pricing,
            base_url: base.into(),
            login_url: login.into(),
            account,
            image,
            token: tokio::sync::Mutex::new(None),
        })
    }

    pub(crate) fn credential_binding(&self) -> Result<(String, Option<String>)> {
        let account = &self.account;
        Ok((
            crate::managed::authorization::credential_digest(
                &ApiAuthentication::Bearer {
                    token: account.client_secret.clone(),
                },
                &(
                    &self.base_url,
                    &account.tenant_id,
                    &account.client_id,
                    &account.subscription_id,
                    &account.resource_group,
                    &account.subnet_id,
                    &self.image,
                ),
            )?,
            None,
        ))
    }

    async fn auth(&self) -> Result<ApiAuthentication> {
        let mut cached = self.token.lock().await;
        if let Some((token, expiry)) = cached.as_ref()
            && *expiry > Instant::now()
        {
            return Ok(ApiAuthentication::Bearer {
                token: token.as_str().to_owned(),
            });
        }
        let form = HashMap::from([
            ("grant_type".to_owned(), "client_credentials".to_owned()),
            ("client_id".to_owned(), self.account.client_id.clone()),
            (
                "client_secret".to_owned(),
                self.account.client_secret.clone(),
            ),
            (
                "scope".to_owned(),
                "https://management.azure.com/.default".to_owned(),
            ),
        ]);
        let url = format!(
            "{}/{}/oauth2/v2.0/token",
            self.login_url, self.account.tenant_id
        );
        let response = match self
            .login
            .post_form(&url, &ApiAuthentication::None, &form)
            .await
        {
            Err(Error::Provider(ProviderErrorCode::ProviderError)) => {
                return Err(Error::Provider(ProviderErrorCode::AuthRejected));
            }
            result => result?,
        };
        let body: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::AuthRejected))?;
        let token = body["access_token"]
            .as_str()
            .filter(|token| !token.is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::AuthRejected))?;
        let seconds = body["expires_in"]
            .as_u64()
            .unwrap_or(300)
            .saturating_sub(120);
        *cached = Some((
            zeroize::Zeroizing::new(token.to_owned()),
            Instant::now() + Duration::from_secs(seconds),
        ));
        Ok(ApiAuthentication::Bearer {
            token: token.to_owned(),
        })
    }

    /// `None` is a 404; an empty success body is `Value::Null`.
    async fn arm(&self, method: Method, url: &str, body: Option<Value>) -> Result<Option<Value>> {
        let url = if url.starts_with('/') {
            format!("{}{url}", self.base_url)
        } else {
            url.to_owned()
        };
        let auth = self.auth().await?;
        let response = self
            .http
            .authenticated_request(method, &url, &auth, body)
            .await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let bytes = response
            .bytes()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        if bytes.is_empty() {
            return Ok(Some(Value::Null));
        }
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))
    }

    fn vm_path(&self, name: &str) -> String {
        format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version={COMPUTE_API}",
            self.account.subscription_id,
            self.account.resource_group,
            urlencoding::encode(name)
        )
    }

    fn class(sku: &str) -> Option<(blueprint_core::gpu::GpuClass, u32)> {
        blueprint_core::gpu::catalog()
            .into_iter()
            .find_map(|class| {
                let count = (1..=class.max_per_node)
                    .find(|count| class.sku("azure", *count) == Some(sku))?;
                Some((class, count))
            })
    }

    async fn price(&self, region: &str, sku: &str) -> Result<Option<f64>> {
        match self
            .pricing
            .quote_instance(CloudProvider::Azure, region, sku)
            .await
        {
            Ok(price) => Ok(price.filter(|price| price.is_finite() && *price > 0.0)),
            Err(PricingError::AuthRejected) => {
                Err(Error::Provider(ProviderErrorCode::AuthRejected))
            }
            Err(_) => Err(Error::Provider(ProviderErrorCode::ProviderError)),
        }
    }

    async fn public_ip(&self, raw: &Value) -> Result<Option<String>> {
        let Some(nic) = raw["properties"]["networkProfile"]["networkInterfaces"][0]["id"].as_str()
        else {
            return Ok(None);
        };
        let Some(nic) = self
            .arm(
                Method::GET,
                &format!("{nic}?api-version={NETWORK_API}"),
                None,
            )
            .await?
        else {
            return Ok(None);
        };
        let Some(ip) =
            nic["properties"]["ipConfigurations"][0]["properties"]["publicIPAddress"]["id"]
                .as_str()
        else {
            return Ok(None);
        };
        Ok(self
            .arm(
                Method::GET,
                &format!("{ip}?api-version={NETWORK_API}"),
                None,
            )
            .await?
            .and_then(|ip| ip["properties"]["ipAddress"].as_str().map(str::to_owned)))
    }

    fn power(raw: &Value) -> InstanceStatus {
        let power = raw["properties"]["instanceView"]["statuses"]
            .as_array()
            .into_iter()
            .flatten()
            .find_map(|status| status["code"].as_str()?.strip_prefix("PowerState/"));
        match (raw["properties"]["provisioningState"].as_str(), power) {
            (Some("Deleting"), _) => InstanceStatus::Stopping,
            (Some("Failed"), _) => InstanceStatus::Unknown,
            (_, Some("running")) => InstanceStatus::Running,
            (_, Some("starting")) => InstanceStatus::Starting,
            (_, Some("stopping" | "deallocating")) => InstanceStatus::Stopping,
            (_, Some("stopped" | "deallocated")) => InstanceStatus::Stopped,
            (Some("Creating" | "Updating"), None) => InstanceStatus::Starting,
            (Some("Succeeded"), None) => InstanceStatus::Running,
            _ => InstanceStatus::Unknown,
        }
    }

    /// `prices` caches live retail rates within one call.
    async fn resource(
        &self,
        raw: &Value,
        network: bool,
        prices: &mut HashMap<(String, String), Option<f64>>,
    ) -> Result<Option<ManagedResource>> {
        let mut tags = BTreeMap::new();
        if let Some(values) = raw["tags"].as_object() {
            for (key, value) in values {
                if let Some(value) = value.as_str() {
                    tags.insert(key.clone(), value.to_owned());
                }
            }
        }
        let Some(metadata) = ManagedMetadata::from_tags(&tags)? else {
            return Ok(None);
        };
        let name = raw["name"]
            .as_str()
            .filter(|name| !name.is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let sku = raw["properties"]["hardwareProfile"]["vmSize"]
            .as_str()
            .unwrap_or("")
            .to_owned();
        let region = raw["location"].as_str().unwrap_or("").to_ascii_lowercase();
        let public_ip = if network {
            self.public_ip(raw).await?
        } else {
            None
        };
        let instance = ProvisionedInstance {
            id: name.into(),
            provider: CloudProvider::Azure,
            instance_type: sku.clone(),
            region: region.clone(),
            public_ip,
            private_ip: None,
            status: Self::power(raw),
        };
        let mut resource = metadata.bind(instance, ManagedResourceKind::Instance);
        if let Some((class, count)) = Self::class(&sku) {
            resource.gpu_class = Some(class.id.clone());
            resource.gpu_count = Some(count);
            resource.gpu_vram_gb = Some(class.vram_gb);
            let key = (region.clone(), sku.clone());
            resource.hourly_cost_usd = match prices.get(&key) {
                Some(price) => *price,
                None => {
                    let price = self.price(&region, &sku).await?;
                    prices.insert(key, price);
                    price
                }
            };
        }
        resource.created_at = raw["properties"]["timeCreated"]
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

    async fn vm(&self, name: &str) -> Result<Option<Value>> {
        let url = format!("{}&$expand=instanceView", self.vm_path(name));
        self.arm(Method::GET, &url, None).await
    }

    fn unsupported(spec: &ResourceSpec) -> bool {
        spec.tee_required
            || spec.volume_gb.is_some()
            || spec.volume_id.is_some()
            || spec.source_network.is_some()
    }
}

#[async_trait]
impl ManagedProviderAdapter for AzureManagedAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if Self::unsupported(spec) {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let region = region.to_ascii_lowercase();
        if region.is_empty() || !region.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        let wanted: Vec<_> = blueprint_core::gpu::catalog()
            .into_iter()
            .filter_map(|class| {
                let sku = class.sku("azure", count)?.to_owned();
                let fits = spec.gpu_class.as_ref().is_none_or(|id| id == &class.id)
                    && spec
                        .gpu_vram_gb
                        .is_none_or(|minimum| class.vram_gb >= minimum);
                fits.then_some((class, sku))
            })
            .collect();
        if wanted.is_empty() {
            return Ok(Vec::new());
        }
        let mut url = format!(
            "/subscriptions/{}/providers/Microsoft.Compute/skus?api-version={SKUS_API}&$filter={}",
            self.account.subscription_id,
            urlencoding::encode(&format!("location eq '{region}'"))
        );
        let mut available = HashMap::new();
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(url.clone()) || seen.len() > 50 {
                return Err(Error::Provider(ProviderErrorCode::ProviderError));
            }
            let page = self
                .arm(Method::GET, &url, None)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
            for sku in page["value"]
                .as_array()
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
            {
                let restricted = sku["restrictions"]
                    .as_array()
                    .is_some_and(|rows| rows.iter().any(|row| row["type"] == "Location"));
                let local = sku["locations"].as_array().is_some_and(|rows| {
                    rows.iter().any(|row| {
                        row.as_str()
                            .is_some_and(|r| r.eq_ignore_ascii_case(&region))
                    })
                });
                let (Some(name), "virtualMachines") = (
                    sku["name"].as_str(),
                    sku["resourceType"].as_str().unwrap_or(""),
                ) else {
                    continue;
                };
                if restricted || !local {
                    continue;
                }
                let capability = |key: &str| {
                    sku["capabilities"].as_array().and_then(|rows| {
                        rows.iter()
                            .find(|row| row["name"] == key)
                            .and_then(|row| row["value"].as_str()?.parse::<f64>().ok())
                    })
                };
                available.insert(
                    name.to_owned(),
                    (
                        capability("vCPUs"),
                        capability("MemoryGB"),
                        capability("GPUs"),
                    ),
                );
            }
            match page["nextLink"].as_str().filter(|next| !next.is_empty()) {
                Some(next) => url = next.to_owned(),
                None => break,
            }
        }
        let mut offers = Vec::new();
        for (class, sku) in wanted {
            let Some((cpu, memory, gpus)) = available.get(&sku) else {
                continue;
            };
            if gpus.is_some_and(|gpus| (gpus - f64::from(count)).abs() >= 0.5)
                || cpu.is_some_and(|cpu| cpu < f64::from(spec.cpu))
                || memory.is_some_and(|memory| memory < f64::from(spec.memory_gb))
            {
                continue;
            }
            let Some(price) = self.price(&region, &sku).await? else {
                continue;
            };
            offers.push(NativeOffer {
                provider: CloudProvider::Azure,
                native_id: sku.clone(),
                instance_type: sku,
                region: region.clone(),
                gpu_class: class.id.clone(),
                gpu_count: count,
                hourly_cost_usd: price,
                spot: false,
                pricing_source: PRICES.into(),
                underlying: None,
            });
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
        if Self::unsupported(spec) || offer.spot || offer.provider != CloudProvider::Azure {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let key = canonical_public_key(
            spec.ssh_public_key
                .as_deref()
                .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?,
        )?;
        let name = metadata.name();
        let [publisher, image_offer, image_sku, version] = &self.image;
        let mut os = json!({"computerName":name,"adminUsername":USER,"linuxConfiguration":{
            "disablePasswordAuthentication":true,
            "ssh":{"publicKeys":[{"path":format!("/home/{USER}/.ssh/authorized_keys"),"keyData":key}]}}});
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
            let encoded = base64::engine::general_purpose::STANDARD.encode(user_data);
            if encoded.len() > 65_535 {
                return Err(Error::Provider(ProviderErrorCode::Unsupported));
            }
            os["customData"] = json!(encoded);
        } else if !spec.env.is_empty() || spec.command.is_some() || spec.manifest.is_some() {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let mut disk = json!({"createOption":"FromImage","deleteOption":"Delete"});
        if spec.storage_gb >= 30.0 {
            disk["diskSizeGB"] = json!(spec.storage_gb.ceil() as u64);
        }
        let body = json!({"location":offer.region,"tags":metadata.tags(),"properties":{
            "hardwareProfile":{"vmSize":offer.instance_type},
            "storageProfile":{"imageReference":{"publisher":publisher,"offer":image_offer,"sku":image_sku,"version":version},"osDisk":disk},
            "osProfile":os,
            "networkProfile":{"networkApiVersion":"2022-11-01","networkInterfaceConfigurations":[{"name":format!("{name}-nic"),"properties":{
                "primary":true,"deleteOption":"Delete","ipConfigurations":[{"name":"primary","properties":{
                    "subnet":{"id":self.account.subnet_id},
                    "publicIPAddressConfiguration":{"name":format!("{name}-ip"),"sku":{"name":"Standard"},
                        "properties":{"deleteOption":"Delete","publicIPAllocationMethod":"Static"}}}}]}}]}}});
        metadata.check_deadline()?;
        let raw = self
            .arm(Method::PUT, &self.vm_path(&name), Some(body))
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        if raw["name"].as_str() != Some(name.as_str())
            || raw["properties"]["hardwareProfile"]["vmSize"].as_str()
                != Some(offer.instance_type.as_str())
        {
            return Err(Error::Provider(ProviderErrorCode::Indeterminate));
        }
        let instance = ProvisionedInstance {
            id: name,
            provider: CloudProvider::Azure,
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
            .vm(&resource.instance.id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        if let Some(expected) = spec.ssh_public_key.as_deref() {
            let expected = parse_public_key(expected)?;
            let installed =
                raw["properties"]["osProfile"]["linuxConfiguration"]["ssh"]["publicKeys"]
                    .as_array()
                    .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
            if !installed.iter().any(|key| {
                key["keyData"].as_str().is_some_and(|data| {
                    parse_public_key(data).is_ok_and(|key| key.key_data() == expected.key_data())
                })
            }) {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
        }
        self.resource(&raw, true, &mut HashMap::new())
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))
    }

    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        let Some(raw) = self.vm(id).await? else {
            return Ok(None);
        };
        let resource = self
            .resource(&raw, true, &mut HashMap::new())
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }

    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let mut url = format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines?api-version={COMPUTE_API}",
            self.account.subscription_id, self.account.resource_group
        );
        let mut resources = Vec::new();
        let mut prices = HashMap::new();
        let mut seen = HashSet::new();
        loop {
            if !seen.insert(url.clone()) || seen.len() > 100 {
                return Err(Error::Provider(ProviderErrorCode::ProviderError));
            }
            let page = self
                .arm(Method::GET, &url, None)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
            for raw in page["value"]
                .as_array()
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
            {
                if raw["tags"]["bpg-namespace"].as_str() != Some(namespace) {
                    continue;
                }
                if let Some(resource) = self.resource(raw, false, &mut prices).await? {
                    resources.push(resource);
                }
            }
            match page["nextLink"].as_str().filter(|next| !next.is_empty()) {
                Some(next) => url = next.to_owned(),
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
        self.arm(Method::DELETE, &self.vm_path(id), None).await?;
        Ok(())
    }
}

#[async_trait]
impl CloudProviderAdapter for AzureManagedAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let raw = self
            .vm(id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))?;
        self.resource(&raw, true, &mut HashMap::new())
            .await?
            .map(|resource| resource.instance)
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))
    }
}
