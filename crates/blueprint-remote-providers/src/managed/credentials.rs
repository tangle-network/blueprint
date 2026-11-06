use super::ProviderErrorCode;
use crate::infra::traits::CloudProviderAdapter;
use crate::{CloudProvider, CloudProvisioner, Error, Result};
use blueprint_std::collections::HashMap;
use serde::Deserialize;

/// Each tag selects its adapter's actual caller-owned constructor.
#[derive(schemars::JsonSchema, Deserialize, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
#[serde(tag = "provider", deny_unknown_fields)]
pub enum ProviderCredentials {
    #[serde(rename = "runpod")]
    RunPod {
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
    },
    #[serde(rename = "vast-ai")]
    VastAi {
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
    },
    #[serde(rename = "lambda-labs")]
    LambdaLabs {
        api_key: String,
        #[serde(default)]
        endpoint: Option<String>,
        #[serde(default)]
        region: Option<String>,
        #[serde(default)]
        ssh_key_name: Option<String>,
    },
    #[serde(rename = "bittensor-lium")]
    BittensorLium {
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
    pub fn from_credentials(credentials: Vec<ProviderCredentials>) -> Result<Self> {
        use crate::providers::{
            BittensorLiumAdapter, LambdaLabsAdapter, RunPodAdapter, VastAiAdapter,
        };
        let mut clients: HashMap<CloudProvider, Box<dyn CloudProviderAdapter>> = HashMap::new();
        let mut regions = HashMap::new();
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
            let client: Box<dyn CloudProviderAdapter> = match credential {
                ProviderCredentials::RunPod { .. } => Box::new(adapter!(RunPodAdapter)),
                ProviderCredentials::VastAi { .. } => Box::new(adapter!(VastAiAdapter)),
                ProviderCredentials::BittensorLium { .. } => {
                    Box::new(adapter!(BittensorLiumAdapter))
                }
                ProviderCredentials::LambdaLabs { ssh_key_name, .. } => {
                    let mut adapter = adapter!(LambdaLabsAdapter);
                    if let Some(name) = ssh_key_name {
                        adapter = adapter.with_ssh_key_name(name.clone());
                    }
                    Box::new(adapter)
                }
            };
            if let Some(region) = region {
                regions.insert(provider.clone(), region.clone());
            }
            clients.insert(provider, client);
        }
        Ok(Self::with_providers(clients).with_regions(regions))
    }
}
