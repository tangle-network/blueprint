//! Serverless deployment for pure-FaaS blueprints.
//!
//! This module handles deployment of blueprints where all jobs run on FaaS platforms,
//! eliminating the need for a full VM by using a minimal orchestrator.

use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::service::Service;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_std::collections::HashMap;
use blueprint_std::path::Path;
use tracing::{info, warn};

/// Serverless deployment configuration.
#[derive(Debug, Clone)]
pub struct ServerlessConfig {
    /// FaaS provider to use
    pub provider: FaasProviderConfig,
    /// Default memory allocation (MB)
    pub default_memory_mb: u32,
    /// Default timeout (seconds)
    pub default_timeout_secs: u32,
    /// Whether to fallback to VM if deployment fails
    pub fallback_to_vm: bool,
}

/// FaaS provider configuration.
#[derive(Debug, Clone)]
pub enum FaasProviderConfig {
    AwsLambda { region: String },
    GcpFunctions { project_id: String },
    AzureFunctions { subscription_id: String },
    Custom { endpoint: String },
}

/// Deploy a blueprint in serverless mode.
///
/// This creates a lightweight orchestrator and optionally deploys jobs to FaaS.
///
/// Note: Custom FaaS endpoints don't support auto-deployment - jobs must be
/// deployed manually and configured via policy.
pub async fn deploy_serverless(
    ctx: &BlueprintManagerContext,
    service_name: &str,
    binary_path: &Path,
    env_vars: BlueprintEnvVars,
    arguments: BlueprintArgs,
    job_ids: Vec<u32>,
    config: &ServerlessConfig,
) -> Result<Service> {
    info!(
        "Deploying service '{}' in serverless mode with {} jobs",
        service_name,
        job_ids.len()
    );

    info!("FaaS provider: {:?}", config.provider);
    info!("Jobs to deploy: {:?}", job_ids);

    // Step 1: Deploy orchestrator (lightweight runner)
    let orchestrator_endpoint = deploy_orchestrator(
        ctx,
        service_name,
        binary_path,
        &env_vars,
        &arguments,
        &job_ids,
        config,
    )
    .await?;

    // Step 2: For cloud providers (AWS/GCP/Azure), attempt auto-deployment
    // For custom, user must deploy manually
    match &config.provider {
        FaasProviderConfig::AwsLambda { .. }
        | FaasProviderConfig::GcpFunctions { .. }
        | FaasProviderConfig::AzureFunctions { .. } => {
            for job_id in &job_ids {
                deploy_job_to_faas(ctx, binary_path, *job_id, config).await?;
            }
        }
        FaasProviderConfig::Custom { .. } => {
            info!("Custom FaaS: skipping auto-deployment (deploy jobs manually)");
        }
    }

    // Step 3: Create service handle
    Ok(Service {
        name: service_name.to_string(),
        // TODO: populate with actual deployment info
        ..Default::default()
    })
}

/// Deploy the minimal orchestrator.
///
/// The orchestrator is a lightweight BlueprintRunner that:
/// 1. Subscribes to Tangle events
/// 2. Invokes FaaS functions for each job
/// 3. Submits results back to Tangle
///
/// For serverless deployments, we use a tiny instance (t4g.nano ~ $3/month)
/// instead of a full VM, since the runner only orchestrates FaaS calls.
async fn deploy_orchestrator(
    ctx: &BlueprintManagerContext,
    service_name: &str,
    binary_path: &Path,
    env_vars: &BlueprintEnvVars,
    arguments: &BlueprintArgs,
    job_ids: &[u32],
    config: &ServerlessConfig,
) -> Result<String> {
    info!("Deploying serverless orchestrator for '{}'", service_name);

    // The orchestrator is just the BlueprintRunner binary, but configured
    // to delegate all jobs to FaaS executors.
    //
    // We could deploy it to:
    // 1. t4g.nano EC2 instance (cheapest, ~$3/month)
    // 2. Cloud Run (pay-per-request)
    // 3. Lambda (polling mode)
    //
    // For MVP, we'll just return a note that the operator should run it locally
    // or deploy via remote-providers with tiny resources.

    info!(
        "Orchestrator deployment: operator should run BlueprintRunner locally or on t4g.nano"
    );
    info!("Configure FaaS executors via runner config for jobs: {:?}", job_ids);

    Ok("local-or-t4g-nano".to_string())
}

