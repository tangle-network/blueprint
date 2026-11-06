//! Runpod REST v2: live inventory and one identity-scoped pod lifecycle.
use crate::core::deployment_target::DeploymentTarget;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode,
};
use crate::providers::common::gpu_adapter::{deploy_via_ssh, require_public_ip};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use blueprint_std::collections::HashMap;
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};

const BASE_URL: &str = "https://api.runpod.io/v2";

pub struct RunPodAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    health_http: reqwest::Client,
    base_url: String,
    cloud_type: String,
    default_region: Option<String>,
}

impl RunPodAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("RUNPOD_API_KEY")
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderNotConfigured))?;
        let mut adapter = Self::with_credentials(key)?;
        if let Ok(cloud) = std::env::var("RUNPOD_CLOUD_TYPE") {
            adapter.cloud_type = cloud;
        }
        adapter.default_region = std::env::var("RUNPOD_REGION").ok();
        Ok(adapter)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Self::with_endpoint(api_key, BASE_URL)
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        Ok(Self {
            http: SecureHttpClient::with_endpoint(endpoint)?,
            auth: ApiAuthentication::runpod(api_key),
            health_http: crate::create_default_provider_client()?,
            base_url: endpoint.trim_end_matches('/').into(),
            cloud_type: "SECURE".into(),
            default_region: None,
        })
    }

    async fn pod(&self, id: &str) -> Result<Option<Value>> {
        let response = self
            .http
            .get(
                &format!("{}/pods/{}", self.base_url, urlencoding::encode(id)),
                &self.auth,
            )
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

    fn instance(raw: &Value) -> Result<ProvisionedInstance> {
        Ok(ProvisionedInstance {
            id: raw["id"]
                .as_str()
                .filter(|id| !id.is_empty())
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
                .into(),
            provider: CloudProvider::RunPod,
            instance_type: raw["gpu"]["id"].as_str().unwrap_or("").into(),
            region: raw["dataCenterId"].as_str().unwrap_or("").into(),
            public_ip: raw["ssh"]["direct"]["host"].as_str().map(str::to_owned),
            private_ip: None,
            status: match raw["status"].as_str() {
                Some("RUNNING") => InstanceStatus::Running,
                Some("PROVISIONING" | "STARTING") => InstanceStatus::Starting,
                Some("EXITED") => InstanceStatus::Stopped,
                Some("TERMINATED") => InstanceStatus::Terminated,
                _ => InstanceStatus::Unknown,
            },
        })
    }

    fn connection(scheme: &str, host: &str, port: u16, username: Option<&str>) -> Option<String> {
        if port == 0 {
            return None;
        }
        let mut url = url::Url::parse(&format!("{scheme}://localhost")).ok()?;
        url.set_host(Some(host)).ok()?;
        url.set_port(Some(port)).ok()?;
        if let Some(username) = username {
            url.set_username(username).ok()?;
        }
        Some(url.into())
    }

    fn managed(raw: &Value) -> Result<Option<ManagedResource>> {
        let env: BTreeMap<String, String> =
            serde_json::from_value(raw.get("env").cloned().unwrap_or_else(|| json!({})))
                .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        let Some(metadata) = ManagedMetadata::from_env(&env)? else {
            return Ok(None);
        };
        let instance = Self::instance(raw)?;
        let count = raw["gpu"]["count"].as_u64().unwrap_or(1);
        let count = u32::try_from(count).ok().filter(|count| *count > 0);
        let class = blueprint_core::gpu::catalog().into_iter().find(|class| {
            count.is_some_and(|count| {
                class.sku("runpod", count) == Some(instance.instance_type.as_str())
            })
        });
        let ssh = raw["ssh"]
            .get("direct")
            .filter(|value| value.is_object())
            .or_else(|| raw["ssh"].get("proxy"));
        let ssh_endpoint = ssh.and_then(|ssh| {
            Self::connection(
                "ssh",
                ssh["host"].as_str()?,
                u16::try_from(ssh["port"].as_u64()?).ok()?,
                ssh["username"].as_str(),
            )
        });
        let endpoint_url = metadata.endpoint_port.and_then(|port| {
            let expected = format!("{port}/http");
            let configured = raw["ports"]
                .as_array()?
                .iter()
                .any(|value| value.as_str() == Some(expected.as_str()));
            let id = instance.id.as_str();
            if !configured
                || !id
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            {
                return None;
            }
            Some(format!("https://{id}-{port}.proxy.runpod.net"))
        });
        let mut resource = metadata.bind(instance, ManagedResourceKind::Instance);
        resource.endpoint_url = endpoint_url;
        resource.ssh_endpoint = ssh_endpoint;
        resource.gpu_class = class.as_ref().map(|class| class.id.clone());
        resource.gpu_count = count;
        resource.gpu_vram_gb = class.map(|class| class.vram_gb);
        resource.hourly_cost_usd = raw["cost"]
            .as_f64()
            .filter(|price| price.is_finite() && *price > 0.0);
        resource.created_at = raw["createdAt"]
            .as_str()
            .and_then(|value| value.parse().ok());
        Ok(Some(resource))
    }
}

