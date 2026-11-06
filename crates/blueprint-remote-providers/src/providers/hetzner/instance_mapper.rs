//! GPU hardware mapping from the SDK-owned catalog. Prices come from provider quotes.
use crate::core::resources::ResourceSpec;
use crate::providers::common::{InstanceSelection, gpu_adapter::catalog_selection};

pub struct HetznerInstanceMapper;

impl HetznerInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        if spec.gpu_count.is_some() || spec.gpu_class.is_some() {
            return catalog_selection(spec, "hetzner");
        }
        let sku = match (spec.cpu, spec.memory_gb) {
            (cpu, mem) if cpu <= 2.0 && mem <= 4.0 => "cpx11",
            (cpu, mem) if cpu <= 3.0 && mem <= 8.0 => "cpx21",
            (cpu, mem) if cpu <= 4.0 && mem <= 16.0 => "cpx31",
            (cpu, mem) if cpu <= 8.0 && mem <= 32.0 => "cpx41",
            _ => "cpx51",
        };
        InstanceSelection {
            instance_type: sku.into(),
            spot_capable: false,
            estimated_hourly_cost: None,
        }
    }
}
