//! Cloud commands for Blueprint services.
//!
//! These commands configure providers, quote live prices, and inspect and terminate deployments
//! recorded in the deployment tracker. None of them allocates capacity.
//!
//! # Examples
//!
//! Configure a cloud provider:
//! ```bash
//! cargo tangle cloud configure aws --region us-east-1 --set-default
//! ```
//!
//! Configure deployment policy:
//! ```bash
//! cargo tangle cloud policy --gpu-providers gcp,aws --cost-providers vultr,do
//! ```
//!
//! Configure serverless FaaS settings:
//! ```bash
//! cargo tangle cloud policy --serverless true --faas-provider aws-lambda
//! cargo tangle cloud policy --faas-memory 1024 --faas-timeout 600
//! ```
//!
//! View current policy:
//! ```bash
//! cargo tangle cloud show
//! ```

use blueprint_remote_providers::DeploymentTracker;
use clap::Subcommand;
use color_eyre::{
    Result,
    eyre::{WrapErr, eyre},
};
use std::path::PathBuf;

mod config;
mod estimate;
mod policy;
mod preflight;
mod status;

pub use config::CloudProvider;
pub use policy::{CostOptimization, RemoteDeploymentPolicy};

#[derive(Subcommand, Debug)]
pub enum CloudCommands {
    /// Set up cloud provider access
    #[command(visible_alias = "cfg")]
    Configure {
        /// Cloud provider (aws, gcp, azure, digitalocean, vultr)
        provider: CloudProvider,

        /// Default region for this provider
        #[arg(short, long)]
        region: Option<String>,

        /// Make this the default provider
        #[arg(short = 'd', long)]
        set_default: bool,
    },

    /// Configure remote deployment policy
    #[command(visible_alias = "policy")]
    ConfigurePolicy {
        #[command(flatten)]
        args: policy::PolicyConfigureArgs,
    },

    /// Show current deployment policy
    #[command(visible_alias = "show")]
    ShowPolicy,

    /// Estimate deployment costs
    #[command(visible_alias = "cost")]
    Estimate(estimate::EstimateOptions),

    /// Check deployment status
    #[command(visible_alias = "s")]
    Status {
        /// Deployment or service ID (shows all if not specified)
        service_id: Option<String>,

        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
    },

    /// Terminate tracked deployments through their provider
    #[command(visible_alias = "term")]
    Terminate {
        /// Deployment or service ID to terminate
        #[arg(required_unless_present = "all")]
        service_id: Option<String>,

        /// Terminate every tracked deployment
        #[arg(long, conflicts_with = "service_id")]
        all: bool,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// List configured providers
    #[command(visible_alias = "ls")]
    List,

    /// Validate cloud/TEE readiness and emit bootstrap env settings
    #[command(visible_alias = "check")]
    Preflight {
        /// Provider to validate (defaults to configured default provider)
        #[arg(short, long)]
        provider: Option<CloudProvider>,

        /// Require TEE readiness checks
        #[arg(long)]
        tee_required: bool,

        /// Print manager bootstrap environment variables
        #[arg(long)]
        bootstrap_env: bool,

        /// Write bootstrap env output to file
        #[arg(long)]
        write_env_file: Option<PathBuf>,
    },
}

/// Open the deployment tracker at the manager's path; a missing one tracks nothing.
async fn open_tracker() -> Result<DeploymentTracker> {
    let path = std::env::var_os("TANGLE_DEPLOYMENT_TRACKER_PATH")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".tangle/remote_deployments")))
        .ok_or_else(|| eyre!("Could not find the home directory"))?;
    DeploymentTracker::new(&path)
        .await
        .wrap_err("Failed to read the deployment tracker")
}

/// Execute cloud commands.
///
/// # Arguments
///
/// * `command` - The cloud subcommand to execute
///
/// # Errors
///
/// Returns an error if:
/// * Provider configuration fails
/// * Deployment fails
/// * Cost estimation encounters invalid parameters
/// * Status check fails to connect
/// * Termination is rejected or fails
pub async fn execute(command: CloudCommands) -> Result<()> {
    match command {
        CloudCommands::Configure {
            provider,
            region,
            set_default,
        } => config::configure(provider, region, set_default).await,

        CloudCommands::ConfigurePolicy { args } => policy::configure_policy(args).await,

        CloudCommands::ShowPolicy => policy::show_policy().await,

        CloudCommands::Estimate(options) => estimate::estimate(options).await,

        CloudCommands::Status { service_id, watch } => status::show_status(service_id, watch).await,

        // Clap requires `service_id` unless `--all`, so no ID here means every deployment.
        CloudCommands::Terminate {
            service_id,
            all: _,
            yes,
        } => status::terminate(service_id, yes).await,

        CloudCommands::List => config::list_providers().await,

        CloudCommands::Preflight {
            provider,
            tee_required,
            bootstrap_env,
            write_env_file,
        } => preflight::run(provider, tee_required, bootstrap_env, write_env_file).await,
    }
}
