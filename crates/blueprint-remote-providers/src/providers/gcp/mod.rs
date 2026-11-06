//! GCE REST provisioning. A successful insert returns its resource name before polling.
pub mod adapter;
use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{InstanceSelection, ProvisionedInfrastructure, ProvisioningConfig};
use crate::security::{ApiAuthentication, SecureHttpClient};

pub struct GcpProvisioner {
    project_id: String,
    client: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
}

impl GcpProvisioner {
    pub async fn new(project_id: String) -> Result<Self> {
        Self::with_credentials(project_id, crate::security::auth::gcp_access_token().await?)
    }

    pub fn with_credentials(project_id: String, token: String) -> Result<Self> {
        Ok(Self {
            project_id,
            client: SecureHttpClient::new()?,
            auth: ApiAuthentication::google_cloud(token),
            base_url: "https://compute.googleapis.com".into(),
        })
    }

    pub fn with_endpoint(project_id: String, token: String, endpoint: &str) -> Result<Self> {
        let mut provisioner = Self::with_credentials(project_id, token)?;
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
        if config
            .custom_config
            .get("require_tee")
            .is_some_and(|v| v == "true")
        {
            return Err(Error::ConfigurationError(
                "GCP lease attestation forwarding is unavailable".into(),
            ));
        }
        let sku = config
            .custom_config
            .get("instance_type")
            .cloned()
            .unwrap_or_else(|| Self::map_instance(spec).instance_type);
        if spec.gpu_count.is_some()
            && !crate::providers::common::gpu_adapter::catalog_skus(spec, "gcp")?
                .iter()
                .any(|native| native == &sku)
        {
            return Err(Error::ConfigurationError(
                "GPU request does not match the provider SKU".into(),
            ));
        }
        if sku.is_empty() {
            return Err(Error::ConfigurationError(
                "No exact GCP machine type".into(),
            ));
        }
        let zone = if config
            .region
            .rsplit('-')
            .next()
            .is_some_and(|suffix| suffix.len() == 1)
        {
            config.region.clone()
        } else {
            format!("{}-a", config.region)
        };
        let key = spec
            .ssh_public_key
            .as_ref()
            .map(|key| format!("blueprint:{key}"))
            .or_else(|| config.custom_config.get("ssh_public_key").cloned())
            .ok_or_else(|| Error::ConfigurationError("GCP instance SSH key required".into()))?;
        let id = format!(
            "projects/{}/zones/{zone}/instances/{}",
            self.project_id, config.name
        );
        let response = self.client.post_json(&format!("{}/compute/v1/projects/{}/zones/{zone}/instances",self.base_url,self.project_id),&self.auth,
            serde_json::json!({"name":config.name,"machineType":format!("zones/{zone}/machineTypes/{sku}"),
                "disks":[{"boot":true,"autoDelete":true,"initializeParams":{"sourceImage":config.machine_image.as_deref()
                    .unwrap_or("projects/ubuntu-os-cloud/global/images/family/ubuntu-2204-lts"),"diskSizeGb":spec.storage_gb.ceil() as u64}}],
                "networkInterfaces":[{"network":"global/networks/default","accessConfigs":[{"type":"ONE_TO_ONE_NAT","name":"External NAT"}]}],
                "metadata":{"items":[{"key":"ssh-keys","value":key},{"key":"enable-oslogin","value":"FALSE"},
                    {"key":"startup-script","value":Self::generate_startup_script()}]},
                "labels":{"managed_by":"blueprint_remote_providers"}
            })).await?;
        if !response.status().is_success() {
            return Err(Error::HttpError(format!(
                "GCP insert HTTP {}",
                response.status()
            )));
        }
        Ok(ProvisionedInfrastructure {
            provider: crate::CloudProvider::GCP,
            instance_id: id,
            public_ip: None,
            private_ip: None,
            region: config.region.clone(),
            instance_type: sku,
            metadata: Default::default(),
        })
    }

    pub async fn get_instance_details(
        &self,
        name: &str,
        zone: &str,
    ) -> Result<ProvisionedInstance> {
        let id = format!("projects/{}/zones/{zone}/instances/{name}", self.project_id);
        let response = self
            .client
            .get(&format!("{}/compute/v1/{id}", self.base_url), &self.auth)
            .await?;
        if response.status().as_u16() == 404 {
            return Ok(ProvisionedInstance {
                id,
                provider: crate::CloudProvider::GCP,
                instance_type: String::new(),
                region: String::new(),
                public_ip: None,
                private_ip: None,
                status: InstanceStatus::Terminated,
            });
        }
        let instance = crate::providers::common::gpu_adapter::api_json(response).await?;
        if instance["name"] != name {
            return Err(Error::HttpError(
                "GCP returned a different instance name".into(),
            ));
        }
        let status = match instance["status"].as_str() {
            Some("RUNNING") => InstanceStatus::Running,
            Some("PROVISIONING" | "STAGING") => InstanceStatus::Starting,
            Some("STOPPING") => InstanceStatus::Stopping,
            Some("TERMINATED" | "SUSPENDED") => InstanceStatus::Stopped,
            _ => InstanceStatus::Unknown,
        };
        Ok(ProvisionedInstance {
            id,
            provider: crate::CloudProvider::GCP,
            instance_type: instance["machineType"]
                .as_str()
                .and_then(|s| s.rsplit('/').next())
                .unwrap_or_default()
                .into(),
            region: zone.rsplit_once('-').map(|(r, _)| r).unwrap_or(zone).into(),
            public_ip: instance["networkInterfaces"][0]["accessConfigs"][0]["natIP"]
                .as_str()
                .map(str::to_owned),
            private_ip: instance["networkInterfaces"][0]["networkIP"]
                .as_str()
                .map(str::to_owned),
            status,
        })
    }

    pub async fn terminate_instance(&self, name: &str, zone: &str) -> Result<()> {
        let response = self
            .client
            .delete(
                &format!(
                    "{}/compute/v1/projects/{}/zones/{zone}/instances/{name}",
                    self.base_url, self.project_id
                ),
                &self.auth,
            )
            .await?;
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(Error::HttpError(format!(
                "GCP deletion HTTP {}",
                response.status()
            )))
        }
    }

    fn generate_startup_script() -> &'static str {
        "#!/bin/sh\nset -eu\napt-get update\napt-get install -y docker.io\n"
    }

    fn map_instance(spec: &ResourceSpec) -> InstanceSelection {
        if spec.gpu_count.is_some() {
            return crate::providers::common::gpu_adapter::catalog_selection(spec, "gcp");
        }
        let sku = match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 2.0 && mem <= 8.0 => "e2-standard-2",
            (cpu, mem) if cpu <= 4.0 && mem <= 16.0 => "n2-standard-4",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "n2-standard-8",
            (cpu, mem) if cpu <= 16.0 && mem <= 64.0 => "n2-standard-16",
            _ => "",
        };
        InstanceSelection {
            gpu_class: spec.gpu_class.clone(),
            instance_type: sku.into(),
            spot_capable: false,
            estimated_hourly_cost: None,
        }
    }

    pub fn get_instance_recommendation(&self, spec: &ResourceSpec) -> InstanceSelection {
        Self::map_instance(spec)
    }
    pub fn get_cost_estimate(&self, _instance_type: &str) -> Option<f64> {
        None
    }
    pub fn get_startup_script(&self) -> &'static str {
        Self::generate_startup_script()
    }
}
pub use adapter::GcpAdapter;
