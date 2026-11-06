//! GPU hardware mapping from the SDK-owned catalog. Prices come from provider quotes.
use crate::core::resources::ResourceSpec;
use crate::providers::common::{InstanceSelection, gpu_adapter::catalog_selection};

pub struct LambdaLabsInstanceMapper;

impl LambdaLabsInstanceMapper {
    pub fn map(spec: &ResourceSpec) -> InstanceSelection {
        catalog_selection(spec, "lambda-labs")
    }
}
