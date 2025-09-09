//! Cloud provider implementations

pub mod common;

#[cfg(feature = "aws")]
pub mod aws;

pub mod gcp;
pub mod azure;
pub mod digitalocean;
pub mod vultr;

pub use common::{CloudProvisioner, InstanceSelection, ProvisionedInfrastructure, ProvisioningConfig};

#[cfg(feature = "aws")]
pub use aws::{AwsProvisioner, AwsInstanceMapper};