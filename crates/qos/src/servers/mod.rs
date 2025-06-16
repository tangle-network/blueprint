pub mod common;
pub mod grafana;
pub mod loki;
pub mod prometheus;

use crate::error::Result;

/// Common trait for server management
pub trait ServerManager: Send + Sync {
    /// Start the server
    ///
    /// # Errors
    /// Returns an error if the server fails to start
    async fn start(&self, network: Option<&str>, bind_ip: Option<String>) -> Result<()>;

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
