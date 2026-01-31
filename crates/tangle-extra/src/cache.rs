//! Service Configuration Cache
//!
//! Provides TTL-based caching for on-chain configuration data to reduce RPC calls.
//! Caches aggregation configs, operator weights, and service operator lists.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_extra::cache::ServiceConfigCache;
//! use blueprint_std::time::Duration;
//!
//! // Create cache with 5 minute TTL
//! let cache = ServiceConfigCache::new(Duration::from_secs(300));
//!
//! // Get aggregation config (fetches from chain if not cached or expired)
//! let config = cache.get_aggregation_config(&client, service_id, job_index).await?;
//!
//! // Get operator weights for a service
//! let weights = cache.get_operator_weights(&client, service_id).await?;
//!
//! // Force refresh a specific service's data
//! cache.invalidate_service(service_id);
//! ```

use alloy_primitives::Address;
use blueprint_client_tangle::{AggregationConfig, OperatorMetadata, TangleClient};
use blueprint_std::collections::HashMap;
use blueprint_std::format;
use blueprint_std::string::{String, ToString};
use blueprint_std::sync::{Arc, RwLock};
use blueprint_std::time::{Duration, Instant};
use blueprint_std::vec::Vec;
use core::fmt;
use core::sync::atomic::{AtomicU64, Ordering};

/// Default cache TTL (5 minutes)
pub const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(300);

/// Error type for cache operations
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// Failed to fetch from chain
    #[error("Failed to fetch from chain: {0}")]
    FetchError(String),
    /// Lock poisoned
    #[error("Cache lock poisoned")]
    LockPoisoned,
}

/// A cached entry with timestamp
#[derive(Clone, Debug)]
struct CacheEntry<T> {
    value: T,
    cached_at: Instant,
}

impl<T> CacheEntry<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            cached_at: Instant::now(),
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.cached_at.elapsed() > ttl
    }
}

/// Operator weight information for a service
#[derive(Clone, Debug)]
pub struct OperatorWeights {
    /// Map of operator address to their exposure in basis points
    pub weights: HashMap<Address, u16>,
    /// Total exposure across all operators
    pub total_exposure: u64,
}

impl OperatorWeights {
    /// Get weight for a specific operator
    pub fn get(&self, operator: &Address) -> Option<u16> {
        self.weights.get(operator).copied()
    }

    /// Check if an operator is active in this service
    pub fn contains(&self, operator: &Address) -> bool {
        self.weights.contains_key(operator)
    }

    /// Get the number of active operators
    pub fn len(&self) -> usize {
        self.weights.len()
    }

    /// Check if there are no operators
    pub fn is_empty(&self) -> bool {
        self.weights.is_empty()
    }

    /// Iterate over all operators and their weights
    pub fn iter(&self) -> impl Iterator<Item = (&Address, &u16)> {
        self.weights.iter()
    }

    /// Calculate the stake-weighted threshold count
    ///
    /// Given a threshold in basis points, calculates how many operators
    /// (sorted by weight descending) are needed to meet the threshold.
    pub fn calculate_threshold_signers(&self, threshold_bps: u16) -> usize {
        if self.weights.is_empty() {
            return 0;
        }

        let required_weight = (self.total_exposure * threshold_bps as u64) / 10000;

        // Sort operators by weight descending
        let mut sorted: Vec<_> = self.weights.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));

        let mut accumulated: u64 = 0;
        let mut count = 0;

        for (_, &weight) in sorted {
            accumulated += weight as u64;
            count += 1;
            if accumulated >= required_weight {
                break;
            }
        }

        count
    }
}

/// Service operators list
#[derive(Clone, Debug)]
pub struct ServiceOperators {
    /// List of operator addresses
    pub operators: Vec<Address>,
    /// Map of operator address to index (for bitmap calculation)
    pub index_map: HashMap<Address, usize>,
}

impl ServiceOperators {
    /// Create from a list of operators
    pub fn new(operators: Vec<Address>) -> Self {
        let index_map = operators
            .iter()
            .enumerate()
            .map(|(i, addr)| (*addr, i))
            .collect();
        Self {
            operators,
            index_map,
        }
    }

