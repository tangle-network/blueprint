//! Vultr provisioning returns the created identifier before any readiness work.
use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use crate::security::{ApiAuthentication, SecureHttpClient};
use serde::Deserialize;

#[derive(Default, Deserialize)]
#[serde(default)]
struct VultrInstance {
    id: String,
    main_ip: String,
    internal_ip: Option<String>,
    region: String,
    plan: String,
    status: String,
    power_status: String,
}

pub struct VultrProvisioner {
    client: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
}

impl VultrProvisioner {
    pub async fn new(api_key: String) -> Result<Self> {
        Self::with_credentials(api_key)
    }

    pub fn with_credentials(api_key: String) -> Result<Self> {
        Ok(Self {
            client: SecureHttpClient::new()?,
            auth: ApiAuthentication::Bearer { token: api_key },
            base_url: "https://api.vultr.com".into(),
        })
    }

    pub fn with_endpoint(api_key: String, endpoint: &str) -> Result<Self> {
        let mut provisioner = Self::with_credentials(api_key)?;
        provisioner.client = SecureHttpClient::with_endpoint(endpoint)?;
        provisioner.base_url = endpoint.trim_end_matches('/').into();
        Ok(provisioner)
    }

    pub async fn provision_instance(
        &self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        spec.validate().map_err(Error::ConfigurationError)?;
        let plan = config
            .custom_config
            .get("instance_type")
            .cloned()
            .unwrap_or_else(|| self.select_plan(spec));
        if spec.gpu_count.is_some()
            && !crate::providers::common::gpu_adapter::catalog_skus(spec, "vultr")?
                .iter()
                .any(|native| native == &plan)
        {
            return Err(Error::ConfigurationError(
                "GPU request does not match the provider SKU".into(),
            ));
        }
        if plan.is_empty() {
            return Err(Error::ConfigurationError("No exact Vultr SKU".into()));
        }
        let region = &config.region;
        let response = self.client.post_json(&format!("{}/v2/instances",self.base_url),&self.auth,serde_json::json!({
            "region":region,"plan":plan,"label":config.name,"hostname":config.name,"os_id":1743,
            "backups":"disabled","enable_ipv6":false,"activation_email":false,
            "ssh_keys":config.ssh_key_name.as_ref().map(|k|vec![k]).unwrap_or_default(),
            "user_data":Self::user_data(spec),
        })).await?;
        let json = crate::providers::common::gpu_adapter::api_json(response).await?;
        let id = json["instance"]["id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or_else(|| Error::HttpError("Vultr create response omitted id".into()))?;
        Ok(ProvisionedInfrastructure {
            provider: crate::CloudProvider::Vultr,
            instance_id: id.into(),
            public_ip: None,
            private_ip: None,
            region: region.clone(),
            instance_type: plan,
            metadata: Default::default(),
        })
    }

    pub async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let response = self
            .client
            .get(&format!("{}/v2/instances/{id}", self.base_url), &self.auth)
            .await?;
        let instance = if response.status().as_u16() == 404 {
            VultrInstance {
                id: id.into(),
                status: "terminated".into(),
                ..Default::default()
            }
        } else {
            let json = crate::providers::common::gpu_adapter::api_json(response).await?;
            serde_json::from_value::<VultrInstance>(json["instance"].clone())
                .map_err(|_| Error::HttpError("Invalid Vultr instance details".into()))?
        };
        if instance.id != id {
            return Err(Error::HttpError(
                "Vultr returned a different instance id".into(),
            ));
        }
        let status = match (instance.status.as_str(), instance.power_status.as_str()) {
            ("active", "running") => InstanceStatus::Running,
            ("active", "stopped") => InstanceStatus::Stopped,
            ("pending", _) => InstanceStatus::Starting,
            ("terminated", _) => InstanceStatus::Terminated,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id: instance.id,
            provider: crate::CloudProvider::Vultr,
            instance_type: instance.plan,
            region: instance.region,
            public_ip: (!instance.main_ip.is_empty() && instance.main_ip != "0.0.0.0")
                .then_some(instance.main_ip),
            private_ip: instance.internal_ip,
            status,
        })
    }

    pub async fn terminate_instance(&self, id: &str) -> Result<()> {
        let response = self
            .client
            .delete(&format!("{}/v2/instances/{id}", self.base_url), &self.auth)
            .await?;
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(Error::HttpError(format!(
                "Vultr deletion HTTP {}",
                response.status()
            )))
        }
    }

    pub async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        Ok(self.get_instance_details(id).await?.status)
    }

    pub(super) fn select_plan(&self, spec: &ResourceSpec) -> String {
        if spec.gpu_count.is_some() {
            return crate::providers::common::gpu_adapter::catalog_selection(spec, "vultr")
                .instance_type;
        }
        match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 1.0 && mem <= 1.0 => "vc2-1c-1gb",
            (cpu, mem) if cpu <= 1.0 && mem <= 2.0 => "vc2-1c-2gb",
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "vc2-2c-4gb",
            (cpu, mem) if cpu <= 4.0 && mem <= 8.0 => "vc2-4c-8gb",
            (cpu, mem) if cpu <= 6.0 && mem <= 16.0 => "vc2-6c-16gb",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "vc2-8c-32gb",
            _ => "",
        }
        .into()
    }

    fn user_data(spec: &ResourceSpec) -> String {
        use base64::Engine;
        let mut cloud_init = serde_json::json!({"packages":["docker.io"],"ssh_pwauth":false});
        if let Some(key) = &spec.ssh_public_key {
            cloud_init["ssh_authorized_keys"] = serde_json::json!([key]);
        }
        base64::engine::general_purpose::STANDARD.encode(format!("#cloud-config\n{cloud_init}"))
    }
}
