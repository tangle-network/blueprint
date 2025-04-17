// src/benchmark_cache.rs
use crate::benchmark::BenchmarkProfile;
use crate::error::{PricingError, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Using u64 for blueprint ID
pub type BlueprintId = u64;

#[derive(Clone)]
pub struct BenchmarkCache {
    cache: Arc<Mutex<HashMap<BlueprintId, BenchmarkProfile>>>,
}

impl BenchmarkCache {
    /// Creates a new in-memory benchmark profile cache.
    pub fn new<P: AsRef<Path>>(_path: P) -> Result<Self> {
        Ok(BenchmarkCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Stores a benchmark profile for a given blueprint ID.
    /// Overwrites existing entries.
    pub fn store_profile(&self, blueprint_id: BlueprintId, profile: &BenchmarkProfile) -> Result<()> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        cache.insert(blueprint_id, profile.clone());
        Ok(())
    }

    /// Retrieves a benchmark profile for a given blueprint ID.
    /// Returns `Ok(None)` if the blueprint ID is not found.
    pub fn get_profile(&self, blueprint_id: BlueprintId) -> Result<Option<BenchmarkProfile>> {
        let cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        Ok(cache.get(&blueprint_id).cloned())
    }

    /// Removes a benchmark profile for a given blueprint ID.
    pub fn remove_profile(&self, blueprint_id: BlueprintId) -> Result<Option<BenchmarkProfile>> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        Ok(cache.remove(&blueprint_id))
    }
}
