//! Akash Network `CloudProviderAdapter` implementation.
//!
//! See [`crate::providers::akash`] for the REST-adapter-over-relay design notes.
//! In short: this adapter speaks a small REST surface (assumed to be hosted by
//! a relay that fronts an Akash CLI / Cosmos SDK runtime) so we can avoid
//! pulling in the full Cosmos transaction-signing dependency tree.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::akash::instance_mapper::AkashInstanceMapper;
use crate::providers::akash::sdl::build_sdl_manifest;
use crate::providers::common::gpu_adapter::{
    ClassifiedError, ErrorClass, RetryPolicy, build_http_client, classify_response, deploy_via_ssh,
    generate_instance_name, poll_until, provision_with_cleanup, retry_with_backoff,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use std::time::Duration;

const AKASH_LEASE_READY_TIMEOUT: Duration = Duration::from_secs(900);
const AKASH_LEASE_POLL_INTERVAL: Duration = Duration::from_secs(15);
const DEFAULT_LEASE_BUDGET_UAKT: u64 = 5_000_000;

/// Adapter for Akash Network's decentralized GPU marketplace.
///
/// All HTTP calls go through [`retry_with_backoff`] with the default
/// [`RetryPolicy`], and provisioning is wrapped in [`provision_with_cleanup`]
/// so a failed deploy automatically closes the on-chain lease to prevent
/// runaway billing.
pub struct AkashAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    rpc_url: String,
    default_region: String,
    lease_budget_uakt: u64,
    preferred_provider: Option<String>,
    retry_policy: RetryPolicy,
}

impl AkashAdapter {
    /// Construct from environment variables.
    pub async fn new() -> Result<Self> {
        let rpc_url = std::env::var("AKASH_RPC_URL")
            .map_err(|_| Error::Other("AKASH_RPC_URL environment variable not set".into()))?;
        let token = std::env::var("AKASH_API_TOKEN").unwrap_or_default();
        let default_region = std::env::var("AKASH_REGION").unwrap_or_else(|_| "global".to_string());
        let lease_budget_uakt = std::env::var("AKASH_LEASE_BUDGET_UAKT")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_LEASE_BUDGET_UAKT);
        let preferred_provider = std::env::var("AKASH_PROVIDER_ADDRESS").ok();

