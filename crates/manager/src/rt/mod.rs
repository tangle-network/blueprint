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
    pub gpu_count: Option<u8>,
    /// GPU scheduling policy (Required = hard constraint, Preferred = soft affinity)
    pub gpu_policy: GpuSchedulingPolicy,
    /// Minimum VRAM per GPU device in GiB (used for node selector labels)
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
            gpu_policy: GpuSchedulingPolicy::None,
            gpu_min_vram_gb: None,
            // No bandwidth limit by default
            network_bandwidth: None,
        }
    }
}
