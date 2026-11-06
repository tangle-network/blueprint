//! Lambda Cloud native SKUs, atomic instance tags and namespace-scoped recovery.
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode, bootstrap,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};

const BASE_URL: &str = "https://cloud.lambda.ai/api/v1";

pub struct LambdaLabsAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    default_region: Option<String>,
    ssh_key_name: Option<String>,
    firewall_ruleset_ids: Vec<String>,
}

impl LambdaLabsAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("LAMBDA_LABS_API_KEY")
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderNotConfigured))?;
        let mut adapter = Self::with_credentials(key)?;
        adapter.default_region = std::env::var("LAMBDA_LABS_REGION").ok();
        adapter.ssh_key_name = std::env::var("LAMBDA_LABS_SSH_KEY_NAME").ok();
        Ok(adapter)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Self::with_endpoint(api_key, BASE_URL)
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        Ok(Self {
            http: SecureHttpClient::with_endpoint(endpoint)?,
            auth: ApiAuthentication::lambda_labs(api_key),
            base_url: endpoint.trim_end_matches('/').into(),
            default_region: None,
            ssh_key_name: None,
            firewall_ruleset_ids: Vec::new(),
        })
    }

    #[must_use]
    pub fn with_ssh_key_name(mut self, name: String) -> Self {
        self.ssh_key_name = Some(name);
        self
    }

    #[must_use]
    pub fn with_firewall_rulesets(mut self, ids: Vec<String>) -> Self {
        self.firewall_ruleset_ids = ids;
        self
    }

    async fn data(&self, path: &str) -> Result<Value> {
        let response = self
            .http
            .get(&format!("{}/{path}", self.base_url), &self.auth)
            .await?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(Error::Provider(ProviderErrorCode::NotFound));
        }
        let document: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        document
            .get("data")
            .cloned()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))
    }

    fn instance(raw: &Value) -> Result<ProvisionedInstance> {
        Ok(ProvisionedInstance {
            id: raw["id"]
                .as_str()
                .filter(|id| !id.is_empty())
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
                .into(),
            provider: CloudProvider::LambdaLabs,
            instance_type: raw["instance_type"]["name"].as_str().unwrap_or("").into(),
            region: raw["region"]["name"].as_str().unwrap_or("").into(),
            public_ip: raw["ip"].as_str().map(str::to_owned),
            private_ip: raw["private_ip"].as_str().map(str::to_owned),
            status: match raw["status"].as_str() {
                Some("active") => InstanceStatus::Running,
                Some("booting") => InstanceStatus::Starting,
                Some("terminating") => InstanceStatus::Stopping,
                Some("terminated") => InstanceStatus::Terminated,
                Some("preempted") => InstanceStatus::Stopped,
                _ => InstanceStatus::Unknown,
            },
        })
    }

    fn class(native: &Value, count: u32) -> Option<blueprint_core::gpu::GpuClass> {
        let classes = blueprint_core::gpu::catalog();
        let known = classes.iter().find(|class| {
            class.sku("lambda-labs", count) == native["name"].as_str() && native["name"].is_string()
        });
        let description = native["gpu_description"].as_str();
        let described = classes
            .iter()
            .filter(|class| {
                count <= class.max_per_node
                    && description.is_some_and(|name| class.matches_name(name))
            })
            .collect::<Vec<_>>();
        if let Some(known) = known {
            return (described.is_empty() || described.iter().any(|class| class.id == known.id))
                .then(|| known.clone());
        }
        (described.len() == 1).then(|| described[0].clone())
    }

    fn managed(raw: &Value) -> Result<Option<ManagedResource>> {
        let mut tags = BTreeMap::new();
        if let Some(values) = raw["tags"].as_array() {
            for tag in values {
                let key = tag["key"]
                    .as_str()
                    .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                let value = tag["value"]
                    .as_str()
                    .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                if tags.insert(key.into(), value.into()).is_some() {
                    return Err(Error::Provider(ProviderErrorCode::Conflict));
                }
            }
        } else if !raw["tags"].is_null() {
            return Err(Error::Provider(ProviderErrorCode::ProviderError));
        }
        let Some(metadata) = ManagedMetadata::from_tags(&tags)? else {
            return Ok(None);
        };
        let instance = Self::instance(raw)?;
        let count = raw["instance_type"]["specs"]["gpus"]
            .as_u64()
            .and_then(|count| u32::try_from(count).ok())
            .filter(|count| *count > 0);
        let class = count.and_then(|count| Self::class(&raw["instance_type"], count));
        let mut resource = metadata.bind(instance, ManagedResourceKind::Instance);
        resource.gpu_class = class.as_ref().map(|class| class.id.clone());
        resource.gpu_vram_gb = class.map(|class| class.vram_gb);
        resource.gpu_count = count;
        resource.hourly_cost_usd = raw["instance_type"]["price_cents_per_hour"]
            .as_u64()
            .filter(|cents| *cents > 0)
            .map(|cents| cents as f64 / 100.0);
        resource.created_at = raw["created_at"]
            .as_str()
            .or_else(|| raw["created"].as_str())
            .and_then(|date| date.parse().ok());
        resource.preempted = raw["status"].as_str() == Some("preempted");
        resource.ssh_endpoint = resource
            .instance
            .public_ip
            .as_ref()
            .and_then(|ip| ip.parse::<std::net::Ipv4Addr>().ok())
            .map(|ip| format!("ssh://ubuntu@{ip}:22"));
        Ok(Some(resource))
    }

    async fn verify_ssh_key(&self, spec: &ResourceSpec) -> Result<()> {
        let Some(public_key) = &spec.ssh_public_key else {
            return Ok(());
        };
        let unsupported = || Error::Provider(ProviderErrorCode::Unsupported);
        let parse = |value: &str| {
            if value.contains(['\0', '\r', '\n']) {
                return Err(unsupported());
            }
            ssh_key::PublicKey::from_openssh(value).map_err(|_| unsupported())
        };
        let expected = parse(public_key)?;
        let name = self
            .ssh_key_name
            .as_deref()
            .filter(|name| !name.is_empty())
            .ok_or_else(unsupported)?;
        let data = match self.data("ssh-keys").await {
            Err(Error::Provider(ProviderErrorCode::NotFound)) => return Err(unsupported()),
            result => result?,
        };
        let mut selected = data
            .as_array()
            .ok_or_else(unsupported)?
            .iter()
            .filter(|key| key["name"].as_str() == Some(name));
        let registered = selected
            .next()
            .and_then(|key| key["public_key"].as_str())
            .ok_or_else(unsupported)?;
        if selected.next().is_some() || parse(registered)?.key_data() != expected.key_data() {
            return Err(unsupported());
        }
        Ok(())
    }

    fn network(value: &str) -> Option<(u64, u64)> {
        let (address, bits) = value.split_once('/').unwrap_or((value, "32"));
        let address = u32::from(address.parse::<std::net::Ipv4Addr>().ok()?);
        let bits = bits.parse::<u32>().ok().filter(|bits| *bits <= 32)?;
        let mask = u32::MAX.checked_shl(32 - bits).unwrap_or(0);
        Some((u64::from(address & mask), u64::from(address | !mask)))
    }

    fn validate_firewall_ids(&self) -> Result<()> {
        let mut seen = HashSet::new();
        if self.firewall_ruleset_ids.iter().any(|id| {
            id.is_empty()
                || id.len() > 128
                || !id
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                || !seen.insert(id)
        }) {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        Ok(())
    }

    async fn verify_network(&self, spec: &ResourceSpec, region: &str) -> Result<()> {
        self.validate_firewall_ids()?;
        let unsupported = || Error::Provider(ProviderErrorCode::Unsupported);
        if region == "us-south-1" {
            return if spec.source_network.is_none() {
                Ok(())
            } else {
                Err(unsupported())
            };
        }
        let requested = spec
            .source_network
            .as_deref()
            .map(Self::network)
            .unwrap_or(Some((0, u64::from(u32::MAX))))
            .ok_or_else(unsupported)?;
        let global = self.data("firewall-rulesets/global").await?;
        let mut rules = global["rules"]
            .as_array()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
            .clone();
        for id in &self.firewall_ruleset_ids {
            let set = self
                .data(&format!("firewall-rulesets/{}", urlencoding::encode(id)))
                .await?;
            if set["id"].as_str() != Some(id.as_str())
                || set["region"]["name"].as_str() != Some(region)
            {
                return Err(unsupported());
            }
            rules.extend(
                set["rules"]
                    .as_array()
                    .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
                    .iter()
                    .cloned(),
            );
        }
        let mut effective = Vec::new();
        for rule in &rules {
            let protocol = match rule["protocol"].as_str() {
                Some(protocol @ ("tcp" | "udp" | "icmp" | "all")) => protocol,
                _ => return Err(unsupported()),
            };
            let source = rule["source_network"]
                .as_str()
                .and_then(Self::network)
                .ok_or_else(unsupported)?;
            if source.0 < requested.0 || source.1 > requested.1 {
                return Err(unsupported());
            }
            let (low, high) = if protocol == "icmp" {
                (0, 0)
            } else {
                let range = rule["port_range"]
                    .as_array()
                    .filter(|range| range.len() == 2)
                    .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                let low = range[0]
                    .as_u64()
                    .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                let high = range[1]
                    .as_u64()
                    .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                if !(1..=65535).contains(&low) || high < low || high > 65535 {
                    return Err(Error::Provider(ProviderErrorCode::ProviderError));
                }
                (low, high)
            };
            effective.push((protocol, low, high, source));
        }
        let mut ports = spec.ports.clone();
        ports.push(22);
        ports.sort_unstable();
        ports.dedup();
        for port in ports {
            let mut intervals = effective
                .iter()
                .filter(|(protocol, low, high, _)| {
                    matches!(*protocol, "tcp" | "all")
                        && u64::from(port) >= *low
                        && u64::from(port) <= *high
                })
                .map(|(_, _, _, source)| *source)
                .collect::<Vec<_>>();
            intervals.sort_unstable();
            let mut covered = requested.0;
            for (start, end) in intervals {
                if start > covered {
                    return Err(unsupported());
                }
                covered = covered.max(end + 1);
            }
            if covered <= requested.1 {
                return Err(unsupported());
            }
        }
        Ok(())
    }
}

