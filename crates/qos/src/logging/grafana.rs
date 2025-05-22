use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

use crate::error::{Error, Result};

/// Configuration for Grafana
#[derive(Clone, Debug)]
pub struct GrafanaConfig {
    /// Grafana server URL
    pub url: String,

    /// API key for authentication
    pub api_key: String,

    /// Default organization ID
    pub org_id: Option<u64>,

    /// Default dashboard folder
    pub folder: Option<String>,
}

impl Default for GrafanaConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:3000".to_string(),
            api_key: String::new(),
            org_id: None,
            folder: None,
        }
    }
}

/// Grafana client for managing dashboards
pub struct GrafanaClient {
    /// HTTP client
    client: Client,

    /// Configuration
    config: GrafanaConfig,
}

/// Dashboard model for Grafana
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dashboard {
    /// Dashboard ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    /// Dashboard UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Dashboard title
    pub title: String,

    /// Dashboard tags
    #[serde(default)]
    pub tags: Vec<String>,

    /// Dashboard timezone
    #[serde(default = "default_timezone")]
    pub timezone: String,

    /// Dashboard refresh interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,

    /// Dashboard schema version
    #[serde(default = "default_schema_version")]
    pub schema_version: u64,

    /// Dashboard version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,

    /// Dashboard panels
    #[serde(default)]
    pub panels: Vec<Panel>,
}

/// Default timezone for dashboards
fn default_timezone() -> String {
    "browser".to_string()
}

/// Default schema version for dashboards
fn default_schema_version() -> u64 {
    36
}

/// Panel model for Grafana dashboards
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Panel {
    /// Panel ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    /// Panel title
    pub title: String,

    /// Panel type
    #[serde(rename = "type")]
    pub panel_type: String,

    /// Panel datasource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource: Option<DataSource>,

    /// Panel grid position
    pub grid_pos: GridPos,

    /// Panel targets (queries)
    #[serde(default)]
    pub targets: Vec<Target>,

    /// Panel options
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,

    /// Panel field config
    #[serde(default)]
    pub field_config: FieldConfig,
}

/// Data source model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataSource {
    /// Data source type
    #[serde(rename = "type")]
    pub ds_type: String,

    /// Data source UID
    pub uid: String,
}

/// Grid position model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GridPos {
    /// X position
    pub x: u64,

    /// Y position
    pub y: u64,

    /// Width
    pub w: u64,

    /// Height
    pub h: u64,
}

/// Target model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Target {
    /// Target reference ID
    pub ref_id: String,

    /// Target expression (query)
    pub expr: String,

    /// Target data source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasource: Option<DataSource>,
}

/// Field config model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FieldConfig {
    /// Default field config
    #[serde(default)]
    pub defaults: FieldDefaults,

    /// Field config overrides
    #[serde(default)]
    pub overrides: Vec<FieldOverride>,
}

/// Field defaults model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FieldDefaults {
    /// Field unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// Field decimals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<u64>,

    /// Field min value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// Field max value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    /// Field thresholds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thresholds: Option<Thresholds>,

    /// Field color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<HashMap<String, serde_json::Value>>,
}

/// Field override model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldOverride {
    /// Override matcher
    pub matcher: Matcher,

    /// Override properties
    pub properties: Vec<Property>,
}

/// Matcher model for Grafana field overrides
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Matcher {
    /// Matcher ID
    pub id: String,

    /// Matcher options
    pub options: serde_json::Value,
}

/// Property model for Grafana field overrides
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Property {
    /// Property ID
    pub id: String,

    /// Property value
    pub value: serde_json::Value,
}

/// Thresholds model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Thresholds {
    /// Threshold mode
    pub mode: String,

    /// Threshold steps
    pub steps: Vec<ThresholdStep>,
}

