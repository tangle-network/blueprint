//! Resource model for pricing-engine, manager, and remote-providers integration
//! 
//! Provides resource management foundation for local and remote deployments
//! with consistent resource definitions and pricing calculations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resource specification for deployment targets
/// 
/// Provides comprehensive resource configuration for local resource 
/// enforcement and remote instance selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// Core compute resources
    pub compute: ComputeResources,
    
    /// Memory and storage
    pub storage: StorageResources,
    
    /// Network requirements
    pub network: NetworkResources,
    
    /// Optional accelerators (GPUs, TPUs, etc)
    pub accelerators: Option<AcceleratorResources>,
    
    /// Quality of service parameters
    pub qos: QosParameters,
    
    /// Container runtime configuration
    pub runtime: RuntimeConfiguration,
    
    /// Monitoring and observability settings
    pub observability: ObservabilityConfiguration,
}

impl Default for ResourceSpec {
    fn default() -> Self {
        Self {
            compute: ComputeResources::default(),
            storage: StorageResources::default(),
            network: NetworkResources::default(),
            accelerators: None,
            qos: QosParameters::default(),
            runtime: RuntimeConfiguration::default(),
            observability: ObservabilityConfiguration::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    /// CPU cores (can be fractional, e.g., 0.5 for half a core)
    pub cpu_cores: f64,
    
    /// CPU architecture preference (x86_64, arm64, etc)
    pub cpu_arch: Option<String>,
    
    /// Minimum CPU frequency in GHz
    pub min_cpu_frequency_ghz: Option<f64>,
    
    /// CPU model preference (e.g., "Intel Xeon", "AMD EPYC")
    pub cpu_model: Option<String>,
    
    /// Required CPU features (AVX2, AVX512, AES-NI, etc)
    pub cpu_features: Vec<String>,
    
    /// CPU performance tier (economy, standard, premium)
    pub cpu_tier: Option<PerformanceTier>,
}

impl Default for ComputeResources {
    fn default() -> Self {
        Self {
            cpu_cores: 1.0,
            cpu_arch: None,
            min_cpu_frequency_ghz: None,
            cpu_model: None,
            cpu_features: Vec::new(),
            cpu_tier: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResources {
    /// RAM in GB
    pub memory_gb: f64,
    
    /// Persistent storage in GB
    pub disk_gb: f64,
    
    /// Storage type (ssd, nvme, hdd)
    pub disk_type: StorageType,
    
    /// Minimum IOPS requirement
    pub iops: Option<u32>,
    
    /// Throughput in MB/s
    pub throughput_mbps: Option<u32>,
    
    /// Memory type (DDR4, DDR5, HBM)
    pub memory_type: Option<MemoryType>,
    
    /// ECC memory required
    pub ecc_required: bool,
    
    /// Ephemeral storage in GB (temporary, instance storage)
    pub ephemeral_gb: Option<f64>,
    
    /// Object storage in GB (S3-compatible)
    pub object_storage_gb: Option<f64>,
}

impl Default for StorageResources {
    fn default() -> Self {
        Self {
            memory_gb: 2.0,
            disk_gb: 10.0,
            disk_type: StorageType::SSD,
            iops: None,
            throughput_mbps: None,
            memory_type: None,
            ecc_required: false,
            ephemeral_gb: None,
            object_storage_gb: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    HDD,
    SSD, 
    NVME,
    EBS,           // Elastic Block Storage
    LocalSSD,      // Local instance SSD
    PersistentSSD, // Network-attached SSD
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTier {
    Economy,
    Standard,
    Premium,
    Ultra,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    DDR4,
    DDR5,
    HBM,    // High Bandwidth Memory
    LPDDR5, // Low Power DDR5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancerType {
    None,
    ApplicationLB,  // Layer 7
    NetworkLB,      // Layer 4
    GlobalLB,       // Multi-region
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DdosProtectionLevel {
    Basic,
    Standard,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Backup frequency in hours
    pub frequency_hours: u32,
    /// Retention period in days
    pub retention_days: u32,
    /// Geographic redundancy
    pub geo_redundant: bool,
    /// Point-in-time recovery
    pub point_in_time_recovery: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryConfig {
    /// Recovery Time Objective in minutes
    pub rto_minutes: u32,
    /// Recovery Point Objective in minutes
    pub rpo_minutes: u32,
    /// Multi-region failover
    pub multi_region: bool,
    /// Automated failover
    pub auto_failover: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceRequirement {
    HIPAA,
    PCIDSS,
    SOC2,
    ISO27001,
    GDPR,
    FedRAMP,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoRestrictions {
    /// Allowed regions/countries
    pub allowed_regions: Vec<String>,
    /// Blocked regions/countries
    pub blocked_regions: Vec<String>,
    /// Prefer certain regions
    pub preferred_regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirements {
    /// Encryption at rest
    pub at_rest: bool,
    /// Encryption in transit
    pub in_transit: bool,
    /// Key management service
    pub kms_required: bool,
    /// Bring your own key
    pub byok_enabled: bool,
    /// Minimum TLS version
    pub min_tls_version: Option<String>,
}

impl Default for EncryptionRequirements {
    fn default() -> Self {
        Self {
            at_rest: true,
            in_transit: true,
            kms_required: false,
            byok_enabled: false,
            min_tls_version: Some("1.2".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResources {
    /// Bandwidth tier
    pub bandwidth_tier: BandwidthTier,
    
    /// Guaranteed bandwidth in Mbps
    pub guaranteed_bandwidth_mbps: Option<u32>,
    
    /// Static IP requirement
    pub static_ip: bool,
    
    /// Public IP requirement
    pub public_ip: bool,
    
    /// IPv6 support required
    pub ipv6_required: bool,
    
    /// Number of public IPs needed
    pub public_ip_count: u32,
    
    /// Network latency requirement in ms
    pub max_latency_ms: Option<u32>,
    
    /// Required network protocols (TCP, UDP, SCTP, etc)
    pub protocols: Vec<String>,
    
    /// Ingress bandwidth limit in Gbps
    pub ingress_limit_gbps: Option<f64>,
    
    /// Egress bandwidth limit in Gbps
    pub egress_limit_gbps: Option<f64>,
    
    /// Load balancer requirement
    pub load_balancer: Option<LoadBalancerType>,
    
    /// CDN requirement
    pub cdn_enabled: bool,
    
    /// DDoS protection level
    pub ddos_protection: Option<DdosProtectionLevel>,
}

impl Default for NetworkResources {
    fn default() -> Self {
        Self {
            bandwidth_tier: BandwidthTier::Standard,
            guaranteed_bandwidth_mbps: None,
            static_ip: false,
            public_ip: false,
            ipv6_required: false,
            public_ip_count: 0,
            max_latency_ms: None,
            protocols: vec!["TCP".to_string(), "UDP".to_string()],
            ingress_limit_gbps: None,
            egress_limit_gbps: None,
            load_balancer: None,
            cdn_enabled: false,
            ddos_protection: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BandwidthTier {
    Low,      // Up to 1 Gbps
    Standard, // Up to 10 Gbps
    High,     // Up to 25 Gbps
    Ultra,    // 50+ Gbps
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceleratorResources {
    /// Number of accelerators
    pub count: u32,
    
    /// Type of accelerator
    pub accelerator_type: AcceleratorType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AcceleratorType {
    GPU(GpuSpec),
    TPU(String),
    FPGA(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSpec {
    /// GPU vendor (nvidia, amd, intel)
    pub vendor: String,
    
    /// GPU model (a100, v100, t4, etc)
    pub model: String,
    
    /// Minimum VRAM in GB
    pub min_vram_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QosParameters {
    /// Priority level (0-100, higher is more important)
    pub priority: u8,
    
    /// Whether spot/preemptible instances are acceptable
    pub allow_spot: bool,
    
    /// Whether burstable instances are acceptable
    pub allow_burstable: bool,
    
    /// Minimum availability SLA (99.9, 99.99, etc)
    pub min_availability_sla: Option<f64>,
    
    /// Maximum acceptable downtime per month in minutes
    pub max_downtime_minutes: Option<u32>,
    
    /// Backup requirements
    pub backup_config: Option<BackupConfig>,
    
    /// Disaster recovery requirements
    pub disaster_recovery: Option<DisasterRecoveryConfig>,
    
    /// Compliance requirements (HIPAA, PCI-DSS, SOC2, etc)
    pub compliance: Vec<ComplianceRequirement>,
    
    /// Geographic restrictions
    pub geo_restrictions: Option<GeoRestrictions>,
    
    /// Data residency requirements
    pub data_residency: Vec<String>,
    
    /// Encryption requirements
    pub encryption: EncryptionRequirements,
}

impl Default for QosParameters {
    fn default() -> Self {
        Self {
            priority: 50,
            allow_spot: false,
            allow_burstable: true,
            min_availability_sla: None,
            max_downtime_minutes: None,
            backup_config: None,
            disaster_recovery: None,
            compliance: Vec::new(),
            geo_restrictions: None,
            data_residency: Vec::new(),
            encryption: EncryptionRequirements::default(),
        }
    }
}

/// Converts resource spec to pricing engine resource units for cost calculation
pub fn to_pricing_units(spec: &ResourceSpec) -> HashMap<String, f64> {
    let mut units = HashMap::new();
    
    // Map to pricing engine ResourceUnit equivalents
    units.insert("CPU".to_string(), spec.compute.cpu_cores);
    units.insert("MemoryMB".to_string(), spec.storage.memory_gb * 1024.0);
    units.insert("StorageMB".to_string(), spec.storage.disk_gb * 1024.0);
    
    // Network units based on tier
    let network_multiplier = match spec.network.bandwidth_tier {
        BandwidthTier::Low => 1.0,
        BandwidthTier::Standard => 2.0,
        BandwidthTier::High => 4.0,
        BandwidthTier::Ultra => 8.0,
    };
    units.insert("NetworkEgressMB".to_string(), 1024.0 * network_multiplier);
    units.insert("NetworkIngressMB".to_string(), 1024.0 * network_multiplier);
    
    // GPU units if present
    if let Some(ref accel) = spec.accelerators {
        if let AcceleratorType::GPU(_) = accel.accelerator_type {
            units.insert("GPU".to_string(), accel.count as f64);
        }
    }
    
    units
}

/// Converts resource spec to Kubernetes resource limits
#[cfg(feature = "kubernetes")]
pub fn to_k8s_resources(spec: &ResourceSpec) -> (k8s_openapi::api::core::v1::ResourceRequirements, Option<k8s_openapi::api::core::v1::PersistentVolumeClaimSpec>) {
    use k8s_openapi::api::core::v1::{ResourceRequirements, PersistentVolumeClaimSpec};
    use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
    
    let mut limits = std::collections::BTreeMap::new();
    let mut requests = std::collections::BTreeMap::new();
    
    // CPU (in cores or millicores)
    let cpu_str = if spec.compute.cpu_cores < 1.0 {
        format!("{}m", (spec.compute.cpu_cores * 1000.0) as i32)
    } else {
        format!("{}", spec.compute.cpu_cores)
    };
    limits.insert("cpu".to_string(), Quantity(cpu_str.clone()));
    requests.insert("cpu".to_string(), Quantity(cpu_str));
    
    // Memory
    let memory_str = format!("{}Gi", spec.storage.memory_gb);
    limits.insert("memory".to_string(), Quantity(memory_str.clone()));
    requests.insert("memory".to_string(), Quantity(memory_str));
    
    // GPU if present
    if let Some(ref accel) = spec.accelerators {
        if let AcceleratorType::GPU(ref gpu_spec) = accel.accelerator_type {
            let gpu_key = match gpu_spec.vendor.as_str() {
                "nvidia" => "nvidia.com/gpu",
                "amd" => "amd.com/gpu",
                _ => "gpu",
            };
            limits.insert(gpu_key.to_string(), Quantity(accel.count.to_string()));
            requests.insert(gpu_key.to_string(), Quantity(accel.count.to_string()));
        }
    }
    
    let resource_req = ResourceRequirements {
        limits: Some(limits),
        requests: Some(requests),
        claims: None,
    };
    
    // Storage as PVC if needed
    let pvc_spec = if spec.storage.disk_gb > 0.0 {
        let mut pvc_requests = std::collections::BTreeMap::new();
        pvc_requests.insert("storage".to_string(), Quantity(format!("{}Gi", spec.storage.disk_gb)));
        
        let storage_class = match spec.storage.disk_type {
            StorageType::HDD => Some("standard".to_string()),
            StorageType::SSD => Some("ssd".to_string()),
            StorageType::NVME => Some("nvme".to_string()),
        };
        
        Some(PersistentVolumeClaimSpec {
            access_modes: Some(vec!["ReadWriteOnce".to_string()]),
            resources: Some(ResourceRequirements {
                requests: Some(pvc_requests),
                limits: None,
                claims: None,
            }),
            storage_class_name: storage_class,
            ..Default::default()
        })
    } else {
        None
    };
    
    (resource_req, pvc_spec)
}

/// Converts resource spec to Docker resource limits
pub fn to_docker_resources(spec: &ResourceSpec) -> serde_json::Value {
    let mut host_config = serde_json::json!({});
    
    // CPU limits (Docker uses nano CPUs)
    let nano_cpus = (spec.compute.cpu_cores * 1_000_000_000.0) as i64;
    host_config["NanoCPUs"] = nano_cpus.into();
    
    // Memory limits (in bytes)
    let memory_bytes = (spec.storage.memory_gb * 1024.0 * 1024.0 * 1024.0) as i64;
    host_config["Memory"] = memory_bytes.into();
    
    // Storage limits if available
    if spec.storage.disk_gb > 0.0 {
        host_config["StorageOpt"] = serde_json::json!({
            "size": format!("{}G", spec.storage.disk_gb)
        });
    }
    
    // GPU support via device requests
    if let Some(ref accel) = spec.accelerators {
        if let AcceleratorType::GPU(_) = accel.accelerator_type {
            host_config["DeviceRequests"] = serde_json::json!([
                {
                    "Driver": "nvidia",
                    "Count": accel.count,
                    "Capabilities": [["gpu"]]
                }
            ]);
        }
    }
    
    host_config
}

/// Converts from legacy manager ResourceLimits
pub fn from_legacy_limits(memory_mb: Option<u64>, storage_mb: Option<u64>) -> ResourceSpec {
    ResourceSpec {
        compute: ComputeResources::default(),
        storage: StorageResources {
            memory_gb: memory_mb.map(|mb| mb as f64 / 1024.0).unwrap_or(2.0),
            disk_gb: storage_mb.map(|mb| mb as f64 / 1024.0).unwrap_or(10.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Converts from remote-providers ResourceRequirements
pub fn from_resource_requirements(req: &crate::provisioning::ResourceRequirements) -> ResourceSpec {
    use crate::provisioning::NetworkTier;
    
    ResourceSpec {
        compute: ComputeResources {
            cpu_cores: req.cpu_cores,
            ..Default::default()
        },
        storage: StorageResources {
            memory_gb: req.memory_gb,
            disk_gb: req.storage_gb,
            ..Default::default()
        },
        network: NetworkResources {
            bandwidth_tier: match req.network_tier {
                NetworkTier::Low => BandwidthTier::Low,
                NetworkTier::Standard => BandwidthTier::Standard,
                NetworkTier::High => BandwidthTier::High,
                NetworkTier::Ultra => BandwidthTier::Ultra,
            },
            ..Default::default()
        },
        accelerators: req.gpu_count.map(|count| AcceleratorResources {
            count,
            accelerator_type: AcceleratorType::GPU(GpuSpec {
                vendor: "nvidia".to_string(),
                model: req.gpu_type.clone().unwrap_or_else(|| "t4".to_string()),
                min_vram_gb: 16.0,
            }),
        }),
        qos: QosParameters {
            allow_spot: req.allow_spot,
            ..Default::default()
        },
        runtime: RuntimeConfiguration::default(),
        observability: ObservabilityConfiguration::default(),
    }
}

/// Container runtime configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfiguration {
    /// Preferred container runtime (Docker, containerd, CRI-O)
    pub runtime_type: RuntimeType,
    
    /// Security context requirements
    pub security_context: SecurityContext,
    
    /// Resource limits enforcement
    pub limits_enforcement: LimitsEnforcement,
    
    /// Init process configuration
    pub init_process: bool,
    
    /// Privileged container access
    pub privileged: bool,
    
    /// User namespace mapping
    pub user_namespace: bool,
    
    /// Read-only root filesystem
    pub read_only_root_fs: bool,
    
    /// Capabilities to add/drop
    pub capabilities: CapabilityConfiguration,
}

impl Default for RuntimeConfiguration {
    fn default() -> Self {
        Self {
            runtime_type: RuntimeType::Containerd,
            security_context: SecurityContext::default(),
            limits_enforcement: LimitsEnforcement::Strict,
            init_process: true,
            privileged: false,
            user_namespace: true,
            read_only_root_fs: true,
            capabilities: CapabilityConfiguration::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeType {
    Docker,
    Containerd,
    CRIO,
    Podman,
    Kata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LimitsEnforcement {
    Strict,
    BestEffort,
    Guaranteed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Run as non-root user
    pub run_as_non_root: bool,
    
    /// Specific user ID to run as
    pub run_as_user: Option<u32>,
    
    /// Specific group ID to run as
    pub run_as_group: Option<u32>,
    
    /// SELinux options
    pub selinux_options: Option<SELinuxOptions>,
    
    /// AppArmor profile
    pub apparmor_profile: Option<String>,
    
    /// Seccomp profile
    pub seccomp_profile: Option<String>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            run_as_non_root: true,
            run_as_user: Some(1000),
            run_as_group: Some(1000),
            selinux_options: None,
            apparmor_profile: Some("runtime/default".to_string()),
            seccomp_profile: Some("runtime/default".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SELinuxOptions {
    pub user: Option<String>,
    pub role: Option<String>,
    pub type_: Option<String>,
    pub level: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityConfiguration {
    /// Linux capabilities to add
    pub add: Vec<String>,
    
    /// Linux capabilities to drop
    pub drop: Vec<String>,
}

impl Default for CapabilityConfiguration {
    fn default() -> Self {
        Self {
            add: vec![],
            drop: vec!["ALL".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfiguration {
    /// Metrics collection settings
    pub metrics: MetricsConfiguration,
    
    /// Logging configuration
    pub logging: LoggingConfiguration,
    
    /// Tracing/APM settings
    pub tracing: TracingConfiguration,
    
    /// Health check configuration
    pub health_checks: HealthCheckConfiguration,
}

impl Default for ObservabilityConfiguration {
    fn default() -> Self {
        Self {
            metrics: MetricsConfiguration::default(),
            logging: LoggingConfiguration::default(),
            tracing: TracingConfiguration::default(),
            health_checks: HealthCheckConfiguration::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfiguration {
    /// Enable metrics collection
    pub enabled: bool,
    
    /// Metrics endpoint path
    pub endpoint_path: String,
    
    /// Metrics port
    pub port: u16,
    
    /// Scrape interval in seconds
    pub scrape_interval: u32,
    
    /// Custom metrics labels
    pub custom_labels: HashMap<String, String>,
}

impl Default for MetricsConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint_path: "/metrics".to_string(),
            port: 9090,
            scrape_interval: 30,
            custom_labels: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfiguration {
    /// Log level (debug, info, warn, error)
    pub level: String,
    
    /// Log format (json, text)
    pub format: String,
    
    /// Log aggregation service
    pub aggregation_service: Option<String>,
    
    /// Log retention period in days
    pub retention_days: Option<u32>,
    
    /// Maximum log size in MB
    pub max_size_mb: Option<u32>,
}

impl Default for LoggingConfiguration {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            aggregation_service: None,
            retention_days: Some(30),
            max_size_mb: Some(100),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfiguration {
    /// Enable distributed tracing
    pub enabled: bool,
    
    /// Tracing service (Jaeger, Zipkin, OpenTelemetry)
    pub service: Option<String>,
    
    /// Sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
    
    /// Custom trace attributes
    pub custom_attributes: HashMap<String, String>,
}

impl Default for TracingConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            service: None,
            sampling_rate: 0.1,
            custom_attributes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfiguration {
    /// Enable health checks
    pub enabled: bool,
    
    /// HTTP health check path
    pub http_path: Option<String>,
    
    /// Health check port
    pub port: Option<u16>,
    
    /// Check interval in seconds
    pub interval_seconds: u32,
    
    /// Timeout in seconds
    pub timeout_seconds: u32,
    
    /// Failure threshold before marking unhealthy
    pub failure_threshold: u32,
    
    /// Success threshold before marking healthy
    pub success_threshold: u32,
}

impl Default for HealthCheckConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            http_path: Some("/health".to_string()),
            port: None, // Use main service port
            interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
            success_threshold: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resource_spec_to_pricing_units() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let units = to_pricing_units(&spec);
        
        assert_eq!(units.get("CPU"), Some(&4.0));
        assert_eq!(units.get("MemoryMB"), Some(&(16.0 * 1024.0)));
        assert_eq!(units.get("StorageMB"), Some(&(100.0 * 1024.0)));
    }
    
    #[test]
    #[cfg(feature = "kubernetes")]
    fn test_k8s_resource_conversion() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 0.5,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 2.0,
                disk_gb: 10.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let (resources, pvc) = to_k8s_resources(&spec);
        
        assert!(resources.limits.is_some());
        let limits = resources.limits.unwrap();
        assert!(limits.contains_key("cpu"));
        assert!(limits.contains_key("memory"));
        
        assert!(pvc.is_some());
    }
    
    #[test]
    fn test_docker_resource_conversion() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 4.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let docker_config = to_docker_resources(&spec);
        
        assert_eq!(docker_config["NanoCPUs"], 2_000_000_000i64);
        assert_eq!(docker_config["Memory"], 4 * 1024 * 1024 * 1024i64);
    }
}