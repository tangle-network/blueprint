//! Prime Intellect `CloudProviderAdapter` implementation.
//!
//! Prime Intellect is a compute aggregator: a single REST surface fronting
//! CoreWeave, Lambda, Crusoe, and other backend GPU providers. The adapter is
//! production-ready — operators can rely on it for routine GPU rentals.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    ClassifiedError, RetryPolicy, build_http_client, classify_response, deploy_via_ssh,
    generate_instance_name, poll_until, provision_with_cleanup, require_public_ip,
    retry_with_backoff,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const BASE_URL: &str = "https://api.primeintellect.ai/v1";
const INSTANCE_READY_TIMEOUT: Duration = Duration::from_secs(600);
const INSTANCE_POLL_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_IMAGE: &str = "ubuntu_22_04_cuda_12";

/// Adapter for the Prime Intellect compute aggregator.
pub struct PrimeIntellectAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    default_region: String,
    /// Sub-provider preference (`auto` lets Prime Intellect pick the cheapest backend).
    provider_preference: String,
    image: String,
}

impl PrimeIntellectAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("PRIME_INTELLECT_API_KEY").map_err(|_| {
            Error::Other("PRIME_INTELLECT_API_KEY environment variable not set".into())
        })?;
        let default_region =
            std::env::var("PRIME_INTELLECT_REGION").unwrap_or_else(|_| "us-east".to_string());
        let provider_preference =
            std::env::var("PRIME_INTELLECT_PROVIDER").unwrap_or_else(|_| "auto".to_string());
        let image =
            std::env::var("PRIME_INTELLECT_IMAGE").unwrap_or_else(|_| DEFAULT_IMAGE.to_string());

        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::prime_intellect(api_key),
            default_region,
            provider_preference,
            image,
        })
    }

    /// Fetch a single instance's JSON payload from the REST API with retry.
    async fn fetch_instance_json(&self, instance_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/instances/{instance_id}");
        retry_with_backoff(
            "prime_intellect.get_instance",
            &RetryPolicy::default(),
            |_| {
                let url = url.clone();
                async move {
                    let response = self.http.get(&url, &self.auth).await.map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Prime Intellect GET {url}: {e}"
                        )))
                    })?;
                    if let Some(class) = classify_response(&response) {
                        let body = response.text().await.unwrap_or_default();
                        let err =
                            Error::HttpError(format!("Prime Intellect GET {url} failed: {body}"));
                        return Err(ClassifiedError { class, inner: err });
                    }
                    response.json::<serde_json::Value>().await.map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Prime Intellect response parse: {e}"
                        )))
                    })
                }
            },
        )
        .await
    }

    /// Convert the Prime Intellect instance JSON into a `ProvisionedInstance`.
    fn parse_instance(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value.get("data").unwrap_or(value);
        let id = data
            .get("instance_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Prime Intellect response missing instance_id".into()))?
            .to_string();
        let instance_type = data
            .get("gpu_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = data
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = data
            .get("public_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = data
            .get("private_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match data.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "running" | "active" | "ready" => InstanceStatus::Running,
            "provisioning" | "starting" | "pending" | "queued" => InstanceStatus::Starting,
            "stopping" | "terminating" => InstanceStatus::Stopping,
            "terminated" | "failed" | "cancelled" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::PrimeIntellect,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, instance_id: &str) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        poll_until(
            "Prime Intellect instance",
            INSTANCE_POLL_INTERVAL,
            INSTANCE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_instance_json(instance_id).await?;
                let instance = Self::parse_instance(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Prime Intellect instance terminated before becoming ready".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }

    async fn launch_instance(&self, gpu_type: &str, region: &str) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("prime");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };

        let payload = serde_json::json!({
            "name": name,
            "gpu_type": gpu_type,
            "gpu_count": 1,
            "provider_preference": self.provider_preference,
            "region": region_name,
            "image": self.image,
        });

        let json: serde_json::Value =
            retry_with_backoff("prime_intellect.launch", &RetryPolicy::default(), |_| {
                let payload = payload.clone();
                async move {
                    let response = self
                        .http
                        .authenticated_request(
                            reqwest::Method::POST,
                            &format!("{BASE_URL}/instances"),
                            &self.auth,
                            Some(payload),
                        )
                        .await
                        .map_err(|e| {
                            ClassifiedError::transient(Error::HttpError(format!(
                                "Prime Intellect launch: {e}"
                            )))
                        })?;
                    if let Some(class) = classify_response(&response) {
                        let body = response.text().await.unwrap_or_default();
                        return Err(ClassifiedError {
                            class,
                            inner: Error::HttpError(format!(
                                "Prime Intellect launch failed: {body}"
                            )),
                        });
                    }
                    response.json::<serde_json::Value>().await.map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Prime Intellect launch parse: {e}"
                        )))
                    })
                }
            })
            .await?;

        let data = json.get("data").unwrap_or(&json);
        let instance_id = data
            .get("instance_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                Error::HttpError("Prime Intellect launch: no instance id returned".into())
            })?
            .to_string();
        let provider_used = data
            .get("provider_used")
            .and_then(|v| v.as_str())
            .unwrap_or("auto");

        info!(%instance_id, %region_name, %gpu_type, %provider_used, "Launched Prime Intellect instance");

        self.wait_for_running(&instance_id).await
    }

    async fn delete_instance(&self, instance_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/instances/{instance_id}");
        retry_with_backoff("prime_intellect.terminate", &RetryPolicy::default(), |_| {
            let url = url.clone();
            async move {
                let response = self
                    .http
                    .authenticated_request(reqwest::Method::DELETE, &url, &self.auth, None)
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Prime Intellect terminate: {e}"
                        )))
                    })?;
                if let Some(class) = classify_response(&response) {
                    // 404 on terminate is fine — already gone.
                    if response.status().as_u16() == 404 {
                        return Ok(());
                    }
                    let body = response.text().await.unwrap_or_default();
                    return Err(ClassifiedError {
                        class,
                        inner: Error::HttpError(format!(
                            "Prime Intellect terminate failed: {body}"
                        )),
                    });
                }
                Ok(())
            }
        })
        .await?;
        info!(%instance_id, "Terminated Prime Intellect instance");
        Ok(())
    }
}

