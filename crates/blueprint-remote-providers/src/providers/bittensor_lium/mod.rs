//! Bittensor Lium — subnet 51 GPU rental marketplace.
//!
//! <https://subnetalpha.ai/subnet/lium>
//!
//! Lium runs as a Bittensor subnet (51) where validators match GPU renters with
//! miner-operated nodes. The HTTP surface provided here is a thin wrapper that
//! handles rental lifecycle (create / poll / terminate). **Payment settlement
//! happens out-of-band via the Bittensor CLI/SDK** — operators must have funded
//! hotkey/coldkey wallets before invoking this adapter; the HTTP layer never
//! touches TAO directly.
//!
//! Status: **best-effort / emerging integration.** Lium is primarily a Python
//! SDK; the REST shim documented at <https://docs.lium.ai> may evolve. The
//! adapter parses lenient JSON and the rental endpoints encoded here track the
//! public preview spec as of 2026-04. Lium specializes in H100 inventory (500+
//! cards across the subnet) — the mapper defaults there when the spec allows.
//!
//! Env vars:
//! - `LIUM_API_KEY` — Bearer token for the Lium REST shim.
//! - `LIUM_WALLET_HOTKEY` — Bittensor hotkey ss58 address (passed in rental body).
//! - `LIUM_WALLET_COLDKEY` — Bittensor coldkey ss58 address (passed in rental body).
//! - `LIUM_REGION` — preferred region/datacenter slug (default `auto`).
//! - `LIUM_DURATION_HOURS` — rental duration in hours (default `24`).
//! - `LIUM_SSH_PUBKEY` — SSH public key contents to inject into the rented miner.
//! - `LIUM_SSH_KEY_PATH` — matching private key path on the orchestrator.

mod adapter;
mod instance_mapper;

pub use adapter::BittensorLiumAdapter;
pub use instance_mapper::BittensorLiumInstanceMapper;
