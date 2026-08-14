//! Persistence layer for aggregation state
//!
//! This module provides traits and implementations for persisting aggregation state
//! across service restarts. The default in-memory implementation provides no persistence,
//! while optional backends can be enabled for production use.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_aggregation_svc::{
//!     persistence::{FilePersistence, PersistenceBackend},
//!     AggregationService,
//! };
//!
//! // Create file-based persistence
//! let persistence = FilePersistence::new("/var/lib/aggregation/state.json");
//!
//! // Create service with persistence
//! let service = AggregationService::with_persistence(config, persistence)?;
//! ```

use crate::state::{TaskState, ThresholdType};
use crate::types::{Bn254SignatureScheme, TaskId};
use alloy_primitives::U256;
use ark_bn254::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Error type for persistence operations
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Task not found
    #[error("Task not found: {0:?}")]
    NotFound(TaskId),
    /// Backend-specific error
    #[error("Backend error: {0}")]
    Backend(String),
}

/// Result type for persistence operations
pub type Result<T> = std::result::Result<T, PersistenceError>;

/// Serializable task state for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedTaskState {
    /// Service ID
    pub service_id: u64,
    /// Call ID
    pub call_id: u64,
    /// The output being signed
    #[serde(with = "hex_bytes")]
    pub output: Vec<u8>,
    /// Exact signed message.
    #[serde(default, with = "hex_bytes")]
    pub message: Vec<u8>,
    /// Hash-to-curve algorithm for the signed message.
    #[serde(default)]
    pub signature_scheme: Bn254SignatureScheme,
    /// Number of operators in the service
    pub operator_count: u32,
    /// Threshold type
    pub threshold_type: PersistedThresholdType,
    /// Bitmap of which operators have signed
    pub signer_bitmap: String, // U256 as hex string
    /// Collected signatures indexed by operator index (hex encoded)
    pub signatures: HashMap<u32, String>,
    /// Collected public keys indexed by operator index (hex encoded)
    pub public_keys: HashMap<u32, String>,
    /// Operator stakes for stake-weighted thresholds
    pub operator_stakes: HashMap<u32, u64>,
    /// Total stake of all operators
    pub total_stake: u64,
    /// Whether this task has been submitted to chain
    pub submitted: bool,
    /// When this task was submitted to chain (unix timestamp millis)
    #[serde(default)]
    pub submitted_at_ms: Option<u64>,
    /// When this task was created (unix timestamp millis)
    pub created_at_ms: u64,
    /// When this task expires (unix timestamp millis, None = never)
    pub expires_at_ms: Option<u64>,
}

/// Serializable threshold type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PersistedThresholdType {
    Count(u32),
    StakeWeighted(u32),
}

impl From<ThresholdType> for PersistedThresholdType {
    fn from(t: ThresholdType) -> Self {
        match t {
            ThresholdType::Count(n) => PersistedThresholdType::Count(n),
            ThresholdType::StakeWeighted(n) => PersistedThresholdType::StakeWeighted(n),
        }
    }
}

impl From<PersistedThresholdType> for ThresholdType {
    fn from(t: PersistedThresholdType) -> Self {
        match t {
            PersistedThresholdType::Count(n) => ThresholdType::Count(n),
            PersistedThresholdType::StakeWeighted(n) => ThresholdType::StakeWeighted(n),
        }
    }
}

impl TryFrom<&TaskState> for PersistedTaskState {
    type Error = PersistenceError;

    fn try_from(task: &TaskState) -> Result<Self> {
        let signatures = task
            .signatures
            .iter()
            .map(|(index, signature)| Ok((*index, encode_point(&signature.0)?)))
            .collect::<Result<HashMap<_, _>>>()?;
        let public_keys = task
            .public_keys
            .iter()
            .map(|(index, public_key)| Ok((*index, encode_point(&public_key.0)?)))
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self {
            service_id: task.service_id,
            call_id: task.call_id,
            output: task.output.clone(),
            message: task.message.clone(),
            signature_scheme: task.signature_scheme,
            operator_count: task.operator_count,
            threshold_type: task.threshold_type.into(),
            signer_bitmap: format!("{:#x}", task.signer_bitmap),
            signatures,
            public_keys,
            operator_stakes: task.operator_stakes.clone(),
            total_stake: task.total_stake,
            submitted: task.submitted,
            submitted_at_ms: task.submitted_at.map(instant_to_millis),
            created_at_ms: instant_to_millis(task.created_at),
            expires_at_ms: task.expires_at.map(instant_to_millis),
        })
    }
}

impl TryFrom<PersistedTaskState> for TaskState {
    type Error = PersistenceError;

