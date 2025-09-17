//! Azure public pricing API (official, no auth required)

use serde::Deserialize;

pub struct AzurePublicPricing;

impl AzurePublicPricing {
    pub const API_URL: &'static str = "https://prices.azure.com/api/retail/prices";

    #[cfg(feature = "api-clients")]
    pub async fn fetch_vm_prices(region: &str, top: u32) -> Result<AzurePricingResponse> {
        let url = format!(
            "{}?$filter=serviceName eq 'Virtual Machines' and armRegionName eq '{}'&$top={}",
            Self::API_URL,
            region,
            top
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::HttpError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| Error::SerializationError(e.to_string()))
    }

    pub fn parse_vm_specs(product_name: &str) -> (u32, f32) {
        // Parse vCPUs and memory from names like "D2 v3", "B2s"
        let vcpus = if product_name.contains("D2") || product_name.contains("B2") {
            2
        } else if product_name.contains("D4") || product_name.contains("B4") {
            4
        } else if product_name.contains("D8") {
            8
        } else if product_name.contains("D16") {
            16
        } else {
            1
        };

        let memory_gb = vcpus as f32 * 4.0; // Rough estimate
        (vcpus, memory_gb)
    }
}

#[derive(Debug, Deserialize)]
pub struct AzurePricingResponse {
    #[serde(rename = "Items")]
    pub items: Vec<AzurePriceItem>,
    #[serde(rename = "NextPageLink")]
    pub next_page_link: Option<String>,
    #[serde(rename = "Count")]
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct AzurePriceItem {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "retailPrice")]
    pub retail_price: f64,
    #[serde(rename = "armSkuName")]
    pub arm_sku_name: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "meterName")]
    pub meter_name: String,
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: String,
    #[serde(rename = "location")]
    pub location: String,
}

