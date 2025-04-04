// src/cache.rs
use crate::error::{PricingError, Result};
use crate::pricing::PriceModel;
use log::info;
use sled::{Db, IVec};
use std::path::Path;
use std::sync::Arc;

// Using String for blueprint hash for simplicity, could be [u8; 32]
pub type BlueprintHash = String;

#[derive(Clone)]
pub struct PriceCache {
    db: Arc<Db>,
}

impl PriceCache {
    /// Opens or creates a new database at the specified path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)?;
        info!("Price cache opened at: {:?}", db.path());
        Ok(PriceCache { db: Arc::new(db) })
    }

    /// Stores a price model for a given blueprint hash.
    /// Overwrites existing entries.
    pub fn store_price(
        &self,
        blueprint_hash: &BlueprintHash,
        price_model: &PriceModel,
    ) -> Result<()> {
        let key = blueprint_hash.as_bytes();
        let value = bincode::serialize(price_model)?;
        self.db.insert(key, value)?;
        self.db.flush()?; // Ensure data is written to disk
        info!("Stored price for blueprint: {}", blueprint_hash);
        Ok(())
    }

    /// Retrieves a price model for a given blueprint hash.
    /// Returns `Ok(None)` if the blueprint hash is not found.
    pub fn get_price(&self, blueprint_hash: &BlueprintHash) -> Result<Option<PriceModel>> {
        let key = blueprint_hash.as_bytes();
        match self.db.get(key)? {
            Some(ivec) => {
                let price_model: PriceModel = bincode::deserialize(&ivec)?;
                info!("Retrieved price for blueprint: {}", blueprint_hash);
                Ok(Some(price_model))
            }
            None => {
                info!("Price not found in cache for blueprint: {}", blueprint_hash);
                Ok(None)
            }
        }
    }

    /// Removes a price model for a given blueprint hash.
    pub fn remove_price(&self, blueprint_hash: &BlueprintHash) -> Result<Option<PriceModel>> {
        let key = blueprint_hash.as_bytes();
        match self.db.remove(key)? {
            Some(ivec) => {
                let price_model: PriceModel = bincode::deserialize(&ivec)?;
                info!("Removed price for blueprint: {}", blueprint_hash);
                Ok(Some(price_model))
            }
            None => {
                info!(
                    "Price to remove not found for blueprint: {}",
                    blueprint_hash
                );
                Ok(None)
            }
        }
    }
}
