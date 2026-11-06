//! Lium GPU rentals through its documented API.
//! <https://docs.lium.io/developers/quickstart>
//! Quotes read live public node prices/stock, with nominal GPU identity from the
//! shared catalog. Rentals bind the exact selected node and cap its live price.
//! Credentials may be supplied by the caller or LIUM_API_KEY.
//! Per-lease SSH public keys come from ResourceSpec, never a wallet or account key.

mod adapter;
mod instance_mapper;

pub use adapter::BittensorLiumAdapter;
pub use instance_mapper::BittensorLiumInstanceMapper;
