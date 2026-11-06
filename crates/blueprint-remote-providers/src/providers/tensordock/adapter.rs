//! TensorDock's documented Marketplace API (form bodies, never URL credentials).
//! https://documenter.getpostman.com/view/20973002/2s8YzMYRDc
use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{
    InstanceSelection,
    gpu_adapter::{
        api_json, build_http_client, catalog_skus, deploy_via_ssh, generate_instance_name,
        quoted_selection, reject_tee, require_public_ip,
    },
};
use crate::providers::tensordock::TensorDockInstanceMapper;
use crate::security::{ApiAuthentication, SecureHttpClient};
use crate::shared::SshDeploymentConfig;
use async_trait::async_trait;
use blueprint_core::warn;
use blueprint_std::collections::HashMap;
use serde_json::{Value, json};

const BASE_URL: &str = "https://dashboard.tensordock.com/api/v0";

pub struct TensorDockAdapter {
    http: SecureHttpClient,
    api_key: String,
    api_token: String,
    base_url: String,
}

impl TensorDockAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("TENSORDOCK_API_KEY")
            .map_err(|_| Error::ConfigurationError("TENSORDOCK_API_KEY is required".into()))?;
        let token = std::env::var("TENSORDOCK_API_TOKEN")
            .map_err(|_| Error::ConfigurationError("TENSORDOCK_API_TOKEN is required".into()))?;
        Self::with_credentials(key, token)
    }

    pub fn with_credentials(api_key: String, api_token: String) -> Result<Self> {
        Ok(Self {
            http: build_http_client()?,
            api_key,
            api_token,
            base_url: BASE_URL.into(),
        })
    }

    pub fn with_endpoint(api_key: String, api_token: String, endpoint: &str) -> Result<Self> {
        let mut adapter = Self::with_credentials(api_key, api_token)?;
        adapter.http = SecureHttpClient::with_endpoint(endpoint)?;
        adapter.base_url = endpoint.trim_end_matches('/').into();
        Ok(adapter)
    }

    async fn post(&self, path: &str, mut fields: HashMap<String, String>) -> Result<Value> {
        fields.insert("api_key".into(), self.api_key.clone());
        fields.insert("api_token".into(), self.api_token.clone());
        let value = api_json(
            self.http
                .post_form(
                    &format!("{}{}", self.base_url, path),
                    &ApiAuthentication::None,
                    &fields,
                )
                .await?,
        )
        .await?;
        if value["success"] != true {
            return Err(Error::HttpError("TensorDock refused request".into()));
        }
        Ok(value)
    }

    async fn offer(&self, spec: &ResourceSpec, region: &str) -> Result<(Value, f64)> {
        let skus = catalog_skus(spec, "tensordock")?;
        let count = spec.gpu_count.unwrap_or(1);
        let value = api_json(
            self.http
                .get(
                    &format!("{}/client/deploy/hostnodes", self.base_url),
                    &ApiAuthentication::None,
                )
                .await?,
        )
        .await?;
        let nodes = value["hostnodes"]
            .as_object()
            .ok_or_else(|| Error::HttpError("TensorDock hostnodes missing".into()))?;
        let mut best: Option<(Value, f64)> = None;
        for (id, node) in nodes {
            for sku in &skus {
                let gpu = &node["specs"]["gpu"][&sku];
                if node["status"]["online"] != true
                    || node["status"]["listed"] != true
                    || gpu["amount"].as_u64().unwrap_or(0) < count.into()
                    || gpu["vram"].as_u64().unwrap_or(0) < spec.gpu_vram_gb.unwrap_or(0).into()
                    || (!region.is_empty() && node["location"]["id"] != region)
                {
                    continue;
                }
                let mut price = gpu["price"].as_f64().unwrap_or(f64::NAN) * f64::from(count);
                let mut valid = true;
                for (field, amount) in [
                    ("cpu", spec.cpu),
                    ("ram", spec.memory_gb),
                    ("storage", spec.storage_gb),
                ] {
                    let resource = &node["specs"][field];
                    let restriction = &node["specs"]["restrictions"][count.to_string()][field];
                    let amount = f64::from(amount.ceil());
                    if resource["amount"].as_f64().unwrap_or(0.0) < amount
                        || restriction["min"].as_f64().is_some_and(|min| amount < min)
                        || restriction["max"].as_f64().is_some_and(|max| amount > max)
                    {
                        valid = false;
                        break;
                    }
                    price += resource["price"].as_f64().unwrap_or(f64::NAN) * amount;
                }
                let Some(ports) = node["networking"]["ports"].as_array() else {
                    continue;
                };
                let Some(port) = ports.first().and_then(Value::as_u64) else {
                    continue;
                };
                if !valid || !price.is_finite() || price <= 0.0 {
                    continue;
                }
                let offer = json!({"hostnode":id,"gpu_model":sku,"ssh_port":port});
                if best.as_ref().is_none_or(|(_, current)| price < *current) {
                    best = Some((offer, price));
                }
            }
        }
        best.ok_or_else(|| {
            Error::ConfigurationError("TensorDock has no priced in-stock matching host".into())
        })
    }

    async fn fetch_instance(&self, id: &str) -> Result<Value> {
        self.post(
            "/client/get/single",
            HashMap::from([("server".into(), id.into())]),
        )
        .await
    }

    fn parse_instance(raw: &Value, id: &str) -> Result<ProvisionedInstance> {
        let vm = &raw["virtualmachine"];
        Ok(ProvisionedInstance {
            id: id.into(),
            provider: CloudProvider::TensorDock,
            instance_type: vm["specs"]["gpu"]["type"]
                .as_str()
                .unwrap_or_default()
                .into(),
            region: vm["location"].as_str().unwrap_or_default().into(),
            public_ip: vm["hostname"].as_str().map(String::from),
            private_ip: None,
            status: match vm["status"].as_str().unwrap_or_default() {
                "Running" | "running" => InstanceStatus::Running,
                "Deleted" => InstanceStatus::Terminated,
                "Stopped" | "Outbid" => InstanceStatus::Stopped,
                _ => InstanceStatus::Starting,
            },
        })
    }
}

