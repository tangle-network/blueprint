//! Crusoe Cloud GPU VMs: operator-configured prices, template-carried ownership, async operations.
use super::CrusoeAdapter;
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    CONFIGURED_PRICING, ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter,
    ManagedResource, ManagedResourceKind, NativeOffer, ProviderErrorCode, bootstrap,
};
use crate::providers::common::gpu_adapter::poll_until;
use crate::security::{
    ApiAuthentication, SecureHttpClient, canonical_public_key, parse_public_key,
};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use reqwest::{Method, StatusCode};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::time::Duration;

const BASE_URL: &str = "https://api.cloud.crusoe.ai/v1";
/// Every Crusoe VM boots from a fixed 128 GB OS disk (docs.crusoe.ai, storage overview).
const OS_DISK_GB: f64 = 128.0;
const USER: &str = "ubuntu";

fn code(code: ProviderErrorCode) -> Error {
    Error::Provider(code)
}

pub struct CrusoeManagedAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    project_url: String,
    image: Option<String>,
    hourly_usd: BTreeMap<String, f64>,
    types: tokio::sync::OnceCell<Vec<Value>>,
}

impl CrusoeManagedAdapter {
    pub const ACCESS_KEY_ID_ENV: &'static str = "CRUSOE_ACCESS_KEY_ID";
    pub const SECRET_KEY_ENV: &'static str = "CRUSOE_SECRET_KEY";
    pub const PROJECT_ID_ENV: &'static str = "CRUSOE_PROJECT_ID";
    pub const HOURLY_USD_ENV: &'static str = "CRUSOE_GPU_HOURLY_USD";
    pub const IMAGE_ENV: &'static str = "CRUSOE_IMAGE";
    pub const REGION_ENV: &'static str = "CRUSOE_REGION";

    /// Crusoe publishes no price API: only the `type=usd,...` types an operator prices are offered.
    pub fn with_endpoint(
        access_key_id: String,
        secret_key: String,
        project_id: &str,
        hourly_usd: &str,
        image: Option<String>,
        endpoint: Option<&str>,
    ) -> Result<Self> {
        let invalid = || code(ProviderErrorCode::InvalidRequest);
        let mut prices = BTreeMap::new();
        for entry in hourly_usd.split(',').map(str::trim) {
            let (sku, usd) = entry.split_once('=').ok_or_else(invalid)?;
            let usd = usd.trim().parse::<f64>().map_err(|_| invalid())?;
            if sku.trim().is_empty()
                || !usd.is_finite()
                || usd <= 0.0
                || prices.insert(sku.trim().to_owned(), usd).is_some()
            {
                return Err(invalid());
            }
        }
        if uuid::Uuid::parse_str(project_id).is_err()
            || access_key_id.is_empty()
            || !access_key_id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || URL_SAFE_NO_PAD
                .decode(secret_key.trim_end_matches('='))
                .ok()
                .is_none_or(|key| key.is_empty())
            || image.as_ref().is_some_and(|image| {
                image.is_empty() || image.chars().any(|c| c.is_whitespace() || c.is_control())
            })
        {
            return Err(invalid());
        }
        let base = endpoint.unwrap_or(BASE_URL).trim_end_matches('/');
        Ok(Self {
            http: SecureHttpClient::with_endpoint(base)?,
            auth: ApiAuthentication::crusoe(access_key_id, secret_key),
            base_url: base.into(),
            project_url: format!("{base}/projects/{project_id}"),
            image,
            hourly_usd: prices,
            types: tokio::sync::OnceCell::new(),
        })
    }

    pub(crate) fn credential_binding(&self) -> Result<(String, Option<String>)> {
        Ok((
            crate::managed::authorization::credential_digest(
                &self.auth,
                &(&self.project_url, &self.image),
            )?,
            None,
        ))
    }

    /// `None` is a 404.
    async fn call(&self, method: Method, url: &str, body: Option<Value>) -> Result<Option<Value>> {
        let response = self
            .http
            .authenticated_request(method, url, &self.auth, body)
            .await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        response
            .json()
            .await
            .map(Some)
            .map_err(|_| code(ProviderErrorCode::ProviderError))
    }

