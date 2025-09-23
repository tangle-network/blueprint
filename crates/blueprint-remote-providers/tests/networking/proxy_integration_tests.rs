//! Integration tests for proxy-to-remote communication
//!
//! Tests the complete flow from proxy through secure bridge to remote instances

use blueprint_remote_providers::{
    secure_bridge::{SecureBridge, SecureBridgeConfig, RemoteEndpoint},
    auth_integration::{SecureCloudCredentials, RemoteServiceAuth, AuthProxyRemoteExtension},
    resilience::{CircuitBreakerConfig, RetryConfig},
};
use blueprint_auth::db::{RocksDb, RocksDbConfig};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

/// Mock remote service for testing
async fn mock_remote_service(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    
    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            
            // Read request
            let n = socket.read(&mut buf).await.unwrap();
            let request = String::from_utf8_lossy(&buf[..n]);
            
            // Parse method
            let response = if request.contains("GET /health") {
                "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
            } else if request.contains("GET /api/data") {
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"data\": \"test\"}"
            } else {
                "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found"
            };
            
            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}

#[tokio::test]
async fn test_proxy_to_remote_health_check() {
    // Start mock remote service
    let port = 9001;
    tokio::spawn(async move {
        let _ = mock_remote_service(port).await;
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Setup secure bridge
    let config = SecureBridgeConfig {
        enable_mtls: false,
        ..Default::default()
    };
    let bridge = Arc::new(SecureBridge::new(config).await.unwrap());
    
    // Register endpoint
    let endpoint = RemoteEndpoint {
        instance_id: "i-test-proxy".to_string(),
        host: "127.0.0.1".to_string(),
        port,
        use_tls: false,
        service_id: 1,
        blueprint_id: 100,
    };
    
    bridge.register_endpoint(1, endpoint).await.unwrap();
    
    // Perform health check
    let healthy = bridge.health_check(1).await.unwrap();
    assert!(healthy);
}

#[tokio::test] 
async fn test_proxy_request_forwarding() {
    // Start mock remote service
    let port = 9002;
    tokio::spawn(async move {
        let _ = mock_remote_service(port).await;
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Setup secure bridge
    let config = SecureBridgeConfig {
        enable_mtls: false,
        ..Default::default()
    };
    let bridge = Arc::new(SecureBridge::new(config).await.unwrap());
    
    // Register endpoint
    let endpoint = RemoteEndpoint {
        instance_id: "i-test-forward".to_string(),
        host: "127.0.0.1".to_string(),
        port,
        use_tls: false,
        service_id: 2,
        blueprint_id: 200,
    };
    
    bridge.register_endpoint(2, endpoint).await.unwrap();
    
    // Forward request
    let headers = HashMap::from([
        ("Accept".to_string(), "application/json".to_string()),
    ]);
    
    let (status, response_headers, body) = bridge.forward_request(
        2,
        "GET",
        "/api/data",
        headers,
        vec![],
    ).await.unwrap();
    
    assert_eq!(status, 200);
    assert_eq!(response_headers.get("Content-Type"), Some(&"application/json".to_string()));
    
    let body_str = String::from_utf8(body).unwrap();
    assert!(body_str.contains("\"data\": \"test\""));
}

#[tokio::test]
async fn test_circuit_breaker_integration() {
    // Don't start mock service - simulate failure
    let config = SecureBridgeConfig {
        enable_mtls: false,
        connect_timeout_secs: 1,
        ..Default::default()
    };
    let bridge = Arc::new(SecureBridge::new(config).await.unwrap());
    
    // Register endpoint to non-existent service
    let endpoint = RemoteEndpoint {
        instance_id: "i-test-circuit".to_string(),
        host: "127.0.0.1".to_string(),
        port: 9999, // Nothing listening here
        use_tls: false,
        service_id: 3,
        blueprint_id: 300,
    };
    
    bridge.register_endpoint(3, endpoint).await.unwrap();
    
    // First few requests should fail but be allowed
    for _ in 0..3 {
        let result = bridge.forward_request(
            3,
            "GET",
            "/health",
            HashMap::new(),
            vec![],
        ).await;
        assert!(result.is_err());
    }
    
    // Circuit should now be open - request blocked immediately
    let start = tokio::time::Instant::now();
    let result = bridge.forward_request(
        3,
        "GET",
        "/health",
        HashMap::new(),
        vec![],
    ).await;
    let elapsed = start.elapsed();
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("circuit breaker open"));
    assert!(elapsed < tokio::time::Duration::from_millis(100)); // Should fail fast
}

#[tokio::test]
async fn test_auth_integration_with_proxy() {
    let temp_dir = TempDir::new().unwrap();
    let db = RocksDb::open(temp_dir.path().join("auth.db"), &RocksDbConfig::default()).unwrap();
    
    // Create secure credentials
    let credentials = SecureCloudCredentials::new(
        1,
        "aws",
        r#"{"access_key": "test_key", "secret_key": "test_secret"}"#,
    ).await.unwrap();
    
    // Verify encryption
    assert!(credentials.encrypted_credentials.len() > 56);
    let decrypted = credentials.decrypt().unwrap();
    assert!(decrypted.contains("test_key"));
    
    // Setup bridge
    let config = SecureBridgeConfig {
        enable_mtls: false,
        ..Default::default()
    };
    let bridge = Arc::new(SecureBridge::new(config).await.unwrap());
    
    // Create auth proxy extension
    let extension = AuthProxyRemoteExtension::new(bridge.clone()).await;
    
    // Register remote service
    let auth = RemoteServiceAuth::register(
        1,
        100,
        "i-auth-test".to_string(),
        "127.0.0.1".to_string(),
        9003,
        credentials,
    ).await.unwrap();
    
    extension.register_service(auth).await;
    
    // Verify service is marked as remote
    assert!(extension.is_remote(1).await);
    assert!(!extension.is_remote(999).await);
}

#[tokio::test]
async fn test_retry_with_intermittent_failures() {
    // Start mock service that fails first 2 requests
    let port = 9004;
    let request_count = Arc::new(tokio::sync::Mutex::new(0));
    let request_count_clone = request_count.clone();
    
    tokio::spawn(async move {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
        
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let count = request_count_clone.clone();
            
            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                let _ = socket.read(&mut buf).await;
                
                let mut c = count.lock().await;
                *c += 1;
                
                let response = if *c <= 2 {
                    // First 2 requests fail
                    socket.shutdown().await.ok();
                    return;
                } else {
                    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
                };
                
                socket.write_all(response.as_bytes()).await.ok();
            });
        }
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Setup bridge with retry
    let config = SecureBridgeConfig {
        enable_mtls: false,
        ..Default::default()
    };
    let bridge = Arc::new(SecureBridge::new(config).await.unwrap());
    
    // Register endpoint
    let endpoint = RemoteEndpoint {
        instance_id: "i-test-retry".to_string(),
        host: "127.0.0.1".to_string(),
        port,
        use_tls: false,
        service_id: 4,
        blueprint_id: 400,
    };
    
    bridge.register_endpoint(4, endpoint).await.unwrap();
    
    // Should succeed after retries
    let result = bridge.forward_request(
        4,
        "GET",
        "/health",
        HashMap::new(),
        vec![],
    ).await;
    
    assert!(result.is_ok());
    let (status, _, _) = result.unwrap();
    assert_eq!(status, 200);
    
    // Verify retry happened
    let count = request_count.lock().await;
    assert_eq!(*count, 3);
}