#[async_trait]
impl ManagedProviderAdapter for RunPodAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.tee_required
            || spec.source_network.is_some()
            || !matches!(self.cloud_type.as_str(), "SECURE" | "COMMUNITY")
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        let mut url = url::Url::parse(&format!("{}/catalog/gpus", self.base_url))
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        url.query_pairs_mut().extend_pairs([
            ("include", "AVAILABILITY"),
            ("product", "POD"),
            ("count", &count.to_string()),
            ("cloud", &self.cloud_type),
        ]);
        let response: Value = self
            .http
            .get(url.as_str(), &self.auth)
            .await?
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        let gpus = response["gpus"]
            .as_array()
            .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
        let region = if region.is_empty() {
            self.default_region.as_deref().unwrap_or("")
        } else {
            region
        };
        let cloud = self.cloud_type.to_ascii_lowercase();
        let classes = blueprint_core::gpu::catalog();
        let mut offers = Vec::new();
        for gpu in gpus {
            let Some(sku) = gpu["id"].as_str() else {
                continue;
            };
            let Some(class) = classes
                .iter()
                .find(|class| class.sku("runpod", count) == Some(sku))
            else {
                continue;
            };
            if !matches!(
                gpu["availability"].as_str(),
                Some("LOW" | "MEDIUM" | "HIGH")
            ) || spec.gpu_class.as_ref().is_some_and(|id| id != &class.id)
                || spec
                    .gpu_vram_gb
                    .is_some_and(|minimum| class.vram_gb < minimum)
                || gpu["maxCount"][&cloud].as_u64().unwrap_or(0) < u64::from(count)
            {
                continue;
            }
            let Some(price) = gpu["price"][&cloud]
                .as_f64()
                .map(|price| price * f64::from(count))
                .filter(|price| price.is_finite() && *price > 0.0)
            else {
                continue;
            };
            let Some(centers) = gpu["dataCenters"].as_array() else {
                continue;
            };
            for center in centers {
                let Some(id) = center["id"].as_str().filter(|id| !id.is_empty()) else {
                    continue;
                };
                if (!region.is_empty() && id != region)
                    || !matches!(
                        center["availability"].as_str(),
                        Some("LOW" | "MEDIUM" | "HIGH")
                    )
                {
                    continue;
                }
                offers.push(NativeOffer {
                    provider: CloudProvider::RunPod,
                    native_id: id.into(),
                    instance_type: sku.into(),
                    region: id.into(),
                    gpu_class: class.id.clone(),
                    gpu_count: count,
                    hourly_cost_usd: price,
                    spot: false,
                    pricing_source: url.to_string(),
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
        fingerprint: &str,
    ) -> Result<ManagedResource> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.tee_required
            || spec.source_network.is_some()
            || offer.spot
            || (spec.volume_id.is_some() && spec.volume_gb.is_some())
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let metadata = ManagedMetadata::for_spec(spec, fingerprint, chrono::Utc::now())?;
        let image = spec
            .image
            .as_ref()
            .filter(|image| !image.is_empty())
            .ok_or(Error::Provider(ProviderErrorCode::Unsupported))?;
        let mut env = spec.env.clone();
        env.extend(metadata.env());
        if let Some(key) = &spec.ssh_public_key {
            env.insert("PUBLIC_KEY".into(), key.clone());
        }
        let mut ports = spec.ports.clone();
        if let Some(port) = spec.endpoint_port {
            ports.push(port);
        }
        if spec.ssh_public_key.is_some() {
            ports.push(22);
        }
        ports.sort_unstable();
        ports.dedup();
        let mut body = json!({
            "name": metadata.name(), "image": image, "cloud": self.cloud_type,
            "gpu": {"id":offer.instance_type, "count":offer.gpu_count,
                "minRamPerGpu":(f64::from(spec.memory_gb)/f64::from(offer.gpu_count)).ceil().max(1.0) as u64,
                "minVcpuCountPerGpu":(f64::from(spec.cpu)/f64::from(offer.gpu_count)).ceil().max(1.0) as u64},
            "dataCenterIds":[offer.region], "disk":spec.storage_gb.ceil() as u64,
            "ports":ports.iter().map(|port|format!("{port}/{}", if spec.endpoint_port==Some(*port) {"http"}else{"tcp"})).collect::<Vec<_>>(),
            "env":env, "startSsh":spec.ssh_public_key.is_some(),
        });
        if let Some(command) = &spec.command {
            body["cmd"] = json!(command);
        }
        if let Some(volume) = spec.volume_gb {
            if volume < 10.0 {
                return Err(Error::Provider(ProviderErrorCode::Unsupported));
            }
            body["mounts"] = json!({"persistent":{"size":volume.ceil() as u64,"path":spec.volume_mount_path.as_deref().unwrap_or("/workspace")}});
        } else if let Some(id) = &spec.volume_id {
            body["mounts"] = json!({"network":[{"volumeId":id,"path":spec.volume_mount_path.as_deref().unwrap_or("/workspace")}]});
        }
        // Create is sent once. An uncertain response is recovered by native metadata/list.
        let response = self
            .http
            .post_json(&format!("{}/pods", self.base_url), &self.auth, body)
            .await?;
        let raw: Value = response
            .json()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        if raw["gpu"]["id"].as_str() != Some(offer.instance_type.as_str())
            || raw["gpu"]["count"].as_u64().unwrap_or(1) != u64::from(offer.gpu_count)
        {
            return Err(Error::Provider(ProviderErrorCode::Indeterminate));
        }
        Self::managed(&raw)?.ok_or(Error::Provider(ProviderErrorCode::Indeterminate))
    }

    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        let Some(raw) = self.pod(id).await? else {
            return Ok(None);
        };
        let resource = Self::managed(&raw)?.ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }

    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let mut resources = Vec::new();
        let mut cursor = None::<String>;
        let mut seen = HashSet::new();
        loop {
            let mut url = url::Url::parse(&format!("{}/pods", self.base_url))
                .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
            if let Some(cursor) = &cursor {
                url.query_pairs_mut().append_pair("cursor", cursor);
            }
            let page: Value = self
                .http
                .get(url.as_str(), &self.auth)
                .await?
                .json()
                .await
                .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
            for raw in page["pods"]
                .as_array()
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
            {
                if let Some(resource) = Self::managed(raw)? {
                    if resource.namespace == namespace {
                        resources.push(resource);
                    }
                }
            }
            match page["pagination"]["hasNextPage"].as_bool() {
                Some(false) => return Ok(resources),
                Some(true) => {
                    let next = page["pagination"]["nextCursor"]
                        .as_str()
                        .filter(|value| !value.is_empty())
                        .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
                    if !seen.insert(next.to_owned()) {
                        return Err(Error::Provider(ProviderErrorCode::ProviderError));
                    }
                    cursor = Some(next.into());
                }
                None => return Err(Error::Provider(ProviderErrorCode::ProviderError)),
            }
        }
    }

    async fn destroy(&self, identity: &ManagedLeaseIdentity, id: &str) -> Result<()> {
        if <Self as ManagedProviderAdapter>::status(self, identity, id)
            .await?
            .is_none()
        {
            return Ok(());
        }
        let response = self
            .http
            .delete(
                &format!("{}/pods/{}", self.base_url, urlencoding::encode(id)),
                &self.auth,
            )
            .await?;
        if response.status().is_success() || response.status() == reqwest::StatusCode::NOT_FOUND {
            Ok(())
        } else {
            Err(Error::Provider(ProviderErrorCode::ProviderError))
        }
    }
}

