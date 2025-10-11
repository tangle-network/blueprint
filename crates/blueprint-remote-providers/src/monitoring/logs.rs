//! Log streaming and collection from remote deployments
//!
//! Provides real-time log streaming from deployed blueprints across
//! different deployment targets (SSH, Kubernetes, cloud provider logs).

use crate::core::error::{Error, Result};
use crate::deployment::ssh::SshDeploymentClient;
use blueprint_core::{debug, error, info, warn};
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;
use std::time::SystemTime;
use tokio::sync::mpsc;
use tokio::time::Duration;

/// Log entry from a remote deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub service_id: String,
    pub container_id: Option<String>,
    pub level: LogLevel,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug" | "trace" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "fatal" | "critical" => LogLevel::Fatal,
            _ => LogLevel::Info,
        }
    }
}

/// Log source type
#[derive(Debug, Clone)]
pub enum LogSource {
    /// Local Docker container logs
    LocalDocker { container_id: String },
    /// Local Kubernetes pod logs
    LocalKubernetes {
        namespace: String,
        pod_name: String,
        container_name: Option<String>,
    },
    /// SSH container logs - stores connection details for creating client
    SshContainer {
        host: String,
        port: u16,
        user: String,
        container_id: String,
    },
    /// Kubernetes pod logs
    #[cfg(feature = "kubernetes")]
    Kubernetes {
        namespace: String,
        pod_name: String,
        container_name: Option<String>,
    },
    /// AWS CloudWatch logs
    #[cfg(feature = "aws")]
    CloudWatch {
        log_group: String,
        log_stream: String,
    },
    /// GCP Cloud Logging
    #[cfg(feature = "gcp")]
    CloudLogging {
        project_id: String,
        resource_type: String,
        resource_id: String,
    },
    /// Generic file-based logs
    File { host: String, file_path: String },
}

/// Log streaming manager
pub struct LogStreamer {
    sources: Vec<(String, LogSource)>,
    buffer_size: usize,
    follow: bool,
}

impl LogStreamer {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            sources: Vec::new(),
            buffer_size,
            follow: true,
        }
    }

    /// Add a log source to stream from
    pub fn add_source(&mut self, service_id: String, source: LogSource) {
        info!("Adding log source for deployment: {}", service_id);
        self.sources.push((service_id, source));
    }

    /// Set whether to follow logs (tail -f behavior)
    pub fn set_follow(&mut self, follow: bool) {
        self.follow = follow;
    }

    /// Start streaming logs from all sources
    pub async fn stream(&self) -> Result<impl Stream<Item = LogEntry>> {
        let (tx, rx) = mpsc::channel::<LogEntry>(self.buffer_size);

        // Start streaming from each source
        for (service_id, source) in &self.sources {
            let tx_clone = tx.clone();
            let service_id = service_id.clone();
            let source = source.clone();
            let follow = self.follow;

            tokio::spawn(async move {
                if let Err(e) = stream_from_source(tx_clone, service_id, source, follow).await {
                    error!("Error streaming logs: {}", e);
                }
            });
        }

        // Convert receiver to stream using tokio_stream
        use futures::stream;
        let stream = stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|entry| (entry, rx))
        });

        Ok(Box::pin(stream))
    }

    /// Stream logs for a specific duration
    pub async fn stream_for_duration(&self, duration: Duration) -> Result<Vec<LogEntry>> {
        let stream = self.stream().await?;
        let mut entries = Vec::new();

        tokio::select! {
            _ = async {
                let mut stream = Box::pin(stream);
                while let Some(entry) = stream.next().await {
                    entries.push(entry);
                }
            } => {}
            _ = tokio::time::sleep(duration) => {
                info!("Log streaming duration reached");
            }
        }

        Ok(entries)
    }
}

