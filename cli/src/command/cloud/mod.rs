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

use clap::Subcommand;
use color_eyre::Result;
use std::path::PathBuf;
use url::Url;

mod config;
mod policy;
mod estimate;
mod status;

pub use config::CloudProvider;
pub use policy::{RemoteDeploymentPolicy, CostOptimization};
use config::CloudConfig;

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
    #[command(visible_alias = "show-policy")]
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
        /// Deployment ID (shows all if not specified)
        deployment_id: Option<String>,
        
        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Terminate cloud deployments  
    #[command(visible_alias = "term")]
    Terminate {
        /// Deployment ID to terminate
        deployment_id: Option<String>,
        
        /// Terminate all deployments
        #[arg(long, conflicts_with = "deployment_id")]
        all: bool,
        
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
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
        CloudCommands::Configure { provider, region, set_default } => {
            config::configure(provider, region, set_default).await
        }
        
        CloudCommands::ConfigurePolicy { args } => {
            policy::configure_policy(args).await
        }
        
        CloudCommands::ShowPolicy => {
            policy::show_policy().await
        }
        
        CloudCommands::Estimate { 
            compare, provider, cpu, memory, gpu, duration, spot 
        } => {
            estimate::estimate(estimate::EstimateOptions {
                compare,
                provider,
                cpu,
                memory,
                gpu,
                duration,
                spot,
            }).await
        }
        
        CloudCommands::Status { deployment_id, watch } => {
            status::show_status(deployment_id, watch).await
        }
        
        CloudCommands::Terminate { deployment_id, all, yes } => {
            status::terminate(deployment_id, all, yes).await
        }
        
        CloudCommands::List => {
            config::list_providers().await
        }
    }
}