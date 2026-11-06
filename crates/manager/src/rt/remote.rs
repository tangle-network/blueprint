//! Remote runtime backed by the SDK lease that owns allocation and retirement.

use super::service::Status;
use crate::error::{Error, Result};
use blueprint_remote_providers::deployment::secure_ssh::SecureSshClient;
use blueprint_remote_providers::{Lease, ResourceSpec};

pub struct RemoteServiceInstance {
    lease: Lease,
    resources: ResourceSpec,
    status: Status,
    process: Option<(SecureSshClient, u32, tempfile::TempDir)>,
}

impl RemoteServiceInstance {
    pub fn from_lease(lease: Lease, resources: ResourceSpec) -> Self {
        Self {
            lease,
            resources,
            status: Status::NotStarted,
            process: None,
        }
    }

    /// Retain the per-lease key and host pin while monitoring the deployed process.
    pub fn with_process(
        mut self,
        ssh: SecureSshClient,
        pid: u32,
        access: tempfile::TempDir,
    ) -> Self {
        self.process = Some((ssh, pid, access));
        self
    }

    pub async fn start(&mut self) -> Result<()> {
        self.resources.validate().map_err(Error::Other)?;
        if !self.lease.is_active().await? {
            self.status = Status::Finished;
            return Err(Error::Other(
                "Remote service lease has expired or terminated".into(),
            ));
        }
        if let Some((ssh, pid, _)) = &self.process {
            ssh.run_script(&format!("kill -0 {pid}")).await?;
        }
        self.status = Status::Running;
        Ok(())
    }

    pub async fn status(&self) -> Result<Status> {
        if !self.lease.is_active().await? {
            return Ok(Status::Finished);
        }
        if let Some((ssh, pid, _)) = &self.process {
            if *pid < 2 || ssh.run_script(&format!("kill -0 {pid}")).await.is_err() {
                return Ok(Status::Error);
            }
        }
        Ok(self.status)
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.lease.terminate().await?;
        self.status = Status::Finished;
        Ok(())
    }

    pub async fn logs(&self, _lines: usize) -> Result<Vec<String>> {
        Err(Error::Other(
            "Fetch workload logs through the allocated SSH endpoint".into(),
        ))
    }
}