    /// Get the index of an operator
    pub fn index_of(&self, operator: &Address) -> Option<usize> {
        self.index_map.get(operator).copied()
    }

    /// Get the number of operators
    pub fn len(&self) -> usize {
        self.operators.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.operators.is_empty()
    }

    /// Iterate over operators
    pub fn iter(&self) -> impl Iterator<Item = &Address> {
        self.operators.iter()
    }
}

/// Thread-safe cache for service configurations
///
/// Caches aggregation configs, operator weights, and operator lists
/// with TTL-based expiration.
pub struct ServiceConfigCache {
    /// TTL for cache entries
    ttl: Duration,
    /// Aggregation config cache: (service_id, job_index) -> config
    aggregation_configs: RwLock<HashMap<(u64, u8), CacheEntry<AggregationConfig>>>,
    /// Operator weights cache: service_id -> weights
    operator_weights: RwLock<HashMap<u64, CacheEntry<OperatorWeights>>>,
    /// Service operators cache: service_id -> operators
    service_operators: RwLock<HashMap<u64, CacheEntry<ServiceOperators>>>,
    /// Operator metadata cache: (blueprint_id, operator) -> metadata
    operator_metadata: RwLock<HashMap<(u64, Address), CacheEntry<OperatorMetadata>>>,
}

impl ServiceConfigCache {
    /// Create a new cache with the specified TTL
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            aggregation_configs: RwLock::new(HashMap::new()),
            operator_weights: RwLock::new(HashMap::new()),
            service_operators: RwLock::new(HashMap::new()),
            operator_metadata: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new cache with default TTL (5 minutes)
    pub fn with_default_ttl() -> Self {
        Self::new(DEFAULT_CACHE_TTL)
    }

