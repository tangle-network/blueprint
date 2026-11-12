//! AWS EC2 GPU leases: catalog SKUs, live Pricing API rates and one tagged instance.
use crate::infra::traits::CloudProviderAdapter;
use crate::managed::{
    ManagedLeaseIdentity, ManagedMetadata, ManagedProviderAdapter, ManagedResource,
    ManagedResourceKind, NativeOffer, ProviderErrorCode, bootstrap,
};
use crate::pricing::{PricingError, PricingFetcher};
use crate::providers::common::gpu_adapter::poll_until;
use crate::security::{ApiAuthentication, canonical_public_key};
use crate::{CloudProvider, Error, InstanceStatus, ProvisionedInstance, ResourceSpec, Result};
use async_trait::async_trait;
use aws_sdk_ec2::config::{Credentials, SharedCredentialsProvider};
use aws_sdk_ec2::error::{ProvideErrorMetadata, SdkError};
use aws_sdk_ec2::types::{
    BlockDeviceMapping, EbsBlockDevice, Filter, Instance, InstanceNetworkInterfaceSpecification,
    InstanceStateName, InstanceType, LocationType, ResourceType, ShutdownBehavior, Tag,
    TagSpecification, VolumeType,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use blueprint_core::gpu::GpuClass;
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};

const PRICING_SOURCE: &str = "https://api.pricing.us-east-1.amazonaws.com";
/// Amazon-owned Turing-or-newer image with Docker and the NVIDIA container toolkit.
const IMAGE_NAME: &str = "Deep Learning Base OSS Nvidia Driver GPU AMI (Ubuntu 22.04) ????????";
/// `shutting-down` already ends billing and cannot be undone, so it counts as absent.
const PRESENT_STATES: [&str; 4] = ["pending", "running", "stopping", "stopped"];

/// One account in one region. Network and image fields are optional overrides.
pub struct AwsAccount {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub region: String,
    pub subnet_id: Option<String>,
    pub security_group_ids: Vec<String>,
    pub ami_id: Option<String>,
}

pub struct AwsManagedAdapter {
    ec2: aws_sdk_ec2::Client,
    pricing: PricingFetcher,
    pricing_source: String,
    binding: String,
    region: String,
    subnet_id: Option<String>,
    security_group_ids: Vec<String>,
    ami_id: Option<String>,
}

fn filter(name: &str, values: &[&str]) -> Filter {
    values
        .iter()
        .fold(Filter::builder().name(name), |builder, value| {
            builder.values(*value)
        })
        .build()
}

fn failure<E: ProvideErrorMetadata>(error: &SdkError<E>) -> Error {
    use crate::managed::ProviderErrorCode as C;
    let SdkError::ServiceError(_) = error else {
        // A request that may have been sent has no proven outcome.
        return Error::Provider(if matches!(error, SdkError::ConstructionFailure(_)) {
            C::InvalidRequest
        } else {
            C::Indeterminate
        });
    };
    let server = error
        .raw_response()
        .is_some_and(|response| response.status().is_server_error());
    Error::Provider(match error.code().unwrap_or_default() {
        "InsufficientInstanceCapacity" | "InsufficientHostCapacity" | "Unsupported" => C::NoStock,
        "AuthFailure"
        | "UnauthorizedOperation"
        | "InvalidClientTokenId"
        | "ExpiredToken"
        | "SignatureDoesNotMatch"
        | "RequestExpired"
        | "OptInRequired"
        | "Blocked" => C::AuthRejected,
        "RequestLimitExceeded" | "Throttling" | "ThrottlingException" => C::RateLimited,
        "InstanceLimitExceeded" | "VcpuLimitExceeded" => C::Quota,
        "IdempotentParameterMismatch" => C::Conflict,
        "InvalidInstanceID.NotFound" | "InvalidInstanceID.Malformed" => C::NotFound,
        code if code.starts_with("Invalid") => C::InvalidRequest,
        _ if server => C::Indeterminate,
        _ => C::ProviderError,
    })
}