    fn try_from(persisted: PersistedTaskState) -> Result<Self> {
        let signer_bitmap = parse_u256(&persisted.signer_bitmap)?;
        let threshold_type = persisted.threshold_type.into();
        let mut task = TaskState::with_message_config(
            persisted.service_id,
            persisted.call_id,
            persisted.output,
            persisted.message,
            persisted.signature_scheme,
            persisted.operator_count,
            threshold_type,
            Some(persisted.operator_stakes.clone()),
            None,
        );

        if task.total_stake != persisted.total_stake {
            return Err(PersistenceError::Serialization(format!(
                "task {}:{} has inconsistent total stake",
                task.service_id, task.call_id
            )));
        }

        if persisted.signatures.len() != persisted.public_keys.len() {
            return Err(PersistenceError::Serialization(format!(
                "task {}:{} has an incomplete signature/public-key pair",
                task.service_id, task.call_id
            )));
        }

        let mut indices: Vec<_> = persisted.signatures.keys().copied().collect();
        indices.sort_unstable();
        for index in indices {
            let signature = decode_signature(
                persisted
                    .signatures
                    .get(&index)
                    .ok_or_else(|| missing_entry("signature", index))?,
            )?;
            let public_key = decode_public_key(
                persisted
                    .public_keys
                    .get(&index)
                    .ok_or_else(|| missing_entry("public key", index))?,
            )?;
            task.add_signature(index, signature, public_key)
                .map_err(|error| PersistenceError::Serialization(error.to_string()))?;
        }

        if task.signer_bitmap != signer_bitmap {
            return Err(PersistenceError::Serialization(format!(
                "task {}:{} has an inconsistent signer bitmap",
                task.service_id, task.call_id
            )));
        }

        task.submitted = persisted.submitted;
        task.submitted_at = persisted
            .submitted_at_ms
            .map(millis_to_instant)
            .or_else(|| task.submitted.then_some(Instant::now()));
        task.created_at = millis_to_instant(persisted.created_at_ms);
        task.expires_at = persisted.expires_at_ms.map(millis_to_instant);
        Ok(task)
    }
}

fn encode_point(point: &impl CanonicalSerialize) -> Result<String> {
    let mut bytes = Vec::new();
    point
        .serialize_compressed(&mut bytes)
        .map_err(|error| PersistenceError::Serialization(error.to_string()))?;
    Ok(format!("0x{}", hex::encode(bytes)))
}

fn decode_bytes(value: &str) -> Result<Vec<u8>> {
    let value = value.strip_prefix("0x").unwrap_or(value);
    hex::decode(value).map_err(|error| PersistenceError::Serialization(error.to_string()))
}

fn decode_signature(value: &str) -> Result<blueprint_crypto_bn254::ArkBlsBn254Signature> {
    Ok(blueprint_crypto_bn254::ArkBlsBn254Signature(
        G1Affine::deserialize_compressed(&*decode_bytes(value)?)
            .map_err(|error| PersistenceError::Serialization(error.to_string()))?,
    ))
}

fn decode_public_key(value: &str) -> Result<blueprint_crypto_bn254::ArkBlsBn254Public> {
    Ok(blueprint_crypto_bn254::ArkBlsBn254Public(
        G2Affine::deserialize_compressed(&*decode_bytes(value)?)
            .map_err(|error| PersistenceError::Serialization(error.to_string()))?,
    ))
}

fn parse_u256(value: &str) -> Result<U256> {
    let value = value.strip_prefix("0x").unwrap_or(value);
    U256::from_str_radix(value, 16)
        .map_err(|error| PersistenceError::Serialization(error.to_string()))
}

fn missing_entry(kind: &str, index: u32) -> PersistenceError {
    PersistenceError::Serialization(format!("missing {kind} for operator {index}"))
}

fn instant_to_millis(instant: Instant) -> u64 {
    let now = Instant::now();
    let now_ms = now_millis();
    if instant >= now {
        now_ms.saturating_add(instant.duration_since(now).as_millis() as u64)
    } else {
        now_ms.saturating_sub(now.duration_since(instant).as_millis() as u64)
    }
}

fn millis_to_instant(timestamp_ms: u64) -> Instant {
    let now = Instant::now();
    let now_ms = now_millis();
    if timestamp_ms >= now_ms {
        now.checked_add(Duration::from_millis(timestamp_ms - now_ms))
            .unwrap_or(now)
    } else {
        now.checked_sub(Duration::from_millis(now_ms - timestamp_ms))
            .unwrap_or(now)
    }
}

