//! Operator capabilities matrix — derives what an operator CAN provision
//! from their configured cloud providers and the blueprint's requirements.
//!
//! Used during pre-registration to auto-generate registration payloads and
//! pricing configs without the operator manually declaring hardware specs.

use crate::config::BlueprintManagerContext;
use blueprint_remote_providers::CloudProvider;
use blueprint_remote_providers::core::resources::ResourceSpec;
use blueprint_remote_providers::providers::common::InstanceSelection;
use serde::{Deserialize, Serialize};

/// A single deployable configuration an operator can offer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorCapability {
    /// Cloud provider that would serve this config.
    pub provider: CloudProvider,
    /// Provider-specific instance type (e.g. "gpu_1x_a100", "NVIDIA A100 80GB PCIe").
    pub instance_type: String,
    /// GPU count for this config.
    pub gpu_count: u32,
    /// Estimated VRAM in MiB (derived from instance type).
    pub estimated_vram_mib: u32,
    /// Whether this provider supports TEE.
    pub tee_capable: bool,
    /// Estimated hourly cost in USD.
    pub hourly_cost_usd: f64,
    /// Can use spot/preemptible pricing.
    pub spot_capable: bool,
}

/// Pricing derived from a capability — what to charge customers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedPricing {
    /// Source capability this pricing is derived from.
    pub capability: OperatorCapability,
    /// Operator margin (e.g. 1.3 = 30% markup over infrastructure cost).
    pub margin: f64,
    /// Per-input-token price (for LLM/embedding blueprints).
    pub price_per_input_token: u64,
    /// Per-output-token price (for LLM blueprints).
    pub price_per_output_token: u64,
    /// Per-compute-second price (for video/avatar/training).
    pub price_per_compute_second: u64,
    /// Per-image price (for image-gen).
    pub price_per_image: u64,
}

