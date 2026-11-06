//! OVHcloud Public Cloud — OpenStack-based instances.
//!
//! <https://api.ovh.com/console/#/cloud>
//!
//! Env vars:
//! - `OVH_ENDPOINT` — `ovh-eu`, `ovh-ca`, `ovh-us`, or an API root URL.
//! - `OVH_APPLICATION_KEY`, `OVH_APPLICATION_SECRET`, `OVH_CONSUMER_KEY` — an API key whose
//!   rules cover `/cloud/project/*`.
//! - `OVH_CLOUD_PROJECT_SERVICE` — the Public Cloud project id.
//! - `OVH_REGION` — default region, e.g. `GRA11`, `US-EAST-VA-1`.
//! - `OVH_IMAGE` — image name, default `Ubuntu 22.04`; `OVH_SSH_KEY_ID` — registered key id.

mod adapter;
mod instance_mapper;

pub use adapter::OvhAdapter;
pub use instance_mapper::OvhInstanceMapper;