/// Trait for persistence backends
///
/// Implement this trait to provide custom storage for aggregation state.
pub trait PersistenceBackend: Send + Sync {
    /// Save a task to persistent storage
    fn save_task(&self, task: &PersistedTaskState) -> Result<()>;

    /// Load a task from persistent storage
    fn load_task(&self, task_id: &TaskId) -> Result<Option<PersistedTaskState>>;

    /// Delete a task from persistent storage
    fn delete_task(&self, task_id: &TaskId) -> Result<()>;

    /// Load all tasks from persistent storage
    fn load_all_tasks(&self) -> Result<Vec<PersistedTaskState>>;

    /// Check if a task exists
    fn task_exists(&self, task_id: &TaskId) -> Result<bool> {
        Ok(self.load_task(task_id)?.is_some())
    }

    /// Flush any buffered writes (optional, default is no-op)
    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

/// No-op persistence backend (in-memory only)
///
/// This is the default backend that provides no persistence.
/// Tasks are lost on service restart.
#[derive(Debug, Clone, Default)]
pub struct NoPersistence;

impl PersistenceBackend for NoPersistence {
    fn save_task(&self, _task: &PersistedTaskState) -> Result<()> {
        Ok(())
    }

    fn load_task(&self, _task_id: &TaskId) -> Result<Option<PersistedTaskState>> {
        Ok(None)
    }

    fn delete_task(&self, _task_id: &TaskId) -> Result<()> {
        Ok(())
    }

    fn load_all_tasks(&self) -> Result<Vec<PersistedTaskState>> {
        Ok(Vec::new())
    }
}

/// File-based persistence backend
///
/// Stores all tasks in a single JSON file. Suitable for small deployments.
/// For high-throughput scenarios, consider using a database backend.
#[derive(Debug)]
pub struct FilePersistence {
    path: std::path::PathBuf,
    lock: parking_lot::RwLock<()>,
}

impl FilePersistence {
    /// Create a new file persistence backend
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            path: path.into(),
            lock: parking_lot::RwLock::new(()),
        }
    }

    fn read_all(&self) -> Result<HashMap<String, PersistedTaskState>> {
        let _guard = self.lock.read();

        if !self.path.exists() {
            return Ok(HashMap::new());
        }

        let contents = std::fs::read_to_string(&self.path)?;
        if contents.is_empty() {
            return Ok(HashMap::new());
        }

        serde_json::from_str(&contents).map_err(|e| PersistenceError::Serialization(e.to_string()))
    }

    fn write_all(&self, tasks: &HashMap<String, PersistedTaskState>) -> Result<()> {
        let _guard = self.lock.write();

        // Create parent directory if needed
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(tasks)
            .map_err(|e| PersistenceError::Serialization(e.to_string()))?;

        // Write to temp file first, then rename for atomicity
        let temp_path = self.path.with_extension("tmp");
        std::fs::write(&temp_path, contents)?;
        std::fs::rename(&temp_path, &self.path)?;

        Ok(())
    }

    fn task_key(task_id: &TaskId) -> String {
        format!("{}:{}", task_id.service_id, task_id.call_id)
    }
}

impl PersistenceBackend for FilePersistence {
    fn save_task(&self, task: &PersistedTaskState) -> Result<()> {
        let mut tasks = self.read_all()?;
        let key = Self::task_key(&TaskId::new(task.service_id, task.call_id));
        tasks.insert(key, task.clone());
        self.write_all(&tasks)
    }

    fn load_task(&self, task_id: &TaskId) -> Result<Option<PersistedTaskState>> {
        let tasks = self.read_all()?;
        let key = Self::task_key(task_id);
        Ok(tasks.get(&key).cloned())
    }

    fn delete_task(&self, task_id: &TaskId) -> Result<()> {
        let mut tasks = self.read_all()?;
        let key = Self::task_key(task_id);
        tasks.remove(&key);
        self.write_all(&tasks)
    }

    fn load_all_tasks(&self) -> Result<Vec<PersistedTaskState>> {
        let tasks = self.read_all()?;
        Ok(tasks.into_values().collect())
    }

    fn flush(&self) -> Result<()> {
        // File writes are already flushed on each operation
        Ok(())
    }
}

/// Helper to get current timestamp in milliseconds
pub fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Helper to convert timestamp to remaining duration
pub fn remaining_duration(expires_at_ms: Option<u64>) -> Option<Duration> {
    expires_at_ms.and_then(|expires| {
        let now = now_millis();
        if expires > now {
            Some(Duration::from_millis(expires - now))
        } else {
            None
        }
    })
}

