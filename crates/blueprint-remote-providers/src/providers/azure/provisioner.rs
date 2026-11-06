//! Azure leases create only a VM, attached to a caller-owned NIC.
//! Network setup belongs to the account; it cannot become an untracked lease side effect.
use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use crate::security::{ApiAuthentication, SecureHttpClient};

pub struct AzureProvisioner {
    subscription_id: String,
    resource_group: String,
    client: SecureHttpClient,
    auth: ApiAuthentication,
    base_url: String,
}

impl AzureProvisioner {
    pub async fn new() -> Result<Self> {
        let subscription = std::env::var("AZURE_SUBSCRIPTION_ID")
            .map_err(|_| Error::ConfigurationError("AZURE_SUBSCRIPTION_ID required".into()))?;
        let group =
            std::env::var("AZURE_RESOURCE_GROUP").unwrap_or_else(|_| "blueprint-resources".into());
        let token = crate::security::auth::azure_access_token().await?;
        Self::with_credentials(subscription, group, token)
    }

    pub fn with_credentials(
        subscription_id: String,
        resource_group: String,
        token: String,
    ) -> Result<Self> {
        Ok(Self {
            subscription_id,
            resource_group,
            client: SecureHttpClient::new()?,
            auth: ApiAuthentication::Bearer { token },
            base_url: "https://management.azure.com".into(),
        })
    }

    pub fn with_endpoint(
        subscription: String,
        group: String,
        token: String,
        endpoint: &str,
    ) -> Result<Self> {
        let mut provisioner = Self::with_credentials(subscription, group, token)?;
        provisioner.client = SecureHttpClient::with_endpoint(endpoint)?;
        provisioner.base_url = endpoint.trim_end_matches('/').into();
        Ok(provisioner)
    }

    pub async fn get_access_token(&mut self) -> Result<String> {
        match &self.auth {
            ApiAuthentication::Bearer { token } => Ok(token.clone()),
            _ => Err(Error::ConfigurationError(
                "Azure bearer token unavailable".into(),
            )),
        }
    }

