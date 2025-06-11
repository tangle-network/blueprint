use blueprint_core::{debug, error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::loki::LokiConfig;
use crate::error::{Error, Result}; // Adjusted import path assuming loki.rs is in the same logging module

// Health check response structures
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceHealthDetails {
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceHealthResponse {
    pub message: String,
    pub status: String, // "OK", "ERROR", etc.
    pub details: Option<DatasourceHealthDetails>,
}

// For parsing generic Grafana JSON error responses
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrafanaApiErrorBody {
    pub message: String,
    #[serde(alias = "statusCode")]
    pub status_code: Option<u16>,
    #[serde(alias = "messageId")]
    pub error_code: Option<String>,
    pub trace_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LokiJsonData {
    pub max_lines: Option<u32>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusJsonData {
    pub http_method: String,
    pub timeout: u32,
}

/// Configuration for Grafana
#[derive(Clone, Debug)]
pub struct GrafanaConfig {
    /// Grafana server URL
    pub url: String,

    /// API key for authentication (preferred)
    pub api_key: String,

    /// Admin username for basic authentication (fallback)
    pub admin_user: Option<String>,

    /// Admin password for basic authentication (fallback)
    pub admin_password: Option<String>,

    /// Default organization ID
    pub org_id: Option<u64>,

    /// Default dashboard folder
    pub folder: Option<String>,

    /// Configuration for the Loki datasource, if Grafana is expected to use one.
    pub loki_config: Option<LokiConfig>,

    /// The URL for the Prometheus datasource that Grafana should use.
    /// If not provided, a default may be assumed.
    pub prometheus_datasource_url: Option<String>,
}

impl Default for GrafanaConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:3000".to_string(),
            api_key: String::new(),
            admin_user: None,
            admin_password: None,
            org_id: None,
            folder: None,
            loki_config: None,
            prometheus_datasource_url: Some("http://localhost:9090".to_string()),
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

    /// Dashboard URL
    url: String,

    /// Dashboard status
    status: String,

    /// Dashboard version
    version: u64,
}

// Helper for parsing Grafana API error responses
#[derive(Deserialize, Debug)]
struct GrafanaApiError {
    message: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDataSourceRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub ds_type: String,
    pub url: String,
    pub access: String, // "proxy" or "direct"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_data: Option<serde_json::Value>,
    // secure_json_data can be added if needed for auth later
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDataSourceResponse {
    pub id: u64,
    // pub uid: String, // Removed: UID is nested in datasource_config
    pub name: String,
    pub message: String,
    #[serde(rename = "datasource")]
    // Ensure this matches the previous working version if it was datasource_config
    pub datasource: DataSourceDetails, // Renamed from datasource_config for consistency if needed, or keep as datasource_config if that was correct
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceDetails {
    pub id: u64,
    pub uid: String,
    pub org_id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub ds_type: String,
    pub type_logo_url: String,
    pub access: String,
    pub url: String,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_data: Option<serde_json::Value>,
    pub version: u64,
    pub read_only: bool,
}

impl GrafanaClient {
    /// Returns the configured Prometheus datasource URL, if any.
    pub fn prometheus_datasource_url(&self) -> Option<&String> {
        self.config.prometheus_datasource_url.as_ref()
    }

    /// Create a new Grafana client
    #[must_use]
    pub fn new(config: GrafanaConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client, config }
    }

    /// Returns a reference to the Grafana client's configuration.
    pub fn config(&self) -> &GrafanaConfig {
        &self.config
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
        let url = format!(
            "{}/api/dashboards/db",
            self.config.url.trim_end_matches('/')
        );

        let request = DashboardCreateRequest {
            dashboard,
            folder_id: folder_id.or(self.config.org_id),
            folder_uid: None,
            message: message.to_string(),
            overwrite: true,
        };

        let dashboard_payload_json = serde_json::to_string_pretty(&request)
            .unwrap_or_else(|e| format!("Failed to serialize dashboard request payload: {}", e));
        info!(
            "Grafana create_dashboard payload:\n{}",
            dashboard_payload_json
        );

        let mut request_builder = self.client.post(&url);

        if !self.config.api_key.is_empty() {
            request_builder = request_builder.header(
                "Authorization",
                format!("Bearer {}", self.config.api_key.trim()),
            );
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            request_builder = request_builder.basic_auth(user, Some(pass.as_str()));
        }

        let response = request_builder
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
        let url = format!("{}/api/folders", self.config.url.trim_end_matches('/'));

        let mut request = HashMap::new();
        request.insert("title", title.to_string());

        if let Some(uid) = uid {
            request.insert("uid", uid.to_string());
        }

        let mut request_builder = self.client.post(&url);

        if !self.config.api_key.is_empty() {
            request_builder = request_builder.header(
                "Authorization",
                format!("Bearer {}", self.config.api_key.trim()),
            );
        } else if let (Some(username), Some(password)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            request_builder = request_builder.basic_auth(username, Some(password.as_str()));
        }

        let response = request_builder
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
                        "otel_job_executions_total{{service_id=\"{}\",blueprint_id=\"{}\"}}",
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

        // Add custom test metrics panel
        let test_metrics_panel = Panel {
            id: Some(7),
            title: "Custom Test Metrics".to_string(),
            panel_type: "timeseries".to_string(),
            datasource: Some(DataSource {
                ds_type: "prometheus".to_string(),
                uid: prometheus_datasource.to_string(),
            }),
            grid_pos: GridPos {
                x: 0,
                y: 20,
                w: 24,
                h: 8,
            },
            targets: vec![
                Target {
                    ref_id: "A".to_string(),
                    expr: "test_blueprint_job_executions".to_string(),
                    datasource: None,
                },
                Target {
                    ref_id: "B".to_string(),
                    expr: "test_blueprint_job_success".to_string(),
                    datasource: None,
                },
                Target {
                    ref_id: "C".to_string(),
                    expr: "test_blueprint_job_latency_ms".to_string(),
                    datasource: None,
                },
            ],
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
        dashboard.panels.push(test_metrics_panel);

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
        self.create_dashboard(dashboard, folder_id, "Create Blueprint Dashboard")
            .await
    }

    /// Performs a health check for a Grafana datasource.
    ///
    /// # Errors
    /// Returns an error if the health check fails or the API request is unsuccessful.
    pub async fn check_datasource_health(&self, uid: &str) -> Result<DatasourceHealthResponse> {
        let url = format!(
            "{}/api/datasources/uid/{}/health",
            self.config.url.trim_end_matches('/'),
            uid
        );
        debug!(
            "Performing health check for datasource UID {} at URL: {}",
            uid, url
        );

        let mut request_builder = self.client.get(&url);

        if !self.config.api_key.is_empty() {
            request_builder = request_builder.bearer_auth(self.config.api_key.trim());
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            request_builder = request_builder.basic_auth(user, Some(pass.as_str()));
        }

        let response = request_builder.send().await.map_err(|e| {
            Error::GrafanaApi(format!(
                "Failed to send health check request for UID {}: {}",
                uid, e
            ))
        })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| {
            Error::GrafanaApi(format!(
                "Failed to read health check response body for UID {}: {}",
                uid, e
            ))
        })?;

        if status.is_success() {
            serde_json::from_str::<DatasourceHealthResponse>(&response_text).map_err(|e| {
                Error::GrafanaApi(format!(
                    "Failed to parse health check response for UID {}: {}. Body: {}",
                    uid, e, response_text
                ))
            })
        } else {
            Err(Error::GrafanaApi(format!(
                "Health check for UID {} failed with status {}. Body: {}",
                uid, status, response_text
            )))
        }
    }

    /// # Errors
    /// Returns an error if the Grafana API request fails or returns an error response
    pub async fn get_datasource(&self, uid: &str) -> Result<Option<DataSourceDetails>> {
        let url = format!(
            "{}/api/datasources/uid/{}",
            self.config.url.trim_end_matches('/'),
            uid
        );
        debug!("Getting datasource UID {} at URL: {}", uid, url);

        let mut request_builder = self.client.get(&url);

        if !self.config.api_key.is_empty() {
            request_builder = request_builder.bearer_auth(self.config.api_key.trim());
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            request_builder = request_builder.basic_auth(user, Some(pass.as_str()));
        }

        let response = request_builder.send().await.map_err(|e| {
            Error::GrafanaApi(format!(
                "Failed to send get datasource request for UID {}: {}",
                uid, e
            ))
        })?;

        let status = response.status();
        if status.is_success() {
            let ds = response.json::<DataSourceDetails>().await.map_err(|e| {
                Error::GrafanaApi(format!(
                    "Failed to parse datasource response for UID {}: {}",
                    uid, e
                ))
            })?;
            Ok(Some(ds))
        } else if status == reqwest::StatusCode::NOT_FOUND {
            Ok(None)
        } else {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::GrafanaApi(format!(
                "Failed to get datasource UID {}. Status: {}. Body: {}",
                uid, status, error_body
            )))
        }
    }

    pub async fn create_or_update_datasource(
        &self,
        payload: CreateDataSourceRequest,
    ) -> Result<CreateDataSourceResponse> {
        let url = format!("{}/api/datasources", self.config.url.trim_end_matches('/'));
        info!(
            "Attempting to create/update Grafana datasource: {} (UID: {:?}) at URL: {}",
            payload.name, payload.uid, payload.url
        );

        let mut request_builder = self.client.post(&url);
        if !self.config.api_key.is_empty() {
            request_builder = request_builder.bearer_auth(self.config.api_key.trim());
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            request_builder = request_builder.basic_auth(user, Some(pass.as_str()));
        }

        let response = request_builder.json(&payload).send().await.map_err(|e| {
            Error::GrafanaApi(format!("Failed to send create datasource request: {}", e))
        })?;

        if response.status().is_success() {
            let response_text = response.text().await.map_err(|e| {
                Error::GrafanaApi(format!(
                    "Failed to read success response body as text: {}",
                    e
                ))
            })?;
            info!(
                "Grafana datasource creation/update successful. Raw response body: {}",
                response_text
            );

            let response_body: CreateDataSourceResponse = serde_json::from_str(&response_text)
                .map_err(|e| {
                    Error::GrafanaApi(format!(
                        "Failed to parse create datasource response from text ({}): {}",
                        response_text, e
                    ))
                })?;
            info!(
                "Successfully created/updated Grafana datasource: {} (ID: {}, UID: {})",
                response_body.name, response_body.id, response_body.datasource.uid
            );
            Ok(response_body)
        } else {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!(
                "Failed to create/update Grafana datasource. Status: {}. Body: {}",
                status, error_body
            );
            // Attempt to parse Grafana's specific error message format
            let grafana_error_message = serde_json::from_str::<GrafanaApiError>(&error_body)
                .map(|e| e.message)
                .unwrap_or_else(|_| error_body.clone());

            Err(Error::GrafanaApi(format!(
                "Grafana API error ({}) creating/updating datasource '{}': {}",
                status, payload.name, grafana_error_message
            )))
        }
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
        assert_eq!(
            config.prometheus_datasource_url.unwrap(),
            "http://localhost:9090"
        );
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
            admin_user: Some("admin".to_string()),
            admin_password: Some("password".to_string()),
            org_id: Some(1),
            folder: None,
            loki_config: None,
            prometheus_datasource_url: Some("http://localhost:9090".to_string()),
        };

        let _client = GrafanaClient::new(config.clone());
    }
}
