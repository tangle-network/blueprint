//! Shared provisioning requirements, also usable without cloud adapters.
pub use blueprint_core::resources::{QosParameters, ResourceSpec};

pub fn to_pricing_units(spec: &ResourceSpec) -> std::collections::HashMap<String, f64> {
    spec.to_pricing_units().into_iter().collect()
}
