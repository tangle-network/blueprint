//! Loki log aggregation integration
//!
//! Provides integration with Grafana Loki for centralized log aggregation
//! from both local and remote blueprint deployments.

use crate::core::error::{Error, Result};
use crate::monitoring::logs::{LogEntry, LogLevel};
use blueprint_core::{debug, error, info};
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// @dev Loki version is pinned to 3.3.4 to avoid breaking changes
/// Relating changes:
/// - LokiClient in `crates/blueprint-remote-providers/src/monitoring/loki.rs`
/// - CI
///
const LOKI_IMAGE_NAME_FULL: &str = "grafana/loki:3.3.4";

/// 
/// @dev Grafana version is pinned to 10.4.3 to avoid breaking changes
/// Relating changes:
/// - LokiClient in `crates/blueprint-remote-providers/src/monitoring/loki.rs`
/// - CI
///
const GRAFANA_IMAGE_NAME_FULL: &str = "grafana/grafana:10.4.3";

/// Loki client for pushing and querying logs
pub struct LokiClient {
    base_url: String,
    client: reqwest::Client,
    labels: HashMap<String, String>,
}

impl LokiClient {
    /// Create new Loki client
    pub fn new(base_url: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| Error::Other(format!("Failed to create HTTP client: {e}")))?;

        let mut labels = HashMap::new();
        labels.insert("job".to_string(), "blueprint".to_string());
        labels.insert("environment".to_string(), "production".to_string());

        Ok(Self {
            base_url,
            client,
            labels,
        })
    }

    /// Push log entries to Loki
    pub async fn push_logs(&self, entries: Vec<LogEntry>) -> Result<()> {
        if entries.is_empty() {
            return Ok(());
        }

        let num_entries = entries.len();
        let streams = self.entries_to_streams(entries);
        let push_request = PushRequest { streams };

        let url = format!("{}/loki/api/v1/push", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&push_request)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to push logs to Loki: {e}")))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Other(format!("Loki push failed: {error_text}")));
        }

        debug!("Successfully pushed {} log entries to Loki", num_entries);
        Ok(())
    }

    /// Query logs from Loki
    pub async fn query_logs(
        &self,
        query: &str,
        start: Option<i64>,
        end: Option<i64>,
        limit: Option<u32>,
    ) -> Result<Vec<LogEntry>> {
        let url = format!("{}/loki/api/v1/query_range", self.base_url);

        let mut params = vec![
            ("query".to_string(), query.to_string()),
            ("limit".to_string(), limit.unwrap_or(1000).to_string()),
        ];

        if let Some(start) = start {
            params.push(("start".to_string(), start.to_string()));
        }
        if let Some(end) = end {
            params.push(("end".to_string(), end.to_string()));
        }

        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to query Loki: {e}")))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Other(format!("Loki query failed: {error_text}")));
        }

        let query_response: QueryResponse = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse Loki response: {e}")))?;

        Ok(self.parse_query_response(query_response))
    }

    /// Convert log entries to Loki streams format
    fn entries_to_streams(&self, entries: Vec<LogEntry>) -> Vec<Stream> {
        let mut streams_map: HashMap<String, Vec<[String; 2]>> = HashMap::new();

        for entry in entries {
            let mut labels = self.labels.clone();
            labels.insert("service_id".to_string(), entry.service_id.clone());
            labels.insert(
                "level".to_string(),
                format!("{:?}", entry.level).to_lowercase(),
            );

            if let Some(container_id) = &entry.container_id {
                labels.insert("container_id".to_string(), container_id.clone());
            }

            // Add metadata as labels (limit to important ones)
            for (key, value) in entry.metadata.iter().take(5) {
                labels.insert(key.clone(), value.clone());
            }

            let labels_str = format_labels(&labels);
            let timestamp = entry
                .timestamp
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
                .to_string();

            streams_map
                .entry(labels_str)
                .or_default()
                .push([timestamp, entry.message]);
        }

        streams_map
            .into_iter()
            .map(|(stream, values)| Stream { stream, values })
            .collect()
    }

    /// Parse Loki query response into log entries
    fn parse_query_response(&self, response: QueryResponse) -> Vec<LogEntry> {
        let mut entries = Vec::new();

        if let Some(result) = response.data.result.first() {
            for value in &result.values {
                if value.len() >= 2 {
                    let timestamp_ns: i64 = value[0].parse().unwrap_or(0);
                    let message = value[1].clone();

                    let timestamp = std::time::UNIX_EPOCH
                        + std::time::Duration::from_nanos(timestamp_ns as u64);

                    let mut metadata = HashMap::new();
                    for (key, value) in &result.stream {
                        if key != "service_id" && key != "level" && key != "container_id" {
                            metadata.insert(key.clone(), value.clone());
                        }
                    }

                    entries.push(LogEntry {
                        timestamp,
                        service_id: result
                            .stream
                            .get("service_id")
                            .cloned()
                            .unwrap_or_else(|| "unknown".to_string()),
                        container_id: result.stream.get("container_id").cloned(),
                        level: result
                            .stream
                            .get("level")
                            .map(|s| LogLevel::from(s.as_str()))
                            .unwrap_or(LogLevel::Info),
                        message,
                        metadata,
                    });
                }
            }
        }

        entries
    }

    /// Set up Loki for local development with Docker
    pub async fn setup_local_loki() -> Result<()> {
        info!("Setting up local Loki instance");

        // Check if Loki is already running
        let output = tokio::process::Command::new("docker")
            .args(["ps", "--filter", "name=loki", "--format", "{{.Names}}"])
            .output()
            .await
            .map_err(|e| Error::Other(format!("Failed to check Docker: {e}")))?;

        if String::from_utf8_lossy(&output.stdout).contains("loki") {
            info!("Loki is already running");
            return Ok(());
        }

        // Start Loki container
        let output = tokio::process::Command::new("docker")
            .args([
                "run",
                "-d",
                "--name",
                "loki",
                "-p",
                "3100:3100",
                "-v",
                "/tmp/loki:/loki",
                LOKI_IMAGE_NAME_FULL,
                "-config.file=/etc/loki/local-config.yaml",
            ])
            .output()
            .await
            .map_err(|e| Error::Other(format!("Failed to start Loki: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.contains("already in use") {
                return Err(Error::Other(format!("Failed to start Loki: {stderr}")));
            }
        }

        info!("Loki started successfully on port 3100");

        // Optional: Start Grafana for visualization
        let _ = tokio::process::Command::new("docker")
            .args([
                "run",
                "-d",
                "--name",
                "grafana",
                "-p",
                "3000:3000",
                "--link",
                "loki:loki",
                GRAFANA_IMAGE_NAME_FULL,
            ])
            .output()
            .await;

        info!("Grafana started on port 3000 (admin/admin)");
        Ok(())
    }
}

