//! CoreWeave — K8s-native GPU cloud. Unlike the other GPU providers, CoreWeave
//! exposes its capacity through a managed Kubernetes offering rather than a
//! bespoke REST instance API. The adapter delegates to `SharedKubernetesDeployment`.
//!
//! <https://docs.coreweave.com/>
//!
//! Env vars:
//! - `COREWEAVE_TOKEN` — enables this adapter in the environment-based factory;
//!   Kubernetes authentication still comes from the configured kubeconfig.
//! - `COREWEAVE_NAMESPACE` — tenant Kubernetes namespace.
//! - `KUBECONFIG` — path to the pre-configured kubeconfig.
//!
//! Raw VM creation, status, termination, and deployment cleanup return
//! `Unsupported`; this adapter cannot prove an instance or Kubernetes workload
//! was retired. Use the Kubernetes deployment owner for cleanup.

mod adapter;
mod instance_mapper;

pub use adapter::CoreWeaveAdapter;
pub use instance_mapper::CoreWeaveInstanceMapper;
