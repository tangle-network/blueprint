//! Error recovery and resilience for remote deployments
//!
//! Provides retry logic, connection recovery, and rollback capabilities
//! for SSH and deployment failures.

use crate::core::error::{Error, Result};
use crate::deployment::ssh::SshDeploymentClient;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

/// Recovery strategy for deployment failures
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Retry with exponential backoff
    Retry {
        max_attempts: u32,
        initial_delay: Duration,
        max_delay: Duration,
        exponential_base: f64,
    },
    /// Attempt rollback to previous state
    Rollback {
        checkpoint: DeploymentCheckpoint,
    },
    /// Fail immediately without recovery
    FailFast,
    /// Try alternative deployment method
    Fallback {
        alternative: Box<RecoveryStrategy>,
    },
}

impl Default for RecoveryStrategy {
    fn default() -> Self {
        Self::Retry {
            max_attempts: 3,
            initial_delay: Duration::from_secs(2),
            max_delay: Duration::from_secs(30),
            exponential_base: 2.0,
        }
    }
}

/// Checkpoint for rollback operations
#[derive(Debug, Clone)]
pub struct DeploymentCheckpoint {
    pub instance_id: String,
    pub container_id: Option<String>,
    pub timestamp: std::time::SystemTime,
    pub state: CheckpointState,
}

#[derive(Debug, Clone)]
pub enum CheckpointState {
    PreDeployment,
    ContainerCreated,
    ContainerStarted,
    HealthCheckPassed,
    Completed,
}

/// Error recovery coordinator
pub struct ErrorRecovery {
    strategy: RecoveryStrategy,
    checkpoints: Vec<DeploymentCheckpoint>,
}

impl ErrorRecovery {
    pub fn new(strategy: RecoveryStrategy) -> Self {
        Self {
            strategy,
            checkpoints: Vec::new(),
        }
    }

    /// Save a deployment checkpoint
    pub fn checkpoint(&mut self, checkpoint: DeploymentCheckpoint) {
        info!("Saving deployment checkpoint: {:?}", checkpoint.state);
        self.checkpoints.push(checkpoint);
    }

    /// Execute an operation with recovery
    pub async fn execute_with_recovery<F, T>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> futures::future::BoxFuture<'static, Result<T>> + Send + Sync,
        T: Send,
    {
        match &self.strategy {
            RecoveryStrategy::Retry {
                max_attempts,
                initial_delay,
                max_delay,
                exponential_base,
            } => {
                self.retry_with_backoff(
                    operation,
                    *max_attempts,
                    *initial_delay,
                    *max_delay,
                    *exponential_base,
                )
                .await
            }
            RecoveryStrategy::FailFast => operation().await,
            RecoveryStrategy::Rollback { checkpoint } => {
                match operation().await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        warn!("Operation failed, attempting rollback: {}", e);
                        self.rollback_to_checkpoint(checkpoint).await?;
                        Err(e)
                    }
                }
            }
            RecoveryStrategy::Fallback { alternative } => {
                match operation().await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        warn!("Primary strategy failed, trying fallback");
                        let fallback_recovery = Self::new((**alternative).clone());
                        fallback_recovery.execute_with_recovery(operation).await
                    }
                }
            }
        }
    }

    async fn retry_with_backoff<F, T>(
        &self,
        operation: F,
        max_attempts: u32,
        initial_delay: Duration,
        max_delay: Duration,
        exponential_base: f64,
    ) -> Result<T>
    where
        F: Fn() -> futures::future::BoxFuture<'static, Result<T>>,
        T: Send,
    {
        let mut attempt = 0;
        let mut delay = initial_delay;

        loop {
            attempt += 1;
            debug!("Attempt {} of {}", attempt, max_attempts);

            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        info!("Operation succeeded after {} attempts", attempt);
                    }
                    return Ok(result);
                }
                Err(e) if attempt >= max_attempts => {
                    error!("Operation failed after {} attempts: {}", max_attempts, e);
                    return Err(e);
                }
                Err(e) => {
                    warn!("Attempt {} failed: {}, retrying in {:?}", attempt, e, delay);
                    sleep(delay).await;

                    // Exponential backoff
                    delay = Duration::from_secs_f64(
                        (delay.as_secs_f64() * exponential_base).min(max_delay.as_secs_f64()),
                    );
                }
            }
        }
    }

    async fn rollback_to_checkpoint(&self, checkpoint: &DeploymentCheckpoint) -> Result<()> {
        info!("Rolling back to checkpoint: {:?}", checkpoint.state);

        // Implementation would depend on the checkpoint state
        match &checkpoint.state {
            CheckpointState::ContainerCreated | CheckpointState::ContainerStarted => {
                if let Some(container_id) = &checkpoint.container_id {
                    warn!("Would remove container: {}", container_id);
                    // In real implementation: ssh_client.remove_container(container_id).await?;
                }
            }
            _ => {
                debug!("No rollback action needed for state: {:?}", checkpoint.state);
            }
        }

        Ok(())
    }
}

