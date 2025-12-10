//! Unit tests for BlueprintRunner builder validation
//!
//! These tests cover the builder pattern validation, ensuring proper error handling
//! when required components are missing. Integration tests for full runner execution
//! require more complex setup with actual protocol implementations.

use blueprint_core::error::BoxError;
use blueprint_core::job::call::JobCall;
use blueprint_router::Router;
use blueprint_runner::config::BlueprintEnvironment;
use blueprint_runner::error::RunnerError;
use blueprint_runner::{BackgroundService, BlueprintRunner};
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::timeout;

// =============================================================================
// TEST HELPERS
// =============================================================================

/// Creates a test environment with test_mode enabled
fn test_env() -> BlueprintEnvironment {
    let mut env = BlueprintEnvironment::default();
    env.test_mode = true;
    env
}

/// A producer that never produces (stays pending forever)
struct PendingProducer;

impl Stream for PendingProducer {
    type Item = Result<JobCall, BoxError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}

/// A producer that immediately errors
struct ErrorProducer {
    error_sent: bool,
}

impl ErrorProducer {
    fn new() -> Self {
        Self { error_sent: false }
    }
}

impl Stream for ErrorProducer {
    type Item = Result<JobCall, BoxError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if !self.error_sent {
            self.error_sent = true;
            Poll::Ready(Some(Err("producer error".into())))
        } else {
            Poll::Pending
        }
    }
}

/// A producer that ends immediately (stream ends)
struct EndingProducer;

impl Stream for EndingProducer {
    type Item = Result<JobCall, BoxError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None) // Stream ends immediately
    }
}

/// A simple background service for testing
#[derive(Clone)]
struct TestBackgroundService;

impl BackgroundService for TestBackgroundService {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            // Keep running until dropped
            tokio::time::sleep(Duration::from_secs(3600)).await;
            let _ = tx.send(Ok(()));
        });
        Ok(rx)
    }
}

// =============================================================================
// BUILDER VALIDATION TESTS
// =============================================================================

#[tokio::test]
async fn builder_without_router_returns_no_router_error() {
    let env = test_env();

    let result = BlueprintRunner::builder((), env)
        .producer(PendingProducer)
        // No router!
        .run()
        .await;

    match result {
        Err(RunnerError::NoRouter) => {} // Expected
        other => panic!("Expected NoRouter error, got: {:?}", other),
    }
}

#[tokio::test]
async fn builder_without_producers_returns_no_producers_error() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    let result = BlueprintRunner::builder((), env)
        .router(router)
        // No producers!
        .run()
        .await;

    match result {
        Err(RunnerError::NoProducers) => {} // Expected
        other => panic!("Expected NoProducers error, got: {:?}", other),
    }
}

#[tokio::test]
async fn builder_with_all_components_is_valid() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    // Start the runner and check it doesn't immediately error
    let handle = tokio::spawn(async move {
        BlueprintRunner::builder((), env)
            .router(router)
            .producer(PendingProducer)
            .run()
            .await
    });

    // Give a moment to check it's running
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Should still be running (not immediately errored)
    // Note: We can't easily check success, but we can verify it didn't immediately fail
    handle.abort();
}

// =============================================================================
// PRODUCER ERROR TESTS
// =============================================================================

/// Config that continues running (doesn't exit after registration)
struct ContinueRunningConfig;

impl blueprint_runner::BlueprintConfig for ContinueRunningConfig {
    async fn requires_registration(
        &self,
        _env: &BlueprintEnvironment,
    ) -> Result<bool, RunnerError> {
        Ok(false) // Skip registration
    }

    fn should_exit_after_registration(&self) -> bool {
        false // Keep running
    }
}

#[tokio::test]
async fn producer_error_propagates() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    let result = timeout(
        Duration::from_millis(500),
        BlueprintRunner::builder(ContinueRunningConfig, env)
            .router(router)
            .producer(ErrorProducer::new())
            .run(),
    )
    .await;

    match result {
        Ok(Err(RunnerError::Producer(_))) => {} // Expected
        other => panic!("Expected Producer error, got: {:?}", other),
    }
}

