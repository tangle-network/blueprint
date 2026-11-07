//! OVHcloud Public Cloud `CloudProviderAdapter` implementation.
//!
//! Uses `/cloud/project/{serviceName}/instance` and nothing outside `/auth/time` and
//! `/cloud/project`: the same OVH account can hold dedicated servers and orders.

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::gpu_adapter::{
    ClassifiedError, RetryPolicy, deploy_via_ssh, generate_instance_name, poll_until,
    require_public_ip, retry_with_backoff,
};
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::{info, warn};
use blueprint_std::collections::HashMap;
use reqwest::{Method, StatusCode};
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::OnceCell;

const READY_TIMEOUT: Duration = Duration::from_secs(600);
const POLL_INTERVAL: Duration = Duration::from_secs(10);

pub struct OvhAdapter {
    http: SecureHttpClient,
    api: String,
    project: String,
    application_key: String,
    application_secret: String,
    consumer_key: String,
    default_region: Option<String>,
    image: String,
    ssh_key_id: Option<String>,
    /// OVH's clock minus ours, read once from `/auth/time`; signatures carry OVH's time.
    clock_offset: OnceCell<i64>,
}

impl OvhAdapter {
    pub async fn new() -> Result<Self> {
        let required = [
            "OVH_ENDPOINT",
            "OVH_APPLICATION_KEY",
            "OVH_APPLICATION_SECRET",
            "OVH_CONSUMER_KEY",
            "OVH_CLOUD_PROJECT_SERVICE",
        ];
        let missing: Vec<_> = required
            .iter()
            .filter(|name| std::env::var_os(name).is_none())
            .copied()
            .collect();
        if !missing.is_empty() {
            return Err(Error::ConfigurationError(format!(
                "OVH needs {}",
                missing.join(", ")
            )));
        }
        let var = |name: &str| std::env::var(name).unwrap_or_default();
        let endpoint = var("OVH_ENDPOINT");
        // The names OVH's own SDKs accept; anything else is an API root URL.
        let api = match endpoint.as_str() {
            "ovh-eu" => "https://eu.api.ovh.com/1.0",
            "ovh-ca" => "https://ca.api.ovh.com/1.0",
            "ovh-us" => "https://api.us.ovhcloud.com/1.0",
            url => url.trim_end_matches('/'),
        }
        .to_owned();
        Ok(Self {
            // Requests go only to the API root the operator named.
            http: SecureHttpClient::with_endpoint(&api)?,
            api,
            project: var("OVH_CLOUD_PROJECT_SERVICE"),
            application_key: var("OVH_APPLICATION_KEY"),
            application_secret: var("OVH_APPLICATION_SECRET"),
            consumer_key: var("OVH_CONSUMER_KEY"),
            default_region: std::env::var("OVH_REGION").ok(),
            image: std::env::var("OVH_IMAGE").unwrap_or_else(|_| "Ubuntu 22.04".to_owned()),
            ssh_key_id: std::env::var("OVH_SSH_KEY_ID").ok(),
            clock_offset: OnceCell::new(),
        })
    }

    async fn auth(&self) -> Result<ApiAuthentication> {
        let now = || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_secs() as i64)
        };
        let offset =
            self.clock_offset
                .get_or_try_init(|| async {
                    let url = format!("{}/auth/time", self.api);
                    let response = self.http.get(&url, &ApiAuthentication::None).await?;
                    let text = response.text().await.unwrap_or_default();
                    let server: i64 = text.trim().parse().map_err(|_| {
                        Error::HttpError(format!("OVH GET /auth/time returned {text}"))
                    })?;
                    Ok::<_, Error>(server - now())
                })
                .await?;
        Ok(ApiAuthentication::Ovh {
            application_key: self.application_key.clone(),
            application_secret: self.application_secret.clone(),
            consumer_key: self.consumer_key.clone(),
            timestamp: now() + offset,
        })
    }

    /// One signed call under `/cloud/project/{project}`, retrying rate limits and server errors.
    /// `Ok(None)` means OVH answered 404.
    async fn call(&self, method: Method, path: &str, body: Option<Value>) -> Result<Option<Value>> {
        let url = &format!("{}/cloud/project/{}{path}", self.api, self.project);
        let (method, body) = (&method, &body);
        let response = retry_with_backoff("OVH call", &RetryPolicy::quick(), |_| async move {
            let auth = self.auth().await?;
            Ok::<_, ClassifiedError>(
                self.http
                    .authenticated_request(method.clone(), url, &auth, body.clone())
                    .await?,
            )
        })
        .await?;
        // The client answers only success and 404; every other status is already an error.
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let text = response.text().await.unwrap_or_default();
        Ok(Some(serde_json::from_str(&text).unwrap_or(Value::Null)))
    }

    fn parse_instance(value: &Value) -> Result<ProvisionedInstance> {
        let text = |field: &str| value.get(field).and_then(Value::as_str);
        let address = |kind: &str| {
            value
                .get("ipAddresses")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .find(|ip| {
                    ip.get("type").and_then(Value::as_str) == Some(kind)
                        && ip.get("version").and_then(Value::as_u64) == Some(4)
                })
                .and_then(|ip| ip.get("ip").and_then(Value::as_str))
                .map(str::to_owned)
        };
        let status = match text("status").unwrap_or_default() {
            "ACTIVE" => InstanceStatus::Running,
            "BUILD" | "BUILDING" | "REBOOT" | "HARD_REBOOT" | "REBUILD" | "RESUMING"
            | "UNSHELVING" | "RESIZE" | "VERIFY_RESIZE" | "MIGRATING" => InstanceStatus::Starting,
            "DELETING" | "SHELVING" => InstanceStatus::Stopping,
            "SHUTOFF" | "STOPPED" | "PAUSED" | "SUSPENDED" | "SHELVED" | "SHELVED_OFFLOADED" => {
                InstanceStatus::Stopped
            }
            "DELETED" | "SOFT_DELETED" => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id: text("id")
                .ok_or_else(|| Error::HttpError("OVH instance has no id".into()))?
                .to_owned(),
            provider: CloudProvider::Ovh,
            instance_type: value
                .pointer("/flavor/name")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_owned(),
            region: text("region").unwrap_or("unknown").to_owned(),
            public_ip: address("public"),
            private_ip: address("private"),
            status,
        })
    }

    /// The id of the first listed item whose name matches, from a region-scoped listing.
    async fn find_id(&self, listing: &str, name: &str) -> Result<String> {
        let items = self
            .call(Method::GET, listing, None)
            .await?
            .unwrap_or_default();
        items
            .as_array()
            .into_iter()
            .flatten()
            .find(|item| {
                item.get("name").and_then(Value::as_str) == Some(name)
                    && item
                        .get("osType")
                        .and_then(Value::as_str)
                        .unwrap_or("linux")
                        == "linux"
            })
            .and_then(|item| item.get("id").and_then(Value::as_str))
            .map(str::to_owned)
            .ok_or_else(|| Error::ConfigurationError(format!("OVH lists no {name} at {listing}")))
    }
}