    /// Get the current TTL
    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    /// Set a new TTL (does not affect already cached entries)
    pub fn set_ttl(&mut self, ttl: Duration) {
        self.ttl = ttl;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // AGGREGATION CONFIG
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get aggregation config, using cache if available and not expired
    pub async fn get_aggregation_config(
        &self,
        client: &TangleClient,
        service_id: u64,
        job_index: u8,
    ) -> Result<AggregationConfig, CacheError> {
        let key = (service_id, job_index);

        // Check cache first
        {
            let cache = self
                .aggregation_configs
                .read()
                .map_err(|_| CacheError::LockPoisoned)?;
            if let Some(entry) = cache.get(&key) {
                if !entry.is_expired(self.ttl) {
                    blueprint_core::trace!(
                        target: "service-config-cache",
                        "Cache hit for aggregation config: service={}, job={}",
                        service_id,
                        job_index
                    );
                    return Ok(entry.value.clone());
                }
            }
        }

        // Cache miss or expired, fetch from chain
        blueprint_core::debug!(
            target: "service-config-cache",
            "Cache miss for aggregation config: service={}, job={}, fetching from chain",
            service_id,
            job_index
        );

        let config = client
            .get_aggregation_config(service_id, job_index)
            .await
            .map_err(|e| CacheError::FetchError(e.to_string()))?;

        // Store in cache
        {
            let mut cache = self
                .aggregation_configs
                .write()
                .map_err(|_| CacheError::LockPoisoned)?;
            cache.insert(key, CacheEntry::new(config.clone()));
        }

        Ok(config)
    }

    /// Pre-populate aggregation config cache
    pub fn set_aggregation_config(
        &self,
        service_id: u64,
        job_index: u8,
        config: AggregationConfig,
    ) -> Result<(), CacheError> {
        let mut cache = self
            .aggregation_configs
            .write()
            .map_err(|_| CacheError::LockPoisoned)?;
        cache.insert((service_id, job_index), CacheEntry::new(config));
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // OPERATOR WEIGHTS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get operator weights for a service, using cache if available
    pub async fn get_operator_weights(
        &self,
        client: &TangleClient,
        service_id: u64,
    ) -> Result<OperatorWeights, CacheError> {
        // Check cache first
        {
            let cache = self
                .operator_weights
                .read()
                .map_err(|_| CacheError::LockPoisoned)?;
            if let Some(entry) = cache.get(&service_id) {
                if !entry.is_expired(self.ttl) {
                    blueprint_core::trace!(
                        target: "service-config-cache",
                        "Cache hit for operator weights: service={}",
                        service_id
                    );
                    return Ok(entry.value.clone());
                }
            }
        }

        // Cache miss or expired, fetch from chain
        blueprint_core::debug!(
            target: "service-config-cache",
            "Cache miss for operator weights: service={}, fetching from chain",
            service_id
        );

        let weights = self.fetch_operator_weights(client, service_id).await?;

        // Store in cache
        {
            let mut cache = self
                .operator_weights
                .write()
                .map_err(|_| CacheError::LockPoisoned)?;
            cache.insert(service_id, CacheEntry::new(weights.clone()));
        }

        Ok(weights)
    }

    /// Fetch operator weights from chain
    async fn fetch_operator_weights(
        &self,
        client: &TangleClient,
        service_id: u64,
    ) -> Result<OperatorWeights, CacheError> {
        // Get list of operators
        let operators = client
            .get_service_operators(service_id)
            .await
            .map_err(|e| CacheError::FetchError(format!("Failed to get operators: {}", e)))?;

        // Fetch each operator's weight
        let mut weights = HashMap::new();
        let mut total_exposure: u64 = 0;

        for operator in operators {
            match client.get_service_operator(service_id, operator).await {
                Ok(op_info) => {
                    if op_info.active {
                        weights.insert(operator, op_info.exposureBps);
                        total_exposure += op_info.exposureBps as u64;
                    }
                }
                Err(e) => {
                    blueprint_core::warn!(
                        target: "service-config-cache",
                        "Failed to get operator info for {}: {}",
                        operator,
                        e
                    );
                }
            }
        }

        Ok(OperatorWeights {
            weights,
            total_exposure,
        })
    }

    /// Pre-populate operator weights cache
    pub fn set_operator_weights(
        &self,
        service_id: u64,
        weights: OperatorWeights,
    ) -> Result<(), CacheError> {
        let mut cache = self
            .operator_weights
            .write()
            .map_err(|_| CacheError::LockPoisoned)?;
        cache.insert(service_id, CacheEntry::new(weights));
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SERVICE OPERATORS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get service operators list, using cache if available
    pub async fn get_service_operators(
        &self,
        client: &TangleClient,
        service_id: u64,
    ) -> Result<ServiceOperators, CacheError> {
        // Check cache first
        {
            let cache = self
                .service_operators
                .read()
                .map_err(|_| CacheError::LockPoisoned)?;
            if let Some(entry) = cache.get(&service_id) {
                if !entry.is_expired(self.ttl) {
                    blueprint_core::trace!(
                        target: "service-config-cache",
                        "Cache hit for service operators: service={}",
                        service_id
                    );
                    return Ok(entry.value.clone());
                }
            }
        }

        // Cache miss or expired, fetch from chain
        blueprint_core::debug!(
            target: "service-config-cache",
            "Cache miss for service operators: service={}, fetching from chain",
            service_id
        );

        let operators_list = client
            .get_service_operators(service_id)
            .await
            .map_err(|e| CacheError::FetchError(e.to_string()))?;

        let operators = ServiceOperators::new(operators_list);

        // Store in cache
        {
            let mut cache = self
                .service_operators
                .write()
                .map_err(|_| CacheError::LockPoisoned)?;
            cache.insert(service_id, CacheEntry::new(operators.clone()));
        }

        Ok(operators)
    }

    /// Get metadata for a specific operator (cached by blueprint + operator)
    pub async fn get_operator_metadata(
        &self,
        client: &TangleClient,
        blueprint_id: u64,
        operator: Address,
    ) -> Result<OperatorMetadata, CacheError> {
        let key = (blueprint_id, operator);
        if let Some(entry) = self
            .operator_metadata
            .read()
            .map_err(|_| CacheError::LockPoisoned)?
            .get(&key)
            .cloned()
        {
            if !entry.is_expired(self.ttl) {
                return Ok(entry.value);
            }
        }

        let metadata = client
            .get_operator_metadata(blueprint_id, operator)
            .await
            .map_err(|e| CacheError::FetchError(e.to_string()))?;
        let mut guard = self
            .operator_metadata
            .write()
            .map_err(|_| CacheError::LockPoisoned)?;
        guard.insert(key, CacheEntry::new(metadata.clone()));
        Ok(metadata)
    }

    /// Get metadata for all operators in a service.
    pub async fn get_service_operator_metadata(
        &self,
        client: &TangleClient,
        blueprint_id: u64,
        service_id: u64,
    ) -> Result<HashMap<Address, OperatorMetadata>, CacheError> {
        let operators = self.get_service_operators(client, service_id).await?;
        let mut result = HashMap::with_capacity(operators.len());
        for operator in operators.iter() {
            let metadata = self
                .get_operator_metadata(client, blueprint_id, *operator)
                .await?;
            result.insert(*operator, metadata);
        }
        Ok(result)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // CACHE MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Invalidate all cached data for a specific service
    pub fn invalidate_service(&self, service_id: u64) {
        blueprint_core::debug!(
            target: "service-config-cache",
            "Invalidating cache for service {}",
            service_id
        );

        // Remove aggregation configs for this service
        if let Ok(mut cache) = self.aggregation_configs.write() {
            cache.retain(|(sid, _), _| *sid != service_id);
        }

        // Remove operator weights
        if let Ok(mut cache) = self.operator_weights.write() {
            cache.remove(&service_id);
        }

        // Remove service operators
        if let Ok(mut cache) = self.service_operators.write() {
            cache.remove(&service_id);
        }
    }

    /// Invalidate a specific aggregation config
    pub fn invalidate_aggregation_config(&self, service_id: u64, job_index: u8) {
        if let Ok(mut cache) = self.aggregation_configs.write() {
            cache.remove(&(service_id, job_index));
        }
    }

    /// Clear all cached data
    pub fn clear(&self) {
        blueprint_core::debug!(
            target: "service-config-cache",
            "Clearing all cached service configs"
        );

        if let Ok(mut cache) = self.aggregation_configs.write() {
            cache.clear();
        }
        if let Ok(mut cache) = self.operator_weights.write() {
            cache.clear();
        }
        if let Ok(mut cache) = self.service_operators.write() {
            cache.clear();
        }
    }

    /// Remove expired entries from all caches
    pub fn cleanup_expired(&self) {
        let ttl = self.ttl;

        if let Ok(mut cache) = self.aggregation_configs.write() {
            cache.retain(|_, entry| !entry.is_expired(ttl));
        }
        if let Ok(mut cache) = self.operator_weights.write() {
            cache.retain(|_, entry| !entry.is_expired(ttl));
        }
        if let Ok(mut cache) = self.service_operators.write() {
            cache.retain(|_, entry| !entry.is_expired(ttl));
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let aggregation_count = self
            .aggregation_configs
            .read()
            .map(|c| c.len())
            .unwrap_or(0);
        let weights_count = self.operator_weights.read().map(|c| c.len()).unwrap_or(0);
        let operators_count = self.service_operators.read().map(|c| c.len()).unwrap_or(0);

        CacheStats {
            aggregation_configs: aggregation_count,
            operator_weights: weights_count,
            service_operators: operators_count,
            ttl: self.ttl,
        }
    }
}

impl Default for ServiceConfigCache {
    fn default() -> Self {
        Self::with_default_ttl()
    }
}

/// Cache statistics
#[derive(Clone, Debug)]
pub struct CacheStats {
    /// Number of cached aggregation configs
    pub aggregation_configs: usize,
    /// Number of cached operator weights
    pub operator_weights: usize,
    /// Number of cached service operator lists
    pub service_operators: usize,
    /// Current TTL setting
    pub ttl: Duration,
}

impl fmt::Display for CacheStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ServiceConfigCache {{ aggregation_configs: {}, operator_weights: {}, service_operators: {}, ttl: {:?} }}",
            self.aggregation_configs, self.operator_weights, self.service_operators, self.ttl
        )
    }
}

/// A shared, thread-safe cache wrapped in Arc
pub type SharedServiceConfigCache = Arc<ServiceConfigCache>;

/// Create a new shared cache with default TTL
pub fn shared_cache() -> SharedServiceConfigCache {
    Arc::new(ServiceConfigCache::with_default_ttl())
}

/// Create a new shared cache with custom TTL
pub fn shared_cache_with_ttl(ttl: Duration) -> SharedServiceConfigCache {
    Arc::new(ServiceConfigCache::new(ttl))
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVENT-DRIVEN CACHE SYNC
// ═══════════════════════════════════════════════════════════════════════════════

/// Events that trigger cache invalidation
#[derive(Debug, Clone)]
pub enum CacheInvalidationEvent {
    /// Operator joined a service - invalidates operator weights and list
    OperatorJoined { service_id: u64, operator: Address },
    /// Operator left a service - invalidates operator weights and list
    OperatorLeft { service_id: u64, operator: Address },
    /// Service was terminated - clears all service data
    ServiceTerminated { service_id: u64 },
    /// Service was activated - optionally pre-warm cache
    ServiceActivated { service_id: u64 },
}

impl fmt::Display for CacheInvalidationEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperatorJoined {
                service_id,
                operator,
            } => {
                write!(
                    f,
                    "OperatorJoined(service={}, operator={})",
                    service_id, operator
                )
            }
            Self::OperatorLeft {
                service_id,
                operator,
            } => {
                write!(
                    f,
                    "OperatorLeft(service={}, operator={})",
                    service_id, operator
                )
            }
            Self::ServiceTerminated { service_id } => {
                write!(f, "ServiceTerminated(service={})", service_id)
            }
            Self::ServiceActivated { service_id } => {
                write!(f, "ServiceActivated(service={})", service_id)
            }
        }
    }
}

