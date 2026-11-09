use super::ProviderErrorCode;
use crate::infra::traits::CloudProviderAdapter;
#[cfg(feature = "aws")]
use crate::providers::aws::AwsManagedAdapter as Aws;
use crate::providers::azure::AzureManagedAdapter as Azure;
use crate::providers::gcp::GcpManagedAdapter as Gcp;
use crate::{CloudProvider, CloudProvisioner, Error, Result};
use blueprint_std::collections::HashMap;
use serde::Deserialize;

/// Each tag selects its adapter's actual caller-owned constructor.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Deserialize, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
#[serde(tag = "provider", deny_unknown_fields)]
pub enum ProviderCredentials {
    #[serde(rename = "runpod")]
    RunPod {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = crate::providers::RunPodAdapter::API_KEY_ENV, "writeOnly" = true)))]
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
    },
    #[serde(rename = "vast-ai")]
    VastAi {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = crate::providers::VastAiAdapter::API_KEY_ENV, "writeOnly" = true)))]
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
    },
    #[serde(rename = "lambda-labs")]
    LambdaLabs {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = crate::providers::LambdaLabsAdapter::API_KEY_ENV, "writeOnly" = true)))]
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
        #[serde(default)]
        ssh_key_name: Option<String>,
        #[serde(default)]
        firewall_ruleset_ids: Vec<String>,
    },
    #[serde(rename = "bittensor-lium")]
    BittensorLium {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = crate::providers::BittensorLiumAdapter::API_KEY_ENV, "writeOnly" = true)))]
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
    },
    #[cfg(feature = "aws")]
    #[serde(rename = "aws")]
    Aws {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::ACCESS_KEY_ID_ENV, "writeOnly" = true)))]
        access_key_id: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::SECRET_ACCESS_KEY_ENV, "writeOnly" = true)))]
        secret_access_key: String,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::SESSION_TOKEN_ENV, "writeOnly" = true)))]
        session_token: Option<String>,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::REGION_ENV)))]
        region: String,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::SUBNET_ID_ENV)))]
        subnet_id: Option<String>,
        /// Comma-separated.
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::SECURITY_GROUP_IDS_ENV)))]
        security_group_ids: Option<String>,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Aws::AMI_ID_ENV)))]
        ami_id: Option<String>,
        #[serde(default)]
        endpoint: Option<String>,
    },
    #[serde(rename = "azure")]
    Azure {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::TENANT_ID_ENV)))]
        tenant_id: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::CLIENT_ID_ENV)))]
        client_id: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::CLIENT_SECRET_ENV, "writeOnly" = true)))]
        client_secret: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::SUBSCRIPTION_ID_ENV)))]
        subscription_id: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::RESOURCE_GROUP_ENV)))]
        resource_group: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::SUBNET_ID_ENV)))]
        subnet_id: String,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::IMAGE_ENV)))]
        image: Option<String>,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Azure::REGION_ENV)))]
        region: Option<String>,
    },
    #[serde(rename = "gcp")]
    Gcp {
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Gcp::PROJECT_ID_ENV)))]
        project_id: String,
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Gcp::SERVICE_ACCOUNT_KEY_ENV, "writeOnly" = true)))]
        service_account_key: String,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Gcp::IMAGE_ENV)))]
        image: Option<String>,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Gcp::BILLING_API_KEY_ENV, "writeOnly" = true)))]
        billing_api_key: Option<String>,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        #[cfg_attr(feature = "schema", schemars(extend("x-env" = Gcp::REGION_ENV)))]
        region: Option<String>,
    },
}

impl core::fmt::Debug for ProviderCredentials {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ProviderCredentials([REDACTED])")
    }
}

