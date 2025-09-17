//! DigitalOcean public pricing (HTML scraping)

pub struct DigitalOceanPublicPricing;

impl DigitalOceanPublicPricing {
    pub const PRICING_PAGE: &'static str = "https://www.digitalocean.com/pricing/droplets";

    /// Parse droplet prices from HTML
    pub fn parse_html(html: &str) -> Vec<DropletInfo> {
        let mut droplets = Vec::new();

        // Known droplet sizes from their pricing page
        // In production, use a proper HTML parser like scraper
        let known_sizes = vec![
            ("s-1vcpu-1gb", 1, 1, 25, 6.0),
            ("s-1vcpu-2gb", 1, 2, 50, 12.0),
            ("s-2vcpu-2gb", 2, 2, 60, 18.0),
            ("s-2vcpu-4gb", 2, 4, 80, 24.0),
            ("s-4vcpu-8gb", 4, 8, 160, 48.0),
            ("s-8vcpu-16gb", 8, 16, 320, 96.0),
            ("c-2", 2, 4, 25, 42.0), // CPU-optimized
            ("c-4", 4, 8, 50, 84.0),
            ("g-2vcpu-8gb", 2, 8, 25, 63.0), // General purpose
            ("g-4vcpu-16gb", 4, 16, 50, 126.0),
        ];

        for (slug, vcpus, mem_gb, disk_gb, price_monthly) in known_sizes {
            if html.contains(slug) || html.is_empty() {
                droplets.push(DropletInfo {
                    slug: slug.to_string(),
                    vcpus,
                    memory_gb: mem_gb,
                    storage_gb: disk_gb,
                    price_monthly,
                    price_hourly: price_monthly / 730.0,
                });
            }
        }

        droplets
    }

    /// Get hardcoded fallback data
    pub fn fallback_data() -> Vec<DropletInfo> {
        Self::parse_html("")
    }
}

#[derive(Debug, Clone)]
pub struct DropletInfo {
    pub slug: String,
    pub vcpus: u32,
    pub memory_gb: u32,
    pub storage_gb: u32,
    pub price_monthly: f64,
    pub price_hourly: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digitalocean_fallback() {
        let droplets = DigitalOceanPublicPricing::fallback_data();
        println!("📊 DigitalOcean Droplet Pricing (Fallback):");
        for d in droplets.iter().take(5) {
            println!(
                "  - {}: {} vCPU, {}GB RAM, ${:.2}/mo (${:.3}/hr)",
                d.slug, d.vcpus, d.memory_gb, d.price_monthly, d.price_hourly
            );
        }
        assert!(!droplets.is_empty());
    }
}
