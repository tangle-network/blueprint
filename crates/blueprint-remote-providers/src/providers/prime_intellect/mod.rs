//! Prime Intellect — compute aggregator that mediates between users and backend
//! GPU providers (CoreWeave, Lambda Labs, Crusoe, etc.).
//!
//! <https://docs.primeintellect.ai/api-reference>
//!
//! Prime Intellect picks the cheapest backend matching the requested GPU type when
//! `provider_preference` is `auto`. Operators can pin a specific sub-provider via
//! `PRIME_INTELLECT_PROVIDER` if compliance, latency, or VRAM ECC needs require it.
//!
//! Status: production-ready. The REST surface is stable and the aggregator routinely
//! brokers H100 / A100 / L40S inventory across multiple sub-providers.
//!
//! Env vars:
//! - `PRIME_INTELLECT_API_KEY` — Bearer token from the Prime Intellect dashboard.
//! - `PRIME_INTELLECT_REGION` — preferred region slug (default `us-east`).
//! - `PRIME_INTELLECT_PROVIDER` — sub-provider preference (default `auto`).
//! - `PRIME_INTELLECT_IMAGE` — container/VM image override (default Ubuntu 22.04 + CUDA).
//! - `PRIME_INTELLECT_SSH_KEY_PATH` — path to the SSH private key matching the
//!   pubkey registered with the Prime Intellect account.

mod adapter;
mod instance_mapper;

pub use adapter::PrimeIntellectAdapter;
pub use instance_mapper::PrimeIntellectInstanceMapper;
