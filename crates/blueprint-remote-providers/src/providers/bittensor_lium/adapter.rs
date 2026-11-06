//! Lium node rentals using its public rent-by-spec API.
use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{
    InstanceSelection,
    gpu_adapter::{
        api_json, build_http_client, catalog_sku, catalog_skus, deploy_via_ssh,
        generate_instance_name, provision_with_cleanup, quoted_selection, reject_tee,
        require_public_ip,
    },
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_std::collections::HashMap;
use serde_json::{Value, json};

const BASE_URL: &str = "https://lium.io/api";

pub struct BittensorLiumAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    duration_hours: u64,
}

impl BittensorLiumAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("LIUM_API_KEY")
            .map_err(|_| Error::ConfigurationError("LIUM_API_KEY is required".into()))?;
        Self::with_credentials(key)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::ApiKey {
                key: api_key,
                header_name: "X-API-Key".into(),
            },
            base_url: BASE_URL.into(),
            duration_hours: 24,
        })
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        Ok(adapter)
    }

    fn rental_request(
        &self,
        spec: &ResourceSpec,
        dry_run: bool,
        selected_sku: Option<&str>,
    ) -> Result<Value> {
        let sku = selected_sku
            .map(String::from)
            .map(Ok)
            .unwrap_or_else(|| catalog_sku(spec, "bittensor-lium"))?;
        Ok(
            json!({"pod_name":generate_instance_name("lium"),"gpu_type":sku,
            "gpu_count":spec.gpu_count.unwrap_or(1),"min_vram_gb":spec.gpu_vram_gb,
            "min_cpus":spec.cpu.ceil() as u32,"min_ram_gb":spec.memory_gb,
            "min_disk_gb":spec.storage_gb,"user_public_key":spec.ssh_public_key.as_deref().unwrap_or(""),
            "termination_hours":spec.lease_duration_secs.map(|secs|secs.div_ceil(3600)).unwrap_or(self.duration_hours),"dry_run":dry_run}),
        )
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
        let id = raw["id"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Lium pod has no id".into()))?;
        Ok(ProvisionedInstance {
            id: id.into(),
            provider: CloudProvider::BittensorLium,
            instance_type: raw["executor"]["machine_name"]
                .as_str()
                .unwrap_or_default()
                .into(),
            region: String::new(),
            public_ip: raw["executor"]["executor_ip_address"]
                .as_str()
                .map(String::from),
            private_ip: None,
            status: match raw["status"].as_str().unwrap_or_default() {
                "RUNNING" => InstanceStatus::Running,
                "FAILED" | "TERMINATED" => InstanceStatus::Terminated,
                _ => InstanceStatus::Starting,
            },
        })
    }

    async fn cancel_rental(&self, id: &str) -> Result<()> {
        let response = self
            .http
            .delete(&format!("{}/pods/{id}", self.base_url), &self.auth)
            .await?;
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(Error::HttpError(format!(
                "Lium terminate HTTP {}",
                response.status()
            )))
        }
    }
}

#[async_trait]
impl CloudProviderAdapter for BittensorLiumAdapter {
    async fn quote(&self, spec: &ResourceSpec, _region: &str) -> Result<InstanceSelection> {
        let mut best: Option<InstanceSelection> = None;
        for sku in catalog_skus(spec, "bittensor-lium")? {
            let response = self
                .http
                .post_json(
                    &format!("{}/executors/rent-by-spec", self.base_url),
                    &self.auth,
                    self.rental_request(spec, true, Some(&sku))?,
                )
                .await;
            let Ok(response) = response else {
                continue;
            };
            let Ok(value) = api_json(response).await else {
                continue;
            };
            let Ok(candidate) =
                quoted_selection(sku, value["price_per_hour"].as_f64().unwrap_or(f64::NAN))
            else {
                continue;
            };
            if best
                .as_ref()
                .is_none_or(|old| candidate.estimated_hourly_cost < old.estimated_hourly_cost)
            {
                best = Some(candidate);
            }
        }
        best.ok_or_else(|| Error::ConfigurationError("Lium has no qualifying priced node".into()))
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
        _region: &str,
        spec: &ResourceSpec,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        reject_tee(tee)?;
        let skus = catalog_skus(spec, "bittensor-lium")?;
        let sku = skus
            .iter()
            .find(|sku| sku.as_str() == instance_type)
            .or_else(|| skus.first())
            .cloned()
            .ok_or_else(|| Error::ConfigurationError("Lium GPU SKU missing".into()))?;
        let request = self.rental_request(spec, false, Some(&sku))?;
        let created = api_json(
            self.http
                .post_json(
                    &format!("{}/executors/rent-by-spec", self.base_url),
                    &self.auth,
                    request,
                )
                .await?,
        )
        .await?;
        let id = created["pod_id"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Lium create missing pod_id".into()))?
            .to_string();
        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::BittensorLium,
            id,
            sku,
            String::new(),
        ))
    }

    async fn terminate_instance(&self, id: &str) -> Result<()> {
        self.cancel_rental(id).await
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::parse_instance(&self.fetch_rental_json(id).await?)
    }
    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        let response = self
            .http
            .get(&format!("{}/pods/{id}", self.base_url), &self.auth)
            .await?;
        match crate::providers::common::gpu_adapter::api_json_or_missing(response).await? {
            Some(raw) => Ok(Self::parse_instance(&raw)?.status),
            None => Ok(InstanceStatus::Terminated),
        }
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        let raw = self.fetch_rental_json(&instance.id).await?;
        let command = raw["ssh_connect_cmd"]
            .as_str()
            .ok_or_else(|| Error::HttpError("Lium pod has no SSH endpoint".into()))?;
        crate::providers::common::gpu_adapter::ssh_endpoint(command)
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
