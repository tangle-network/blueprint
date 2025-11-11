use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;

use rocksdb::ColumnFamilyDescriptor;
use serde::{Deserialize, Serialize};

type MultiThreadedRocksDb = rocksdb::OptimisticTransactionDB<rocksdb::MultiThreaded>;

/// RocksDB instance
///
/// This is cheap to clone, as it uses an [`Arc`] internally.
#[derive(Debug, Clone)]
pub struct RocksDb {
    db: Arc<MultiThreadedRocksDb>,
}

impl std::ops::Deref for RocksDb {
    type Target = Arc<MultiThreadedRocksDb>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl RocksDb {
    pub fn open<P>(path: P, config: &RocksDbConfig) -> Result<Self, crate::Error>
    where
        P: AsRef<Path>,
    {
        let mut db_opts = rocksdb::Options::default();
        db_opts.create_if_missing(config.create_if_missing);
        db_opts.create_missing_column_families(config.create_missing_column_families);
        db_opts.increase_parallelism(config.parallelism);
        db_opts.set_write_buffer_size(config.write_buffer_size);
        db_opts.set_max_open_files(config.max_open_files);
        db_opts.set_allow_mmap_reads(true);
        db_opts.set_allow_mmap_writes(true);

        if let Some(max_background_jobs) = config.max_background_jobs {
            db_opts.set_max_background_jobs(max_background_jobs);
        }
        if let Some(compaction_style) = &config.compaction_style {
            db_opts.set_compaction_style(compaction_style_from_str(compaction_style)?);
        }
        if let Some(compression_type) = &config.compression_type {
            db_opts.set_compression_type(compression_type_from_str(compression_type)?);
        }
        if config.enable_statistics {
            db_opts.enable_statistics();
        };

        // Set the merge operator for the sequence column family
        let mut seq_cf_opts = db_opts.clone();
        seq_cf_opts.set_merge_operator_associative("add", adder_merge_operator);

        let db = MultiThreadedRocksDb::open_cf_descriptors(
            &db_opts,
            path,
            [
                ColumnFamilyDescriptor::new(cf::SEQ_CF, seq_cf_opts),
                ColumnFamilyDescriptor::new(cf::USER_TOKENS_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::TOKENS_OPTS_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::SERVICES_USER_KEYS_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::API_KEYS_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::API_KEYS_BY_ID_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::SERVICES_OAUTH_POLICY_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::OAUTH_JTI_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::OAUTH_RL_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::TLS_ASSETS_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::TLS_CERT_METADATA_CF, db_opts.clone()),
                ColumnFamilyDescriptor::new(cf::TLS_ISSUANCE_LOG_CF, db_opts.clone()),
            ],
        )?;
        Ok(Self { db: Arc::new(db) })
    }
}

/// Column family names
pub mod cf {
    /// Sequence column family (used to store sequence numbers)
    pub const SEQ_CF: &str = "seq";
    /// Tokens options column family (used to store the tokens options, like the token expiration time)
    pub const TOKENS_OPTS_CF: &str = "tkns_opts";
    /// Users' tokens column family (used to store the tokens of the users)
    pub const USER_TOKENS_CF: &str = "usr_tkns";
    /// Services column family (used to store the services with their user keys)
    pub const SERVICES_USER_KEYS_CF: &str = "svs_usr_keys";
    /// API keys column family (used to store long-lived API keys by key_id)
    pub const API_KEYS_CF: &str = "api_keys";
    /// API keys by ID column family (used to lookup API keys by database ID)
    pub const API_KEYS_BY_ID_CF: &str = "api_keys_by_id";
    /// OAuth per-service policy configuration
    pub const SERVICES_OAUTH_POLICY_CF: &str = "services_oauth_policy";
    /// OAuth assertion replay cache (jti -> exp)
    pub const OAUTH_JTI_CF: &str = "oauth_jti";
    /// OAuth token endpoint rate limit buckets
    pub const OAUTH_RL_CF: &str = "oauth_rl";
    /// TLS assets (encrypted certificates, keys, CA bundles)
    pub const TLS_ASSETS_CF: &str = "tls_assets";
    /// TLS certificate metadata (service_id, cert_id) -> metadata
    pub const TLS_CERT_METADATA_CF: &str = "tls_cert_metadata";
    /// TLS certificate issuance log (append-only for auditing)
    pub const TLS_ISSUANCE_LOG_CF: &str = "tls_issuance_log";
}

/// RocksDbConfig is used to configure RocksDb.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RocksDbConfig {
    pub create_if_missing: bool,
    pub create_missing_column_families: bool,
    pub parallelism: i32,
    pub write_buffer_size: usize,
    pub max_open_files: i32,
    pub max_background_jobs: Option<i32>,
    pub compression_type: Option<String>,
    pub compaction_style: Option<String>,
    pub enable_statistics: bool,
}

