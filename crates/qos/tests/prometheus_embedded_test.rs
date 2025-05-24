use blueprint_qos::servers::prometheus::{PrometheusServer, PrometheusServerConfig};
use blueprint_qos::servers::ServerManager;
use std::sync::Arc;
use tokio::time::Duration;
use blueprint_core::info;
use blueprint_testing_utils::setup_log;

#[tokio::test]
async fn test_prometheus_embedded_server() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    setup_log();

    info!("Starting Prometheus embedded server test...");

    // Configure Prometheus server to use embedded mode
    let prometheus_config = PrometheusServerConfig {
        port: 9090,
        host: "0.0.0.0".to_string(),
        use_docker: false, // Use embedded server instead of Docker
        docker_image: "prom/prometheus:latest".to_string(),
        docker_container_name: "blueprint-test-prometheus".to_string(),
        config_path: None,
        data_path: None,
    };

    // Create the Prometheus server directly
    let server = Arc::new(PrometheusServer::new(prometheus_config));
    
    // Start the server
    info!("Starting Prometheus server...");
    server.start().await?;
    
    // Get the server URL
    let url = server.url();
    info!("Prometheus server URL: {}", url);
    
    // Wait for the server to be ready
    info!("Waiting for server to be ready...");
    server.wait_until_ready(30).await?;
    
    // Check if the server is running
    let is_running = server.is_running().await?;
    info!("Prometheus server is running: {}", is_running);
    
    if is_running {
        info!("Test passed! Prometheus embedded server is working correctly.");
    } else {
        return Err("Prometheus server is not running".into());
    }
    
    // Sleep for a few seconds to allow manual verification if needed
    info!("Sleeping for 5 seconds before shutdown...");
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    // Stop the server
    info!("Stopping Prometheus server...");
    server.stop().await?;
    info!("Prometheus server stopped.");
    
    Ok(())
}
