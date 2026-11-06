//! RunPod `CloudProviderAdapter` implementation.
//!
//! Uses the REST API (`https://rest.runpod.io/v1/pods`) to manage pods.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    build_http_client, deploy_via_ssh, generate_instance_name, require_public_ip,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://rest.runpod.io/v1";

pub struct RunPodAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    pricing_url: String,
    cloud_type: String,
    default_region: String,
}

impl RunPodAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("RUNPOD_API_KEY")
            .map_err(|_| Error::ConfigurationError("RUNPOD_API_KEY is required".into()))?;
        let mut adapter = Self::with_credentials(key)?;
        adapter.cloud_type = std::env::var("RUNPOD_CLOUD_TYPE").unwrap_or_else(|_| "SECURE".into());
        adapter.default_region = std::env::var("RUNPOD_REGION").unwrap_or_default();
        Ok(adapter)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::runpod(api_key),
            base_url: BASE_URL.into(),
            pricing_url: "https://api.runpod.io/graphql".into(),
            cloud_type: "SECURE".into(),
            default_region: String::new(),
        })
    }

    /// Use a caller-owned provider gateway; HTTP is restricted to loopback peers.
    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        adapter.pricing_url = format!("{}/graphql", adapter.base_url);
        Ok(adapter)
    }

    async fn list_pods(&self) -> Result<Vec<serde_json::Value>> {
        let value = crate::providers::common::gpu_adapter::api_json(
            self.http
                .get(&format!("{}/pods", self.base_url), &self.auth)
                .await?,
        )
        .await?;
        value
            .as_array()
            .cloned()
            .ok_or_else(|| Error::HttpError("RunPod instance list is invalid".into()))
    }

    async fn fetch_pod(&self, pod_id: &str) -> Result<serde_json::Value> {
        crate::providers::common::gpu_adapter::api_json(
            self.http
                .get(&format!("{}/pods/{pod_id}", self.base_url), &self.auth)
                .await?,
        )
        .await
    }

    fn parse_pod(value: &serde_json::Value, fallback_region: &str) -> Result<ProvisionedInstance> {
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("RunPod response missing id".into()))?
            .to_string();
        let instance_type = value
            .get("machine")
            .and_then(|m| m.get("gpuTypeId"))
            .or_else(|| value.get("gpuTypeId"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = value
            .get("machine")
            .and_then(|m| m.get("dataCenterId"))
            .or_else(|| value.get("dataCenterId"))
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = value
            .get("publicIp")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = value
            .get("privateIp")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match value
            .get("desiredStatus")
            .or_else(|| value.get("status"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
        {
            "RUNNING" => InstanceStatus::Running,
            "CREATED" | "PROVISIONING" | "STARTING" => InstanceStatus::Starting,
            "PAUSED" => InstanceStatus::Stopped,
            "EXITED" | "TERMINATED" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::RunPod,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }
}

fn runpod_application(
    raw: &serde_json::Value,
    requested: Option<u16>,
) -> (Option<String>, Option<crate::infra::types::TlsMode>) {
    use crate::infra::types::TlsMode;
    let mut spec = ResourceSpec::default();
    spec.env = serde_json::from_value(raw["env"].clone()).unwrap_or_default();
    let ports: Vec<_> = raw["ports"]
        .as_array()
        .into_iter()
        .flatten()
        .chain(raw["runtime"]["ports"].as_array().into_iter().flatten())
        .collect();
    let mut http_ports: Vec<u16> = ports
        .iter()
        .filter_map(|item| item.as_str()?.strip_suffix("/http")?.parse().ok())
        .collect();
    http_ports.sort_unstable();
    http_ports.dedup();
    let port = requested
        .or_else(|| {
            spec.env
                .get("TANGLE_GPU_LEASE_ENDPOINT_PORT")
                .and_then(|p| p.parse().ok())
        })
        .or_else(|| {
            if http_ports.len() == 1 {
                http_ports.first().copied()
            } else {
                None
            }
        });
    let Some(port) = port.filter(|p| *p > 0) else {
        return (None, None);
    };
    let tls = match crate::providers::common::gpu_adapter::worker_tls_enabled(&spec) {
        Ok(tls) => tls,
        Err(_) => return (None, None),
    };
    if tls {
        let external = raw["portMappings"][port.to_string()]
            .as_u64()
            .or_else(|| raw["portMappings"][port.to_string()].as_str()?.parse().ok())
            .or_else(|| {
                ports
                    .iter()
                    .find(|item| {
                        item["privatePort"].as_u64() == Some(u64::from(port))
                            && item["isIpPublic"].as_bool() == Some(true)
                    })
                    .and_then(|item| item["publicPort"].as_u64())
            });
        let Some(external) = external
            .and_then(|p| u16::try_from(p).ok())
            .filter(|p| *p > 0)
        else {
            return (None, None);
        };
        let Some(host) = raw["publicIp"].as_str() else {
            return (None, None);
        };
        let host = if host.parse::<std::net::Ipv6Addr>().is_ok() {
            format!("[{host}]")
        } else {
            host.to_string()
        };
        let Ok(url) = url::Url::parse(&format!("https://{host}:{external}")) else {
            return (None, None);
        };
        return if url.username().is_empty() && url.password().is_none() {
            (
                Some(url.to_string().trim_end_matches('/').into()),
                Some(TlsMode::Pinned),
            )
        } else {
            (None, None)
        };
    }
    let proxy = http_ports.contains(&port)
        || ports.iter().any(|item| {
            item["privatePort"].as_u64() == Some(u64::from(port))
                && item["type"]
                    .as_str()
                    .is_some_and(|kind| kind.eq_ignore_ascii_case("http"))
        });
    let Some(id) = raw["id"]
        .as_str()
        .filter(|id| !id.is_empty() && id.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-'))
    else {
        return (None, None);
    };
    if proxy {
        (
            Some(format!("https://{id}-{port}.proxy.runpod.net")),
            Some(TlsMode::Public),
        )
    } else {
        (None, None)
    }
}

#[async_trait]
impl CloudProviderAdapter for RunPodAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        use crate::providers::common::gpu_adapter::{
            api_json, catalog_class_for_sku, catalog_skus, quoted_gpu_selection,
        };
        if spec.source_network.is_some() {
            return Err(Error::ConfigurationError(
                "RunPod cannot enforce caller source-network restrictions".into(),
            ));
        }
        let skus = catalog_skus(spec, "runpod")?;
        let filter = if skus.len() == 1 {
            serde_json::json!({"id":skus[0]})
        } else {
            serde_json::json!({"ids":skus})
        };
        let count = spec.gpu_count.unwrap_or(1);
        let mut input = serde_json::json!({
            "gpuCount": count, "secureCloud": self.cloud_type == "SECURE",
            "minMemoryInGb": spec.memory_gb.ceil() as u32,
            "minVcpuCount": spec.cpu.ceil() as u32, "supportPublicIp": true,
        });
        if spec.image.is_none() {
            input["minCudaVersion"] = serde_json::json!("12.8");
        }
        let region = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };
        if region.len() == 2 && region.bytes().all(|c| c.is_ascii_uppercase()) {
            input["countryCode"] = serde_json::json!(region);
        } else if !region.is_empty() {
            input["dataCenterIds"] = serde_json::json!([region]);
        }
        let json = api_json(self.http.post_json(&self.pricing_url, &self.auth, serde_json::json!({
            "query": "query($filter:GpuTypeFilter,$input:GpuLowestPriceInput){gpuTypes(input:$filter){id lowestPrice(input:$input){uninterruptablePrice stockStatus}}}",
            "variables": {"filter":filter,"input":input}
        })).await?).await?;
        let rows = json["data"]["gpuTypes"]
            .as_array()
            .ok_or_else(|| Error::HttpError("RunPod GPU list missing".into()))?;
        let (sku, price) = rows
            .iter()
            .filter_map(|row| {
                let sku = row["id"].as_str()?;
                if !skus.iter().any(|candidate| candidate == sku) {
                    return None;
                }
                let lowest = &row["lowestPrice"];
                if !matches!(
                    lowest["stockStatus"].as_str(),
                    Some("High" | "Medium" | "Low" | "AVAILABLE")
                ) {
                    return None;
                }
                let price = lowest["uninterruptablePrice"].as_f64()? * f64::from(count);
                (price.is_finite() && price > 0.0).then_some((sku.to_string(), price))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .ok_or_else(|| Error::ConfigurationError("RunPod has no priced in-stock GPU".into()))?;
        let class = catalog_class_for_sku(spec, "runpod", &sku)?;
        quoted_gpu_selection(sku, price, class)
    }

    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let instance = self
            .provision_resources(
                instance_type,
                region,
                &ResourceSpec::default().with_gpu(1),
                require_tee,
            )
            .await?;
        self.prepare_access(&instance, &ResourceSpec::default())
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
        if !crate::providers::common::gpu_adapter::catalog_skus(spec, "runpod")?
            .iter()
            .any(|sku| sku == instance_type)
        {
            return Err(Error::ConfigurationError(
                "Provider SKU disagrees with requested GPU class/count".into(),
            ));
        }
        spec.validate().map_err(Error::ConfigurationError)?;
        // Fail closed at the trait boundary: this provider has no
        // confidential-compute / TEE attestation path. Refuse rather than
        // silently return a non-TEE instance for a require_tee request.
        if require_tee {
            return Err(Error::ConfigurationError(
                "RunPod does not support confidential-compute / TEE attestation; \
                 refusing require_tee"
                    .into(),
            ));
        }
        if spec.source_network.is_some() {
            return Err(Error::ConfigurationError(
                "RunPod cannot enforce caller source-network restrictions".into(),
            ));
        }
        let tls = crate::providers::common::gpu_adapter::worker_tls_enabled(spec)?;
        let name = if let Some(ownership) = &spec.ownership {
            let name = crate::providers::common::gpu_adapter::managed_resource_name(
                &ownership.namespace,
                &ownership.lease_id,
            )?;
            let existing: Vec<_> = self
                .list_pods()
                .await?
                .into_iter()
                .filter(|p| p["name"].as_str() == Some(name.as_str()))
                .collect();
            if existing.len() > 1 {
                return Err(Error::ConfigurationError(
                    "Multiple RunPod resources claim this lease identity".into(),
                ));
            }
            if let Some(existing) = existing.first() {
                let id = existing["id"]
                    .as_str()
                    .ok_or_else(|| Error::HttpError("Managed RunPod resource has no id".into()))?;
                let existing = self.fetch_pod(id).await?;
                let actual = Self::parse_pod(&existing, &self.default_region)?;
                if existing["env"]["TANGLE_GPU_REQUEST_FINGERPRINT"].as_str()
                    != Some(crate::managed::request_fingerprint(spec)?.as_str())
                {
                    return Err(Error::ConfigurationError(
                        "Managed RunPod resource conflicts with requested allocation".into(),
                    ));
                }
                if actual.instance_type != instance_type
                    || existing["gpuCount"].as_u64() != spec.gpu_count.map(u64::from)
                    || existing["imageName"].as_str().is_some_and(|image| {
                        image
                            != spec
                                .image
                                .as_deref()
                                .unwrap_or(crate::providers::common::gpu_adapter::DEFAULT_GPU_IMAGE)
                    })
                {
                    return Err(Error::ConfigurationError(
                        "Managed RunPod lease conflicts with requested hardware/image".into(),
                    ));
                }
                return Ok(actual);
            }
            name
        } else {
            generate_instance_name("runpod")
        };
        let mut environment = crate::providers::common::gpu_adapter::workload_environment(
            spec,
            spec.endpoint_port
                .map(|_| if tls { "pinned" } else { "public" }),
        )?;
        if let Some(key) = &spec.ssh_public_key {
            environment.insert("PUBLIC_KEY".into(), key.clone());
        }
        let mut ports = crate::providers::common::gpu_adapter::workload_ports(
            spec,
            spec.ssh_public_key.is_some(),
        );
        if ports.is_empty() {
            ports.extend([22, 9615, 9944]);
        }
        let exposed: Vec<_> = ports
            .into_iter()
            .map(|port| {
                format!(
                    "{port}/{}",
                    if Some(port) == spec.endpoint_port && !tls {
                        "http"
                    } else {
                        "tcp"
                    }
                )
            })
            .collect();
        let mut payload = serde_json::json!({
            "name": name,
            "gpuTypeIds": [instance_type],
            "cloudType": self.cloud_type,
            "gpuCount": spec.gpu_count.unwrap_or(1),
            "containerDiskInGb": spec.storage_gb.ceil() as u32,
            "volumeInGb": 0,
            "imageName": spec.image.as_deref().unwrap_or(crate::providers::common::gpu_adapter::DEFAULT_GPU_IMAGE),
            "env": environment,
            "ports": exposed,
            "supportPublicIp": true,
        });

        let location = if region.is_empty() {
            &self.default_region
        } else {
            region
        };
        if location.len() == 2 && location.bytes().all(|c| c.is_ascii_uppercase()) {
            payload["countryCodes"] = serde_json::json!([location]);
        } else if !location.is_empty() {
            payload["dataCenterIds"] = serde_json::json!([location]);
        }
        if let Some(command) = &spec.command {
            if command.is_empty() {
                return Err(Error::ConfigurationError(
                    "Workload command cannot be empty".into(),
                ));
            }
            payload["dockerStartCmd"] = serde_json::json!(command);
        }
        let create_url = format!("{}/pods", self.base_url);
        let json = crate::providers::common::gpu_adapter::api_json(
            self.http
                .post_json(&create_url, &self.auth, payload)
                .await?,
        )
        .await?;
        let pod_id = json
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("RunPod create pod: no id in response".into()))?
            .to_string();

        info!(%pod_id, %instance_type, "Created RunPod pod");

        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::RunPod,
            pod_id,
            instance_type.to_string(),
            location.to_string(),
        ))
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::parse_pod(&self.fetch_pod(id).await?, &self.default_region)
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        let raw = self.fetch_pod(&instance.id).await?;
        let public_ip = raw["publicIp"]
            .as_str()
            .ok_or_else(|| Error::HttpError("RunPod has no public SSH IP".into()))?;
        let port = raw["portMappings"]["22"]
            .as_u64()
            .or_else(|| {
                raw["runtime"]["ports"]
                    .as_array()
                    .and_then(|ports| {
                        ports.iter().find(|p| {
                            p["privatePort"].as_u64() == Some(22)
                                && p["isIpPublic"].as_bool() == Some(true)
                        })
                    })
                    .and_then(|p| p["publicPort"].as_u64())
            })
            .ok_or_else(|| Error::HttpError("RunPod has no public SSH port mapping".into()))?;
        Ok(format!("ssh://root@{public_ip}:{port}"))
    }

    fn application_tls_mode(&self, spec: &ResourceSpec) -> Option<crate::infra::types::TlsMode> {
        spec.endpoint_port?;
        crate::providers::common::gpu_adapter::worker_tls_enabled(spec)
            .ok()
            .map(|tls| {
                if tls {
                    crate::infra::types::TlsMode::Pinned
                } else {
                    crate::infra::types::TlsMode::Public
                }
            })
    }
    async fn application_endpoint(
        &self,
        instance: &ProvisionedInstance,
        spec: &ResourceSpec,
    ) -> Result<Option<String>> {
        Ok(runpod_application(&self.fetch_pod(&instance.id).await?, spec.endpoint_port).0)
    }

    async fn list_managed(
        &self,
        namespace: &str,
    ) -> Result<Vec<crate::infra::types::ManagedResource>> {
        use crate::providers::common::gpu_adapter::{
            managed_lease_id, managed_resource_prefix, provider_created_at, provider_hourly_price,
        };
        managed_resource_prefix(namespace)?;
        let mut result = Vec::new();
        for raw in self.list_pods().await? {
            let Some(lease_id) = raw["name"]
                .as_str()
                .and_then(|name| managed_lease_id(name, namespace))
            else {
                continue;
            };
            let (endpoint, tls_mode) = runpod_application(&raw, None);
            result.push(crate::infra::types::ManagedResource {
                instance: Self::parse_pod(&raw, &self.default_region)?,
                namespace: namespace.into(),
                lease_id,
                kind: crate::infra::types::ManagedResourceKind::Instance,
                endpoint,
                tls_mode,
                request_fingerprint: raw["env"]["TANGLE_GPU_REQUEST_FINGERPRINT"]
                    .as_str()
                    .map(String::from),
                expires_at: None,
                expiry_source: None,
                hourly_cost_usd: provider_hourly_price(&raw["costPerHr"]),
                created_at: provider_created_at(&raw["lastStartedAt"]),
            });
        }
        Ok(result)
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("{}/pods/{instance_id}", self.base_url);
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self.http.delete(&url, &self.auth).await {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated RunPod pod");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "RunPod pod already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "RunPod delete pod: transient {}",
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
                    last_err = Some(Error::HttpError(format!("RunPod delete pod: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        let response = self
            .http
            .get(&format!("{}/pods/{id}", self.base_url), &self.auth)
            .await?;
        match crate::providers::common::gpu_adapter::api_json_or_missing(response).await? {
            Some(raw) => Ok(Self::parse_pod(&raw, &self.default_region)?.status),
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
                let selection = crate::providers::runpod::RunPodInstanceMapper::map(resource_spec);
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
                            provider = "runpod",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "runpod",
                                instance_id = %instance_id_for_cleanup,
                                cleanup_error = %cleanup_err,
                                "Cleanup after failed deploy also failed — instance may be orphaned"
                            );
                        }
                        Err(deploy_err)
                    }
                }
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "RunPod does not expose a managed Kubernetes product via this API".into(),
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
                "RunPod Serverless uses a distinct /v2/serverless API — not supported here".into(),
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
            SshDeploymentConfig::runpod(),
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