/// Build the full capabilities matrix from an operator's configured providers.
///
/// For each configured provider, maps the blueprint's GPU requirements to
/// concrete instance types and derives pricing. Returns multiple capabilities
/// if the operator has multiple providers configured.
pub fn build_capabilities_matrix(
    ctx: &BlueprintManagerContext,
    gpu_count: u32,
    min_vram_gb: u32,
) -> Vec<OperatorCapability> {
    let mut capabilities = Vec::new();

    let Some(config) = ctx.cloud_config() else {
        return capabilities;
    };

    // Build a ResourceSpec from the blueprint's GPU requirements.
    // memory_gb is system RAM (separate from VRAM). GPU instances typically
    // have 4-8x system RAM per GPU VRAM — use 4x as a conservative floor.
    let system_ram_gb = (min_vram_gb as f32 * 4.0).max(32.0);
    let spec = ResourceSpec {
        cpu: 8.0,
        memory_gb: system_ram_gb,
        storage_gb: 100.0,
        gpu_count: if gpu_count > 0 { Some(gpu_count) } else { None },
        allow_spot: false,
        qos: Default::default(),
    };

    let providers_with_regions = configured_providers(config);

    for (provider, _region) in &providers_with_regions {
        let selection = map_provider_instance(provider, &spec);
        let provider_supports_tee = provider_supports_tee_instances(provider);
        let hourly = selection.estimated_hourly_cost.unwrap_or(0.0);

        // Skip providers that returned zero cost (unconfigured or mapping failed)
        if hourly <= 0.0 {
            continue;
        }

        // Estimate VRAM from instance type name
        let vram_per_gpu = estimate_vram_mib(&selection.instance_type);
        let total_vram = if gpu_count > 0 {
            vram_per_gpu * gpu_count
        } else {
            0 // no GPU = no VRAM
        };

        // Non-TEE config
        capabilities.push(OperatorCapability {
            provider: provider.clone(),
            instance_type: selection.instance_type.clone(),
            gpu_count,
            estimated_vram_mib: total_vram,
            tee_capable: false, // explicit: this config does NOT require TEE
            hourly_cost_usd: hourly,
            spot_capable: selection.spot_capable,
        });

        // TEE variant (only if provider supports it AND blueprint uses GPUs)
        if provider_supports_tee && gpu_count > 0 {
            capabilities.push(OperatorCapability {
                provider: provider.clone(),
                instance_type: selection.instance_type.clone(),
                gpu_count,
                estimated_vram_mib: total_vram,
                tee_capable: true,
                // TEE cost is provider-specific. Without live pricing API,
                // we can't know the real premium. Report the base cost and
                // let the pricing engine apply its configured tee.multiplier.
                hourly_cost_usd: hourly,
                spot_capable: false, // TEE instances are never spot
            });
        }
    }

    // Sort by cost ascending, NaN-safe
    capabilities.sort_by(|a, b| {
        a.hourly_cost_usd
            .partial_cmp(&b.hourly_cost_usd)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    capabilities
}

/// Derive pricing from a capability with a margin and throughput estimate.
///
/// `throughput` specifies the service-specific throughput for this GPU class,
/// which determines per-unit pricing. Callers MUST provide accurate estimates
/// for their service type — do not use defaults for production pricing.
pub fn derive_pricing(cap: &OperatorCapability, margin: f64) -> DerivedPricing {
    derive_pricing_with_throughput(cap, margin, &ThroughputEstimate::default())
}

/// Throughput estimates for pricing derivation. These are hardware + model
/// dependent — operators should benchmark and provide real numbers.
#[derive(Debug, Clone)]
pub struct ThroughputEstimate {
    /// Tokens per GPU-hour (input + output combined). Varies by model size:
    /// - 7B model on A100: ~100-150K tokens/hr
    /// - 70B model on A100: ~30K tokens/hr
    /// - 405B on 8xA100: ~15K tokens/hr
    pub tokens_per_hour: f64,
    /// Images per GPU-hour. Varies by model + resolution:
    /// - SD 1.5 512x512 on A100: ~2000/hr
    /// - SDXL 1024x1024: ~400/hr
    /// - FLUX.1: ~200/hr
    pub images_per_hour: f64,
    /// Payment token decimals (USDC=6, ETH=18).
    pub token_decimals: u32,
}

impl Default for ThroughputEstimate {
    fn default() -> Self {
        Self {
            tokens_per_hour: 30_000.0, // conservative: 70B model on A100 (published benchmarks: 20-35K)
            images_per_hour: 400.0,    // conservative: SDXL on A100
            token_decimals: 6,         // USDC default
        }
    }
}

/// Safely convert an f64 to u64, clamping NaN/Inf/negative to 0 and overflow to u64::MAX.
fn safe_f64_to_u64(value: f64) -> u64 {
    if value.is_nan() || value.is_infinite() || value < 0.0 {
        return 0;
    }
    if value > u64::MAX as f64 {
        return u64::MAX;
    }
    value as u64
}

pub fn derive_pricing_with_throughput(
    cap: &OperatorCapability,
    margin: f64,
    throughput: &ThroughputEstimate,
) -> DerivedPricing {
    let hourly = cap.hourly_cost_usd * margin;
    let base_unit = 10_f64.powi(throughput.token_decimals as i32);

    let cost_per_token = (hourly / throughput.tokens_per_hour) * base_unit;
    let price_per_input_token = safe_f64_to_u64((cost_per_token * 0.4).ceil());
    let price_per_output_token = safe_f64_to_u64((cost_per_token * 1.0).ceil());
    let price_per_compute_second = safe_f64_to_u64(((hourly / 3600.0) * base_unit).ceil());
    let price_per_image =
        safe_f64_to_u64(((hourly / throughput.images_per_hour) * base_unit).ceil());

    DerivedPricing {
        capability: cap.clone(),
        margin,
        price_per_input_token: price_per_input_token.max(1),
        price_per_output_token: price_per_output_token.max(1),
        price_per_compute_second: price_per_compute_second.max(1),
        price_per_image: price_per_image.max(1),
    }
}

/// Generate a pricing TOML string from derived pricing.
pub fn generate_pricing_toml(configs: &[DerivedPricing], tee_capable: bool) -> String {
    let mut toml = String::from("[default]\nresources = [\n");

    // Use the first config as the representative pricing
    if let Some(primary) = configs.first() {
        let cap = &primary.capability;
        if cap.gpu_count > 0 {
            toml.push_str(&format!(
                "  {{ kind = \"GPU\", count = {}, price_per_unit_rate = {:.6} }},\n",
                cap.gpu_count,
                cap.hourly_cost_usd * primary.margin / cap.gpu_count as f64
            ));
        }
        toml.push_str(&format!(
            "  {{ kind = \"CPU\", count = 8, price_per_unit_rate = {:.6} }},\n",
            0.001 * primary.margin
        ));
        toml.push_str("]\n\n");

        // Per-job pricing
        toml.push_str(&format!(
            "# Auto-generated from {} {} (${:.2}/hr + {:.0}% margin)\n",
            cap.provider,
            cap.instance_type,
            cap.hourly_cost_usd,
            (primary.margin - 1.0) * 100.0
        ));
        toml.push_str(&format!(
            "[0]\n0 = \"{}\"\n\n",
            primary.price_per_input_token
        ));
    }

    // TEE section
    toml.push_str(&format!(
        "[tee]\navailable = {}\nmultiplier = 1.5\n",
        tee_capable
    ));

    toml
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Whether a cloud provider offers TEE-capable instances.
///
/// This answers "can this PROVIDER provision TEE instances?", not "is THIS
/// machine running in a TEE?". For local hardware detection, see
/// `blueprint_tee::runtime::detect::detect_tee_provider`.
fn provider_supports_tee_instances(provider: &CloudProvider) -> bool {
    matches!(
        provider,
        CloudProvider::AWS       // Nitro Enclaves, SEV-SNP on c6a/m6a
        | CloudProvider::GCP     // Confidential VMs (SEV-SNP, TDX)
        | CloudProvider::Azure   // DCasv5 (SEV-SNP), ECasv5
        | CloudProvider::CoreWeave // Can provision on TDX/SEV hardware
    )
}

fn configured_providers(
    config: &blueprint_remote_providers::config::CloudConfig,
) -> Vec<(CloudProvider, String)> {
    let mut providers = Vec::new();
    if let Some(c) = &config.lambda_labs {
        if c.enabled {
            providers.push((CloudProvider::LambdaLabs, c.region.clone()));
        }
    }
    if let Some(c) = &config.runpod {
        if c.enabled {
            providers.push((CloudProvider::RunPod, c.region.clone()));
        }
    }
    if let Some(c) = &config.vast_ai {
        if c.enabled {
            providers.push((CloudProvider::VastAi, "global".into()));
        }
    }
    if let Some(c) = &config.coreweave {
        if c.enabled {
            providers.push((CloudProvider::CoreWeave, c.region.clone()));
        }
    }
    if let Some(c) = &config.paperspace {
        if c.enabled {
            providers.push((CloudProvider::Paperspace, c.region.clone()));
        }
    }
    if let Some(c) = &config.fluidstack {
        if c.enabled {
            providers.push((CloudProvider::Fluidstack, c.region.clone()));
        }
    }
    if let Some(c) = &config.tensordock {
        if c.enabled {
            providers.push((CloudProvider::TensorDock, c.region.clone()));
        }
    }
    if let Some(c) = &config.akash {
        if c.enabled {
            providers.push((CloudProvider::Akash, "global".into()));
        }
    }
    if let Some(c) = &config.io_net {
        if c.enabled {
            providers.push((CloudProvider::IoNet, c.region.clone()));
        }
    }
    if let Some(c) = &config.prime_intellect {
        if c.enabled {
            providers.push((CloudProvider::PrimeIntellect, c.region.clone()));
        }
    }
    if let Some(c) = &config.aws {
        if c.enabled {
            providers.push((CloudProvider::AWS, c.region.clone()));
        }
    }
    if let Some(c) = &config.gcp {
        if c.enabled {
            providers.push((CloudProvider::GCP, c.region.clone()));
        }
    }
    if let Some(c) = &config.azure {
        if c.enabled {
            providers.push((CloudProvider::Azure, c.region.clone()));
        }
    }
    providers
}

fn map_provider_instance(provider: &CloudProvider, spec: &ResourceSpec) -> InstanceSelection {
    use blueprint_remote_providers::infra::mapper::InstanceTypeMapper;
    let mapped = InstanceTypeMapper::map_to_instance_type(spec, provider);
    InstanceSelection {
        instance_type: mapped.instance_type,
        spot_capable: mapped.spot_capable,
        estimated_hourly_cost: Some(mapped.estimated_hourly_cost),
    }
}

fn estimate_vram_mib(instance_type: &str) -> u32 {
    let lower = instance_type.to_lowercase();
    if lower.contains("h200") {
        144384
    }
    // 141 GB HBM3e
    else if lower.contains("h100") {
        81920
    } else if lower.contains("a100") && lower.contains("80") {
        81920
    } else if lower.contains("a100") {
        40960
    } else if lower.contains("a6000") || lower.contains("rtx6000") {
        49152
    } else if lower.contains("a40") {
        49152
    } else if lower.contains("4090") {
        24576
    } else if lower.contains("3090") {
        24576
    } else if lower.contains("a10") {
        24576
    } else if lower.contains("t4") {
        16384
    } else if lower.contains("4080") {
        16384
    } else {
        16384
    } // default to T4 class
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a100_cap() -> OperatorCapability {
        OperatorCapability {
            provider: CloudProvider::LambdaLabs,
            instance_type: "gpu_1x_a100".into(),
            gpu_count: 1,
            estimated_vram_mib: 40960,
            tee_capable: false,
            hourly_cost_usd: 1.29,
            spot_capable: false,
        }
    }

    // ── VRAM estimation ─────────────────────────────────────────────────

    #[test]
    fn vram_h100_variants() {
        assert_eq!(estimate_vram_mib("gpu_1x_h100_pcie"), 81920);
        assert_eq!(estimate_vram_mib("H100_NVLINK_80GB"), 81920);
        assert_eq!(estimate_vram_mib("h100-80gb-sxm5"), 81920);
    }

    #[test]
    fn vram_a100_40_vs_80() {
        assert_eq!(estimate_vram_mib("gpu_1x_a100"), 40960); // no "80" → 40GB
        assert_eq!(estimate_vram_mib("a100_80gb_pcie"), 81920); // has "80" → 80GB
        assert_eq!(estimate_vram_mib("NVIDIA A100 80GB PCIe"), 81920);
    }

    #[test]
    fn vram_consumer_gpus() {
        assert_eq!(estimate_vram_mib("NVIDIA GeForce RTX 4090"), 24576);
        assert_eq!(estimate_vram_mib("rtx3090-24gb"), 24576);
        assert_eq!(estimate_vram_mib("NVIDIA GeForce RTX 3090"), 24576);
    }

    #[test]
    fn vram_unknown_defaults_to_t4() {
        assert_eq!(estimate_vram_mib("some_unknown_instance"), 16384);
    }

    // ── TEE support ─────────────────────────────────────────────────────

    #[test]
    fn tee_only_hyperscalers_and_coreweave() {
        assert!(provider_supports_tee_instances(&CloudProvider::AWS));
        assert!(provider_supports_tee_instances(&CloudProvider::GCP));
        assert!(provider_supports_tee_instances(&CloudProvider::Azure));
        assert!(provider_supports_tee_instances(&CloudProvider::CoreWeave));
        // GPU marketplaces don't offer TEE
        assert!(!provider_supports_tee_instances(&CloudProvider::RunPod));
        assert!(!provider_supports_tee_instances(&CloudProvider::LambdaLabs));
        assert!(!provider_supports_tee_instances(&CloudProvider::VastAi));
        assert!(!provider_supports_tee_instances(&CloudProvider::Paperspace));
        assert!(!provider_supports_tee_instances(&CloudProvider::Fluidstack));
        assert!(!provider_supports_tee_instances(&CloudProvider::TensorDock));
        // Decentralized providers: Akash can run on TEE hardware but we can't
        // guarantee it; Render has no TEE offering.
        assert!(!provider_supports_tee_instances(&CloudProvider::Akash));
        assert!(!provider_supports_tee_instances(&CloudProvider::Render));
    }

    // ── Pricing derivation ──────────────────────────────────────────────

    #[test]
    fn derive_pricing_all_fields_nonzero() {
        let pricing = derive_pricing(&a100_cap(), 1.3);
        assert!(
            pricing.price_per_input_token > 0,
            "input token price must be > 0"
        );
        assert!(
            pricing.price_per_output_token > 0,
            "output token price must be > 0"
        );
        assert!(
            pricing.price_per_compute_second > 0,
            "compute second price must be > 0"
        );
        assert!(pricing.price_per_image > 0, "image price must be > 0");
    }

    #[test]
    fn output_tokens_cost_more_than_input() {
        let pricing = derive_pricing(&a100_cap(), 1.0);
        assert!(
            pricing.price_per_output_token > pricing.price_per_input_token,
            "output ({}) should cost more than input ({})",
            pricing.price_per_output_token,
            pricing.price_per_input_token
        );
    }

    #[test]
    fn margin_increases_all_prices() {
        let cap = OperatorCapability {
            hourly_cost_usd: 10.0,
            ..a100_cap()
        };
        let low = derive_pricing(&cap, 1.0);
        let high = derive_pricing(&cap, 2.0);
        assert!(high.price_per_input_token > low.price_per_input_token);
        assert!(high.price_per_output_token > low.price_per_output_token);
        assert!(high.price_per_compute_second > low.price_per_compute_second);
        assert!(high.price_per_image > low.price_per_image);
    }

    #[test]
    fn custom_throughput_changes_prices() {
        let cap = OperatorCapability {
            hourly_cost_usd: 5.0,
            ..a100_cap()
        };
        let fast = derive_pricing_with_throughput(
            &cap,
            1.0,
            &ThroughputEstimate {
                tokens_per_hour: 200_000.0, // fast GPU
                images_per_hour: 2000.0,
                token_decimals: 6,
            },
        );
        let slow = derive_pricing_with_throughput(
            &cap,
            1.0,
            &ThroughputEstimate {
                tokens_per_hour: 10_000.0, // slow GPU
                images_per_hour: 100.0,
                token_decimals: 6,
            },
        );
        // Slower throughput → higher per-token cost
        assert!(
            slow.price_per_input_token > fast.price_per_input_token,
            "slow ({}) should cost more per token than fast ({})",
            slow.price_per_input_token,
            fast.price_per_input_token
        );
    }

    #[test]
    fn token_decimals_affect_prices() {
        let cap = OperatorCapability {
            hourly_cost_usd: 1.0,
            ..a100_cap()
        };
        let usdc = derive_pricing_with_throughput(
            &cap,
            1.0,
            &ThroughputEstimate {
                token_decimals: 6,
                ..ThroughputEstimate::default()
            },
        );
        let eth = derive_pricing_with_throughput(
            &cap,
            1.0,
            &ThroughputEstimate {
                token_decimals: 18,
                ..ThroughputEstimate::default()
            },
        );
        // 18-decimal token should have much larger base unit numbers
        assert!(
            eth.price_per_input_token > usdc.price_per_input_token,
            "18-decimal ({}) should be larger than 6-decimal ({})",
            eth.price_per_input_token,
            usdc.price_per_input_token
        );
    }

    #[test]
    fn compute_second_is_hourly_divided_by_3600() {
        let cap = OperatorCapability {
            hourly_cost_usd: 3600.0,
            ..a100_cap()
        };
        let pricing = derive_pricing_with_throughput(
            &cap,
            1.0,
            &ThroughputEstimate {
                token_decimals: 6,
                ..ThroughputEstimate::default()
            },
        );
        // $3600/hr / 3600 sec = $1/sec = 1_000_000 base units (6 decimals)
        assert_eq!(pricing.price_per_compute_second, 1_000_000);
    }

    // ── CPU-only (no GPU) ───────────────────────────────────────────────

    #[test]
    fn cpu_only_cap_has_zero_vram() {
        let cap = OperatorCapability {
            gpu_count: 0,
            estimated_vram_mib: 0,
            ..a100_cap()
        };
        assert_eq!(cap.estimated_vram_mib, 0);
    }

    // ── TOML generation ─────────────────────────────────────────────────

    #[test]
    fn toml_includes_gpu_resource() {
        let pricing = derive_pricing(&a100_cap(), 1.5);
        let toml = generate_pricing_toml(&[pricing], false);
        assert!(toml.contains("GPU"), "TOML should include GPU resource");
        assert!(toml.contains("count = 1"), "TOML should have gpu count");
    }

    #[test]
    fn toml_includes_tee_section() {
        let pricing = derive_pricing(&a100_cap(), 1.0);
        let toml = generate_pricing_toml(&[pricing], true);
        assert!(toml.contains("[tee]"));
        assert!(toml.contains("available = true"));
    }

    #[test]
    fn toml_tee_false_when_not_capable() {
        let pricing = derive_pricing(&a100_cap(), 1.0);
        let toml = generate_pricing_toml(&[pricing], false);
        assert!(toml.contains("available = false"));
    }

    #[test]
    fn toml_includes_source_comment() {
        let pricing = derive_pricing(&a100_cap(), 1.3);
        let toml = generate_pricing_toml(&[pricing], false);
        assert!(
            toml.contains("Auto-generated"),
            "TOML should have source comment"
        );
        assert!(
            toml.contains("Lambda Labs"),
            "TOML should name the provider"
        );
    }

    #[test]
    fn empty_configs_produces_minimal_toml() {
        let toml = generate_pricing_toml(&[], false);
        assert!(toml.contains("[default]"));
        assert!(toml.contains("[tee]"));
    }

    #[test]
    fn vram_h200_separate_from_h100() {
        assert_eq!(estimate_vram_mib("gpu_1x_h200"), 144384);
        assert_eq!(estimate_vram_mib("H200_SXM"), 144384);
        // H100 is still 80 GB
        assert_eq!(estimate_vram_mib("gpu_1x_h100_pcie"), 81920);
    }

    #[test]
    fn pricing_with_18_decimal_token_doesnt_overflow() {
        let cap = OperatorCapability {
            hourly_cost_usd: 100.0,
            ..a100_cap()
        };
        let pricing = derive_pricing_with_throughput(
            &cap,
            10.0,
            &ThroughputEstimate {
                tokens_per_hour: 1.0, // extreme: 1 token per hour
                images_per_hour: 1.0,
                token_decimals: 18, // ETH-like
            },
        );
        // Should not panic, values should be capped at u64::MAX or saturated
        assert!(pricing.price_per_input_token > 0);
    }
}