    async fn get(&self, url: &str) -> Result<Value> {
        self.call(Method::GET, url, None)
            .await?
            .ok_or(code(ProviderErrorCode::ProviderError))
    }

    fn id(raw: &Value) -> Result<&str> {
        raw["id"]
            .as_str()
            .filter(|id| uuid::Uuid::parse_str(id).is_ok())
            .ok_or(code(ProviderErrorCode::ProviderError))
    }

    fn vm_url(&self, id: &str) -> Result<String> {
        uuid::Uuid::parse_str(id).map_err(|_| code(ProviderErrorCode::InvalidRequest))?;
        Ok(format!("{}/compute/vms/instances/{id}", self.project_url))
    }

    fn template_url(&self, id: &str) -> Result<String> {
        uuid::Uuid::parse_str(id).map_err(|_| code(ProviderErrorCode::InvalidRequest))?;
        Ok(format!(
            "{}/compute/instance-templates/{id}",
            self.project_url
        ))
    }

    /// The project's VM types, fetched once per adapter.
    async fn types(&self) -> Result<&Vec<Value>> {
        self.types
            .get_or_try_init(|| async {
                self.get(&format!("{}/compute/vms/types", self.project_url))
                    .await?["items"]
                    .as_array()
                    .cloned()
                    .ok_or(code(ProviderErrorCode::ProviderError))
            })
            .await
    }

    /// The catalog class whose exact hardware name a VM type advertises.
    fn class(native: &Value) -> Option<(blueprint_core::gpu::GpuClass, u32)> {
        let count = u32::try_from(native["num_gpu"].as_u64()?).ok()?;
        let name = native["gpu_type"].as_str()?;
        let mut matches = blueprint_core::gpu::catalog()
            .into_iter()
            .filter(|class| count > 0 && count <= class.max_per_node && class.matches_name(name));
        let class = matches.next()?;
        matches.next().is_none().then_some((class, count))
    }

    /// SXM nodes need the image with the NVSwitch fabric manager.
    fn image_for(&self, sku: &str) -> &str {
        match &self.image {
            Some(image) => image,
            None if sku.contains("-sxm") => "ubuntu22.04-nvidia-sxm-docker",
            None => "ubuntu22.04-nvidia-pcie-docker",
        }
    }

    /// The lease label is the template startup script's second line.
    fn metadata(template: &Value) -> Result<Option<ManagedMetadata>> {
        match template["startup_script"]
            .as_str()
            .and_then(|script| script.lines().nth(1))
            .and_then(|line| line.strip_prefix("# "))
        {
            Some(label) => ManagedMetadata::from_label(label),
            None => Ok(None),
        }
    }

    /// Templates carrying a lease label, by template id.
    async fn templates(&self) -> Result<Vec<(String, ManagedMetadata)>> {
        let page = self
            .get(&format!("{}/compute/instance-templates", self.project_url))
            .await?;
        let mut owned = Vec::new();
        for raw in page["items"]
            .as_array()
            .ok_or(code(ProviderErrorCode::ProviderError))?
        {
            if let Some(metadata) = Self::metadata(raw)? {
                owned.push((Self::id(raw)?.to_owned(), metadata));
            }
        }
        Ok(owned)
    }

    async fn vms(&self) -> Result<Vec<Value>> {
        let mut vms = Vec::new();
        let mut seen = HashSet::new();
        let mut url = format!("{}/compute/vms/instances", self.project_url);
        loop {
            let page = self.get(&url).await?;
            vms.extend(
                page["items"]
                    .as_array()
                    .ok_or(code(ProviderErrorCode::ProviderError))?
                    .iter()
                    .cloned(),
            );
            match page["next_page_token"]
                .as_str()
                .filter(|next| !next.is_empty())
            {
                Some(next) if seen.insert(next.to_owned()) => {
                    url = format!(
                        "{}/compute/vms/instances?next_token={}",
                        self.project_url,
                        urlencoding::encode(next)
                    );
                }
                Some(_) => return Err(code(ProviderErrorCode::ProviderError)),
                None => return Ok(vms),
            }
        }
    }

