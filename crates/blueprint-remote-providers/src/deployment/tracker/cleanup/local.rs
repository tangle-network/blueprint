//! Local deployment cleanup handlers

use super::super::types::{CleanupHandler, DeploymentRecord};
use crate::core::error::{Error, Result};
use tracing::{info, warn};

/// Local Docker cleanup
pub(crate) struct LocalDockerCleanup;

#[async_trait::async_trait]
impl CleanupHandler for LocalDockerCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(container_id) = deployment.resource_ids.get("container_id") {
            info!("Cleaning up Docker container: {}", container_id);

            let output = tokio::process::Command::new("docker")
                .args(["rm", "-f", container_id])
                .output()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Docker cleanup failed: {e}")))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("No such container") {
                    return Err(Error::ConfigurationError(format!(
                        "Docker rm failed: {stderr}"
                    )));
                }
            }
        }

        Ok(())
    }
}

/// Local Kubernetes cleanup
pub(crate) struct LocalKubernetesCleanup;

#[async_trait::async_trait]
impl CleanupHandler for LocalKubernetesCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        let namespace = deployment
            .resource_ids
            .get("namespace")
            .map(|s| s.as_str())
            .unwrap_or("default");

        if let Some(pod_name) = deployment.resource_ids.get("pod") {
            info!("Cleaning up Kubernetes pod: {}/{}", namespace, pod_name);

            let output = tokio::process::Command::new("kubectl")
                .args([
                    "delete",
                    "pod",
                    pod_name,
                    "-n",
                    namespace,
                    "--grace-period=30",
                ])
                .output()
                .await
                .map_err(|e| Error::ConfigurationError(format!("kubectl cleanup failed: {e}")))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("NotFound") {
                    return Err(Error::ConfigurationError(format!(
                        "kubectl delete failed: {stderr}"
                    )));
                }
            }
        }

        // Also cleanup any services, configmaps, etc.
        for (resource_type, resource_name) in &deployment.resource_ids {
            if resource_type != "pod" && resource_type != "namespace" {
                let _ = tokio::process::Command::new("kubectl")
                    .args(["delete", resource_type, resource_name, "-n", namespace])
                    .output()
                    .await;
            }
        }

        Ok(())
    }
}

/// Local Hypervisor cleanup (Cloud Hypervisor)
pub(crate) struct LocalHypervisorCleanup;

#[async_trait::async_trait]
impl CleanupHandler for LocalHypervisorCleanup {
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()> {
        if let Some(vm_id) = deployment.resource_ids.get("vm_id") {
            info!("Cleaning up Cloud Hypervisor VM: {}", vm_id);

            // Send shutdown signal to Cloud Hypervisor API
            if let Some(api_socket) = deployment.resource_ids.get("api_socket") {
                let client = reqwest::Client::new();
                let _ = client
                    .put(format!("http://localhost/{api_socket}/shutdown"))
                    .send()
                    .await;
            }

            // Terminate the process if still running
            if let Some(pid_str) = deployment.resource_ids.get("pid") {
                if let Ok(pid_num) = pid_str.parse::<i32>() {
                    if let Err(e) = Self::safe_terminate_process(pid_num).await {
                        warn!("Failed to terminate process {}: {}", pid_num, e);
                    }
                }
            }

            // Clean up disk images and sockets
            if let Some(disk_path) = deployment.resource_ids.get("disk_image") {
                let _ = tokio::fs::remove_file(disk_path).await;
            }
        }

        Ok(())
    }
}

impl LocalHypervisorCleanup {
    /// Safely terminate a process by PID.
    ///
    /// This function validates the PID exists before attempting termination,
    /// first tries SIGTERM for graceful shutdown, then SIGKILL if needed.
    ///
    /// # Safety
    ///
    /// This function uses `libc::kill` which is unsafe. We mitigate risks by:
    /// 1. Checking if the PID exists before sending signals
    /// 2. Only killing PIDs that we explicitly tracked (stored in deployment record)
    /// 3. Using standard signal handling (SIGTERM then SIGKILL)
    ///
    /// # Errors
    ///
    /// Returns error if signal sending fails or process doesn't exist.
    async fn safe_terminate_process(pid: i32) -> Result<()> {
        // Validate PID is positive (defensive programming)
        if pid <= 0 {
            return Err(crate::core::error::Error::ConfigurationError(
                format!("Invalid PID: {}", pid),
            ));
        }

        // Check if process exists by sending signal 0 (no-op signal for process existence check)
        let exists = unsafe { libc::kill(pid, 0) == 0 };

        if !exists {
            info!("Process {} already terminated", pid);
            return Ok(());
        }

        info!("Sending SIGTERM to process {}", pid);

        // Send SIGTERM for graceful shutdown
        // SAFETY: We've validated the PID exists and is positive. This PID was stored
        // by us when we created the process, so we have permission to terminate it.
        let result = unsafe { libc::kill(pid, libc::SIGTERM) };

        if result != 0 {
            return Err(crate::core::error::Error::ConfigurationError(
                format!("Failed to send SIGTERM to process {}", pid),
            ));
        }

        // Wait for graceful shutdown
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Check if process is still running
        let still_running = unsafe { libc::kill(pid, 0) == 0 };

        if still_running {
            info!("Process {} did not terminate gracefully, sending SIGKILL", pid);

            // Force kill if still running
            // SAFETY: Same safety considerations as above
            let result = unsafe { libc::kill(pid, libc::SIGKILL) };

            if result != 0 {
                return Err(crate::core::error::Error::ConfigurationError(
                    format!("Failed to send SIGKILL to process {}", pid),
                ));
            }
        }

        Ok(())
    }
}