impl ServiceConfigCache {
    /// Handle a cache invalidation event
    ///
    /// Call this when you receive relevant on-chain events to keep the cache in sync.
    /// Logs clearly when invalidation occurs.
    pub fn handle_event(&self, event: &CacheInvalidationEvent) {
        blueprint_core::info!(
            target: "service-config-cache",
            "⚡ Cache invalidation triggered by event: {}",
            event
        );

        match event {
            CacheInvalidationEvent::OperatorJoined {
                service_id,
                operator,
            } => {
                blueprint_core::info!(
                    target: "service-config-cache",
                    "🔄 Invalidating cache: operator {} joined service {}",
                    operator,
                    service_id
                );
                self.invalidate_operator_data(*service_id);
            }
            CacheInvalidationEvent::OperatorLeft {
                service_id,
                operator,
            } => {
                blueprint_core::info!(
                    target: "service-config-cache",
                    "🔄 Invalidating cache: operator {} left service {}",
                    operator,
                    service_id
                );
                self.invalidate_operator_data(*service_id);
            }
            CacheInvalidationEvent::ServiceTerminated { service_id } => {
                blueprint_core::info!(
                    target: "service-config-cache",
                    "🗑️ Clearing all cache for terminated service {}",
                    service_id
                );
                self.invalidate_service(*service_id);
            }
            CacheInvalidationEvent::ServiceActivated { service_id } => {
                blueprint_core::info!(
                    target: "service-config-cache",
                    "✨ Service {} activated (cache will be populated on first access)",
                    service_id
                );
                // No invalidation needed - cache will be populated on first access
            }
        }
    }