/// Stream logs from a specific source
async fn stream_from_source(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    source: LogSource,
    follow: bool,
) -> Result<()> {
    match source {
        LogSource::LocalDocker { container_id } => {
            stream_local_docker_logs(tx, service_id, container_id, follow).await
        }
        LogSource::LocalKubernetes {
            namespace,
            pod_name,
            container_name,
        } => {
            stream_local_kubernetes_logs(
                tx,
                service_id,
                namespace,
                pod_name,
                container_name,
                follow,
            )
            .await
        }
        LogSource::SshContainer {
            host,
            port,
            user,
            container_id,
        } => {
            // Create SSH client from connection details
            use crate::deployment::ssh::{
                ContainerRuntime, DeploymentConfig, SshConnection, SshDeploymentClient,
            };

            let connection = SshConnection {
                host,
                port,
                user,
                key_path: None,
                password: None,
                jump_host: None,
            };

            let deployment_config = DeploymentConfig {
                name: format!("log-stream-{container_id}"),
                namespace: "default".to_string(),
                restart_policy: crate::deployment::ssh::RestartPolicy::OnFailure,
                health_check: None,
            };

            match SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config)
                .await
            {
                Ok(ssh_client) => {
                    stream_ssh_logs(tx, service_id, ssh_client, container_id, follow).await
                }
                Err(e) => {
                    error!("Failed to create SSH client for log streaming: {}", e);
                    Err(e)
                }
            }
        }
        #[cfg(feature = "kubernetes")]
        LogSource::Kubernetes {
            namespace,
            pod_name,
            container_name,
        } => {
            stream_kubernetes_logs(tx, service_id, namespace, pod_name, container_name, follow)
                .await
        }
        #[cfg(feature = "aws")]
        LogSource::CloudWatch {
            log_group,
            log_stream,
        } => stream_cloudwatch_logs(tx, service_id, log_group, log_stream, follow).await,
        #[cfg(feature = "gcp")]
        LogSource::CloudLogging {
            project_id,
            resource_type,
            resource_id,
        } => {
            stream_cloud_logging(
                tx,
                service_id,
                project_id,
                resource_type,
                resource_id,
                follow,
            )
            .await
        }
        LogSource::File { host, file_path } => {
            stream_file_logs(tx, service_id, host, file_path, follow).await
        }
    }
}

/// Stream logs from SSH container
async fn stream_ssh_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    ssh_client: SshDeploymentClient,
    container_id: String,
    follow: bool,
) -> Result<()> {
    info!("Streaming SSH container logs for: {}", container_id);

    loop {
        // Get logs from container
        let logs = ssh_client.stream_logs(&container_id, follow).await?;

        // Parse and send log entries
        for line in logs.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let entry = parse_log_line(&service_id, &container_id, line);

            if tx.send(entry).await.is_err() {
                debug!("Log receiver dropped, stopping stream");
                break;
            }
        }

        if !follow {
            break;
        }

        // Wait before next poll
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

/// Stream logs from Kubernetes pod
#[cfg(feature = "kubernetes")]
async fn stream_kubernetes_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    namespace: String,
    pod_name: String,
    container_name: Option<String>,
    follow: bool,
) -> Result<()> {
    use k8s_openapi::api::core::v1::Pod;
    use kube::{
        Client,
        api::{Api, LogParams},
    };

    info!(
        "Streaming Kubernetes logs for pod: {}/{}",
        namespace, pod_name
    );

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, &namespace);

    let log_params = LogParams {
        follow,
        container: container_name,
        timestamps: true,
        ..Default::default()
    };

    // Get logs directly instead of streaming (simpler approach)
    let logs = pods
        .logs(&pod_name, &log_params)
        .await
        .map_err(|e| Error::ConfigurationError(format!("Failed to get logs: {}", e)))?;

    // Process the log lines
    for log_line in logs.lines() {
        if log_line.trim().is_empty() {
            continue;
        }

        let entry = parse_k8s_log_line(&service_id, &pod_name, log_line);

        if tx.send(entry).await.is_err() {
            debug!("Log receiver dropped, stopping stream");
            break;
        }
    }

    // If follow is enabled, we could implement polling here
    if follow {
        warn!("Log following not fully implemented - would need streaming setup");
    }

    Ok(())
}

