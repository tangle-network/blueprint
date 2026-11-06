//! The SDK owns the JSON contract consumed by the maintained CLI.

use super::*;
use crate::{CloudProvider, CloudProvisioner, Error, ResourceSpec};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct SchemaVersion(
    #[serde(deserialize_with = "deserialize_version")]
    #[schemars(range(min = SCHEMA_VERSION, max = SCHEMA_VERSION))]
    u32,
);

fn deserialize_version<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> core::result::Result<u32, D::Error> {
    let value = u32::deserialize(deserializer)?;
    if value != SCHEMA_VERSION {
        return Err(serde::de::Error::custom("unsupported schema version"));
    }
    Ok(value)
}

fn version() -> SchemaVersion {
    SchemaVersion(SCHEMA_VERSION)
}

#[derive(Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum BoundaryRequest {
    Catalog {
        #[serde(default = "version")]
        schema_version: SchemaVersion,
    },
    Quote {
        schema_version: SchemaVersion,
        credentials: Vec<ProviderCredentials>,
        spec: ResourceSpec,
    },
    Create {
        schema_version: SchemaVersion,
        credentials: Vec<ProviderCredentials>,
        spec: ResourceSpec,
        ownership: ManagedLeaseIdentity,
        candidate: String,
    },
    Status {
        schema_version: SchemaVersion,
        credentials: Vec<ProviderCredentials>,
        provider: CloudProvider,
        ownership: ManagedLeaseIdentity,
        provider_id: String,
    },
    List {
        schema_version: SchemaVersion,
        credentials: Vec<ProviderCredentials>,
        ownership: Namespace,
    },
    Destroy {
        schema_version: SchemaVersion,
        credentials: Vec<ProviderCredentials>,
        provider: CloudProvider,
        ownership: ManagedLeaseIdentity,
        #[serde(default)]
        provider_id: Option<String>,
    },
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Namespace {
    pub namespace: String,
}

#[derive(Serialize, JsonSchema)]
pub struct ProviderDescriptor {
    pub provider: CloudProvider,
    pub credential_schema: Value,
}

#[derive(Serialize, JsonSchema)]
pub struct CatalogData {
    pub sdk_version: String,
    pub catalog_sha256: String,
    pub schema_sha256: String,
    pub gpus: Vec<blueprint_core::gpu::GpuClass>,
    pub providers: Vec<ProviderDescriptor>,
    pub boundary_schema: Value,
}

#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub enum BoundaryData {
    Catalog(CatalogData),
    Quotes(QuoteSet),
    Resource(ManagedResource),
    Resources { resources: Vec<ManagedResource> },
    Destroyed { destroyed: bool },
}

#[derive(Serialize, JsonSchema)]
pub struct BoundaryError {
    pub code: ProviderErrorCode,
    pub message: &'static str,
    pub retryable: bool,
}

impl From<Error> for BoundaryError {
    fn from(error: Error) -> Self {
        let code = match error {
            Error::Provider(code) => code,
            Error::ProviderNotConfigured(_) => ProviderErrorCode::ProviderNotConfigured,
            Error::ConfigurationError(_) | Error::SerializationError(_) => {
                ProviderErrorCode::InvalidRequest
            }
            _ => ProviderErrorCode::ProviderError,
        };
        Self {
            code,
            message: "Cloud operation failed",
            retryable: matches!(
                code,
                ProviderErrorCode::NoStock
                    | ProviderErrorCode::RateLimited
                    | ProviderErrorCode::ProviderError
            ),
        }
    }
}

#[derive(Serialize, JsonSchema)]
#[serde(untagged)]
pub enum BoundaryResponse {
    Success {
        schema_version: SchemaVersion,
        ok: bool,
        data: BoundaryData,
    },
    Failure {
        schema_version: SchemaVersion,
        ok: bool,
        error: BoundaryError,
    },
}

/// Both sides are derived from their actual Rust request and result types.
#[derive(JsonSchema)]
pub struct BoundaryContract {
    pub request: BoundaryRequest,
    pub response: BoundaryResponse,
}

fn schema<T: JsonSchema>() -> Value {
    let generator = schemars::generate::SchemaSettings::default()
        .for_deserialize()
        .into_generator();
    serde_json::to_value(generator.into_root_schema_for::<T>())
        .expect("derived JSON Schema is serializable")
}

fn digest(bytes: &[u8]) -> String {
    hex::encode(aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA256, bytes))
}

