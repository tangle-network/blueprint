//! Hetzner Cloud — European cloud with dedicated GPU servers.
//!
//! <https://docs.hetzner.cloud/>
//!
//! Env vars:
//! - `HETZNER_API_TOKEN` — API token from the Hetzner Cloud Console.
//! - `HETZNER_REGION` — datacenter slug, e.g. `fsn1`, `nbg1`, `hel1`, `ash`, `hil`.
//! - `HETZNER_SSH_KEY_NAME` — SSH key name pre-registered in the console.

mod adapter;
mod instance_mapper;

pub use adapter::HetznerAdapter;
pub use instance_mapper::HetznerInstanceMapper;
