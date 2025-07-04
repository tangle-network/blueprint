use blueprint_core::{debug, error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::loki::LokiConfig;
use crate::error::{Error, Result};

const DEFAULT_ADMIN_PASSWORD: &str = "please_change_this_default_password";
const DEFAULT_GRAFANA_URL: &str = "http://localhost:3000";
const DEFAULT_GRAFANA_ADMIN_USER: &str = "admin";
const DEFAULT_GRAFANA_PROMETHEUS_DATASOURCE_URL: &str = "http://localhost:9090";

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
    pub status: String,
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

/// Configuration for connecting to and interacting with a Grafana server.
///
/// This structure encapsulates the connection details, authentication credentials,
/// and integration settings needed to communicate with a Grafana instance. It supports
/// both `API` key authentication (preferred) and basic authentication as a fallback.
/// The configuration also includes references to related data sources like `Prometheus` and `Loki`.
#[derive(Clone, Debug)]
pub struct GrafanaConfig {
    /// The base URL for the Grafana server (e.g., "<http://localhost:3000>").
    pub url: String,

    /// API key for Grafana, if used. This is the preferred authentication method.
    pub api_key: Option<String>,

    /// Optional admin username for basic authentication (fallback if API key is not provided).
    pub admin_user: Option<String>,

    /// Optional admin password for basic authentication.
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
            url: DEFAULT_GRAFANA_URL.to_string(),
            api_key: None,
            admin_user: Some(DEFAULT_GRAFANA_ADMIN_USER.to_string()),
            admin_password: Some(DEFAULT_ADMIN_PASSWORD.to_string()),
            org_id: None,
            folder: None,
            loki_config: None,
            prometheus_datasource_url: Some(DEFAULT_GRAFANA_PROMETHEUS_DATASOURCE_URL.to_string()),
        }
    }
}

