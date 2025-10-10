//! Azure Resource Manager provisioning
//!
//! Provisions Azure Virtual Machines using Azure Resource Manager APIs

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::providers::common::{ProvisionedInfrastructure, ProvisioningConfig};
use blueprint_core::{debug, info, warn};

/// Azure Resource Manager provisioner
pub struct AzureProvisioner {
    subscription_id: String,
    resource_group: String,
    client: reqwest::Client,
    access_token: Option<String>,
}

impl AzureProvisioner {
    /// Create new Azure provisioner
    pub async fn new() -> Result<Self> {
        let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")
            .map_err(|_| Error::ConfigurationError("AZURE_SUBSCRIPTION_ID not set".into()))?;

        let resource_group = std::env::var("AZURE_RESOURCE_GROUP")
            .unwrap_or_else(|_| "blueprint-resources".to_string());

        let client = reqwest::Client::new();

        Ok(Self {
            subscription_id,
            resource_group,
            client,
            access_token: None,
        })
    }

    /// Get Azure access token
    pub async fn get_access_token(&mut self) -> Result<String> {
        if let Some(token) = &self.access_token {
            return Ok(token.clone());
        }

        // Try managed identity first
        let token_url = "http://169.254.169.254/metadata/identity/oauth2/token";
        let params = [
            ("api-version", "2018-02-01"),
            ("resource", "https://management.azure.com/"),
        ];

        let response = self
            .client
            .get(token_url)
            .header("Metadata", "true")
            .query(&params)
            .send()
            .await;

        if let Ok(resp) = response {
            if resp.status().is_success() {
                let json: serde_json::Value = resp.json().await.map_err(|e| {
                    Error::ConfigurationError(format!("Failed to parse token: {e}"))
                })?;
                if let Some(token) = json["access_token"].as_str() {
                    self.access_token = Some(token.to_string());
                    return Ok(token.to_string());
                }
            }
        }

        // Fall back to Azure CLI
        use std::process::Command;
        let output = Command::new("az")
            .args([
                "account",
                "get-access-token",
                "--query",
                "accessToken",
                "-o",
                "tsv",
            ])
            .output()
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to get Azure token via CLI: {e}"))
            })?;

        if !output.status.success() {
            return Err(Error::ConfigurationError(
                "Failed to get Azure access token".into(),
            ));
        }

        let token = String::from_utf8(output.stdout)
            .map_err(|e| Error::ConfigurationError(format!("Invalid token format: {e}")))?
            .trim()
            .to_string();

        self.access_token = Some(token.clone());
        Ok(token)
    }

    /// Provision an Azure VM
    pub async fn provision_instance(
        &mut self,
        spec: &ResourceSpec,
        config: &ProvisioningConfig,
    ) -> Result<ProvisionedInfrastructure> {
        let token = self.get_access_token().await?;
        let vm_name = config.name.clone();
        let location = if config.region.is_empty() {
            "eastus"
        } else {
            &config.region
        };

        // Validate SSH public key is provided
        let ssh_public_key = std::env::var("AZURE_SSH_PUBLIC_KEY").map_err(|_| {
            Error::ConfigurationError(
                "AZURE_SSH_PUBLIC_KEY environment variable is required for Azure VM provisioning. \
                 Generate a key with: ssh-keygen -t rsa -b 4096 -f ~/.ssh/azure_key".into(),
            )
        })?;

        // Create network interface first
        let nic_name = format!("{vm_name}-nic");
        let nic_id = self
            .create_network_interface(&nic_name, location, &token)
            .await?;

        // Determine VM size based on spec
        let vm_size = self.select_vm_size(spec);

        // Create VM
        let vm_body = serde_json::json!({
            "location": location,
            "properties": {
                "hardwareProfile": {
                    "vmSize": vm_size
                },
                "storageProfile": {
                    "imageReference": {
                        "publisher": "Canonical",
                        "offer": "0001-com-ubuntu-server-jammy",
                        "sku": "22_04-lts-gen2",
                        "version": "latest"
                    },
                    "osDisk": {
                        "createOption": "FromImage",
                        "managedDisk": {
                            "storageAccountType": "Premium_LRS"
                        }
                    }
                },
                "osProfile": {
                    "computerName": vm_name,
                    "adminUsername": "azureuser",
                    "linuxConfiguration": {
                        "disablePasswordAuthentication": true,
                        "ssh": {
                            "publicKeys": [{
                                "path": "/home/azureuser/.ssh/authorized_keys",
                                "keyData": ssh_public_key
                            }]
                        }
                    }
                },
                "networkProfile": {
                    "networkInterfaces": [{
                        "id": nic_id,
                        "properties": {
                            "primary": true
                        }
                    }]
                }
            }
        });

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, vm_name
        );

        let response = self
            .client
            .put(&url)
            .bearer_auth(&token)
            .json(&vm_body)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create VM: {e}")))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Azure API error: {error_text}"
            )));
        }

        // Wait for VM to be ready and get IP
        let public_ip = self.wait_for_vm(&vm_name, &token).await?;

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("vm_size".to_string(), vm_size.to_string());
        metadata.insert("location".to_string(), location.to_string());
        metadata.insert("os".to_string(), "Ubuntu 22.04 LTS".to_string());

        Ok(ProvisionedInfrastructure {
            provider: crate::core::remote::CloudProvider::Azure,
            instance_id: format!(
                "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}",
                self.subscription_id, self.resource_group, vm_name
            ),
            public_ip: Some(public_ip),
            private_ip: None,
            region: location.to_string(),
            instance_type: vm_size.to_string(),
            metadata,
        })
    }

    /// Create network interface
    async fn create_network_interface(
        &self,
        nic_name: &str,
        location: &str,
        token: &str,
    ) -> Result<String> {
        // First ensure we have a virtual network
        let vnet_name = "blueprint-vnet";
        let subnet_name = "default";
        self.ensure_virtual_network(vnet_name, subnet_name, location, token)
            .await?;

        // Create public IP
        let pip_name = format!("{nic_name}-pip");
        let pip_id = self.create_public_ip(&pip_name, location, token).await?;

        // Create network interface
        let subnet_id = format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}/subnets/{}",
            self.subscription_id, self.resource_group, vnet_name, subnet_name
        );

        let nic_body = serde_json::json!({
            "location": location,
            "properties": {
                "ipConfigurations": [{
                    "name": "ipconfig1",
                    "properties": {
                        "subnet": {
                            "id": subnet_id
                        },
                        "privateIPAllocationMethod": "Dynamic",
                        "publicIPAddress": {
                            "id": pip_id
                        }
                    }
                }]
            }
        });

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkInterfaces/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, nic_name
        );

        let response = self
            .client
            .put(&url)
            .bearer_auth(token)
            .json(&nic_body)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create NIC: {e}")))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Failed to create NIC: {error_text}"
            )));
        }

        Ok(format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkInterfaces/{}",
            self.subscription_id, self.resource_group, nic_name
        ))
    }

    /// Ensure virtual network exists
    async fn ensure_virtual_network(
        &self,
        vnet_name: &str,
        subnet_name: &str,
        location: &str,
        token: &str,
    ) -> Result<()> {
        let vnet_body = serde_json::json!({
            "location": location,
            "properties": {
                "addressSpace": {
                    "addressPrefixes": ["10.0.0.0/16"]
                },
                "subnets": [{
                    "name": subnet_name,
                    "properties": {
                        "addressPrefix": "10.0.1.0/24"
                    }
                }]
            }
        });

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/virtualNetworks/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, vnet_name
        );

        match self
            .client
            .put(&url)
            .bearer_auth(token)
            .json(&vnet_body)
            .send()
            .await
        {
            Ok(_) => info!("Virtual network {} created successfully", vnet_name),
            Err(e) => warn!("Failed to create virtual network {}: {}", vnet_name, e),
        }

        Ok(())
    }

    /// Create public IP
    async fn create_public_ip(
        &self,
        pip_name: &str,
        location: &str,
        token: &str,
    ) -> Result<String> {
        let pip_body = serde_json::json!({
            "location": location,
            "properties": {
                "publicIPAllocationMethod": "Static",
                "publicIPAddressVersion": "IPv4"
            },
            "sku": {
                "name": "Standard"
            }
        });

        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/publicIPAddresses/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, pip_name
        );

        let response = self
            .client
            .put(&url)
            .bearer_auth(token)
            .json(&pip_body)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create public IP: {e}")))?;

        if !response.status().is_success() {
            return Err(Error::ConfigurationError(
                "Failed to create public IP".into(),
            ));
        }

        Ok(format!(
            "/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/publicIPAddresses/{}",
            self.subscription_id, self.resource_group, pip_name
        ))
    }

    /// Wait for VM to be ready and get public IP
    async fn wait_for_vm(&self, vm_name: &str, token: &str) -> Result<String> {
        let mut attempts = 0;
        let max_attempts = 60;

        loop {
            if attempts >= max_attempts {
                return Err(Error::ConfigurationError("VM provisioning timeout".into()));
            }

            // Get VM status
            let url = format!(
                "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/instanceView?api-version=2023-09-01",
                self.subscription_id, self.resource_group, vm_name
            );

            let response = self.client.get(&url).bearer_auth(token).send().await;

            if let Ok(resp) = response {
                if resp.status().is_success() {
                    let json: serde_json::Value = resp.json().await.map_err(|e| {
                        Error::ConfigurationError(format!("Failed to parse response: {e}"))
                    })?;

                    if let Some(statuses) = json["statuses"].as_array() {
                        let is_running = statuses
                            .iter()
                            .any(|s| s["code"].as_str() == Some("PowerState/running"));

                        if is_running {
                            // Get public IP
                            let pip_name = format!("{vm_name}-nic-pip");
                            let pip_url = format!(
                                "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/publicIPAddresses/{}?api-version=2023-09-01",
                                self.subscription_id, self.resource_group, pip_name
                            );

                            let pip_response = self
                                .client
                                .get(&pip_url)
                                .bearer_auth(token)
                                .send()
                                .await
                                .map_err(|e| {
                                    Error::ConfigurationError(format!(
                                        "Failed to get public IP: {e}"
                                    ))
                                })?;

                            if pip_response.status().is_success() {
                                let pip_json: serde_json::Value =
                                    pip_response.json().await.map_err(|e| {
                                        Error::ConfigurationError(format!(
                                            "Failed to parse IP response: {e}"
                                        ))
                                    })?;

                                if let Some(ip) = pip_json["properties"]["ipAddress"].as_str() {
                                    return Ok(ip.to_string());
                                }
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            attempts += 1;
        }
    }

    /// Select VM size based on resource requirements
    fn select_vm_size(&self, spec: &ResourceSpec) -> &'static str {
        match (spec.cpu, spec.memory_gb, spec.gpu_count) {
            // GPU instances
            (_, _, Some(gpu)) if gpu >= 4 => "Standard_NC24ads_A100_v4",
            (_, _, Some(gpu)) if gpu >= 2 => "Standard_NC12s_v3",
            (_, _, Some(_)) => "Standard_NC6s_v3",

            // High memory
            (cpu, mem, _) if mem > cpu * 8.0 => {
                if mem <= 16.0 {
                    "Standard_E2as_v5"
                } else if mem <= 32.0 {
                    "Standard_E4as_v5"
                } else if mem <= 64.0 {
                    "Standard_E8as_v5"
                } else {
                    "Standard_E16as_v5"
                }
            }

            // High CPU
            (cpu, _, _) if cpu >= 16.0 => "Standard_F16s_v2",
            (cpu, _, _) if cpu >= 8.0 => "Standard_F8s_v2",
            (cpu, _, _) if cpu >= 4.0 => "Standard_F4s_v2",

            // Standard
            (cpu, mem, _) if cpu <= 2.0 && mem <= 8.0 => "Standard_B2ms",
            (cpu, mem, _) if cpu <= 4.0 && mem <= 16.0 => "Standard_B4ms",
            _ => "Standard_D4s_v5",
        }
    }

    /// Terminate an Azure VM
    pub async fn terminate_instance(&mut self, instance_id: &str) -> Result<()> {
        let token = self.get_access_token().await?;
        let vm_name = instance_id.split('/').next_back().unwrap_or(instance_id);

        // Delete VM
        let url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, vm_name
        );

        let response = self
            .client
            .delete(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to terminate VM: {e}")))?;

        if !response.status().is_success() && response.status() != 404 {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ConfigurationError(format!(
                "Failed to terminate VM: {error_text}"
            )));
        }

        // Clean up associated resources
        if let Err(e) = self.cleanup_vm_resources(vm_name, &token).await {
            warn!("Failed to cleanup VM resources for {}: {}", vm_name, e);
        }

        Ok(())
    }

    /// Clean up VM resources (NIC, public IP, disks)
    async fn cleanup_vm_resources(&self, vm_name: &str, token: &str) -> Result<()> {
        // Delete NIC
        let nic_name = format!("{vm_name}-nic");
        let nic_url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/networkInterfaces/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, nic_name
        );
        if let Err(e) = self.client.delete(&nic_url).bearer_auth(token).send().await {
            debug!("Failed to delete NIC (may not exist): {}", e);
        }

        // Delete public IP
        let pip_name = format!("{vm_name}-nic-pip");
        let pip_url = format!(
            "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Network/publicIPAddresses/{}?api-version=2023-09-01",
            self.subscription_id, self.resource_group, pip_name
        );
        if let Err(e) = self.client.delete(&pip_url).bearer_auth(token).send().await {
            debug!("Failed to delete public IP (may not exist): {}", e);
        }

        Ok(())
    }
}
