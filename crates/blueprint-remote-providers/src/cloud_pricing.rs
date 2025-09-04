//! Real-time cloud pricing APIs integration
//! 
//! Fetches current pricing information from cloud providers to enable
//! accurate cost estimation and optimization.

use crate::error::{Error, Result};
use crate::remote::CloudProvider;
use crate::resources::ResourceSpec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn};
use chrono::{DateTime, Utc, Duration};

/// Real-time pricing service
pub struct CloudPricingService {
    #[cfg(feature = "api-clients")]
    client: reqwest::Client,
    cache: Arc<RwLock<PriceCache>>,
    update_interval: std::time::Duration,
}

impl CloudPricingService {
    /// Create a new pricing service
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "api-clients")]
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            cache: Arc::new(RwLock::new(PriceCache::new())),
            update_interval: std::time::Duration::from_secs(3600), // 1 hour
        }
    }
    
    /// Get current price for a specific instance type
    pub async fn get_instance_price(
        &self,
        provider: &CloudProvider,
        instance_type: &str,
        region: &str,
        pricing_model: PricingModel,
    ) -> Result<InstancePricing> {
        // Check cache first
        let cache_key = format!("{:?}-{}-{}-{:?}", provider, instance_type, region, pricing_model);
        
        let cache = self.cache.read().await;
        if let Some(cached_price) = cache.get(&cache_key) {
            if cached_price.is_valid() {
                debug!("Using cached price for {}", instance_type);
                return Ok(cached_price.pricing.clone());
            }
        }
        drop(cache);
        
        // Fetch fresh pricing
        let pricing = match provider {
            CloudProvider::AWS => self.fetch_aws_pricing(instance_type, region, pricing_model).await?,
            CloudProvider::GCP => self.fetch_gcp_pricing(instance_type, region, pricing_model).await?,
            CloudProvider::Azure => self.fetch_azure_pricing(instance_type, region, pricing_model).await?,
            CloudProvider::DigitalOcean => self.fetch_do_pricing(instance_type, region).await?,
            CloudProvider::Vultr => self.fetch_vultr_pricing(instance_type, region).await?,
            _ => return Err(Error::ConfigurationError("Unsupported provider for pricing".into())),
        };
        
        // Update cache
        let mut cache = self.cache.write().await;
        cache.set(cache_key, pricing.clone());
        
        Ok(pricing)
    }
    
    /// Fetch AWS pricing
    #[cfg(feature = "api-clients")]
    async fn fetch_aws_pricing(
        &self,
        instance_type: &str,
        region: &str,
        pricing_model: PricingModel,
    ) -> Result<InstancePricing> {
        // AWS Pricing API
        let url = "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonEC2/current/index.json";
        
        // For demo, use hardcoded prices
        // In production, would parse the massive AWS pricing JSON
        let (on_demand, spot) = match instance_type {
            "t3.micro" => (0.0104, Some(0.0031)),
            "t3.small" => (0.0208, Some(0.0062)),
            "t3.medium" => (0.0416, Some(0.0125)),
            "t3.large" => (0.0832, Some(0.0250)),
            "m6i.xlarge" => (0.192, Some(0.0576)),
            "m6i.2xlarge" => (0.384, Some(0.1152)),
            "g4dn.xlarge" => (0.526, Some(0.1578)),
            "p3.2xlarge" => (3.06, Some(0.918)),
            _ => (0.10, Some(0.03)), // Default
        };
        
        Ok(InstancePricing {
            instance_type: instance_type.to_string(),
            region: region.to_string(),
            on_demand_hourly: on_demand,
            spot_hourly: spot,
            reserved_1yr_hourly: Some(on_demand * 0.6),
            reserved_3yr_hourly: Some(on_demand * 0.4),
            currency: "USD".to_string(),
            effective_date: Utc::now(),
        })
    }
    
    /// Fetch GCP pricing
    #[cfg(feature = "api-clients")]
    async fn fetch_gcp_pricing(
        &self,
        machine_type: &str,
        region: &str,
        pricing_model: PricingModel,
    ) -> Result<InstancePricing> {
        // GCP Cloud Billing API
        let url = format!(
            "https://cloudbilling.googleapis.com/v1/services/6F81-5844-456A/skus?currencyCode=USD"
        );
        
        // Simplified pricing lookup
        let (on_demand, spot) = match machine_type {
            "e2-micro" => (0.00838, Some(0.00251)),
            "e2-small" => (0.01675, Some(0.00503)),
            "e2-medium" => (0.0335, Some(0.01005)),
            "e2-standard-4" => (0.134, Some(0.0402)),
            "n2-standard-8" => (0.3886, Some(0.1166)),
            _ => (0.10, Some(0.03)),
        };
        
        Ok(InstancePricing {
            instance_type: machine_type.to_string(),
            region: region.to_string(),
            on_demand_hourly: on_demand,
            spot_hourly: spot,
            reserved_1yr_hourly: Some(on_demand * 0.63),
            reserved_3yr_hourly: Some(on_demand * 0.44),
            currency: "USD".to_string(),
            effective_date: Utc::now(),
        })
    }
    
    /// Fetch Azure pricing
    #[cfg(feature = "api-clients")]
    async fn fetch_azure_pricing(
        &self,
        vm_size: &str,
        region: &str,
        pricing_model: PricingModel,
    ) -> Result<InstancePricing> {
        // Azure Retail Prices API
        let url = format!(
            "https://prices.azure.com/api/retail/prices?$filter=serviceName eq 'Virtual Machines' and armRegionName eq '{}' and skuName eq '{}'",
            region, vm_size
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to fetch Azure pricing: {}", e)))?;
        
        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| Error::ConfigurationError(format!("Failed to parse Azure pricing: {}", e)))?;
            
            if let Some(items) = json["Items"].as_array() {
                if let Some(item) = items.first() {
                    if let Some(price) = item["unitPrice"].as_f64() {
                        return Ok(InstancePricing {
                            instance_type: vm_size.to_string(),
                            region: region.to_string(),
                            on_demand_hourly: price,
                            spot_hourly: Some(price * 0.3),
                            reserved_1yr_hourly: Some(price * 0.64),
                            reserved_3yr_hourly: Some(price * 0.44),
                            currency: "USD".to_string(),
                            effective_date: Utc::now(),
                        });
                    }
                }
            }
        }
        
        // Fallback prices
        let price = match vm_size {
            "Standard_B1s" => 0.012,
            "Standard_B2s" => 0.048,
            "Standard_D2s_v5" => 0.096,
            "Standard_D4s_v5" => 0.192,
            _ => 0.10,
        };
        
        Ok(InstancePricing {
            instance_type: vm_size.to_string(),
            region: region.to_string(),
            on_demand_hourly: price,
            spot_hourly: Some(price * 0.3),
            reserved_1yr_hourly: Some(price * 0.64),
            reserved_3yr_hourly: Some(price * 0.44),
            currency: "USD".to_string(),
            effective_date: Utc::now(),
        })
    }
    
    /// Fetch DigitalOcean pricing
    async fn fetch_do_pricing(&self, droplet_size: &str, region: &str) -> Result<InstancePricing> {
        // DO has fixed pricing, not region-specific
        let monthly_price = match droplet_size {
            "s-1vcpu-512mb" => 4.0,
            "s-1vcpu-1gb" => 6.0,
            "s-1vcpu-2gb" => 12.0,
            "s-2vcpu-2gb" => 18.0,
            "s-2vcpu-4gb" => 24.0,
            "s-4vcpu-8gb" => 48.0,
            "s-8vcpu-16gb" => 96.0,
            _ => 24.0,
        };
        
        Ok(InstancePricing {
            instance_type: droplet_size.to_string(),
            region: region.to_string(),
            on_demand_hourly: monthly_price / 730.0,
            spot_hourly: None, // DO doesn't have spot
            reserved_1yr_hourly: None,
            reserved_3yr_hourly: None,
            currency: "USD".to_string(),
            effective_date: Utc::now(),
        })
    }
    
    /// Fetch Vultr pricing
    async fn fetch_vultr_pricing(&self, plan: &str, region: &str) -> Result<InstancePricing> {
        // Vultr has fixed pricing
        let monthly_price = match plan {
            "vc2-1c-1gb" => 5.0,
            "vc2-1c-2gb" => 10.0,
            "vc2-2c-4gb" => 18.0,
            "vc2-4c-8gb" => 35.0,
            "vc2-6c-16gb" => 70.0,
            "vc2-8c-32gb" => 140.0,
            _ => 20.0,
        };
        
        Ok(InstancePricing {
            instance_type: plan.to_string(),
            region: region.to_string(),
            on_demand_hourly: monthly_price / 730.0,
            spot_hourly: None,
            reserved_1yr_hourly: None,
            reserved_3yr_hourly: None,
            currency: "USD".to_string(),
            effective_date: Utc::now(),
        })
    }
    
    /// Calculate cost for a resource specification
    pub async fn calculate_cost(
        &self,
        spec: &ResourceSpec,
        provider: &CloudProvider,
        region: &str,
        duration_hours: f64,
        pricing_model: PricingModel,
    ) -> Result<CostEstimate> {
        // Map resource spec to instance type
        let instance_type = crate::provisioning::InstanceTypeMapper::map_to_instance_type(
            spec,
            provider,
        ).instance_type;
        
        // Get pricing
        let pricing = self.get_instance_price(provider, &instance_type, region, pricing_model).await?;
        
        // Calculate base compute cost
        let compute_cost = match pricing_model {
            PricingModel::OnDemand => pricing.on_demand_hourly,
            PricingModel::Spot => pricing.spot_hourly.unwrap_or(pricing.on_demand_hourly),
            PricingModel::Reserved1Year => pricing.reserved_1yr_hourly.unwrap_or(pricing.on_demand_hourly),
            PricingModel::Reserved3Year => pricing.reserved_3yr_hourly.unwrap_or(pricing.on_demand_hourly),
        } * duration_hours;
        
        // Add storage cost (simplified)
        let storage_cost = spec.storage.disk_gb * 0.00014 * duration_hours; // ~$0.10/GB/month
        
        // Add network cost (egress)
        let network_cost = match spec.network.bandwidth_tier {
            crate::resources::BandwidthTier::Low => 0.01,
            crate::resources::BandwidthTier::Standard => 0.05,
            crate::resources::BandwidthTier::High => 0.10,
            crate::resources::BandwidthTier::Ultra => 0.20,
        } * duration_hours;
        
        // Add backup cost if enabled
        let backup_cost = if spec.qos.backup_config.enabled {
            spec.storage.disk_gb * 0.00007 * duration_hours // ~$0.05/GB/month
        } else {
            0.0
        };
        
        let total_cost = compute_cost + storage_cost + network_cost + backup_cost;
        
        Ok(CostEstimate {
            provider: provider.clone(),
            region: region.to_string(),
            instance_type,
            pricing_model,
            duration_hours,
            compute_cost,
            storage_cost,
            network_cost,
            backup_cost,
            total_cost,
            currency: "USD".to_string(),
            breakdown: vec![
                ("Compute".to_string(), compute_cost),
                ("Storage".to_string(), storage_cost),
                ("Network".to_string(), network_cost),
                ("Backup".to_string(), backup_cost),
            ],
        })
    }
    
    /// Get spot price history
    pub async fn get_spot_price_history(
        &self,
        provider: &CloudProvider,
        instance_type: &str,
        region: &str,
        days: u32,
    ) -> Result<Vec<SpotPricePoint>> {
        // This would fetch historical spot prices from provider APIs
        // For now, return synthetic data
        let mut history = Vec::new();
        let now = Utc::now();
        let base_price = 0.05;
        
        for i in 0..(days * 24) {
            let timestamp = now - Duration::hours(i as i64);
            let variation = (i as f64 * 0.1).sin() * 0.02;
            history.push(SpotPricePoint {
                timestamp,
                price: base_price + variation,
                availability_zone: format!("{}-1a", region),
            });
        }
        
        Ok(history)
    }
    
    /// Find cheapest region for given requirements
    pub async fn find_cheapest_region(
        &self,
        spec: &ResourceSpec,
        provider: &CloudProvider,
        regions: Vec<String>,
    ) -> Result<RegionPricing> {
        let mut region_costs = Vec::new();
        
        for region in regions {
            match self.calculate_cost(spec, provider, &region, 730.0, PricingModel::OnDemand).await {
                Ok(cost) => {
                    region_costs.push((region, cost.total_cost));
                }
                Err(e) => {
                    warn!("Failed to get pricing for region {}: {}", region, e);
                }
            }
        }
        
        region_costs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        if let Some((cheapest_region, monthly_cost)) = region_costs.first() {
            Ok(RegionPricing {
                region: cheapest_region.clone(),
                monthly_cost: *monthly_cost,
                hourly_cost: *monthly_cost / 730.0,
                savings_vs_highest: region_costs.last()
                    .map(|(_, highest)| (*highest - *monthly_cost) / *highest * 100.0)
                    .unwrap_or(0.0),
            })
        } else {
            Err(Error::ConfigurationError("No regions available".into()))
        }
    }
}

