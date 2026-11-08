//! Thin, durable process boundary. The SDK owns all provider and rental decisions.
use super::schema::*;
use blueprint_remote_providers::{
    CloudProvisioner, Error, ManagedResource, ResourceSpec,
    managed::{ManagedResourceKind, ProviderCredentials, ProviderErrorCode},
};
use serde::{
    Deserialize,
    de::{self, DeserializeOwned, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fmt,
    io::{Read, Write},
    path::{Component, Path},
    time::Duration,
};
use zeroize::Zeroizing;

#[derive(Clone, Copy)]
pub(super) enum Operation {
    Catalog,
    Quote,
    Create,
    Recover,
    StateInit,
    Status,
    List,
    Destroy,
}
type Result<T> = std::result::Result<T, ProviderErrorCode>;

fn code(error: Error) -> ProviderErrorCode {
    match error {
        Error::Provider(code) => code,
        Error::ProviderNotConfigured(_) => ProviderErrorCode::ProviderNotConfigured,
        Error::ConfigurationError(_) | Error::SerializationError(_) => {
            ProviderErrorCode::InvalidRequest
        }
        _ => ProviderErrorCode::ProviderError,
    }
}

fn failure(code: ProviderErrorCode) -> Failure {
    use ProviderErrorCode as E;
    Failure {
        code,
        message: match code {
            E::InvalidRequest => "Invalid cloud request",
            E::AuthRejected => "Provider rejected its credential",
            E::ProviderNotConfigured => "Required provider account is unavailable",
            E::NoStock => "No matching provider stock",
            E::SkuUnknown => "Provider SKU is unknown",
            E::Unsupported => "Cloud request is outside the supported boundary",
            E::StillPresent => "Provider absence is not confirmed",
            E::Preempted => "Provider preempted the resource",
            E::Quota => "Provider quota is exhausted",
            E::RateLimited => "Provider rate limited the operation",
            E::NotFound => "Managed resource was not found",
            E::Conflict => "Cloud request conflicts with its original authority",
            E::Indeterminate => "Provider assignment is indeterminate",
            E::ProviderError => "Provider operation failed",
        },
        retryable: matches!(code, E::RateLimited | E::StillPresent | E::Indeterminate),
    }
}

/// Retains serde_json's normal recursion bound while rejecting duplicate object keys.
struct Unique;
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: de::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        struct UniqueVisitor;
        impl<'de> Visitor<'de> for UniqueVisitor {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("one bounded JSON document")
            }
            fn visit_bool<E: de::Error>(self, _: bool) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_i64<E: de::Error>(self, _: i64) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_u64<E: de::Error>(self, _: u64) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_str<E: de::Error>(self, _: &str) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_string<E: de::Error>(self, _: String) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique)
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> std::result::Result<Unique, A::Error> {
                while seq.next_element::<Unique>()?.is_some() {}
                Ok(Unique)
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut keys = std::collections::BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    if !keys.insert(key) {
                        return Err(de::Error::custom("duplicate field"));
                    }
                    let _: Unique = map.next_value()?;
                }
                Ok(Unique)
            }
        }
        deserializer.deserialize_any(UniqueVisitor)
    }
}

fn parse<T: DeserializeOwned + VersionedRequest>(input: &[u8]) -> Result<T> {
    let mut deserializer = serde_json::Deserializer::from_slice(input);
    Unique::deserialize(&mut deserializer).map_err(|_| ProviderErrorCode::InvalidRequest)?;
    deserializer
        .end()
        .map_err(|_| ProviderErrorCode::InvalidRequest)?;
    // Decode the canonical type directly: no Value round-trip loses input-only env.
    let request: T =
        serde_json::from_slice(input).map_err(|_| ProviderErrorCode::InvalidRequest)?;
    if request.version() != SCHEMA_VERSION {
        return Err(ProviderErrorCode::InvalidRequest);
    }
    Ok(request)
}

fn configured(credentials: Vec<ProviderCredentials>) -> Result<CloudProvisioner> {
    CloudProvisioner::from_credentials(credentials).map_err(code)
}

