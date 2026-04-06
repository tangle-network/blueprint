//! Cloud provider configuration types

use serde::{Deserialize, Serialize};

/// Cloud provider configuration for all supported providers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudConfig {
    pub enabled: bool,
    pub aws: Option<AwsConfig>,
    pub gcp: Option<GcpConfig>,
    pub azure: Option<AzureConfig>,
    pub digital_ocean: Option<DigitalOceanConfig>,
    pub vultr: Option<VultrConfig>,
    pub lambda_labs: Option<LambdaLabsConfig>,
    pub runpod: Option<RunPodConfig>,
    pub vast_ai: Option<VastAiConfig>,
    pub coreweave: Option<CoreWeaveConfig>,
    pub paperspace: Option<PaperspaceConfig>,
    pub fluidstack: Option<FluidstackConfig>,
    pub tensordock: Option<TensorDockConfig>,
    pub akash: Option<AkashConfig>,
    pub io_net: Option<IoNetConfig>,
    pub prime_intellect: Option<PrimeIntellectConfig>,
    pub render: Option<RenderConfig>,
    pub bittensor_lium: Option<BittensorLiumConfig>,
}

/// AWS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub enabled: bool,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
    pub priority: Option<u8>,
}

/// GCP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpConfig {
    pub enabled: bool,
    pub region: String,
    pub project_id: String,
    pub service_account_path: String,
    pub priority: Option<u8>,
}

/// Azure configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    pub enabled: bool,
    pub region: String,
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: String,
    pub priority: Option<u8>,
}

/// DigitalOcean configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalOceanConfig {
    pub enabled: bool,
    pub region: String,
    pub api_token: String,
    pub priority: Option<u8>,
}

/// Vultr configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VultrConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub priority: Option<u8>,
}

/// Lambda Labs configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaLabsConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub ssh_key_name: Option<String>,
    pub priority: Option<u8>,
}

/// RunPod configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunPodConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub cloud_type: Option<String>,
    pub priority: Option<u8>,
}

/// Vast.ai configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VastAiConfig {
    pub enabled: bool,
    pub api_key: String,
    pub max_price_per_hour: Option<f64>,
    pub min_reliability: Option<f64>,
    pub priority: Option<u8>,
}

/// CoreWeave configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreWeaveConfig {
    pub enabled: bool,
    pub region: String,
    pub token: String,
    pub namespace: Option<String>,
    pub priority: Option<u8>,
}

/// Paperspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperspaceConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub priority: Option<u8>,
}

/// Fluidstack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidstackConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub priority: Option<u8>,
}

/// TensorDock configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorDockConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub api_token: String,
    pub priority: Option<u8>,
}

/// Akash configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AkashConfig {
    pub enabled: bool,
    pub rpc_url: String,
    pub wallet_mnemonic: String,
    pub chain_id: Option<String>,
    pub priority: Option<u8>,
}

/// io.net configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoNetConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub cluster_type: Option<String>,
    pub priority: Option<u8>,
}

/// Prime Intellect configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeIntellectConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub priority: Option<u8>,
}

/// Render configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub enabled: bool,
    pub region: String,
    pub api_key: String,
    pub priority: Option<u8>,
}

/// Bittensor/Lium configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BittensorLiumConfig {
    pub enabled: bool,
    pub api_key: String,
    pub wallet_hotkey: Option<String>,
    pub wallet_coldkey: Option<String>,
    pub priority: Option<u8>,
}

/// Try to load a provider config from env. If `primary_env` is set, calls `builder`
/// with the value, assigns the result to `field`, and sets `any_enabled`.
fn load_provider<T>(
    field: &mut Option<T>,
    primary_env: &str,
    any_enabled: &mut bool,
    builder: impl FnOnce(String) -> T,
) {
    if let Ok(val) = std::env::var(primary_env) {
        *field = Some(builder(val));
        *any_enabled = true;
    }
}

/// Same as `load_provider` but requires two env vars to both be present.
fn load_provider2<T>(
    field: &mut Option<T>,
    env_a: &str,
    env_b: &str,
    any_enabled: &mut bool,
    builder: impl FnOnce(String, String) -> T,
) {
    if let (Ok(a), Ok(b)) = (std::env::var(env_a), std::env::var(env_b)) {
        *field = Some(builder(a, b));
        *any_enabled = true;
    }
}

/// Shorthand: read an env var with a default fallback.
fn env_or(var: &str, default: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}

impl CloudConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Option<Self> {
        use std::env;

        let mut c = CloudConfig::default();
        let mut any = false;

        load_provider2(
            &mut c.aws,
            "AWS_ACCESS_KEY_ID",
            "AWS_SECRET_ACCESS_KEY",
            &mut any,
            |key, secret| AwsConfig {
                enabled: true,
                region: env_or("AWS_DEFAULT_REGION", "us-east-1"),
                access_key: key,
                secret_key: secret,
                priority: Some(10),
            },
        );

        load_provider(&mut c.gcp, "GCP_PROJECT_ID", &mut any, |project_id| {
            GcpConfig {
                enabled: true,
                region: env_or("GCP_DEFAULT_REGION", "us-central1"),
                project_id,
                service_account_path: env_or(
                    "GOOGLE_APPLICATION_CREDENTIALS",
                    "/etc/gcp/service-account.json",
                ),
                priority: Some(8),
            }
        });

