#[cfg(feature = "containers")]
pub mod container;
#[cfg(feature = "vm-sandbox")]
pub mod hypervisor;
pub mod native;
#[cfg(feature = "remote-providers")]
pub mod remote;
pub mod service;

/// GPU scheduling policy for container runtime.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GpuSchedulingPolicy {
    /// No GPU resources requested.
    #[default]
    None,
    /// GPU is mandatory — add `nvidia.com/gpu` to resource requests (hard constraint).
    /// Pod stays Pending if no GPU node exists. This is the correct K8s behavior.
    Required,
    /// GPU is preferred — use node affinity `preferredDuringSchedulingIgnoredDuringExecution`
    /// with GPU labels. Pod can schedule on CPU-only nodes as fallback.
    Preferred,
}

pub struct ResourceLimits {
    /// Allocated storage space in bytes
    pub storage_space: u64,
    /// Allocated memory space in bytes
    pub memory_size: u64,
    /// Number of CPU cores
    pub cpu_count: Option<u8>,
    /// Number of GPU devices
    pub gpu_count: Option<u32>,
    /// Canonical GPU class id from the SDK catalog.
    pub gpu_class: Option<String>,
    /// GPU scheduling policy (Required = hard constraint, Preferred = soft affinity)
    pub gpu_policy: GpuSchedulingPolicy,
    /// Minimum decimal GB of VRAM per GPU device.
    pub gpu_min_vram_gb: Option<u32>,
    /// Network bandwidth in Mbps
    pub network_bandwidth: Option<u32>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            // 20GB
            storage_space: 1024 * 1024 * 1024 * 20,
            // 4GB
            memory_size: 1024 * 1024 * 1024 * 4,
            // 2 CPU cores by default
            cpu_count: Some(2),
            // No GPU by default
            gpu_count: None,
            gpu_class: None,
            gpu_policy: GpuSchedulingPolicy::None,
            gpu_min_vram_gb: None,
            // No bandwidth limit by default
            network_bandwidth: None,
        }
    }
}

impl From<&blueprint_client_tangle::GpuRequirements> for ResourceLimits {
    fn from(gpu: &blueprint_client_tangle::GpuRequirements) -> Self {
        use blueprint_client_tangle::GpuPolicy;
        if matches!(gpu.policy, GpuPolicy::None) {
            return Self::default();
        }
        Self {
            gpu_count: Some(gpu.min_count.max(1)),
            gpu_class: gpu.class.clone(),
            gpu_min_vram_gb: (gpu.min_vram_gb > 0).then_some(gpu.min_vram_gb),
            gpu_policy: match gpu.policy {
                GpuPolicy::Required => GpuSchedulingPolicy::Required,
                GpuPolicy::Preferred => GpuSchedulingPolicy::Preferred,
                GpuPolicy::None => GpuSchedulingPolicy::None,
            },
            ..Self::default()
        }
    }
}

impl From<&ResourceLimits> for blueprint_core::resources::ResourceSpec {
    fn from(limits: &ResourceLimits) -> Self {
        Self {
            cpu: f32::from(limits.cpu_count.unwrap_or(2)),
            memory_gb: limits.memory_size as f32 / (1024.0 * 1024.0 * 1024.0),
            storage_gb: limits.storage_space as f32 / (1024.0 * 1024.0 * 1024.0),
            gpu_count: limits.gpu_count,
            gpu_class: limits.gpu_class.clone(),
            gpu_vram_gb: limits.gpu_min_vram_gb,
            ..Self::default()
        }
    }
}
