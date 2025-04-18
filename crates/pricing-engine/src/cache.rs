use crate::error::{PricingError, Result};
use crate::pricing::PriceModel;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Using u64 for blueprint ID
pub type BlueprintId = u64;

#[derive(Clone)]
pub struct PriceCache {
    cache: Arc<Mutex<HashMap<BlueprintId, PriceModel>>>,
}

impl PriceCache {
    /// Creates a new in-memory price cache.
    pub fn new<P: AsRef<Path>>(_path: P) -> Result<Self> {
        Ok(PriceCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Stores a price model for a given blueprint ID.
    /// Overwrites existing entries.
    pub fn store_price(&self, blueprint_id: BlueprintId, price_model: &PriceModel) -> Result<()> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        cache.insert(blueprint_id, price_model.clone());
        Ok(())
    }

    /// Retrieves a price model for a given blueprint ID.
    /// Returns `Ok(None)` if the blueprint ID is not found.
    pub fn get_price(&self, blueprint_id: BlueprintId) -> Result<Option<PriceModel>> {
        let cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        Ok(cache.get(&blueprint_id).cloned())
    }

    /// Removes a price model for a given blueprint ID.
    pub fn remove_price(&self, blueprint_id: BlueprintId) -> Result<Option<PriceModel>> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        Ok(cache.remove(&blueprint_id))
    }
}
