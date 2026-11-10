use super::leases::failure;
use super::*;
use crate::{CloudProvider, CloudProvisioner, Error};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use blueprint_core::resources::GpuArm;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const MAX_ENCODED_CANDIDATE_BYTES: usize = 65_536;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Candidate {
    pub request_fingerprint: String,
    pub offer: NativeOffer,
    pub arm: Option<GpuArm>,
}

impl Candidate {
    pub fn decode(value: &str) -> Result<Self> {
        if value.len() > MAX_ENCODED_CANDIDATE_BYTES {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        URL_SAFE_NO_PAD
            .decode(value)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .ok_or(Error::Provider(ProviderErrorCode::InvalidRequest))
    }

    pub fn allocation_fingerprint(&self) -> Result<String> {
        let bytes = serde_json::to_vec(&(
            &self.request_fingerprint,
            &self.offer.provider,
            &self.offer.native_id,
            &self.offer.instance_type,
            &self.offer.region,
            &self.offer.gpu_class,
            self.offer.gpu_count,
            self.offer.spot,
        ))
        .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        Ok(hex::encode(aws_lc_rs::digest::digest(
            &aws_lc_rs::digest::SHA256,
            &bytes,
        )))
    }
}

/// One selector projection shared by new encoding and legacy-aware validation.
pub(super) fn selector(arm: &GpuArm) -> GpuArm {
    let mut selected = arm.clone();
    selected.measurements.clear();
    selected
}

pub(super) fn resolve(spec: &ResourceSpec, arm: Option<&GpuArm>) -> ResourceSpec {
    let mut resolved = spec.clone();
    if let Some(arm) = arm {
        resolved.gpu_class = Some(arm.gpu_class.clone());
        resolved.gpu_count = Some(arm.gpu_count);
        resolved.arms.clear();
    }
    resolved
}

pub(super) fn valid_offer(spec: &ResourceSpec, offer: &NativeOffer) -> bool {
    !offer.native_id.is_empty()
        && !offer.instance_type.is_empty()
        && !offer.region.is_empty()
        && !offer.pricing_source.is_empty()
        && offer.hourly_cost_usd.is_finite()
        && offer.hourly_cost_usd > 0.0
        && (!offer.spot || spec.allow_spot)
        && spec
            .provider
            .as_ref()
            .is_none_or(|provider| provider == &offer.provider)
        && spec
            .region
            .as_ref()
            .is_none_or(|region| region == &offer.region)
        && spec
            .gpu_class
            .as_ref()
            .is_none_or(|class| class == &offer.gpu_class)
        && spec.gpu_count.is_none_or(|count| count == offer.gpu_count)
        && spec
            .max_hourly_cost_usd
            .is_none_or(|cap| offer.hourly_cost_usd <= cap)
        && blueprint_core::gpu::find(&offer.gpu_class).is_some_and(|class| {
            offer.gpu_count > 0
                && offer.gpu_count <= class.max_per_node
                && spec
                    .gpu_vram_gb
                    .is_none_or(|minimum| class.vram_gb >= minimum)
        })
}

/// Measured offers sort by USD/output token. Unmeasured offers retain USD/hour.
/// Deduplication requires an explicitly advertised underlying allocation identity.
pub fn rank_quotes(mut quotes: Vec<ProviderQuote>) -> Vec<ProviderQuote> {
    quotes.sort_by(|a, b| {
        let measured = match (
            a.usd_per_million_output_tokens,
            b.usd_per_million_output_tokens,
        ) {
            (Some(a), Some(b)) => a.total_cmp(&b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.offer.hourly_cost_usd.total_cmp(&b.offer.hourly_cost_usd),
        };
        measured
            .then_with(|| {
                a.offer
                    .provider
                    .catalog_key()
                    .cmp(b.offer.provider.catalog_key())
            })
            .then_with(|| a.offer.native_id.cmp(&b.offer.native_id))
            .then_with(|| a.offer.region.cmp(&b.offer.region))
    });
    let mut seen = HashSet::new();
    quotes.retain(|quote| match &quote.offer.underlying {
        Some(identity)
            if !identity.cloud.is_empty()
                && !identity.sku.is_empty()
                && !identity.region.is_empty() =>
        {
            seen.insert(identity.clone())
        }
        _ => true,
    });
    quotes
}

fn quote(spec: &ResourceSpec, offer: NativeOffer, arm: Option<&GpuArm>) -> Result<ProviderQuote> {
    let measurement = arm.and_then(|arm| {
        arm.measurements
            .iter()
            .filter(|row| {
                row.provider == offer.provider.catalog_key()
                    && row.provider_sku == offer.instance_type
                    && row.region == offer.region
                    && row.gpu_class == offer.gpu_class
                    && row.allocated_gpu_count == offer.gpu_count
                    && spec
                        .weights
                        .as_ref()
                        .is_some_and(|weights| weights.manifest_sha == row.weights_manifest_sha256)
                    && spec.image.as_ref().is_some_and(|image| image == &row.image)
                    && row.raw_result_sha256.len() == 64
                    && row
                        .raw_result_sha256
                        .bytes()
                        .all(|byte| byte.is_ascii_hexdigit())
                    && row.output_tokens_per_second().is_some()
            })
            .filter_map(|row| {
                chrono::DateTime::parse_from_rfc3339(&row.measured_at)
                    .ok()
                    .map(|time| (time, row))
            })
            .max_by_key(|(time, _)| *time)
            .map(|(_, row)| row.clone())
    });
    let throughput = measurement
        .as_ref()
        .and_then(|row| row.output_tokens_per_second());
    let token_cost = throughput.map(|rate| offer.hourly_cost_usd * 1_000_000.0 / (rate * 3600.0));
    // Normalize only newly emitted public provenance; old stored candidates are untouched.
    let mut offer = offer;
    if offer.pricing_source != CONFIGURED_PRICING {
        let source = url::Url::parse(&offer.pricing_source)
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))?;
        if !matches!(source.scheme(), "http" | "https") || source.host_str().is_none() {
            return Err(Error::Provider(ProviderErrorCode::ProviderError));
        }
        offer.pricing_source = source.origin().ascii_serialization();
    }
    let candidate = Candidate {
        request_fingerprint: request_fingerprint(spec)?,
        offer: offer.clone(),
        arm: arm.map(selector),
    };
    let candidate = serde_json::to_vec(&candidate)
        .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
    let candidate = URL_SAFE_NO_PAD.encode(candidate);
    if candidate.len() > MAX_ENCODED_CANDIDATE_BYTES {
        return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
    }
    Ok(ProviderQuote {
        offer,
        candidate,
        family: arm.map(|arm| arm.family.clone()),
        arm: arm.map(|arm| arm.arm.clone()),
        output_tokens_per_second: throughput,
        usd_per_million_output_tokens: token_cost,
        measurement: measurement.as_ref().map(MeasurementReceipt::from),
    })
}

