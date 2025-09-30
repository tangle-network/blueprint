//! Infrastructure monitoring and discovery

pub mod discovery;
pub mod health;
pub mod logs;
pub mod loki;

pub use discovery::{MachineType, MachineTypeDiscovery};
pub use health::{HealthCheckResult, HealthMonitor, HealthStatus};
pub use logs::{LogStreamer, LogEntry, LogLevel, LogSource, LogAggregator};
pub use loki::{LokiClient, LogAggregationPipeline};