/// Format labels for Loki stream
fn format_labels(labels: &HashMap<String, String>) -> String {
    let mut parts: Vec<String> = labels
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v.replace('"', "\\\"")))
        .collect();
    parts.sort();
    format!("{{{}}}", parts.join(","))
}

/// Loki push request format
#[derive(Debug, Serialize)]
struct PushRequest {
    streams: Vec<Stream>,
}

#[derive(Debug, Serialize)]
struct Stream {
    stream: String,
    values: Vec<[String; 2]>,
}

/// Loki query response format
#[derive(Debug, Deserialize)]
struct QueryResponse {
    data: QueryData,
}

#[derive(Debug, Deserialize)]
struct QueryData {
    result: Vec<QueryResult>,
}

#[derive(Debug, Deserialize)]
struct QueryResult {
    stream: HashMap<String, String>,
    values: Vec<Vec<String>>,
}

/// Log aggregation pipeline for continuous streaming to Loki
pub struct LogAggregationPipeline {
    loki_client: LokiClient,
    buffer: Vec<LogEntry>,
    buffer_size: usize,
    flush_interval: std::time::Duration,
}

impl LogAggregationPipeline {
    pub fn new(loki_url: String, buffer_size: usize) -> Result<Self> {
        Ok(Self {
            loki_client: LokiClient::new(loki_url)?,
            buffer: Vec::with_capacity(buffer_size),
            buffer_size,
            flush_interval: std::time::Duration::from_secs(10),
        })
    }

    /// Add log entry to buffer
    pub async fn add_entry(&mut self, entry: LogEntry) -> Result<()> {
        self.buffer.push(entry);

        if self.buffer.len() >= self.buffer_size {
            self.flush().await?;
        }

        Ok(())
    }

    /// Flush buffered logs to Loki
    pub async fn flush(&mut self) -> Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let entries: Vec<LogEntry> = std::mem::take(&mut self.buffer);
        self.loki_client.push_logs(entries).await?;
        Ok(())
    }

    /// Start background flush task
    pub fn start_background_flush(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.flush_interval);
            loop {
                interval.tick().await;
                if let Err(e) = self.flush().await {
                    error!("Failed to flush logs to Loki: {}", e);
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_labels() {
        let mut labels = HashMap::new();
        labels.insert("job".to_string(), "test".to_string());
        labels.insert("env".to_string(), "prod".to_string());

        let formatted = format_labels(&labels);
        assert!(formatted.contains("env=\"prod\""));
        assert!(formatted.contains("job=\"test\""));
        assert!(formatted.starts_with('{'));
        assert!(formatted.ends_with('}'));
    }

    #[tokio::test]
    async fn test_loki_client_creation() {
        let client = LokiClient::new("http://localhost:3100".to_string());
        assert!(client.is_ok());
    }
}
