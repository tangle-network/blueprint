//! In-memory aggregation state management

use crate::types::TaskId;
use alloy_primitives::U256;
use blueprint_crypto_bn254::{ArkBlsBn254Public, ArkBlsBn254Signature};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Threshold type for aggregation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdType {
    /// Count-based: need at least N signatures
    Count(u32),
    /// Stake-weighted: need at least N basis points (0-10000) of total stake
    StakeWeighted(u32),
}

impl Default for ThresholdType {
    fn default() -> Self {
        ThresholdType::Count(1)
    }
}

/// Operator information for stake-weighted aggregation
#[derive(Debug, Clone)]
pub struct OperatorInfo {
    /// Operator's stake weight (can be actual stake or relative weight)
    pub stake: u64,
    /// Whether this operator has been registered
    pub registered: bool,
}

impl Default for OperatorInfo {
    fn default() -> Self {
        Self {
            stake: 1, // Default weight of 1 for count-based
            registered: true,
        }
    }
}

/// State for a single aggregation task
#[derive(Debug)]
pub struct TaskState {
    /// Service ID
    pub service_id: u64,
    /// Call ID
    pub call_id: u64,
    /// The output being signed
    pub output: Vec<u8>,
    /// Number of operators in the service
    pub operator_count: u32,
    /// Threshold type and value
    pub threshold_type: ThresholdType,
    /// Bitmap of which operators have signed (bit i = operator i signed)
    pub signer_bitmap: U256,
    /// Collected signatures indexed by operator index
    pub signatures: HashMap<u32, ArkBlsBn254Signature>,
    /// Collected public keys indexed by operator index
    pub public_keys: HashMap<u32, ArkBlsBn254Public>,
    /// Operator stakes for stake-weighted thresholds
    pub operator_stakes: HashMap<u32, u64>,
    /// Total stake of all operators
    pub total_stake: u64,
    /// Whether this task has been submitted to chain
    pub submitted: bool,
    /// When this task was created
    pub created_at: Instant,
    /// When this task expires (None = never)
    pub expires_at: Option<Instant>,
}

impl TaskState {
    /// Create a new task state with count-based threshold
    pub fn new(
        service_id: u64,
        call_id: u64,
        output: Vec<u8>,
        operator_count: u32,
        threshold: u32,
    ) -> Self {
        Self::with_config(
            service_id,
            call_id,
            output,
            operator_count,
            ThresholdType::Count(threshold),
            None,
            None,
        )
    }

    /// Create a new task state with full configuration
    pub fn with_config(
        service_id: u64,
        call_id: u64,
        output: Vec<u8>,
        operator_count: u32,
        threshold_type: ThresholdType,
        operator_stakes: Option<HashMap<u32, u64>>,
        ttl: Option<Duration>,
    ) -> Self {
        let now = Instant::now();
        let expires_at = ttl.map(|d| now + d);

        // Calculate total stake
        let (stakes, total_stake) = if let Some(stakes) = operator_stakes {
            let total: u64 = stakes.values().sum();
            (stakes, total)
        } else {
            // Default: each operator has stake of 1
            let stakes: HashMap<u32, u64> = (0..operator_count).map(|i| (i, 1u64)).collect();
            let total = operator_count as u64;
            (stakes, total)
        };

        Self {
            service_id,
            call_id,
            output,
            operator_count,
            threshold_type,
            signer_bitmap: U256::ZERO,
            signatures: HashMap::new(),
            public_keys: HashMap::new(),
            operator_stakes: stakes,
            total_stake,
            submitted: false,
            created_at: now,
            expires_at,
        }
    }