fn spec(spec: &ResourceSpec) -> Result<()> {
    spec.validate()
        .map_err(|_| ProviderErrorCode::InvalidRequest)?;
    let qos = &spec.qos;
    if !qos.priority.is_finite()
        || !qos.sla_target.is_finite()
        || !qos.reliability_multiplier.is_finite()
    {
        return Err(ProviderErrorCode::InvalidRequest);
    }
    if qos.priority != 0.0 || qos.sla_target != 0.0 || qos.reliability_multiplier != 0.0 {
        return Err(ProviderErrorCode::Unsupported);
    }
    if spec
        .env
        .keys()
        .any(|key| key.eq_ignore_ascii_case("TANGLE_GPU_WORKER_TOKEN"))
    {
        return Err(ProviderErrorCode::InvalidRequest);
    }
    Ok(())
}

fn rental_spec(request: &ResourceSpec) -> Result<Duration> {
    spec(request)?;
    if request.ownership.is_none()
        || request
            .ssh_public_key
            .as_ref()
            .is_none_or(|key| key.trim().is_empty())
    {
        return Err(ProviderErrorCode::InvalidRequest);
    }
    request
        .lease_duration_secs
        .filter(|seconds| *seconds > 0)
        .map(Duration::from_secs)
        .ok_or(ProviderErrorCode::InvalidRequest)
}

fn private_root(root: &Path) -> Result<()> {
    if !root.is_absolute()
        || root
            .components()
            .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(ProviderErrorCode::Conflict);
    }
    let mut path = std::path::PathBuf::new();
    for part in root.components() {
        path.push(part);
        if std::fs::symlink_metadata(&path)
            .map_err(|_| ProviderErrorCode::Conflict)?
            .file_type()
            .is_symlink()
        {
            return Err(ProviderErrorCode::Conflict);
        }
    }
    let metadata = std::fs::symlink_metadata(root).map_err(|_| ProviderErrorCode::Conflict)?;
    if !metadata.is_dir() {
        return Err(ProviderErrorCode::Conflict);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(ProviderErrorCode::Conflict);
        }
    }
    Ok(())
}

fn resume(credentials: Vec<ProviderCredentials>, root: &Path) -> Result<CloudProvisioner> {
    private_root(root)?;
    configured(credentials)?
        .with_lease_state_dir(root)
        .map_err(code)
}

fn public(resource: ManagedResource) -> Result<Resource> {
    if resource.kind == ManagedResourceKind::Instance
        && resource.is_present()
        && resource.expires_at.is_none()
    {
        return Err(ProviderErrorCode::Indeterminate);
    }
    Ok(Resource {
        provider: resource.instance.provider,
        provider_id: resource.instance.id,
        instance_type: resource.instance.instance_type,
        region: resource.instance.region,
        identity: blueprint_core::resources::ManagedLeaseIdentity {
            namespace: resource.namespace,
            lease_id: resource.lease_id,
        },
        kind: resource.kind,
        status: resource.instance.status,
        preempted: resource.preempted,
        gpu_class: resource.gpu_class,
        gpu_count: resource.gpu_count,
        gpu_vram_gb: resource.gpu_vram_gb,
        hourly_cost_usd: resource.hourly_cost_usd,
        created_at: resource.created_at.map(|time| time.to_rfc3339()),
        expires_at: resource.expires_at.map(|time| time.to_rfc3339()),
        worker_url: resource.worker_url,
        endpoint_url: resource.endpoint_url,
        ssh_endpoint: resource.ssh_endpoint,
    })
}

fn value(value: impl serde::Serialize) -> Result<Value> {
    serde_json::to_value(value).map_err(|_| ProviderErrorCode::ProviderError)
}