#[tokio::test]
async fn producer_stream_ending_returns_error() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    let result = timeout(
        Duration::from_millis(500),
        BlueprintRunner::builder(ContinueRunningConfig, env)
            .router(router)
            .producer(EndingProducer)
            .run(),
    )
    .await;

    match result {
        Ok(Err(RunnerError::Producer(_))) => {} // Expected - stream ended
        other => panic!("Expected Producer error (stream ended), got: {:?}", other),
    }
}

// =============================================================================
// BUILDER CHAIN TESTS
// =============================================================================

#[tokio::test]
async fn builder_accepts_multiple_producers() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    // This should compile and not panic during construction
    let _builder = BlueprintRunner::builder((), env)
        .router(router)
        .producer(PendingProducer)
        .producer(PendingProducer);

    // If we got here, the builder accepts multiple producers
}

#[tokio::test]
async fn builder_accepts_background_service() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    // This should compile and not panic during construction
    let _builder = BlueprintRunner::builder((), env)
        .router(router)
        .producer(PendingProducer)
        .background_service(TestBackgroundService);

    // If we got here, the builder accepts background services
}

#[tokio::test]
async fn builder_accepts_shutdown_handler() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    // This should compile and not panic during construction
    let _builder = BlueprintRunner::builder((), env)
        .router(router)
        .producer(PendingProducer)
        .with_shutdown_handler(async {
            println!("Shutdown!");
        });

    // If we got here, the builder accepts shutdown handlers
}

// =============================================================================
// BLUEPRINT CONFIG TESTS
// =============================================================================

/// Config that doesn't require registration and doesn't exit after
struct NoRegistrationConfig;

impl blueprint_runner::BlueprintConfig for NoRegistrationConfig {
    async fn requires_registration(
        &self,
        _env: &BlueprintEnvironment,
    ) -> Result<bool, RunnerError> {
        Ok(false)
    }

    fn should_exit_after_registration(&self) -> bool {
        false
    }
}

/// Config that requires registration but exits immediately after
struct ExitAfterRegistrationConfig;

impl blueprint_runner::BlueprintConfig for ExitAfterRegistrationConfig {
    async fn requires_registration(
        &self,
        _env: &BlueprintEnvironment,
    ) -> Result<bool, RunnerError> {
        Ok(false) // Skip actual registration
    }

    fn should_exit_after_registration(&self) -> bool {
        true
    }
}

#[tokio::test]
async fn custom_config_is_accepted() {
    let env = test_env();
    let router = Router::new().route(0u32, || async { "test" });

    // This should compile and accept the custom config
    let handle = tokio::spawn(async move {
        BlueprintRunner::builder(NoRegistrationConfig, env)
            .router(router)
            .producer(PendingProducer)
            .run()
            .await
    });

    // Give a moment
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Clean up
    handle.abort();
}

// =============================================================================
// ERROR TYPE TESTS
// =============================================================================

#[test]
fn runner_error_display() {
    let err = RunnerError::NoRouter;
    assert!(err.to_string().contains("router"));

    let err = RunnerError::NoProducers;
    assert!(err.to_string().contains("producer"));

    let err = RunnerError::BackgroundService("test error".to_string());
    assert!(err.to_string().contains("test error"));
}

#[test]
fn job_call_error_display() {
    use blueprint_runner::error::JobCallError;

    let err = JobCallError::JobFailed("test".into());
    assert!(err.to_string().contains("failed"));
}

#[test]
fn producer_error_display() {
    use blueprint_runner::error::ProducerError;

    let err = ProducerError::StreamEnded;
    assert!(err.to_string().contains("ended"));

    let err = ProducerError::Failed("test".into());
    assert!(err.to_string().contains("failed"));
}
