//! Bittensor Lium (subnet 51) `CloudProviderAdapter` implementation.
//!
//! **Stability note:** Lium is primarily a Python SDK; the REST shim used here
//! is documented at <https://docs.lium.ai> as a best-effort interop surface.
//! Endpoint shapes may evolve as the subnet matures. Payment settlement happens
//! out-of-band via the Bittensor CLI/SDK against the operator's coldkey — this
//! adapter only handles the rental lifecycle (create/poll/terminate), never
//! TAO transfers. Operators must fund their hotkey/coldkey wallets before use.

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

const BASE_URL: &str = "https://api.lium.ai/v1";
// Subnet consensus + miner orchestration is slower than centralized clouds; give
// rentals more headroom before timing out.
const RENTAL_READY_TIMEOUT: Duration = Duration::from_secs(900);
const RENTAL_POLL_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_DURATION_HOURS: u64 = 24;

/// Adapter for Bittensor Lium subnet 51 GPU rentals.
pub struct BittensorLiumAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    hotkey: String,
    coldkey: String,
    region: String,
    duration_hours: u64,
    ssh_pubkey: Option<String>,
}

impl BittensorLiumAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("LIUM_API_KEY")
            .map_err(|_| Error::Other("LIUM_API_KEY environment variable not set".into()))?;
        let hotkey = std::env::var("LIUM_WALLET_HOTKEY")
            .map_err(|_| Error::Other("LIUM_WALLET_HOTKEY environment variable not set".into()))?;
        let coldkey = std::env::var("LIUM_WALLET_COLDKEY")
            .map_err(|_| Error::Other("LIUM_WALLET_COLDKEY environment variable not set".into()))?;
        let region = std::env::var("LIUM_REGION").unwrap_or_else(|_| "auto".to_string());
        let duration_hours = std::env::var("LIUM_DURATION_HOURS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_DURATION_HOURS);
        let ssh_pubkey = std::env::var("LIUM_SSH_PUBKEY").ok();

        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::bittensor_lium(api_key),
            hotkey,
            coldkey,
            region,
            duration_hours,
            ssh_pubkey,
        })
    }

    async fn fetch_rental_json(&self, rental_id: &str) -> Result<serde_json::Value> {
        let url = format!("{BASE_URL}/rentals/{rental_id}");
        retry_with_backoff("lium.get_rental", &RetryPolicy::default(), |_| {
            let url = url.clone();
            async move {
                let response = self.http.get(&url, &self.auth).await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!("Lium GET {url}: {e}")))
                })?;
                if let Some(class) = classify_response(&response) {
                    let body = response.text().await.unwrap_or_default();
                    return Err(ClassifiedError {
                        class,
                        inner: Error::HttpError(format!("Lium GET {url} failed: {body}")),
                    });
                }
                response.json::<serde_json::Value>().await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!(
                        "Lium response parse: {e}"
                    )))
                })
            }
        })
        .await
    }

    /// Convert a Lium rental JSON payload into a `ProvisionedInstance`.
    fn parse_instance(
        value: &serde_json::Value,
        fallback_region: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value
            .get("rental")
            .or_else(|| value.get("data"))
            .unwrap_or(value);
        let id = data
            .get("rental_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Lium response missing rental_id".into()))?
            .to_string();
        let instance_type = data
            .get("gpu_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let region = data
            .get("region")
            .or_else(|| data.get("datacenter"))
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();
        let public_ip = data
            .get("public_ip")
            .or_else(|| data.get("ip"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let private_ip = data
            .get("private_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let status = match data.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "active" | "running" | "ready" => InstanceStatus::Running,
            "matching" | "pending" | "provisioning" | "starting" | "queued" => {
                InstanceStatus::Starting
            }
            "stopping" | "ending" => InstanceStatus::Stopping,
            "ended" | "expired" | "terminated" | "failed" | "cancelled" => {
                InstanceStatus::Terminated
            }
            _ => InstanceStatus::Unknown,
        };

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::BittensorLium,
            instance_type,
            region,
            public_ip,
            private_ip,
            status,
        })
    }

    async fn wait_for_running(&self, rental_id: &str) -> Result<ProvisionedInstance> {
        let region = self.region.clone();
        poll_until(
            "Bittensor Lium rental",
            RENTAL_POLL_INTERVAL,
            RENTAL_READY_TIMEOUT,
            || async {
                let raw = self.fetch_rental_json(rental_id).await?;
                let instance = Self::parse_instance(&raw, &region)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Lium rental terminated before becoming ready".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }

    async fn create_rental(&self, gpu_type: &str, _region: &str) -> Result<ProvisionedInstance> {
        let name = generate_instance_name("lium");
        let payload = serde_json::json!({
            "name": name,
            "hotkey": self.hotkey,
            "coldkey": self.coldkey,
            "gpu_type": gpu_type,
            "duration_hours": self.duration_hours,
            "ssh_pubkey": self.ssh_pubkey.clone().unwrap_or_default(),
        });

        let json: serde_json::Value =
            retry_with_backoff("lium.create_rental", &RetryPolicy::default(), |_| {
                let payload = payload.clone();
                async move {
                    let response = self
                        .http
                        .authenticated_request(
                            reqwest::Method::POST,
                            &format!("{BASE_URL}/rentals"),
                            &self.auth,
                            Some(payload),
                        )
                        .await
                        .map_err(|e| {
                            ClassifiedError::transient(Error::HttpError(format!(
                                "Lium create rental: {e}"
                            )))
                        })?;
                    if let Some(class) = classify_response(&response) {
                        let body = response.text().await.unwrap_or_default();
                        return Err(ClassifiedError {
                            class,
                            inner: Error::HttpError(format!("Lium create rental failed: {body}")),
                        });
                    }
                    response.json::<serde_json::Value>().await.map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Lium rental parse: {e}"
                        )))
                    })
                }
            })
            .await?;

        let data = json
            .get("rental")
            .or_else(|| json.get("data"))
            .unwrap_or(&json);
        let rental_id = data
            .get("rental_id")
            .or_else(|| data.get("id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::HttpError("Lium create: no rental id returned".into()))?
            .to_string();
        let miner_uid = data
            .get("miner_uid")
            .and_then(|v| {
                v.as_u64()
                    .map(|u| u.to_string())
                    .or_else(|| v.as_str().map(String::from))
            })
            .unwrap_or_else(|| "unknown".to_string());

        info!(%rental_id, %gpu_type, %miner_uid, "Created Bittensor Lium rental");

        self.wait_for_running(&rental_id).await
    }

    async fn cancel_rental(&self, rental_id: &str) -> Result<()> {
        let url = format!("{BASE_URL}/rentals/{rental_id}");
        retry_with_backoff("lium.cancel_rental", &RetryPolicy::default(), |_| {
            let url = url.clone();
            async move {
                let response = self
                    .http
                    .authenticated_request(reqwest::Method::DELETE, &url, &self.auth, None)
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Lium cancel rental: {e}"
                        )))
                    })?;
                if let Some(class) = classify_response(&response) {
                    if response.status().as_u16() == 404 {
                        return Ok(());
                    }
                    let body = response.text().await.unwrap_or_default();
                    return Err(ClassifiedError {
                        class,
                        inner: Error::HttpError(format!("Lium cancel failed: {body}")),
                    });
                }
                Ok(())
            }
        })
        .await?;
        info!(%rental_id, "Cancelled Bittensor Lium rental");
        Ok(())
    }
}

