//! io.net — REST-based decentralized GPU cloud (io.cloud product line).
//!
//! <https://docs.io.net/reference>
//!
//! io.net rents GPU compute as multi-node *clusters* rather than individual VMs.
//! For the blueprint-manager case we model a cluster as a single-node provisioning
//! unit and route SSH deployment at the first cluster node's public IP.
//!
//! Env vars:
//! - `IO_NET_API_KEY` — bearer token from the io.net dashboard.
//! - `IO_NET_REGION` — default region slug, e.g. `us`.
//! - `IO_NET_CLUSTER_TYPE` — `Ray`, `Kubernetes`, or `BareMetal` (default `BareMetal`).
//! - `IO_NET_DURATION_HOURS` — rental duration when launching a cluster (default `1`).
//! - `IO_NET_SSH_KEY_PATH` — local path to SSH private key for SSH deployment.

mod adapter;
mod instance_mapper;

pub use adapter::IoNetAdapter;
pub use instance_mapper::IoNetInstanceMapper;
