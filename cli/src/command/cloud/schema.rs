//! Versioned process types. Domain types remain owned by core and remote-providers.
use blueprint_core::{gpu::GpuClass, resources::ManagedLeaseIdentity};
use blueprint_remote_providers::{
    CloudProvider, InstanceStatus, QuoteSet, ResourceSpec,
    managed::{ManagedResourceKind, ProviderCredentials, ProviderDescriptor, ProviderErrorCode},
};
use schemars::{JsonSchema, generate::SchemaSettings};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const SCHEMA_VERSION: u32 = 2;
/// Canonical input AND output ceiling, exported by catalog for process consumers.
pub const MAX_JSON_BYTES: usize = 1024 * 1024;

pub(super) trait VersionedRequest {
    fn version(&self) -> u32;
}

macro_rules! request {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Deserialize, JsonSchema)]
        #[serde(deny_unknown_fields)]
        pub(super) struct $name {
            #[schemars(extend("const" = SCHEMA_VERSION))]
            pub schema_version: u32,
            pub credentials: Vec<ProviderCredentials>,
            $(pub $field: $ty),*
        }
        impl VersionedRequest for $name { fn version(&self) -> u32 { self.schema_version } }
    };
}
request!(QuoteRequest { spec: ResourceSpec });
request!(CreateRequest {
    state_dir: PathBuf,
    spec: ResourceSpec,
    candidate: String
});
request!(RecoverRequest {
    state_dir: PathBuf,
    spec: ResourceSpec
});
request!(StateRequest { state_dir: PathBuf });
request!(StatusRequest {
    state_dir: PathBuf,
    provider: CloudProvider,
    identity: ManagedLeaseIdentity,
    provider_id: String
});
request!(ListRequest {
    state_dir: PathBuf,
    namespace: String
});
request!(DestroyRequest { state_dir: PathBuf, provider: CloudProvider, identity: ManagedLeaseIdentity, provider_id: Option<String> });

#[derive(Serialize, JsonSchema)]
pub(super) struct Failure {
    pub code: ProviderErrorCode,
    pub message: &'static str,
    /// Reissuing a call never authorizes a new identity or a second rental.
    pub retryable: bool,
}

#[derive(Serialize, JsonSchema)]
pub(super) struct Success<T> {
    #[schemars(extend("const" = SCHEMA_VERSION))]
    pub schema_version: u32,
    #[schemars(extend("const" = true))]
    pub ok: bool,
    pub data: T,
}

#[derive(Serialize, JsonSchema)]
pub(super) struct Rejection {
    #[schemars(extend("const" = SCHEMA_VERSION))]
    pub schema_version: u32,
    #[schemars(extend("const" = false))]
    pub ok: bool,
    pub error: Failure,
}

/// Deliberate public projection: no native instance metadata, env or private access.
#[derive(Serialize, JsonSchema)]
pub(super) struct Resource {
    pub provider: CloudProvider,
    pub provider_id: String,
    pub instance_type: String,
    pub region: String,
    pub identity: ManagedLeaseIdentity,
    pub kind: ManagedResourceKind,
    pub status: InstanceStatus,
    pub preempted: bool,
    pub gpu_class: Option<String>,
    pub gpu_count: Option<u32>,
    pub gpu_vram_gb: Option<u32>,
    pub hourly_cost_usd: Option<f64>,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub worker_url: Option<String>,
    pub endpoint_url: Option<String>,
    pub ssh_endpoint: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub(super) struct Resources {
    pub resources: Vec<Resource>,
}
#[derive(Serialize, JsonSchema)]
pub(super) struct Recovery {
    pub resource: Option<Resource>,
}
#[derive(Serialize, JsonSchema)]
pub(super) struct Destroyed {
    pub destroyed: bool,
}
#[derive(Serialize, JsonSchema)]
pub(super) struct Initialized {
    pub initialized: bool,
}

#[derive(Serialize, JsonSchema)]
pub(super) struct Catalog {
    pub gpus: Vec<GpuClass>,
    pub providers: Vec<ProviderDescriptor>,
    pub max_json_bytes: usize,
    pub boundary_schema: serde_json::Value,
    pub schema_sha256: String,
}

fn request_schema<T: JsonSchema>() -> serde_json::Value {
    serde_json::to_value(
        SchemaSettings::draft2020_12()
            .for_deserialize()
            .into_generator()
            .into_root_schema_for::<T>(),
    )
    .expect("schema has a JSON representation")
}
fn response_schema<T: JsonSchema>() -> serde_json::Value {
    serde_json::to_value(
        SchemaSettings::draft2020_12()
            .for_serialize()
            .into_generator()
            .into_root_schema_for::<Success<T>>(),
    )
    .expect("schema has a JSON representation")
}

pub(super) fn boundary_schema() -> serde_json::Value {
    serde_json::json!({
        "schema_version": SCHEMA_VERSION,
        "max_json_bytes": MAX_JSON_BYTES,
        "requests": {
            "quote": request_schema::<QuoteRequest>(), "create": request_schema::<CreateRequest>(),
            "recover": request_schema::<RecoverRequest>(), "state-init": request_schema::<StateRequest>(),
            "status": request_schema::<StatusRequest>(), "list": request_schema::<ListRequest>(),
            "destroy": request_schema::<DestroyRequest>()
        },
        "responses": {
            "quote": response_schema::<QuoteSet>(), "create": response_schema::<Resource>(),
            "recover": response_schema::<Recovery>(), "state-init": response_schema::<Initialized>(),
            "status": response_schema::<Resource>(), "list": response_schema::<Resources>(),
            "destroy": response_schema::<Destroyed>(),
            "catalog": response_schema::<Catalog>(),
            "error": serde_json::to_value(SchemaSettings::draft2020_12().for_serialize().into_generator().into_root_schema_for::<Rejection>()).expect("schema has a JSON representation")
        }
    })
}