    /// Invalidate only operator-related data for a service (weights and operator list)
    fn invalidate_operator_data(&self, service_id: u64) {
        if let Ok(mut cache) = self.operator_weights.write() {
            cache.remove(&service_id);
        }
        if let Ok(mut cache) = self.service_operators.write() {
            cache.remove(&service_id);
        }
    }
}

/// Service that syncs the cache with on-chain events
///
/// Provides both polling-based and manual event processing for cache invalidation.
///
/// # Example
///
/// ```rust,ignore
/// use blueprint_tangle_extra::cache::{CacheSyncService, shared_cache};
///
/// let cache = shared_cache();
/// let sync_service = CacheSyncService::new(client, cache.clone());
///
/// // Option 1: Poll for events periodically
/// loop {
///     let events_processed = sync_service.poll_and_sync(last_block).await?;
///     tokio::time::sleep(Duration::from_secs(12)).await;
/// }
///
/// // Option 2: Process events from your own subscription
/// sync_service.process_logs(&logs);
/// ```
pub struct CacheSyncService {
    client: Arc<TangleClient>,
    cache: SharedServiceConfigCache,
    /// Services to watch (None = watch all)
    watched_services: Option<Vec<u64>>,
    /// Last processed block
    last_block: AtomicU64,
}

impl CacheSyncService {
    /// Create a new cache sync service
    pub fn new(client: Arc<TangleClient>, cache: SharedServiceConfigCache) -> Self {
        Self {
            client,
            cache,
            watched_services: None,
            last_block: AtomicU64::new(0),
        }
    }

