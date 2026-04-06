//! Paperspace — GPU cloud with a simple REST "machines" API.
//!
//! <https://docs.digitalocean.com/reference/paperspace/core/api/>
//!
//! Env vars:
//! - `PAPERSPACE_API_KEY` — API key from Paperspace console (X-Api-Key header).
//! - `PAPERSPACE_REGION` — region like `East Coast (NY2)`.

mod adapter;
mod instance_mapper;

pub use adapter::PaperspaceAdapter;
pub use instance_mapper::PaperspaceInstanceMapper;