    /// Check if this task has expired
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|t| Instant::now() > t).unwrap_or(false)
    }

    /// Get remaining time until expiry
    pub fn time_remaining(&self) -> Option<Duration> {
        self.expires_at.and_then(|t| {
            let now = Instant::now();
            if now < t {
                Some(t - now)
            } else {
                None
            }
        })
    }

    /// Add a signature from an operator
    pub fn add_signature(
        &mut self,
        operator_index: u32,
        signature: ArkBlsBn254Signature,
        public_key: ArkBlsBn254Public,
    ) -> Result<(), &'static str> {
        if operator_index >= self.operator_count {
            return Err("Operator index out of bounds");
        }

        if self.has_signed(operator_index) {
            return Err("Operator already signed");
        }

        if self.is_expired() {
            return Err("Task has expired");
        }

        // Set bit in bitmap
        self.signer_bitmap |= U256::from(1u64) << operator_index as usize;

        // Store signature and public key
        self.signatures.insert(operator_index, signature);
        self.public_keys.insert(operator_index, public_key);

        Ok(())
    }

    /// Check if an operator has already signed
    pub fn has_signed(&self, operator_index: u32) -> bool {
        (self.signer_bitmap >> operator_index as usize) & U256::from(1u64) == U256::from(1u64)
    }

    /// Get the number of signatures collected
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }

    /// Get the total stake that has signed
    pub fn signed_stake(&self) -> u64 {
        self.signatures
            .keys()
            .map(|idx| self.operator_stakes.get(idx).copied().unwrap_or(0))
            .sum()
    }

    /// Get the signed stake as basis points (0-10000) of total stake
    pub fn signed_stake_bps(&self) -> u32 {
        if self.total_stake == 0 {
            return 0;
        }
        ((self.signed_stake() * 10000) / self.total_stake) as u32
    }

    /// Check if threshold is met
    pub fn threshold_met(&self) -> bool {
        match self.threshold_type {
            ThresholdType::Count(n) => self.signature_count() >= n as usize,
            ThresholdType::StakeWeighted(bps) => self.signed_stake_bps() >= bps,
        }
    }

    /// Get the threshold value (for API responses)
    pub fn threshold_value(&self) -> usize {
        match self.threshold_type {
            ThresholdType::Count(n) => n as usize,
            ThresholdType::StakeWeighted(bps) => bps as usize,
        }
    }

    /// Get list of operators who haven't signed (non-signers)
    pub fn get_non_signers(&self) -> Vec<u32> {
        (0..self.operator_count)
            .filter(|&i| !self.has_signed(i))
            .collect()
    }

    /// Get list of operators who have signed
    pub fn get_signers(&self) -> Vec<u32> {
        let mut signers: Vec<_> = self.signatures.keys().copied().collect();
        signers.sort();
        signers
    }

    /// Get all signatures and public keys in order for aggregation
    pub fn get_signatures_for_aggregation(&self) -> (Vec<ArkBlsBn254Signature>, Vec<ArkBlsBn254Public>) {
        let mut sigs = Vec::with_capacity(self.signatures.len());
        let mut pks = Vec::with_capacity(self.public_keys.len());

        // Collect in sorted order by operator index
        let indices = self.get_signers();

        for idx in indices {
            if let (Some(sig), Some(pk)) = (self.signatures.get(&idx), self.public_keys.get(&idx)) {
                sigs.push(sig.clone());
                pks.push(pk.clone());
            }
        }

        (sigs, pks)
    }
}

/// Configuration for task initialization
#[derive(Debug, Clone)]
pub struct TaskConfig {
    /// Threshold type
    pub threshold_type: ThresholdType,
    /// Operator stakes (optional, defaults to equal weight)
    pub operator_stakes: Option<HashMap<u32, u64>>,
    /// Time-to-live for the task
    pub ttl: Option<Duration>,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            threshold_type: ThresholdType::Count(1),
            operator_stakes: None,
            ttl: None,
        }
    }
}

/// Global aggregation state manager
#[derive(Debug, Clone)]
pub struct AggregationState {
    /// All active tasks
    tasks: Arc<RwLock<HashMap<TaskId, TaskState>>>,
}

impl Default for AggregationState {
    fn default() -> Self {
        Self::new()
    }
}

