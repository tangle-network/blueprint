//! In-memory registry for tracking outstanding price quotes.
//!
//! When the pricing engine issues a `SignedJobQuote`, or when the x402 gateway
//! dynamically prices a request, the quote is stored here keyed by its EIP-712 digest.
//! On payment, the gateway looks up the quote to verify the amount matches.

use alloy_primitives::U256;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A tracked quote entry.
#[derive(Debug, Clone)]
pub struct QuoteEntry {
    /// The service instance ID.
    pub service_id: u64,
    /// The job type index.
    pub job_index: u32,
    /// Price in wei (native denomination).
    pub price_wei: U256,
    /// When the quote was created.
    pub created_at: Instant,
    /// When the quote expires.
    pub expires_at: Instant,
    /// Whether this quote has been consumed (paid).
    pub consumed: bool,
}

/// Thread-safe registry mapping quote digest → quote details.
///
/// Uses [`DashMap`] for lock-free concurrent reads.
#[derive(Debug, Clone)]
pub struct QuoteRegistry {
    entries: Arc<DashMap<[u8; 32], QuoteEntry>>,
    default_ttl: Duration,
}

impl QuoteRegistry {
    /// Create a new registry with the given default quote TTL.
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            entries: Arc::new(DashMap::new()),
            default_ttl,
        }
    }

    /// Register a new quote. Returns the digest key.
    ///
    /// The `digest` should be the EIP-712 hash of the quote details, which
    /// serves as a unique, deterministic identifier.
    pub fn insert(&self, digest: [u8; 32], entry: QuoteEntry) {
        self.entries.insert(digest, entry);
    }

    /// Register a quote from job parameters, computing a simple digest.
    ///
    /// For quotes generated dynamically (not via the RFQ gRPC endpoint),
    /// we derive a digest from the job parameters + timestamp.
    pub fn insert_dynamic(&self, service_id: u64, job_index: u32, price_wei: U256) -> [u8; 32] {
        let now = Instant::now();
        let entry = QuoteEntry {
            service_id,
            job_index,
            price_wei,
            created_at: now,
            expires_at: now + self.default_ttl,
            consumed: false,
        };

        // Build a deterministic key from the parameters + wall-clock timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let mut hasher_input = Vec::with_capacity(64);
        hasher_input.extend_from_slice(&service_id.to_be_bytes());
        hasher_input.extend_from_slice(&job_index.to_be_bytes());
        hasher_input.extend_from_slice(&price_wei.to_be_bytes::<32>());
        hasher_input.extend_from_slice(&timestamp.to_be_bytes());

        let digest: [u8; 32] = alloy_primitives::keccak256(&hasher_input).into();
        self.entries.insert(digest, entry);
        digest
    }

    /// Look up a quote by its digest. Returns `None` if not found or expired.
    pub fn get(&self, digest: &[u8; 32]) -> Option<QuoteEntry> {
        let entry = self.entries.get(digest)?;
        if entry.expires_at < Instant::now() || entry.consumed {
            return None;
        }
        Some(entry.clone())
    }

    /// Mark a quote as consumed (paid). Returns the entry if successful.
    pub fn consume(&self, digest: &[u8; 32]) -> Option<QuoteEntry> {
        let mut entry = self.entries.get_mut(digest)?;
        if entry.expires_at < Instant::now() || entry.consumed {
            return None;
        }
        entry.consumed = true;
        Some(entry.clone())
    }

    /// Remove expired and consumed entries. Call periodically.
    pub fn gc(&self) {
        let now = Instant::now();
        self.entries
            .retain(|_, entry| !entry.consumed && entry.expires_at > now);
    }

    /// Number of active (non-expired, non-consumed) entries.
    pub fn active_count(&self) -> usize {
        let now = Instant::now();
        self.entries
            .iter()
            .filter(|e| !e.consumed && e.expires_at > now)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let registry = QuoteRegistry::new(Duration::from_secs(60));
        let digest = [1u8; 32];
        let entry = QuoteEntry {
            service_id: 42,
            job_index: 0,
            price_wei: U256::from(1_000_000u64),
            created_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(60),
            consumed: false,
        };

        registry.insert(digest, entry.clone());
        let got = registry.get(&digest).unwrap();
        assert_eq!(got.service_id, 42);
        assert_eq!(got.price_wei, U256::from(1_000_000u64));
    }

    #[test]
    fn test_consume_prevents_double_spend() {
        let registry = QuoteRegistry::new(Duration::from_secs(60));
        let digest = [2u8; 32];
        let entry = QuoteEntry {
            service_id: 1,
            job_index: 0,
            price_wei: U256::from(100u64),
            created_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(60),
            consumed: false,
        };

        registry.insert(digest, entry);
        assert!(registry.consume(&digest).is_some());
        // Second consume should fail
        assert!(registry.consume(&digest).is_none());
        // Get should also fail after consumption
        assert!(registry.get(&digest).is_none());
    }

    #[test]
    fn test_expired_quote_not_returned() {
        let registry = QuoteRegistry::new(Duration::from_secs(60));
        let digest = [3u8; 32];
        let entry = QuoteEntry {
            service_id: 1,
            job_index: 0,
            price_wei: U256::from(100u64),
            created_at: Instant::now(),
            expires_at: Instant::now() - Duration::from_secs(1), // already expired
            consumed: false,
        };

        registry.insert(digest, entry);
        assert!(registry.get(&digest).is_none());
    }

    #[test]
    fn test_insert_dynamic() {
        let registry = QuoteRegistry::new(Duration::from_secs(300));
        let digest = registry.insert_dynamic(42, 0, U256::from(500u64));
        let entry = registry.get(&digest).unwrap();
        assert_eq!(entry.service_id, 42);
        assert_eq!(entry.job_index, 0);
    }

    #[test]
    fn test_gc_removes_expired_and_consumed() {
        let registry = QuoteRegistry::new(Duration::from_secs(60));

        // Active entry
        registry.insert(
            [1u8; 32],
            QuoteEntry {
                service_id: 1,
                job_index: 0,
                price_wei: U256::ZERO,
                created_at: Instant::now(),
                expires_at: Instant::now() + Duration::from_secs(60),
                consumed: false,
            },
        );

        // Expired entry
        registry.insert(
            [2u8; 32],
            QuoteEntry {
                service_id: 2,
                job_index: 0,
                price_wei: U256::ZERO,
                created_at: Instant::now(),
                expires_at: Instant::now() - Duration::from_secs(1),
                consumed: false,
            },
        );

        // Consumed entry
        registry.insert(
            [3u8; 32],
            QuoteEntry {
                service_id: 3,
                job_index: 0,
                price_wei: U256::ZERO,
                created_at: Instant::now(),
                expires_at: Instant::now() + Duration::from_secs(60),
                consumed: true,
            },
        );

        registry.gc();
        assert_eq!(registry.active_count(), 1);
    }
}