/// Stream logs from AWS CloudWatch
#[cfg(feature = "aws")]
async fn stream_cloudwatch_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    log_group: String,
    log_stream: String,
    follow: bool,
) -> Result<()> {
    use aws_config;
    use aws_sdk_cloudwatchlogs::Client;

    info!("Streaming CloudWatch logs: {}/{}", log_group, log_stream);

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let mut next_token = None;
    let mut last_timestamp = None;

    loop {
        let response = client
            .filter_log_events()
            .log_group_name(&log_group)
            .log_stream_names(&log_stream)
            .set_next_token(next_token.clone())
            .set_start_time(last_timestamp)
            .send()
            .await?;

        if let Some(events) = response.events {
            for event in events {
                if let Some(message) = event.message {
                    let entry = LogEntry {
                        timestamp: SystemTime::now(),
                        service_id: service_id.clone(),
                        container_id: Some(log_stream.clone()),
                        level: LogLevel::Info,
                        message,
                        metadata: HashMap::new(),
                    };

                    if tx.send(entry).await.is_err() {
                        debug!("Log receiver dropped, stopping stream");
                        return Ok(());
                    }

                    if let Some(timestamp) = event.timestamp {
                        last_timestamp = Some(timestamp);
                    }
                }
            }
        }

        next_token = response.next_token;

        if !follow || next_token.is_none() {
            break;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

/// Stream logs from GCP Cloud Logging
#[cfg(feature = "gcp")]
async fn stream_cloud_logging(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    project_id: String,
    resource_type: String,
    resource_id: String,
    follow: bool,
) -> Result<()> {
    // Implementation would use google-cloud-logging crate
    warn!("GCP Cloud Logging streaming not yet implemented");
    Ok(())
}

/// Stream logs from remote file
async fn stream_file_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    host: String,
    file_path: String,
    follow: bool,
) -> Result<()> {
    info!("Streaming file logs from {}:{}", host, file_path);

    if host == "localhost" || host == "127.0.0.1" {
        // Local file - use tail
        stream_local_file_logs(tx, service_id, file_path, follow).await
    } else {
        // Remote file - use SSH
        use crate::deployment::ssh::{DeploymentConfig, SshConnection};

        let _connection = SshConnection {
            host,
            port: 22,
            user: "root".to_string(),
            key_path: None,
            password: None,
            jump_host: None,
        };

        let _deployment_config = DeploymentConfig {
            name: format!("file-log-{}", uuid::Uuid::new_v4()),
            namespace: "default".to_string(),
            restart_policy: crate::deployment::ssh::RestartPolicy::OnFailure,
            health_check: None,
        };

        // For remote files, we need to use SSH streaming
        // This is a simplified implementation - could be enhanced
        warn!("Remote file log streaming requires SSH - not fully implemented");
        Ok(())
    }
}

/// Stream logs from local Docker container
async fn stream_local_docker_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    container_id: String,
    follow: bool,
) -> Result<()> {
    info!(
        "Streaming local Docker logs for container: {}",
        container_id
    );

    let mut cmd = tokio::process::Command::new("docker");
    cmd.arg("logs");
    if follow {
        cmd.arg("-f");
    }
    cmd.arg(&container_id);

    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| Error::Other(format!("Failed to start docker logs: {e}")))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| Error::Other("Failed to capture stdout".into()))?;

    use tokio::io::{AsyncBufReadExt, BufReader};
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // EOF
            Ok(_) => {
                let entry = parse_log_line(&service_id, &container_id, line.trim());
                if tx.send(entry).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                warn!("Error reading docker logs: {}", e);
                break;
            }
        }
    }

    let _ = child.kill().await;
    Ok(())
}

/// Stream logs from local Kubernetes pod
async fn stream_local_kubernetes_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    namespace: String,
    pod_name: String,
    container_name: Option<String>,
    follow: bool,
) -> Result<()> {
    info!(
        "Streaming local Kubernetes logs for pod: {}/{}",
        namespace, pod_name
    );

    let mut cmd = tokio::process::Command::new("kubectl");
    cmd.arg("logs").arg("-n").arg(&namespace);

    if follow {
        cmd.arg("-f");
    }

    if let Some(container) = &container_name {
        cmd.arg("-c").arg(container);
    }

    cmd.arg(&pod_name);

    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| Error::Other(format!("Failed to start kubectl logs: {e}")))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| Error::Other("Failed to capture stdout".into()))?;

    use tokio::io::{AsyncBufReadExt, BufReader};
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let entry = parse_log_line(&service_id, &pod_name, line.trim());
                if tx.send(entry).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                warn!("Error reading kubectl logs: {}", e);
                break;
            }
        }
    }

    let _ = child.kill().await;
    Ok(())
}

