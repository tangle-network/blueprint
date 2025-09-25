#[cfg(feature = "containers")]
pub mod container;
#[cfg(feature = "vm-sandbox")]
pub mod hypervisor;
pub mod native;
#[cfg(feature = "remote-providers")]
pub mod remote;
pub mod service;

pub struct ResourceLimits {
    /// Allocated storage space in bytes
    pub storage_space: u64,
    /// Allocated memory space in bytes
    pub memory_size: u64,
    /// Number of CPU cores
    pub cpu_count: Option<u8>,
    /// Number of GPU devices
    pub gpu_count: Option<u8>,
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
            // No bandwidth limit by default
            network_bandwidth: None,
        }
    }
}
