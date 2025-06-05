use blueprint_core::{debug, error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{Error, Result};

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

// Specific jsonData structs
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusJsonData {
    pub http_method: String, // e.g., "POST"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    pub disable_metrics_lookup: bool, // Added to control metrics lookup
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LokiJsonData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<u32>,
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
        let url = format!("{}/api/folders", self.config.url);

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
                        "blueprint_cpu_usage{{service_id=\\\"{}\\\",blueprint_id=\\\"{}\\\"}}",
                        service_id, blueprint_id
                    ),
                    datasource: None,
                },
                Target {
                    ref_id: "B".to_string(),
                    expr: format!(
                        "blueprint_memory_usage{{service_id=\\\"{}\\\",blueprint_id=\\\"{}\\\"}}",
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
                    expr: "up{job=\"prometheus\"}".to_string(),
                    datasource: None,
                },
                Target {
                    ref_id: "B".to_string(),
                    expr: format!(
                        "blueprint_job_errors{{service_id=\\\"{}\\\",blueprint_id=\\\"{}\\\"}}",
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
                    "blueprint_last_heartbeat{{service_id=\\\"{}\\\",blueprint_id=\\\"{}\\\"}}",
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
                    "blueprint_status_code{{service_id=\\\"{}\\\",blueprint_id=\\\"{}\\\"}}",
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
                    "blueprint_uptime{{service_id=\\\"{}\\\",blueprint_id=\\\"{}\\\"}}",
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

    /// Updates the URL for an existing Prometheus datasource in Grafana
    ///
    /// # Errors
    /// Returns an error if the datasource update fails or the datasource doesn't exist
    pub async fn update_datasource_url(
        &self,
        datasource_uid: &str,
        new_url: &str,
    ) -> Result<Option<String>> {
        info!(
            "Updating Grafana datasource {} URL to {}",
            datasource_uid, new_url
        );

        // First, get the datasource by UID to preserve all other settings
        let url = format!("{}/api/datasources/uid/{}", self.config.url, datasource_uid);
        let mut request_builder = self.client.get(&url);

        if !self.config.api_key.is_empty() {
            request_builder = request_builder.bearer_auth(self.config.api_key.trim());
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            request_builder = request_builder.basic_auth(user, Some(pass.as_str()));
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| Error::GrafanaApi(format!("Failed to get datasource: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::GrafanaApi(format!(
                "Failed to get datasource {}: {}",
                datasource_uid, error_text
            )));
        }

        // Parse the existing datasource
        let mut datasource: serde_json::Value = response.json().await.map_err(|e| {
            Error::GrafanaApi(format!("Failed to parse datasource response: {}", e))
        })?;

        // Update the URL
        datasource["url"] = serde_json::Value::String(new_url.to_string());

        // Convert to a proper request for update
        let payload = CreateDataSourceRequest {
            name: datasource["name"]
                .as_str()
                .unwrap_or("Prometheus")
                .to_string(),
            ds_type: datasource["type"]
                .as_str()
                .unwrap_or("prometheus")
                .to_string(),
            url: new_url.to_string(),
            access: datasource["access"].as_str().unwrap_or("proxy").to_string(),
            uid: Some(datasource_uid.to_string()),
            is_default: datasource["isDefault"].as_bool(),
            json_data: None,
        };

        // Update the datasource
        match self.create_or_update_datasource(payload).await {
            Ok(response) => {
                info!(
                    "Successfully updated datasource {} URL to {}",
                    datasource_uid, new_url
                );
                Ok(Some(format!(
                    "Datasource {} URL updated to {}",
                    response.name, new_url
                )))
            }
            Err(e) => {
                error!("Failed to update datasource URL: {}", e);
                Err(e)
            }
        }
    }

    pub async fn check_datasource_health(&self, uid: &str) -> Result<DatasourceHealthResponse> {
        let path = format!("api/datasources/uid/{}/health", uid);
        let url = format!("{}/{}", self.config.url.trim_end_matches('/'), path);

        debug!(target: "blueprint_qos::logging::grafana", "Performing health check for datasource UID {} at URL: {}", uid, url);

        let mut request_builder = self.client.get(&url); // Use self.client

        if !self.config.api_key.is_empty() {
            // Check if api_key string is non-empty
            request_builder = request_builder.bearer_auth(self.config.api_key.trim());
            debug!(target: "blueprint_qos::logging::grafana", "Health check for datasource UID {}: Using API Key auth", uid);
        } else if let (Some(username), Some(password)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            // Fallback to basic auth if API key is empty string AND basic auth creds are present
            request_builder = request_builder.basic_auth(username, Some(password));
            debug!(target: "blueprint_qos::logging::grafana", "Health check for datasource UID {}: Using Basic auth (API key was empty) for user {}", uid, username);
        } else {
            warn!(target: "blueprint_qos::logging::grafana", "Health check for datasource UID {}: No authentication configured (API key empty, no basic auth). This is unlikely to succeed.", uid);
        }

        let response = match request_builder.send().await {
            Ok(res) => res,
            Err(e) => {
                error!(target: "blueprint_qos::logging::grafana", "Health check request for datasource UID {} to {} failed: {}", uid, url, e);
                return Err(Error::GrafanaApi(format!(
                    "HTTP request to {} failed: {}",
                    url, e
                )));
            }
        };

        let response_status = response.status();
        let response_text = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                error!(target: "blueprint_qos::logging::grafana", "Failed to read response body for UID {} (status {}): {}", uid, response_status, e);
                return Err(Error::GrafanaApi(format!(
                    "Failed to read response body for UID {} (status {}): {}",
                    uid, response_status, e
                )));
            }
        };

        if response_status.is_success() {
            match serde_json::from_str::<DatasourceHealthResponse>(&response_text) {
                Ok(health_response) => {
                    debug!(target: "blueprint_qos::logging::grafana", "Health check for datasource UID {} successful: Status {}, Message: {}. Body: {}", uid, health_response.status, health_response.message, response_text);
                    Ok(health_response)
                }
                Err(e) => {
                    error!(target: "blueprint_qos::logging::grafana", "Failed to parse successful health check response for UID {} from body '{}': {}", uid, response_text, e);
                    Err(Error::Json(format!(
                        "Failed to parse health check response for UID {} from body '{}': {}",
                        uid, response_text, e
                    )))
                }
            }
        } else {
            error!(target: "blueprint_qos::logging::grafana", "Health check for datasource UID {} failed with status {}. Body: {}", uid, response_status, response_text);

            match serde_json::from_str::<GrafanaApiErrorBody>(&response_text) {
                Ok(api_err) => Err(Error::GrafanaApi(format!(
                    "Grafana API error ({}) during health check: {}. UID: {}. Full Body: {}",
                    response_status, api_err.message, uid, response_text
                ))),
                Err(_) => Err(Error::GrafanaApi(format!(
                    "Grafana API request failed with status {} for UID {} during health check. Full Body: {}",
                    response_status, uid, response_text
                ))),
            }
        }
    }

    pub async fn create_or_update_datasource(
        &self,
        payload: CreateDataSourceRequest,
    ) -> Result<CreateDataSourceResponse> {
        let url = format!("{}/api/datasources", self.config.url);
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
        };

        let _client = GrafanaClient::new(config.clone());
    }
}
