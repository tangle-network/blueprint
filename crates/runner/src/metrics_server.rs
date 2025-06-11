use crate::BackgroundService;
use crate::error::RunnerError as Error;
use blueprint_qos::servers::ServerManager;
use blueprint_qos::servers::prometheus::PrometheusServer;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info};

/// Adapter for running a Prometheus metrics server as a background service
pub struct MetricsServerAdapter {
    /// The Prometheus server to run
    server: Arc<PrometheusServer>,
}

impl MetricsServerAdapter {
    /// Create a new metrics server adapter
    #[must_use]
    pub fn new(server: Arc<PrometheusServer>) -> Self {
        Self { server }
    }
}

impl BackgroundService for MetricsServerAdapter {
    fn start(
        &self,
    ) -> impl Future<Output = Result<oneshot::Receiver<Result<(), Error>>, Error>> + Send {
        let server = self.server.clone();

        async move {
            // Create a channel to signal when the service is done
            let (tx, rx) = oneshot::channel();

            // Start the server in a background task
            tokio::spawn(async move {
                info!("Starting metrics server...");

                if let Err(e) = server.start(None).await {
                    error!("Failed to start metrics server: {}", e);
                    let _ = tx.send(Err(Error::Other(
                        format!("Failed to start metrics server: {}", e).into(),
                    )));
                    return;
                }

                info!("Metrics server started successfully at {}", server.url());

                // Wait for the server to be ready
                if let Err(e) = server.wait_until_ready(30).await {
                    error!("Metrics server failed to become ready: {}", e);
                    let _ = tx.send(Err(Error::Other(
                        format!("Metrics server failed to become ready: {}", e).into(),
                    )));
                    return;
                }

                info!("Metrics server is ready");

                // Keep the server running until the channel is closed
                let _ = tx.send(Ok(()));
            });

            Ok(rx)
        }
    }
}