impl AggregationState {
    /// Create a new aggregation state manager
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize a new aggregation task (simple API)
    pub fn init_task(
        &self,
        service_id: u64,
        call_id: u64,
        output: Vec<u8>,
        operator_count: u32,
        threshold: u32,
    ) -> Result<(), &'static str> {
        self.init_task_with_config(
            service_id,
            call_id,
            output,
            operator_count,
            TaskConfig {
                threshold_type: ThresholdType::Count(threshold),
                ..Default::default()
            },
        )
    }

    /// Initialize a new aggregation task with full configuration
    pub fn init_task_with_config(
        &self,
        service_id: u64,
        call_id: u64,
        output: Vec<u8>,
        operator_count: u32,
        config: TaskConfig,
    ) -> Result<(), &'static str> {
        let task_id = TaskId::new(service_id, call_id);
        let mut tasks = self.tasks.write();

        if tasks.contains_key(&task_id) {
            return Err("Task already exists");
        }

        let state = TaskState::with_config(
            service_id,
            call_id,
            output,
            operator_count,
            config.threshold_type,
            config.operator_stakes,
            config.ttl,
        );
        tasks.insert(task_id, state);
        Ok(())
    }

    /// Get the expected output for a task (for validation)
    pub fn get_task_output(&self, service_id: u64, call_id: u64) -> Option<Vec<u8>> {
        let task_id = TaskId::new(service_id, call_id);
        let tasks = self.tasks.read();
        tasks.get(&task_id).map(|t| t.output.clone())
    }

    /// Submit a signature for a task
    pub fn submit_signature(
        &self,
        service_id: u64,
        call_id: u64,
        operator_index: u32,
        signature: ArkBlsBn254Signature,
        public_key: ArkBlsBn254Public,
    ) -> Result<(usize, bool), &'static str> {
        let task_id = TaskId::new(service_id, call_id);
        let mut tasks = self.tasks.write();

        let task = tasks.get_mut(&task_id).ok_or("Task not found")?;

        if task.submitted {
            return Err("Task already submitted to chain");
        }

        if task.is_expired() {
            return Err("Task has expired");
        }

        task.add_signature(operator_index, signature, public_key)?;

        Ok((task.signature_count(), task.threshold_met()))
    }

    /// Get task status
    pub fn get_status(&self, service_id: u64, call_id: u64) -> Option<TaskStatus> {
        let task_id = TaskId::new(service_id, call_id);
        let tasks = self.tasks.read();

        tasks.get(&task_id).map(|task| TaskStatus {
            signatures_collected: task.signature_count(),
            threshold_required: task.threshold_value(),
            threshold_type: task.threshold_type,
            threshold_met: task.threshold_met(),
            signer_bitmap: task.signer_bitmap,
            signed_stake_bps: task.signed_stake_bps(),
            submitted: task.submitted,
            is_expired: task.is_expired(),
            time_remaining_secs: task.time_remaining().map(|d| d.as_secs()),
        })
    }

    /// Get task for aggregation (if threshold met)
    pub fn get_for_aggregation(&self, service_id: u64, call_id: u64) -> Option<TaskForAggregation> {
        let task_id = TaskId::new(service_id, call_id);
        let tasks = self.tasks.read();

        let task = tasks.get(&task_id)?;

        if !task.threshold_met() || task.submitted || task.is_expired() {
            return None;
        }

        let (signatures, public_keys) = task.get_signatures_for_aggregation();

        Some(TaskForAggregation {
            service_id: task.service_id,
            call_id: task.call_id,
            output: task.output.clone(),
            signer_bitmap: task.signer_bitmap,
            non_signer_indices: task.get_non_signers(),
            signatures,
            public_keys,
        })
    }

    /// Mark task as submitted
    pub fn mark_submitted(&self, service_id: u64, call_id: u64) -> Result<(), &'static str> {
        let task_id = TaskId::new(service_id, call_id);
        let mut tasks = self.tasks.write();

        let task = tasks.get_mut(&task_id).ok_or("Task not found")?;
        task.submitted = true;
        Ok(())
    }

    /// Remove a completed task
    pub fn remove_task(&self, service_id: u64, call_id: u64) -> bool {
        let task_id = TaskId::new(service_id, call_id);
        self.tasks.write().remove(&task_id).is_some()
    }

    /// Cleanup expired tasks
    /// Returns the number of tasks removed
    pub fn cleanup_expired(&self) -> usize {
        let mut tasks = self.tasks.write();
        let before = tasks.len();
        tasks.retain(|_, task| !task.is_expired());
        before - tasks.len()
    }

    /// Cleanup submitted tasks
    /// Returns the number of tasks removed
    pub fn cleanup_submitted(&self) -> usize {
        let mut tasks = self.tasks.write();
        let before = tasks.len();
        tasks.retain(|_, task| !task.submitted);
        before - tasks.len()
    }

    /// Cleanup both expired and submitted tasks
    /// Returns the number of tasks removed
    pub fn cleanup(&self) -> usize {
        let mut tasks = self.tasks.write();
        let before = tasks.len();
        tasks.retain(|_, task| !task.is_expired() && !task.submitted);
        before - tasks.len()
    }

    /// Get count of active tasks
    pub fn task_count(&self) -> usize {
        self.tasks.read().len()
    }

    /// Get count of tasks by status
    pub fn task_counts(&self) -> TaskCounts {
        let tasks = self.tasks.read();
        let mut counts = TaskCounts::default();

        for task in tasks.values() {
            counts.total += 1;
            if task.is_expired() {
                counts.expired += 1;
            } else if task.submitted {
                counts.submitted += 1;
            } else if task.threshold_met() {
                counts.ready += 1;
            } else {
                counts.pending += 1;
            }
        }

        counts
    }
}

