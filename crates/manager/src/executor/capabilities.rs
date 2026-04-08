//! Operator capabilities matrix — derives what an operator CAN provision
//! from their configured cloud providers and the blueprint's requirements.
//!
//! Used during pre-registration to auto-generate registration payloads and
//! pricing configs without the operator manually declaring hardware specs.

use blueprint_remote_providers::core::resources::ResourceSpec;
use blueprint_remote_providers::CloudProvider;
use blueprint_remote_providers::providers::common::InstanceSelection;
use crate::config::BlueprintManagerContext;
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

    // Build a ResourceSpec from the blueprint's GPU requirements
    let spec = ResourceSpec {
        cpu: 8.0,
        memory_gb: (min_vram_gb as f32).max(32.0),
        storage_gb: 100.0,
        gpu_count: if gpu_count > 0 { Some(gpu_count) } else { None },
        allow_spot: false,
        qos: Default::default(),
    };

    // For each configured provider, map to an instance type
    let providers_with_regions = configured_providers(config);

    for (provider, _region) in &providers_with_regions {
        let selection = map_provider_instance(provider, &spec);
        let tee = supports_tee(provider);

        // Estimate VRAM from GPU count (rough: A100=80GB, H100=80GB, T4=16GB)
        let vram_per_gpu = estimate_vram_mib(&selection.instance_type);
        let total_vram = vram_per_gpu * gpu_count.max(1);

        capabilities.push(OperatorCapability {
            provider: provider.clone(),
            instance_type: selection.instance_type.clone(),
            gpu_count: gpu_count.max(if selection.instance_type.contains("gpu") { 1 } else { 0 }),
            estimated_vram_mib: total_vram,
            tee_capable: tee,
            hourly_cost_usd: selection.estimated_hourly_cost.unwrap_or(0.0),
            spot_capable: selection.spot_capable,
        });

        // If TEE is supported, also add a TEE variant
        if tee && gpu_count > 0 {
            capabilities.push(OperatorCapability {
                provider: provider.clone(),
                instance_type: selection.instance_type.clone(),
                gpu_count: gpu_count.max(1),
                estimated_vram_mib: total_vram,
                tee_capable: true,
                hourly_cost_usd: selection.estimated_hourly_cost.unwrap_or(0.0) * 1.3,
                spot_capable: false,
            });
        }
    }

    // Sort by cost ascending
    capabilities.sort_by(|a, b| a.hourly_cost_usd.partial_cmp(&b.hourly_cost_usd).unwrap());
    capabilities
}

