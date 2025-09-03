#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod bridge;
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

pub use bridge::{RemoteBridgeManager, BridgeConnection, ConnectionStatus, ConnectionType};
pub use error::{Error, Result};
pub use provider::{RemoteInfrastructureProvider, ProviderRegistry, ProviderType};
pub use types::{
    ContainerImage, Cost, DeploymentSpec, InstanceId, InstanceStatus, 
    PortMapping, Protocol, PullPolicy, RemoteInstance, Resources, 
    ResourceLimits, ServiceEndpoint, TunnelConfig, TunnelHandle, TunnelHub,
    VolumeMount,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");