use std::sync::Arc;
use tokio::sync::oneshot::{self, Receiver};
use tracing::{error, info};

// use blueprint_runner::{BackgroundService, error::RunnerError};

use crate::error::Result;
use crate::metrics::opentelemetry::OpenTelemetryConfig;
use crate::metrics::provider::EnhancedMetricsProvider;
use crate::metrics::types::MetricsConfig;

/// Metrics service for collecting and exposing metrics
pub struct MetricsService {
    /// Metrics provider
    provider: Arc<EnhancedMetricsProvider>,

    /// Configuration
    config: MetricsConfig,
}

impl MetricsService {
    /// Create a new metrics service
    pub fn new(config: MetricsConfig) -> Result<Self> {
        let otel_config = OpenTelemetryConfig::default();
        let provider = Arc::new(EnhancedMetricsProvider::new(config.clone(), otel_config)?);

        Ok(Self { provider, config })
    }

    /// Create a new metrics service with custom OpenTelemetry configuration
    pub fn with_otel_config(
        config: MetricsConfig,
        otel_config: OpenTelemetryConfig,
    ) -> Result<Self> {
        let provider = Arc::new(EnhancedMetricsProvider::new(config.clone(), otel_config)?);

        Ok(Self { provider, config })
    }

    /// Get the metrics provider
    pub fn provider(&self) -> Arc<EnhancedMetricsProvider> {
        self.provider.clone()
    }

    /// Record job execution
    pub fn record_job_execution(&self, job_id: u64, execution_time: f64) {
        self.provider.record_job_execution(job_id, execution_time);
    }

    /// Record job error
    pub fn record_job_error(&self, job_id: u64, error_type: &str) {
        self.provider.record_job_error(job_id, error_type);
    }
}

// #[tonic::async_trait]
// impl BackgroundService for MetricsService {
//     async fn start(&self) -> std::result::Result<Receiver<()>, RunnerError> {
//         // Start the metrics collection
//         self.provider.start_collection().await.map_err(|e| {
//             RunnerError::BackgroundServiceError(format!(
//                 "Failed to start metrics collection: {}",
//                 e
//             ))
//         })?;

//         // Create a channel for shutdown
//         let (tx, rx) = oneshot::channel();

//         info!("Started metrics service");

//         Ok(rx)
//     }
// }

/// Run the QoS metrics server
pub async fn run_metrics_server(config: MetricsConfig) -> Result<Arc<EnhancedMetricsProvider>> {
    let otel_config = OpenTelemetryConfig::default();
    let provider = Arc::new(EnhancedMetricsProvider::new(config, otel_config)?);

    // Start the metrics collection
    provider.start_collection().await?;

    info!("Started metrics server");

    Ok(provider)
}
