//! Vultr cloud provider implementation

pub mod adapter;
pub mod provisioner;

pub use adapter::VultrAdapter;
pub use provisioner::VultrProvisioner;