fn catalog_data() -> crate::Result<BoundaryData> {
    let credentials = schema::<ProviderCredentials>();
    let alternatives = credentials["oneOf"]
        .as_array()
        .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
    let providers = alternatives
        .iter()
        .map(|variant| {
            let provider =
                serde_json::from_value(variant["properties"]["provider"]["const"].clone())
                    .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
            Ok(ProviderDescriptor {
                provider,
                credential_schema: variant.clone(),
            })
        })
        .collect::<crate::Result<Vec<_>>>()?;
    let boundary_schema = schema::<BoundaryContract>();
    Ok(BoundaryData::Catalog(CatalogData {
        sdk_version: crate::VERSION.into(),
        catalog_sha256: digest(blueprint_core::gpu::CATALOG_JSON.as_bytes()),
        schema_sha256: digest(&serde_json::to_vec(&boundary_schema).expect("JSON schema")),
        gpus: blueprint_core::gpu::catalog(),
        providers,
        boundary_schema,
    }))
}

impl BoundaryResponse {
    pub fn invalid() -> Self {
        Self::failure(Error::Provider(ProviderErrorCode::InvalidRequest))
    }

    fn failure(error: Error) -> Self {
        Self::Failure {
            schema_version: version(),
            ok: false,
            error: error.into(),
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }
}

/// Credentials enter only this call; errors never serialize provider bodies.
pub async fn execute_boundary(request: BoundaryRequest) -> BoundaryResponse {
    match run(request).await {
        Ok(data) => BoundaryResponse::Success {
            schema_version: version(),
            ok: true,
            data,
        },
        Err(error) => BoundaryResponse::failure(error),
    }
}

async fn run(request: BoundaryRequest) -> crate::Result<BoundaryData> {
    match request {
        BoundaryRequest::Catalog { .. } => catalog_data(),
        BoundaryRequest::Quote {
            schema_version: _,
            credentials,
            spec,
        } => {
            if spec.ownership.is_some() {
                return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
            }
            Ok(BoundaryData::Quotes(
                CloudProvisioner::from_credentials(credentials)?
                    .quotes(&spec)
                    .await?,
            ))
        }
        BoundaryRequest::Create {
            schema_version: _,
            credentials,
            mut spec,
            ownership,
            candidate,
        } => {
            if spec.ownership.is_some() {
                return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
            }
            spec.ownership = Some(ownership);
            Ok(BoundaryData::Resource(
                CloudProvisioner::from_credentials(credentials)?
                    .create_managed(&spec, &candidate)
                    .await?,
            ))
        }
        BoundaryRequest::Status {
            schema_version: _,
            credentials,
            provider,
            ownership,
            provider_id,
        } => {
            let resource = CloudProvisioner::from_credentials(credentials)?
                .status_managed(&provider, &ownership, &provider_id)
                .await?
                .ok_or(Error::Provider(ProviderErrorCode::NotFound))?;
            Ok(BoundaryData::Resource(resource))
        }
        BoundaryRequest::List {
            schema_version: _,
            credentials,
            ownership,
        } => Ok(BoundaryData::Resources {
            resources: CloudProvisioner::from_credentials(credentials)?
                .list_managed(&ownership.namespace)
                .await?,
        }),
        BoundaryRequest::Destroy {
            schema_version: _,
            credentials,
            provider,
            ownership,
            provider_id,
        } => {
            CloudProvisioner::from_credentials(credentials)?
                .destroy_managed(&provider, &ownership, provider_id.as_deref())
                .await?;
            Ok(BoundaryData::Destroyed { destroyed: true })
        }
    }
}

/// Decode the process payload without retaining the credential input buffer.
pub fn read_boundary_request(operation: &str, reader: impl Read) -> Option<BoundaryRequest> {
    const LIMIT: u64 = 1_048_576;
    let mut input = zeroize::Zeroizing::new(Vec::new());
    reader.take(LIMIT + 1).read_to_end(&mut input).ok()?;
    if input.len() as u64 > LIMIT {
        return None;
    }
    let mut value: Value = if input.is_empty() && operation == "catalog" {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&input).ok()?
    };
    let object = value.as_object_mut()?;
    if object
        .get("operation")
        .is_some_and(|value| value.as_str() != Some(operation))
    {
        return None;
    }
    object.insert("operation".into(), Value::String(operation.into()));
    serde_json::from_value(value).ok()
}
