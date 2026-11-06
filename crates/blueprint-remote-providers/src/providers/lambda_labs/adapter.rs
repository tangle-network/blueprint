//! Lambda Labs `CloudProviderAdapter` implementation.

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

const BASE_URL: &str = "https://cloud.lambda.com/api/v1";

/// Adapter for Lambda Labs on-demand GPU cloud.
pub struct LambdaLabsAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
    default_region: String,
    ssh_key_name: Option<String>,
}

impl LambdaLabsAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let key = std::env::var("LAMBDA_LABS_API_KEY")
            .map_err(|_| Error::ConfigurationError("LAMBDA_LABS_API_KEY is required".into()))?;
        let mut adapter = Self::with_credentials(key)?;
        adapter.default_region =
            std::env::var("LAMBDA_LABS_REGION").unwrap_or_else(|_| "us-west-1".into());
        adapter.ssh_key_name = std::env::var("LAMBDA_LABS_SSH_KEY_NAME").ok();
        Ok(adapter)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::lambda_labs(api_key),
            base_url: BASE_URL.into(),
            default_region: "us-west-1".into(),
            ssh_key_name: None,
        })
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        Ok(adapter)
    }

    /// Fetch a single instance's JSON payload from the REST API.
    async fn fetch_instance_json(&self, instance_id: &str) -> Result<serde_json::Value> {
        crate::providers::common::gpu_adapter::api_json(
            self.http
                .get(
                    &format!("{}/instances/{instance_id}", self.base_url),
                    &self.auth,
                )
                .await?,
        )
        .await
    }

    fn parse_instance(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value.get("data").unwrap_or(value);
        let id = data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Lambda Labs response missing id".into()))?
            .to_string();
        let instance_type = data
            .get("instance_type")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = data
            .get("region")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = data
            .get("ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = data
            .get("private_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match data.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "active" => InstanceStatus::Running,
            "booting" => InstanceStatus::Starting,
            "terminating" => InstanceStatus::Stopping,
            "terminated" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::LambdaLabs,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }
}

#[async_trait]
impl CloudProviderAdapter for LambdaLabsAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        use crate::providers::common::gpu_adapter::{api_json, catalog_skus, quoted_selection};
        let skus = catalog_skus(spec, "lambda-labs")?;
        let value = api_json(
            self.http
                .get(&format!("{}/instance-types", self.base_url), &self.auth)
                .await?,
        )
        .await?;
        let mut best: Option<crate::providers::common::InstanceSelection> = None;
        for sku in skus {
            let candidate = (|| {
                let entry = &value["data"][&sku];
                let available = entry["regions_with_capacity_available"]
                    .as_array()
                    .is_some_and(|regions| {
                        regions
                            .iter()
                            .any(|item| region.is_empty() || item["name"] == region)
                    });
                if !available {
                    return Err(Error::ConfigurationError(
                        "Lambda GPU is out of stock in requested region".into(),
                    ));
                }
                let instance = &entry["instance_type"];
                if instance["specs"]["gpus"].as_u64() != Some(spec.gpu_count.unwrap_or(1).into()) {
                    return Err(Error::ConfigurationError(
                        "Lambda SKU does not satisfy exact GPU count".into(),
                    ));
                }
                let minimum = spec.gpu_vram_gb.unwrap_or(0).max(
                    spec.gpu_class
                        .as_deref()
                        .and_then(blueprint_core::gpu::find)
                        .map(|class| class.vram_gb)
                        .unwrap_or(0),
                );
                let description = instance["gpu_description"]
                    .as_str()
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                let reported_vram = description
                    .match_indices("gb")
                    .filter_map(|(index, _)| {
                        let digits: String = description[..index]
                            .trim_end()
                            .chars()
                            .rev()
                            .take_while(char::is_ascii_digit)
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect();
                        digits.parse::<u32>().ok()
                    })
                    .max();
                let ambiguous = blueprint_core::gpu::catalog().iter().any(|class| {
                    class.sku("lambda-labs", spec.gpu_count.unwrap_or(1)) == Some(sku.as_str())
                        && class.vram_gb < minimum
                });
                if reported_vram.is_some_and(|vram| vram < minimum)
                    || (reported_vram.is_none() && ambiguous)
                {
                    return Err(Error::ConfigurationError(
                        "Lambda SKU does not prove requested VRAM per GPU".into(),
                    ));
                }
                for (field, minimum) in [
                    ("vcpus", spec.cpu),
                    ("memory_gib", spec.memory_gb),
                    ("storage_gib", spec.storage_gb),
                ] {
                    if instance["specs"][field]
                        .as_f64()
                        .is_some_and(|amount| amount < f64::from(minimum))
                    {
                        return Err(Error::ConfigurationError(format!(
                            "Lambda SKU cannot satisfy {field}"
                        )));
                    }
                }
                quoted_selection(
                    sku,
                    instance["price_cents_per_hour"]
                        .as_f64()
                        .unwrap_or(f64::NAN)
                        / 100.0,
                )
            })();
            if let Ok(candidate) = candidate {
                if best
                    .as_ref()
                    .is_none_or(|old| candidate.estimated_hourly_cost < old.estimated_hourly_cost)
                {
                    best = Some(candidate);
                }
            }
        }
        best.ok_or_else(|| {
            Error::ConfigurationError("Lambda has no qualifying priced in-stock node".into())
        })
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
                &ResourceSpec::default().with_gpu(
                    crate::providers::common::gpu_adapter::sku_gpu_count(
                        "lambda-labs",
                        instance_type,
                    ),
                ),
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
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        if !crate::providers::common::gpu_adapter::catalog_skus(spec, "lambda-labs")?
            .iter()
            .any(|sku| sku == instance_type)
        {
            return Err(Error::ConfigurationError(
                "Provider SKU disagrees with requested GPU class/count".into(),
            ));
        }
        spec.validate().map_err(Error::ConfigurationError)?;
        if instance_type.is_empty() {
            return Err(Error::ConfigurationError(
                "Lambda has no exact-count catalog SKU".into(),
            ));
        }
        // Fail closed at the trait boundary: this provider has no
        // confidential-compute / TEE attestation path. Refuse rather than
        // silently return a non-TEE instance for a require_tee request.
        if require_tee {
            return Err(Error::ConfigurationError(
                "Lambda Labs does not support confidential-compute / TEE attestation; \
                 refusing require_tee"
                    .into(),
            ));
        }
        let name = generate_instance_name("lambda");
        let available_region;
        let region_name = if region.is_empty() {
            let value = crate::providers::common::gpu_adapter::api_json(
                self.http
                    .get(&format!("{}/instance-types", self.base_url), &self.auth)
                    .await?,
            )
            .await?;
            let regions = value["data"][instance_type]["regions_with_capacity_available"]
                .as_array()
                .ok_or_else(|| {
                    Error::ConfigurationError("Lambda has no capacity listing for SKU".into())
                })?;
            available_region = regions
                .iter()
                .find(|region| region["name"] == self.default_region)
                .or_else(|| regions.first())
                .and_then(|region| region["name"].as_str())
                .map(String::from)
                .ok_or_else(|| {
                    Error::ConfigurationError("Lambda SKU has no in-stock region".into())
                })?;
            available_region.as_str()
        } else {
            region
        };

        let mut payload = serde_json::json!({
            "region_name": region_name,
            "instance_type_name": instance_type,
            "quantity": 1,
            "name": name,
        });
        if let Some(public_key) = &spec.ssh_public_key {
            let key_name = generate_instance_name("lease-key");
            crate::providers::common::gpu_adapter::api_json(
                self.http
                    .post_json(
                        &format!("{}/ssh-keys", self.base_url),
                        &self.auth,
                        serde_json::json!({"name":key_name,"public_key":public_key}),
                    )
                    .await?,
            )
            .await?;
            payload["ssh_key_names"] = serde_json::json!([key_name]);
        } else if let Some(key) = &self.ssh_key_name {
            payload["ssh_key_names"] = serde_json::json!([key]);
        }

        let launch_url = format!("{}/instance-operations/launch", self.base_url);
        let json = crate::providers::common::gpu_adapter::api_json(
            self.http
                .post_json(&launch_url, &self.auth, payload)
                .await?,
        )
        .await?;
        let instance_id = json
            .get("data")
            .and_then(|d| d.get("instance_ids"))
            .and_then(|ids| ids.get(0))
            .and_then(|id| id.as_str())
            .ok_or_else(|| Error::HttpError("Lambda Labs launch: no instance id returned".into()))?
            .to_string();

        info!(%instance_id, %region_name, %instance_type, "Launched Lambda Labs instance");

        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::LambdaLabs,
            instance_id,
            instance_type.to_string(),
            region_name.to_string(),
        ))
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::parse_instance(&self.fetch_instance_json(id).await?, &self.default_region)
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        Ok(format!("ssh://ubuntu@{}:22", require_public_ip(instance)?))
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        let payload = serde_json::json!({ "instance_ids": [instance_id] });
        let terminate_url = format!("{}/instance-operations/terminate", self.base_url);
        let mut last_err = None;
        for attempt in 0..3u32 {
            if attempt > 0 {
                tokio::time::sleep(Duration::from_millis(500 * 2u64.pow(attempt))).await;
            }
            match self
                .http
                .authenticated_request(
                    reqwest::Method::POST,
                    &terminate_url,
                    &self.auth,
                    Some(payload.clone()),
                )
                .await
            {
                Ok(response) if response.status().is_success() => {
                    info!(%instance_id, "Terminated Lambda Labs instance");
                    return Ok(());
                }
                Ok(response) if response.status() == reqwest::StatusCode::NOT_FOUND => {
                    info!(%instance_id, "Lambda Labs instance already terminated (404)");
                    return Ok(());
                }
                Ok(response) if response.status() == 429 || response.status().is_server_error() => {
                    last_err = Some(Error::HttpError(format!(
                        "Lambda Labs terminate: transient {}",
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
                    last_err = Some(Error::HttpError(format!("Lambda Labs terminate: {e}")));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        let response = self
            .http
            .get(&format!("{}/instances/{id}", self.base_url), &self.auth)
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
                let selection =
                    crate::providers::lambda_labs::LambdaLabsInstanceMapper::map(resource_spec);
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
                            provider = "lambda-labs",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "lambda-labs",
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
                "Lambda Labs does not offer managed Kubernetes".into(),
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
                "Lambda Labs does not offer serverless deployment".into(),
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
            SshDeploymentConfig::lambda_labs(),
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
