//! Cloud provider implementations

pub mod common;

#[cfg(feature = "aws")]
pub mod aws;

pub mod akash;
pub mod azure;
pub mod bittensor_lium;
pub mod coreweave;
pub mod digitalocean;
pub mod fluidstack;
pub mod gcp;
pub mod io_net;
pub mod lambda_labs;
pub mod paperspace;
pub mod prime_intellect;
pub mod render;
pub mod runpod;
pub mod tensordock;
pub mod vast_ai;
pub mod vultr;

pub use common::{
    CloudProvisioner, InstanceSelection, ProvisionedInfrastructure, ProvisioningConfig,
};

#[cfg(feature = "aws")]
pub use aws::{AwsInstanceMapper, AwsProvisioner};

pub use akash::AkashAdapter;
pub use bittensor_lium::BittensorLiumAdapter;
pub use coreweave::CoreWeaveAdapter;
pub use fluidstack::FluidstackAdapter;
pub use io_net::IoNetAdapter;
pub use lambda_labs::LambdaLabsAdapter;
pub use paperspace::PaperspaceAdapter;
pub use prime_intellect::PrimeIntellectAdapter;
pub use render::RenderAdapter;
pub use runpod::RunPodAdapter;
pub use tensordock::TensorDockAdapter;
pub use vast_ai::VastAiAdapter;
