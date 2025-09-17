//! Observability and metrics for remote providers
//!
//! Provides instrumentation for monitoring remote deployments

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn};
use serde::{Deserialize, Serialize};
use blueprint_std::collections::HashMap;

/// Metrics for a remote service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// Total requests forwarded
    pub request_count: u64,
    /// Successful requests
    pub success_count: u64,
    /// Failed requests
    pub failure_count: u64,
    /// Average response time in ms
    pub avg_response_time_ms: f64,
    /// P95 response time in ms
    pub p95_response_time_ms: f64,
    /// P99 response time in ms
    pub p99_response_time_ms: f64,
    /// Circuit breaker trips
    pub circuit_breaker_trips: u64,
    /// Current circuit state
    pub circuit_state: String,
    /// Last health check time
    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    /// Health check status
    pub is_healthy: bool,
}

/// Metrics collector for all services
pub struct MetricsCollector {
    /// Metrics per service
    metrics: Arc<RwLock<HashMap<u64, ServiceMetrics>>>,
    /// Response time samples for percentile calculation
    response_times: Arc<RwLock<HashMap<u64, Vec<Duration>>>>,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            response_times: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Record a request
    pub async fn record_request(
        &self,
        service_id: u64,
        success: bool,
        response_time: Duration,
    ) {
        let mut metrics = self.metrics.write().await;
        let metric = metrics.entry(service_id).or_default();
        
        metric.request_count += 1;
        if success {
            metric.success_count += 1;
        } else {
            metric.failure_count += 1;
        }
        
        // Store response time sample
        let mut times = self.response_times.write().await;
        let samples = times.entry(service_id).or_insert_with(Vec::new);
        samples.push(response_time);
        
        // Keep only last 1000 samples for percentile calculation
        if samples.len() > 1000 {
            samples.drain(0..samples.len() - 1000);
        }
        
        // Calculate percentiles
        if !samples.is_empty() {
            let mut sorted = samples.clone();
            sorted.sort();
            
            let avg: Duration = sorted.iter().sum::<Duration>() / sorted.len() as u32;
            metric.avg_response_time_ms = avg.as_secs_f64() * 1000.0;
            
            let p95_idx = (sorted.len() as f64 * 0.95) as usize;
            metric.p95_response_time_ms = sorted.get(p95_idx)
                .unwrap_or(&sorted[sorted.len() - 1])
                .as_secs_f64() * 1000.0;
            
            let p99_idx = (sorted.len() as f64 * 0.99) as usize;
            metric.p99_response_time_ms = sorted.get(p99_idx)
                .unwrap_or(&sorted[sorted.len() - 1])
                .as_secs_f64() * 1000.0;
        }
    }
    
    /// Record circuit breaker trip
    pub async fn record_circuit_breaker_trip(&self, service_id: u64, state: &str) {
        let mut metrics = self.metrics.write().await;
        let metric = metrics.entry(service_id).or_default();
        metric.circuit_breaker_trips += 1;
        metric.circuit_state = state.to_string();
        
        warn!(
            service_id = service_id,
            state = state,
            "Circuit breaker state changed"
        );
    }
    
    /// Record health check
    pub async fn record_health_check(&self, service_id: u64, healthy: bool) {
        let mut metrics = self.metrics.write().await;
        let metric = metrics.entry(service_id).or_default();
        metric.is_healthy = healthy;
        metric.last_health_check = Some(chrono::Utc::now());
    }
    
    /// Get metrics for a service
    pub async fn get_metrics(&self, service_id: u64) -> Option<ServiceMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(&service_id).cloned()
    }
    
    /// Get all metrics
    pub async fn get_all_metrics(&self) -> HashMap<u64, ServiceMetrics> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Export metrics in Prometheus format
    pub async fn export_prometheus(&self) -> String {
        let metrics = self.metrics.read().await;
        let mut output = String::new();
        
        for (service_id, metric) in metrics.iter() {
            output.push_str(&format!(
                "# HELP remote_requests_total Total requests to remote service\n\
                # TYPE remote_requests_total counter\n\
                remote_requests_total{{service_id=\"{}\"}} {}\n\n",
                service_id, metric.request_count
            ));
            
            output.push_str(&format!(
                "# HELP remote_requests_success_total Successful requests to remote service\n\
                # TYPE remote_requests_success_total counter\n\
                remote_requests_success_total{{service_id=\"{}\"}} {}\n\n",
                service_id, metric.success_count
            ));
            
            output.push_str(&format!(
                "# HELP remote_response_time_ms Response time in milliseconds\n\
                # TYPE remote_response_time_ms summary\n\
                remote_response_time_ms{{service_id=\"{}\",quantile=\"0.5\"}} {}\n\
                remote_response_time_ms{{service_id=\"{}\",quantile=\"0.95\"}} {}\n\
                remote_response_time_ms{{service_id=\"{}\",quantile=\"0.99\"}} {}\n\n",
                service_id, metric.avg_response_time_ms,
                service_id, metric.p95_response_time_ms,
                service_id, metric.p99_response_time_ms
            ));
            
            output.push_str(&format!(
                "# HELP remote_circuit_breaker_trips Circuit breaker trip count\n\
                # TYPE remote_circuit_breaker_trips counter\n\
                remote_circuit_breaker_trips{{service_id=\"{}\"}} {}\n\n",
                service_id, metric.circuit_breaker_trips
            ));
            
            output.push_str(&format!(
                "# HELP remote_service_healthy Service health status\n\
                # TYPE remote_service_healthy gauge\n\
                remote_service_healthy{{service_id=\"{}\"}} {}\n\n",
                service_id, if metric.is_healthy { 1 } else { 0 }
            ));
        }
        
        output
    }
}

/// Request span for tracing
pub struct RequestSpan {
    service_id: u64,
    start_time: Instant,
    collector: Arc<MetricsCollector>,
}

impl RequestSpan {
    /// Create new request span
    pub fn new(service_id: u64, collector: Arc<MetricsCollector>) -> Self {
        info!(service_id = service_id, "Starting request");
        Self {
            service_id,
            start_time: Instant::now(),
            collector,
        }
    }
    
    /// Complete the span
    pub async fn complete(self, success: bool) {
        let duration = self.start_time.elapsed();
        
        info!(
            service_id = self.service_id,
            success = success,
            duration_ms = duration.as_millis(),
            "Request completed"
        );
        
        self.collector.record_request(self.service_id, success, duration).await;
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}