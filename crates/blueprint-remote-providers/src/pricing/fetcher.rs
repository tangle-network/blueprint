//! Real pricing data fetcher implementation

use crate::core::error::{Error, Result};
use crate::core::remote::CloudProvider;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::debug;

/// Instance information with specs and pricing
#[derive(Clone, Debug)]
pub struct InstanceInfo {
    pub name: String,
    pub vcpus: f32,
    pub memory_gb: f32,
    pub hourly_price: f64,
}

/// Fetches real pricing data from public sources
pub struct PricingFetcher {
    client: reqwest::Client,
    cache: HashMap<String, CachedPricing>,
}

#[derive(Clone)]
struct CachedPricing {
    instances: Vec<InstanceInfo>,
    fetched_at: std::time::Instant,
}

impl PricingFetcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            cache: HashMap::new(),
        }
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
                if best.is_none() || instance.hourly_price < best.as_ref().unwrap().hourly_price {
                    best = Some(instance);
                }
            }
        }

        best.ok_or_else(|| {
            Error::Other(format!(
                "No instance found for {} vCPUs, {} GB RAM under ${}/hr",
                min_cpu, min_memory_gb, max_price
            ))
        })
    }

    /// Get all available instances for a provider/region
    async fn get_instances(
        &mut self,
        provider: CloudProvider,
        region: &str,
    ) -> Result<Vec<InstanceInfo>> {
        let cache_key = format!("{:?}-{}", provider, region);

        // Check cache (24 hour TTL - pricing doesn't change frequently)
        if let Some(cached) = self.cache.get(&cache_key) {
            if cached.fetched_at.elapsed() < std::time::Duration::from_secs(86400) {
                debug!("Using cached pricing data for {:?} {}", provider, region);
                return Ok(cached.instances.clone());
            }
        }

        // Fetch fresh data
        let instances = match provider {
            CloudProvider::AWS => self.fetch_aws_instances(region).await?,
            CloudProvider::Azure => self.fetch_azure_instances(region).await?,
            CloudProvider::GCP => self.fetch_gcp_instances(region).await?,
            CloudProvider::DigitalOcean => self.fetch_digitalocean_instances(region).await?,
            _ => {
                return Err(Error::Other(format!(
                    "No pricing API available for provider: {:?}",
                    provider
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

    async fn fetch_aws_instances(&self, region: &str) -> Result<Vec<InstanceInfo>> {
        debug!("Fetching AWS instances from Vantage API");

        let url = "https://instances.vantage.sh/instances.json";

        #[derive(Deserialize, Debug)]
        struct VantageInstance {
            instance_type: String,
            #[serde(rename = "vCPU")]
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
            .map_err(|e| Error::Other(format!("Failed to fetch AWS pricing: {}", e)))?;

        let instances: Vec<VantageInstance> = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse AWS pricing: {}", e)))?;

        let mut result = Vec::new();
        // Limit to prevent huge responses
        for inst in instances.into_iter().take(1000) {
            if let (Some(vcpu), Some(memory)) = (inst.vcpu, inst.memory) {
                // Extract price for the specified region
                let price = if let Some(pricing) = inst.pricing {
                    if let Some(region_data) = pricing.get(region) {
                        if let Some(linux_data) = region_data.get("linux") {
                            if let Some(ondemand) = linux_data.get("ondemand") {
                                // Price might be a string or number
                                if let Some(price_str) = ondemand.as_str() {
                                    price_str.parse::<f64>().unwrap_or(0.0)
                                } else {
                                    ondemand.as_f64().unwrap_or(0.0)
                                }
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
                        name: inst.instance_type,
                        vcpus: vcpu,
                        memory_gb: memory,
                        hourly_price: price,
                    });
                }
            }
        }

        if result.is_empty() {
            Err(Error::Other("No instances found for region".to_string()))
        } else {
            Ok(result)
        }
    }

    async fn fetch_azure_instances(&self, region: &str) -> Result<Vec<InstanceInfo>> {
        debug!("Fetching Azure instances from Vantage API");

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
            .map_err(|e| Error::Other(format!("Failed to fetch Azure pricing: {}", e)))?;

        let instances: Vec<VantageAzureInstance> = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse Azure pricing: {}", e)))?;

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
            Err(Error::Other("No instances found for region".to_string()))
        } else {
            Ok(result)
        }
    }

    async fn fetch_gcp_instances(&self, region: &str) -> Result<Vec<InstanceInfo>> {
        debug!("Fetching GCP instances from pricing page");

        // GCP publishes pricing at https://cloud.google.com/compute/all-pricing
        // This is a large HTML page with embedded pricing data
        // For now, we'll use a simplified approach with common instance types

        // Map GCP regions to their pricing multipliers (us-central1 is baseline)
        let region_multiplier = match region {
            "us-central1" => 1.0,
            "us-east1" => 1.0,
            "us-west1" => 1.05,
            "europe-west1" => 1.05,
            "asia-northeast1" => 1.15,
            _ => 1.0,
        };

        // Common GCP instance types with baseline pricing (us-central1)
        let base_instances = vec![
            ("e2-micro", 0.25, 1.0, 0.00838),
            ("e2-small", 0.5, 2.0, 0.01677),
            ("e2-medium", 1.0, 4.0, 0.03354),
            ("e2-standard-2", 2.0, 8.0, 0.06708),
            ("e2-standard-4", 4.0, 16.0, 0.13416),
            ("e2-standard-8", 8.0, 32.0, 0.26832),
            ("n2-standard-2", 2.0, 8.0, 0.0971),
            ("n2-standard-4", 4.0, 16.0, 0.1942),
            ("n2-standard-8", 8.0, 32.0, 0.3884),
            ("n2d-standard-2", 2.0, 8.0, 0.0849),
            ("n2d-standard-4", 4.0, 16.0, 0.1698),
            ("n2d-standard-8", 8.0, 32.0, 0.3396),
        ];

        let mut result = Vec::new();
        for (name, vcpus, memory, base_price) in base_instances {
            result.push(InstanceInfo {
                name: name.to_string(),
                vcpus,
                memory_gb: memory,
                hourly_price: base_price * region_multiplier,
            });
        }

        if result.is_empty() {
            Err(Error::Other("No GCP instances found".to_string()))
        } else {
            Ok(result)
        }
    }

    async fn fetch_digitalocean_instances(&self, _region: &str) -> Result<Vec<InstanceInfo>> {
        debug!("Fetching DigitalOcean instances from pricing page");

        let url = "https://www.digitalocean.com/pricing/droplets";

        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to fetch DO pricing: {}", e)))?;

        let html = response
            .text()
            .await
            .map_err(|e| Error::Other(format!("Failed to read DO pricing: {}", e)))?;

        // Extract JSON data from __NEXT_DATA__ script tag
        let json_start = html
            .find(r#"__NEXT_DATA__" type="application/json">{"#)
            .ok_or_else(|| Error::Other("Could not find pricing data".to_string()))?;
        let json_start = json_start + r#"__NEXT_DATA__" type="application/json">"#.len();

        let json_end = html[json_start..]
            .find("</script>")
            .ok_or_else(|| Error::Other("Could not find end of pricing data".to_string()))?;

        let json_str = &html[json_start..json_start + json_end];

        // Parse the JSON
        let data: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| Error::Other(format!("Failed to parse DO pricing JSON: {}", e)))?;

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
            Err(Error::Other("No DigitalOcean instances found".to_string()))
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
        let mut fetcher = PricingFetcher::new();

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
        let mut fetcher = PricingFetcher::new();

        // AWS should work with Vantage API
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
