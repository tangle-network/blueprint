//! Akash Console API: SDK-owned auction preparation, side-effect-free live bids,
//! then lease acceptance after the deployment id has been durably recorded.
use crate::{
    CloudProvider, ResourceSpec,
    core::error::{Error, Result},
    infra::{
        traits::{BlueprintDeploymentResult, CloudProviderAdapter},
        types::{InstanceStatus, ProvisionedInstance},
    },
    providers::{
        akash::sdl::build_sdl_manifest,
        common::gpu_adapter::{
            DEFAULT_GPU_IMAGE, api_json, api_json_or_missing, catalog_sku, catalog_skus,
            quoted_selection, reject_tee, ssh_endpoint, starting_instance,
        },
    },
    security::{ApiAuthentication, SecureHttpClient},
};
use async_trait::async_trait;
use blueprint_std::collections::HashMap;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};

struct Auction {
    dseq: String,
    spec: ResourceSpec,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Bid {
    dseq: String,
    gseq: u64,
    oseq: u64,
    provider: String,
    amount: String,
    denom: String,
}

pub struct AkashAdapter {
    http: SecureHttpClient,
    auth: ApiAuthentication,
    endpoint: String,
    auction: Option<Auction>,
    claimed: AtomicBool,
}

impl AkashAdapter {
    pub async fn new() -> Result<Self> {
        let key = std::env::var("AKASH_API_KEY")
            .map_err(|_| Error::ConfigurationError("AKASH_API_KEY is required".into()))?;
        Self::with_credentials(key)
    }

    pub fn with_credentials(key: String) -> Result<Self> {
        Self::with_endpoint(key, "https://console-api.akash.network")
    }
    pub fn with_endpoint(key: String, endpoint: &str) -> Result<Self> {
        Ok(Self {
            http: SecureHttpClient::with_endpoint(endpoint)?,
            auth: ApiAuthentication::ApiKey {
                key,
                header_name: "x-api-key".into(),
            },
            endpoint: endpoint.trim_end_matches('/').into(),
            auction: None,
            claimed: AtomicBool::new(false),
        })
    }

    /// Open an unleased auction explicitly, before adding this adapter to provider
    /// comparison. The SDK owns the SDL and chain HTTP; quote never creates one.
    /// Keep the returned adapter or close its auction if no lease is wanted.
    pub async fn prepare_auction(mut self, spec: &ResourceSpec) -> Result<Self> {
        supported_workload(spec)?;
        let model = catalog_sku(spec, "akash")?;
        let key = spec
            .ssh_public_key
            .as_deref()
            .filter(|k| !k.is_empty())
            .ok_or_else(|| {
                configuration("Akash auction preparation requires the lessee's SSH public key")
            })?;
        let ceiling = spec
            .max_hourly_cost_usd
            .filter(|p| p.is_finite() && *p > 0.0)
            .ok_or_else(|| {
                configuration("Set max_hourly_cost_usd before preparing an Akash auction")
            })?;
        let blocks = self.blocks_per_hour().await?;
        let per_block = (ceiling * 1_000_000.0 / blocks).floor();
        if per_block < 1.0 || per_block > u64::MAX as f64 {
            return Err(configuration(
                "Akash bid ceiling is outside supported integer units",
            ));
        }
        let sdl = build_sdl_manifest(
            spec.image.as_deref().unwrap_or(DEFAULT_GPU_IMAGE),
            &model,
            spec,
            &[22],
            per_block as u64,
        );
        let _ = key;
        let mut body = json!({"data":{"sdl":sdl}});
        if let Some(hours) = spec
            .lease_duration_secs
            .map(|s| s.div_ceil(3600))
            .filter(|h| (1..=48).contains(h))
        {
            body["data"]["runtimeLimitHours"] = json!(hours);
        }
        let response = api_json(
            self.http
                .post(
                    &format!("{}/v1/deployments", self.endpoint),
                    &self.auth,
                    Some(body),
                )
                .await?,
        )
        .await?;
        let data = response
            .get("data")
            .ok_or_else(|| configuration("Akash response missing deployment data"))?;
        if data
            .pointer("/signTx/code")
            .and_then(Value::as_u64)
            .is_some_and(|code| code != 0)
        {
            return Err(configuration("Akash deployment transaction failed"));
        }
        let dseq = data
            .get("dseq")
            .and_then(Value::as_str)
            .filter(|s| valid_dseq(s))
            .ok_or_else(|| configuration("Akash response missing deployment sequence"))?
            .to_string();
        self.auction = Some(Auction {
            dseq,
            spec: spec.clone(),
        });
        Ok(self)
    }

    pub fn auction_id(&self) -> Option<&str> {
        self.auction.as_ref().map(|a| a.dseq.as_str())
    }