#[async_trait]
impl CloudProviderAdapter for PrimeIntellectAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        self.launch_instance(instance_type, region).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.delete_instance(instance_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_instance_json(instance_id).await {
            Ok(raw) => {
                let instance = Self::parse_instance(&raw, &self.default_region)?;
                Ok(instance.status)
            }
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Prime Intellect instance status");
                Ok(InstanceStatus::Unknown)
            }
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
                    "prime_intellect",
                    || self.launch_instance(&plan.instance_type, ""),
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
            let client = build_http_client()?;
            match client
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_running_json() {
        let json = serde_json::json!({
            "data": {
                "instance_id": "pi-abc123",
                "status": "running",
                "gpu_type": "H100-80GB",
                "region": "us-east",
                "public_ip": "203.0.113.10",
                "private_ip": "10.0.1.5",
                "provider_used": "coreweave"
            }
        });
        let instance = PrimeIntellectAdapter::parse_instance(&json, "us-west").unwrap();
        assert_eq!(instance.id, "pi-abc123");
        assert_eq!(instance.public_ip.as_deref(), Some("203.0.113.10"));
        assert_eq!(instance.region, "us-east");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::PrimeIntellect);
    }

    #[test]
    fn parses_starting_json() {
        let json = serde_json::json!({
            "data": {
                "instance_id": "pi-pending",
                "status": "provisioning",
                "gpu_type": "A100-80GB",
                "region": "us-east"
            }
        });
        let instance = PrimeIntellectAdapter::parse_instance(&json, "us-east").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
        assert!(instance.public_ip.is_none());
    }

    #[test]
    fn parses_terminated_json() {
        let json = serde_json::json!({
            "data": {
                "instance_id": "pi-dead",
                "status": "terminated",
                "gpu_type": "RTX_4090",
                "region": "us-east"
            }
        });
        let instance = PrimeIntellectAdapter::parse_instance(&json, "us-east").unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    #[test]
    fn parse_fails_without_id() {
        let json = serde_json::json!({ "data": { "status": "running" } });
        assert!(PrimeIntellectAdapter::parse_instance(&json, "us-east").is_err());
    }
}