#[async_trait]
impl CloudProviderAdapter for RunPodAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }

    async fn provision_instance(
        &self,
        _instance_type: &str,
        _region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        Err(Error::Provider(ProviderErrorCode::Unsupported))
    }

    async fn terminate_instance(&self, _id: &str) -> Result<()> {
        Err(Error::Provider(ProviderErrorCode::Unsupported))
    }

    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        self.get_instance_details(id)
            .await
            .map(|instance| instance.status)
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::instance(
            &self
                .pod(id)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::NotFound))?,
        )
    }

    async fn deploy_blueprint_with_target(
        &self,
        target: &DeploymentTarget,
        image: &str,
        spec: &ResourceSpec,
        env: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        match target {
            DeploymentTarget::GenericKubernetes { namespace, .. } => {
                #[cfg(feature = "kubernetes")]
                {
                    crate::shared::SharedKubernetesDeployment::deploy_to_generic_k8s(
                        namespace, image, spec, env,
                    )
                    .await
                }
                #[cfg(not(feature = "kubernetes"))]
                {
                    let _ = (namespace, image, spec, env);
                    Err(Error::Provider(ProviderErrorCode::Unsupported))
                }
            }
            _ => Err(Error::Provider(ProviderErrorCode::Unsupported)),
        }
    }

    async fn deploy_blueprint(
        &self,
        instance: &ProvisionedInstance,
        image: &str,
        spec: &ResourceSpec,
        env: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        let identity = spec
            .ownership
            .as_ref()
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        <Self as ManagedProviderAdapter>::status(self, identity, &instance.id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))?;
        require_public_ip(instance)?;
        let raw = self
            .pod(&instance.id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))?;
        let metadata = ManagedMetadata::from_env(
            &serde_json::from_value(raw["env"].clone())
                .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?,
        )?
        .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        let mut result =
            deploy_via_ssh(instance, image, spec, env, SshDeploymentConfig::runpod()).await?;
        result.metadata.extend(metadata.tags());
        Ok(result)
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        let tags = deployment
            .metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let metadata = ManagedMetadata::from_tags(&tags)?
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if !<Self as ManagedProviderAdapter>::status(
            self,
            &metadata.identity,
            &deployment.instance.id,
        )
        .await?
        .is_some_and(|resource| resource.instance.status == InstanceStatus::Running)
        {
            return Ok(false);
        }
        let Some(endpoint) = deployment.qos_grpc_endpoint() else {
            return Ok(false);
        };
        Ok(self
            .health_http
            .get(format!("{endpoint}/health"))
            .send()
            .await
            .is_ok_and(|response| response.status().is_success()))
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        let tags = deployment
            .metadata
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let metadata = ManagedMetadata::from_tags(&tags)?
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        <Self as ManagedProviderAdapter>::destroy(self, &metadata.identity, &deployment.instance.id)
            .await
    }
}