        Ok(Self {
            http: build_http_client()?,
            auth: ApiAuthentication::akash(token),
            rpc_url: rpc_url.trim_end_matches('/').to_string(),
            default_region,
            lease_budget_uakt,
            preferred_provider,
            retry_policy: RetryPolicy::default(),
        })
    }

    /// Submit a new SDL deployment to the relay and return the deployment id.
    async fn submit_deployment(&self, sdl: &str) -> Result<String> {
        let url = format!("{}/deployments", self.rpc_url);
        let body = serde_json::json!({
            "sdl": sdl,
            "lease_budget_uakt": self.lease_budget_uakt,
        });

        let json = retry_with_backoff("akash submit_deployment", &self.retry_policy, |_attempt| {
            let url = url.clone();
            let body = body.clone();
            async move {
                let response = self
                    .http
                    .post(&url, &self.auth, Some(body))
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Akash POST {url}: {e}"
                        )))
                    })?;

                if let Some(class) = classify_response(&response) {
                    let body = response.text().await.unwrap_or_default();
                    let inner = Error::HttpError(format!("Akash POST {url} failed: {body}"));
                    return Err(ClassifiedError { class, inner });
                }

                response.json::<serde_json::Value>().await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!(
                        "Akash response parse: {e}"
                    )))
                })
            }
        })
        .await?;

        json.get("deployment_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| Error::HttpError("Akash submit: no deployment_id returned".into()))
    }

    /// Accept a bid against a deployment, creating an on-chain lease.
    async fn accept_bid(&self, deployment_id: &str) -> Result<()> {
        let url = format!("{}/deployments/{deployment_id}/accept", self.rpc_url);
        let body = serde_json::json!({
            "provider_address": self.preferred_provider.clone().unwrap_or_default(),
        });

        retry_with_backoff("akash accept_bid", &self.retry_policy, |_attempt| {
            let url = url.clone();
            let body = body.clone();
            async move {
                let response = self
                    .http
                    .post(&url, &self.auth, Some(body))
                    .await
                    .map_err(|e| {
                        ClassifiedError::transient(Error::HttpError(format!(
                            "Akash POST {url}: {e}"
                        )))
                    })?;
                if let Some(class) = classify_response(&response) {
                    let body = response.text().await.unwrap_or_default();
                    let inner = Error::HttpError(format!("Akash accept {url} failed: {body}"));
                    return Err(ClassifiedError { class, inner });
                }
                Ok(())
            }
        })
        .await
    }

    /// Fetch a deployment's current JSON payload.
    async fn fetch_deployment_json(&self, deployment_id: &str) -> Result<serde_json::Value> {
        let url = format!("{}/deployments/{deployment_id}", self.rpc_url);
        retry_with_backoff("akash fetch_deployment", &self.retry_policy, |_attempt| {
            let url = url.clone();
            async move {
                let response = self.http.get(&url, &self.auth).await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!("Akash GET {url}: {e}")))
                })?;
                if let Some(class) = classify_response(&response) {
                    let body = response.text().await.unwrap_or_default();
                    let inner = Error::HttpError(format!("Akash GET {url} failed: {body}"));
                    return Err(ClassifiedError { class, inner });
                }
                response.json::<serde_json::Value>().await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!(
                        "Akash response parse: {e}"
                    )))
                })
            }
        })
        .await
    }

    /// Parse a relay deployment payload into a `ProvisionedInstance`.
    fn parse_deployment(
        value: &serde_json::Value,
        fallback_region: &str,
        instance_type: &str,
    ) -> Result<ProvisionedInstance> {
        let data = value.get("data").unwrap_or(value);

        // Lease id is either a flat string or a dseq/gseq/oseq triple — serialize
        // the triple as `dseq/gseq/oseq` for storage.
        let id = if let Some(s) = data.get("lease_id").and_then(|v| v.as_str()) {
            s.to_string()
        } else if let Some(obj) = data.get("lease_id").and_then(|v| v.as_object()) {
            let dseq = obj.get("dseq").and_then(|v| v.as_str()).unwrap_or("0");
            let gseq = obj.get("gseq").and_then(|v| v.as_str()).unwrap_or("0");
            let oseq = obj.get("oseq").and_then(|v| v.as_str()).unwrap_or("0");
            format!("{dseq}/{gseq}/{oseq}")
        } else if let Some(s) = data.get("deployment_id").and_then(|v| v.as_str()) {
            s.to_string()
        } else {
            return Err(Error::HttpError(
                "Akash response missing lease_id/deployment_id".into(),
            ));
        };

        let public_ip = data
            .get("public_ip")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let region = data
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(fallback_region)
            .to_string();

        let status = match data.get("status").and_then(|v| v.as_str()).unwrap_or("") {
            "active" | "running" | "leased" => InstanceStatus::Running,
            "pending" | "bidding" | "matched" | "starting" => InstanceStatus::Starting,
            "closing" => InstanceStatus::Stopping,
            "closed" | "terminated" | "expired" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };

        Ok(ProvisionedInstance {
            id,
            provider: CloudProvider::Akash,
            instance_type: instance_type.to_string(),
            region,
            public_ip,
            private_ip: None,
            status,
        })
    }

    async fn wait_for_lease_ready(
        &self,
        deployment_id: &str,
        instance_type: &str,
    ) -> Result<ProvisionedInstance> {
        let region = self.default_region.clone();
        let instance_type = instance_type.to_string();
        poll_until(
            "Akash lease",
            AKASH_LEASE_POLL_INTERVAL,
            AKASH_LEASE_READY_TIMEOUT,
            || async {
                let raw = self.fetch_deployment_json(deployment_id).await?;
                let instance = Self::parse_deployment(&raw, &region, &instance_type)?;
                match instance.status {
                    InstanceStatus::Running if instance.public_ip.is_some() => Ok(Some(instance)),
                    InstanceStatus::Terminated => Err(Error::HttpError(
                        "Akash lease terminated before becoming ready".into(),
                    )),
                    _ => Ok(None),
                }
            },
        )
        .await
    }

    async fn close_deployment(&self, deployment_id: &str) -> Result<()> {
        let url = format!("{}/deployments/{deployment_id}", self.rpc_url);
        retry_with_backoff("akash close_deployment", &self.retry_policy, |_attempt| {
            let url = url.clone();
            async move {
                let response = self.http.delete(&url, &self.auth).await.map_err(|e| {
                    ClassifiedError::transient(Error::HttpError(format!("Akash DELETE {url}: {e}")))
                })?;
                if let Some(class) = classify_response(&response) {
                    // 404 means it's already gone — treat as success.
                    if matches!(class, ErrorClass::Permanent) && response.status().as_u16() == 404 {
                        return Ok(());
                    }
                    let body = response.text().await.unwrap_or_default();
                    let inner = Error::HttpError(format!("Akash DELETE {url} failed: {body}"));
                    return Err(ClassifiedError { class, inner });
                }
                Ok(())
            }
        })
        .await
    }
}