/// SSH connection recovery
pub struct SshConnectionRecovery {
    max_reconnect_attempts: u32,
    connection_timeout: Duration,
    keepalive_interval: Duration,
}

impl Default for SshConnectionRecovery {
    fn default() -> Self {
        Self {
            max_reconnect_attempts: 5,
            connection_timeout: Duration::from_secs(30),
            keepalive_interval: Duration::from_secs(60),
        }
    }
}

impl SshConnectionRecovery {
    /// Verify SSH connection is alive
    pub async fn verify_connection(
        &self,
        host: &str,
        port: u16,
    ) -> Result<bool> {
        use tokio::net::TcpStream;

        match timeout(
            self.connection_timeout,
            TcpStream::connect(format!("{}:{}", host, port)),
        )
        .await
        {
            Ok(Ok(_)) => Ok(true),
            Ok(Err(e)) => {
                warn!("SSH connection check failed: {}", e);
                Ok(false)
            }
            Err(_) => {
                warn!("SSH connection check timed out");
                Ok(false)
            }
        }
    }

    /// Reconnect with retry logic
    pub async fn reconnect(
        &self,
        client: &mut SshDeploymentClient,
    ) -> Result<()> {
        let mut attempts = 0;

        while attempts < self.max_reconnect_attempts {
            attempts += 1;
            info!("SSH reconnection attempt {}", attempts);

            if client.reconnect().await.is_ok() {
                info!("SSH reconnection successful");
                return Ok(());
            }

            if attempts < self.max_reconnect_attempts {
                let delay = Duration::from_secs(attempts as u64 * 2);
                sleep(delay).await;
            }
        }

        Err(Error::Other(format!(
            "Failed to reconnect after {} attempts",
            self.max_reconnect_attempts
        )))
    }

    /// Execute command with automatic reconnection
    pub async fn execute_with_reconnect<F, T>(
        &self,
        client: &mut SshDeploymentClient,
        operation: F,
    ) -> Result<T>
    where
        F: Fn(&SshDeploymentClient) -> futures::future::BoxFuture<'_, Result<T>>,
    {
        // First attempt
        match operation(client).await {
            Ok(result) => Ok(result),
            Err(e) => {
                warn!("Operation failed, attempting reconnection: {}", e);

                // Try to reconnect
                self.reconnect(client).await?;

                // Retry operation once after reconnection
                operation(client).await
            }
        }
    }
}

/// Transaction-like deployment operations
pub struct DeploymentTransaction {
    operations: Vec<DeploymentOperation>,
    completed: Vec<usize>,
    recovery: ErrorRecovery,
}

#[derive(Clone)]
pub enum DeploymentOperation {
    CreateContainer {
        image: String,
        name: String,
    },
    StartContainer {
        container_id: String,
    },
    StopContainer {
        container_id: String,
    },
    RemoveContainer {
        container_id: String,
    },
    ExecuteCommand {
        command: String,
        critical: bool,
    },
}

impl DeploymentTransaction {
    pub fn new(recovery_strategy: RecoveryStrategy) -> Self {
        Self {
            operations: Vec::new(),
            completed: Vec::new(),
            recovery: ErrorRecovery::new(recovery_strategy),
        }
    }

    /// Add an operation to the transaction
    pub fn add_operation(&mut self, operation: DeploymentOperation) {
        self.operations.push(operation);
    }