#[async_trait]
impl CloudProviderAdapter for OvhAdapter {
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        if require_tee {
            return Err(Error::ConfigurationError(
                "OVH Public Cloud has no TEE attestation path; refusing require_tee".into(),
            ));
        }
        let region = match (region, &self.default_region) {
            ("", Some(default)) => default.as_str(),
            ("", None) => {
                return Err(Error::ConfigurationError(
                    "OVH needs a region: pass one or export OVH_REGION".into(),
                ));
            }
            (region, _) => region,
        };
        let flavor = self
            .find_id(&format!("/flavor?region={region}"), instance_type)
            .await?;
        let image = self
            .find_id(&format!("/image?osType=linux&region={region}"), &self.image)
            .await?;
        let mut payload = serde_json::json!({
            "name": generate_instance_name("ovh"),
            "flavorId": flavor,
            "imageId": image,
            "region": region,
            "monthlyBilling": false,
        });
        if let Some(key) = &self.ssh_key_id {
            payload["sshKeyId"] = key.as_str().into();
        }
        let created = self
            .call(Method::POST, "/instance", Some(payload))
            .await?
            .ok_or_else(|| Error::HttpError("OVH create instance: 404".into()))?;
        let id = Self::parse_instance(&created)?.id;
        info!(%id, %instance_type, region, "Created OVH instance");
        poll_until("OVH instance", POLL_INTERVAL, READY_TIMEOUT, || async {
            let instance = match self
                .call(Method::GET, &format!("/instance/{id}"), None)
                .await?
            {
                Some(value) => Self::parse_instance(&value)?,
                None => {
                    return Err(Error::HttpError(
                        "OVH instance vanished while building".into(),
                    ));
                }
            };
            Ok(
                (instance.status == InstanceStatus::Running && instance.public_ip.is_some())
                    .then_some(instance),
            )
        })
        .await
    }

    async fn terminate_instance(&self, instance_id: &str) -> Result<()> {
        // A 404 is OVH saying the instance is already gone.
        self.call(Method::DELETE, &format!("/instance/{instance_id}"), None)
            .await?;
        info!(%instance_id, "OVH accepted the instance deletion");
        Ok(())
    }

    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus> {
        match self
            .call(Method::GET, &format!("/instance/{instance_id}"), None)
            .await?
        {
            Some(value) => Ok(Self::parse_instance(&value)?.status),
            None => Ok(InstanceStatus::Terminated),
        }
    }

    async fn get_instance_details(&self, instance_id: &str) -> Result<ProvisionedInstance> {
        let value = self
            .call(Method::GET, &format!("/instance/{instance_id}"), None)
            .await?
            .ok_or_else(|| Error::HttpError(format!("OVH instance {instance_id}: 404")))?;
        Self::parse_instance(&value)
    }

    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        blueprint_image: &str,
        resource_spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::core::deployment_target::DeploymentTarget;
        let DeploymentTarget::VirtualMachine { .. } = target else {
            return Err(Error::ConfigurationError(
                "OVH deploys only to virtual machines through this adapter".into(),
            ));
        };
        let selection = crate::providers::ovh::OvhInstanceMapper::map(resource_spec);
        let instance = self
            .provision_instance(&selection.instance_type, "", false)
            .await?;
        let deployed = self
            .deploy_blueprint(&instance, blueprint_image, resource_spec, env_vars)
            .await;
        if let Err(error) = &deployed {
            warn!(instance_id = %instance.id, %error, "OVH deploy failed; deleting the instance");
            if let Err(cleanup) = self.terminate_instance(&instance.id).await {
                warn!(instance_id = %instance.id, %cleanup, "OVH cleanup failed; instance may be orphaned");
            }
        }
        deployed
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
            SshDeploymentConfig::ovh(),
        )
        .await
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        let Some(endpoint) = deployment.qos_grpc_endpoint() else {
            return Ok(false);
        };
        Ok(self
            .http
            .get(&format!("{endpoint}/health"), &ApiAuthentication::None)
            .await
            .is_ok_and(|response| response.status().is_success()))
    }

    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        self.terminate_instance(&deployment.instance.id).await
    }
}