        if let (Ok(client_id), Ok(client_secret), Ok(tenant_id)) = (
            env::var("AZURE_CLIENT_ID"),
            env::var("AZURE_CLIENT_SECRET"),
            env::var("AZURE_TENANT_ID"),
        ) {
            c.azure = Some(AzureConfig {
                enabled: true,
                region: env_or("AZURE_DEFAULT_REGION", "East US"),
                client_id,
                client_secret,
                tenant_id,
                priority: Some(7),
            });
            any = true;
        }

        load_provider(&mut c.digital_ocean, "DO_API_TOKEN", &mut any, |token| {
            DigitalOceanConfig {
                enabled: true,
                region: env_or("DO_DEFAULT_REGION", "nyc3"),
                api_token: token,
                priority: Some(5),
            }
        });

        load_provider(&mut c.vultr, "VULTR_API_KEY", &mut any, |key| VultrConfig {
            enabled: true,
            region: env_or("VULTR_DEFAULT_REGION", "ewr"),
            api_key: key,
            priority: Some(3),
        });

        load_provider(&mut c.lambda_labs, "LAMBDA_LABS_API_KEY", &mut any, |key| {
            LambdaLabsConfig {
                enabled: true,
                region: env_or("LAMBDA_LABS_REGION", "us-west-1"),
                api_key: key,
                ssh_key_name: env::var("LAMBDA_LABS_SSH_KEY_NAME").ok(),
                priority: Some(6),
            }
        });

        load_provider(&mut c.runpod, "RUNPOD_API_KEY", &mut any, |key| {
            RunPodConfig {
                enabled: true,
                region: env_or("RUNPOD_REGION", "US"),
                api_key: key,
                cloud_type: env::var("RUNPOD_CLOUD_TYPE").ok(),
                priority: Some(6),
            }
        });

        load_provider(&mut c.vast_ai, "VAST_AI_API_KEY", &mut any, |key| {
            VastAiConfig {
                enabled: true,
                api_key: key,
                max_price_per_hour: env::var("VAST_AI_MAX_PRICE_PER_HOUR")
                    .ok()
                    .and_then(|v| v.parse().ok()),
                min_reliability: env::var("VAST_AI_MIN_RELIABILITY")
                    .ok()
                    .and_then(|v| v.parse().ok()),
                priority: Some(4),
            }
        });

        load_provider(&mut c.coreweave, "COREWEAVE_TOKEN", &mut any, |token| {
            CoreWeaveConfig {
                enabled: true,
                region: env_or("COREWEAVE_REGION", "ORD1"),
                token,
                namespace: env::var("COREWEAVE_NAMESPACE").ok(),
                priority: Some(7),
            }
        });

        load_provider(&mut c.paperspace, "PAPERSPACE_API_KEY", &mut any, |key| {
            PaperspaceConfig {
                enabled: true,
                region: env_or("PAPERSPACE_REGION", "NY2"),
                api_key: key,
                priority: Some(5),
            }
        });

        load_provider(&mut c.fluidstack, "FLUIDSTACK_API_KEY", &mut any, |key| {
            FluidstackConfig {
                enabled: true,
                region: env_or("FLUIDSTACK_REGION", "us-east"),
                api_key: key,
                priority: Some(4),
            }
        });

        load_provider2(
            &mut c.tensordock,
            "TENSORDOCK_API_KEY",
            "TENSORDOCK_API_TOKEN",
            &mut any,
            |key, token| TensorDockConfig {
                enabled: true,
                region: env_or("TENSORDOCK_REGION", "us-central"),
                api_key: key,
                api_token: token,
                priority: Some(4),
            },
        );

        load_provider2(
            &mut c.akash,
            "AKASH_RPC_URL",
            "AKASH_WALLET_MNEMONIC",
            &mut any,
            |rpc_url, mnemonic| AkashConfig {
                enabled: true,
                rpc_url,
                wallet_mnemonic: mnemonic,
                chain_id: env::var("AKASH_CHAIN_ID").ok(),
                priority: Some(3),
            },
        );

        load_provider(&mut c.io_net, "IO_NET_API_KEY", &mut any, |key| {
            IoNetConfig {
                enabled: true,
                region: env_or("IO_NET_REGION", "us-east"),
                api_key: key,
                cluster_type: env::var("IO_NET_CLUSTER_TYPE").ok(),
                priority: Some(4),
            }
        });

        load_provider(
            &mut c.prime_intellect,
            "PRIME_INTELLECT_API_KEY",
            &mut any,
            |key| PrimeIntellectConfig {
                enabled: true,
                region: env_or("PRIME_INTELLECT_REGION", "us-east"),
                api_key: key,
                priority: Some(5),
            },
        );

        load_provider(&mut c.render, "RENDER_API_KEY", &mut any, |key| {
            RenderConfig {
                enabled: true,
                region: env_or("RENDER_REGION", "oregon"),
                api_key: key,
                priority: Some(3),
            }
        });

        load_provider(&mut c.bittensor_lium, "LIUM_API_KEY", &mut any, |key| {
            BittensorLiumConfig {
                enabled: true,
                api_key: key,
                wallet_hotkey: env::var("LIUM_WALLET_HOTKEY").ok(),
                wallet_coldkey: env::var("LIUM_WALLET_COLDKEY").ok(),
                priority: Some(2),
            }
        });

        if any {
            c.enabled = true;
            Some(c)
        } else {
            None
        }
    }
}