/// Simplified task status for API responses
#[derive(Debug, Clone)]
pub struct TaskStatus {
    pub signatures_collected: usize,
    pub threshold_required: usize,
    pub threshold_type: ThresholdType,
    pub threshold_met: bool,
    pub signer_bitmap: U256,
    pub signed_stake_bps: u32,
    pub submitted: bool,
    pub is_expired: bool,
    pub time_remaining_secs: Option<u64>,
}

/// Task data ready for aggregation
#[derive(Debug)]
pub struct TaskForAggregation {
    pub service_id: u64,
    pub call_id: u64,
    pub output: Vec<u8>,
    pub signer_bitmap: U256,
    pub non_signer_indices: Vec<u32>,
    pub signatures: Vec<ArkBlsBn254Signature>,
    pub public_keys: Vec<ArkBlsBn254Public>,
}

/// Task count statistics
#[derive(Debug, Clone, Default)]
pub struct TaskCounts {
    pub total: usize,
    pub pending: usize,
    pub ready: usize,
    pub submitted: usize,
    pub expired: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{G1Affine, G2Affine};
    use ark_ec::AffineRepr;

    fn dummy_signature() -> ArkBlsBn254Signature {
        ArkBlsBn254Signature(G1Affine::generator())
    }

    fn dummy_public_key() -> ArkBlsBn254Public {
        ArkBlsBn254Public(G2Affine::generator())
    }

    #[test]
    fn test_task_state_new() {
        let state = TaskState::new(1, 100, vec![1, 2, 3], 5, 3);
        assert_eq!(state.service_id, 1);
        assert_eq!(state.call_id, 100);
        assert_eq!(state.output, vec![1, 2, 3]);
        assert_eq!(state.operator_count, 5);
        assert_eq!(state.threshold_type, ThresholdType::Count(3));
        assert_eq!(state.signer_bitmap, U256::ZERO);
        assert!(state.signatures.is_empty());
        assert!(state.public_keys.is_empty());
        assert!(!state.submitted);
        assert!(!state.is_expired());
    }

    #[test]
    fn test_task_state_add_signature() {
        let mut state = TaskState::new(1, 100, vec![], 5, 3);

        // Add first signature
        assert!(state.add_signature(0, dummy_signature(), dummy_public_key()).is_ok());
        assert!(state.has_signed(0));
        assert!(!state.has_signed(1));
        assert_eq!(state.signature_count(), 1);

        // Add second signature
        assert!(state.add_signature(2, dummy_signature(), dummy_public_key()).is_ok());
        assert!(state.has_signed(2));
        assert_eq!(state.signature_count(), 2);
    }