impl Default for RocksDbConfig {
    fn default() -> Self {
        Self {
            create_if_missing: true,
            create_missing_column_families: true,
            parallelism: std::thread::available_parallelism()
                .unwrap_or(NonZeroUsize::new(1).unwrap())
                .saturating_mul(NonZeroUsize::new(2).unwrap())
                .get() as i32,
            write_buffer_size: 256 * 1024 * 1024,
            max_open_files: 1024,
            max_background_jobs: None,
            compaction_style: None,
            compression_type: Some("none".into()),
            enable_statistics: false,
        }
    }
}

/// Converts string to a compaction style RocksDB variant.
pub(crate) fn compaction_style_from_str(
    s: &str,
) -> Result<rocksdb::DBCompactionStyle, crate::Error> {
    match s.to_lowercase().as_str() {
        "level" => Ok(rocksdb::DBCompactionStyle::Level),
        "universal" => Ok(rocksdb::DBCompactionStyle::Universal),
        "fifo" => Ok(rocksdb::DBCompactionStyle::Fifo),
        _ => Err(crate::Error::InvalidDBCompactionStyle(s.into())),
    }
}

/// Converts string to a compression type RocksDB variant.
pub(crate) fn compression_type_from_str(
    s: &str,
) -> Result<rocksdb::DBCompressionType, crate::Error> {
    match s.to_lowercase().as_str() {
        "bz2" => Ok(rocksdb::DBCompressionType::Bz2),
        "lz4" => Ok(rocksdb::DBCompressionType::Lz4),
        "lz4hc" => Ok(rocksdb::DBCompressionType::Lz4hc),
        "snappy" => Ok(rocksdb::DBCompressionType::Snappy),
        "zlib" => Ok(rocksdb::DBCompressionType::Zlib),
        "zstd" => Ok(rocksdb::DBCompressionType::Zstd),
        "none" => Ok(rocksdb::DBCompressionType::None),
        _ => Err(crate::Error::InvalidDBCompressionType(s.into())),
    }
}

#[allow(unused)]
fn concat_merge_operator(
    _key: &[u8],
    existing_value: Option<&[u8]>,
    operands: &rocksdb::merge_operator::MergeOperands,
) -> Option<Vec<u8>> {
    let mut result = existing_value.unwrap_or(&[]).to_vec();
    for operand in operands {
        result.extend_from_slice(operand);
    }
    Some(result)
}