#[async_trait]
impl CloudProviderAdapter for BittensorLiumAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        self.create_rental(instance_type, region).await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.cancel_rental(instance_id).await
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_rental_json(instance_id).await {
            Ok(raw) => {
                let instance = Self::parse_instance(&raw, &self.region)?;
                Ok(instance.status)
            }
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Lium rental status");
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
                let plan = super::BittensorLiumInstanceMapper::map(resource_spec);
                provision_with_cleanup(
                    "bittensor_lium",
                    || self.create_rental(&plan.instance_type, ""),
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
            "rental": {
                "rental_id": "lium-9001",
                "status": "active",
                "gpu_type": "H100-80GB",
                "datacenter": "eu-central",
                "public_ip": "192.0.2.55",
                "private_ip": "10.20.0.7",
                "miner_uid": 42
            }
        });
        let instance = BittensorLiumAdapter::parse_instance(&json, "auto").unwrap();
        assert_eq!(instance.id, "lium-9001");
        assert_eq!(instance.public_ip.as_deref(), Some("192.0.2.55"));
        assert_eq!(instance.region, "eu-central");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::BittensorLium);
    }

    #[test]
    fn parses_starting_json() {
        let json = serde_json::json!({
            "rental": {
                "rental_id": "lium-pending",
                "status": "matching",
                "gpu_type": "A100-80GB"
            }
        });
        let instance = BittensorLiumAdapter::parse_instance(&json, "auto").unwrap();
        assert_eq!(instance.status, InstanceStatus::Starting);
        assert!(instance.public_ip.is_none());
        assert_eq!(instance.region, "auto");
    }

    #[test]
    fn parses_terminated_json() {
        let json = serde_json::json!({
            "rental": {
                "rental_id": "lium-dead",
                "status": "ended",
                "gpu_type": "RTX_4090"
            }
        });
        let instance = BittensorLiumAdapter::parse_instance(&json, "auto").unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    #[test]
    fn parse_fails_without_id() {
        let json = serde_json::json!({ "rental": { "status": "active" } });
        assert!(BittensorLiumAdapter::parse_instance(&json, "auto").is_err());
    }
}
