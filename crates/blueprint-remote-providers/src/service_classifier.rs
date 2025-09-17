//! Service classification for intelligent deployment decisions

use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Service type determines deployment strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceType {
    /// Web service with HTTP endpoints (needs ingress/load balancer)
    WebService,
    /// Long-running worker/bot (no ingress needed)
    Worker,
    /// Batch/cron job (periodic execution)
    BatchJob,
    /// Stateful service (needs persistent storage)
    StatefulService,
    /// AI/ML service (may need GPU)
    AIService,
    /// Real-time service (WebSocket, streaming)
    RealtimeService,
}

/// Deployment hints from blueprint developer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentHints {
    /// Primary service type
    pub service_type: ServiceType,

    /// Exposed ports and their purposes
    pub ports: Vec<PortSpec>,

    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,

    /// Scaling behavior
    pub scaling: ScalingConfig,

    /// State requirements
    pub state: StateRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSpec {
    pub port: u16,
    pub protocol: PortProtocol,
    pub purpose: PortPurpose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    WebSocket,
    Grpc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortPurpose {
    /// Main service endpoint
    Primary,
    /// Health/readiness checks
    Health,
    /// Metrics/monitoring
    Metrics,
    /// Admin/debug interface
    Admin,
    /// P2P communication
    P2P,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// HTTP endpoint for health checks
    pub http_path: Option<String>,
    /// TCP port for health checks
    pub tcp_port: Option<u16>,
    /// Custom command for health checks
    pub exec_command: Option<Vec<String>>,
    /// Interval between checks
    pub interval_seconds: u32,
    /// Timeout for each check
    pub timeout_seconds: u32,
    /// Consecutive failures before unhealthy
    pub failure_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Can this service run multiple instances?
    pub horizontal_scaling: bool,
    /// Preferred scaling metric
    pub scaling_metric: ScalingMetric,
    /// Minimum instances (0 = can scale to zero)
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
    /// Startup time in seconds (affects scaling aggressiveness)
    pub startup_time_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingMetric {
    /// Scale based on CPU usage
    Cpu,
    /// Scale based on memory usage
    Memory,
    /// Scale based on request rate
    RequestRate,
    /// Scale based on queue depth
    QueueDepth,
    /// Custom metric from application
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateRequirements {
    /// Does this service need persistent storage?
    pub persistent_storage: bool,
    /// Storage size in GB (if needed)
    pub storage_size_gb: Option<f32>,
    /// Can instances share state? (false = sticky sessions needed)
    pub shared_state: bool,
    /// Database requirements
    pub database: Option<DatabaseRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRequirement {
    pub db_type: DatabaseType,
    pub size_gb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    Postgres,
    MySQL,
    MongoDB,
    Redis,
    Custom(String),
}

/// Service classifier that determines deployment strategy
pub struct ServiceClassifier;

impl ServiceClassifier {
    /// Classify service based on available information
    pub fn classify(
        hints: Option<&DeploymentHints>,
        container_metadata: Option<&ContainerMetadata>,
    ) -> ServiceType {
        // If developer provided hints, use them
        if let Some(hints) = hints {
            return hints.service_type;
        }

        // Otherwise, try to infer from container metadata
        if let Some(metadata) = container_metadata {
            return Self::infer_from_metadata(metadata);
        }

        // Default to worker (safest assumption)
        ServiceType::Worker
    }

    /// Infer service type from container metadata
    fn infer_from_metadata(metadata: &ContainerMetadata) -> ServiceType {
        // Check exposed ports
        for port in &metadata.exposed_ports {
            match port {
                80 | 443 | 8080 | 8443 | 3000 | 5000 => return ServiceType::WebService,
                6379 => return ServiceType::StatefulService, // Redis
                5432 | 3306 | 27017 => return ServiceType::StatefulService, // DBs
                _ => {}
            }
        }

        // Check environment variables for hints
        for (key, _) in &metadata.env_vars {
            let key_lower = key.to_lowercase();
            if key_lower.contains("http_port") || key_lower.contains("web_") {
                return ServiceType::WebService;
            }
            if key_lower.contains("worker") || key_lower.contains("queue") {
                return ServiceType::Worker;
            }
            if key_lower.contains("cuda") || key_lower.contains("gpu") {
                return ServiceType::AIService;
            }
        }

        // Check labels
        if let Some(service_type) = metadata.labels.get("blueprint.service.type") {
            match service_type.as_str() {
                "web" => return ServiceType::WebService,
                "worker" => return ServiceType::Worker,
                "batch" => return ServiceType::BatchJob,
                "stateful" => return ServiceType::StatefulService,
                "ai" => return ServiceType::AIService,
                "realtime" => return ServiceType::RealtimeService,
                _ => {}
            }
        }

        ServiceType::Worker
    }

    /// Generate Kubernetes deployment strategy based on service type
    pub fn deployment_strategy(service_type: ServiceType) -> K8sDeploymentStrategy {
        match service_type {
            ServiceType::WebService => K8sDeploymentStrategy {
                workload_type: K8sWorkloadType::Deployment,
                service_type: Some(K8sServiceType::LoadBalancer),
                ingress: true,
                hpa: true,
                pdb: true, // PodDisruptionBudget for high availability
            },

            ServiceType::Worker => K8sDeploymentStrategy {
                workload_type: K8sWorkloadType::Deployment,
                service_type: None, // No service needed
                ingress: false,
                hpa: true,
                pdb: false,
            },

            ServiceType::BatchJob => K8sDeploymentStrategy {
                workload_type: K8sWorkloadType::CronJob,
                service_type: None,
                ingress: false,
                hpa: false, // Jobs don't use HPA
                pdb: false,
            },

            ServiceType::StatefulService => K8sDeploymentStrategy {
                workload_type: K8sWorkloadType::StatefulSet,
                service_type: Some(K8sServiceType::ClusterIP),
                ingress: false,
                hpa: false, // StatefulSets scale differently
                pdb: true,
            },

            ServiceType::AIService => K8sDeploymentStrategy {
                workload_type: K8sWorkloadType::Deployment,
                service_type: Some(K8sServiceType::LoadBalancer),
                ingress: true,
                hpa: false, // GPU instances are expensive, scale carefully
                pdb: true,
            },

            ServiceType::RealtimeService => K8sDeploymentStrategy {
                workload_type: K8sWorkloadType::Deployment,
                service_type: Some(K8sServiceType::LoadBalancer),
                ingress: true,
                hpa: true,
                pdb: true,
            },
        }
    }
}

/// Container metadata extracted from image
#[derive(Debug, Clone)]
pub struct ContainerMetadata {
    pub exposed_ports: Vec<u16>,
    pub env_vars: HashMap<String, String>,
    pub labels: HashMap<String, String>,
    pub cmd: Vec<String>,
    pub entrypoint: Vec<String>,
}

/// Kubernetes deployment strategy
#[derive(Debug, Clone)]
pub struct K8sDeploymentStrategy {
    pub workload_type: K8sWorkloadType,
    pub service_type: Option<K8sServiceType>,
    pub ingress: bool,
    pub hpa: bool, // Horizontal Pod Autoscaler
    pub pdb: bool, // PodDisruptionBudget
}

#[derive(Debug, Clone)]
pub enum K8sWorkloadType {
    Deployment,
    StatefulSet,
    DaemonSet,
    Job,
    CronJob,
}

#[derive(Debug, Clone)]
pub enum K8sServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_service_classification() {
        let mut metadata = ContainerMetadata {
            exposed_ports: vec![8080],
            env_vars: HashMap::new(),
            labels: HashMap::new(),
            cmd: vec![],
            entrypoint: vec![],
        };

        assert_eq!(
            ServiceClassifier::infer_from_metadata(&metadata),
            ServiceType::WebService
        );

        // Test with environment variable hint
        metadata.exposed_ports.clear();
        metadata
            .env_vars
            .insert("HTTP_PORT".to_string(), "3000".to_string());
        assert_eq!(
            ServiceClassifier::infer_from_metadata(&metadata),
            ServiceType::WebService
        );
    }

    #[test]
    fn test_deployment_strategy() {
        let strategy = ServiceClassifier::deployment_strategy(ServiceType::WebService);
        assert!(matches!(
            strategy.workload_type,
            K8sWorkloadType::Deployment
        ));
        assert!(matches!(
            strategy.service_type,
            Some(K8sServiceType::LoadBalancer)
        ));
        assert!(strategy.ingress);
        assert!(strategy.hpa);

        let strategy = ServiceClassifier::deployment_strategy(ServiceType::BatchJob);
        assert!(matches!(strategy.workload_type, K8sWorkloadType::CronJob));
        assert!(strategy.service_type.is_none());
        assert!(!strategy.hpa);
    }
}
