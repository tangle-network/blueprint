//! io.net GPU identity and count from the shared catalog.
use crate::core::resources::ResourceSpec;
use crate::providers::common::{InstanceSelection, gpu_adapter::catalog_selection};

pub struct IoNetInstanceMapper;
#[derive(Debug, Clone)]
pub struct IoNetSelection {
    pub gpu_type: String,
    pub gpu_count: u32,
}
impl IoNetInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        catalog_selection(spec, "io-net")
    }
    pub fn select(spec: &ResourceSpec) -> IoNetSelection {
        IoNetSelection {
            gpu_type: Self::map(spec).instance_type,
            gpu_count: spec.gpu_count.unwrap_or(1),
        }
    }
}