/// Stream logs from local file
async fn stream_local_file_logs(
    tx: mpsc::Sender<LogEntry>,
    service_id: String,
    file_path: String,
    follow: bool,
) -> Result<()> {
    info!("Streaming local file logs: {}", file_path);

    let mut cmd = tokio::process::Command::new("tail");
    if follow {
        cmd.arg("-f");
    } else {
        cmd.arg("-n").arg("1000");
    }
    cmd.arg(&file_path);

    let mut child = cmd
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| Error::Other(format!("Failed to start tail: {e}")))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| Error::Other("Failed to capture stdout".into()))?;

    use tokio::io::{AsyncBufReadExt, BufReader};
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let entry = parse_log_line(&service_id, &file_path, line.trim());
                if tx.send(entry).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                warn!("Error reading file: {}", e);
                break;
            }
        }
    }

    let _ = child.kill().await;
    Ok(())
}

/// Parse a log line into a LogEntry
fn parse_log_line(service_id: &str, container_id: &str, line: &str) -> LogEntry {
    // Try to parse structured logs (JSON)
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
        let level = json["level"]
            .as_str()
            .map(LogLevel::from)
            .unwrap_or(LogLevel::Info);

        let message = json["message"].as_str().unwrap_or(line).to_string();

        let mut metadata = HashMap::new();
        if let Some(obj) = json.as_object() {
            for (key, value) in obj {
                if key != "level" && key != "message" && key != "timestamp" {
                    metadata.insert(key.clone(), value.to_string());
                }
            }
        }

        LogEntry {
            timestamp: SystemTime::now(),
            service_id: service_id.to_string(),
            container_id: Some(container_id.to_string()),
            level,
            message,
            metadata,
        }
    } else {
        // Plain text log
        let level = detect_log_level(line);

        LogEntry {
            timestamp: SystemTime::now(),
            service_id: service_id.to_string(),
            container_id: Some(container_id.to_string()),
            level,
            message: line.to_string(),
            metadata: HashMap::new(),
        }
    }
}

/// Parse Kubernetes log line (with timestamp prefix)
#[allow(dead_code)]
fn parse_k8s_log_line(service_id: &str, pod_name: &str, line: &str) -> LogEntry {
    // K8s logs often have format: "2024-01-01T12:00:00.000Z message"
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    let (timestamp_str, message) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        ("", line)
    };

    let timestamp = parse_timestamp(timestamp_str).unwrap_or_else(SystemTime::now);
    let level = detect_log_level(message);

    LogEntry {
        timestamp,
        service_id: service_id.to_string(),
        container_id: Some(pod_name.to_string()),
        level,
        message: message.to_string(),
        metadata: HashMap::new(),
    }
}

/// Detect log level from message content
fn detect_log_level(message: &str) -> LogLevel {
    let lower = message.to_lowercase();

    if lower.contains("error") || lower.contains("err:") {
        LogLevel::Error
    } else if lower.contains("warn") || lower.contains("warning") {
        LogLevel::Warn
    } else if lower.contains("debug") || lower.contains("dbg:") {
        LogLevel::Debug
    } else if lower.contains("fatal") || lower.contains("panic") || lower.contains("critical") {
        LogLevel::Fatal
    } else {
        LogLevel::Info
    }
}

/// Parse timestamp string
#[allow(dead_code)]
fn parse_timestamp(s: &str) -> Option<SystemTime> {
    // Try ISO 8601 format
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Some(SystemTime::from(dt));
    }

    // Try other formats...
    None
}

/// Log aggregator for collecting logs from multiple deployments
pub struct LogAggregator {
    deployments: HashMap<String, LogSource>,
    filters: LogFilters,
}