/// Derive pricing from a capability with a margin.
///
/// Converts hourly infrastructure cost to per-unit pricing using rough
/// throughput estimates for each service type.
pub fn derive_pricing(cap: &OperatorCapability, margin: f64) -> DerivedPricing {
    let hourly = cap.hourly_cost_usd * margin;

    // Rough throughput estimates per GPU-hour for pricing derivation:
    // - LLM: ~1M tokens/hour on A100 (mixed input/output)
    // - Image: ~500 images/hour on A100
    // - Video/Avatar: 3600 compute-seconds/hour (1:1 mapping)
    let tokens_per_hour = 1_000_000.0;
    let images_per_hour = 500.0;

    // Convert to base units (6 decimal stablecoin, 1 USD = 1_000_000 base units)
    let usd_to_base = 1_000_000.0;

    let cost_per_token = (hourly / tokens_per_hour) * usd_to_base;
    let price_per_input_token = (cost_per_token * 0.4).ceil() as u64; // 40% weight for input
    let price_per_output_token = (cost_per_token * 1.0).ceil() as u64; // 100% weight for output (output is more expensive)
    let price_per_compute_second = ((hourly / 3600.0) * usd_to_base) as u64;
    let price_per_image = ((hourly / images_per_hour) * usd_to_base) as u64;

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
                cap.gpu_count, cap.hourly_cost_usd * primary.margin / cap.gpu_count as f64
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
            cap.provider, cap.instance_type, cap.hourly_cost_usd, (primary.margin - 1.0) * 100.0
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

fn supports_tee(provider: &CloudProvider) -> bool {
    matches!(
        provider,
        CloudProvider::AWS | CloudProvider::GCP | CloudProvider::Azure | CloudProvider::CoreWeave
    )
}

fn configured_providers(
    config: &blueprint_remote_providers::config::CloudConfig,
) -> Vec<(CloudProvider, String)> {
    let mut providers = Vec::new();
    if let Some(c) = &config.lambda_labs { if c.enabled { providers.push((CloudProvider::LambdaLabs, c.region.clone())); } }
    if let Some(c) = &config.runpod { if c.enabled { providers.push((CloudProvider::RunPod, c.region.clone())); } }
    if let Some(c) = &config.vast_ai { if c.enabled { providers.push((CloudProvider::VastAi, "global".into())); } }
    if let Some(c) = &config.coreweave { if c.enabled { providers.push((CloudProvider::CoreWeave, c.region.clone())); } }
    if let Some(c) = &config.paperspace { if c.enabled { providers.push((CloudProvider::Paperspace, c.region.clone())); } }
    if let Some(c) = &config.fluidstack { if c.enabled { providers.push((CloudProvider::Fluidstack, c.region.clone())); } }
    if let Some(c) = &config.tensordock { if c.enabled { providers.push((CloudProvider::TensorDock, c.region.clone())); } }
    if let Some(c) = &config.akash { if c.enabled { providers.push((CloudProvider::Akash, "global".into())); } }
    if let Some(c) = &config.io_net { if c.enabled { providers.push((CloudProvider::IoNet, c.region.clone())); } }
    if let Some(c) = &config.prime_intellect { if c.enabled { providers.push((CloudProvider::PrimeIntellect, c.region.clone())); } }
    if let Some(c) = &config.aws { if c.enabled { providers.push((CloudProvider::AWS, c.region.clone())); } }
    if let Some(c) = &config.gcp { if c.enabled { providers.push((CloudProvider::GCP, c.region.clone())); } }
    if let Some(c) = &config.azure { if c.enabled { providers.push((CloudProvider::Azure, c.region.clone())); } }
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
    if lower.contains("h100") || lower.contains("h200") { 81920 }
    else if lower.contains("a100") && lower.contains("80") { 81920 }
    else if lower.contains("a100") { 40960 }
    else if lower.contains("a6000") || lower.contains("rtx6000") { 49152 }
    else if lower.contains("a40") { 49152 }
    else if lower.contains("4090") { 24576 }
    else if lower.contains("3090") { 24576 }
    else if lower.contains("a10") { 24576 }
    else if lower.contains("t4") { 16384 }
    else if lower.contains("4080") { 16384 }
    else { 16384 } // default to T4 class
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimate_vram_h100() {
        assert_eq!(estimate_vram_mib("gpu_1x_h100_pcie"), 81920);
        assert_eq!(estimate_vram_mib("H100_NVLINK_80GB"), 81920);
    }

    #[test]
    fn estimate_vram_a100() {
        assert_eq!(estimate_vram_mib("gpu_1x_a100"), 40960);
        assert_eq!(estimate_vram_mib("a100_80gb_pcie"), 81920);
    }

    #[test]
    fn estimate_vram_consumer() {
        assert_eq!(estimate_vram_mib("NVIDIA GeForce RTX 4090"), 24576);
        assert_eq!(estimate_vram_mib("rtx3090-24gb"), 24576);
    }

    #[test]
    fn derive_pricing_produces_nonzero_values() {
        let cap = OperatorCapability {
            provider: CloudProvider::LambdaLabs,
            instance_type: "gpu_1x_a100".into(),
            gpu_count: 1,
            estimated_vram_mib: 40960,
            tee_capable: false,
            hourly_cost_usd: 1.29,
            spot_capable: false,
        };
        let pricing = derive_pricing(&cap, 1.3);
        assert!(pricing.price_per_input_token > 0);
        assert!(pricing.price_per_output_token > 0);
        assert!(pricing.price_per_compute_second > 0);
        assert!(pricing.price_per_image > 0);
        assert!(pricing.price_per_output_token > pricing.price_per_input_token);
    }

    #[test]
    fn derive_pricing_margin_increases_prices() {
        let cap = OperatorCapability {
            provider: CloudProvider::RunPod,
            instance_type: "A100".into(),
            gpu_count: 1,
            estimated_vram_mib: 81920,
            tee_capable: false,
            hourly_cost_usd: 10.0, // high enough that 1.0x vs 2.0x produces different integers after ceil
            spot_capable: true,
        };
        let low = derive_pricing(&cap, 1.0);
        let high = derive_pricing(&cap, 2.0);
        assert!(high.price_per_input_token > low.price_per_input_token);
    }

    #[test]
    fn tee_providers_are_correct() {
        assert!(supports_tee(&CloudProvider::AWS));
        assert!(supports_tee(&CloudProvider::GCP));
        assert!(supports_tee(&CloudProvider::Azure));
        assert!(supports_tee(&CloudProvider::CoreWeave));
        assert!(!supports_tee(&CloudProvider::RunPod));
        assert!(!supports_tee(&CloudProvider::LambdaLabs));
        assert!(!supports_tee(&CloudProvider::VastAi));
    }

    #[test]
    fn generate_pricing_toml_includes_tee() {
        let cap = OperatorCapability {
            provider: CloudProvider::AWS,
            instance_type: "p4d.24xlarge".into(),
            gpu_count: 8,
            estimated_vram_mib: 655360,
            tee_capable: true,
            hourly_cost_usd: 32.0,
            spot_capable: false,
        };
        let pricing = derive_pricing(&cap, 1.5);
        let toml = generate_pricing_toml(&[pricing], true);
        assert!(toml.contains("[tee]"));
        assert!(toml.contains("available = true"));
        assert!(toml.contains("GPU"));
    }
}
