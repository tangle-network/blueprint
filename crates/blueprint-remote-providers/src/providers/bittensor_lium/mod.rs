//! Lium executor rentals and recoverable private workload templates.
//! Native API: <https://docs.lium.io/developers/quickstart>.
//! Native volumes attach at `/mnt`; unsupported size policies fail closed.

mod adapter;

pub use adapter::BittensorLiumAdapter;