/// Deploy a single job to the FaaS platform using the factory pattern.
#[cfg(all(
    feature = "blueprint-faas",
    any(feature = "aws", feature = "gcp", feature = "azure", feature = "custom")
))]
async fn deploy_job_to_faas(
    _ctx: &BlueprintManagerContext,
    binary_path: &Path,
    job_id: u32,
    config: &ServerlessConfig,
) -> Result<()> {
    use blueprint_faas::factory;

    info!("Deploying job {} to FaaS via factory", job_id);

    // Read the binary (this should be the faas_handler or blueprint binary)
    let binary = std::fs::read(binary_path).map_err(|e| {
        Error::Other(format!("Failed to read binary at {}: {}", binary_path.display(), e))
    })?;

    // Convert manager's config to factory config
    let provider_config = convert_to_factory_config(config)?;

    // Use the factory to deploy
    let deployment = factory::deploy_job(provider_config, job_id, &binary)
        .await
        .map_err(|e| Error::Other(format!("FaaS deployment failed: {}", e)))?;

    info!(
        "Successfully deployed job {} to {}: {}",
        job_id,
        deployment.function_id,
        deployment.endpoint
    );

    Ok(())
}

#[cfg(not(all(
    feature = "blueprint-faas",
    any(feature = "aws", feature = "gcp", feature = "azure", feature = "custom")
)))]
async fn deploy_job_to_faas(
    _ctx: &BlueprintManagerContext,
    _binary_path: &Path,
    job_id: u32,
    _config: &ServerlessConfig,
) -> Result<()> {
    warn!(
        "FaaS deployment requested for job {} but required features not enabled",
        job_id
    );
    warn!("Enable blueprint-faas with at least one provider feature (aws/gcp/azure/custom)");
    Ok(())
}

/// Convert manager's ServerlessConfig to factory's FaasProviderConfig
#[cfg(all(
    feature = "blueprint-faas",
    any(feature = "aws", feature = "gcp", feature = "azure", feature = "custom")
))]
fn convert_to_factory_config(
    config: &ServerlessConfig,
) -> Result<blueprint_faas::factory::FaasProviderConfig> {
    use blueprint_faas::factory::FaasProvider;

    let provider = match &config.provider {
        #[cfg(feature = "aws")]
        FaasProviderConfig::AwsLambda { region } => {
            let role_arn = std::env::var("AWS_LAMBDA_ROLE_ARN").unwrap_or_else(|_| {
                warn!("AWS_LAMBDA_ROLE_ARN not set, using default role");
                "arn:aws:iam::000000000000:role/blueprint-lambda-execution".to_string()
            });
            FaasProvider::AwsLambda {
                region: region.clone(),
                role_arn,
            }
        }
        #[cfg(feature = "gcp")]
        FaasProviderConfig::GcpFunctions { project_id } => {
            let region = std::env::var("GCP_REGION").unwrap_or_else(|_| "us-central1".to_string());
            FaasProvider::GcpFunctions {
                project_id: project_id.clone(),
                region,
            }
        }
        #[cfg(feature = "azure")]
        FaasProviderConfig::AzureFunctions { subscription_id } => {
            let region = std::env::var("AZURE_REGION").unwrap_or_else(|_| "eastus".to_string());
            FaasProvider::AzureFunctions {
                subscription_id: subscription_id.clone(),
                region,
            }
        }
        #[cfg(feature = "custom")]
        FaasProviderConfig::Custom { endpoint } => FaasProvider::Custom {
            endpoint: endpoint.clone(),
        },
        #[allow(unreachable_patterns)]
        _ => {
            return Err(Error::Other(
                "Provider not supported with current feature flags".to_string(),
            ))
        }
    };

    Ok(blueprint_faas::factory::FaasProviderConfig {
        provider,
        default_memory_mb: config.default_memory_mb,
        default_timeout_secs: config.default_timeout_secs,
    })
}

