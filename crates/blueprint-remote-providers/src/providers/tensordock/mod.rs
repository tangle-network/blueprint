//! TensorDock Marketplace host pricing and VM lifecycle.
//! <https://documenter.getpostman.com/view/20973002/2s8YzMYRDc>
//! The documented Marketplace API uses form bodies with key/token authentication.
//! Credentials may be supplied by the caller or TENSORDOCK_API_KEY/TENSORDOCK_API_TOKEN.

mod adapter;
mod instance_mapper;

pub use adapter::TensorDockAdapter;
pub use instance_mapper::TensorDockInstanceMapper;
