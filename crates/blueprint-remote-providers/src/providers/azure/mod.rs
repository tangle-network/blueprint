//! Microsoft Azure provider implementation
//! 
//! This module provides Azure Resource Manager integration for provisioning
//! and deploying Blueprint containers to Azure Virtual Machines.

pub mod adapter;
pub mod provisioner;

pub use adapter::AzureAdapter;
pub use provisioner::AzureProvisioner;