    /// Close an unselected prepared auction through the same provider retire path.
    pub async fn close_auction(&self) -> Result<()> {
        self.terminate_instance(
            self.auction_id()
                .ok_or_else(|| configuration("No prepared Akash auction"))?,
        )
        .await
    }

    fn auction_for(&self, spec: &ResourceSpec, region: &str) -> Result<&Auction> {
        supported_workload(spec)?;
        spec.validate().map_err(Error::ConfigurationError)?;
        if !matches!(region, "" | "global") {
            return Err(configuration("This Akash auction has global placement"));
        }
        let auction = self.auction.as_ref().ok_or_else(|| {
            configuration("Prepare an SDK Akash auction before comparing live bids")
        })?;
        let bound = &auction.spec;
        if spec.cpu != bound.cpu
            || spec.memory_gb != bound.memory_gb
            || spec.storage_gb != bound.storage_gb
            || spec.gpu_count != bound.gpu_count
            || spec.gpu_class != bound.gpu_class
            || spec.gpu_vram_gb != bound.gpu_vram_gb
            || spec.image != bound.image
            || spec.ssh_public_key != bound.ssh_public_key
        {
            return Err(configuration(
                "Request does not match the prepared Akash auction",
            ));
        }
        Ok(auction)
    }

    /// Billing is micro-USD per block. Convert using observed recent chain block
    /// timestamps; neither AKT exchange rates nor a fixed block-time guess enter.
    async fn blocks_per_hour(&self) -> Result<f64> {
        let value = api_json(
            self.http
                .get(&format!("{}/v1/blocks", self.endpoint), &self.auth)
                .await?,
        )
        .await?;
        let mut samples: Vec<_> = value
            .as_array()
            .ok_or_else(|| configuration("Missing Akash block observations"))?
            .iter()
            .filter_map(|v| {
                Some((
                    v.get("height")?.as_u64()?,
                    chrono::DateTime::parse_from_rfc3339(v.get("datetime")?.as_str()?).ok()?,
                ))
            })
            .collect();
        samples.sort_by_key(|(height, _)| *height);
        let (first, last) = (
            samples
                .first()
                .ok_or_else(|| configuration("Missing Akash block observations"))?,
            samples
                .last()
                .ok_or_else(|| configuration("Missing Akash block observations"))?,
        );
        let elapsed = last.1.signed_duration_since(first.1).num_milliseconds() as f64 / 1000.0;
        let age = chrono::Utc::now()
            .signed_duration_since(last.1)
            .num_seconds();
        if elapsed <= 0.0 || last.0 <= first.0 || !(-60..=300).contains(&age) {
            return Err(configuration(
                "Akash block observations are stale or invalid",
            ));
        }
        Ok(3600.0 * (last.0 - first.0) as f64 / elapsed)
    }

