//! CoreWeave — K8s-native GPU cloud. Unlike the other GPU providers, CoreWeave
//! exposes its capacity through a managed Kubernetes offering rather than a
//! bespoke REST instance API. The adapter delegates to `SharedKubernetesDeployment`.
//!
//! <https://docs.coreweave.com/>
//!
//! Env vars:
//! - `COREWEAVE_TOKEN` — API / kubeconfig token (used to pull kubeconfig).
//! - `COREWEAVE_REGION` — datacenter identifier (e.g. `ORD1`, `LAS1`).
//! - `COREWEAVE_NAMESPACE` — tenant Kubernetes namespace.
//! - `KUBECONFIG` — path to the pre-configured kubeconfig (takes precedence).

mod adapter;
mod instance_mapper;

pub use adapter::CoreWeaveAdapter;
pub use instance_mapper::CoreWeaveInstanceMapper;