    /// Only watch specific services
    pub fn with_services(mut self, services: Vec<u64>) -> Self {
        self.watched_services = Some(services);
        self
    }

    /// Set the starting block for polling
    pub fn from_block(self, block: u64) -> Self {
        self.last_block.store(block, Ordering::Relaxed);
        self
    }

    /// Check if a service should be watched
    fn should_watch(&self, service_id: u64) -> bool {
        self.watched_services
            .as_ref()
            .map(|s| s.contains(&service_id))
            .unwrap_or(true)
    }

    /// Poll for new events and sync the cache
    ///
    /// Returns the number of events processed.
    pub async fn poll_and_sync(&self) -> Result<usize, CacheError> {
        use alloy_rpc_types::Filter;
        use blueprint_client_tangle::contracts::ITangle;

        let from_block = self.last_block.load(Ordering::Relaxed);
        let tangle_address = self.client.config.settings.tangle_contract;

        // Create filter for relevant events
        let filter = Filter::new()
            .address(tangle_address)
            .from_block(from_block)
            .events([
                <ITangle::OperatorJoinedService as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                <ITangle::OperatorLeftService as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                <ITangle::ServiceTerminated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                <ITangle::ServiceActivated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
            ]);

        let logs = self
            .client
            .get_logs(&filter)
            .await
            .map_err(|e| CacheError::FetchError(format!("Failed to fetch logs: {}", e)))?;

        let count = self.process_logs(&logs);

        // Update last block
        if let Some(last_log) = logs.last() {
            if let Some(block_num) = last_log.block_number {
                self.last_block.store(block_num + 1, Ordering::Relaxed);
            }
        }

        Ok(count)
    }

    /// Process a batch of logs and invalidate cache as needed
    ///
    /// Returns the number of events processed.
    pub fn process_logs(&self, logs: &[alloy_rpc_types::Log]) -> usize {
        let mut count = 0;
        for log in logs {
            if let Some(event) = self.parse_log(log) {
                let service_id = match &event {
                    CacheInvalidationEvent::OperatorJoined { service_id, .. } => *service_id,
                    CacheInvalidationEvent::OperatorLeft { service_id, .. } => *service_id,
                    CacheInvalidationEvent::ServiceTerminated { service_id } => *service_id,
                    CacheInvalidationEvent::ServiceActivated { service_id } => *service_id,
                };
                if self.should_watch(service_id) {
                    self.cache.handle_event(&event);
                    count += 1;
                }
            }
        }
        count
    }

