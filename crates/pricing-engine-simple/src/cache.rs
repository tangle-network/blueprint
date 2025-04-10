// src/cache.rs
use crate::error::{PricingError, Result};
use crate::pricing::PriceModel;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Using String for blueprint hash for simplicity, could be [u8; 32]
pub type BlueprintHash = String;

#[derive(Clone)]
pub struct PriceCache {
    cache: Arc<Mutex<HashMap<BlueprintHash, PriceModel>>>,
}

impl PriceCache {
    /// Creates a new in-memory price cache.
    pub fn new<P: AsRef<Path>>(_path: P) -> Result<Self> {
        Ok(PriceCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Stores a price model for a given blueprint hash.
    /// Overwrites existing entries.
    pub fn store_price(
        &self,
        blueprint_hash: &BlueprintHash,
        price_model: &PriceModel,
    ) -> Result<()> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        cache.insert(blueprint_hash.clone(), price_model.clone());
        Ok(())
    }

    /// Retrieves a price model for a given blueprint hash.
    /// Returns `Ok(None)` if the blueprint hash is not found.
    pub fn get_price(&self, blueprint_hash: &BlueprintHash) -> Result<Option<PriceModel>> {
        let cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        Ok(cache.get(blueprint_hash).cloned())
    }

    /// Removes a price model for a given blueprint hash.
    pub fn remove_price(&self, blueprint_hash: &BlueprintHash) -> Result<Option<PriceModel>> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PricingError::Cache(format!("Lock error: {}", e)))?;
        Ok(cache.remove(blueprint_hash))
    }
}