    #[test]
    fn test_task_state_duplicate_signature() {
        let mut state = TaskState::new(1, 100, vec![], 5, 3);

        assert!(state.add_signature(0, dummy_signature(), dummy_public_key()).is_ok());
        let result = state.add_signature(0, dummy_signature(), dummy_public_key());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Operator already signed");
    }

    #[test]
    fn test_task_state_out_of_bounds() {
        let mut state = TaskState::new(1, 100, vec![], 5, 3);

        let result = state.add_signature(5, dummy_signature(), dummy_public_key());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Operator index out of bounds");
    }

    #[test]
    fn test_task_state_threshold() {
        let mut state = TaskState::new(1, 100, vec![], 5, 3);

        assert!(!state.threshold_met());

        state.add_signature(0, dummy_signature(), dummy_public_key()).unwrap();
        assert!(!state.threshold_met());

        state.add_signature(1, dummy_signature(), dummy_public_key()).unwrap();
        assert!(!state.threshold_met());

        state.add_signature(2, dummy_signature(), dummy_public_key()).unwrap();
        assert!(state.threshold_met());
    }

    #[test]
    fn test_task_state_bitmap() {
        let mut state = TaskState::new(1, 100, vec![], 10, 3);

        state.add_signature(0, dummy_signature(), dummy_public_key()).unwrap();
        state.add_signature(3, dummy_signature(), dummy_public_key()).unwrap();
        state.add_signature(7, dummy_signature(), dummy_public_key()).unwrap();

        // Bitmap should be 0b10001001 = 137
        assert_eq!(state.signer_bitmap, U256::from(137));
    }

    #[test]
    fn test_task_state_non_signers() {
        let mut state = TaskState::new(1, 100, vec![], 5, 3);

        state.add_signature(0, dummy_signature(), dummy_public_key()).unwrap();
        state.add_signature(2, dummy_signature(), dummy_public_key()).unwrap();
        state.add_signature(4, dummy_signature(), dummy_public_key()).unwrap();

        let non_signers = state.get_non_signers();
        assert_eq!(non_signers, vec![1, 3]);

        let signers = state.get_signers();
        assert_eq!(signers, vec![0, 2, 4]);
    }

    #[test]
    fn test_task_state_stake_weighted() {
        let mut stakes = HashMap::new();
        stakes.insert(0, 1000); // 10%
        stakes.insert(1, 2000); // 20%
        stakes.insert(2, 3000); // 30%
        stakes.insert(3, 4000); // 40%

        let mut state = TaskState::with_config(
            1,
            100,
            vec![],
            4,
            ThresholdType::StakeWeighted(5000), // 50% required
            Some(stakes),
            None,
        );

        assert_eq!(state.total_stake, 10000);
        assert_eq!(state.signed_stake(), 0);
        assert_eq!(state.signed_stake_bps(), 0);
        assert!(!state.threshold_met());

        // Add operator 3 (40%)
        state.add_signature(3, dummy_signature(), dummy_public_key()).unwrap();
        assert_eq!(state.signed_stake(), 4000);
        assert_eq!(state.signed_stake_bps(), 4000);
        assert!(!state.threshold_met());

        // Add operator 1 (20%) -> now 60%
        state.add_signature(1, dummy_signature(), dummy_public_key()).unwrap();
        assert_eq!(state.signed_stake(), 6000);
        assert_eq!(state.signed_stake_bps(), 6000);
        assert!(state.threshold_met());
    }

    #[test]
    fn test_task_state_expiry() {
        let state = TaskState::with_config(
            1,
            100,
            vec![],
            5,
            ThresholdType::Count(3),
            None,
            Some(Duration::from_millis(50)), // Very short TTL
        );

        assert!(!state.is_expired());
        assert!(state.time_remaining().is_some());

        // Wait for expiry
        std::thread::sleep(Duration::from_millis(60));

        assert!(state.is_expired());
        assert!(state.time_remaining().is_none());
    }