    async fn resource(&self, vm: &Value, metadata: ManagedMetadata) -> Result<ManagedResource> {
        let mut resource =
            metadata.bind(CrusoeAdapter::parse_vm(vm)?, ManagedResourceKind::Instance);
        let native = self
            .types()
            .await?
            .iter()
            .find(|native| native["product_name"] == vm["type"]);
        if let Some((class, count)) = native.and_then(Self::class) {
            resource.gpu_class = Some(class.id);
            resource.gpu_vram_gb = Some(class.vram_gb);
            resource.gpu_count = Some(count);
        }
        resource.hourly_cost_usd = self
            .hourly_usd
            .get(&resource.instance.instance_type)
            .copied();
        resource.created_at = vm["created_at"].as_str().and_then(|time| time.parse().ok());
        resource.ssh_endpoint = resource
            .instance
            .public_ip
            .as_ref()
            .and_then(|ip| ip.parse::<std::net::Ipv4Addr>().ok())
            .map(|ip| format!("ssh://{USER}@{ip}:22"));
        Ok(resource)
    }

    /// A present VM, its template, and the lease that template records.
    async fn observe(&self, id: &str) -> Result<Option<(ManagedResource, Value)>> {
        let Some(vm) = self.call(Method::GET, &self.vm_url(id)?, None).await? else {
            return Ok(None);
        };
        let template = vm["instance_template_id"]
            .as_str()
            .ok_or(code(ProviderErrorCode::Conflict))?;
        let template = self
            .call(Method::GET, &self.template_url(template)?, None)
            .await?
            .ok_or(code(ProviderErrorCode::Conflict))?;
        let metadata = Self::metadata(&template)?.ok_or(code(ProviderErrorCode::Conflict))?;
        Ok(Some((self.resource(&vm, metadata).await?, template)))
    }

    /// Wait until an accepted operation leaves `IN_PROGRESS`.
    async fn operation(&self, accepted: &Value) -> Result<Value> {
        let operation = &accepted["operation"];
        if operation["state"] != "IN_PROGRESS" {
            return Ok(operation.clone());
        }
        let url = operation["operation_id"]
            .as_str()
            .filter(|id| uuid::Uuid::parse_str(id).is_ok())
            .map(|id| format!("{}/compute/vms/instances/operations/{id}", self.project_url))
            .ok_or(code(ProviderErrorCode::Indeterminate))?;
        poll_until(
            "Crusoe operation",
            Duration::from_secs(5),
            Duration::from_secs(600),
            || async {
                let operation = self.get(&url).await?;
                Ok((operation["state"] != "IN_PROGRESS").then_some(operation))
            },
        )
        .await
    }

    fn unsupported(spec: &ResourceSpec) -> bool {
        spec.tee_required
            || spec.volume_gb.is_some()
            || spec.volume_id.is_some()
            || spec.source_network.is_some()
    }
}

