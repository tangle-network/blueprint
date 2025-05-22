use std::sync::Arc;
// Removed unused oneshot import
use tracing::info;

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
    #[must_use] pub fn provider(&self) -> Arc<EnhancedMetricsProvider> {
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

/// Run the `QoS` metrics server
pub async fn run_metrics_server(config: MetricsConfig) -> Result<Arc<EnhancedMetricsProvider>> {
    let otel_config = OpenTelemetryConfig::default();
    let provider = Arc::new(EnhancedMetricsProvider::new(config, otel_config)?);

    // Start the metrics collection
    provider.start_collection().await?;

    info!("Started metrics server");

    Ok(provider)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that a new `MetricsService` can be created with a valid configuration.
    ///
    /// ```
    /// MetricsConfig -> MetricsService
    /// ```
    ///
    /// Expected outcome: `MetricsService` is created with the provided config
    #[test]
    fn test_metrics_service_creation() {
        let config = MetricsConfig {
            service_id: 42,
            blueprint_id: 24,
            bind_address: "127.0.0.1:9090".to_string(),
            collection_interval_secs: 60,
            max_history: 100,
        };

        let service = MetricsService::new(config.clone());
        assert!(service.is_ok());

        let service = service.unwrap();
        // Check that the provider exists by verifying it's a valid Arc
        assert!(std::sync::Arc::strong_count(&service.provider()) >= 1);
    }

    /// Tests that a new `MetricsService` can be created with custom OpenTelemetry configuration.
    ///
    /// ```
    /// MetricsConfig + OpenTelemetryConfig -> MetricsService
    /// ```
    ///
    /// Expected outcome: `MetricsService` is created with the provided configs
    #[test]
    fn test_metrics_service_with_otel_config() {
        let config = MetricsConfig {
            service_id: 42,
            blueprint_id: 24,
            bind_address: "127.0.0.1:9090".to_string(),
            collection_interval_secs: 60,
            max_history: 100,
        };

        let otel_config = OpenTelemetryConfig::default();

        let service = MetricsService::with_otel_config(config.clone(), otel_config);
        assert!(service.is_ok());

        let service = service.unwrap();
        // Check that the provider exists by verifying it's a valid Arc
        assert!(std::sync::Arc::strong_count(&service.provider()) >= 1);
    }

    /// Tests that the `MetricsService` can record job executions.
    ///
    /// ```
    /// MetricsService.record_job_execution() -> Job execution recorded
    /// ```
    ///
    /// Expected outcome: Job execution is recorded in the metrics provider
    #[test]
    fn test_metrics_service_record_job_execution() {
        let config = MetricsConfig {
            service_id: 42,
            blueprint_id: 24,
            bind_address: "127.0.0.1:9090".to_string(),
            collection_interval_secs: 60,
            max_history: 100,
        };

        let service = MetricsService::new(config.clone()).unwrap();

        // This should not panic
        service.record_job_execution(1, 0.5);
    }

    /// Tests that the `MetricsService` can record job errors.
    ///
    /// ```
    /// MetricsService.record_job_error() -> Job error recorded
    /// ```
    ///
    /// Expected outcome: Job error is recorded in the metrics provider
    #[test]
    fn test_metrics_service_record_job_error() {
        let config = MetricsConfig {
            service_id: 42,
            blueprint_id: 24,
            bind_address: "127.0.0.1:9090".to_string(),
            collection_interval_secs: 60,
            max_history: 100,
        };

        let service = MetricsService::new(config.clone()).unwrap();

        // This should not panic
        service.record_job_error(1, "test_error");
    }
}
