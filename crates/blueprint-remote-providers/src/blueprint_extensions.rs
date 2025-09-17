//! Blueprint manifest extensions for deployment configuration
//!
//! These can be specified in the blueprint's Cargo.toml:
//! ```toml
//! [package.metadata.blueprint.deployment]
//! service_type = "web"
//! health_check_path = "/health"
//! scaling_enabled = true
//! min_instances = 2
//! max_instances = 10
//! ```

use crate::error::Result;
use crate::service_classifier::{
    DeploymentHints, HealthCheckConfig, PortProtocol, PortPurpose, PortSpec, ScalingConfig,
    ScalingMetric, ServiceType, StateRequirements,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Blueprint deployment configuration from Cargo.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintDeploymentConfig {
    /// Service type hint (web, worker, batch, stateful, ai, realtime)
    pub service_type: Option<String>,

    /// Ports configuration
    pub ports: Option<Vec<PortConfig>>,

    /// Health check configuration
    pub health: Option<HealthConfig>,

    /// Scaling configuration
    pub scaling: Option<ScalingSettings>,

    /// State requirements
    pub state: Option<StateConfig>,

    /// Operator hints
    pub operator_hints: Option<OperatorHints>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    pub port: u16,
    #[serde(default = "default_protocol")]
    pub protocol: String,
    #[serde(default = "default_purpose")]
    pub purpose: String,
}

fn default_protocol() -> String {
    "http".to_string()
}

fn default_purpose() -> String {
    "primary".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub enabled: bool,
    pub http_endpoint: Option<String>,
    pub tcp_port: Option<u16>,
    pub interval_seconds: Option<u32>,
    pub timeout_seconds: Option<u32>,
    pub unhealthy_threshold: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingSettings {
    pub enabled: bool,
    pub min_instances: Option<u32>,
    pub max_instances: Option<u32>,
    pub target_cpu_percent: Option<u32>,
    pub startup_time_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateConfig {
    pub persistent_storage: bool,
    pub storage_size_gb: Option<f32>,
    pub database_type: Option<String>,
    pub database_size_gb: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorHints {
    /// Preferred deployment regions
    pub preferred_regions: Option<Vec<String>>,

    /// Allow spot/preemptible instances
    pub allow_spot: Option<bool>,

    /// Network requirements
    pub public_ip: Option<bool>,
    pub bandwidth_tier: Option<String>,

    /// Cost optimization hints
    pub max_hourly_cost: Option<f64>,

    /// Special requirements
    pub requires_gpu: Option<bool>,
    pub requires_tpm: Option<bool>,
    pub requires_sgx: Option<bool>,
}

impl BlueprintDeploymentConfig {
    /// Load from Cargo.toml metadata
    pub async fn from_cargo_toml(path: &Path) -> Result<Option<Self>> {
        let content = tokio::fs::read_to_string(path).await?;
        let manifest: toml::Value = toml::from_str(&content)?;

        // Look for [package.metadata.blueprint.deployment]
        let deployment = manifest
            .get("package")
            .and_then(|p| p.get("metadata"))
            .and_then(|m| m.get("blueprint"))
            .and_then(|b| b.get("deployment"));

        if let Some(deployment) = deployment {
            let config: BlueprintDeploymentConfig = deployment.clone().try_into()?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    /// Convert to DeploymentHints for the classifier
    pub fn to_deployment_hints(&self) -> DeploymentHints {
        DeploymentHints {
            service_type: self.parse_service_type(),
            ports: self.parse_ports(),
            health_check: self.parse_health_check(),
            scaling: self.parse_scaling(),
            state: self.parse_state(),
        }
    }

    fn parse_service_type(&self) -> ServiceType {
        match self.service_type.as_deref() {
            Some("web") => ServiceType::WebService,
            Some("worker") => ServiceType::Worker,
            Some("batch") => ServiceType::BatchJob,
            Some("stateful") => ServiceType::StatefulService,
            Some("ai") => ServiceType::AIService,
            Some("realtime") => ServiceType::RealtimeService,
            _ => ServiceType::Worker,
        }
    }

    fn parse_ports(&self) -> Vec<PortSpec> {
        self.ports.as_ref().map_or(vec![], |ports| {
            ports
                .iter()
                .map(|p| PortSpec {
                    port: p.port,
                    protocol: match p.protocol.as_str() {
                        "http" => PortProtocol::Http,
                        "https" => PortProtocol::Https,
                        "tcp" => PortProtocol::Tcp,
                        "udp" => PortProtocol::Udp,
                        "websocket" | "ws" => PortProtocol::WebSocket,
                        "grpc" => PortProtocol::Grpc,
                        _ => PortProtocol::Tcp,
                    },
                    purpose: match p.purpose.as_str() {
                        "primary" => PortPurpose::Primary,
                        "health" => PortPurpose::Health,
                        "metrics" => PortPurpose::Metrics,
                        "admin" => PortPurpose::Admin,
                        "p2p" => PortPurpose::P2P,
                        _ => PortPurpose::Primary,
                    },
                })
                .collect()
        })
    }

    fn parse_health_check(&self) -> Option<HealthCheckConfig> {
        self.health
            .as_ref()
            .filter(|h| h.enabled)
            .map(|h| HealthCheckConfig {
                http_path: h.http_endpoint.clone(),
                tcp_port: h.tcp_port,
                exec_command: None,
                interval_seconds: h.interval_seconds.unwrap_or(30),
                timeout_seconds: h.timeout_seconds.unwrap_or(5),
                failure_threshold: h.unhealthy_threshold.unwrap_or(3),
            })
    }

    fn parse_scaling(&self) -> ScalingConfig {
        let scaling = self.scaling.as_ref();
        ScalingConfig {
            horizontal_scaling: scaling.map_or(false, |s| s.enabled),
            scaling_metric: ScalingMetric::Cpu,
            min_instances: scaling.and_then(|s| s.min_instances).unwrap_or(1),
            max_instances: scaling.and_then(|s| s.max_instances).unwrap_or(10),
            startup_time_seconds: scaling.and_then(|s| s.startup_time_seconds).unwrap_or(30),
        }
    }

    fn parse_state(&self) -> StateRequirements {
        let state = self.state.as_ref();
        StateRequirements {
            persistent_storage: state.map_or(false, |s| s.persistent_storage),
            storage_size_gb: state.and_then(|s| s.storage_size_gb),
            shared_state: true, // Default to shareable
            database: None,     // TODO: Parse database requirements
        }
    }
}

/// Sane defaults for different blueprint types
pub struct DeploymentDefaults;

impl DeploymentDefaults {
    pub fn web_service() -> BlueprintDeploymentConfig {
        BlueprintDeploymentConfig {
            service_type: Some("web".to_string()),
            ports: Some(vec![PortConfig {
                port: 8080,
                protocol: "http".to_string(),
                purpose: "primary".to_string(),
            }]),
            health: Some(HealthConfig {
                enabled: true,
                http_endpoint: Some("/health".to_string()),
                tcp_port: None,
                interval_seconds: Some(30),
                timeout_seconds: Some(5),
                unhealthy_threshold: Some(3),
            }),
            scaling: Some(ScalingSettings {
                enabled: true,
                min_instances: Some(2),
                max_instances: Some(10),
                target_cpu_percent: Some(70),
                startup_time_seconds: Some(30),
            }),
            state: Some(StateConfig {
                persistent_storage: false,
                storage_size_gb: None,
                database_type: None,
                database_size_gb: None,
            }),
            operator_hints: None,
        }
    }

    pub fn worker() -> BlueprintDeploymentConfig {
        BlueprintDeploymentConfig {
            service_type: Some("worker".to_string()),
            ports: None,
            health: Some(HealthConfig {
                enabled: true,
                http_endpoint: None,
                tcp_port: None,
                interval_seconds: Some(60),
                timeout_seconds: Some(10),
                unhealthy_threshold: Some(3),
            }),
            scaling: Some(ScalingSettings {
                enabled: true,
                min_instances: Some(1),
                max_instances: Some(5),
                target_cpu_percent: Some(80),
                startup_time_seconds: Some(10),
            }),
            state: None,
            operator_hints: None,
        }
    }

    pub fn ai_service() -> BlueprintDeploymentConfig {
        BlueprintDeploymentConfig {
            service_type: Some("ai".to_string()),
            ports: Some(vec![PortConfig {
                port: 8080,
                protocol: "http".to_string(),
                purpose: "primary".to_string(),
            }]),
            health: Some(HealthConfig {
                enabled: true,
                http_endpoint: Some("/health".to_string()),
                tcp_port: None,
                interval_seconds: Some(60),
                timeout_seconds: Some(30), // AI models can be slow
                unhealthy_threshold: Some(2),
            }),
            scaling: Some(ScalingSettings {
                enabled: false, // GPU instances are expensive
                min_instances: Some(1),
                max_instances: Some(3),
                target_cpu_percent: None,
                startup_time_seconds: Some(120), // Model loading time
            }),
            state: Some(StateConfig {
                persistent_storage: true, // For model cache
                storage_size_gb: Some(100.0),
                database_type: None,
                database_size_gb: None,
            }),
            operator_hints: Some(OperatorHints {
                preferred_regions: None,
                allow_spot: Some(false), // Don't interrupt AI workloads
                public_ip: Some(true),
                bandwidth_tier: Some("premium".to_string()),
                max_hourly_cost: None,
                requires_gpu: Some(true),
                requires_tpm: None,
                requires_sgx: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_defaults() {
        let web = DeploymentDefaults::web_service();
        assert_eq!(web.service_type, Some("web".to_string()));
        assert!(web.scaling.as_ref().unwrap().enabled);

        let worker = DeploymentDefaults::worker();
        assert_eq!(worker.service_type, Some("worker".to_string()));
        assert!(worker.ports.is_none());

        let ai = DeploymentDefaults::ai_service();
        assert_eq!(ai.service_type, Some("ai".to_string()));
        assert!(!ai.scaling.as_ref().unwrap().enabled);
        assert!(ai.operator_hints.as_ref().unwrap().requires_gpu.unwrap());
    }
}
