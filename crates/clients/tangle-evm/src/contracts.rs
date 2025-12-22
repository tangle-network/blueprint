//! Re-exported TNT Core contract bindings.

pub use tnt_core_bindings::bindings::{
    r#i_blueprint_service_manager::IBlueprintServiceManager,
    r#i_multi_asset_delegation::IMultiAssetDelegation,
    r#i_operator_status_registry::IOperatorStatusRegistry, r#i_tangle::ITangle,
    r#i_tangle::Types as ITangleTypes, r#i_tangle_services::ITangleServices,
    r#i_tangle_services::Types as ITangleServicesTypes,
    r#multi_asset_delegation::MultiAssetDelegation,
};
