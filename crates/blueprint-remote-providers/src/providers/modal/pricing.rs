//! Bind opaque workspace billing identifiers to requested native resources.

use super::{ModalAdapter, wire::Rates};
use crate::providers::common::{
    InstanceSelection,
    gpu_adapter::{catalog_class_for_sku, quoted_gpu_selection},
};
use crate::{Error, ResourceSpec, Result};
use std::collections::HashMap;

/// API identifiers, never prices. Bind these from the account's live rates response.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModalRateKeys {
    pub cpu_per_core_hour: String,
    pub memory_per_gib_hour: String,
    pub gpu_per_device_hour: HashMap<String, String>,
}

impl ModalAdapter {
    pub fn with_rate_keys(mut self, keys: ModalRateKeys) -> Self {
        self.rate_keys = Some(keys);
        self
    }

    pub(super) fn select_rate(
        &self,
        spec: &ResourceSpec,
        cpu: u32,
        memory: u32,
        candidates: Vec<String>,
        rates: Rates,
    ) -> Result<InstanceSelection> {
        let keys = self.rate_keys.as_ref().ok_or_else(|| {
            Error::ConfigurationError(
                "Modal workspace billing resource identifiers are not configured".into(),
            )
        })?;
        let read = |key: &str| -> Option<f64> {
            if key.is_empty() || rates.warnings.contains_key(key) || rates.errors.contains_key(key)
            {
                return None;
            }
            rates
                .rates
                .get(key)?
                .parse::<f64>()
                .ok()
                .filter(|rate| rate.is_finite() && *rate >= 0.0)
        };
        let cpu_rate = read(&keys.cpu_per_core_hour).ok_or_else(|| {
            Error::ConfigurationError("Modal CPU price is unknown or deprecated".into())
        })?;
        let memory_rate = read(&keys.memory_per_gib_hour).ok_or_else(|| {
            Error::ConfigurationError("Modal memory price is unknown or deprecated".into())
        })?;
        let mut priced: Vec<_> = candidates
            .into_iter()
            .filter_map(|sku| {
                let gpu_rate = read(keys.gpu_per_device_hour.get(&sku)?)?;
                if gpu_rate <= 0.0 {
                    return None;
                }
                let cost = gpu_rate * f64::from(spec.gpu_count?)
                    + cpu_rate * f64::from(cpu) / 1000.0
                    + memory_rate * f64::from(memory) / 1024.0;
                let class = catalog_class_for_sku(spec, "modal", &sku).ok()?;
                quoted_gpu_selection(sku, cost, class).ok()
            })
            .collect();
        priced.sort_by(|left, right| {
            left.estimated_hourly_cost
                .unwrap()
                .total_cmp(&right.estimated_hourly_cost.unwrap())
        });
        priced.into_iter().next().ok_or_else(|| {
            Error::ConfigurationError("Modal GPU price is unknown or deprecated".into())
        })
    }
}
