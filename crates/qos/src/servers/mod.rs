//! Server management module for QoS
//!
//! This module provides functionality to start and manage Grafana and Loki servers
//! for metrics visualization and log aggregation.

pub mod grafana;
pub mod loki;
pub mod common;

use crate::error::Result;
use std::sync::Arc;
use tokio::sync::oneshot;

/// Common trait for server management
#[async_trait::async_trait]
pub trait ServerManager: Send + Sync {
    /// Start the server
    ///
    /// # Errors
    /// Returns an error if the server fails to start
    async fn start(&self) -> Result<()>;

    /// Stop the server
    ///
    /// # Errors
    /// Returns an error if the server fails to stop
    async fn stop(&self) -> Result<()>;

    /// Get the URL of the server
    fn url(&self) -> String;

    /// Check if the server is running
    ///
    /// # Errors
    /// Returns an error if the check fails
    async fn is_running(&self) -> Result<bool>;

    /// Wait for the server to be ready
    ///
    /// # Errors
    /// Returns an error if the server fails to become ready within the timeout
    async fn wait_until_ready(&self, timeout_secs: u64) -> Result<()>;
}
