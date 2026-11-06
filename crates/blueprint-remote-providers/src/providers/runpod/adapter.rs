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

#[async_trait]
impl CloudProviderAdapter for RunPodAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        use crate::providers::common::gpu_adapter::{api_json, catalog_skus, quoted_selection};
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
        if !region.is_empty() {
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
        quoted_selection(sku, price)
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
        let name = generate_instance_name("runpod");
        let mut payload = serde_json::json!({
            "name": name,
            "gpuTypeIds": [instance_type],
            "cloudType": self.cloud_type,
            "gpuCount": spec.gpu_count.unwrap_or(1),
            "containerDiskInGb": spec.storage_gb.ceil() as u32,
            "volumeInGb": 0,
            "imageName": spec.image.as_deref().unwrap_or(crate::providers::common::gpu_adapter::DEFAULT_GPU_IMAGE),
            "env": {"PUBLIC_KEY":spec.ssh_public_key.as_deref().unwrap_or("")},
            "ports": "22/tcp,9615/tcp,9944/tcp",
            "supportPublicIp": true,
        });

        let location = if region.is_empty() {
            &self.default_region
        } else {
            region
        };
        if !location.is_empty() {
            payload["dataCenterIds"] = serde_json::json!([location]);
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
