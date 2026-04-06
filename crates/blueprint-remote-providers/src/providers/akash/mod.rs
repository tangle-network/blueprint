//! Akash Network — Cosmos-based decentralized GPU compute marketplace.
//!
//! <https://akash.network/docs/>
//!
//! Akash is fundamentally different from a traditional REST cloud:
//! tenants submit a deployment described by an SDL (Stack Definition Language)
//! manifest, providers on the network bid on it, the tenant accepts a bid, a
//! lease is created on-chain, and the provider runs the workload. Payment flows
//! through Cosmos SDK transactions denominated in `uakt`.
//!
//! ## REST-adapter-over-relay design
//!
//! Implementing a native Cosmos transaction signer in this crate would pull in a
//! large dependency tree (cosmrs, secp256k1, bip39, …) for a feature most users
//! won't enable. Instead, this adapter assumes a thin **relay service** sitting
//! in front of an Akash CLI / Cosmos SDK runtime that exposes a small REST
//! surface and handles the actual on-chain transactions:
//!
//! - `POST {AKASH_RPC_URL}/deployments` — body `{sdl, lease_budget_uakt}`,
//!   returns `{deployment_id, bids[]}`
//! - `POST {AKASH_RPC_URL}/deployments/{id}/accept` — body `{provider_address}`,
//!   returns `{lease_id, public_ip}`
//! - `GET  {AKASH_RPC_URL}/deployments/{id}` — returns
//!   `{status, lease_info, public_ip}`
//! - `DELETE {AKASH_RPC_URL}/deployments/{id}` — close the deployment
//!
//! Lease IDs are surfaced as `dseq/gseq/oseq` triples and stored as the
//! `ProvisionedInstance::id`.
//!
//! ## Env vars
//! - `AKASH_RPC_URL` — base URL of the relay (e.g. `https://api.akash.network`).
//! - `AKASH_API_TOKEN` — bearer token for the relay (optional but recommended).
//! - `AKASH_REGION` — default region label for `ProvisionedInstance.region`.
//! - `AKASH_LEASE_BUDGET_UAKT` — max budget per lease in `uakt` (default 5_000_000).
//! - `AKASH_PROVIDER_ADDRESS` — optional preferred provider; otherwise the
//!   relay picks the cheapest bid.
//! - `AKASH_SSH_KEY_PATH` — path to the private SSH key for blueprint deploy.

mod adapter;
mod instance_mapper;
mod sdl;

pub use adapter::AkashAdapter;
pub use instance_mapper::AkashInstanceMapper;
pub use sdl::build_sdl_manifest;
