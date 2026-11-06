//! Prime Intellect GPU rentals through its documented availability and pods APIs.
use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{
    InstanceSelection,
    gpu_adapter::{
        api_json, build_http_client, catalog_skus, deploy_via_ssh, generate_instance_name,
        provision_with_cleanup, quoted_selection, reject_tee, require_public_ip,
    },
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_std::collections::HashMap;
use serde_json::{Value, json};

const BASE_URL: &str = "https://api.primeintellect.ai/api/v1";

pub struct PrimeIntellectAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    default_region: String,
}

impl PrimeIntellectAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("PRIME_INTELLECT_API_KEY")
            .map_err(|_| Error::ConfigurationError("PRIME_INTELLECT_API_KEY is required".into()))?;
        let mut adapter = Self::with_credentials(key)?;
        adapter.default_region = std::env::var("PRIME_INTELLECT_REGION").unwrap_or_default();
        Ok(adapter)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::prime_intellect(api_key),
            base_url: BASE_URL.into(),
            default_region: String::new(),
        })
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        Ok(adapter)
    }

    async fn offer(&self, spec: &ResourceSpec, region: &str) -> Result<(Value, f64)> {
        let skus = catalog_skus(spec, "prime-intellect")?;
        let count = spec.gpu_count.unwrap_or(1);
        let mut best: Option<(Value, f64)> = None;
        let mut page = 1;
        loop {
            let mut url = url::Url::parse(&format!("{}/availability/gpus", self.base_url))
                .map_err(|e| Error::ConfigurationError(e.to_string()))?;
            url.query_pairs_mut()
                .append_pair("gpu_count", &count.to_string())
                .append_pair("page", &page.to_string());
            if skus.len() == 1 {
                url.query_pairs_mut().append_pair("gpu_type", &skus[0]);
            }
            if !region.is_empty() {
                url.query_pairs_mut().append_pair("regions", region);
            }
            let json = api_json(self.http.get(url.as_str(), &self.auth).await?).await?;
            let rows = json["items"]
                .as_array()
                .ok_or_else(|| Error::HttpError("Prime availability missing items".into()))?;
            for row in rows {
                let Some(sku) = row["gpuType"]
                    .as_str()
                    .filter(|sku| skus.iter().any(|candidate| candidate == sku))
                else {
                    continue;
                };
                if row["gpuCount"].as_u64() != Some(count.into())
                    || row["gpuMemory"].as_u64().unwrap_or(0) < spec.gpu_vram_gb.unwrap_or(0).into()
                    || row["stockStatus"] == "Unavailable"
                    || row["prices"]["currency"] != "USD"
                {
                    continue;
                }
                let mut price = row["prices"]["onDemand"].as_f64().unwrap_or(f64::NAN);
                let mut pod = json!({"name":generate_instance_name("prime"),"cloudId":row["cloudId"],
                    "gpuType":sku,"gpuCount":count,"socket":row["socket"],
                    "image":"ubuntu_22_cuda_12","security":row["security"],
                    "dataCenterId":row["dataCenter"],"country":row["country"]});
                let mut valid = true;
                for (field, request, value) in [
                    ("vcpu", "vcpus", spec.cpu.ceil()),
                    ("memory", "memory", spec.memory_gb.ceil()),
                    ("disk", "diskSize", spec.storage_gb.ceil()),
                ] {
                    let resource = &row[field];
                    let default = resource["defaultCount"].as_f64().unwrap_or(0.0);
                    let desired = f64::from(value).max(default);
                    let max = resource["maxCount"].as_f64().unwrap_or(default);
                    if desired > max {
                        valid = false;
                        break;
                    }
                    let included = resource["defaultIncludedInPrice"].as_bool().unwrap_or(true);
                    let units = if included { desired - default } else { desired };
                    if units > 0.0 {
                        let Some(per_unit) = resource["pricePerUnit"].as_f64() else {
                            valid = false;
                            break;
                        };
                        price += units * per_unit;
                    }
                    pod[request] = json!(desired as u64);
                }
                if !valid || !price.is_finite() || price <= 0.0 {
                    continue;
                }
                pod["maxPrice"] = json!(price);
                let payload = json!({"pod":pod,"provider":{"type":row["provider"]}});
                if best.as_ref().is_none_or(|(_, current)| price < *current) {
                    best = Some((payload, price));
                }
            }
            if rows.is_empty()
                || page * 100 >= json["totalCount"].as_u64().unwrap_or(rows.len() as u64)
            {
                break;
            }
            page += 1;
        }
        best.ok_or_else(|| {
            Error::ConfigurationError("Prime has no priced in-stock offer for the request".into())
        })
    }

    async fn fetch_instance_json(&self, id: &str) -> Result<Value> {
        api_json(
            self.http
                .get(&format!("{}/pods/{id}", self.base_url), &self.auth)
                .await?,
        )
        .await
    }

    fn parse_instance(value: &Value, region: &str) -> Result<ProvisionedInstance> {
        let data = value.get("data").unwrap_or(value);
        let id = data["id"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Prime pod has no id".into()))?;
        Ok(ProvisionedInstance {
            id: id.into(),
            provider: CloudProvider::PrimeIntellect,
            instance_type: data["gpuName"].as_str().unwrap_or_default().into(),
            region: region.into(),
            public_ip: data["ip"].as_str().map(String::from),
            private_ip: None,
            status: match data["status"].as_str().unwrap_or_default() {
                "ACTIVE" => InstanceStatus::Running,
                "TERMINATED" | "ERROR" => InstanceStatus::Terminated,
                "STOPPED" => InstanceStatus::Stopped,
                _ => InstanceStatus::Starting,
            },
        })
    }

    async fn delete_instance(&self, id: &str) -> Result<()> {
        let response = self
            .http
            .delete(&format!("{}/pods/{id}", self.base_url), &self.auth)
            .await?;
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(Error::HttpError(format!(
                "Prime terminate HTTP {}",
                response.status()
            )))
        }
    }
}