/// Helper to check if expired
pub fn is_expired(expires_at_ms: Option<u64>) -> bool {
    expires_at_ms
        .map(|expires| now_millis() > expires)
        .unwrap_or(false)
}

/// Hex encoding for byte arrays in persistence
mod hex_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn sample_task() -> PersistedTaskState {
        PersistedTaskState {
            service_id: 1,
            call_id: 100,
            output: vec![1, 2, 3, 4],
            message: vec![5, 6, 7, 8],
            signature_scheme: Bn254SignatureScheme::TangleKeccak256,
            operator_count: 5,
            threshold_type: PersistedThresholdType::Count(3),
            signer_bitmap: "0x7".to_string(), // operators 0, 1, 2 signed
            signatures: HashMap::from([
                (0, "0xabc123".to_string()),
                (1, "0xdef456".to_string()),
                (2, "0x789abc".to_string()),
            ]),
            public_keys: HashMap::from([
                (0, "0xpk1".to_string()),
                (1, "0xpk2".to_string()),
                (2, "0xpk3".to_string()),
            ]),
            operator_stakes: HashMap::from([(0, 100), (1, 100), (2, 100), (3, 100), (4, 100)]),
            total_stake: 500,
            submitted: false,
            submitted_at_ms: None,
            created_at_ms: 1700000000000,
            expires_at_ms: Some(1700001000000),
        }
    }

    #[test]
    fn test_no_persistence() {
        let backend = NoPersistence;
        let task = sample_task();
        let task_id = TaskId::new(task.service_id, task.call_id);

        // Save should succeed but not persist
        assert!(backend.save_task(&task).is_ok());

        // Load should return None
        assert!(backend.load_task(&task_id).unwrap().is_none());

        // Delete should succeed
        assert!(backend.delete_task(&task_id).is_ok());

        // Load all should return empty
        assert!(backend.load_all_tasks().unwrap().is_empty());
    }

    #[test]
    fn test_file_persistence() {
        let temp_file = NamedTempFile::new().unwrap();
        let backend = FilePersistence::new(temp_file.path());

        let task = sample_task();
        let task_id = TaskId::new(task.service_id, task.call_id);

        // Save task
        backend.save_task(&task).unwrap();

        // Load task
        let loaded = backend.load_task(&task_id).unwrap().unwrap();
        assert_eq!(loaded.service_id, task.service_id);
        assert_eq!(loaded.call_id, task.call_id);
        assert_eq!(loaded.output, task.output);
        assert_eq!(loaded.operator_count, task.operator_count);
        assert_eq!(loaded.signatures.len(), 3);

        // Load all
        let all = backend.load_all_tasks().unwrap();
        assert_eq!(all.len(), 1);

        // Delete task
        backend.delete_task(&task_id).unwrap();
        assert!(backend.load_task(&task_id).unwrap().is_none());
    }

    #[test]
    fn test_file_persistence_multiple_tasks() {
        let temp_file = NamedTempFile::new().unwrap();
        let backend = FilePersistence::new(temp_file.path());

        // Create and save multiple tasks
        for i in 0..5 {
            let mut task = sample_task();
            task.call_id = 100 + i;
            backend.save_task(&task).unwrap();
        }

        let all = backend.load_all_tasks().unwrap();
        assert_eq!(all.len(), 5);

        // Delete one
        backend.delete_task(&TaskId::new(1, 102)).unwrap();
        let all = backend.load_all_tasks().unwrap();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_threshold_type_conversion() {
        let count = ThresholdType::Count(5);
        let persisted: PersistedThresholdType = count.into();
        let recovered: ThresholdType = persisted.into();
        assert_eq!(count, recovered);

        let stake = ThresholdType::StakeWeighted(6700);
        let persisted: PersistedThresholdType = stake.into();
        let recovered: ThresholdType = persisted.into();
        assert_eq!(stake, recovered);
    }

    #[test]
    fn test_time_helpers() {
        let now = now_millis();
        assert!(now > 0);

        // Not expired
        let future = Some(now + 10000);
        assert!(!is_expired(future));
        assert!(remaining_duration(future).is_some());

        // Expired
        let past = Some(now - 10000);
        assert!(is_expired(past));
        assert!(remaining_duration(past).is_none());

        // Never expires
        assert!(!is_expired(None));
        assert!(remaining_duration(None).is_none());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let task = sample_task();
        let json = serde_json::to_string(&task).unwrap();
        let recovered: PersistedTaskState = serde_json::from_str(&json).unwrap();

        assert_eq!(task.service_id, recovered.service_id);
        assert_eq!(task.call_id, recovered.call_id);
        assert_eq!(task.output, recovered.output);
    }
}
