//! Cloud deployment commands for Blueprint services.
//!
//! This module provides commands for deploying Blueprint services to various cloud providers
//! including AWS, GCP, Azure, DigitalOcean, and Vultr. It enables remote deployment of
//! Blueprint instances with resource configuration, cost estimation, and lifecycle management.
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
//! Deploy a blueprint (uses configured policy):
//! ```bash
//! cargo tangle blueprint deploy tangle --remote
//! ```

#![allow(unexpected_cfgs)]
#![allow(unused_imports)]
#![allow(dead_code)]

use clap::Subcommand;
use color_eyre::Result;
use std::path::PathBuf;
use url::Url;

mod config;
mod estimate;
mod logs;
mod policy;
mod status;
mod update;

pub use config::CloudProvider;
pub use policy::{CostOptimization, RemoteDeploymentPolicy};

#[derive(Subcommand, Debug)]
pub enum CloudCommands {
    /// Set up cloud provider access
    #[command(visible_alias = "cfg")]
    Configure {
        /// Cloud provider (aws, gcp, azure, digitalocean, vultr)
        #[arg(value_enum)]
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
    Estimate {
        /// Compare all providers
        #[arg(short = 'c', long)]
        compare: bool,

        /// Specific provider to estimate
        #[arg(short, long, value_enum)]
        provider: Option<CloudProvider>,

        /// CPU cores
        #[arg(long, default_value = "4")]
        cpu: f32,

        /// Memory in GB
        #[arg(long, default_value = "16")]
        memory: f32,

        /// Number of GPUs
        #[arg(long)]
        gpu: Option<u32>,

        /// Duration (e.g., 1h, 24h, 30d)
        #[arg(short = 'd', long, default_value = "24h")]
        duration: String,

        /// Include spot pricing
        #[arg(short, long)]
        spot: bool,
    },

    /// Check deployment status
    #[command(visible_alias = "s")]
    Status {
        /// Service ID (shows all if not specified)
        service_id: Option<String>,

        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
    },

    /// Terminate cloud deployments
    #[command(visible_alias = "term")]
    Terminate {
        /// Service ID to terminate
        service_id: Option<String>,

        /// Terminate all deployments
        #[arg(long, conflicts_with = "service_id")]
        all: bool,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// Update deployed blueprint to new version
    #[command(visible_alias = "up")]
    Update {
        /// Service ID to update
        service_id: String,

        /// New blueprint image to deploy
        #[arg(short, long)]
        image: String,

        /// Update strategy (blue-green, rolling, canary, recreate)
        #[arg(short = 's', long, default_value = "blue-green")]
        strategy: String,

        /// Environment variables (KEY=VALUE)
        #[arg(short, long)]
        env: Vec<String>,

        /// Skip health checks
        #[arg(long)]
        skip_health_check: bool,
    },

    /// Rollback blueprint to previous version
    #[command(visible_alias = "rb")]
    Rollback {
        /// Service ID to rollback
        service_id: String,

        /// Target version to rollback to (defaults to previous)
        #[arg(short, long)]
        version: Option<String>,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },

    /// View deployment history
    #[command(visible_alias = "hist")]
    History {
        /// Service ID
        service_id: String,

        /// Number of versions to show
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,
    },

    /// Stream logs from deployed blueprint
    #[command(visible_alias = "logs")]
    Logs {
        /// Service ID
        service_id: String,

        /// Follow log output (like tail -f)
        #[arg(short, long)]
        follow: bool,

        /// Filter by log level (debug, info, warn, error)
        #[arg(short, long)]
        level: Option<String>,

        /// Search for specific text
        #[arg(short, long)]
        search: Option<String>,

        /// Show logs since duration (e.g., 5m, 1h, 1d)
        #[arg(long)]
        since: Option<String>,

        /// Number of lines to show (when not following)
        #[arg(short = 'n', long, default_value = "100")]
        lines: usize,
    },

    /// List configured providers
    #[command(visible_alias = "ls")]
    List,
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

        CloudCommands::Estimate {
            compare,
            provider,
            cpu,
            memory,
            gpu,
            duration,
            spot,
        } => {
            estimate::estimate(estimate::EstimateOptions {
                compare,
                provider,
                cpu,
                memory,
                gpu,
                duration,
                spot,
            })
            .await
        }

        CloudCommands::Status { service_id, watch } => status::show_status(service_id, watch).await,

        CloudCommands::Terminate {
            service_id,
            all,
            yes,
        } => status::terminate(service_id, all, yes).await,

        CloudCommands::Update {
            service_id,
            image,
            strategy,
            env,
            skip_health_check,
        } => update::update(service_id, image, strategy, env, skip_health_check).await,

        CloudCommands::Rollback {
            service_id,
            version,
            yes,
        } => update::rollback(service_id, version, yes).await,

        CloudCommands::History { service_id, limit } => update::history(service_id, limit).await,

        CloudCommands::Logs {
            service_id,
            follow,
            level,
            search,
            since,
            lines,
        } => logs::stream_logs(service_id, follow, level, search, since, lines).await,

        CloudCommands::List => config::list_providers().await,
    }
}
