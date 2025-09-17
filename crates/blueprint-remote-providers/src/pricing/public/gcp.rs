//! GCP public pricing sources

pub struct GcpPublicPricing;

impl GcpPublicPricing {
    /// GCP Cloud Pricing Calculator data
    pub const CALCULATOR_URL: &'static str =
        "https://cloudpricingcalculator.appspot.com/static/data/pricelist.json";

    /// Note: Vantage does NOT have GCP data - only AWS and Azure

    /// Machine types by family
    pub fn get_common_machine_types() -> Vec<GcpMachineType> {
        vec![
            // E2 Series (cost-optimized)
            GcpMachineType {
                name: "e2-micro".to_string(),
                vcpus: 0.25,
                memory_gb: 1.0,
                price_hourly_us: 0.00838,
            },
            GcpMachineType {
                name: "e2-small".to_string(),
                vcpus: 0.5,
                memory_gb: 2.0,
                price_hourly_us: 0.01675,
            },
            GcpMachineType {
                name: "e2-medium".to_string(),
                vcpus: 1.0,
                memory_gb: 4.0,
                price_hourly_us: 0.03351,
            },
            // N2 Series (balanced)
            GcpMachineType {
                name: "n2-standard-2".to_string(),
                vcpus: 2.0,
                memory_gb: 8.0,
                price_hourly_us: 0.0971,
            },
            GcpMachineType {
                name: "n2-standard-4".to_string(),
                vcpus: 4.0,
                memory_gb: 16.0,
                price_hourly_us: 0.1942,
            },
            // C2 Series (compute-optimized)
            GcpMachineType {
                name: "c2-standard-4".to_string(),
                vcpus: 4.0,
                memory_gb: 16.0,
                price_hourly_us: 0.2088,
            },
        ]
    }

    #[cfg(feature = "api-clients")]
    pub async fn fetch_from_calculator() -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let response = client
            .get(Self::CALCULATOR_URL)
            .send()
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| Error::SerializationError(e.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct GcpMachineType {
    pub name: String,
    pub vcpus: f32,
    pub memory_gb: f32,
    pub price_hourly_us: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcp_machine_types() {
        let machines = GcpPublicPricing::get_common_machine_types();
        println!("📊 GCP Machine Types:");
        for m in &machines {
            println!(
                "  - {}: {} vCPUs, {:.1}GB RAM, ${:.4}/hr",
                m.name, m.vcpus, m.memory_gb, m.price_hourly_us
            );
        }
        assert!(!machines.is_empty());
    }
}