impl AwsManagedAdapter {
    /// Environment bindings shared with the generated credential descriptor.
    pub const ACCESS_KEY_ID_ENV: &'static str = "AWS_ACCESS_KEY_ID";
    pub const SECRET_ACCESS_KEY_ENV: &'static str = "AWS_SECRET_ACCESS_KEY";
    pub const SESSION_TOKEN_ENV: &'static str = "AWS_SESSION_TOKEN";
    pub const REGION_ENV: &'static str = "AWS_REGION";
    pub const SUBNET_ID_ENV: &'static str = "AWS_GPU_SUBNET_ID";
    pub const SECURITY_GROUP_IDS_ENV: &'static str = "AWS_GPU_SECURITY_GROUP_IDS";
    pub const AMI_ID_ENV: &'static str = "AWS_GPU_AMI_ID";

    /// `endpoint` replaces both the EC2 and Pricing endpoints (HTTPS, or loopback HTTP).
    pub fn with_endpoint(account: AwsAccount, endpoint: Option<&str>) -> Result<Self> {
        let invalid = || Error::Provider(ProviderErrorCode::InvalidRequest);
        let AwsAccount {
            access_key_id,
            secret_access_key,
            session_token,
            region,
            subnet_id,
            security_group_ids,
            ami_id,
        } = account;
        let blank = |value: &str| value.trim().is_empty() || value.chars().any(char::is_control);
        let native = |value: &str, prefix: &str| {
            value.len() <= 64
                && value.strip_prefix(prefix).is_some_and(|rest| {
                    !rest.is_empty() && rest.bytes().all(|byte| byte.is_ascii_alphanumeric())
                })
        };
        if blank(&access_key_id)
            || blank(&secret_access_key)
            || session_token.as_deref().is_some_and(blank)
            || region.is_empty()
            || !region
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
            || subnet_id
                .as_deref()
                .is_some_and(|id| !native(id, "subnet-"))
            || ami_id.as_deref().is_some_and(|id| !native(id, "ami-"))
            || security_group_ids.iter().any(|id| !native(id, "sg-"))
        {
            return Err(invalid());
        }
        let binding = crate::managed::authorization::credential_digest(
            &ApiAuthentication::Bearer {
                token: format!("{access_key_id}\n{secret_access_key}"),
            },
            (endpoint, &subnet_id, &security_group_ids, &ami_id),
        )?;
        let credentials = Credentials::new(
            access_key_id,
            secret_access_key,
            session_token,
            None,
            "blueprint-static",
        );
        let mut config = aws_config::SdkConfig::builder()
            .behavior_version(aws_config::BehaviorVersion::latest())
            .region(aws_config::Region::new(region.clone()))
            .credentials_provider(SharedCredentialsProvider::new(credentials));
        config.set_endpoint_url(endpoint.map(str::to_owned));
        let config = config.build();
        let pricing = PricingFetcher::new()
            .and_then(|fetcher| fetcher.with_aws_config(&config))
            .map_err(|_| invalid())?;
        Ok(Self {
            ec2: aws_sdk_ec2::Client::new(&config),
            pricing,
            pricing_source: endpoint.map_or_else(
                || PRICING_SOURCE.to_owned(),
                |url| url.trim_end_matches('/').to_owned(),
            ),
            binding,
            region,
            subnet_id,
            security_group_ids,
            ami_id,
        })
    }

    pub(crate) fn credential_binding(&self) -> Result<(String, Option<String>)> {
        Ok((self.binding.clone(), Some(self.region.clone())))
    }