async fn run(operation: Operation, input: &[u8]) -> Result<Value> {
    match operation {
        Operation::Catalog => {
            let schema = boundary_schema();
            let bytes =
                serde_json::to_vec(&schema).map_err(|_| ProviderErrorCode::ProviderError)?;
            value(Catalog {
                gpus: blueprint_core::gpu::catalog(),
                providers: blueprint_remote_providers::managed::provider_catalog().map_err(code)?,
                max_json_bytes: MAX_JSON_BYTES,
                schema_sha256: format!("{:x}", Sha256::digest(bytes)),
                boundary_schema: schema,
            })
        }
        Operation::Quote => {
            let request: QuoteRequest = parse(input)?;
            spec(&request.spec)?;
            value(
                configured(request.credentials)?
                    .quotes(&request.spec)
                    .await
                    .map_err(code)?,
            )
        }
        Operation::Create => {
            let request: CreateRequest = parse(input)?;
            rental_spec(&request.spec)?;
            let provisioner = resume(request.credentials, &request.state_dir)?;
            value(public(
                provisioner
                    .create_managed(&request.spec, &request.candidate)
                    .await
                    .map_err(code)?,
            )?)
        }
        Operation::Recover => {
            let request: RecoverRequest = parse(input)?;
            let ttl = rental_spec(&request.spec)?;
            let provisioner = resume(request.credentials, &request.state_dir)?;
            value(Recovery {
                resource: provisioner
                    .recover_lease(&request.spec, ttl)
                    .await
                    .map_err(code)?
                    .map(|lease| public(lease.resource))
                    .transpose()?,
            })
        }
        Operation::StateInit => {
            let request: StateRequest = parse(input)?;
            private_root(&request.state_dir)?;
            if std::fs::read_dir(&request.state_dir)
                .map_err(|_| ProviderErrorCode::Conflict)?
                .next()
                .is_some()
            {
                return Err(ProviderErrorCode::Conflict);
            }
            configured(request.credentials)?
                .initialize_lease_state_dir(&request.state_dir)
                .await
                .map_err(code)?;
            value(Initialized { initialized: true })
        }
        Operation::Status => {
            let request: StatusRequest = parse(input)?;
            let provisioner = resume(request.credentials, &request.state_dir)?;
            value(public(
                provisioner
                    .status_managed(&request.provider, &request.identity, &request.provider_id)
                    .await
                    .map_err(code)?
                    .ok_or(ProviderErrorCode::NotFound)?,
            )?)
        }
        Operation::List => {
            let request: ListRequest = parse(input)?;
            let provisioner = resume(request.credentials, &request.state_dir)?;
            let resources = provisioner
                .list_managed(&request.namespace)
                .await
                .map_err(code)?;
            value(Resources {
                resources: resources.into_iter().map(public).collect::<Result<_>>()?,
            })
        }
        Operation::Destroy => {
            let request: DestroyRequest = parse(input)?;
            let provisioner = resume(request.credentials, &request.state_dir)?;
            provisioner
                .destroy_managed(
                    &request.provider,
                    &request.identity,
                    request.provider_id.as_deref(),
                )
                .await
                .map_err(code)?;
            value(Destroyed { destroyed: true })
        }
    }
}

pub(super) async fn execute(operation: Operation) -> color_eyre::Result<()> {
    let mut input = Zeroizing::new(Vec::new());
    let outcome = if matches!(operation, Operation::Catalog) {
        run(operation, &[]).await
    } else if std::io::stdin()
        .take(MAX_JSON_BYTES as u64 + 1)
        .read_to_end(&mut input)
        .is_err()
        || input.len() > MAX_JSON_BYTES
    {
        Err(ProviderErrorCode::InvalidRequest)
    } else {
        run(operation, &input).await
    };
    let (response, mut failed) = match outcome {
        Ok(data) => (
            value(Success {
                schema_version: SCHEMA_VERSION,
                ok: true,
                data,
            })
            .expect("public JSON"),
            false,
        ),
        Err(error) => (
            value(Rejection {
                schema_version: SCHEMA_VERSION,
                ok: false,
                error: failure(error),
            })
            .expect("public JSON"),
            true,
        ),
    };
    let mut output = serde_json::to_vec(&response)?;
    if output.len() + 1 > MAX_JSON_BYTES {
        output = serde_json::to_vec(&Rejection {
            schema_version: SCHEMA_VERSION,
            ok: false,
            error: failure(ProviderErrorCode::InvalidRequest),
        })?;
        failed = true;
    }
    output.push(b'\n');
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&output)?;
    stdout.flush()?;
    drop(stdout);
    drop(input);
    if failed {
        std::process::exit(2);
    }
    Ok(())
}