#[async_trait]
impl ManagedProviderAdapter for CrusoeManagedAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        spec.validate()
            .map_err(|_| code(ProviderErrorCode::InvalidRequest))?;
        if Self::unsupported(spec) {
            return Err(code(ProviderErrorCode::Unsupported));
        }
        let count = spec
            .gpu_count
            .ok_or(code(ProviderErrorCode::InvalidRequest))?;
        let locations = self.get(&format!("{}/locations", self.base_url)).await?;
        let locations: HashSet<&str> = locations["items"]
            .as_array()
            .ok_or(code(ProviderErrorCode::ProviderError))?
            .iter()
            .filter_map(Value::as_str)
            .filter(|location| region.is_empty() || *location == region)
            .collect();
        let capacity = self.get(&format!("{}/capacities", self.base_url)).await?;
        let stocked: BTreeSet<(&str, &str)> = capacity["items"]
            .as_array()
            .ok_or(code(ProviderErrorCode::ProviderError))?
            .iter()
            .filter(|row| {
                row["quantity"]
                    .as_u64()
                    .is_some_and(|quantity| quantity > 0)
            })
            .filter_map(|row| Some((row["type"].as_str()?, row["location"].as_str()?)))
            .filter(|(_, location)| locations.contains(location))
            .collect();
        let mut offers = Vec::new();
        for native in self.types().await? {
            let (Some(sku), Some((class, n))) =
                (native["product_name"].as_str(), Self::class(native))
            else {
                continue;
            };
            let Some(&price) = self.hourly_usd.get(sku) else {
                continue;
            };
            if n != count
                || spec.gpu_class.as_ref().is_some_and(|id| id != &class.id)
                || spec
                    .gpu_vram_gb
                    .is_some_and(|minimum| class.vram_gb < minimum)
                || native["cpu_cores"]
                    .as_f64()
                    .is_none_or(|cpu| cpu < f64::from(spec.cpu))
                || native["memory_gb"]
                    .as_f64()
                    .is_none_or(|memory| memory < f64::from(spec.memory_gb))
                || f64::from(spec.storage_gb)
                    > native["disk_gb"].as_f64().unwrap_or(0.0).max(OS_DISK_GB)
            {
                continue;
            }
            for (_, location) in stocked.iter().filter(|(row, _)| *row == sku) {
                offers.push(NativeOffer {
                    provider: CloudProvider::Crusoe,
                    native_id: sku.into(),
                    instance_type: sku.into(),
                    region: (*location).into(),
                    gpu_class: class.id.clone(),
                    gpu_count: count,
                    hourly_cost_usd: price,
                    spot: false,
                    pricing_source: CONFIGURED_PRICING.into(),
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
            .map_err(|_| code(ProviderErrorCode::InvalidRequest))?;
        if Self::unsupported(spec) || offer.spot || offer.provider != CloudProvider::Crusoe {
            return Err(code(ProviderErrorCode::Unsupported));
        }
        let key = canonical_public_key(
            spec.ssh_public_key
                .as_deref()
                .ok_or(code(ProviderErrorCode::Unsupported))?,
        )?;
        let mut script = format!("#!/bin/bash\n# {}\n", metadata.label()?);
        if let Some(image) = &spec.image {
            let mut env = bootstrap::workload_environment(spec)?;
            env.extend(metadata.env());
            script.push_str(&bootstrap::gpu_container_startup_script(
                image,
                &env,
                &spec.ports,
                spec.command.as_deref(),
            )?);
        } else if !spec.env.is_empty() || spec.command.is_some() || spec.manifest.is_some() {
            return Err(code(ProviderErrorCode::Unsupported));
        }
        // Crusoe takes startup scripts under 64 KB.
        if script.len() >= 65_536 {
            return Err(code(ProviderErrorCode::Unsupported));
        }
        metadata.check_deadline()?;
        let mut owned = self
            .templates()
            .await?
            .into_iter()
            .filter(|(_, existing)| existing.identity == metadata.identity);
        // A template left by an interrupted create is reused only with its exact lease.
        let template = match (owned.next(), owned.next()) {
            (None, _) => {
                let body = json!({"template_name":metadata.name(),"type":offer.instance_type,"ssh_public_key":key,
                    "image_name":self.image_for(&offer.instance_type),"startup_script":script,"public_ip_address_type":"dynamic"});
                metadata.check_deadline()?;
                let url = format!("{}/compute/instance-templates", self.project_url);
                let created = self
                    .call(Method::POST, &url, Some(body))
                    .await?
                    .ok_or(code(ProviderErrorCode::Indeterminate))?;
                Self::id(&created)
                    .map_err(|_| code(ProviderErrorCode::Indeterminate))?
                    .to_owned()
            }
            (Some((id, existing)), None)
                if existing.request_fingerprint == metadata.request_fingerprint
                    && existing.created_at == metadata.created_at
                    && existing.expires_at == metadata.expires_at =>
            {
                id
            }
            _ => return Err(code(ProviderErrorCode::Conflict)),
        };
        // The VM records the template it came from, which carries the lease.
        let body = json!({"name_prefix":metadata.name(),"count":1,"instance_template_id":template,
            "locations":[{"location":offer.region}]});
        metadata.check_deadline()?;
        let url = format!("{}/compute/vms/bulk-instances", self.project_url);
        let accepted = self
            .call(Method::POST, &url, Some(body))
            .await?
            .ok_or(code(ProviderErrorCode::Indeterminate))?;
        // After an accepted create, any polling failure leaves the outcome unknown.
        let done = self
            .operation(&accepted)
            .await
            .map_err(|_| code(ProviderErrorCode::Indeterminate))?;
        match done["state"].as_str() {
            Some("SUCCEEDED") => {}
            Some("FAILED") => {
                let quota = done["result"]
                    .to_string()
                    .to_ascii_lowercase()
                    .contains("quota");
                return Err(code(if quota {
                    ProviderErrorCode::Quota
                } else {
                    ProviderErrorCode::NoStock
                }));
            }
            _ => return Err(code(ProviderErrorCode::Indeterminate)),
        }
        let vm = done["result"]
            .as_array()
            .filter(|vms| vms.len() == 1)
            .ok_or(code(ProviderErrorCode::Indeterminate))?;
        let instance =
            CrusoeAdapter::parse_vm(&vm[0]).map_err(|_| code(ProviderErrorCode::Indeterminate))?;
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
        let (mut observed, template) = self
            .observe(&resource.instance.id)
            .await?
            .ok_or(code(ProviderErrorCode::Indeterminate))?;
        if let Some(expected) = spec.ssh_public_key.as_deref() {
            let installed = template["ssh_public_key"]
                .as_str()
                .ok_or(code(ProviderErrorCode::Conflict))?;
            if parse_public_key(installed)?.key_data() != parse_public_key(expected)?.key_data() {
                return Err(code(ProviderErrorCode::Conflict));
            }
        }
        observed.hourly_cost_usd = resource.hourly_cost_usd.or(observed.hourly_cost_usd);
        Ok(observed)
    }

    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        let Some((resource, _)) = self.observe(id).await? else {
            return Ok(None);
        };
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(code(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }

    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let templates: HashMap<_, _> = self
            .templates()
            .await?
            .into_iter()
            .filter(|(_, metadata)| metadata.identity.namespace == namespace)
            .collect();
        let mut resources = Vec::new();
        for vm in self.vms().await? {
            if let Some(metadata) = vm["instance_template_id"]
                .as_str()
                .and_then(|id| templates.get(id))
            {
                resources.push(self.resource(&vm, metadata.clone()).await?);
            }
        }
        for (id, metadata) in templates {
            resources.push(metadata.bind(
                ProvisionedInstance {
                    id,
                    provider: CloudProvider::Crusoe,
                    instance_type: "template".into(),
                    region: String::new(),
                    public_ip: None,
                    private_ip: None,
                    status: InstanceStatus::Running,
                },
                ManagedResourceKind::Template,
            ));
        }
        Ok(resources)
    }

    async fn destroy(
        &self,
        identity: &ManagedLeaseIdentity,
        resource: &ManagedResource,
    ) -> Result<()> {
        let id = resource.instance.id.as_str();
        match resource.kind {
            ManagedResourceKind::Instance => {
                if <Self as ManagedProviderAdapter>::status(self, identity, id)
                    .await?
                    .is_none()
                {
                    return Ok(());
                }
                let Some(accepted) = self.call(Method::DELETE, &self.vm_url(id)?, None).await?
                else {
                    return Ok(());
                };
                // The template is the VM's only lease record, so it must outlive the VM.
                let done = self.operation(&accepted).await.ok();
                if done.is_none_or(|done| done["state"] != "SUCCEEDED") {
                    return Err(code(ProviderErrorCode::StillPresent));
                }
                Ok(())
            }
            ManagedResourceKind::Template => {
                let url = self.template_url(id)?;
                let Some(template) = self.call(Method::GET, &url, None).await? else {
                    return Ok(());
                };
                if Self::metadata(&template)?.is_none_or(|metadata| metadata.identity != *identity)
                {
                    return Err(code(ProviderErrorCode::Conflict));
                }
                if self
                    .vms()
                    .await?
                    .iter()
                    .any(|vm| vm["instance_template_id"] == id)
                {
                    return Err(code(ProviderErrorCode::StillPresent));
                }
                self.http.delete(&url, &self.auth).await?;
                Ok(())
            }
            _ => Err(code(ProviderErrorCode::Unsupported)),
        }
    }
}

#[async_trait]
impl CloudProviderAdapter for CrusoeManagedAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        self.observe(id)
            .await?
            .map(|(resource, _)| resource.instance)
            .ok_or(code(ProviderErrorCode::NotFound))
    }
}