    fn vm_id(&self, name: &str) -> String {
        format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{name}",
            self.subscription_id, self.resource_group
        )
    }

    pub async fn provision_instance(
        &mut self,
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
                "Azure lease attestation forwarding is unavailable".into(),
            ));
        }
        let key = spec
            .ssh_public_key
            .as_ref()
            .ok_or_else(|| Error::ConfigurationError("Azure lease SSH key required".into()))?;
        let nic = config.custom_config.get("network_interface_id")
            .ok_or_else(||Error::ConfigurationError("Azure requires a caller-owned network_interface_id; no untracked NIC/IP will be created".into()))?;
        let size = config
            .custom_config
            .get("vm_size")
            .cloned()
            .unwrap_or_else(|| self.select_vm_size(spec));
        if spec.gpu_count.is_some()
            && !crate::providers::common::gpu_adapter::catalog_skus(spec, "azure")?
                .iter()
                .any(|native| native == &size)
        {
            return Err(Error::ConfigurationError(
                "GPU request does not match the provider SKU".into(),
            ));
        }
        if size.is_empty() {
            return Err(Error::ConfigurationError("No exact Azure VM SKU".into()));
        }
        let id = self.vm_id(&config.name);
        let response = self.client.authenticated_request(reqwest::Method::PUT,&format!("{}{id}?api-version=2023-09-01",self.base_url),&self.auth,
            Some(serde_json::json!({"location":config.region,"tags":{"blueprint-lease":"true"},"properties":{
                "hardwareProfile":{"vmSize":size},
                "storageProfile":{"imageReference":{"publisher":"Canonical","offer":"0001-com-ubuntu-server-jammy","sku":"22_04-lts-gen2","version":"latest"},
                    "osDisk":{"createOption":"FromImage","deleteOption":"Delete","diskSizeGB":spec.storage_gb.ceil() as u64}},
                "osProfile":{"computerName":config.name,"adminUsername":"blueprint","linuxConfiguration":{"disablePasswordAuthentication":true,
                    "ssh":{"publicKeys":[{"path":"/home/blueprint/.ssh/authorized_keys","keyData":key}]}}},
                "networkProfile":{"networkInterfaces":[{"id":nic,"properties":{"primary":true,"deleteOption":"Detach"}}]}
            }}))).await?;
        if !response.status().is_success() {
            return Err(Error::HttpError(format!(
                "Azure VM create HTTP {}",
                response.status()
            )));
        }
        Ok(ProvisionedInfrastructure {
            provider: crate::CloudProvider::Azure,
            instance_id: id,
            public_ip: None,
            private_ip: None,
            region: config.region.clone(),
            instance_type: size,
            metadata: Default::default(),
        })
    }

    pub async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let id = if id.starts_with("/subscriptions/") {
            id.to_string()
        } else {
            self.vm_id(id)
        };
        let response = self
            .client
            .get(
                &format!(
                    "{}{id}?api-version=2023-09-01&$expand=instanceView",
                    self.base_url
                ),
                &self.auth,
            )
            .await?;
        if response.status().as_u16() == 404 {
            return Ok(ProvisionedInstance {
                id,
                provider: crate::CloudProvider::Azure,
                instance_type: String::new(),
                region: String::new(),
                public_ip: None,
                private_ip: None,
                status: InstanceStatus::Terminated,
            });
        }
        let vm = crate::providers::common::gpu_adapter::api_json(response).await?;
        let statuses = vm["properties"]["instanceView"]["statuses"].as_array();
        let power = statuses.and_then(|rows| {
            rows.iter()
                .filter_map(|r| r["code"].as_str())
                .find(|s| s.starts_with("PowerState/"))
        });
        let status = match power {
            Some("PowerState/running") => InstanceStatus::Running,
            Some("PowerState/starting") => InstanceStatus::Starting,
            Some("PowerState/stopped" | "PowerState/deallocated") => InstanceStatus::Stopped,
            _ if vm["properties"]["provisioningState"] == "Creating"
                || vm["properties"]["provisioningState"] == "Updating" =>
            {
                InstanceStatus::Starting
            }
            _ => InstanceStatus::Unknown,
        };
        let mut public_ip = None;
        let mut private_ip = None;
        if let Some(nic) = vm["properties"]["networkProfile"]["networkInterfaces"][0]["id"].as_str()
        {
            let network = self.resource(nic).await?;
            private_ip =
                network["properties"]["ipConfigurations"][0]["properties"]["privateIPAddress"]
                    .as_str()
                    .map(str::to_owned);
            if let Some(pip) =
                network["properties"]["ipConfigurations"][0]["properties"]["publicIPAddress"]["id"]
                    .as_str()
            {
                public_ip = self.resource(pip).await?["properties"]["ipAddress"]
                    .as_str()
                    .map(str::to_owned);
            }
        }
        Ok(ProvisionedInstance {
            id,
            provider: crate::CloudProvider::Azure,
            instance_type: vm["properties"]["hardwareProfile"]["vmSize"]
                .as_str()
                .unwrap_or_default()
                .into(),
            region: vm["location"].as_str().unwrap_or_default().into(),
            public_ip,
            private_ip,
            status,
        })
    }

    async fn resource(&self, id: &str) -> Result<serde_json::Value> {
        if !id.starts_with("/subscriptions/") {
            return Err(Error::HttpError("Invalid Azure resource reference".into()));
        }
        let response = self
            .client
            .get(
                &format!("{}{id}?api-version=2023-09-01", self.base_url),
                &self.auth,
            )
            .await?;
        crate::providers::common::gpu_adapter::api_json(response).await
    }

    pub(super) fn select_vm_size(&self, spec: &ResourceSpec) -> String {
        if spec.gpu_count.is_some() {
            return crate::providers::common::gpu_adapter::catalog_selection(spec, "azure")
                .instance_type;
        }
        match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 2.0 && mem <= 8.0 => "Standard_B2ms",
            (cpu, mem) if cpu <= 4.0 && mem <= 16.0 => "Standard_B4ms",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "Standard_D8s_v5",
            (cpu, mem) if cpu <= 16.0 && mem <= 64.0 => "Standard_D16s_v5",
            _ => "",
        }
        .into()
    }

    pub async fn terminate_instance(&mut self, id: &str) -> Result<()> {
        let id = if id.starts_with("/subscriptions/") {
            id.to_string()
        } else {
            self.vm_id(id)
        };
        let response = self
            .client
            .delete(
                &format!("{}{id}?api-version=2023-09-01", self.base_url),
                &self.auth,
            )
            .await?;
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(Error::HttpError(format!(
                "Azure VM deletion HTTP {}",
                response.status()
            )))
        }
    }
}