    async fn bids(&self, spec: &ResourceSpec, region: &str) -> Result<Vec<(Bid, f64)>> {
        let auction = self.auction_for(spec, region)?;
        let models = catalog_skus(spec, "akash")?;
        let required_vram = spec
            .gpu_vram_gb
            .or_else(|| {
                spec.gpu_class
                    .as_deref()
                    .and_then(blueprint_core::gpu::find)
                    .map(|g| g.vram_gb)
            })
            .unwrap_or(0);
        let value = api_json(
            self.http
                .get(
                    &format!("{}/v1/bids?dseq={}", self.endpoint, auction.dseq),
                    &self.auth,
                )
                .await?,
        )
        .await?;
        let blocks = self.blocks_per_hour().await?;
        let mut bids = Vec::new();
        for item in value
            .get("data")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let Some(bid) = item.get("bid") else { continue };
            let Some(id) = bid.get("id") else { continue };
            if bid.get("state").and_then(Value::as_str) != Some("open")
                || id.get("dseq").and_then(Value::as_str) != Some(auction.dseq.as_str())
            {
                continue;
            }
            // Native ACT credits are denominated in USD. AKT and other unknown
            // conversion rates are not executable USD quotes.
            if bid.pointer("/price/denom").and_then(Value::as_str) != Some("uact") {
                continue;
            }
            let Some(amount) = bid.pointer("/price/amount").and_then(Value::as_str) else {
                continue;
            };
            let Some(price) = amount
                .parse::<f64>()
                .ok()
                .filter(|p| p.is_finite() && *p > 0.0)
                .map(|p| p * blocks / 1_000_000.0)
            else {
                continue;
            };
            if spec.max_hourly_cost_usd.is_some_and(|cap| price > cap) {
                continue;
            }
            let Some(offers) = bid
                .get("resources_offer")
                .and_then(Value::as_array)
                .filter(|o| o.len() == 1)
            else {
                continue;
            };
            let offer = &offers[0];
            if offer.get("count").and_then(Value::as_u64) != Some(1) {
                continue;
            }
            let resource = &offer["resources"];
            if integer(resource, "/gpu/units/val") != spec.gpu_count.map(u64::from)
                || integer(resource, "/cpu/units/val")
                    .is_none_or(|v| v < (spec.cpu as f64 * 1000.0).ceil() as u64)
                || integer(resource, "/memory/quantity/val")
                    .is_none_or(|v| v < (spec.memory_gb as f64 * 1_073_741_824.0).ceil() as u64)
                || resource
                    .get("storage")
                    .and_then(Value::as_array)
                    .map(|s| {
                        s.iter()
                            .filter_map(|s| integer(s, "/quantity/val"))
                            .fold(0u64, u64::saturating_add)
                    })
                    .is_none_or(|v| v < (spec.storage_gb as f64 * 1_073_741_824.0).ceil() as u64)
            {
                continue;
            }
            let matching = resource
                .pointer("/gpu/attributes")
                .and_then(Value::as_array)
                .is_some_and(|attributes| {
                    attributes.iter().any(|attribute| {
                        if attribute.get("value").and_then(Value::as_str) != Some("true") {
                            return false;
                        }
                        let Some(key) = attribute.get("key").and_then(Value::as_str) else {
                            return false;
                        };
                        let parts: Vec<_> = key.split('/').collect();
                        let model = parts.windows(2).find(|p| p[0] == "model").map(|p| p[1]);
                        let ram = parts
                            .windows(2)
                            .find(|p| p[0] == "ram")
                            .and_then(|p| p[1].strip_suffix("Gi"))
                            .and_then(|v| v.parse::<u32>().ok());
                        key.starts_with("vendor/nvidia/")
                            && model.is_some_and(|m| models.iter().any(|v| v == m))
                            && ram.is_some_and(|ram| ram >= required_vram)
                    })
                });
            if !matching {
                continue;
            }
            let (Some(gseq), Some(oseq), Some(provider)) = (
                id.get("gseq").and_then(Value::as_u64),
                id.get("oseq").and_then(Value::as_u64),
                id.get("provider").and_then(Value::as_str),
            ) else {
                continue;
            };
            bids.push((
                Bid {
                    dseq: auction.dseq.clone(),
                    gseq,
                    oseq,
                    provider: provider.into(),
                    amount: amount.into(),
                    denom: "uact".into(),
                },
                price,
            ));
        }
        bids.sort_by(|a, b| a.1.total_cmp(&b.1));
        Ok(bids)
    }

    async fn deployment(&self, id: &str) -> Result<Option<Value>> {
        if !valid_dseq(id) {
            return Err(configuration("Invalid Akash deployment sequence"));
        }
        api_json_or_missing(
            self.http
                .get(
                    &format!("{}/v1/deployments/{id}", self.endpoint),
                    &self.auth,
                )
                .await?,
        )
        .await
    }

    fn endpoint(value: &Value) -> Result<String> {
        for lease in value
            .pointer("/data/leases")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if lease.get("state").and_then(Value::as_str) != Some("active") {
                continue;
            }
            for ports in lease
                .pointer("/status/forwarded_ports")
                .and_then(Value::as_object)
                .into_iter()
                .flat_map(|v| v.values())
            {
                for port in ports.as_array().into_iter().flatten() {
                    if port.get("port").and_then(Value::as_u64) == Some(22)
                        && port
                            .get("available")
                            .and_then(Value::as_u64)
                            .is_some_and(|n| n > 0)
                    {
                        if let (Some(host), Some(external)) = (
                            port.get("host").and_then(Value::as_str),
                            port.get("externalPort")
                                .and_then(Value::as_u64)
                                .and_then(|p| u16::try_from(p).ok()),
                        ) {
                            return ssh_endpoint(&format!("ssh root@{host} -p {external}"));
                        }
                    }
                }
            }
        }
        Err(configuration("Akash SSH endpoint is not ready"))
    }
}

fn configuration(message: &str) -> Error {
    Error::ConfigurationError(message.into())
}
fn valid_dseq(id: &str) -> bool {
    !id.is_empty()
        && id.bytes().all(|b| b.is_ascii_digit())
        && id.parse::<u64>().is_ok_and(|n| n > 0)
}
fn integer(value: &Value, path: &str) -> Option<u64> {
    value.pointer(path)?.as_str()?.parse().ok()
}

fn supported_workload(spec: &ResourceSpec) -> Result<()> {
    if !spec.env.is_empty()
        || !spec.ports.is_empty()
        || spec.command.is_some()
        || spec.endpoint_port.is_some()
        || spec.source_network.is_some()
        || spec.volume_gb.is_some_and(|v| v > 0.0)
        || spec.volume_mount_path.is_some()
        || spec.manifest.is_some()
    {
        return Err(Error::ConfigurationError(
            "This adapter does not support the requested workload/network/volume fields".into(),
        ));
    }
    Ok(())
}

