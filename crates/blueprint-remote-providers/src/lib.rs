#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod provider;
pub mod types;

#[cfg(feature = "kubernetes")]
pub mod kubernetes;

#[cfg(feature = "docker")]
pub mod docker;

#[cfg(feature = "ssh")]
pub mod ssh;

#[cfg(feature = "tunnel")]
pub mod tunnel;

#[cfg(feature = "testing")]
pub mod testing;

#[cfg(feature = "blueprint-manager")]
pub mod manager_integration;

pub use error::{Error, Result};
pub use provider::{RemoteInfrastructureProvider, ProviderRegistry};
pub use types::{
    Cost, DeploymentSpec, InstanceId, InstanceStatus, RemoteInstance, 
    Resources, ResourceLimits, TunnelConfig, TunnelHandle,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");