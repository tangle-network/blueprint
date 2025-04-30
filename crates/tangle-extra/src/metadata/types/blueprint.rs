use super::job::JobDefinition;
use super::sources::BlueprintSource;
use crate::serde::new_bounded_string;
use alloc::borrow::Cow;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::field::FieldType;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::pricing::ResourcePricing;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::service::BlueprintServiceManager as SubxtBlueprintServiceManager;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::service::MasterBlueprintServiceManagerRevision;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::service::ServiceBlueprint as SubxtServiceBlueprint;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::service::ServiceMetadata as SubxtServiceMetadata;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::MembershipModelType;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServiceMetadata<'a> {
    /// The Service name.
    pub name: Cow<'a, str>,
    /// The Service description.
    pub description: Option<Cow<'a, str>>,
    /// The Service author.
    /// Could be a company or a person.
    pub author: Option<Cow<'a, str>>,
    /// The Job category.
    pub category: Option<Cow<'a, str>>,
    /// Code Repository URL.
    /// Could be a github, gitlab, or any other code repository.
    pub code_repository: Option<Cow<'a, str>>,
    /// Service Logo URL.
    pub logo: Option<Cow<'a, str>>,
    /// Service Website URL.
    pub website: Option<Cow<'a, str>>,
    /// Service License.
    pub license: Option<Cow<'a, str>>,
}

impl From<ServiceMetadata<'_>> for SubxtServiceMetadata {
    fn from(value: ServiceMetadata<'_>) -> Self {
        let ServiceMetadata {
            name,
            description,
            author,
            category,
            code_repository,
            logo,
            website,
            license,
        } = value;

        SubxtServiceMetadata {
            name: new_bounded_string(name),
            description: description.map(new_bounded_string),
            author: author.map(new_bounded_string),
            category: category.map(new_bounded_string),
            code_repository: code_repository.map(new_bounded_string),
            logo: logo.map(new_bounded_string),
            website: website.map(new_bounded_string),
            license: license.map(new_bounded_string),
        }
    }
}

/// Service Blueprint Manager is a smart contract that will manage the service lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum BlueprintServiceManager {
    /// A Smart contract that will manage the service lifecycle.
    Evm(String),
}

impl TryFrom<BlueprintServiceManager> for SubxtBlueprintServiceManager {
    type Error = super::error::Error;

    fn try_from(value: BlueprintServiceManager) -> Result<Self, Self::Error> {
        match value {
            BlueprintServiceManager::Evm(evm) => Ok(SubxtBlueprintServiceManager::Evm(
                evm.parse().map_err(|_| Self::Error::BadServiceManager)?,
            )),
        }
    }
}

/// Helper type for specifying resource requirements in a blueprint
/// This is used to make it easier to specify resource requirements in the blueprint macro
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BlueprintResourceRequirement {
    /// CPU cores or vCPUs
    CPU(u64),
    /// Memory in megabytes
    MemoryMB(u64),
    /// Storage in megabytes
    StorageMB(u64),
    /// Network egress in megabytes
    NetworkEgressMB(u64),
    /// Network ingress in megabytes
    NetworkIngressMB(u64),
    /// GPU units
    GPU(u64),
    /// Request count
    Request(u64),
    /// Invocation count
    Invocation(u64),
    /// Execution time in milliseconds
    ExecutionTimeMS(u64),
    /// Storage IO operations per second
    StorageIOPS(u64),
}

impl From<BlueprintResourceRequirement> for ResourcePricing {
    fn from(value: BlueprintResourceRequirement) -> Self {
        match value {
            BlueprintResourceRequirement::CPU(count) => ResourcePricing {
                kind: new_bounded_string("CPU"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::MemoryMB(count) => ResourcePricing {
                kind: new_bounded_string("MemoryMB"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::StorageMB(count) => ResourcePricing {
                kind: new_bounded_string("StorageMB"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::NetworkEgressMB(count) => ResourcePricing {
                kind: new_bounded_string("NetworkEgressMB"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::NetworkIngressMB(count) => ResourcePricing {
                kind: new_bounded_string("NetworkIngressMB"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::GPU(count) => ResourcePricing {
                kind: new_bounded_string("GPU"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::Request(count) => ResourcePricing {
                kind: new_bounded_string("Request"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::Invocation(count) => ResourcePricing {
                kind: new_bounded_string("Invocation"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::ExecutionTimeMS(count) => ResourcePricing {
                kind: new_bounded_string("ExecutionTimeMS"),
                count,
                price_per_unit_rate: 0,
            },
            BlueprintResourceRequirement::StorageIOPS(count) => ResourcePricing {
                kind: new_bounded_string("StorageIOPS"),
                count,
                price_per_unit_rate: 0,
            },
        }
    }
}

/// Mirror of [`ServiceBlueprint`] for un-deployed blueprints
///
/// This only exists, as the [`ServiceBlueprint`] uses `Vec<u8>` instead of `String` for string fields,
/// and expects an address for the manager contract, but we haven't yet deployed.
///
/// This needs to be kept up to date to reflect [`ServiceBlueprint`] otherwise.
///
/// [`ServiceBlueprint`]: SubxtServiceBlueprint
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServiceBlueprint<'a> {
    /// The metadata of the service.
    pub metadata: ServiceMetadata<'a>,
    /// The blueprint manager that will be used to manage the blueprints lifecycle.
    pub manager: BlueprintServiceManager,
    /// The Revision number of the Master Blueprint Service Manager.
    ///
    /// If not sure what to use, use `MasterBlueprintServiceManagerRevision::default()` which will use
    /// the latest revision available.
    pub master_manager_revision: MasterBlueprintServiceManagerRevision,
    /// The job definitions that are available in this service.
    pub jobs: Vec<JobDefinition<'a>>,
    /// The parameters that are required for the service registration.
    pub registration_params: Vec<FieldType>,
    /// The parameters that are required for the service request.
    pub request_params: Vec<FieldType>,
    /// The binary sources for the blueprint.
    pub sources: Vec<BlueprintSource<'a>>,
    /// Recommended resources for running this blueprint
    pub recommended_resources: Vec<BlueprintResourceRequirement>,
}

impl TryFrom<ServiceBlueprint<'_>> for SubxtServiceBlueprint {
    type Error = super::error::Error;

    fn try_from(value: ServiceBlueprint<'_>) -> Result<Self, Self::Error> {
        let ServiceBlueprint {
            metadata,
            manager,
            master_manager_revision,
            jobs,
            registration_params,
            request_params,
            sources,
            recommended_resources,
        } = value;

        Ok(SubxtServiceBlueprint {
            metadata: metadata.into(),
            manager: manager.try_into()?,
            master_manager_revision,
            sources: BoundedVec(sources.into_iter().map(Into::into).collect()),
            jobs: BoundedVec(jobs.into_iter().map(Into::into).collect()),
            registration_params: BoundedVec(registration_params),
            request_params: BoundedVec(request_params),
            // TODO: Not supported in the macro yet
            supported_membership_models: BoundedVec(vec![MembershipModelType::Fixed]),
            recommended_resources: BoundedVec(
                recommended_resources.into_iter().map(Into::into).collect(),
            ),
        })
    }
}