/// merge operator that will add all values together
///
/// Note that it treats the values as u64 big endian encoded numbers.
///
/// This will wrap around if the value will overflow.
#[allow(unused)]
fn adder_merge_operator(
    _key: &[u8],
    existing_value: Option<&[u8]>,
    operands: &rocksdb::merge_operator::MergeOperands,
) -> Option<Vec<u8>> {
    let current_value = existing_value
        .and_then(|v| v.try_into().ok())
        .map(u64::from_be_bytes)
        .unwrap_or(0);
    let mut sum = current_value;
    for operand in operands {
        let v = operand.try_into().ok().map(u64::from_be_bytes).unwrap_or(0);
        // No overflow needed, we will wrap around back to 0
        sum = sum.wrapping_add(v);
    }
    let result = sum.to_be_bytes().to_vec();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocksdb_config_default() {
        let config = RocksDbConfig::default();

        assert!(config.create_if_missing);
        assert!(config.create_missing_column_families);
        assert_eq!(config.compression_type, Some("none".to_string()));
        assert!(!config.enable_statistics);

        // Check that parallelism is reasonable (should be at least 1)
        assert!(config.parallelism >= 1);
    }

    #[test]
    fn test_rocksdb_open() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let config = RocksDbConfig::default();

        let db_result = RocksDb::open(tmp_dir.path(), &config);
        assert!(db_result.is_ok());

        let db = db_result.unwrap();

        // Verify that DB opened successfully by testing a simple put/get
        let cf_names = vec![
            cf::SEQ_CF.to_string(),
            cf::USER_TOKENS_CF.to_string(),
            cf::TOKENS_OPTS_CF.to_string(),
            cf::SERVICES_USER_KEYS_CF.to_string(),
            cf::TLS_ASSETS_CF.to_string(),
            cf::TLS_CERT_METADATA_CF.to_string(),
            cf::TLS_ISSUANCE_LOG_CF.to_string(),
        ];

        // Check that we can get each column family
        for name in cf_names {
            let cf_handle = db.cf_handle(&name);
            assert!(cf_handle.is_some());
        }
    }

    #[test]
    fn test_rocksdb_compression_types() {
        // Test valid compression types
        assert!(compression_type_from_str("none").is_ok());
        assert!(compression_type_from_str("lz4").is_ok());
        assert!(compression_type_from_str("snappy").is_ok());
        assert!(compression_type_from_str("zlib").is_ok());
        assert!(compression_type_from_str("zstd").is_ok());
        assert!(compression_type_from_str("bz2").is_ok());
        assert!(compression_type_from_str("lz4hc").is_ok());

        // Case insensitivity
        assert!(compression_type_from_str("LZ4").is_ok());
        assert!(compression_type_from_str("Snappy").is_ok());

        // Invalid compression type
        let invalid = compression_type_from_str("invalid_compression");
        assert!(invalid.is_err());
        assert!(format!("{}", invalid.unwrap_err()).contains("Invalid"));
    }

    #[test]
    fn test_rocksdb_compaction_styles() {
        // Test valid compaction styles
        assert!(compaction_style_from_str("level").is_ok());
        assert!(compaction_style_from_str("universal").is_ok());
        assert!(compaction_style_from_str("fifo").is_ok());

        // Case insensitivity
        assert!(compaction_style_from_str("LEVEL").is_ok());
        assert!(compaction_style_from_str("Universal").is_ok());

        // Invalid compaction style
        let invalid = compaction_style_from_str("invalid_compaction");
        assert!(invalid.is_err());
        assert!(format!("{}", invalid.unwrap_err()).contains("Invalid"));
    }

    #[test]
    fn test_adder_merge_operator() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let config = RocksDbConfig::default();
        let db = RocksDb::open(tmp_dir.path(), &config).unwrap();

        let seq_cf = db.cf_handle(cf::SEQ_CF).unwrap();

        // Test merge operation with sequence counter
        let result = db.merge_cf(&seq_cf, b"test_counter", 1u64.to_be_bytes());
        assert!(result.is_ok());

        let result = db.merge_cf(&seq_cf, b"test_counter", 2u64.to_be_bytes());
        assert!(result.is_ok());

        let value = db.get_cf(&seq_cf, b"test_counter").unwrap();
        assert!(value.is_some());

        // Convert the value to u64, should be 3 (1 + 2)
        let bytes = value.unwrap();
        let mut value_bytes = [0u8; 8];
        value_bytes.copy_from_slice(&bytes);
        let value = u64::from_be_bytes(value_bytes);
        assert_eq!(value, 3);
    }
}
