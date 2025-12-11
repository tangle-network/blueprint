//! Log streaming command for cloud deployments

#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::{
    deployment::ssh::SshDeploymentClient,
    DeploymentTracker,
    infra::provisioner::CloudProvisioner,
    monitoring::logs::{LogAggregator, LogFilters, LogLevel, LogSource, LogStreamer},
};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Result, eyre::eyre};
use colored::Colorize;
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Duration, SystemTime};
use tokio::time::sleep;
use tracing::{error, info};

/// Stream logs from a deployed blueprint
pub async fn stream_logs(
    service_id: String,
    follow: bool,
    level: Option<String>,
    search: Option<String>,
    since: Option<String>,
    lines: usize,
) -> Result<()> {
    println!("📜 Streaming logs for service: {}", service_id);

    // Parse log level filter
    let level_filter = level.map(|l| match l.to_lowercase().as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" | "warning" => LogLevel::Warn,
        "error" => LogLevel::Error,
        "fatal" => LogLevel::Fatal,
        _ => LogLevel::Info,
    });

    // Parse since duration
    let since_time = since
        .map(|s| parse_duration(&s))
        .transpose()?
        .map(|d| SystemTime::now() - d);

    // Get deployment information
    let tracker_path = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".tangle")
        .join("remote_deployments");

    let deployments = if tracker_path.exists() {
        let tracker = DeploymentTracker::new(&tracker_path).await?;
        tracker.list_all().await?
    } else {
        Vec::new()
    };

    let deployment = deployments
        .iter()
        .find(|d| d.blueprint_id == service_id)
        .ok_or_else(|| eyre!("Deployment {} not found", service_id))?;

    // Determine log source based on deployment type
    let log_source = determine_log_source(deployment).await?;

    // Create log streamer
    let mut streamer = LogStreamer::new(1000);
    streamer.add_source(service_id.clone(), log_source);
    streamer.set_follow(follow);

    // Create aggregator with filters
    let mut aggregator = LogAggregator::new();

    let mut filters = LogFilters::default();
    filters.level_min = level_filter;
    filters.search_text = search;
    filters.since = since_time;

    aggregator.set_filters(filters);

    if follow {
        println!("Following logs... (Press Ctrl+C to stop)");
        println!();

        // Stream logs continuously
        let mut stream = streamer.stream().await?;
        let mut count = 0;

        while let Some(entry) = stream.next().await {
            if let Some(ref level_min) = level_filter {
                if entry.level < *level_min {
                    continue;
                }
            }

            if let Some(ref search_text) = search {
                if !entry.message.contains(search_text) {
                    continue;
                }
            }

            print_log_entry(&entry);
            count += 1;
        }

        println!("\n📊 Streamed {} log entries", count);
    } else {
        // Collect limited number of logs
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );

        pb.set_message("Fetching logs...");
        pb.enable_steady_tick(Duration::from_millis(100));

        // Stream for a short duration to collect logs
        let entries = streamer.stream_for_duration(Duration::from_secs(5)).await?;

        pb.finish_and_clear();

        // Apply filters and limit
        let filtered: Vec<_> = entries
            .into_iter()
            .filter(|e| {
                let mut pass = true;

                if let Some(ref level_min) = level_filter {
                    pass &= e.level >= *level_min;
                }

                if let Some(ref search_text) = search {
                    pass &= e.message.contains(search_text);
                }

                if let Some(since) = since_time {
                    pass &= e.timestamp >= since;
                }

                pass
            })
            .take(lines)
            .collect();

        if filtered.is_empty() {
            println!("No logs found matching the criteria");
        } else {
            println!("Showing {} log entries:\n", filtered.len());

            for entry in &filtered {
                print_log_entry(entry);
            }

            println!(
                "\n📊 Displayed {} of {} total log entries",
                filtered.len(),
                lines
            );
        }
    }

    Ok(())
}

/// Print a formatted log entry
fn print_log_entry(entry: &blueprint_remote_providers::monitoring::logs::LogEntry) {
    let timestamp = format_timestamp(entry.timestamp);

    let level_str = match entry.level {
        LogLevel::Debug => format!("{}", "DEBUG".bright_black()),
        LogLevel::Info => format!("{}", "INFO ".green()),
        LogLevel::Warn => format!("{}", "WARN ".yellow()),
        LogLevel::Error => format!("{}", "ERROR".red()),
        LogLevel::Fatal => format!("{}", "FATAL".bright_red().bold()),
    };

    let container_id = entry
        .container_id
        .as_ref()
        .map(|id| if id.len() > 12 { &id[..12] } else { id })
        .unwrap_or("unknown");

    println!(
        "{} {} [{}] {}",
        timestamp.bright_black(),
        level_str,
        container_id.cyan(),
        entry.message
    );

    // Print metadata if present
    if !entry.metadata.is_empty() {
        for (key, value) in &entry.metadata {
            println!("      {} = {}", key.bright_black(), value);
        }
    }
}

