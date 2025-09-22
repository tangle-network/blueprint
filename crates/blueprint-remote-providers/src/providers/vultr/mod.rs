//! Vultr cloud provider implementation

pub mod adapter;

pub use adapter::VultrAdapter;

// TODO: Implement VultrProvisioner for actual instance provisioning
// pub mod provisioner;
// pub use provisioner::VultrProvisioner;