    /// Execute all operations with automatic rollback on failure
    pub async fn execute(
        &mut self,
        client: &SshDeploymentClient,
    ) -> Result<()> {
        for (index, operation) in self.operations.iter().enumerate() {
            match self.execute_operation(client, operation).await {
                Ok(()) => {
                    self.completed.push(index);
                    self.recovery.checkpoint(DeploymentCheckpoint {
                        instance_id: format!("ssh-deployment-{}", uuid::Uuid::new_v4()),
                        container_id: None, // Would be set based on operation
                        timestamp: std::time::SystemTime::now(),
                        state: self.operation_to_checkpoint_state(operation),
                    });
                }
                Err(e) => {
                    error!("Operation {} failed: {}, rolling back", index, e);
                    self.rollback(client).await?;
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    async fn execute_operation(
        &self,
        _client: &SshDeploymentClient,
        operation: &DeploymentOperation,
    ) -> Result<()> {
        match operation {
            DeploymentOperation::CreateContainer { image, name } => {
                info!("Creating container {} from image {}", name, image);
                // client.create_container(image, name).await
                Ok(())
            }
            DeploymentOperation::StartContainer { container_id } => {
                info!("Starting container {}", container_id);
                // client.start_container(container_id).await
                Ok(())
            }
            DeploymentOperation::ExecuteCommand { command, critical: _ } => {
                info!("Executing command: {}", command);
                // let result = client.execute_command(command).await;
                // if *critical { result } else { Ok(()) }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn rollback(&mut self, client: &SshDeploymentClient) -> Result<()> {
        warn!("Rolling back {} completed operations", self.completed.len());

        // Rollback in reverse order
        for &index in self.completed.iter().rev() {
            let operation = &self.operations[index];
            self.rollback_operation(client, operation).await?;
        }

        Ok(())
    }

    async fn rollback_operation(
        &self,
        _client: &SshDeploymentClient,
        operation: &DeploymentOperation,
    ) -> Result<()> {
        match operation {
            DeploymentOperation::CreateContainer { name, .. } => {
                info!("Rolling back: removing container {}", name);
                // client.remove_container(name).await
            }
            DeploymentOperation::StartContainer { container_id } => {
                info!("Rolling back: stopping container {}", container_id);
                // client.stop_container(container_id).await
            }
            _ => {
                // Some operations don't need rollback
            }
        }
        Ok(())
    }

    fn operation_to_checkpoint_state(&self, operation: &DeploymentOperation) -> CheckpointState {
        match operation {
            DeploymentOperation::CreateContainer { .. } => CheckpointState::ContainerCreated,
            DeploymentOperation::StartContainer { .. } => CheckpointState::ContainerStarted,
            _ => CheckpointState::PreDeployment,
        }
    }
}

/// Circuit breaker for preventing cascading failures
pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    state: CircuitState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<std::time::Instant>,
}

#[derive(Debug, PartialEq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold,
            timeout,
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
        }
    }

    pub async fn call<F, T>(&mut self, operation: F) -> Result<T>
    where
        F: futures::future::Future<Output = Result<T>>,
    {
        // Check if circuit should transition from Open to HalfOpen
        if self.state == CircuitState::Open {
            if let Some(last_failure) = self.last_failure_time {
                if last_failure.elapsed() >= self.timeout {
                    info!("Circuit breaker transitioning to half-open");
                    self.state = CircuitState::HalfOpen;
                }
            }
        }

        match self.state {
            CircuitState::Open => {
                Err(Error::Other("Circuit breaker is open".into()))
            }
            CircuitState::Closed | CircuitState::HalfOpen => {
                match operation.await {
                    Ok(result) => {
                        self.on_success();
                        Ok(result)
                    }
                    Err(e) => {
                        self.on_failure();
                        Err(e)
                    }
                }
            }
        }
    }

    fn on_success(&mut self) {
        self.failure_count = 0;

        if self.state == CircuitState::HalfOpen {
            self.success_count += 1;
            if self.success_count >= self.success_threshold {
                info!("Circuit breaker closing after successful operations");
                self.state = CircuitState::Closed;
                self.success_count = 0;
            }
        }
    }

    fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(std::time::Instant::now());

        if self.state == CircuitState::HalfOpen {
            warn!("Circuit breaker reopening after failure in half-open state");
            self.state = CircuitState::Open;
            self.success_count = 0;
        } else if self.failure_count >= self.failure_threshold {
            error!("Circuit breaker opening after {} failures", self.failure_count);
            self.state = CircuitState::Open;
        }
    }
}