#[async_trait]
impl ManagedProviderAdapter for LambdaLabsAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        self.validate_firewall_ids()?;
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.tee_required || spec.volume_gb.is_some() {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        self.verify_ssh_key(spec).await?;
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let types = self.data("instance-types").await?;
        let types = types
            .as_object()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let region = if region.is_empty() {
            self.default_region.as_deref().unwrap_or("")
        } else {
            region
        };
        let mut offers = Vec::new();
        for entry in types.values() {
            let native = &entry["instance_type"];
            let Some(sku) = native["name"].as_str() else {
                continue;
            };
            if native["specs"]["gpus"].as_u64() != Some(u64::from(count)) {
                continue;
            }
            let Some(class) = Self::class(native, count) else {
                continue;
            };
            if spec.gpu_class.as_ref().is_some_and(|id| id != &class.id)
                || spec
                    .gpu_vram_gb
                    .is_some_and(|minimum| class.vram_gb < minimum)
                || native["specs"]["vcpus"]
                    .as_f64()
                    .is_none_or(|cpu| cpu < f64::from(spec.cpu))
                || native["specs"]["memory_gib"]
                    .as_f64()
                    .is_none_or(|memory| memory * 1.073_741_824 < f64::from(spec.memory_gb))
                || native["specs"]["storage_gib"]
                    .as_f64()
                    .is_none_or(|storage| storage * 1.073_741_824 < f64::from(spec.storage_gb))
            {
                continue;
            }
            let Some(price) = native["price_cents_per_hour"]
                .as_u64()
                .filter(|price| *price > 0)
                .map(|price| price as f64 / 100.0)
            else {
                continue;
            };
            let Some(regions) = entry["regions_with_capacity_available"].as_array() else {
                continue;
            };
            for capacity in regions {
                let Some(id) = capacity["name"].as_str().filter(|id| !id.is_empty()) else {
                    continue;
                };
                if !region.is_empty() && id != region {
                    continue;
                }
                offers.push(NativeOffer {
                    provider: CloudProvider::LambdaLabs,
                    native_id: sku.into(),
                    instance_type: sku.into(),
                    region: id.into(),
                    gpu_class: class.id.clone(),
                    gpu_count: count,
                    hourly_cost_usd: price,
                    spot: false,
                    pricing_source: format!("{}/instance-types", self.base_url),
                    underlying: None,
                });
            }
        }
        let had_stock = !offers.is_empty();
        let mut supported = Vec::new();
        for offer in offers {
            match self.verify_network(spec, &offer.region).await {
                Ok(()) => supported.push(offer),
                Err(Error::Provider(ProviderErrorCode::Unsupported)) => {}
                Err(error) => return Err(error),
            }
        }
        if had_stock && supported.is_empty() {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        Ok(supported)
    }

    async fn create(
        &self,
        spec: &ResourceSpec,
        offer: &NativeOffer,
        fingerprint: &str,
    ) -> Result<ManagedResource> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.tee_required
            || spec.volume_gb.is_some()
            || offer.spot
            || offer.provider != CloudProvider::LambdaLabs
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        self.verify_network(spec, &offer.region).await?;
        self.verify_ssh_key(spec).await?;
        let key = self
            .ssh_key_name
            .as_ref()
            .filter(|name| !name.is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
        let metadata = ManagedMetadata::for_spec(spec, fingerprint, chrono::Utc::now())?;
        let mut body = json!({"name":metadata.name(),"region_name":offer.region,"instance_type_name":offer.instance_type,
            "quantity":1,"ssh_key_names":[key],"tags":metadata.tags().into_iter().map(|(key,value)|json!({"key":key,"value":value})).collect::<Vec<_>>(),
            "firewall_rulesets":self.firewall_ruleset_ids.iter().map(|id|json!({"id":id})).collect::<Vec<_>>()});
        let mut mounts = Vec::new();
        if let Some(id) = &spec.volume_id {
            let path = spec
                .volume_mount_path
                .as_deref()
                .unwrap_or("/mnt/blueprint-data");
            body["file_system_mounts"] = json!([{"file_system_id":id,"mount_point":path}]);
            mounts.push((path.to_owned(), path.to_owned()));
        }
        if let Some(image) = &spec.image {
            let mut env = bootstrap::workload_environment(spec)?;
            env.extend(metadata.env());
            let user_data = bootstrap::gpu_container_cloud_init(
                image,
                &env,
                &spec.ports,
                &mounts,
                spec.command.as_deref(),
            )?;
            if user_data.len() > 1_048_576 {
                return Err(Error::Provider(ProviderErrorCode::Unsupported));
            }
            body["user_data"] = json!(user_data);
        } else if !spec.env.is_empty() || spec.command.is_some() || spec.manifest.is_some() {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let response = self
            .http
            .post_json(
                &format!("{}/instance-operations/launch", self.base_url),
                &self.auth,
                body,
            )
            .await?;
        let response: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        let ids = response["data"]["instance_ids"]
            .as_array()
            .filter(|ids| ids.len() == 1)
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let id = ids[0]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?;
        let instance = ProvisionedInstance {
            id: id.into(),
            provider: CloudProvider::LambdaLabs,
            instance_type: offer.instance_type.clone(),
            region: offer.region.clone(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        };
        let mut resource = metadata.bind(instance, ManagedResourceKind::Instance);
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
        let raw = match self
            .data(&format!("instances/{}", urlencoding::encode(id)))
            .await
        {
            Ok(value) => value,
            Err(Error::Provider(ProviderErrorCode::NotFound)) => return Ok(None),
            Err(error) => return Err(error),
        };
        let resource = Self::managed(&raw)?.ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }

    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let mut resources = Vec::new();
        let data = self.data("instances").await?;
        for raw in data
            .as_array()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
        {
            if let Some(resource) = Self::managed(raw)? {
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
            .post_json(
                &format!("{}/instance-operations/terminate", self.base_url),
                &self.auth,
                json!({"instance_ids":[id]}),
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl CloudProviderAdapter for LambdaLabsAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::instance(
            &self
                .data(&format!("instances/{}", urlencoding::encode(id)))
                .await?,
        )
    }
}