impl CloudProvisioner {
    /// Add checked native accounts to an allocator with other (e.g. CPU) adapters.
    pub fn with_credentials(mut self, credentials: Vec<ProviderCredentials>) -> Result<Self> {
        let native = Self::from_credentials(credentials)?;
        if native
            .providers
            .keys()
            .any(|provider| self.providers.contains_key(provider))
        {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        self.providers.extend(native.providers);
        self.account_bindings.extend(native.account_bindings);
        self.regions.extend(native.regions);
        self.account_regions.extend(native.account_regions);
        Ok(self)
    }

    pub fn from_credentials(credentials: Vec<ProviderCredentials>) -> Result<Self> {
        use crate::providers::{
            BittensorLiumAdapter, LambdaLabsAdapter, RunPodAdapter, VastAiAdapter,
        };
        let mut clients: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>> = HashMap::new();
        let mut regions = HashMap::new();
        let mut bindings = HashMap::new();
        for credential in &credentials {
            let (provider, api_key, endpoint, region) = match credential {
                ProviderCredentials::RunPod {
                    api_key,
                    endpoint,
                    region,
                } => (CloudProvider::RunPod, api_key, endpoint, region.as_ref()),
                ProviderCredentials::VastAi {
                    api_key,
                    endpoint,
                    region,
                } => (CloudProvider::VastAi, api_key, endpoint, region.as_ref()),
                ProviderCredentials::LambdaLabs {
                    api_key,
                    endpoint,
                    region,
                    ..
                } => (
                    CloudProvider::LambdaLabs,
                    api_key,
                    endpoint,
                    region.as_ref(),
                ),
                ProviderCredentials::BittensorLium {
                    api_key,
                    endpoint,
                    region,
                } => (
                    CloudProvider::BittensorLium,
                    api_key,
                    endpoint,
                    region.as_ref(),
                ),
                ProviderCredentials::Azure {
                    client_secret,
                    endpoint,
                    region,
                    ..
                } => (
                    CloudProvider::Azure,
                    client_secret,
                    endpoint,
                    region.as_ref(),
                ),
                ProviderCredentials::Gcp {
                    service_account_key,
                    endpoint,
                    region,
                    ..
                } => (
                    CloudProvider::GCP,
                    service_account_key,
                    endpoint,
                    region.as_ref(),
                ),
                #[cfg(feature = "aws")]
                ProviderCredentials::Aws {
                    secret_access_key,
                    endpoint,
                    region,
                    ..
                } => (
                    CloudProvider::AWS,
                    secret_access_key,
                    endpoint,
                    Some(region),
                ),
            };
            // A service-account key is multi-line JSON that its own parser checks.
            let json = matches!(credential, ProviderCredentials::Gcp { .. });
            if api_key.trim().is_empty()
                || (!json && api_key.chars().any(char::is_control))
                || clients.contains_key(&provider)
            {
                return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
            }
            macro_rules! adapter {
                ($adapter:ty) => {
                    match endpoint {
                        Some(url) => <$adapter>::with_endpoint(api_key.clone(), url)?,
                        None => <$adapter>::with_credentials(api_key.clone())?,
                    }
                };
            }
            macro_rules! bound {
                ($client:expr) => {{
                    let client = $client;
                    fn managed<T: super::ManagedProviderAdapter>(_: &T) {}
                    managed(&client);
                    bindings.insert(provider.clone(), client.credential_binding()?);
                    Box::new(client) as Box<dyn CloudProviderAdapter>
                }};
            }
            let client: Box<dyn CloudProviderAdapter> = match credential {
                ProviderCredentials::RunPod { .. } => bound!(adapter!(RunPodAdapter)),
                ProviderCredentials::VastAi { .. } => bound!(adapter!(VastAiAdapter)),
                ProviderCredentials::BittensorLium { .. } => {
                    bound!(adapter!(BittensorLiumAdapter))
                }
                ProviderCredentials::LambdaLabs {
                    ssh_key_name,
                    firewall_ruleset_ids,
                    ..
                } => {
                    let mut adapter = adapter!(LambdaLabsAdapter)
                        .with_firewall_rulesets(firewall_ruleset_ids.clone());
                    if let Some(name) = ssh_key_name {
                        adapter = adapter.with_ssh_key_name(name.clone());
                    }
                    bound!(adapter)
                }
                ProviderCredentials::Azure {
                    tenant_id,
                    client_id,
                    client_secret,
                    subscription_id,
                    resource_group,
                    subnet_id,
                    image,
                    ..
                } => bound!(Azure::with_endpoint(
                    crate::providers::azure::AzureAccount {
                        tenant_id: tenant_id.clone(),
                        client_id: client_id.clone(),
                        client_secret: client_secret.clone(),
                        subscription_id: subscription_id.clone(),
                        resource_group: resource_group.clone(),
                        subnet_id: subnet_id.clone(),
                        image: image.clone(),
                    },
                    endpoint.as_deref(),
                )?),
                ProviderCredentials::Gcp {
                    project_id,
                    image,
                    billing_api_key,
                    ..
                } => bound!(Gcp::with_endpoint(
                    project_id.clone(),
                    api_key,
                    image.clone(),
                    billing_api_key.clone(),
                    endpoint.as_deref(),
                )?),
                #[cfg(feature = "aws")]
                ProviderCredentials::Aws {
                    access_key_id,
                    secret_access_key,
                    session_token,
                    region,
                    subnet_id,
                    security_group_ids,
                    ami_id,
                    ..
                } => bound!(Aws::with_endpoint(
                    crate::providers::aws::AwsAccount {
                        access_key_id: access_key_id.clone(),
                        secret_access_key: secret_access_key.clone(),
                        session_token: session_token.clone(),
                        region: region.clone(),
                        subnet_id: subnet_id.clone(),
                        security_group_ids: security_group_ids
                            .iter()
                            .flat_map(|ids| ids.split(','))
                            .map(|id| id.trim().to_owned())
                            .filter(|id| !id.is_empty())
                            .collect(),
                        ami_id: ami_id.clone(),
                    },
                    endpoint.as_deref(),
                )?),
            };
            if let Some(region) = region {
                regions.insert(provider.clone(), region.clone());
            }
            clients.insert(provider, client);
        }
        let mut provisioner = Self::with_providers(clients).with_regions(regions);
        provisioner.account_bindings = bindings;
        Ok(provisioner)
    }
}