    #[test]
    fn test_task_state_expired_signature_rejected() {
        let mut state = TaskState::with_config(
            1,
            100,
            vec![],
            5,
            ThresholdType::Count(3),
            None,
            Some(Duration::from_millis(10)),
        );

        // Wait for expiry
        std::thread::sleep(Duration::from_millis(20));

        let result = state.add_signature(0, dummy_signature(), dummy_public_key());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task has expired");
    }

    #[test]
    fn test_aggregation_state_init_task() {
        let state = AggregationState::new();

        assert!(state.init_task(1, 100, vec![1, 2, 3], 5, 3).is_ok());

        // Duplicate should fail
        let result = state.init_task(1, 100, vec![1, 2, 3], 5, 3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task already exists");
    }

    #[test]
    fn test_aggregation_state_submit_signature() {
        let state = AggregationState::new();
        state.init_task(1, 100, vec![], 5, 3).unwrap();

        let (count, threshold_met) = state
            .submit_signature(1, 100, 0, dummy_signature(), dummy_public_key())
            .unwrap();
        assert_eq!(count, 1);
        assert!(!threshold_met);

        let (count, threshold_met) = state
            .submit_signature(1, 100, 1, dummy_signature(), dummy_public_key())
            .unwrap();
        assert_eq!(count, 2);
        assert!(!threshold_met);

        let (count, threshold_met) = state
            .submit_signature(1, 100, 2, dummy_signature(), dummy_public_key())
            .unwrap();
        assert_eq!(count, 3);
        assert!(threshold_met);
    }

    #[test]
    fn test_aggregation_state_get_status() {
        let state = AggregationState::new();

        // Non-existent task
        assert!(state.get_status(1, 100).is_none());

        // Create task
        state.init_task(1, 100, vec![], 5, 3).unwrap();

        let status = state.get_status(1, 100).unwrap();
        assert_eq!(status.signatures_collected, 0);
        assert_eq!(status.threshold_required, 3);
        assert!(!status.threshold_met);
        assert!(!status.submitted);
        assert!(!status.is_expired);
    }

    #[test]
    fn test_aggregation_state_mark_submitted() {
        let state = AggregationState::new();
        state.init_task(1, 100, vec![], 5, 3).unwrap();

        assert!(state.mark_submitted(1, 100).is_ok());

        let status = state.get_status(1, 100).unwrap();
        assert!(status.submitted);

        // Can't submit signatures after marked submitted
        let result = state.submit_signature(1, 100, 0, dummy_signature(), dummy_public_key());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task already submitted to chain");
    }

    #[test]
    fn test_aggregation_state_get_for_aggregation() {
        let state = AggregationState::new();
        state.init_task(1, 100, vec![1, 2, 3], 5, 2).unwrap();

        // Not enough signatures
        assert!(state.get_for_aggregation(1, 100).is_none());

        // Add signatures to meet threshold
        state.submit_signature(1, 100, 0, dummy_signature(), dummy_public_key()).unwrap();
        state.submit_signature(1, 100, 1, dummy_signature(), dummy_public_key()).unwrap();

        let task = state.get_for_aggregation(1, 100).unwrap();
        assert_eq!(task.service_id, 1);
        assert_eq!(task.call_id, 100);
        assert_eq!(task.output, vec![1, 2, 3]);
        assert_eq!(task.signatures.len(), 2);
        assert_eq!(task.public_keys.len(), 2);
        assert_eq!(task.non_signer_indices, vec![2, 3, 4]);
    }

    #[test]
    fn test_aggregation_state_get_for_aggregation_submitted() {
        let state = AggregationState::new();
        state.init_task(1, 100, vec![], 5, 2).unwrap();
        state.submit_signature(1, 100, 0, dummy_signature(), dummy_public_key()).unwrap();
        state.submit_signature(1, 100, 1, dummy_signature(), dummy_public_key()).unwrap();

        // Should return aggregation data
        assert!(state.get_for_aggregation(1, 100).is_some());

        // Mark as submitted
        state.mark_submitted(1, 100).unwrap();

        // Should no longer return data after submission
        assert!(state.get_for_aggregation(1, 100).is_none());
    }

    #[test]
    fn test_aggregation_state_remove_task() {
        let state = AggregationState::new();
        state.init_task(1, 100, vec![], 5, 3).unwrap();

        assert!(state.get_status(1, 100).is_some());
        assert!(state.remove_task(1, 100));
        assert!(state.get_status(1, 100).is_none());

        // Removing non-existent task returns false
        assert!(!state.remove_task(1, 100));
    }

    #[test]
    fn test_multiple_tasks() {
        let state = AggregationState::new();

        // Create multiple tasks
        state.init_task(1, 100, vec![1], 5, 3).unwrap();
        state.init_task(1, 101, vec![2], 5, 3).unwrap();
        state.init_task(2, 100, vec![3], 5, 3).unwrap();

        // Each task is independent
        state.submit_signature(1, 100, 0, dummy_signature(), dummy_public_key()).unwrap();

        assert_eq!(state.get_status(1, 100).unwrap().signatures_collected, 1);
        assert_eq!(state.get_status(1, 101).unwrap().signatures_collected, 0);
        assert_eq!(state.get_status(2, 100).unwrap().signatures_collected, 0);
    }

    #[test]
    fn test_cleanup_expired() {
        let state = AggregationState::new();

        // Create expired task
        state.init_task_with_config(
            1,
            100,
            vec![],
            5,
            TaskConfig {
                threshold_type: ThresholdType::Count(3),
                ttl: Some(Duration::from_millis(10)),
                ..Default::default()
            },
        ).unwrap();

        // Create non-expired task
        state.init_task(1, 101, vec![], 5, 3).unwrap();

        assert_eq!(state.task_count(), 2);

        // Wait for expiry
        std::thread::sleep(Duration::from_millis(20));

        let removed = state.cleanup_expired();
        assert_eq!(removed, 1);
        assert_eq!(state.task_count(), 1);
        assert!(state.get_status(1, 101).is_some());
    }

    #[test]
    fn test_cleanup_submitted() {
        let state = AggregationState::new();

        state.init_task(1, 100, vec![], 5, 1).unwrap();
        state.init_task(1, 101, vec![], 5, 1).unwrap();

        state.submit_signature(1, 100, 0, dummy_signature(), dummy_public_key()).unwrap();
        state.mark_submitted(1, 100).unwrap();

        assert_eq!(state.task_count(), 2);

        let removed = state.cleanup_submitted();
        assert_eq!(removed, 1);
        assert_eq!(state.task_count(), 1);
        assert!(state.get_status(1, 101).is_some());
    }

    #[test]
    fn test_task_counts() {
        let state = AggregationState::new();

        // Pending task
        state.init_task(1, 100, vec![], 5, 3).unwrap();

        // Ready task (threshold met)
        state.init_task(1, 101, vec![], 5, 1).unwrap();
        state.submit_signature(1, 101, 0, dummy_signature(), dummy_public_key()).unwrap();

        // Submitted task
        state.init_task(1, 102, vec![], 5, 1).unwrap();
        state.submit_signature(1, 102, 0, dummy_signature(), dummy_public_key()).unwrap();
        state.mark_submitted(1, 102).unwrap();

        let counts = state.task_counts();
        assert_eq!(counts.total, 3);
        assert_eq!(counts.pending, 1);
        assert_eq!(counts.ready, 1);
        assert_eq!(counts.submitted, 1);
        assert_eq!(counts.expired, 0);
    }

    #[test]
    fn test_get_task_output() {
        let state = AggregationState::new();
        let output = vec![1, 2, 3, 4, 5];

        state.init_task(1, 100, output.clone(), 5, 3).unwrap();

        let retrieved = state.get_task_output(1, 100);
        assert_eq!(retrieved, Some(output));

        // Non-existent task
        assert!(state.get_task_output(1, 999).is_none());
    }
}
