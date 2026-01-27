//! Vultr public pricing (HTML scraping)

pub struct VultrPublicPricing;

impl VultrPublicPricing {
    pub const PRICING_PAGE: &'static str = "https://www.vultr.com/pricing/";

    /// Get hardcoded Vultr plans (from their public pricing page)
    pub fn get_plans() -> Vec<VultrPlan> {
        vec![
            // Regular Cloud Compute
            VultrPlan {
                id: "vc2-1c-1gb".to_string(),
                vcpus: 1,
                memory_gb: 1,
                storage_gb: 25,
                bandwidth_gb: 1000,
                price_monthly: 5.0,
                price_hourly: 0.007,
            },
            VultrPlan {
                id: "vc2-1c-2gb".to_string(),
                vcpus: 1,
                memory_gb: 2,
                storage_gb: 55,
                bandwidth_gb: 2000,
                price_monthly: 10.0,
                price_hourly: 0.015,
            },
            VultrPlan {
                id: "vc2-2c-4gb".to_string(),
                vcpus: 2,
                memory_gb: 4,
                storage_gb: 80,
                bandwidth_gb: 3000,
                price_monthly: 20.0,
                price_hourly: 0.030,
            },
            VultrPlan {
                id: "vc2-4c-8gb".to_string(),
                vcpus: 4,
                memory_gb: 8,
                storage_gb: 160,
                bandwidth_gb: 4000,
                price_monthly: 40.0,
                price_hourly: 0.060,
            },
            // High Frequency
            VultrPlan {
                id: "vhf-1c-1gb".to_string(),
                vcpus: 1,
                memory_gb: 1,
                storage_gb: 32,
                bandwidth_gb: 1000,
                price_monthly: 6.0,
                price_hourly: 0.009,
            },
            VultrPlan {
                id: "vhf-2c-4gb".to_string(),
                vcpus: 2,
                memory_gb: 4,
                storage_gb: 128,
                bandwidth_gb: 3000,
                price_monthly: 24.0,
                price_hourly: 0.036,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct VultrPlan {
    pub id: String,
    pub vcpus: u32,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub bandwidth_gb: u32,
    pub price_monthly: f64,
    pub price_hourly: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vultr_plans() {
        let plans = VultrPublicPricing::get_plans();
        println!("📊 Vultr Plans:");
        for p in plans.iter().take(5) {
            println!(
                "  - {}: {} vCPU, {}GB RAM, ${:.2}/mo (${:.3}/hr)",
                p.id, p.vcpus, p.memory_gb, p.price_monthly, p.price_hourly
            );
        }
        assert!(!plans.is_empty());
    }
}
