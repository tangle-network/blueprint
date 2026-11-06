//! Versioned process boundary for external GPU lease orchestrators.

use blueprint_core::resources::ManagedLeaseIdentity;
use blueprint_remote_providers::{
    CloudProvider, CloudProvisioner, Error, InstanceStatus, ManagedResource, ResourceSpec,
    managed::ProviderCredentials,
};
use color_eyre::Result;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, io::Read};
use zeroize::Zeroizing;

#[derive(Clone, Copy)]
pub(super) enum Operation {
    Catalog,
    Quote,
    Create,
    Status,
    Destroy,
    List,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Ownership {
    namespace: String,
    #[serde(default)]
    lease_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema_version: u32,
    #[serde(default)]
    provider: Option<CloudProvider>,
    #[serde(default)]
    credentials: HashMap<CloudProvider, ProviderCredentials>,
    #[serde(default)]
    spec: Option<ResourceSpec>,
    #[serde(default)]
    provider_id: Option<String>,
    #[serde(default)]
    ownership: Option<Ownership>,
}

struct Failure {
    code: &'static str,
    message: &'static str,
    retryable: bool,
}

impl Failure {
    fn invalid() -> Self {
        Self {
            code: "INVALID_REQUEST",
            message: "Invalid cloud request",
            retryable: false,
        }
    }
}

impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        // Provider errors can contain reflected credentials, URLs or environment
        // values. Only this fixed projection crosses the process boundary.
        match error {
            Error::AuthRejected => Self {
                code: "AUTH_REJECTED",
                message: "Provider rejected its credential",
                retryable: false,
            },
            Error::ConfigurationError(_) | Error::SerializationError(_) => Self::invalid(),
            Error::ProviderNotConfigured(_) => Self {
                code: "PROVIDER_NOT_CONFIGURED",
                message: "Provider credentials are not configured",
                retryable: false,
            },
            _ => Self {
                code: "PROVIDER_ERROR",
                message: "Provider operation failed",
                retryable: true,
            },
        }
    }
}

pub(super) async fn execute(operation: Operation) -> Result<()> {
    let outcome = run(operation).await;
    let (response, code) = match outcome {
        Ok(data) => (json!({"schema_version":1,"ok":true,"data":data}), 0),
        Err(error) => (
            json!({"schema_version":1,"ok":false,"error":{
                "code":error.code,"message":error.message,"retryable":error.retryable
            }}),
            2,
        ),
    };
    println!("{response}");
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}

async fn run(operation: Operation) -> std::result::Result<Value, Failure> {
    if matches!(operation, Operation::Catalog) {
        let mut catalog: Value = serde_json::from_str(blueprint_core::gpu::CATALOG_JSON)
            .map_err(|_| Failure::invalid())?;
        catalog["providers"] =
            serde_json::to_value(blueprint_remote_providers::managed::provider_catalog())
                .map_err(|_| Failure::invalid())?;
        return Ok(catalog);
    }
    let mut input = Zeroizing::new(String::new());
    std::io::stdin()
        .take(1_048_577)
        .read_to_string(&mut input)
        .map_err(|_| Failure::invalid())?;
    if input.len() > 1_048_576 {
        return Err(Failure::invalid());
    }
    let mut request: Request = serde_json::from_str(&input).map_err(|_| Failure::invalid())?;
    if request.schema_version != 1
        || request
            .spec
            .as_ref()
            .is_some_and(|spec| spec.ownership.is_some())
    {
        return Err(Failure::invalid());
    }
    if let Some(provider) = &request.provider {
        request
            .credentials
            .retain(|configured, _| configured == provider);
    }
    let provisioner = CloudProvisioner::from_credentials(request.credentials)?;
    match operation {
        Operation::Quote | Operation::Create => {
            let mut spec = request.spec.ok_or_else(Failure::invalid)?;
            if let Some(provider) = request.provider {
                if spec
                    .provider
                    .as_ref()
                    .is_some_and(|constraint| constraint != &provider)
                {
                    return Err(Failure::invalid());
                }
                spec.provider = Some(provider);
            }
            if matches!(operation, Operation::Quote) {
                let quote = provisioner
                    .quotes(&spec)
                    .await?
                    .into_iter()
                    .next()
                    .ok_or(Failure {
                        code: "NO_PRICE",
                        message: "No configured provider has a known available price",
                        retryable: false,
                    })?;
                return Ok(json!({"provider":quote.provider.catalog_key(),
                    "instance_type":quote.selection.instance_type,"region":quote.region,
                    "pricing_source":quote.pricing_source,"hourly_cost_usd":quote.selection.estimated_hourly_cost}));
            }
            spec.ownership = Some(identity(request.ownership)?);
            Ok(resource(provisioner.create_managed(&spec).await?))
        }
        Operation::Status => {
            let provider = request.provider.ok_or_else(Failure::invalid)?;
            let identity = identity(request.ownership)?;
            let provider_id = request
                .provider_id
                .filter(|id| !id.is_empty())
                .ok_or_else(Failure::invalid)?;
            let result = provisioner
                .status_managed(&provider, &identity, &provider_id)
                .await?
                .ok_or(Failure {
                    code: "NOT_FOUND",
                    message: "Managed resource not found",
                    retryable: false,
                })?;
            Ok(resource(result))
        }
        Operation::Destroy => {
            let provider = request.provider.ok_or_else(Failure::invalid)?;
            provisioner
                .destroy_managed(&provider, &identity(request.ownership)?)
                .await?;
            Ok(json!({"destroyed":true}))
        }
        Operation::List => {
            let ownership = request.ownership.ok_or_else(Failure::invalid)?;
            if ownership.lease_id.is_some() {
                return Err(Failure::invalid());
            }
            let resources = provisioner.list_managed(&ownership.namespace).await?;
            Ok(
                json!({"resources":resources.into_iter().filter(|resource| request.provider.as_ref().is_none_or(|provider| provider == &resource.instance.provider)).map(resource).collect::<Vec<_>>()}),
            )
        }
        Operation::Catalog => unreachable!(),
    }
}

fn identity(ownership: Option<Ownership>) -> std::result::Result<ManagedLeaseIdentity, Failure> {
    let ownership = ownership.ok_or_else(Failure::invalid)?;
    let identity = ManagedLeaseIdentity {
        namespace: ownership.namespace,
        lease_id: ownership.lease_id.ok_or_else(Failure::invalid)?,
    };
    identity.validate().map_err(|_| Failure::invalid())?;
    Ok(identity)
}

fn resource(resource: ManagedResource) -> Value {
    let status = match resource.instance.status {
        InstanceStatus::Running => "ready",
        InstanceStatus::Starting => "provisioning",
        _ => "failed",
    };
    json!({"provider":resource.instance.provider.catalog_key(),
        "provider_id":resource.instance.id,"instance_type":resource.instance.instance_type,
        "region":resource.instance.region,"status":status,"started_at":resource.created_at,
        "worker_url":resource.endpoint,"endpoint_url":resource.endpoint,
        "hourly_cost_usd":resource.hourly_cost_usd,"lease_id":resource.lease_id,
        "ownership":{"namespace":resource.namespace,"lease_id":resource.lease_id},
        "metadata":{"kind":resource.kind,"tls_mode":resource.tls_mode,"expires_at":resource.expires_at,"expiry_source":resource.expiry_source,"public_ip":resource.instance.public_ip,
            "private_ip":resource.instance.private_ip}})
}
