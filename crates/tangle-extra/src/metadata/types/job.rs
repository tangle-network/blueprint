use crate::serde::new_bounded_string;
use alloc::borrow::Cow;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::FieldType;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::jobs::JobDefinition as SubxtJobDefinition;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::jobs::JobMetadata as SubxtJobMetadata;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::PricingModel;

/// A Job Definition is a definition of a job that can be called.
/// It contains the input and output fields of the job with the permitted caller.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct JobDefinition<'a> {
    pub job_id: u64,
    /// The metadata of the job.
    pub metadata: JobMetadata<'a>,
    /// These are parameters that are required for this job.
    /// i.e. the input.
    pub params: Vec<FieldType>,
    /// These are the result, the return values of this job.
    /// i.e. the output.
    pub result: Vec<FieldType>,
    /// The pricing model for the job.
    pub pricing_model: PricingModel<u32, u128>,
}

impl From<SubxtJobDefinition> for JobDefinition<'static> {
    fn from(value: SubxtJobDefinition) -> Self {
        Self {
            job_id: 0,
            metadata: value.metadata.into(),
            params: value.params.0,
            result: value.result.0,
            pricing_model: value.pricing_model,
        }
    }
}

impl From<JobDefinition<'_>> for SubxtJobDefinition {
    fn from(value: JobDefinition<'_>) -> Self {
        Self {
            metadata: value.metadata.into(),
            params: BoundedVec(value.params),
            result: BoundedVec(value.result),
            pricing_model: value.pricing_model,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct JobMetadata<'a> {
    /// The Job name.
    pub name: Cow<'a, str>,
    /// The Job description.
    pub description: Option<Cow<'a, str>>,
}

impl From<SubxtJobMetadata> for JobMetadata<'static> {
    fn from(value: SubxtJobMetadata) -> Self {
        Self {
            name: String::from_utf8_lossy(&value.name.0.0).into_owned().into(),
            description: value
                .description
                .map(|desc| String::from_utf8_lossy(&desc.0.0).into_owned().into()),
        }
    }
}

impl From<JobMetadata<'_>> for SubxtJobMetadata {
    fn from(value: JobMetadata<'_>) -> Self {
        Self {
            name: new_bounded_string(value.name),
            description: value.description.map(new_bounded_string),
        }
    }
}

impl Default for JobDefinition<'_> {
    fn default() -> Self {
        Self {
            job_id: 0,
            metadata: JobMetadata::default(),
            params: Vec::new(),
            result: Vec::new(),
            pricing_model: PricingModel::PayOnce { amount: 0 },
        }
    }
}