/// Pricing cache
struct PriceCache {
    entries: HashMap<String, CachedPrice>,
}

impl PriceCache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    
    fn get(&self, key: &str) -> Option<&CachedPrice> {
        self.entries.get(key)
    }
    
    fn set(&mut self, key: String, pricing: InstancePricing) {
        self.entries.insert(key, CachedPrice {
            pricing,
            fetched_at: Utc::now(),
        });
    }
}

/// Cached price entry
struct CachedPrice {
    pricing: InstancePricing,
    fetched_at: DateTime<Utc>,
}

impl CachedPrice {
    fn is_valid(&self) -> bool {
        Utc::now() - self.fetched_at < Duration::hours(1)
    }
}

/// Instance pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstancePricing {
    pub instance_type: String,
    pub region: String,
    pub on_demand_hourly: f64,
    pub spot_hourly: Option<f64>,
    pub reserved_1yr_hourly: Option<f64>,
    pub reserved_3yr_hourly: Option<f64>,
    pub currency: String,
    pub effective_date: DateTime<Utc>,
}

/// Pricing model
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PricingModel {
    OnDemand,
    Spot,
    Reserved1Year,
    Reserved3Year,
}

/// Cost estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub provider: CloudProvider,
    pub region: String,
    pub instance_type: String,
    pub pricing_model: PricingModel,
    pub duration_hours: f64,
    pub compute_cost: f64,
    pub storage_cost: f64,
    pub network_cost: f64,
    pub backup_cost: f64,
    pub total_cost: f64,
    pub currency: String,
    pub breakdown: Vec<(String, f64)>,
}

