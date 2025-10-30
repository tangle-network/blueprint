//! SSH tunnel support for secure QoS metrics collection
//!
//! This module provides SSH tunneling capabilities to securely access
//! QoS metrics from remote deployments without exposing ports publicly.

use crate::core::error::{Error, Result};
use blueprint_core::{info, warn};
use std::process::Stdio;
use tokio::process::{Child, Command};

/// SSH tunnel for QoS metrics collection
pub struct QosTunnel {
    /// SSH tunnel process handle
    process: Option<Child>,
    /// Local port for tunnel
    local_port: u16,
    /// Remote host
    remote_host: String,
    /// Remote port (usually 9615 for QoS)
    remote_port: u16,
    /// SSH user
    ssh_user: String,
    /// SSH key path (optional)
    ssh_key_path: Option<String>,
}

impl QosTunnel {
    /// Create a new QoS tunnel configuration
    pub fn new(
        local_port: u16,
        remote_host: String,
        remote_port: u16,
        ssh_user: String,
        ssh_key_path: Option<String>,
    ) -> Self {
        Self {
            process: None,
            local_port,
            remote_host,
            remote_port,
            ssh_user,
            ssh_key_path,
        }
    }

    /// Establish SSH tunnel for QoS metrics
    pub async fn connect(&mut self) -> Result<()> {
        info!(
            "Creating SSH tunnel for QoS metrics: localhost:{} -> {}@{}:{}",
            self.local_port, self.ssh_user, self.remote_host, self.remote_port
        );

        let mut cmd = Command::new("ssh");

        // Basic SSH options for tunneling
        cmd.arg("-N") // Don't execute remote command
            .arg("-L")
            .arg(format!(
                "{}:localhost:{}",
                self.local_port, self.remote_port
            ))
            .arg(format!("{}@{}", self.ssh_user, self.remote_host))
            .arg("-o")
            .arg("StrictHostKeyChecking=accept-new")
            .arg("-o")
            .arg("ServerAliveInterval=30")
            .arg("-o")
            .arg("ServerAliveCountMax=3");

        // Add SSH key if provided
        if let Some(ref key_path) = self.ssh_key_path {
            cmd.arg("-i").arg(key_path);
        }

        // Start tunnel in background
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = cmd
            .spawn()
            .map_err(|e| Error::ConfigurationError(format!("Failed to start SSH tunnel: {e}")))?;

        self.process = Some(child);

        // Give tunnel time to establish
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Verify tunnel is working by checking if local port is open
        match tokio::net::TcpStream::connect(format!("127.0.0.1:{}", self.local_port)).await {
            Ok(_) => {
                info!(
                    "QoS tunnel established successfully on localhost:{}",
                    self.local_port
                );
                Ok(())
            }
            Err(e) => {
                warn!("QoS tunnel may not be ready yet: {}", e);
                // Don't fail immediately - tunnel might still be establishing
                Ok(())
            }
        }
    }

    /// Get the local endpoint for QoS metrics collection
    pub fn get_local_endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.local_port)
    }

    /// Disconnect the SSH tunnel
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            info!("Closing QoS tunnel on localhost:{}", self.local_port);

            // Try graceful shutdown first
            if let Err(e) = process.kill().await {
                warn!("Failed to kill SSH tunnel process: {}", e);
            }

            // Wait for process to exit
            let _ = process.wait().await;
        }

        Ok(())
    }

    /// Check if tunnel is still active
    pub async fn is_active(&self) -> bool {
        // Check if we can connect to the local port
        tokio::net::TcpStream::connect(format!("127.0.0.1:{}", self.local_port))
            .await
            .is_ok()
    }
}

impl Drop for QosTunnel {
    fn drop(&mut self) {
        // Ensure tunnel is closed on drop
        if let Some(mut process) = self.process.take() {
            // Try to kill the process (blocking in drop is not ideal but necessary)
            let _ = process.start_kill();
        }
    }
}

/// Manager for multiple QoS tunnels
pub struct QosTunnelManager {
    tunnels: Vec<QosTunnel>,
    next_local_port: u16,
}

impl QosTunnelManager {
    /// Create a new tunnel manager
    pub fn new(starting_port: u16) -> Self {
        Self {
            tunnels: Vec::new(),
            next_local_port: starting_port,
        }
    }

    /// Create and connect a new QoS tunnel
    pub async fn create_tunnel(
        &mut self,
        remote_host: String,
        ssh_user: String,
        ssh_key_path: Option<String>,
    ) -> Result<String> {
        let local_port = self.next_local_port;
        self.next_local_port += 1;

        let mut tunnel = QosTunnel::new(
            local_port,
            remote_host,
            9615, // Standard QoS port
            ssh_user,
            ssh_key_path,
        );

        tunnel.connect().await?;
        let endpoint = tunnel.get_local_endpoint();

        self.tunnels.push(tunnel);

        Ok(endpoint)
    }

    /// Close all tunnels
    pub async fn close_all(&mut self) -> Result<()> {
        for mut tunnel in self.tunnels.drain(..) {
            tunnel.disconnect().await?;
        }
        Ok(())
    }

    /// Get active tunnel count
    pub fn active_count(&self) -> usize {
        self.tunnels.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_configuration() {
        let tunnel = QosTunnel::new(
            19615,
            "remote-host.example.com".to_string(),
            9615,
            "ubuntu".to_string(),
            Some("/path/to/key".to_string()),
        );

        assert_eq!(tunnel.local_port, 19615);
        assert_eq!(tunnel.remote_port, 9615);
        assert_eq!(tunnel.get_local_endpoint(), "http://127.0.0.1:19615");
    }

    #[tokio::test]
    async fn test_tunnel_manager() {
        let manager = QosTunnelManager::new(20000);

        assert_eq!(manager.active_count(), 0);
        assert_eq!(manager.next_local_port, 20000);

        // Note: Actual connection test would require a real SSH server
        // This just tests the configuration
    }
}
