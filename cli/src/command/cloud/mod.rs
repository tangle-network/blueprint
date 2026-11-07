//! Cloud commands for Blueprint services.
//!
//! These commands configure providers, quote live prices, and inspect and terminate deployments
//! recorded in the deployment tracker. Managed JSON verbs share the SDK rental lifecycle.
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
mod json;
mod schema;
pub use schema::{MAX_JSON_BYTES, SCHEMA_VERSION};
mod estimate;
mod policy;
mod preflight;
mod status;

pub use config::CloudProvider;
pub use policy::{CostOptimization, RemoteDeploymentPolicy};

#[derive(Subcommand, Debug)]
pub enum CloudCommands {
    /// Emit the offline catalog and generated process contract.
    Catalog {
        #[arg(long, required = true)]
        json: bool,
    },
    /// Return all SDK-ranked live quotes and provider failures.
    Quote {
        #[arg(long, required = true)]
        json: bool,
    },
    /// Create from an exact quote with existing durable authorization state.
    Create {
        #[arg(long, required = true)]
        json: bool,
    },
    /// Recover an original lease without requesting capacity.
    Recover {
        #[arg(long, required = true)]
        json: bool,
    },
    /// Enroll deliberately new caller-owned private state; never repairs lost state.
    StateInit {
        #[arg(long, required = true)]
        json: bool,
    },
    /// Destroy only the exact managed identity and confirm absence.
    Destroy {
        #[arg(long, required = true)]
        json: bool,
    },

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
        /// Read a managed provider ID from JSON stdin instead of the tracker.
        #[arg(long, conflicts_with_all = ["service_id", "watch"])]
        json: bool,
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
    List {
        /// List namespace-scoped managed resources from JSON stdin.
        #[arg(long)]
        json: bool,
    },

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

impl CloudCommands {
    /// JSON calls never initialize tracing with inherited process environment.
    pub fn is_json(&self) -> bool {
        matches!(
            self,
            Self::Catalog { .. }
                | Self::Quote { .. }
                | Self::Create { .. }
                | Self::Recover { .. }
                | Self::StateInit { .. }
                | Self::Destroy { .. }
                | Self::Status { json: true, .. }
                | Self::List { json: true }
        )
    }
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
        CloudCommands::Catalog { .. } => json::execute(json::Operation::Catalog).await,
        CloudCommands::Quote { .. } => json::execute(json::Operation::Quote).await,
        CloudCommands::Create { .. } => json::execute(json::Operation::Create).await,
        CloudCommands::Recover { .. } => json::execute(json::Operation::Recover).await,
        CloudCommands::StateInit { .. } => json::execute(json::Operation::StateInit).await,
        CloudCommands::Destroy { .. } => json::execute(json::Operation::Destroy).await,
        CloudCommands::Status { json: true, .. } => json::execute(json::Operation::Status).await,
        CloudCommands::List { json: true } => json::execute(json::Operation::List).await,
        CloudCommands::Configure {
            provider,
            region,
            set_default,
        } => config::configure(provider, region, set_default).await,

        CloudCommands::ConfigurePolicy { args } => policy::configure_policy(args).await,

        CloudCommands::ShowPolicy => policy::show_policy().await,

        CloudCommands::Estimate(options) => estimate::estimate(options).await,

        CloudCommands::Status {
            service_id,
            watch,
            json: false,
        } => status::show_status(service_id, watch).await,

        // Clap requires `service_id` unless `--all`, so no ID here means every deployment.
        CloudCommands::Terminate {
            service_id,
            all: _,
            yes,
        } => status::terminate(service_id, yes).await,

        CloudCommands::List { json: false } => config::list_providers().await,

        CloudCommands::Preflight {
            provider,
            tee_required,
            bootstrap_env,
            write_env_file,
        } => preflight::run(provider, tee_required, bootstrap_env, write_env_file).await,
    }
}
