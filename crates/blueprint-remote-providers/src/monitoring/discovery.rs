//! Machine selection from fresh provider-owned VM rates.

use crate::{CloudProvider, Error, Result};
use blueprint_pricing_engine_lib::PricingFetcher;
use blueprint_std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct MachineTypeDiscovery {
    quotes: HashMap<CloudProvider, Vec<MachineType>>,
}

impl MachineTypeDiscovery {
    pub fn new() -> Self {
        Self::default()
    }

    /// Refresh this provider's quote snapshot using caller-configured credentials.
    /// A failure clears the old snapshot and is returned to the caller.
    pub async fn discover_machine_types(
        &mut self,
        provider: &CloudProvider,
        region: &str,
        pricing: &PricingFetcher,
    ) -> Result<Vec<MachineType>> {
        self.quotes.remove(provider);
        let machines: Vec<_> = pricing
            .get_instances(provider.clone(), region)
            .await
            .map_err(|error| Error::ConfigurationError(error.to_string()))?
            .into_iter()
            .map(|instance| MachineType {
                name: instance.name,
                provider: provider.clone(),
                vcpus: instance.vcpus as u32,
                memory_gb: instance.memory_gb.into(),
                storage_gb: None,
                gpu_count: 0,
                gpu_type: None,
                network_performance: "Unknown".into(),
                hourly_price: Some(instance.hourly_price),
                spot_price: None,
            })
            .collect();
        self.quotes.insert(provider.clone(), machines.clone());
        Ok(machines)
    }

    /// Select from the last successful refresh. Missing rates are unselectable.
    pub fn find_best_match(
        &self,
        provider: &CloudProvider,
        min_vcpus: u32,
        min_memory_gb: f64,
        needs_gpu: bool,
        max_price_per_hour: Option<f64>,
    ) -> Option<MachineType> {
        if min_vcpus == 0
            || min_memory_gb <= 0.0
            || !min_memory_gb.is_finite()
            || max_price_per_hour.is_some_and(|cap| !cap.is_finite() || cap <= 0.0)
        {
            return None;
        }
        self.quotes
            .get(provider)?
            .iter()
            .filter(|machine| machine.vcpus >= min_vcpus && machine.memory_gb >= min_memory_gb)
            .filter(|machine| !needs_gpu || machine.gpu_count > 0)
            .filter(|machine| {
                machine.hourly_price.is_some_and(|price| {
                    price.is_finite()
                        && price > 0.0
                        && max_price_per_hour.is_none_or(|cap| price <= cap)
                })
            })
            .min_by(|left, right| {
                left.hourly_price
                    .unwrap()
                    .total_cmp(&right.hourly_price.unwrap())
            })
            .cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineType {
    pub name: String,
    pub provider: CloudProvider,
    pub vcpus: u32,
    pub memory_gb: f64,
    pub storage_gb: Option<f64>,
    pub gpu_count: u32,
    pub gpu_type: Option<String>,
    pub network_performance: String,
    pub hourly_price: Option<f64>,
    pub spot_price: Option<f64>,
}
