//! AWS public pricing sources

use serde::Deserialize;

pub struct AwsPublicPricing;

impl AwsPublicPricing {
    /// EC2Instances.info - Community maintained
    pub const EC2_INSTANCES_URL: &'static str = "https://ec2instances.info/instances.json";

    /// AWS official pricing (huge file, >1GB)
    pub const OFFICIAL_URL: &'static str =
        "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonEC2/current/index.json";

    /// Vantage is better for AWS data
    pub fn use_vantage_instead() -> &'static str {
        "https://instances.vantage.sh/aws/instances.json"
    }

    #[cfg(feature = "api-clients")]
    pub async fn fetch_ec2_instances() -> Result<Vec<Ec2Instance>> {
        let client = reqwest::Client::new();
        let response = client
            .get(Self::EC2_INSTANCES_URL)
            .send()
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| Error::SerializationError(e.to_string()))
    }
}

#[derive(Debug, Deserialize)]
pub struct Ec2Instance {
    pub instance_type: String,
    pub vcpu: u32,
    pub memory: f32,
    pub storage: Option<String>,
    pub network_performance: Option<String>,
    pub pricing: Option<Ec2Pricing>,
}

#[derive(Debug, Deserialize)]
pub struct Ec2Pricing {
    #[serde(rename = "us-east-1")]
    pub us_east_1: Option<RegionPricing>,
}

#[derive(Debug, Deserialize)]
pub struct RegionPricing {
    pub on_demand: Option<f64>,
    pub spot: Option<SpotPricing>,
}

#[derive(Debug, Deserialize)]
pub struct SpotPricing {
    pub min: Option<f64>,
    pub max: Option<f64>,
}