#[derive(Default, Clone)]
pub struct LogFilters {
    pub level_min: Option<LogLevel>,
    pub service_ids: Option<Vec<String>>,
    pub search_text: Option<String>,
    pub since: Option<SystemTime>,
    pub until: Option<SystemTime>,
}

impl Default for LogAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl LogAggregator {
    pub fn new() -> Self {
        Self {
            deployments: HashMap::new(),
            filters: LogFilters::default(),
        }
    }

    /// Add deployment to aggregate logs from
    pub fn add_deployment(&mut self, service_id: String, source: LogSource) {
        self.deployments.insert(service_id, source);
    }

    /// Set log filters
    pub fn set_filters(&mut self, filters: LogFilters) {
        self.filters = filters;
    }

    /// Stream aggregated logs with filters applied
    pub async fn stream_filtered(&self) -> Result<Pin<Box<dyn Stream<Item = LogEntry> + Send>>> {
        let (tx, rx) = mpsc::channel::<LogEntry>(1000);

        // Start streaming from each deployment source
        for (service_id, source) in &self.deployments {
            let tx_clone = tx.clone();
            let service_id = service_id.clone();
            let source = source.clone();

            tokio::spawn(async move {
                if let Err(e) = stream_from_source(tx_clone, service_id, source, true).await {
                    error!("Error streaming logs: {}", e);
                }
            });
        }

        // Drop the sender so the channel closes when all spawned tasks complete
        drop(tx);

        // Convert receiver to stream
        use futures::stream;
        let stream = stream::unfold(rx, |mut rx| async move {
            rx.recv().await.map(|entry| (entry, rx))
        });

        let filters = self.filters.clone();

        // Apply filters
        let filtered_stream = stream.filter(move |entry| {
            let mut pass = true;

            // Filter by level
            if let Some(ref min_level) = filters.level_min {
                pass &= entry.level >= *min_level;
            }

            // Filter by deployment ID
            if let Some(ref ids) = filters.service_ids {
                pass &= ids.contains(&entry.service_id);
            }

            // Filter by search text
            if let Some(ref text) = filters.search_text {
                pass &= entry.message.contains(text);
            }

            // Filter by time range
            if let Some(since) = filters.since {
                pass &= entry.timestamp >= since;
            }

            if let Some(until) = filters.until {
                pass &= entry.timestamp <= until;
            }

            async move { pass }
        });

        Ok(Box::pin(filtered_stream))
    }

    /// Collect logs for a specific time window
    pub async fn collect_window(&self, duration: Duration) -> Result<Vec<LogEntry>> {
        let stream = self.stream_filtered().await?;
        let mut entries = Vec::new();

        let mut stream = Box::pin(stream);
        let timeout = tokio::time::sleep(duration);
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                Some(entry) = stream.next() => {
                    entries.push(entry);
                }
                _ = &mut timeout => {
                    break;
                }
            }
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_detection() {
        assert_eq!(
            detect_log_level("ERROR: Something went wrong"),
            LogLevel::Error
        );
        assert_eq!(detect_log_level("WARN: Low memory"), LogLevel::Warn);
        assert_eq!(detect_log_level("Debug: Variable x = 5"), LogLevel::Debug);
        assert_eq!(detect_log_level("Info: Server started"), LogLevel::Info);
        assert_eq!(detect_log_level("FATAL: System crash"), LogLevel::Fatal);
    }

    #[test]
    fn test_parse_log_line() {
        let entry = parse_log_line(
            "deploy-1",
            "container-1",
            "ERROR: Database connection failed",
        );

        assert_eq!(entry.service_id, "deploy-1");
        assert_eq!(entry.container_id, Some("container-1".to_string()));
        assert_eq!(entry.level, LogLevel::Error);
        assert!(entry.message.contains("Database connection failed"));
    }

    #[test]
    fn test_json_log_parsing() {
        let json_log =
            r#"{"level":"error","message":"Connection timeout","host":"db.example.com"}"#;
        let entry = parse_log_line("deploy-1", "container-1", json_log);

        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "Connection timeout");
        assert_eq!(
            entry.metadata.get("host"),
            Some(&"\"db.example.com\"".to_string())
        );
    }
}
