//! Lium public inventory quotes and exact-node native rentals.
use crate::{
    core::error::{Error, Result},
    core::remote::CloudProvider,
    core::resources::ResourceSpec,
    infra::{
        traits::{BlueprintDeploymentResult, CloudProviderAdapter},
        types::{InstanceStatus, ProvisionedInstance},
    },
    providers::common::{
        InstanceSelection,
        gpu_adapter::{
            api_json, build_http_client, catalog_skus, deploy_via_ssh, generate_instance_name,
            provision_with_cleanup, quoted_selection, reject_tee, require_public_ip,
        },
    },
    security::{ApiAuthentication, SecureHttpClient},
    shared::SshDeploymentConfig,
};
use async_trait::async_trait;
use blueprint_std::collections::HashMap;
use serde_json::{Value, json};

const BASE_URL: &str = "https://lium.io/api";
#[derive(serde::Serialize, serde::Deserialize)]
struct NodePick {
    executor_id: String,
    gpu_model: String,
    price_per_gpu_hour: f64,
    gpu_count: u32,
    spot: bool,
    gpu_class: String,
}
pub struct BittensorLiumAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    pricing_url: String,
}

impl BittensorLiumAdapter {
    pub async fn new() -> Result<Self> {
        Self::with_credentials(
            std::env::var("LIUM_API_KEY").map_err(|_| configuration("LIUM_API_KEY is required"))?,
        )
    }
    pub fn with_credentials(key: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::ApiKey {
                key,
                header_name: "X-API-Key".into(),
            },
            base_url: BASE_URL.into(),
            pricing_url: "https://lium.io/pricing.json".into(),
        })
    }
    pub fn with_endpoint(key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        adapter.pricing_url = format!("{}/pricing.json", adapter.base_url);
        Ok(adapter)
    }

    async fn candidates(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NodePick>> {
        if spec.source_network.is_some() {
            return Err(configuration(
                "Lium cannot enforce caller source-network restrictions",
            ));
        }
        let region = spec.region.as_deref().unwrap_or(region);
        catalog_skus(spec, "bittensor-lium")?;
        if spec.volume_gb.is_some_and(|size| size > 0.0) {
            return Err(configuration(
                "Lium attached volumes require an existing native volume id",
            ));
        }
        let nodes = api_json(
            self.http
                .get(
                    &format!("{}/public/v1/nodes", self.base_url),
                    &ApiAuthentication::None,
                )
                .await?,
        )
        .await?;
        let pricing = api_json(
            self.http
                .get(&self.pricing_url, &ApiAuthentication::None)
                .await?,
        )
        .await?;
        if pricing["stale"].as_bool() != Some(false) {
            return Err(configuration(
                "Lium public pricing feed is stale or invalid",
            ));
        }
        let models = pricing["models"]
            .as_array()
            .ok_or_else(|| configuration("Lium model metadata is unavailable"))?;
        let count = spec.gpu_count.unwrap_or(0);
        let classes = blueprint_core::gpu::catalog();
        let mut choices = Vec::new();
        for node in nodes["nodes"]
            .as_array()
            .ok_or_else(|| configuration("Lium public node inventory is unavailable"))?
        {
            let Some(model) = node["gpu_model"].as_str() else {
                continue;
            };
            let Some(id) = node["id"]
                .as_str()
                .filter(|id| uuid::Uuid::parse_str(id).is_ok())
            else {
                continue;
            };
            let total = node["gpu_count"].as_u64().unwrap_or(0);
            if count == 0
                || total == 0
                || u64::from(count) > total
                || node["available_now"].as_bool() != Some(true)
                || node["available_gpu_count"].as_u64().unwrap_or(0) < u64::from(count)
                || node["min_rentable_gpu_count"]
                    .as_u64()
                    .is_none_or(|min| u64::from(count) < min)
                || !(node["tier"].as_str() == Some("secure")
                    || spec.allow_spot && node["tier"].as_str() == Some("spot"))
            {
                continue;
            }
            if !region.is_empty()
                && !["country_code", "country", "region", "region_name", "city"]
                    .iter()
                    .any(|key| {
                        node[key]
                            .as_str()
                            .is_some_and(|value| value.eq_ignore_ascii_case(region))
                    })
            {
                continue;
            }
            let Some(class) = classes.iter().find(|class| {
                spec.gpu_class.as_ref().is_none_or(|id| id == &class.id)
                    && class.vram_gb >= spec.gpu_vram_gb.unwrap_or(0)
                    && class
                        .sku("bittensor-lium", count)
                        .is_some_and(|sku| model_matches(sku, model))
            }) else {
                continue;
            };
            let sku = class.sku("bittensor-lium", count).unwrap();
            let exact = models
                .iter()
                .find(|m| m["gpu_model"].as_str() == Some(model));
            let nominal = if let Some(exact) = exact {
                exact["vram_gb"].as_u64()
            } else {
                let family: Vec<_> = models
                    .iter()
                    .filter(|m| {
                        m["gpu_model"]
                            .as_str()
                            .is_some_and(|m| model_matches(sku, m))
                    })
                    .collect();
                (!family.is_empty()
                    && family
                        .iter()
                        .all(|m| m["vram_gb"].as_u64() == Some(u64::from(class.vram_gb))))
                .then_some(u64::from(class.vram_gb))
            };
            // The public model feed carries nominal VRAM; node driver reports
            // can be smaller GiB figures (B300 269 versus nominal 288).
            if nominal.is_none_or(|vram| vram < u64::from(class.vram_gb))
                || node["gpu_memory_gb"].as_f64().is_none_or(|gib| {
                    gib * 1_073_741_824.0 < f64::from(class.vram_gb) * 1_000_000_000.0
                })
            {
                continue;
            }
            let fraction = f64::from(count) / total as f64;
            if node["cpu_count"]
                .as_f64()
                .is_none_or(|cpu| cpu * fraction < f64::from(spec.cpu))
                || node["ram_gb"]
                    .as_f64()
                    .is_none_or(|ram| (ram - 2.0) * fraction < f64::from(spec.memory_gb))
                || node["disk_free_gb"]
                    .as_f64()
                    .is_none_or(|disk| disk * fraction < f64::from(spec.storage_gb))
            {
                continue;
            }
            let Some(price) = crate::providers::common::gpu_adapter::provider_hourly_price(
                &node["price_per_gpu_hour"],
            ) else {
                continue;
            };
            if spec
                .max_hourly_cost_usd
                .is_some_and(|cap| price * f64::from(count) > cap)
            {
                continue;
            }
            choices.push(NodePick {
                executor_id: id.into(),
                gpu_model: model.into(),
                price_per_gpu_hour: price,
                gpu_count: count,
                spot: node["tier"].as_str() == Some("spot"),
                gpu_class: class.id.clone(),
            });
        }
        choices.sort_by(|a, b| a.price_per_gpu_hour.total_cmp(&b.price_per_gpu_hour));
        Ok(choices)
    }

    async fn pods(&self) -> Result<Vec<Value>> {
        api_json(
            self.http
                .get(&format!("{}/pods", self.base_url), &self.auth)
                .await?,
        )
        .await?
        .as_array()
        .cloned()
        .ok_or_else(|| configuration("Lium pod list is invalid"))
    }
    async fn fetch_rental_json(&self, id: &str) -> Result<Value> {
        api_json(
            self.http
                .get(&format!("{}/pods/{id}", self.base_url), &self.auth)
                .await?,
        )
        .await
    }
    fn parse_instance(raw: &Value) -> Result<ProvisionedInstance> {
        Ok(ProvisionedInstance {
            id: raw["id"]
                .as_str()
                .ok_or_else(|| configuration("Lium pod has no id"))?
                .into(),
            provider: CloudProvider::BittensorLium,
            instance_type: raw["gpu_name"]
                .as_str()
                .or_else(|| raw["executor"]["machine_name"].as_str())
                .unwrap_or_default()
                .into(),
            region: raw["executor"]["country_code"]
                .as_str()
                .unwrap_or_default()
                .into(),
            public_ip: raw["executor"]["executor_ip_address"]
                .as_str()
                .map(String::from),
            private_ip: None,
            status: if raw["node_offline"].as_bool() == Some(true) {
                InstanceStatus::Unknown
            } else {
                match raw["status"].as_str().unwrap_or_default() {
                    "RUNNING" => InstanceStatus::Running,
                    "FAILED" | "TERMINATED" | "DELETED" => InstanceStatus::Terminated,
                    _ => InstanceStatus::Starting,
                }
            },
        })
    }
    async fn cancel_rental(&self, id: &str) -> Result<()> {
        let path = if let Some(id) = id.strip_prefix("lium-template:") {
            uuid::Uuid::parse_str(id).map_err(|_| configuration("Invalid Lium template id"))?;
            format!("{}/templates/{id}", self.base_url)
        } else {
            format!("{}/pods/{id}", self.base_url)
        };
        let response = self.http.delete(&path, &self.auth).await?;
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(Error::HttpError(format!(
                "Lium terminate HTTP {}",
                response.status()
            )))
        }
    }

    async fn template(
        &self,
        spec: &ResourceSpec,
        name: &str,
        public_key: &str,
    ) -> Result<(String, String, usize)> {
        let templates = api_json(
            self.http
                .get(&format!("{}/templates", self.base_url), &self.auth)
                .await?,
        )
        .await?;
        if let Some(existing) = templates
            .as_array()
            .into_iter()
            .flatten()
            .find(|t| t["name"].as_str() == Some(name))
        {
            if existing["environment"]["TANGLE_GPU_REQUEST_FINGERPRINT"].as_str()
                != Some(crate::managed::request_fingerprint(spec)?.as_str())
            {
                return Err(configuration(
                    "Managed Lium template conflicts with this request",
                ));
            }
            let id = existing["id"]
                .as_str()
                .ok_or_else(|| configuration("Managed Lium template has no id"))?;
            let key = existing["environment"]["PUBLIC_KEY"]
                .as_str()
                .ok_or_else(|| configuration("Managed Lium template has no SSH key"))?;
            let ports = existing["internal_ports"]
                .as_array()
                .ok_or_else(|| configuration("Managed Lium template has no port metadata"))?
                .len();
            return Ok((id.into(), key.into(), ports));
        }
        let image = spec
            .image
            .as_deref()
            .unwrap_or(crate::providers::common::gpu_adapter::DEFAULT_GPU_IMAGE);
        let (reference, digest) = image
            .split_once('@')
            .map(|(r, d)| (r, Some(d)))
            .unwrap_or((image, None));
        let tag_index = reference
            .rfind(':')
            .filter(|index| reference.rfind('/').is_none_or(|slash| *index > slash));
        let (repository, tag) = tag_index
            .map(|index| (&reference[..index], &reference[index + 1..]))
            .unwrap_or((reference, "latest"));
        let tls = crate::providers::common::gpu_adapter::worker_tls_enabled(spec)?;
        if spec.env.contains_key("TANGLE_GPU_WORKER_TOKEN") && !tls {
            return Err(configuration("Lium worker tokens require configured TLS"));
        }
        let mut environment = crate::providers::common::gpu_adapter::workload_environment(
            spec,
            tls.then_some("pinned"),
        )?;
        environment.insert("PUBLIC_KEY".into(), public_key.into());
        let ports = crate::providers::common::gpu_adapter::workload_ports(spec, true);
        let port_count = ports.len();
        let mut body = json!({"name":name,"docker_image":repository,"docker_image_tag":tag,"docker_image_digest":digest.unwrap_or(""),"environment":environment,"internal_ports":ports,"is_private":true,"one_time_template":true,"container_start_immediately":true});
        if let Some(command) = &spec.command {
            body["startup_commands"] = json!(
                command
                    .iter()
                    .map(|word| shell_escape::escape(word.as_str().into()).into_owned())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        let created = api_json(
            self.http
                .post_json(&format!("{}/templates", self.base_url), &self.auth, body)
                .await?,
        )
        .await?;
        let id = created["id"]
            .as_str()
            .ok_or_else(|| configuration("Lium template create returned no id"))?;
        Ok((id.into(), public_key.into(), port_count))
    }
}
fn configuration(message: &str) -> Error {
    Error::ConfigurationError(message.into())
}
fn model_matches(sku: &str, model: &str) -> bool {
    fn tokens(value: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut token = String::new();
        let mut digit = None;
        for ch in value.chars() {
            if !ch.is_ascii_alphanumeric() {
                if !token.is_empty() {
                    result.push(std::mem::take(&mut token));
                }
                digit = None;
                continue;
            }
            let next = ch.is_ascii_digit();
            if digit.is_some_and(|old| old != next) && !token.is_empty() {
                result.push(std::mem::take(&mut token));
            }
            token.push(ch.to_ascii_lowercase());
            digit = Some(next);
        }
        if !token.is_empty() {
            result.push(token);
        }
        result
    }
    let wanted = tokens(sku);
    let actual = tokens(model);
    !wanted.is_empty() && wanted.iter().all(|token| actual.contains(token))
}

fn lium_endpoint(raw: &Value, port: u16) -> Option<String> {
    let mappings = if let Some(text) = raw["ports_mapping"].as_str() {
        serde_json::from_str::<Value>(text).ok()?
    } else {
        raw["ports_mapping"].clone()
    };
    let value = &mappings[port.to_string()];
    let external = value
        .as_u64()
        .or_else(|| value.as_str()?.parse().ok())
        .and_then(|p| u16::try_from(p).ok())
        .filter(|p| *p > 0)?;
    let host = raw["executor"]["executor_ip_address"].as_str()?;
    let host = if host.parse::<std::net::Ipv6Addr>().is_ok() {
        format!("[{host}]")
    } else {
        host.to_string()
    };
    let url = url::Url::parse(&format!("https://{host}:{external}")).ok()?;
    (url.username().is_empty() && url.password().is_none())
        .then(|| url.to_string().trim_end_matches('/').to_string())
}

#[async_trait]
impl CloudProviderAdapter for BittensorLiumAdapter {
    async fn quote(&self, spec: &ResourceSpec, region: &str) -> Result<InstanceSelection> {
        let pick = self
            .candidates(spec, region)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| configuration("Lium has no matching priced in-stock node"))?;
        let price = pick.price_per_gpu_hour * f64::from(pick.gpu_count);
        let mut selection = quoted_selection(
            serde_json::to_string(&pick).map_err(|_| configuration("Invalid Lium selection"))?,
            price,
        )?;
        selection.spot_capable = pick.spot;
        selection.gpu_class = Some(pick.gpu_class);
        Ok(selection)
    }
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        self.provision_resources(
            instance_type,
            region,
            &ResourceSpec::default().with_gpu(1),
            tee,
        )
        .await
    }
    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        reject_tee(tee)?;
        if spec.source_network.is_some() {
            return Err(Error::ConfigurationError(
                "Lium cannot enforce caller source-network restrictions".into(),
            ));
        }
        let name = if let Some(ownership) = &spec.ownership {
            crate::providers::common::gpu_adapter::managed_resource_name(
                &ownership.namespace,
                &ownership.lease_id,
            )?
        } else {
            generate_instance_name("lium")
        };
        if spec.ownership.is_some() {
            let existing: Vec<_> = self
                .pods()
                .await?
                .into_iter()
                .filter(|p| p["pod_name"].as_str() == Some(name.as_str()))
                .collect();
            if existing.len() > 1 {
                return Err(configuration(
                    "Multiple Lium pods claim this lease identity",
                ));
            }
            if let Some(existing) = existing.first() {
                if existing["template"]["environment"]["TANGLE_GPU_REQUEST_FINGERPRINT"].as_str()
                    != Some(crate::managed::request_fingerprint(spec)?.as_str())
                {
                    return Err(configuration(
                        "Managed Lium pod conflicts with this request",
                    ));
                }
                return Self::parse_instance(existing);
            }
        }
        let choices = self.candidates(spec, region).await?;
        let selected = serde_json::from_str::<NodePick>(instance_type).ok();
        let pick = choices
            .into_iter()
            .find(|p| {
                selected.as_ref().is_none_or(|selected| {
                    selected.executor_id == p.executor_id
                        && selected.gpu_count == p.gpu_count
                        && p.price_per_gpu_hour <= selected.price_per_gpu_hour
                })
            })
            .ok_or_else(|| {
                configuration("Quoted Lium node is no longer available at its quoted rate")
            })?;
        let hours = spec
            .lease_duration_secs
            .map(|secs| secs.div_ceil(3600))
            .unwrap_or(24);
        if !(1..=720).contains(&hours) {
            return Err(configuration(
                "Lium native lease lifetime must be1..720hours",
            ));
        }
        let access = crate::leases::allocation_access(spec.ssh_public_key.as_deref()).await?;
        let (template_id, public_key, port_count) =
            self.template(spec, &name, &access.public_key).await?;
        let body = json!({"pod_name":name,"template_id":template_id,"gpu_count":pick.gpu_count,"user_public_key":[public_key],"termination_hours":hours,"initial_port_count":port_count,"expected_price_per_gpu_hour":pick.price_per_gpu_hour,"skip_add_agent_ssh_key":true});
        let url = format!("{}/executors/{}/rent", self.base_url, pick.executor_id);
        let response = if spec.ownership.is_some() {
            self.http
                .post_json_idempotent(&url, &self.auth, body, &name)
                .await?
        } else {
            self.http.post_json(&url, &self.auth, body).await?
        };
        let created = api_json(response).await?;
        let id = created["pod_id"]
            .as_str()
            .or_else(|| created["id"].as_str())
            .ok_or_else(|| configuration("Lium create returned no pod id"))?;
        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::BittensorLium,
            id.into(),
            pick.gpu_model,
            region.into(),
        ))
    }
    async fn terminate_instance(&self, id: &str) -> Result<()> {
        self.cancel_rental(id).await
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::parse_instance(&self.fetch_rental_json(id).await?)
    }
    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        if let Some(id) = id.strip_prefix("lium-template:") {
            let response = self
                .http
                .get(&format!("{}/templates/{id}", self.base_url), &self.auth)
                .await?;
            return Ok(
                if crate::providers::common::gpu_adapter::api_json_or_missing(response)
                    .await?
                    .is_some()
                {
                    InstanceStatus::Running
                } else {
                    InstanceStatus::Terminated
                },
            );
        }
        match crate::providers::common::gpu_adapter::api_json_or_missing(
            self.http
                .get(&format!("{}/pods/{id}", self.base_url), &self.auth)
                .await?,
        )
        .await?
        {
            Some(raw) => Ok(Self::parse_instance(&raw)?.status),
            None => Ok(InstanceStatus::Terminated),
        }
    }
    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        let raw = self.fetch_rental_json(&instance.id).await?;
        crate::providers::common::gpu_adapter::ssh_endpoint(
            raw["ssh_connect_cmd"]
                .as_str()
                .ok_or_else(|| configuration("Lium SSH endpoint is not ready"))?,
        )
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
        let raw = self.fetch_rental_json(&instance.id).await?;
        if Self::parse_instance(&raw)?.status != InstanceStatus::Running {
            return Ok(None);
        }
        Ok(lium_endpoint(&raw, port))
    }
    async fn list_managed(
        &self,
        namespace: &str,
    ) -> Result<Vec<crate::infra::types::ManagedResource>> {
        use crate::infra::types::{ManagedResource, ManagedResourceKind, TlsMode};
        use crate::providers::common::gpu_adapter::{
            managed_lease_id, managed_resource_prefix, provider_created_at, provider_hourly_price,
        };
        managed_resource_prefix(namespace)?;
        let pods = self.pods().await?;
        let used: std::collections::HashSet<_> = pods
            .iter()
            .filter_map(|pod| pod["template"]["id"].as_str())
            .collect();
        let mut resources = Vec::new();
        for raw in &pods {
            let Some(lease_id) = raw["pod_name"]
                .as_str()
                .and_then(|name| managed_lease_id(name, namespace))
            else {
                continue;
            };
            let environment = &raw["template"]["environment"];
            let mut spec = ResourceSpec::default();
            spec.env = serde_json::from_value(environment.clone()).unwrap_or_default();
            let tls =
                crate::providers::common::gpu_adapter::worker_tls_enabled(&spec).unwrap_or(false);
            let port = environment["TANGLE_GPU_LEASE_ENDPOINT_PORT"]
                .as_str()
                .and_then(|p| p.parse().ok());
            resources.push(ManagedResource {
                instance: Self::parse_instance(raw)?,
                namespace: namespace.into(),
                lease_id,
                kind: ManagedResourceKind::Instance,
                endpoint: port
                    .filter(|_| {
                        tls && Self::parse_instance(raw)
                            .is_ok_and(|i| i.status == InstanceStatus::Running)
                    })
                    .and_then(|port| lium_endpoint(raw, port)),
                tls_mode: tls.then_some(TlsMode::Pinned),
                request_fingerprint: environment["TANGLE_GPU_REQUEST_FINGERPRINT"]
                    .as_str()
                    .map(String::from),
                expires_at: None,
                expiry_source: None,
                hourly_cost_usd: provider_hourly_price(&raw["price"]),
                created_at: provider_created_at(&raw["created_at"]),
            });
        }
        let templates = api_json(
            self.http
                .get(&format!("{}/templates", self.base_url), &self.auth)
                .await?,
        )
        .await?;
        for template in templates
            .as_array()
            .ok_or_else(|| configuration("Lium template list is invalid"))?
        {
            let Some(lease_id) = template["name"]
                .as_str()
                .and_then(|name| managed_lease_id(name, namespace))
            else {
                continue;
            };
            let Some(id) = template["id"].as_str().filter(|id| !used.contains(id)) else {
                continue;
            };
            resources.push(ManagedResource {
                instance: ProvisionedInstance {
                    id: format!("lium-template:{id}"),
                    provider: CloudProvider::BittensorLium,
                    instance_type: "template".into(),
                    region: String::new(),
                    public_ip: None,
                    private_ip: None,
                    status: InstanceStatus::Running,
                },
                namespace: namespace.into(),
                lease_id,
                kind: ManagedResourceKind::Template,
                endpoint: None,
                tls_mode: None,
                request_fingerprint: template["environment"]["TANGLE_GPU_REQUEST_FINGERPRINT"]
                    .as_str()
                    .map(String::from),
                expires_at: None,
                expiry_source: None,
                hourly_cost_usd: None,
                created_at: None,
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
                let plan = super::BittensorLiumInstanceMapper::map(resource_spec);
                provision_with_cleanup(
                    "bittensor-lium",
                    || self.provision_resources(&plan.instance_type, "", resource_spec, false),
                    |instance| {
                        let env_vars = env_vars.clone();
                        async move {
                            self.deploy_blueprint(
                                &instance,
                                blueprint_image,
                                resource_spec,
                                env_vars,
                            )
                            .await
                        }
                    },
                    |id| async move { self.cancel_rental(&id).await },
                )
                .await
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "Bittensor Lium does not offer managed Kubernetes".into(),
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
                "Bittensor Lium does not offer serverless deployment".into(),
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
            SshDeploymentConfig::bittensor_lium(),
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
