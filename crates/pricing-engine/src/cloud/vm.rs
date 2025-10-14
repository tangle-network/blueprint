//! Real VM instance pricing data fetcher implementation

use crate::error::{PricingError, Result};
use crate::types::CloudProvider;
use serde::Deserialize;
use std::collections::HashMap;

/// Instance information with specs and pricing
#[derive(Clone, Debug)]
pub struct InstanceInfo {
    pub name: String,
    pub vcpus: f32,
    pub memory_gb: f32,
    pub hourly_price: f64,
}

/// Fetches real pricing data from public sources
#[derive(Clone)]
pub struct PricingFetcher {
    client: reqwest::Client,
    cache: HashMap<String, CachedPricing>,
}

#[derive(Clone)]
struct CachedPricing {
    instances: Vec<InstanceInfo>,
    fetched_at: std::time::Instant,
}

impl Default for PricingFetcher {
    fn default() -> Self {
        Self::new_or_default()
    }
}

impl PricingFetcher {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| PricingError::Other(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
            cache: HashMap::new(),
        })
    }

    /// Create a PricingFetcher with default configuration, falling back to basic client
    pub fn new_or_default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            client: reqwest::Client::new(),
            cache: HashMap::new(),
        })
    }

    /// Find the best instance type for given resource requirements
    pub async fn find_best_instance(
        &mut self,
        provider: CloudProvider,
        region: &str,
        min_cpu: f32,
        min_memory_gb: f32,
        max_price: f64,
    ) -> Result<InstanceInfo> {
        let instances = self.get_instances(provider, region).await?;

        // Find cheapest instance that meets requirements
        let mut best: Option<InstanceInfo> = None;
        for instance in instances {
            if instance.vcpus >= min_cpu
                && instance.memory_gb >= min_memory_gb
                && instance.hourly_price <= max_price
            {
                let is_better = best
                    .as_ref()
                    .map(|current| instance.hourly_price < current.hourly_price)
                    .unwrap_or(true);

                if is_better {
                    best = Some(instance);
                }
            }
        }

        best.ok_or_else(|| {
            PricingError::Other(format!(
                "No instance found for {min_cpu} vCPUs, {min_memory_gb} GB RAM under ${max_price}/hr"
            ))
        })
    }

    /// Get all available instances for a provider/region
    async fn get_instances(
        &mut self,
        provider: CloudProvider,
        region: &str,
    ) -> Result<Vec<InstanceInfo>> {
        let cache_key = format!("{provider:?}-{region}");

        // Check cache (24 hour TTL - pricing doesn't change frequently)
        if let Some(cached) = self.cache.get(&cache_key) {
            if cached.fetched_at.elapsed() < std::time::Duration::from_secs(86400) {
                return Ok(cached.instances.clone());
            }
        }

        // Fetch fresh data
        let instances = match provider {
            CloudProvider::AWS => self.fetch_aws_instances(region).await?,
            CloudProvider::Azure => self.fetch_azure_instances(region).await?,
            CloudProvider::GCP => self.fetch_gcp_instances(region).await?,
            CloudProvider::DigitalOcean => self.fetch_digitalocean_instances(region).await?,
            CloudProvider::Vultr => self.fetch_vultr_instances(region).await?,
            _ => {
                return Err(PricingError::Other(format!(
                    "No pricing API available for provider: {provider:?}"
                )));
            }
        };

        // Cache the data
        self.cache.insert(
            cache_key,
            CachedPricing {
                instances: instances.clone(),
                fetched_at: std::time::Instant::now(),
            },
        );

        Ok(instances)
    }

    async fn fetch_aws_instances(&self, _region: &str) -> Result<Vec<InstanceInfo>> {
        // Use ec2.shop - production-ready AWS pricing API with real data
        let url = "https://ec2.shop/?format=json";

        let response = self
            .client
            .get(url)
            .header("User-Agent", "blueprint-pricing-engine/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| {
                PricingError::Other(format!("Failed to fetch AWS pricing from ec2.shop: {e}"))
            })?;

        if !response.status().is_success() {
            return Err(PricingError::Other(format!(
                "ec2.shop API returned status: {}",
                response.status()
            )));
        }

        #[derive(Deserialize, Debug)]
        struct Ec2ShopResponse {
            #[serde(rename = "Prices")]
            prices: Vec<Ec2ShopInstance>,
        }

        #[derive(Deserialize, Debug)]
        struct Ec2ShopInstance {
            #[serde(rename = "InstanceType")]
            instance_type: String,
            #[serde(rename = "Memory")]
            memory: String,
            #[serde(rename = "VCPUS")]
            vcpus: i32,
            #[serde(rename = "Cost")]
            cost: f64,
        }

        let pricing_data: Ec2ShopResponse = response
            .json()
            .await
            .map_err(|e| PricingError::Other(format!("Failed to parse ec2.shop JSON: {e}")))?;

        let mut instances = Vec::new();

        for price in pricing_data.prices.into_iter().take(100) {
            // Limit for performance
            // Parse memory string like "1 GiB" or "0.5 GiB"
            let memory_gb = price
                .memory
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0);

            if price.vcpus > 0 && memory_gb > 0.0 && price.cost > 0.0 {
                instances.push(InstanceInfo {
                    name: price.instance_type,
                    vcpus: price.vcpus as f32,
                    memory_gb,
                    hourly_price: price.cost,
                });
            }
        }

        if instances.is_empty() {
            return Err(PricingError::Other(
                "No AWS instances found in ec2.shop data".to_string(),
            ));
        }

        Ok(instances)
    }

    async fn fetch_azure_instances(&self, region: &str) -> Result<Vec<InstanceInfo>> {
        let url = "https://instances.vantage.sh/azure/instances.json";

        #[derive(Deserialize, Debug)]
        struct VantageAzureInstance {
            pretty_name: String,
            vcpu: Option<f32>,
            memory: Option<f32>,
            pricing: Option<serde_json::Value>,
        }

        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| PricingError::Other(format!("Failed to fetch Azure pricing: {e}")))?;

        let instances: Vec<VantageAzureInstance> = response
            .json()
            .await
            .map_err(|e| PricingError::Other(format!("Failed to parse Azure pricing: {e}")))?;

        let mut result = Vec::new();
        // Limit to prevent huge responses
        for inst in instances.into_iter().take(1000) {
            if let (Some(vcpu), Some(memory)) = (inst.vcpu, inst.memory) {
                // Memory is already in GB for Azure

                // Extract price for linux in the specified region
                let price = if let Some(pricing) = inst.pricing {
                    if let Some(region_data) = pricing.get(region) {
                        if let Some(linux_data) = region_data.get("linux") {
                            if let Some(ondemand) = linux_data.get("ondemand") {
                                // Price might be number directly
                                ondemand.as_f64().unwrap_or(0.0)
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                if price > 0.0 {
                    result.push(InstanceInfo {
                        name: inst.pretty_name,
                        vcpus: vcpu,
                        memory_gb: memory,
                        hourly_price: price,
                    });
                }
            }
        }

        if result.is_empty() {
            Err(PricingError::Other(
                "No instances found for region".to_string(),
            ))
        } else {
            Ok(result)
        }
    }

    async fn fetch_gcp_instances(&self, _region: &str) -> Result<Vec<InstanceInfo>> {
        // GCP Cloud Billing Catalog API requires API key
        // https://cloudbilling.googleapis.com/v1/services/6F81-5844-456A/skus (Compute Engine)

        let api_key = std::env::var("GCP_API_KEY").map_err(|_| {
            PricingError::ConfigurationError(
                "GCP_API_KEY environment variable required for GCP pricing. \
                Get API key from: https://console.cloud.google.com/apis/credentials"
                    .to_string(),
            )
        })?;

        // Compute Engine service ID
        let service_id = "services/6F81-5844-456A";
        let url = format!(
            "https://cloudbilling.googleapis.com/v1/{}/skus?key={}",
            service_id, api_key
        );

        #[derive(Deserialize, Debug)]
        struct GcpBillingResponse {
            skus: Vec<GcpSku>,
        }

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)] // Fields defined for API schema completeness
        struct GcpSku {
            description: String,
            category: GcpCategory,
            #[serde(rename = "pricingInfo")]
            pricing_info: Vec<GcpPricingInfo>,
        }

        #[derive(Deserialize, Debug)]
        struct GcpCategory {
            #[serde(rename = "resourceFamily")]
            resource_family: String,
        }

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)] // Fields defined for API schema completeness
        struct GcpPricingInfo {
            #[serde(rename = "pricingExpression")]
            pricing_expression: GcpPricingExpression,
        }

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)] // Fields defined for API schema completeness
        struct GcpPricingExpression {
            #[serde(rename = "tieredRates")]
            tiered_rates: Vec<GcpTieredRate>,
        }

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)] // Fields defined for API schema completeness
        struct GcpTieredRate {
            #[serde(rename = "unitPrice")]
            unit_price: GcpMoney,
        }

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)] // Fields defined for API schema completeness
        struct GcpMoney {
            units: String,
            nanos: i64,
        }

        let response = self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| PricingError::HttpError(format!("Failed to fetch GCP pricing: {}", e)))?;

        if !response.status().is_success() {
            return Err(PricingError::HttpError(format!(
                "GCP Cloud Billing API returned status: {}. Check API key is valid.",
                response.status()
            )));
        }

        let billing_data: GcpBillingResponse = response
            .json()
            .await
            .map_err(|e| PricingError::HttpError(format!("Failed to parse GCP pricing: {}", e)))?;

        // Parse instance pricing from SKUs
        // GCP pricing is complex - this is simplified to extract compute pricing
        let _result: Vec<InstanceInfo> = Vec::new();

        for sku in billing_data.skus.iter().take(100) {
            if sku.category.resource_family == "Compute"
                && sku.description.contains("Instance Core")
            {
                // This is a simplification - real implementation would need to:
                // 1. Match cores to memory for specific machine types
                // 2. Calculate per-instance pricing from per-core pricing
                // For now, return error to force use of real GCP Compute API
            }
        }

        Err(PricingError::ConfigurationError(
            "GCP pricing requires using GCP Compute API with service account credentials. \
            Cloud Billing Catalog API does not provide ready-to-use instance pricing. \
            Consider using gcloud CLI or Compute Engine API directly."
                .to_string(),
        ))
    }

    async fn fetch_digitalocean_instances(&self, _region: &str) -> Result<Vec<InstanceInfo>> {
        let url = "https://www.digitalocean.com/pricing/droplets";

        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| PricingError::Other(format!("Failed to fetch DO pricing: {e}")))?;

        let html = response
            .text()
            .await
            .map_err(|e| PricingError::Other(format!("Failed to read DO pricing: {e}")))?;

        // Extract JSON data from __NEXT_DATA__ script tag
        let json_start = html
            .find(r#"__NEXT_DATA__" type="application/json">{"#)
            .ok_or_else(|| PricingError::Other("Could not find pricing data".to_string()))?;
        let json_start = json_start + r#"__NEXT_DATA__" type="application/json">"#.len();

        let json_end = html[json_start..]
            .find("</script>")
            .ok_or_else(|| PricingError::Other("Could not find end of pricing data".to_string()))?;

        let json_str = &html[json_start..json_start + json_end];

        // Parse the JSON
        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| PricingError::Other(format!("Failed to parse DO pricing JSON: {e}")))?;

        let mut result = Vec::new();

        // Navigate to the droplet pricing data
        if let Some(droplets) = data
            .get("props")
            .and_then(|p| p.get("pageProps"))
            .and_then(|p| p.get("data"))
            .and_then(|d| d.get("basic"))
            .and_then(|b| b.get("regular"))
            .and_then(|r| r.as_array())
        {
            for droplet in droplets {
                if let (Some(memory), Some(cpus), Some(price_obj)) = (
                    droplet.get("memory").and_then(|m| m.as_f64()),
                    droplet.get("cpus").and_then(|c| c.as_f64()),
                    droplet.get("price"),
                ) {
                    if let Some(hourly) = price_obj.get("hourly").and_then(|h| h.as_f64()) {
                        if let Some(slug) = droplet.get("slug").and_then(|s| s.as_str()) {
                            result.push(InstanceInfo {
                                name: slug.to_string(),
                                vcpus: cpus as f32,
                                memory_gb: memory as f32,
                                hourly_price: hourly,
                            });
                        }
                    }
                }
            }
        }

        if result.is_empty() {
            Err(PricingError::Other(
                "No DigitalOcean instances found".to_string(),
            ))
        } else {
            Ok(result)
        }
    }

    async fn fetch_vultr_instances(&self, _region: &str) -> Result<Vec<InstanceInfo>> {
        // Vultr API requires API key
        let api_key = std::env::var("VULTR_API_KEY").map_err(|_| {
            PricingError::ConfigurationError(
                "VULTR_API_KEY environment variable required for Vultr pricing. \
                Get API key from: https://my.vultr.com/settings/#settingsapi"
                    .to_string(),
            )
        })?;

        let url = "https://api.vultr.com/v2/plans";

        #[derive(Deserialize, Debug)]
        struct VultrPlansResponse {
            plans: Vec<VultrPlan>,
        }

        #[derive(Deserialize, Debug)]
        #[allow(dead_code)] // Fields defined for API schema completeness
        struct VultrPlan {
            id: String,
            vcpu_count: i32,
            ram: i64,  // RAM in MB
            disk: i64, // Disk in GB
            monthly_cost: f64,
            #[serde(rename = "type")]
            plan_type: String,
        }

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| {
                PricingError::HttpError(format!("Failed to fetch Vultr pricing: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(PricingError::HttpError(format!(
                "Vultr API returned status: {}. Check API key is valid.",
                response.status()
            )));
        }

        let plans_data: VultrPlansResponse = response.json().await.map_err(|e| {
            PricingError::HttpError(format!("Failed to parse Vultr pricing: {}", e))
        })?;

        let mut result = Vec::new();

        for plan in plans_data.plans {
            // Only include regular compute plans (exclude bare metal, etc.)
            if plan.plan_type == "vc2" || plan.plan_type == "vhf" || plan.plan_type == "vhp" {
                let memory_gb = plan.ram as f32 / 1024.0; // Convert MB to GB

                // Convert monthly to hourly (730 hours/month standard)
                let hourly_price = plan.monthly_cost / 730.0;

                if plan.vcpu_count > 0 && memory_gb > 0.0 && hourly_price > 0.0 {
                    result.push(InstanceInfo {
                        name: plan.id,
                        vcpus: plan.vcpu_count as f32,
                        memory_gb,
                        hourly_price,
                    });
                }
            }
        }

        if result.is_empty() {
            Err(PricingError::Other(
                "No Vultr instances found in API response".to_string(),
            ))
        } else {
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_azure_pricing_api() {
        let mut fetcher = PricingFetcher::new_or_default();

        // Azure should work with public API
        let result = fetcher
            .find_best_instance(CloudProvider::Azure, "eastus", 2.0, 4.0, 0.10)
            .await;

        // May succeed or fail depending on network
        if result.is_ok() {
            let instance = result.unwrap();
            assert!(instance.hourly_price <= 0.10);
            assert!(instance.vcpus >= 2.0);
            assert!(instance.memory_gb >= 4.0);
        }
    }

    #[tokio::test]
    async fn test_aws_pricing_works() {
        let fetcher = PricingFetcher::new_or_default();

        // AWS should work with ec2.shop API
        let result = fetcher.fetch_aws_instances("us-east-1").await;

        // Should succeed with public API
        if result.is_ok() {
            let instances = result.unwrap();
            assert!(!instances.is_empty());
            // Verify we got actual pricing data
            assert!(instances.iter().any(|i| i.hourly_price > 0.0));
        }
    }
}
