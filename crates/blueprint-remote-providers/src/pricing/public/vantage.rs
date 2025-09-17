//! Vantage.sh aggregated pricing data (best public source)

use crate::core::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Vantage.sh aggregates pricing from AWS and Azure in clean JSON format
/// Note: GCP is NOT available on Vantage
pub struct VantageAggregator;

impl VantageAggregator {
    pub const AWS_URL: &'static str = "https://instances.vantage.sh/aws/instances.json";
    pub const AZURE_URL: &'static str = "https://instances.vantage.sh/azure/instances.json";
    // GCP not available on Vantage - use GCP pricing calculator instead

    pub async fn fetch_aws() -> Result<Vec<VantageInstance>> {
        Self::fetch_json(Self::AWS_URL).await
    }

    pub async fn fetch_azure() -> Result<Vec<VantageInstance>> {
        Self::fetch_json(Self::AZURE_URL).await
    }

    async fn fetch_json(url: &str) -> Result<Vec<VantageInstance>> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| Error::SerializationError(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VantageInstance {
    pub instance_type: String,
    pub name: Option<String>,
    pub vcpus: Option<f32>,
    pub memory_gib: Option<f32>,
    pub storage_gb: Option<f32>,
    pub gpu_count: Option<u32>,
    pub gpu_memory_gb: Option<f32>,
    pub price_hourly: Option<f64>,
    pub price_monthly: Option<f64>,
    pub region: Option<String>,
    pub availability_zone: Option<String>,
    pub on_demand_price: Option<f64>,
    pub spot_price: Option<f64>,
}

