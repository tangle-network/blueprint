use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;

use rocksdb::ColumnFamilyDescriptor;
use serde::{Deserialize, Serialize};

type MultiThreadedRocksDb = rocksdb::OptimisticTransactionDB<rocksdb::MultiThreaded>;

/// RocksDB instance this satisfies the [Store] interface.
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

/// RocksDb is used as the KV store.
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