#[async_trait]
impl CloudProviderAdapter for AkashAdapter {
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        let (bid, price) = self
            .bids(spec, region)
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| {
                configuration("No live Akash USD bid satisfies the exact GPU request")
            })?;
        quoted_selection(
            serde_json::to_string(&bid).map_err(|_| configuration("Invalid Akash bid"))?,
            price,
        )
    }
    async fn provision_instance(
        &self,
        _instance_type: &str,
        _region: &str,
        _tee: bool,
    ) -> Result<ProvisionedInstance> {
        Err(configuration(
            "Akash requires a prepared auction and complete ResourceSpec",
        ))
    }
    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        reject_tee(tee)?;
        let expected: Bid = serde_json::from_str(instance_type)
            .map_err(|_| configuration("Invalid selected Akash bid"))?;
        let valid = self.bids(spec, region).await?.into_iter().any(|(b, _)| {
            b.dseq == expected.dseq
                && b.gseq == expected.gseq
                && b.oseq == expected.oseq
                && b.provider == expected.provider
                && b.amount == expected.amount
                && b.denom == expected.denom
        });
        if !valid {
            return Err(configuration("Selected Akash bid is no longer available"));
        }
        self.claimed
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .map_err(|_| configuration("This Akash auction has already been allocated"))?;
        Ok(starting_instance(
            CloudProvider::Akash,
            expected.dseq,
            instance_type.into(),
            "global".into(),
        ))
    }
    async fn prepare_access(
        &self,
        instance: &ProvisionedInstance,
        spec: &ResourceSpec,
    ) -> Result<()> {
        let selected: Bid = serde_json::from_str(&instance.instance_type)
            .map_err(|_| configuration("Invalid selected Akash bid"))?;
        let valid = self.bids(spec, "").await?.into_iter().any(|(b, _)| {
            b.dseq == selected.dseq
                && b.gseq == selected.gseq
                && b.oseq == selected.oseq
                && b.provider == selected.provider
                && b.amount == selected.amount
                && b.denom == selected.denom
        });
        if !valid || selected.dseq != instance.id {
            return Err(configuration(
                "Selected Akash bid changed before lease acceptance",
            ));
        }
        api_json(self.http.post(&format!("{}/v1/leases",self.endpoint),&self.auth,Some(json!({"leases":[{"dseq":selected.dseq,"gseq":selected.gseq,"oseq":selected.oseq,"provider":selected.provider}]}))).await?).await?;
        Ok(())
    }
    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        let mut instance = starting_instance(
            CloudProvider::Akash,
            id.into(),
            "akash-deployment".into(),
            "global".into(),
        );
        let Some(value) = self.deployment(id).await? else {
            instance.status = InstanceStatus::Terminated;
            return Ok(instance);
        };
        if value
            .pointer("/data/deployment/state")
            .and_then(Value::as_str)
            == Some("closed")
        {
            instance.status = InstanceStatus::Terminated;
            return Ok(instance);
        }
        if let Ok(endpoint) = Self::endpoint(&value) {
            let url = url::Url::parse(&endpoint)
                .map_err(|_| configuration("Invalid Akash access endpoint"))?;
            instance.public_ip = url.host_str().map(str::to_string);
            instance.status = InstanceStatus::Running;
        }
        Ok(instance)
    }
    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        Ok(self.get_instance_details(id).await?.status)
    }
    async fn lease_endpoint(&self, instance: &ProvisionedInstance) -> Result<String> {
        Self::endpoint(
            &self
                .deployment(&instance.id)
                .await?
                .ok_or_else(|| configuration("Akash deployment no longer exists"))?,
        )
    }
    async fn terminate_instance(&self, id: &str) -> Result<()> {
        if !valid_dseq(id) {
            return Err(configuration("Invalid Akash deployment sequence"));
        }
        let response = self
            .http
            .delete(
                &format!("{}/v1/deployments/{id}", self.endpoint),
                &self.auth,
            )
            .await?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(());
        }
        let value = api_json(response).await?;
        if value.pointer("/data/success").and_then(Value::as_bool) == Some(false) {
            return Err(configuration("Akash deployment close was refused"));
        }
        Ok(())
    }
    async fn deploy_blueprint_with_target(
        &self,
        _target: &crate::core::deployment_target::DeploymentTarget,
        _image: &str,
        _spec: &ResourceSpec,
        _env: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        Err(configuration(
            "Set the workload image in ResourceSpec, prepare its Akash auction, then allocate the lease",
        ))
    }
    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        Ok(self.get_instance_status(&deployment.instance.id).await? == InstanceStatus::Running)
    }
    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        self.terminate_instance(&deployment.instance.id).await
    }
}