    /// Parse a log into a cache invalidation event
    pub fn parse_log(&self, log: &alloy_rpc_types::Log) -> Option<CacheInvalidationEvent> {
        use blueprint_client_tangle::contracts::ITangle;

        // Try to decode each event type
        if let Ok(event) = log.log_decode::<ITangle::OperatorJoinedService>() {
            return Some(CacheInvalidationEvent::OperatorJoined {
                service_id: event.inner.serviceId,
                operator: event.inner.operator,
            });
        }

        if let Ok(event) = log.log_decode::<ITangle::OperatorLeftService>() {
            return Some(CacheInvalidationEvent::OperatorLeft {
                service_id: event.inner.serviceId,
                operator: event.inner.operator,
            });
        }

        if let Ok(event) = log.log_decode::<ITangle::ServiceTerminated>() {
            return Some(CacheInvalidationEvent::ServiceTerminated {
                service_id: event.inner.serviceId,
            });
        }

        if let Ok(event) = log.log_decode::<ITangle::ServiceActivated>() {
            return Some(CacheInvalidationEvent::ServiceActivated {
                service_id: event.inner.serviceId,
            });
        }

        None
    }

    /// Process a single event manually (useful for testing or custom event sources)
    pub fn process_event(&self, event: CacheInvalidationEvent) {
        self.cache.handle_event(&event);
    }

    /// Get the last processed block number
    pub fn last_block(&self) -> u64 {
        self.last_block.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new(42);

        // Should not be expired immediately
        assert!(!entry.is_expired(Duration::from_secs(1)));

        // Should be expired with zero TTL
        assert!(entry.is_expired(Duration::ZERO));
    }

    #[test]
    fn test_operator_weights_threshold_calculation() {
        let mut weights = HashMap::new();
        // 3 operators: 5000, 3000, 2000 bps (total = 10000)
        weights.insert(Address::ZERO, 5000);
        weights.insert(Address::repeat_byte(1), 3000);
        weights.insert(Address::repeat_byte(2), 2000);

        let op_weights = OperatorWeights {
            weights,
            total_exposure: 10000,
        };

        // 50% threshold = 5000 bps needed
        // Sorted: [5000, 3000, 2000]
        // Just operator 0 (5000) meets 50%
        assert_eq!(op_weights.calculate_threshold_signers(5000), 1);

        // 67% threshold = 6700 bps needed
        // Need operator 0 (5000) + operator 1 (3000) = 8000
        assert_eq!(op_weights.calculate_threshold_signers(6700), 2);

        // 100% threshold = 10000 bps needed
        // Need all 3 operators
        assert_eq!(op_weights.calculate_threshold_signers(10000), 3);
    }

    #[test]
    fn test_service_operators_index() {
        let ops = vec![
            Address::repeat_byte(1),
            Address::repeat_byte(2),
            Address::repeat_byte(3),
        ];
        let service_ops = ServiceOperators::new(ops);

        assert_eq!(service_ops.index_of(&Address::repeat_byte(1)), Some(0));
        assert_eq!(service_ops.index_of(&Address::repeat_byte(2)), Some(1));
        assert_eq!(service_ops.index_of(&Address::repeat_byte(3)), Some(2));
        assert_eq!(service_ops.index_of(&Address::repeat_byte(4)), None);
    }

    #[test]
    fn test_cache_stats() {
        let cache = ServiceConfigCache::with_default_ttl();
        let stats = cache.stats();

        assert_eq!(stats.aggregation_configs, 0);
        assert_eq!(stats.operator_weights, 0);
        assert_eq!(stats.service_operators, 0);
        assert_eq!(stats.ttl, DEFAULT_CACHE_TTL);
    }

    #[test]
    fn test_cache_invalidation_event_display() {
        let event = CacheInvalidationEvent::OperatorJoined {
            service_id: 1,
            operator: Address::repeat_byte(0xAB),
        };
        assert!(event.to_string().contains("OperatorJoined"));
        assert!(event.to_string().contains("service=1"));

        let event = CacheInvalidationEvent::OperatorLeft {
            service_id: 2,
            operator: Address::repeat_byte(0xCD),
        };
        assert!(event.to_string().contains("OperatorLeft"));

        let event = CacheInvalidationEvent::ServiceTerminated { service_id: 3 };
        assert!(event.to_string().contains("ServiceTerminated"));

        let event = CacheInvalidationEvent::ServiceActivated { service_id: 4 };
        assert!(event.to_string().contains("ServiceActivated"));
    }

