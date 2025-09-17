//! Observability server for Blueprint Manager
//!
//! Provides a local dashboard and monitoring interface for operators to view
//! the health and performance of their Blueprint instances (both local and remote).
//! This is separate from QoS which handles on-chain reporting.

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use blueprint_std::sync::Arc;
use prometheus::{Encoder, Registry, TextEncoder, Gauge, Counter, Opts};
use std::net::SocketAddr;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::info;

#[cfg(feature = "remote-providers")]
use blueprint_remote_providers::observability::MetricsCollector as RemoteMetricsCollector;

/// Observability server that provides monitoring dashboards for operators
pub struct ObservabilityServer {
    registry: Arc<Registry>,
    // Local metrics
    cpu_usage: Gauge,
    memory_usage: Gauge,
    active_blueprints: Gauge,
    total_requests: Counter,
    failed_requests: Counter,
    #[cfg(feature = "remote-providers")]
    remote_metrics: Option<Arc<RemoteMetricsCollector>>,
    port: u16,
}

impl ObservabilityServer {
    /// Create a new observability server
    pub fn new(port: u16) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let registry = Arc::new(Registry::new());
        
        // Create metrics
        let cpu_usage = Gauge::with_opts(Opts::new("manager_cpu_usage", "CPU usage percentage"))?;
        let memory_usage = Gauge::with_opts(Opts::new("manager_memory_usage", "Memory usage in bytes"))?;
        let active_blueprints = Gauge::with_opts(Opts::new("active_blueprints", "Number of active blueprints"))?;
        let total_requests = Counter::with_opts(Opts::new("total_requests", "Total requests processed"))?;
        let failed_requests = Counter::with_opts(Opts::new("failed_requests", "Total failed requests"))?;
        
        // Register metrics
        registry.register(Box::new(cpu_usage.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(active_blueprints.clone()))?;
        registry.register(Box::new(total_requests.clone()))?;
        registry.register(Box::new(failed_requests.clone()))?;
        
        Ok(Self {
            registry,
            cpu_usage,
            memory_usage,
            active_blueprints,
            total_requests,
            failed_requests,
            #[cfg(feature = "remote-providers")]
            remote_metrics: None,
            port,
        })
    }
    
    /// Set the remote metrics collector for remote instances
    #[cfg(feature = "remote-providers")]
    pub fn with_remote_metrics(mut self, collector: Arc<RemoteMetricsCollector>) -> Self {
        self.remote_metrics = Some(collector);
        self
    }
    
    /// Start the observability server
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        
        let app = Router::new()
            .route("/", get(dashboard_handler))
            .route("/metrics", get(metrics_handler))
            .route("/health", get(health_handler))
            .with_state(Arc::new(self));
        
        info!("Starting observability server on http://{}", addr);
        
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
    
    /// Collect all metrics for Prometheus exposition
    fn collect_metrics(&self) -> String {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        
        // Collect standard Prometheus metrics from registry
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        
        String::from_utf8(buffer).unwrap()
    }
    
    /// Update local system metrics (call periodically)
    pub fn update_system_metrics(&self) {
        // In a real implementation, would collect actual system metrics
        // For now, just example values
        self.cpu_usage.set(0.25);
        self.memory_usage.set(1024.0 * 1024.0 * 512.0); // 512MB
    }
    
    /// Update active blueprint count
    pub fn set_active_blueprints(&self, count: f64) {
        self.active_blueprints.set(count);
    }
    
    /// Record a request
    pub fn record_request(&self, success: bool) {
        self.total_requests.inc();
        if !success {
            self.failed_requests.inc();
        }
    }
}

/// Handler for the main dashboard page
async fn dashboard_handler(State(server): State<Arc<ObservabilityServer>>) -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Blueprint Manager Dashboard</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #333; }
        .metric { 
            background: #f5f5f5; 
            padding: 15px; 
            margin: 10px 0; 
            border-radius: 5px;
        }
        .metric-label { font-weight: bold; color: #666; }
        .metric-value { font-size: 24px; color: #000; }
        .links { margin: 20px 0; }
        .links a { 
            margin-right: 20px; 
            padding: 10px 20px;
            background: #007bff;
            color: white;
            text-decoration: none;
            border-radius: 5px;
        }
        .links a:hover { background: #0056b3; }
    </style>
    <script>
        async function refreshMetrics() {
            const response = await fetch('/metrics');
            const text = await response.text();
            document.getElementById('metrics').innerText = text;
        }
        setInterval(refreshMetrics, 5000);
        window.onload = refreshMetrics;
    </script>
</head>
<body>
    <h1>Blueprint Manager Observability Dashboard</h1>
    
    <div class="links">
        <a href="/metrics">Prometheus Metrics</a>
        <a href="/health">Health Status</a>
    </div>
    
    <h2>Live Metrics</h2>
    <pre id="metrics">Loading...</pre>
    
    <h2>Quick Links</h2>
    <ul>
        <li>Grafana: <a href="http://localhost:3000">http://localhost:3000</a> (if configured)</li>
        <li>Prometheus: <a href="http://localhost:9090">http://localhost:9090</a></li>
    </ul>
</body>
</html>
    "#)
}

/// Handler for Prometheus metrics endpoint
async fn metrics_handler(State(server): State<Arc<ObservabilityServer>>) -> impl IntoResponse {
    server.collect_metrics()
}

/// Handler for health check endpoint
async fn health_handler() -> impl IntoResponse {
    "OK"
}