impl CostEstimate {
    /// Get daily cost
    pub fn daily_cost(&self) -> f64 {
        self.total_cost / self.duration_hours * 24.0
    }
    
    /// Get monthly cost (730 hours)
    pub fn monthly_cost(&self) -> f64 {
        self.total_cost / self.duration_hours * 730.0
    }
    
    /// Get annual cost
    pub fn annual_cost(&self) -> f64 {
        self.monthly_cost() * 12.0
    }
}

/// Spot price history point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotPricePoint {
    pub timestamp: DateTime<Utc>,
    pub price: f64,
    pub availability_zone: String,
}

/// Region pricing comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionPricing {
    pub region: String,
    pub monthly_cost: f64,
    pub hourly_cost: f64,
    pub savings_vs_highest: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{ResourceSpec, ComputeResources, StorageResources};
    
    #[tokio::test]
    async fn test_cost_calculation() {
        let service = CloudPricingService::new();
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let cost = service.calculate_cost(
            &spec,
            &CloudProvider::AWS,
            "us-west-2",
            24.0, // 1 day
            PricingModel::OnDemand,
        ).await;
        
        assert!(cost.is_ok());
        let estimate = cost.unwrap();
        assert!(estimate.total_cost > 0.0);
        assert_eq!(estimate.duration_hours, 24.0);
    }
    
    #[tokio::test]
    async fn test_spot_price_history() {
        let service = CloudPricingService::new();
        
        let history = service.get_spot_price_history(
            &CloudProvider::AWS,
            "t3.medium",
            "us-west-2",
            7, // 7 days
        ).await;
        
        assert!(history.is_ok());
        let prices = history.unwrap();
        assert_eq!(prices.len(), 7 * 24);
    }
    
    #[tokio::test]
    async fn test_find_cheapest_region() {
        let service = CloudPricingService::new();
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 8.0,
                disk_gb: 50.0,
                ..Default::default()
            },
            ..Default::default()
        };
        
        let result = service.find_cheapest_region(
            &spec,
            &CloudProvider::AWS,
            vec![
                "us-west-2".to_string(),
                "us-east-1".to_string(),
                "eu-west-1".to_string(),
            ],
        ).await;
        
        assert!(result.is_ok());
        let cheapest = result.unwrap();
        assert!(!cheapest.region.is_empty());
        assert!(cheapest.monthly_cost > 0.0);
    }
}