    fn supported(spec: &ResourceSpec) -> Result<()> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.tee_required
            || spec.volume_gb.is_some()
            || spec.volume_id.is_some()
            || spec.source_network.is_some()
            || spec.manifest.is_some()
            || spec.storage_gb > 16_384.0
            || (spec.image.is_none() && (!spec.env.is_empty() || spec.command.is_some()))
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        if let Some(key) = &spec.ssh_public_key {
            canonical_public_key(key)?;
        }
        Ok(())
    }

    /// The exact catalog class and count for a SKU, or none when it is unknown or ambiguous.
    fn hardware(sku: &str) -> Option<(GpuClass, u32)> {
        let mut found = blueprint_core::gpu::catalog()
            .into_iter()
            .filter_map(|class| {
                let count = class
                    .provider_skus
                    .get("aws")?
                    .iter()
                    .find(|(_, value)| value.as_str() == sku)?
                    .0
                    .parse::<u32>()
                    .ok()?;
                let exact = class.sku("aws", count) == Some(sku);
                exact.then_some((class, count))
            });
        let first = found.next()?;
        found.next().is_none().then_some(first)
    }

    /// Image id, root device and its minimum size in GiB.
    async fn image(&self) -> Result<(String, String, i32)> {
        let unsupported = || Error::Provider(ProviderErrorCode::Unsupported);
        let request = match &self.ami_id {
            Some(id) => self.ec2.describe_images().image_ids(id),
            None => self
                .ec2
                .describe_images()
                .owners("amazon")
                .filters(filter("name", &[IMAGE_NAME]))
                .filters(filter("architecture", &["x86_64"])),
        };
        let response = request
            .filters(filter("state", &["available"]))
            .send()
            .await
            .map_err(|error| failure(&error))?;
        let image = response
            .images()
            .iter()
            .filter(|image| image.image_id().is_some())
            .max_by(|left, right| left.creation_date().cmp(&right.creation_date()))
            .ok_or_else(unsupported)?;
        let root = image.root_device_name().ok_or_else(unsupported)?;
        let size = image
            .block_device_mappings()
            .iter()
            .find(|mapping| mapping.device_name() == Some(root))
            .and_then(|mapping| mapping.ebs())
            .and_then(|ebs| ebs.volume_size())
            .ok_or_else(unsupported)?;
        let id = image.image_id().ok_or_else(unsupported)?;
        Ok((id.to_owned(), root.to_owned(), size))
    }

    /// Cloud-init installs the caller key, the optional container and an expiry poweroff.
    fn user_data(spec: &ResourceSpec, metadata: &ManagedMetadata) -> Result<String> {
        let invalid = || Error::Provider(ProviderErrorCode::InvalidRequest);
        let mut document: Value = match &spec.image {
            Some(image) => {
                let mut env = bootstrap::workload_environment(spec)?;
                env.extend(metadata.env());
                let init = bootstrap::gpu_container_cloud_init(
                    image,
                    &env,
                    &spec.ports,
                    &[],
                    spec.command.as_deref(),
                )?;
                serde_yaml::from_str(&init).map_err(|_| invalid())?
            }
            None => json!({}),
        };
        let object = document.as_object_mut().ok_or_else(invalid)?;
        if let Some(key) = &spec.ssh_public_key {
            let key = canonical_public_key(key)?;
            object.insert("ssh_authorized_keys".into(), json!([key]));
        }
        // With shutdown behavior `terminate`, the poweroff at expiry ends the rental natively.
        let timer = format!(
            "[Timer]\nOnCalendar={}\nPersistent=true\n\n[Install]\nWantedBy=timers.target\n",
            metadata.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        let files = object
            .entry("write_files")
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .ok_or_else(invalid)?;
        files.push(json!({"path":"/etc/systemd/system/tangle-expiry.timer","content":timer}));
        files.push(json!({"path":"/etc/systemd/system/tangle-expiry.service",
            "content":"[Service]\nType=oneshot\nExecStart=/usr/bin/systemctl poweroff\n"}));
        object
            .entry("runcmd")
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .ok_or_else(invalid)?
            .insert(
                0,
                json!(["systemctl", "enable", "--now", "tangle-expiry.timer"]),
            );
        let user_data = serde_yaml::to_string(&document)
            .map(|yaml| format!("#cloud-config\n{yaml}"))
            .map_err(|_| invalid())?;
        if user_data.len() > 16_384 {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        Ok(user_data)
    }

    async fn describe(&self, id: &str) -> Result<Option<Instance>> {
        match self.ec2.describe_instances().instance_ids(id).send().await {
            Ok(response) => Ok(response
                .reservations()
                .iter()
                .flat_map(|reservation| reservation.instances())
                .find(|instance| instance.instance_id() == Some(id))
                .cloned()),
            Err(error) => match failure(&error) {
                Error::Provider(ProviderErrorCode::NotFound) => Ok(None),
                error => Err(error),
            },
        }
    }

    fn instance(&self, raw: &Instance) -> Result<ProvisionedInstance> {
        Ok(ProvisionedInstance {
            id: raw
                .instance_id()
                .filter(|id| !id.is_empty())
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?
                .into(),
            provider: CloudProvider::AWS,
            instance_type: raw.instance_type().map_or("", InstanceType::as_str).into(),
            region: self.region.clone(),
            public_ip: raw.public_ip_address().map(str::to_owned),
            private_ip: raw.private_ip_address().map(str::to_owned),
            status: match raw.state().and_then(|state| state.name()) {
                Some(InstanceStateName::Pending) => InstanceStatus::Starting,
                Some(InstanceStateName::Running) => InstanceStatus::Running,
                Some(InstanceStateName::Stopping) => InstanceStatus::Stopping,
                Some(InstanceStateName::Stopped) => InstanceStatus::Stopped,
                Some(InstanceStateName::ShuttingDown | InstanceStateName::Terminated) => {
                    InstanceStatus::Terminated
                }
                _ => InstanceStatus::Unknown,
            },
        })
    }

    fn resource(&self, raw: &Instance) -> Result<Option<ManagedResource>> {
        let mut tags = BTreeMap::new();
        for tag in raw.tags() {
            let key = tag
                .key()
                .ok_or(Error::Provider(ProviderErrorCode::ProviderError))?;
            let value = tag.value().unwrap_or_default();
            if tags.insert(key.to_owned(), value.to_owned()).is_some() {
                return Err(Error::Provider(ProviderErrorCode::Conflict));
            }
        }
        let Some(metadata) = ManagedMetadata::from_tags(&tags)? else {
            return Ok(None);
        };
        let instance = self.instance(raw)?;
        let hardware = Self::hardware(&instance.instance_type);
        let mut resource = metadata.bind(instance, ManagedResourceKind::Instance);
        resource.gpu_vram_gb = hardware.as_ref().map(|(class, _)| class.vram_gb);
        resource.gpu_count = hardware.as_ref().map(|(_, count)| *count);
        resource.gpu_class = hardware.map(|(class, _)| class.id);
        resource.created_at = raw
            .launch_time()
            .and_then(|time| chrono::DateTime::from_timestamp(time.secs(), time.subsec_nanos()));
        resource.ssh_endpoint = resource
            .instance
            .public_ip
            .as_deref()
            .and_then(|ip| ip.parse::<std::net::Ipv4Addr>().ok())
            .map(|ip| format!("ssh://ubuntu@{ip}:22"));
        Ok(Some(resource))
    }

    /// Managed instances with their live node-hour rate; an unpriced SKU stays unpriced.
    async fn observe(&self, raw: &[Instance]) -> Result<Vec<ManagedResource>> {
        let mut prices: BTreeMap<String, Option<f64>> = BTreeMap::new();
        let mut resources = Vec::new();
        for instance in raw {
            let Some(mut resource) = self.resource(instance)? else {
                continue;
            };
            let sku = resource.instance.instance_type.clone();
            let cached = prices.get(&sku).copied();
            resource.hourly_cost_usd = match cached {
                Some(price) => price,
                None => {
                    let price = self
                        .pricing
                        .quote_instance(CloudProvider::AWS, &self.region, &sku)
                        .await
                        .ok()
                        .flatten()
                        .filter(|price| price.is_finite() && *price > 0.0);
                    prices.insert(sku, price);
                    price
                }
            };
            resources.push(resource);
        }
        Ok(resources)
    }
}

#[async_trait]
impl ManagedProviderAdapter for AwsManagedAdapter {
    async fn offers(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<NativeOffer>> {
        Self::supported(spec)?;
        if !region.is_empty() && region != self.region.as_str() {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let count = spec
            .gpu_count
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))?;
        // The default image's open kernel driver needs Turing or newer.
        let candidates: Vec<(GpuClass, String)> = blueprint_core::gpu::catalog()
            .into_iter()
            .filter_map(|class| {
                let sku = class.sku("aws", count)?.to_owned();
                let usable = spec.gpu_class.as_ref().is_none_or(|id| id == &class.id)
                    && spec
                        .gpu_vram_gb
                        .is_none_or(|minimum| class.vram_gb >= minimum)
                    && (self.ami_id.is_some()
                        || !matches!(class.generation.as_str(), "pascal" | "volta"));
                usable.then_some((class, sku))
            })
            .collect();
        if candidates.is_empty() {
            return Ok(Vec::new());
        }
        let names: Vec<&str> = candidates.iter().map(|(_, sku)| sku.as_str()).collect();
        let response = self
            .ec2
            .describe_instance_type_offerings()
            .location_type(LocationType::Region)
            .filters(filter("instance-type", &names))
            .max_results(1000)
            .send()
            .await
            .map_err(|error| failure(&error))?;
        let offered: HashSet<String> = response
            .instance_type_offerings()
            .iter()
            .filter_map(|offering| offering.instance_type())
            .map(|sku| sku.as_str().to_owned())
            .collect();
        if offered.is_empty() {
            return Ok(Vec::new());
        }
        let request = candidates
            .iter()
            .filter(|(_, sku)| offered.contains(sku))
            .fold(self.ec2.describe_instance_types(), |request, (_, sku)| {
                request.instance_types(InstanceType::from(sku.as_str()))
            });
        let types = request.send().await.map_err(|error| failure(&error))?;
        let mut offers = Vec::new();
        for info in types.instance_types() {
            let Some(sku) = info.instance_type().map(InstanceType::as_str) else {
                continue;
            };
            let Some((class, _)) = candidates.iter().find(|(_, name)| name.as_str() == sku) else {
                continue;
            };
            let gpus: i32 = info.gpu_info().map_or(0, |gpu| {
                gpu.gpus().iter().filter_map(|device| device.count()).sum()
            });
            let vcpus = info
                .v_cpu_info()
                .and_then(|cpu| cpu.default_v_cpus())
                .unwrap_or(0);
            let memory_mib = info
                .memory_info()
                .and_then(|memory| memory.size_in_mib())
                .unwrap_or(0);
            if u32::try_from(gpus).ok() != Some(count)
                || f64::from(vcpus) < f64::from(spec.cpu)
                || memory_mib as f64 * 1.048_576 / 1000.0 < f64::from(spec.memory_gb)
            {
                continue;
            }
            let price = self
                .pricing
                .quote_instance(CloudProvider::AWS, &self.region, sku)
                .await
                .map_err(|error| {
                    Error::Provider(if matches!(error, PricingError::AuthRejected) {
                        ProviderErrorCode::AuthRejected
                    } else {
                        ProviderErrorCode::ProviderError
                    })
                })?;
            let Some(price) = price.filter(|price| price.is_finite() && *price > 0.0) else {
                continue;
            };
            offers.push(NativeOffer {
                provider: CloudProvider::AWS,
                native_id: sku.into(),
                instance_type: sku.into(),
                region: self.region.clone(),
                gpu_class: class.id.clone(),
                gpu_count: count,
                hourly_cost_usd: price,
                spot: false,
                pricing_source: self.pricing_source.clone(),
                underlying: None,
            });
        }
        if !offers.is_empty() {
            self.image().await?;
        }
        Ok(offers)
    }

    async fn create(
        &self,
        spec: &ResourceSpec,
        offer: &NativeOffer,
        metadata: &ManagedMetadata,
    ) -> Result<ManagedResource> {
        Self::supported(spec)?;
        if offer.provider != CloudProvider::AWS
            || offer.spot
            || offer.region != self.region
            || Self::hardware(&offer.instance_type).is_none_or(|(class, count)| {
                class.id != offer.gpu_class || count != offer.gpu_count
            })
        {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        metadata.check_deadline()?;
        let (image, root, minimum) = self.image().await?;
        let user_data = Self::user_data(spec, metadata)?;
        let tags = metadata.tags().into_iter().fold(
            TagSpecification::builder()
                .resource_type(ResourceType::Instance)
                .tags(Tag::builder().key("Name").value(metadata.name()).build()),
            |tags, (key, value)| tags.tags(Tag::builder().key(key).value(value).build()),
        );
        let root = BlockDeviceMapping::builder()
            .device_name(root)
            .ebs(
                EbsBlockDevice::builder()
                    .volume_size((spec.storage_gb.ceil() as i32).max(minimum))
                    .volume_type(VolumeType::Gp3)
                    .delete_on_termination(true)
                    .build(),
            )
            .build();
        // One native request; the token makes an SDK retry return the same instance.
        let mut request = self
            .ec2
            .run_instances()
            .image_id(image)
            .instance_type(InstanceType::from(offer.instance_type.as_str()))
            .min_count(1)
            .max_count(1)
            .client_token(metadata.idempotency_key())
            .user_data(STANDARD.encode(user_data))
            .instance_initiated_shutdown_behavior(ShutdownBehavior::Terminate)
            .tag_specifications(tags.build())
            .block_device_mappings(root);
        request = match &self.subnet_id {
            Some(subnet) => request.network_interfaces(
                self.security_group_ids
                    .iter()
                    .fold(
                        InstanceNetworkInterfaceSpecification::builder()
                            .device_index(0)
                            .subnet_id(subnet)
                            .associate_public_ip_address(true)
                            .delete_on_termination(true),
                        |interface, group| interface.groups(group),
                    )
                    .build(),
            ),
            None => self
                .security_group_ids
                .iter()
                .fold(request, |request, group| request.security_group_ids(group)),
        };
        metadata.check_deadline()?;
        let response = request.send().await.map_err(|error| failure(&error))?;
        let id = match response.instances() {
            [instance] => instance.instance_id().filter(|id| !id.is_empty()),
            _ => None,
        }
        .ok_or(Error::Provider(ProviderErrorCode::Indeterminate))?
        .to_owned();
        // DescribeInstances is eventually consistent; wait briefly so assignment can read it.
        let wait = std::time::Duration::from_secs(2);
        let _ = poll_until("EC2 instance", wait, wait * 10, || async {
            Ok(self.describe(&id).await.ok().flatten().map(drop))
        })
        .await;
        let instance = ProvisionedInstance {
            id,
            provider: CloudProvider::AWS,
            instance_type: offer.instance_type.clone(),
            region: self.region.clone(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        };
        let mut resource = metadata
            .clone()
            .bind(instance, ManagedResourceKind::Instance);
        resource.gpu_class = Some(offer.gpu_class.clone());
        resource.gpu_count = Some(offer.gpu_count);
        resource.gpu_vram_gb =
            blueprint_core::gpu::find(&offer.gpu_class).map(|class| class.vram_gb);
        resource.hourly_cost_usd = Some(offer.hourly_cost_usd);
        Ok(resource)
    }

    async fn complete(
        &self,
        _spec: &ResourceSpec,
        resource: ManagedResource,
    ) -> Result<ManagedResource> {
        Ok(resource)
    }

    async fn status(
        &self,
        identity: &ManagedLeaseIdentity,
        id: &str,
    ) -> Result<Option<ManagedResource>> {
        let Some(raw) = self.describe(id).await? else {
            return Ok(None);
        };
        let resource = self
            .observe(std::slice::from_ref(&raw))
            .await?
            .pop()
            .ok_or(Error::Provider(ProviderErrorCode::Conflict))?;
        if resource.namespace != identity.namespace || resource.lease_id != identity.lease_id {
            return Err(Error::Provider(ProviderErrorCode::Conflict));
        }
        Ok(Some(resource))
    }

    async fn list(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let mut instances = Vec::new();
        let mut token = None::<String>;
        let mut seen = HashSet::new();
        loop {
            let page = self
                .ec2
                .describe_instances()
                .filters(filter("tag:bpg-namespace", &[namespace]))
                .filters(filter("instance-state-name", &PRESENT_STATES))
                .max_results(1000)
                .set_next_token(token.take())
                .send()
                .await
                .map_err(|error| failure(&error))?;
            instances.extend(
                page.reservations()
                    .iter()
                    .flat_map(|reservation| reservation.instances())
                    .cloned(),
            );
            let Some(next) = page.next_token().filter(|next| !next.is_empty()) else {
                break;
            };
            if !seen.insert(next.to_owned()) {
                return Err(Error::Provider(ProviderErrorCode::ProviderError));
            }
            token = Some(next.to_owned());
        }
        let mut resources = self.observe(&instances).await?;
        resources.retain(|resource| resource.namespace == namespace);
        Ok(resources)
    }

    async fn destroy(
        &self,
        identity: &ManagedLeaseIdentity,
        resource: &ManagedResource,
    ) -> Result<()> {
        if resource.kind != ManagedResourceKind::Instance {
            return Err(Error::Provider(ProviderErrorCode::Unsupported));
        }
        let id = resource.instance.id.as_str();
        if <Self as ManagedProviderAdapter>::status(self, identity, id)
            .await?
            .is_none_or(|current| !current.is_present())
        {
            return Ok(());
        }
        match self.ec2.terminate_instances().instance_ids(id).send().await {
            Ok(_) => Ok(()),
            Err(error) => match failure(&error) {
                Error::Provider(ProviderErrorCode::NotFound) => Ok(()),
                error => Err(error),
            },
        }
    }
}

#[async_trait]
impl CloudProviderAdapter for AwsManagedAdapter {
    fn managed(&self) -> Option<&dyn ManagedProviderAdapter> {
        Some(self)
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let raw = self
            .describe(id)
            .await?
            .ok_or(Error::Provider(ProviderErrorCode::NotFound))?;
        self.instance(&raw)
    }
}