#[async_trait]
impl CloudProviderAdapter for TensorDockAdapter {
    async fn quote(&self, spec: &ResourceSpec, region: &str) -> Result<InstanceSelection> {
        let (offer, price) = self.offer(spec, region).await?;
        quoted_selection(offer.to_string(), price)
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
        let skus = catalog_skus(spec, "tensordock")?;
        let offer = match serde_json::from_str::<Value>(instance_type) {
            Ok(offer)
                if offer["gpu_model"]
                    .as_str()
                    .is_some_and(|sku| skus.iter().any(|candidate| candidate == sku)) =>
            {
                offer
            }
            _ => self.offer(spec, region).await?.0,
        };
        let sku = offer["gpu_model"]
            .as_str()
            .ok_or_else(|| Error::HttpError("TensorDock quote missing GPU type".into()))?
            .to_string();
        let mut fields = HashMap::from([
            ("name".into(), generate_instance_name("tensordock")),
            ("deployment_type".into(), "local".into()),
            ("gpu_model".into(), sku.clone()),
            ("gpu_count".into(), spec.gpu_count.unwrap_or(1).to_string()),
            ("vcpus".into(), spec.cpu.ceil().to_string()),
            ("ram".into(), spec.memory_gb.ceil().to_string()),
            ("storage".into(), spec.storage_gb.ceil().to_string()),
            ("operating_system".into(), "Ubuntu 22.04 LTS".into()),
            (
                "hostnode".into(),
                offer["hostnode"].as_str().unwrap_or_default().into(),
            ),
            ("external_ports".into(), format!("[{}]", offer["ssh_port"])),
            ("internal_ports".into(), "[22]".into()),
        ]);
        if let Some(key) = &spec.ssh_public_key {
            fields.insert("ssh_key".into(), key.clone());
        }
        let created = self.post("/client/deploy/single", fields).await?;
        let id = created["server"]
            .as_str()
            .ok_or_else(|| Error::HttpError("TensorDock create missing server".into()))?
            .to_string();
        Ok(crate::providers::common::gpu_adapter::starting_instance(
            CloudProvider::TensorDock,
            id,
            sku,
            region.to_string(),
        ))
    }

    async fn terminate_instance(&self, id: &str) -> Result<()> {
        self.post(
            "/client/delete/single",
            HashMap::from([("server".into(), id.into())]),
        )
        .await?;
        Ok(())
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        Self::parse_instance(&self.fetch_instance(id).await?, id)
    }
    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        let list = self.post("/client/list", HashMap::new()).await?;
        let instances = list["virtualmachines"]
            .as_object()
            .ok_or_else(|| Error::HttpError("TensorDock VM list missing".into()))?;
        match instances.get(id) {
            Some(vm) => Ok(Self::parse_instance(&json!({"virtualmachine":vm}), id)?.status),
            None => Ok(InstanceStatus::Terminated),
        }
    }

    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        let raw = self.fetch_instance(&instance.id).await?;
        let vm = &raw["virtualmachine"];
        let port = vm["port_forwards"]
            .as_object()
            .and_then(|ports| {
                ports
                    .iter()
                    .find(|(_, internal)| {
                        internal.as_str() == Some("22") || internal.as_u64() == Some(22)
                    })
                    .map(|(external, _)| external)
            })
            .ok_or_else(|| Error::HttpError("TensorDock has no SSH port forward".into()))?;
        Ok(format!(
            "ssh://user@{}:{port}",
            require_public_ip(instance)?
        ))
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
                let selection = TensorDockInstanceMapper::map(resource_spec);
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
                            provider = "tensordock",
                            instance_id = %instance_id_for_cleanup,
                            error = %deploy_err,
                            "Deploy failed after provisioning; terminating instance to prevent billing leak"
                        );
                        if let Err(cleanup_err) =
                            self.terminate_instance(&instance_id_for_cleanup).await
                        {
                            warn!(
                                provider = "tensordock",
                                instance_id = %instance_id_for_cleanup,
                                cleanup_error = %cleanup_err,
                                "Cleanup after failed deploy also failed — instance may be orphaned"
                            );
                        }
                        Err(deploy_err)
                    }
                }
            }
            _ => Err(Error::ConfigurationError(
                "TensorDock only supports VirtualMachine deployment targets".into(),
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
            SshDeploymentConfig::tensordock(),
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
