//! Maps `ResourceSpec` to OVH Public Cloud flavor names.
//!
//! The adapter resolves a name to the region's flavor id, so a flavor a region lacks fails at
//! provisioning with OVH's own listing. Prices come from OVH's catalog, not from here.

use crate::core::resources::ResourceSpec;
use crate::providers::common::InstanceSelection;

pub struct OvhInstanceMapper;

impl OvhInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        let fits = |cpu: f32, memory: f32| spec.cpu <= cpu && spec.memory_gb <= memory;
        let flavor = match spec.gpu_count.unwrap_or(0) {
            0 if fits(1.0, 2.0) => "d2-2",
            0 if fits(2.0, 4.0) => "d2-4",
            0 if fits(4.0, 8.0) => "d2-8",
            0 if fits(4.0, 15.0) => "b2-15",
            0 if fits(8.0, 30.0) => "b2-30",
            0 => "b2-60",
            1 => "t1-45",
            2 => "t1-90",
            _ => "t1-180",
        };
        InstanceSelection {
            instance_type: flavor.to_owned(),
            spot_capable: false,
            estimated_hourly_cost: None,
        }
    }
}