#[async_trait]
impl CloudProviderAdapter for AkashAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        _require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        let label = generate_instance_name("akash");
        let region_name = if region.is_empty() {
            self.default_region.as_str()
        } else {
            region
        };

        // The `instance_type` is either a named profile (e.g. "gpu-a100-80gb")
        // or a pre-built SDL manifest. If it doesn't look like YAML, build a
        // default SDL around it using a placeholder image — the deploy step
        // will replace the image when actually launching the blueprint.
        let sdl = if instance_type.contains("---") || instance_type.contains("services:") {
            instance_type.to_string()
        } else {
            build_sdl_manifest(
                "blueprint/placeholder:latest",
                instance_type,
                &ResourceSpec {
                    cpu: 1.0,
                    memory_gb: 1.0,
                    storage_gb: 10.0,
                    gpu_count: Some(1),
                    allow_spot: false,
                    qos: Default::default(),
                },
                &[],
            )
        };

        let deployment_id = self.submit_deployment(&sdl).await?;
        info!(%deployment_id, %label, "Submitted Akash deployment");

        if let Err(e) = self.accept_bid(&deployment_id).await {
            warn!(%deployment_id, error = %e, "Accept bid failed; closing deployment");
            let _ = self.close_deployment(&deployment_id).await;
            return Err(e);
        }

        let mut instance = self
            .wait_for_lease_ready(&deployment_id, instance_type)
            .await?;
        // Carry the deployment id forward — that's what we DELETE on terminate.
        instance.id = deployment_id;
        instance.region = region_name.to_string();
        Ok(instance)
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        self.close_deployment(instance_id).await?;
        info!(%instance_id, "Closed Akash deployment");
        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self.fetch_deployment_json(instance_id).await {
            Ok(raw) => {
                let instance = Self::parse_deployment(&raw, &self.default_region, "akash-lease")?;
                Ok(instance.status)
            }
            Err(e) => {
                warn!(%instance_id, error = %e, "Failed to get Akash deployment status");
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
                let plan = AkashInstanceMapper::map(resource_spec);
                let instance_type = plan.instance_type.clone();
                provision_with_cleanup(
                    "akash",
                    || async { self.provision_instance(&instance_type, "", false).await },
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
                    |id| async move { self.terminate_instance(&id).await },
                )
                .await
            }
            DeploymentTarget::ManagedKubernetes { .. } => Err(Error::ConfigurationError(
                "Akash does not offer managed Kubernetes".into(),
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
                "Akash does not offer serverless deployment".into(),
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
        // Akash leases expose a forwarded SSH endpoint via the lease's public IP.
        if instance.public_ip.is_none() {
            return Err(Error::ConfigurationError(
                "Akash lease has no public_ip; cannot SSH-deploy".into(),
            ));
        }
        deploy_via_ssh(
            instance,
            blueprint_image,
            resource_spec,
            env_vars,
            SshDeploymentConfig::akash(),
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
    fn parses_active_lease_json() {
        let json = serde_json::json!({
            "data": {
                "lease_id": { "dseq": "12345", "gseq": "1", "oseq": "1" },
                "status": "active",
                "public_ip": "203.0.113.42",
                "region": "us-west"
            }
        });
        let instance = AkashAdapter::parse_deployment(&json, "global", "gpu-a100-80gb").unwrap();
        assert_eq!(instance.id, "12345/1/1");
        assert_eq!(instance.public_ip.as_deref(), Some("203.0.113.42"));
        assert_eq!(instance.region, "us-west");
        assert_eq!(instance.status, InstanceStatus::Running);
        assert_eq!(instance.provider, CloudProvider::Akash);
        assert_eq!(instance.instance_type, "gpu-a100-80gb");
    }

    #[test]
    fn parses_pending_lease_as_starting() {
        let json = serde_json::json!({
            "data": {
                "deployment_id": "dep-abc",
                "status": "pending"
            }
        });
        let instance = AkashAdapter::parse_deployment(&json, "global", "gpu-t4").unwrap();
        assert_eq!(instance.id, "dep-abc");
        assert_eq!(instance.status, InstanceStatus::Starting);
        assert!(instance.public_ip.is_none());
        assert_eq!(instance.region, "global");
    }

    #[test]
    fn parses_closed_lease_as_terminated() {
        let json = serde_json::json!({
            "data": {
                "lease_id": "lease-1",
                "status": "closed"
            }
        });
        let instance = AkashAdapter::parse_deployment(&json, "global", "gpu-h100").unwrap();
        assert_eq!(instance.status, InstanceStatus::Terminated);
    }

    #[test]
    fn parses_flat_lease_id_string() {
        let json = serde_json::json!({
            "lease_id": "abc/1/2",
            "status": "running",
            "public_ip": "10.0.0.1"
        });
        let instance = AkashAdapter::parse_deployment(&json, "global", "gpu-t4").unwrap();
        assert_eq!(instance.id, "abc/1/2");
        assert_eq!(instance.status, InstanceStatus::Running);
    }

    #[test]
    fn parse_fails_without_any_id() {
        let json = serde_json::json!({
            "data": { "status": "active" }
        });
        assert!(AkashAdapter::parse_deployment(&json, "global", "gpu-t4").is_err());
    }
}
