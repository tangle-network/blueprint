//! Tests for resilience patterns
//!
//! Verifies circuit breaker, retry logic, and rate limiting

use blueprint_remote_providers::resilience::*;
use blueprint_remote_providers::error::{Error, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_circuit_breaker_opens_on_failures() {
    let config = CircuitBreakerConfig {
        failure_threshold: 3,
        ..Default::default()
    };
    let cb = CircuitBreaker::new(config);
    
    assert_eq!(cb.state().await, CircuitState::Closed);
    assert!(cb.is_allowed().await);
    
    // Record failures
    for _ in 0..3 {
        cb.record_failure().await;
    }
    
    assert_eq!(cb.state().await, CircuitState::Open);
    assert!(!cb.is_allowed().await);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_transition() {
    let config = CircuitBreakerConfig {
        failure_threshold: 1,
        timeout: Duration::from_millis(100),
        ..Default::default()
    };
    let cb = CircuitBreaker::new(config);
    
    cb.record_failure().await;
    assert_eq!(cb.state().await, CircuitState::Open);
    
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    assert!(cb.is_allowed().await);
    assert_eq!(cb.state().await, CircuitState::HalfOpen);
}

#[tokio::test]
async fn test_circuit_breaker_recovery() {
    let config = CircuitBreakerConfig {
        failure_threshold: 2,
        success_threshold: 2,
        timeout: Duration::from_millis(100),
        ..Default::default()
    };
    let cb = CircuitBreaker::new(config);
    
    // Open the circuit
    cb.record_failure().await;
    cb.record_failure().await;
    assert_eq!(cb.state().await, CircuitState::Open);
    
    // Wait for half-open
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert!(cb.is_allowed().await);
    assert_eq!(cb.state().await, CircuitState::HalfOpen);
    
    // Record successes to close
    cb.record_success().await;
    assert_eq!(cb.state().await, CircuitState::HalfOpen);
    cb.record_success().await;
    assert_eq!(cb.state().await, CircuitState::Closed);
}

#[tokio::test]
async fn test_retry_with_backoff() {
    let config = RetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        jitter: false,
        ..Default::default()
    };
    
    let attempt = Arc::new(RwLock::new(0));
    let attempt_clone = attempt.clone();
    
    let result = with_retry(&config, || {
        let attempt = attempt_clone.clone();
        async move {
            let mut count = attempt.write().await;
            *count += 1;
            if *count < 3 {
                Err(Error::ConfigurationError("test error".into()))
            } else {
                Ok(42)
            }
        }
    }).await;
    
    assert_eq!(result.unwrap(), 42);
    assert_eq!(*attempt.read().await, 3);
}

#[tokio::test]
async fn test_retry_all_attempts_fail() {
    let config = RetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis(1),
        jitter: false,
        ..Default::default()
    };
    
    let result = with_retry(&config, || async {
        Err::<i32, _>(Error::ConfigurationError("always fails".into()))
    }).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("always fails"));
}

#[tokio::test]
async fn test_rate_limiter() {
    let limiter = RateLimiter::new(10, 5.0);
    
    // Should allow initial burst
    assert!(limiter.try_acquire(5).await);
    assert!(limiter.try_acquire(5).await);
    assert!(!limiter.try_acquire(1).await);
    
    // Wait for refill
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert!(limiter.try_acquire(1).await);
}

#[tokio::test]
async fn test_rate_limiter_refill() {
    let limiter = RateLimiter::new(5, 10.0); // 10 tokens per second
    
    // Use all tokens
    assert!(limiter.try_acquire(5).await);
    assert!(!limiter.try_acquire(1).await);
    
    // Wait for partial refill (100ms = 1 token)
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(limiter.try_acquire(1).await);
    assert!(!limiter.try_acquire(1).await);
}

#[tokio::test]
async fn test_rate_limiter_acquire_wait() {
    let limiter = RateLimiter::new(1, 10.0);
    
    // Use the token
    assert!(limiter.try_acquire(1).await);
    
    // This should wait ~100ms for refill
    let start = tokio::time::Instant::now();
    limiter.acquire(1).await;
    let elapsed = start.elapsed();
    
    assert!(elapsed >= Duration::from_millis(90));
    assert!(elapsed < Duration::from_millis(200));
}