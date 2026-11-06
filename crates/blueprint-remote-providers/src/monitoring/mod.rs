//! Infrastructure monitoring and discovery

pub mod discovery;
pub mod health;
pub mod logs;

pub use discovery::{MachineType, MachineTypeDiscovery};
pub use health::{HealthCheckResult, HealthMonitor, HealthStatus};
pub use logs::{LogAggregator, LogEntry, LogLevel, LogSource, LogStreamer};
