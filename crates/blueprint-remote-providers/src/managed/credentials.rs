use super::ProviderErrorCode;
use crate::infra::traits::CloudProviderAdapter;
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
                } => (CloudProvider::RunPod, api_key, endpoint, region),
                ProviderCredentials::VastAi {
                    api_key,
                    endpoint,
                    region,
                } => (CloudProvider::VastAi, api_key, endpoint, region),
                ProviderCredentials::LambdaLabs {
                    api_key,
                    endpoint,
                    region,
                    ..
                } => (CloudProvider::LambdaLabs, api_key, endpoint, region),
                ProviderCredentials::BittensorLium {
                    api_key,
                    endpoint,
                    region,
                } => (CloudProvider::BittensorLium, api_key, endpoint, region),
            };
            if api_key.trim().is_empty()
                || api_key.chars().any(char::is_control)
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
