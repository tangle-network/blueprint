//! Resilience patterns for remote communication
//!
//! Implements circuit breaker, retry logic, and other resilience patterns

use crate::error::{Error, Result};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    /// Circuit is closed, requests flow normally
    Closed,
    /// Circuit is open, requests are blocked
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: u32,
    /// Success threshold to close circuit from half-open
    pub success_threshold: u32,
    /// Duration to wait before trying half-open
    pub timeout: Duration,
    /// Time window for counting failures
    pub window: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            window: Duration::from_secs(60),
        }
    }
}

/// Circuit breaker for a service
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitState>>,
    failure_count: Arc<RwLock<u32>>,
    success_count: Arc<RwLock<u32>>,
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    state_changed_at: Arc<RwLock<Instant>>,
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
            state_changed_at: Arc::new(RwLock::new(Instant::now())),
        }
    }
    
    /// Check if request is allowed
    pub async fn is_allowed(&self) -> bool {
        let mut state = self.state.write().await;
        let state_changed_at = *self.state_changed_at.read().await;
        
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout has passed
                if state_changed_at.elapsed() >= self.config.timeout {
                    *state = CircuitState::HalfOpen;
                    *self.state_changed_at.write().await = Instant::now();
                    *self.success_count.write().await = 0;
                    info!("Circuit breaker transitioning to half-open");
                    true
                } else {
                    false
                }
            },
            CircuitState::HalfOpen => true,
        }
    }
    
    /// Record successful request
    pub async fn record_success(&self) {
        let mut state = self.state.write().await;
        let mut success_count = self.success_count.write().await;
        
        match *state {
            CircuitState::HalfOpen => {
                *success_count += 1;
                debug!("Circuit breaker success count: {}", *success_count);
                
                if *success_count >= self.config.success_threshold {
                    *state = CircuitState::Closed;
                    *self.state_changed_at.write().await = Instant::now();
                    *self.failure_count.write().await = 0;
                    info!("Circuit breaker closed after successful recovery");
                }
            },
            CircuitState::Closed => {
                // Reset failure count on success
                *self.failure_count.write().await = 0;
            },
            _ => {},
        }
    }
    
    /// Record failed request
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        let mut failure_count = self.failure_count.write().await;
        let mut last_failure_time = self.last_failure_time.write().await;
        
        // Check if failures are within window
        if let Some(last_time) = *last_failure_time {
            if last_time.elapsed() > self.config.window {
                *failure_count = 0;
            }
        }
        
        *failure_count += 1;
        *last_failure_time = Some(Instant::now());
        
        match *state {
            CircuitState::Closed => {
                if *failure_count >= self.config.failure_threshold {
                    *state = CircuitState::Open;
                    *self.state_changed_at.write().await = Instant::now();
                    warn!("Circuit breaker opened after {} failures", *failure_count);
                }
            },
            CircuitState::HalfOpen => {
                *state = CircuitState::Open;
                *self.state_changed_at.write().await = Instant::now();
                warn!("Circuit breaker re-opened from half-open state");
            },
            _ => {},
        }
    }
    
    /// Get current state
    pub async fn state(&self) -> CircuitState {
        *self.state.read().await
    }
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Exponential backoff multiplier
    pub multiplier: f64,
    /// Add jitter to delays
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Execute function with retry logic
pub async fn with_retry<F, Fut, T>(
    config: &RetryConfig,
    mut f: F,
) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut delay = config.initial_delay;
    
    for attempt in 1..=config.max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt == config.max_attempts => {
                warn!("All {} retry attempts failed", config.max_attempts);
                return Err(e);
            },
            Err(e) => {
                warn!("Attempt {} failed: {}, retrying in {:?}", attempt, e, delay);
                
                // Apply jitter if configured
                let actual_delay = if config.jitter {
                    use rand::Rng;
                    let jitter = rand::thread_rng().gen_range(0..=delay.as_millis() as u64 / 4);
                    delay + Duration::from_millis(jitter)
                } else {
                    delay
                };
                
                tokio::time::sleep(actual_delay).await;
                
                // Calculate next delay with exponential backoff
                delay = Duration::from_secs_f64(
                    (delay.as_secs_f64() * config.multiplier).min(config.max_delay.as_secs_f64())
                );
            }
        }
    }
    
    unreachable!()
}

/// Rate limiter using token bucket algorithm
pub struct RateLimiter {
    capacity: u32,
    tokens: Arc<RwLock<f64>>,
    refill_rate: f64,
    last_refill: Arc<RwLock<Instant>>,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(capacity: u32, refill_per_second: f64) -> Self {
        Self {
            capacity,
            tokens: Arc::new(RwLock::new(capacity as f64)),
            refill_rate: refill_per_second,
            last_refill: Arc::new(RwLock::new(Instant::now())),
        }
    }
    
    /// Try to acquire tokens
    pub async fn try_acquire(&self, count: u32) -> bool {
        let mut tokens = self.tokens.write().await;
        let mut last_refill = self.last_refill.write().await;
        
        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let refill = (elapsed * self.refill_rate).min(self.capacity as f64 - *tokens);
        *tokens += refill;
        *last_refill = now;
        
        // Try to acquire
        if *tokens >= count as f64 {
            *tokens -= count as f64;
            true
        } else {
            false
        }
    }
    
    /// Wait until tokens are available
    pub async fn acquire(&self, count: u32) {
        loop {
            if self.try_acquire(count).await {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}

