//! Update and rollback commands for cloud deployments

use blueprint_remote_providers::{
    deployment::{UpdateManager, UpdateStrategy, DeploymentVersion},
    infra::provisioner::CloudProvisioner,
};
use color_eyre::{eyre::eyre, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

/// Update a deployed blueprint to a new version
pub async fn update(
    service_id: String,
    image: String,
    strategy: String,
    env: Vec<String>,
    skip_health_check: bool,
) -> Result<()> {
    println!("🚀 Updating service {}", service_id);

    // Parse environment variables
    let env_vars = parse_env_vars(env)?;

    // Parse update strategy
    let update_strategy = match strategy.as_str() {
        "blue-green" => UpdateStrategy::BlueGreen {
            switch_timeout: Duration::from_secs(300),
            health_check_duration: Duration::from_secs(60),
        },
        "rolling" => UpdateStrategy::RollingUpdate {
            max_unavailable: 1,
            max_surge: 1,
        },
        "canary" => UpdateStrategy::Canary {
            initial_percentage: 10,
            increment: 20,
            interval: Duration::from_secs(60),
        },
        "recreate" => UpdateStrategy::Recreate,
        _ => {
            return Err(eyre!(
                "Invalid update strategy. Choose: blue-green, rolling, canary, or recreate"
            ))
        }
    };

    // Create update manager
    let mut update_manager = UpdateManager::new(update_strategy.clone());

    // Get current deployment
    let provisioner = CloudProvisioner::new().await?;
    let deployments = provisioner.list_deployments().await?;

    let current = deployments
        .iter()
        .find(|d| d.blueprint_id == service_id)
        .ok_or_else(|| eyre!("Service {} not found", service_id))?;

    // Show update plan
    println!("\n📋 Update Plan:");
    println!("  Current Image: {}", current.metadata.get("image").unwrap_or(&"unknown".to_string()));
    println!("  New Image: {}", image);
    println!("  Strategy: {}", strategy);
    println!("  Environment Variables: {} configured", env_vars.len());

    if !skip_health_check {
        println!("  Health Checks: Enabled");
    } else {
        println!("  Health Checks: SKIPPED (not recommended)");
    }

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    // Perform update
    pb.set_message("Starting update...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Get the appropriate adapter based on provider
    let provider = current.instance.provider.clone();
    let adapter = provisioner.get_adapter(&provider)?;

    // Extract resource spec from current deployment
    let resource_spec = blueprint_remote_providers::core::resources::ResourceSpec {
        cpu: current.metadata.get("cpu").and_then(|v| v.parse().ok()).unwrap_or(1.0),
        memory_gb: current.metadata.get("memory_gb").and_then(|v| v.parse().ok()).unwrap_or(1.0),
        storage_gb: current.metadata.get("storage_gb").and_then(|v| v.parse().ok()).unwrap_or(10.0),
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    match update_manager.update_blueprint(
        adapter.as_ref(),
        &image,
        &resource_spec,
        env_vars,
        current,
    ).await {
        Ok(new_deployment) => {
            pb.finish_with_message("✅ Update completed successfully!");

            println!("\n📊 Update Summary:");
            println!("  New Deployment ID: {}", new_deployment.blueprint_id);
            println!("  Instance: {}", new_deployment.instance.id);

            if let Some(ip) = &new_deployment.instance.public_ip {
                println!("  Public IP: {}", ip);
            }

            if !new_deployment.port_mappings.is_empty() {
                println!("  Exposed Ports:");
                for (internal, external) in &new_deployment.port_mappings {
                    println!("    {} -> {}", internal, external);
                }
            }

            Ok(())
        }
        Err(e) => {
            pb.finish_with_message("❌ Update failed!");
            error!("Update failed: {}", e);

            // Ask about rollback
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Would you like to rollback to the previous version?")
                .default(true)
                .interact()?
            {
                rollback(service_id, None, true).await?;
            }

            Err(e.into())
        }
    }
}

/// Rollback a deployment to a previous version
pub async fn rollback(
    service_id: String,
    version: Option<String>,
    yes: bool,
) -> Result<()> {
    println!("⏪ Rolling back service {}", service_id);

    let mut update_manager = UpdateManager::new(UpdateStrategy::default());

    // Get deployment history
    let provisioner = CloudProvisioner::new().await?;
    let deployments = provisioner.list_deployments().await?;

    let current = deployments
        .iter()
        .find(|d| d.blueprint_id == service_id)
        .ok_or_else(|| eyre!("Service {} not found", service_id))?;

    // Determine target version
    let target_version = if let Some(v) = version {
        v
    } else {
        // Get previous version from history
        let versions = update_manager.list_versions();
        if versions.len() < 2 {
            return Err(eyre!("No previous version available for rollback"));
        }

        // Select interactively
        let version_strings: Vec<String> = versions
            .iter()
            .rev()
            .take(5)
            .map(|v| format!("{} - {} ({})",
                v.version,
                v.blueprint_image,
                match v.status {
                    blueprint_remote_providers::deployment::update_manager::VersionStatus::Active => "active",
                    blueprint_remote_providers::deployment::update_manager::VersionStatus::Inactive => "inactive",
                    blueprint_remote_providers::deployment::update_manager::VersionStatus::Failed => "failed",
                    blueprint_remote_providers::deployment::update_manager::VersionStatus::RolledBack => "rolled back",
                    blueprint_remote_providers::deployment::update_manager::VersionStatus::Staging => "staging",
                }
            ))
            .collect();

        if version_strings.is_empty() {
            return Err(eyre!("No versions available for rollback"));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select version to rollback to")
            .items(&version_strings)
            .default(0)
            .interact()?;

        versions[versions.len() - 1 - selection].version.clone()
    };

    // Confirm rollback
    if !yes {
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Rollback to version {}?", target_version))
            .default(false)
            .interact()?
        {
            println!("Rollback cancelled");
            return Ok(());
        }
    }

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );

    pb.set_message("Rolling back...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Get the appropriate adapter
    let provider = current.instance.provider.clone();
    let adapter = provisioner.get_adapter(&provider)?;

    match update_manager.rollback(
        adapter.as_ref(),
        &target_version,
        current,
    ).await {
        Ok(rollback_deployment) => {
            pb.finish_with_message("✅ Rollback completed successfully!");

            println!("\n📊 Rollback Summary:");
            println!("  Active Version: {}", target_version);
            println!("  Deployment ID: {}", rollback_deployment.blueprint_id);
            println!("  Instance: {}", rollback_deployment.instance.id);

            Ok(())
        }
        Err(e) => {
            pb.finish_with_message("❌ Rollback failed!");
            error!("Rollback failed: {}", e);
            Err(e.into())
        }
    }
}

/// View deployment history
pub async fn history(service_id: String, limit: usize) -> Result<()> {
    println!("📜 Service History for {}", service_id);
    println!();

    let update_manager = UpdateManager::new(UpdateStrategy::default());
    let history = update_manager.get_history(limit);

    if history.is_empty() {
        println!("No deployment history available");
        return Ok(());
    }

    // Display history in table format
    println!("{:<15} {:<30} {:<20} {:<10}", "Version", "Image", "Deployed", "Status");
    println!("{}", "-".repeat(80));

    for version in history {
        let deployed = if let Ok(duration) = version.deployment_time.elapsed() {
            format_duration(duration)
        } else {
            "Unknown".to_string()
        };

        let status = match version.status {
            blueprint_remote_providers::deployment::update_manager::VersionStatus::Active => "✅ Active",
            blueprint_remote_providers::deployment::update_manager::VersionStatus::Inactive => "⭕ Inactive",
            blueprint_remote_providers::deployment::update_manager::VersionStatus::Failed => "❌ Failed",
            blueprint_remote_providers::deployment::update_manager::VersionStatus::RolledBack => "⏪ Rolled Back",
            blueprint_remote_providers::deployment::update_manager::VersionStatus::Staging => "🔄 Staging",
        };

        println!(
            "{:<15} {:<30} {:<20} {:<10}",
            version.version,
            truncate(&version.blueprint_image, 28),
            deployed,
            status
        );
    }

    Ok(())
}

/// Parse environment variables from KEY=VALUE format
fn parse_env_vars(env: Vec<String>) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    for e in env {
        let parts: Vec<&str> = e.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(eyre!("Invalid environment variable format: {}. Use KEY=VALUE", e));
        }
        vars.insert(parts[0].to_string(), parts[1].to_string());
    }
    Ok(vars)
}

/// Format duration in human-readable format
fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    if seconds < 60 {
        format!("{}s ago", seconds)
    } else if seconds < 3600 {
        format!("{}m ago", seconds / 60)
    } else if seconds < 86400 {
        format!("{}h ago", seconds / 3600)
    } else {
        format!("{}d ago", seconds / 86400)
    }
}

/// Truncate string to specified length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}