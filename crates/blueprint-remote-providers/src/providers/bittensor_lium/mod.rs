//! Lium GPU rentals through its documented API.
//! <https://docs.lium.io/developers/quickstart>
//! Quotes use rent-by-spec dry runs; allocation passes the same GPU requirements.
//! Credentials may be supplied by the caller or LIUM_API_KEY.
//! Per-lease SSH public keys come from ResourceSpec, never a wallet or account key.

mod adapter;
mod instance_mapper;

pub use adapter::BittensorLiumAdapter;
pub use instance_mapper::BittensorLiumInstanceMapper;
