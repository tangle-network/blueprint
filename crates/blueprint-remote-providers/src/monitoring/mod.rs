//! Infrastructure monitoring and discovery

pub mod discovery;
pub mod health;

pub use discovery::{MachineType, MachineTypeDiscovery};
pub use health::{HealthCheckResult, HealthMonitor, HealthStatus};