/// Threshold step model for Grafana panels
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThresholdStep {
    /// Step color
    pub color: String,

    /// Step value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// Dashboard creation request for Grafana API
#[derive(Serialize, Deserialize, Clone, Debug)]
struct DashboardCreateRequest {
    /// Dashboard
    dashboard: Dashboard,

    /// Folder ID
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_id: Option<u64>,

    /// Folder UID
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_uid: Option<String>,

    /// Message
    message: String,

    /// Overwrite
    overwrite: bool,
}

/// Dashboard creation response from Grafana API
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct DashboardCreateResponse {
    /// Dashboard ID
    id: u64,

    /// Dashboard UID
    uid: String,

    /// Dashboard URL
    url: String,

    /// Dashboard status
    status: String,

    /// Dashboard version
    version: u64,
}

impl GrafanaClient {
    /// Create a new Grafana client
    #[must_use]
    pub fn new(config: GrafanaConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client, config }
    }

    /// Create or update a Grafana dashboard
    ///
    /// # Errors
    /// Returns an error if the Grafana API request fails or returns an error response
    pub async fn create_dashboard(
        &self,
        dashboard: Dashboard,
        folder_id: Option<u64>,
        message: &str,
    ) -> Result<String> {
        let url = format!("{}/api/dashboards/db", self.config.url);

        let request = DashboardCreateRequest {
            dashboard,
            folder_id: folder_id.or(self.config.org_id),
            folder_uid: None,
            message: message.to_string(),
            overwrite: true,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to create dashboard: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::Other(format!(
                "Failed to create dashboard: {}",
                error_text
            )));
        }

        let dashboard_response: DashboardCreateResponse = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse dashboard response: {}", e)))?;

        info!("Created dashboard: {}", dashboard_response.url);

        Ok(dashboard_response.url)
    }

    /// Create a folder in Grafana
    ///
    /// # Errors
    /// Returns an error if the Grafana API request fails or returns an error response
    pub async fn create_folder(&self, title: &str, uid: Option<&str>) -> Result<u64> {
        let url = format!("{}/api/folders", self.config.url);

        let mut request = HashMap::new();
        request.insert("title", title.to_string());

        if let Some(uid) = uid {
            request.insert("uid", uid.to_string());
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to create folder: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::Other(format!(
                "Failed to create folder: {}",
                error_text
            )));
        }

        let folder: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse folder response: {}", e)))?;

        let folder_id = folder["id"]
            .as_u64()
            .ok_or_else(|| Error::Other("Failed to get folder ID".to_string()))?;

        info!("Created folder: {} (ID: {})", title, folder_id);

        Ok(folder_id)
    }

    /// Create a blueprint dashboard in Grafana
    ///
    /// # Errors
    /// Returns an error if the dashboard creation fails or if the Grafana API returns an error
    pub async fn create_blueprint_dashboard(
        &self,
        service_id: u64,
        blueprint_id: u64,
        prometheus_datasource: &str,
        loki_datasource: &str,
    ) -> Result<String> {
        // Create a dashboard for the Blueprint
        let mut dashboard = Dashboard {
            id: None,
            uid: Some(format!("blueprint-{}-{}", service_id, blueprint_id)),
            title: format!("Blueprint Service {} - {}", service_id, blueprint_id),
            tags: vec!["blueprint".to_string(), "tangle".to_string()],
            timezone: "browser".to_string(),
            refresh: Some("10s".to_string()),
            schema_version: 36,
            version: None,
            panels: Vec::new(),
        };

        // Add system metrics panel
        let system_metrics_panel = Panel {
            id: Some(1),
            title: "System Metrics".to_string(),
            panel_type: "timeseries".to_string(),
            datasource: Some(DataSource {
                ds_type: "prometheus".to_string(),
                uid: prometheus_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 0,
                y: 0,
                w: 12,
                h: 8,
            },
            targets: vec![
                Target {
                    ref_id: "A".to_string(),
                    expr: format!(
                        "blueprint_cpu_usage{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                        service_id, blueprint_id
                    ),
                    datasource: None,
                },
                Target {
                    ref_id: "B".to_string(),
                    expr: format!(
                        "blueprint_memory_usage{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                        service_id, blueprint_id
                    ),
                    datasource: None,
                },
            ],
            options: HashMap::new(),
            field_config: FieldConfig::default(),
        };

        // Add job metrics panel
        let job_metrics_panel = Panel {
            id: Some(2),
            title: "Job Executions".to_string(),
            panel_type: "timeseries".to_string(),
            datasource: Some(DataSource {
                ds_type: "prometheus".to_string(),
                uid: prometheus_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 12,
                y: 0,
                w: 12,
                h: 8,
            },
            targets: vec![
                Target {
                    ref_id: "A".to_string(),
                    expr: format!(
                        "blueprint_job_executions{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                        service_id, blueprint_id
                    ),
                    datasource: None,
                },
                Target {
                    ref_id: "B".to_string(),
                    expr: format!(
                        "blueprint_job_errors{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                        service_id, blueprint_id
                    ),
                    datasource: None,
                },
            ],
            options: HashMap::new(),
            field_config: FieldConfig::default(),
        };

        // Add logs panel
        let logs_panel = Panel {
            id: Some(3),
            title: "Logs".to_string(),
            panel_type: "logs".to_string(),
            datasource: Some(DataSource {
                ds_type: "loki".to_string(),
                uid: loki_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 0,
                y: 8,
                w: 24,
                h: 8,
            },
            targets: vec![Target {
                ref_id: "A".to_string(),
                expr: format!(
                    "{{service=\"blueprint\",service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            }],
            options: HashMap::new(),
            field_config: FieldConfig::default(),
        };

        // Add heartbeat panel
        let heartbeat_panel = Panel {
            id: Some(4),
            title: "Heartbeats".to_string(),
            panel_type: "stat".to_string(),
            datasource: Some(DataSource {
                ds_type: "prometheus".to_string(),
                uid: prometheus_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 0,
                y: 16,
                w: 8,
                h: 4,
            },
            targets: vec![Target {
                ref_id: "A".to_string(),
                expr: format!(
                    "blueprint_last_heartbeat{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            }],
            options: HashMap::new(),
            field_config: FieldConfig::default(),
        };

        // Add status panel
        let status_panel = Panel {
            id: Some(5),
            title: "Status".to_string(),
            panel_type: "stat".to_string(),
            datasource: Some(DataSource {
                ds_type: "prometheus".to_string(),
                uid: prometheus_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 8,
                y: 16,
                w: 8,
                h: 4,
            },
            targets: vec![Target {
                ref_id: "A".to_string(),
                expr: format!(
                    "blueprint_status_code{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            }],
            options: HashMap::new(),
            field_config: FieldConfig::default(),
        };

        // Add uptime panel
        let uptime_panel = Panel {
            id: Some(6),
            title: "Uptime".to_string(),
            panel_type: "stat".to_string(),
            datasource: Some(DataSource {
                ds_type: "prometheus".to_string(),
                uid: prometheus_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 16,
                y: 16,
                w: 8,
                h: 4,
            },
            targets: vec![Target {
                ref_id: "A".to_string(),
                expr: format!(
                    "blueprint_uptime{{service_id=\"{}\",blueprint_id=\"{}\"}}",
                    service_id, blueprint_id
                ),
                datasource: None,
            }],
            options: HashMap::new(),
            field_config: FieldConfig::default(),
        };

        // Add panels to dashboard
        dashboard.panels.push(system_metrics_panel);
        dashboard.panels.push(job_metrics_panel);
        dashboard.panels.push(logs_panel);
        dashboard.panels.push(heartbeat_panel);
        dashboard.panels.push(status_panel);
        dashboard.panels.push(uptime_panel);

        // Create folder if needed
        let folder_id = if let Some(folder) = &self.config.folder {
            match self.create_folder(folder, None).await {
                Ok(id) => Some(id),
                Err(e) => {
                    error!("Failed to create folder: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Create dashboard
        self.create_dashboard(dashboard, folder_id, "Created by Blueprint QoS")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `GrafanaConfig` default implementation returns a valid configuration.
    ///
    /// ```
    /// GrafanaConfig::default() -> Valid config
    /// ```
    ///
    /// Expected outcome: Default config has reasonable values
    #[test]
    fn test_grafana_config_default() {
        let config = GrafanaConfig::default();
        assert_eq!(config.url, "http://localhost:3000");
        assert_eq!(config.api_key, "");
        assert_eq!(config.org_id, None);
        assert_eq!(config.folder, None);
    }

    /// Tests that a new `GrafanaClient` can be created with a valid configuration.
    ///
    /// ```
    /// GrafanaConfig -> GrafanaClient
    /// ```
    ///
    /// Expected outcome: `GrafanaClient` is created with the provided config
    #[test]
    fn test_grafana_client_creation() {
        let config = GrafanaConfig {
            url: "http://localhost:3000".to_string(),
            api_key: "test_key".to_string(),
            org_id: Some(1),
            folder: None,
        };

        let _client = GrafanaClient::new(config.clone());
    }
}