impl CloudProvisioner {
    pub(super) fn managed_adapter(
        &self,
        provider: &CloudProvider,
    ) -> Result<&dyn ManagedProviderAdapter> {
        self.get_adapter(provider)?
            .managed()
            .ok_or(Error::Provider(ProviderErrorCode::Unsupported))
    }

    pub async fn quotes(&self, spec: &ResourceSpec) -> Result<QuoteSet> {
        spec.validate()
            .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
        if spec.gpu_count.is_none() && spec.arms.is_empty() {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        if !self.providers.keys().any(|provider| {
            spec.provider
                .as_ref()
                .is_none_or(|requested| requested == provider)
        }) {
            return Err(Error::Provider(ProviderErrorCode::ProviderNotConfigured));
        }
        let arms: Vec<_> = if spec.arms.is_empty() {
            vec![None]
        } else {
            spec.arms.iter().map(Some).collect()
        };
        let mut result = QuoteSet {
            quotes: Vec::new(),
            errors: Vec::new(),
        };
        for (provider, adapter) in &self.providers {
            if self
                .selection
                .as_ref()
                .is_some_and(|selected| !selected.contains(provider))
            {
                continue;
            }
            if spec
                .provider
                .as_ref()
                .is_some_and(|constraint| constraint != provider)
            {
                continue;
            }
            let Some(native) = adapter.managed() else {
                result
                    .errors
                    .push(failure(provider, ProviderErrorCode::Unsupported));
                continue;
            };
            for arm in &arms {
                let mut resolved = resolve(spec, *arm);
                let region = spec
                    .region
                    .as_deref()
                    .or_else(|| self.regions.get(provider).map(String::as_str))
                    .unwrap_or("");
                if !region.is_empty() {
                    resolved.region = Some(region.to_owned());
                }
                match native.offers(&resolved, region).await {
                    Ok(offers) => {
                        let before = result.quotes.len();
                        for offer in offers {
                            if &offer.provider == provider && valid_offer(&resolved, &offer) {
                                result.quotes.push(quote(spec, offer, *arm)?);
                            }
                        }
                        if before == result.quotes.len() {
                            result
                                .errors
                                .push(failure(provider, ProviderErrorCode::NoStock));
                        }
                    }
                    Err(error) => result.errors.push(failure(provider, error.into())),
                }
            }
        }
        result.quotes = rank_quotes(result.quotes);
        Ok(result)
    }
}
