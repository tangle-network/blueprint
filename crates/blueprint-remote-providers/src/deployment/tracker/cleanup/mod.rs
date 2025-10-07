//! Cleanup handlers for different deployment types

mod cloud_vms;
mod kubernetes;
mod local;
mod ssh;

pub(super) use cloud_vms::{AwsCleanup, AzureCleanup, DigitalOceanCleanup, GcpCleanup, VultrCleanup};
pub(super) use kubernetes::{AksCleanup, EksCleanup, GkeCleanup};
pub(super) use local::{LocalDockerCleanup, LocalHypervisorCleanup, LocalKubernetesCleanup};
pub(super) use ssh::SshCleanup;