#[async_trait]
impl CloudProviderAdapter for PrimeIntellectAdapter {
    async fn quote(&self, spec: &ResourceSpec, region: &str) -> Result<InstanceSelection> {
        let (payload, price) = self.offer(spec, region).await?;
        quoted_selection(payload.to_string(), price)
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
        self.prepare_access(&instance, &ResourceSpec::default())
            .await?;
        crate::providers::common::gpu_adapter::wait_for_instance(self, &instance.id).await
    }

    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        reject_tee(tee)?;
        spec.validate().map_err(Error::ConfigurationError)?;
        let skus = catalog_skus(spec, "prime-intellect")?;
        let mut payload = match serde_json::from_str::<Value>(instance_type) {
            Ok(payload)
                if payload["pod"]["gpuType"]
                    .as_str()
                    .is_some_and(|sku| skus.iter().any(|candidate| candidate == sku))
                    && payload["pod"]["gpuCount"].as_u64()
                        == Some(spec.gpu_count.unwrap_or(1).into()) =>
            {
                payload
            }
            _ => self.offer(spec, region).await?.0,
        };
        if let Some(public_key) = &spec.ssh_public_key {
            let key = api_json(
                self.http
                    .post_json(
                        &format!("{}/ssh_keys/", self.base_url),
                        &self.auth,
                        json!({"name":generate_instance_name("lease-key"),"publicKey":public_key}),
                    )
                    .await?,
            )
            .await?;
            payload["pod"]["sshKeyId"] = key["id"]
                .as_str()
                .map(|id| json!(id))
                .ok_or_else(|| Error::HttpError("Prime SSH key registration missing id".into()))?;
        }
        let sku = payload["pod"]["gpuType"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Prime quote missing GPU type".into()))?
            .to_string();
        let created = api_json(
            self.http
                .post_json(&format!("{}/pods/", self.base_url), &self.auth, payload)
                .await?,
        )
        .await?;
        let id = created["id"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Prime create missing id".into()))?
            .to_string();
        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::PrimeIntellect,
            id,
            sku,
            region.to_string(),
        ))
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        let raw = self.fetch_instance_json(&instance.id).await?;
        let command = raw["sshConnection"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Prime has no SSH endpoint".into()))?;
        crate::providers::common::gpu_adapter::ssh_endpoint(command)
    }

    async fn terminate_instance(&self, id: &str) -> Result<()> {
        self.delete_instance(id).await
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::parse_instance(&self.fetch_instance_json(id).await?, &self.default_region)
    }
    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        let response = self
            .http
            .get(&format!("{}/pods/{id}", self.base_url), &self.auth)
            .await?;
        match crate::providers::common::gpu_adapter::api_json_or_missing(response).await? {
            Some(raw) => Ok(Self::parse_instance(&raw, &self.default_region)?.status),
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
                let plan = super::PrimeIntellectInstanceMapper::map(resource_spec);
                provision_with_cleanup(
                    "prime-intellect",
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
                    |id| async move { self.delete_instance(&id).await },
                )
                .await
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "Prime Intellect does not offer managed Kubernetes".into(),
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
                "Prime Intellect does not offer serverless deployment".into(),
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
            SshDeploymentConfig::prime_intellect(),
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