/// Determine the log source for a deployment
async fn determine_log_source(
    deployment: &blueprint_remote_providers::deployment::tracker::DeploymentRecord,
) -> Result<LogSource> {
    use blueprint_remote_providers::core::remote::CloudProvider;

    // Check deployment metadata to determine type
    if let Some(deployment_type) = deployment.metadata.get("deployment_type") {
        match deployment_type.as_str() {
            "ssh" => {
                // SSH deployment
                let host = deployment
                    .metadata
                    .get("ssh_host")
                    .ok_or_else(|| eyre!("SSH host not found in metadata"))?;

                let container_id = deployment
                    .metadata
                    .get("container_id")
                    .ok_or_else(|| eyre!("Container ID not found in metadata"))?;

                // Create SSH client (would need proper connection details)
                info!("Creating SSH log source for container: {}", container_id);

                Ok(LogSource::File {
                    host: host.clone(),
                    file_path: format!("/var/log/containers/{}.log", container_id),
                })
            }
            #[cfg(feature = "kubernetes")]
            "kubernetes" => {
                // Kubernetes deployment
                let namespace = deployment
                    .metadata
                    .get("namespace")
                    .unwrap_or(&"default".to_string())
                    .clone();

                let pod_name = deployment
                    .metadata
                    .get("pod_name")
                    .or_else(|| deployment.metadata.get("deployment_name"))
                    .ok_or_else(|| eyre!("Pod name not found in metadata"))?
                    .clone();

                Ok(LogSource::Kubernetes {
                    namespace,
                    pod_name,
                    container_name: None,
                })
            }
            _ => {
                // Fall back to provider-specific logs
                determine_provider_log_source(deployment)
            }
        }
    } else {
        // Use provider-specific log source
        determine_provider_log_source(deployment)
    }
}

/// Determine provider-specific log source
fn determine_provider_log_source(
    deployment: &blueprint_remote_providers::deployment::tracker::DeploymentRecord,
) -> Result<LogSource> {
    use blueprint_remote_providers::core::remote::CloudProvider;

    let provider = deployment.provider.as_ref().ok_or_else(|| eyre!("Provider not found in deployment"))?;
    let instance_id = deployment.id.clone();

    match provider {
        #[cfg(feature = "aws")]
        CloudProvider::AWS => {
            // CloudWatch logs
            Ok(LogSource::CloudWatch {
                log_group: format!("/aws/ec2/{}", instance_id),
                log_stream: deployment.blueprint_id.clone(),
            })
        }
        #[cfg(feature = "gcp")]
        CloudProvider::GCP => {
            // GCP Cloud Logging
            Ok(LogSource::CloudLogging {
                project_id: deployment
                    .metadata
                    .get("project_id")
                    .unwrap_or(&"default-project".to_string())
                    .clone(),
                resource_type: "gce_instance".to_string(),
                resource_id: instance_id.clone(),
            })
        }
        _ => {
            // Default to file-based logs
            let host = deployment
                .metadata
                .get("public_ip")
                .or(deployment.metadata.get("private_ip"))
                .ok_or_else(|| eyre!("No IP address found for deployment"))?
                .clone();

            Ok(LogSource::File {
                host,
                file_path: format!("/var/log/blueprint/{}.log", deployment.blueprint_id),
            })
        }
    }
}

/// Parse duration string (e.g., "5m", "1h", "2d")
fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim().to_lowercase();

    // Extract number and unit
    let (num_str, unit) = s.split_at(
        s.find(|c: char| c.is_alphabetic())
            .ok_or_else(|| eyre!("Invalid duration format: {}", s))?,
    );

    let num: u64 = num_str
        .parse()
        .map_err(|_| eyre!("Invalid number in duration: {}", num_str))?;

    let duration = match unit {
        "s" | "sec" | "secs" | "second" | "seconds" => Duration::from_secs(num),
        "m" | "min" | "mins" | "minute" | "minutes" => Duration::from_secs(num * 60),
        "h" | "hr" | "hrs" | "hour" | "hours" => Duration::from_secs(num * 3600),
        "d" | "day" | "days" => Duration::from_secs(num * 86400),
        _ => return Err(eyre!("Unknown time unit: {}", unit)),
    };

    Ok(duration)
}

/// Format timestamp for display
fn format_timestamp(time: SystemTime) -> String {
    if let Ok(duration) = time.elapsed() {
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
    } else {
        // Future time or error
        "now".to_string()
    }
}