/// Client for interacting with the Grafana HTTP API.
///
/// This client provides methods to create and manage Grafana resources including
/// dashboards, folders, and data sources. It handles authentication, request formatting,
/// and response parsing for the Grafana API. The client is designed to support Blueprint
/// monitoring by creating pre-configured dashboards that visualize metrics and logs.
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
    pub access: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_data: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDataSourceResponse {
    pub id: u64,
    pub name: String,
    pub message: String,
    #[serde(rename = "datasource")]
    pub datasource: DataSourceDetails,
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
    ///
    /// This method provides access to the Prometheus URL that has been configured
    /// for use with Grafana dashboards. It's used when setting up Prometheus
    /// as a data source for created dashboards.
    #[must_use]
    pub fn prometheus_datasource_url(&self) -> Option<&String> {
        self.config.prometheus_datasource_url.as_ref()
    }

    /// Creates a new Grafana client with the specified configuration.
    ///
    /// Initializes an HTTP client with appropriate authentication headers based on
    /// the provided configuration. The client will use API key authentication if available,
    /// falling back to basic authentication if credentials are provided.
    #[must_use]
    pub fn new(config: GrafanaConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        if config.api_key.as_deref().unwrap_or("").is_empty() {
            if let Some(pass) = &config.admin_password {
                if pass == DEFAULT_ADMIN_PASSWORD {
                    warn!(
                        "GrafanaClient is configured to use basic authentication with the default insecure password. Please change it or provide an API key."
                    );
                }
            }
        }

        Self { client, config }
    }

    /// Returns a reference to the Grafana client's configuration.
    ///
    /// Provides access to the underlying configuration settings that this client
    /// was initialized with, including connection URLs and authentication details.
    #[must_use]
    pub fn config(&self) -> &GrafanaConfig {
        &self.config
    }

    /// Creates or updates a Grafana dashboard.
    ///
    /// This method sends a dashboard configuration to the Grafana API, either creating
    /// a new dashboard or updating an existing one if the dashboard UID already exists.
    /// It handles the proper JSON formatting required by the Grafana API and processes
    /// the response.
    ///
    /// # Parameters
    /// * `dashboard` - The dashboard configuration to create or update
    /// * `folder_id` - Optional folder ID to organize the dashboard in
    /// * `message` - Commit message for the dashboard change
    ///
    /// # Returns
    /// The dashboard URL on success
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

        if let Some(api_key) = &self.config.api_key {
            if !api_key.is_empty() {
                request_builder = request_builder.bearer_auth(api_key);
            }
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            if !user.is_empty() && !pass.is_empty() {
                if pass == DEFAULT_ADMIN_PASSWORD {
                    warn!(
                        "Grafana basic authentication is using the default insecure password. Please change it."
                    );
                }
                request_builder = request_builder.basic_auth(user, Some(pass.clone()));
            }
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

    /// Creates a folder in Grafana for organizing dashboards.
    ///
    /// Folders help organize dashboards in the Grafana UI. This method attempts
    /// to create a new folder with the specified title and optional UID.
    /// If a folder with the same title already exists, it returns the existing
    /// folder's ID rather than creating a duplicate.
    ///
    /// # Parameters
    /// * `title` - The display name for the folder
    /// * `uid` - Optional unique identifier for the folder
    ///
    /// # Returns
    /// The folder ID (either newly created or existing)
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

        if let Some(api_key) = &self.config.api_key {
            if !api_key.is_empty() {
                request_builder = request_builder.bearer_auth(api_key);
            }
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            if !user.is_empty() && !pass.is_empty() {
                if pass == DEFAULT_ADMIN_PASSWORD {
                    warn!(
                        "Grafana basic authentication is using the default insecure password. Please change it."
                    );
                }
                request_builder = request_builder.basic_auth(user, Some(pass.clone()));
            }
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

    /// Creates a pre-configured dashboard for monitoring a Blueprint service.
    ///
    /// Generates a comprehensive dashboard with panels for system metrics,
    /// application metrics, and logs specific to the identified Blueprint service.
    /// The dashboard includes panels for CPU usage, memory usage, job execution metrics,
    /// and log streams from the specified Loki datasource.
    ///
    /// # Parameters
    /// * `service_id` - The service ID to monitor
    /// * `blueprint_id` - The blueprint ID to monitor
    /// * `prometheus_datasource` - Name of the Prometheus datasource to use
    /// * `loki_datasource` - Name of the Loki datasource to use for logs
    ///
    /// # Returns
    /// The URL of the created dashboard
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

        if let Some(api_key) = &self.config.api_key {
            if !api_key.is_empty() {
                request_builder = request_builder.bearer_auth(api_key);
            }
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            if !user.is_empty() && !pass.is_empty() {
                if pass == DEFAULT_ADMIN_PASSWORD {
                    warn!(
                        "Grafana basic authentication is using the default insecure password. Please change it."
                    );
                }
                request_builder = request_builder.basic_auth(user, Some(pass.clone()));
            }
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

        if let Some(api_key) = &self.config.api_key {
            if !api_key.is_empty() {
                request_builder = request_builder.bearer_auth(api_key);
            }
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            if !user.is_empty() && !pass.is_empty() {
                if pass == DEFAULT_ADMIN_PASSWORD {
                    warn!(
                        "Grafana basic authentication is using the default insecure password. Please change it."
                    );
                }
                request_builder = request_builder.basic_auth(user, Some(pass.clone()));
            }
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

    /// # Errors
    ///
    /// Returns an error if the request to Grafana fails, if the response cannot be parsed,
    /// or if Grafana returns a non-success status code.
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
        if let Some(api_key) = &self.config.api_key {
            if !api_key.is_empty() {
                request_builder = request_builder.bearer_auth(api_key);
            }
        } else if let (Some(user), Some(pass)) =
            (&self.config.admin_user, &self.config.admin_password)
        {
            if !user.is_empty() && !pass.is_empty() {
                if pass == DEFAULT_ADMIN_PASSWORD {
                    warn!(
                        "Grafana basic authentication is using the default insecure password. Please change it."
                    );
                }
                request_builder = request_builder.basic_auth(user, Some(pass.clone()));
            }
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
                .map_or_else(|_| error_body.clone(), |e| e.message);

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
        assert_eq!(config.api_key, None);
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
            api_key: Some("test_api_key".to_string()),
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
