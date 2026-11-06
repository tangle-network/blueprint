//! GPU hardware mapping from the SDK-owned catalog. Prices come from provider quotes.
use crate::core::resources::ResourceSpec;
use crate::providers::common::{InstanceSelection, gpu_adapter::catalog_selection};

pub struct CrusoeInstanceMapper;

impl CrusoeInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        if spec.gpu_count.is_some() || spec.gpu_class.is_some() {
            return catalog_selection(spec, "crusoe");
        }
        let sku = match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 4.0 && mem <= 16.0 => "c1.4x16",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "c1.8x32",
            _ => "c1.16x64",
        };
        InstanceSelection {
            instance_type: sku.into(),
            spot_capable: false,
            estimated_hourly_cost: None,
        }
    }
}
