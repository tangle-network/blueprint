//! Infrastructure monitoring and discovery

pub mod health;
pub mod discovery;

pub use health::{HealthMonitor, HealthStatus, HealthCheckResult};
pub use discovery::{MachineTypeDiscovery, MachineType};