    #[test]
    fn test_handle_operator_joined_invalidates_cache() {
        let cache = ServiceConfigCache::with_default_ttl();

        // Pre-populate cache
        let mut weights = HashMap::new();
        weights.insert(Address::ZERO, 5000u16);
        cache
            .set_operator_weights(
                1,
                OperatorWeights {
                    weights,
                    total_exposure: 5000,
                },
            )
            .unwrap();

        // Verify cache is populated
        assert_eq!(cache.stats().operator_weights, 1);

        // Handle operator joined event
        cache.handle_event(&CacheInvalidationEvent::OperatorJoined {
            service_id: 1,
            operator: Address::repeat_byte(1),
        });

        // Cache should be invalidated
        assert_eq!(cache.stats().operator_weights, 0);
    }

    #[test]
    fn test_handle_operator_left_invalidates_cache() {
        let cache = ServiceConfigCache::with_default_ttl();

        // Pre-populate cache
        let mut weights = HashMap::new();
        weights.insert(Address::ZERO, 5000u16);
        cache
            .set_operator_weights(
                1,
                OperatorWeights {
                    weights,
                    total_exposure: 5000,
                },
            )
            .unwrap();

        assert_eq!(cache.stats().operator_weights, 1);

        // Handle operator left event
        cache.handle_event(&CacheInvalidationEvent::OperatorLeft {
            service_id: 1,
            operator: Address::ZERO,
        });

        // Cache should be invalidated
        assert_eq!(cache.stats().operator_weights, 0);
    }

    #[test]
    fn test_handle_service_terminated_clears_all() {
        let cache = ServiceConfigCache::with_default_ttl();

        // Pre-populate cache for service 1
        let mut weights = HashMap::new();
        weights.insert(Address::ZERO, 5000u16);
        cache
            .set_operator_weights(
                1,
                OperatorWeights {
                    weights: weights.clone(),
                    total_exposure: 5000,
                },
            )
            .unwrap();

        // Also populate service 2
        cache
            .set_operator_weights(
                2,
                OperatorWeights {
                    weights,
                    total_exposure: 5000,
                },
            )
            .unwrap();

        assert_eq!(cache.stats().operator_weights, 2);

        // Terminate service 1
        cache.handle_event(&CacheInvalidationEvent::ServiceTerminated { service_id: 1 });

        // Only service 1 should be cleared
        assert_eq!(cache.stats().operator_weights, 1);
    }

    #[test]
    fn test_handle_service_activated_no_invalidation() {
        let cache = ServiceConfigCache::with_default_ttl();

        // Pre-populate cache
        let mut weights = HashMap::new();
        weights.insert(Address::ZERO, 5000u16);
        cache
            .set_operator_weights(
                1,
                OperatorWeights {
                    weights,
                    total_exposure: 5000,
                },
            )
            .unwrap();

        assert_eq!(cache.stats().operator_weights, 1);

        // Service activated should NOT invalidate existing cache
        cache.handle_event(&CacheInvalidationEvent::ServiceActivated { service_id: 1 });

        // Cache should still be there
        assert_eq!(cache.stats().operator_weights, 1);
    }

    #[test]
    fn test_invalidation_only_affects_target_service() {
        let cache = ServiceConfigCache::with_default_ttl();

        // Populate cache for services 1, 2, 3
        for service_id in 1..=3 {
            let mut weights = HashMap::new();
            weights.insert(Address::repeat_byte(service_id as u8), 5000u16);
            cache
                .set_operator_weights(
                    service_id,
                    OperatorWeights {
                        weights,
                        total_exposure: 5000,
                    },
                )
                .unwrap();
        }

        assert_eq!(cache.stats().operator_weights, 3);

        // Invalidate only service 2
        cache.handle_event(&CacheInvalidationEvent::OperatorJoined {
            service_id: 2,
            operator: Address::repeat_byte(0xFF),
        });

        // Should have 2 services left
        assert_eq!(cache.stats().operator_weights, 2);
    }
}
