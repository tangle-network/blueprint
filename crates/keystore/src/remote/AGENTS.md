# remote

## Purpose
Defines remote signing backend abstractions and implementations for hardware wallets and cloud KMS providers. Provides traits for remote key operations and concrete implementations for AWS KMS, GCP Cloud KMS, and Ledger hardware wallets.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Core trait definitions: `RemoteBackend` (capabilities discovery), `RemoteSigner<T>` (generic sign/get_public_key), `EcdsaRemoteSigner<T>` (ECDSA-specific with key ID lookup, iteration, and chain ID support), `RemoteOperations` (extensible operation dispatch). `RemoteConfig` enum with AWS/GCP/Ledger variants. `RemoteCapabilities` flags for signing, key generation, derivation, encryption.
- `aws.rs` - `AwsRemoteSigner` implementing `EcdsaRemoteSigner<K256Ecdsa>` via `alloy_signer_aws::AwsSigner`. Manages multiple AWS KMS keys indexed by `(key_id, chain_id)`. `AwsKeyConfig` specifies key ID, region, and optional chain ID.
- `gcp.rs` - `GcpRemoteSigner` implementing `EcdsaRemoteSigner<K256Ecdsa>` via `alloy_signer_gcp::GcpSigner`. Manages GCP Cloud KMS keys with project/location/keyring/key_name/version. `GcpKeyConfig` specifies full key path.
- `ledger.rs` - `LedgerRemoteSigner` implementing `EcdsaRemoteSigner<K256Ecdsa>` via `alloy_signer_ledger::LedgerSigner`. Supports HD path derivation (LedgerLive, Legacy, Other). Uses `Address` as key identifier instead of string key IDs. `HDPathWrapper` provides serde support for HD paths.

## Key APIs
- `RemoteConfig` - enum selecting AWS/GCP/Ledger backend with per-key configuration
- `EcdsaRemoteSigner<T>` trait - `build`, `get_public_key`, `iter_public_keys`, `get_key_id_from_public_key`, `sign_message_with_key_id`
- `RemoteSigner<T>` trait - simplified `get_public_key` and `sign` interface
- `AwsRemoteSigner`, `GcpRemoteSigner`, `LedgerRemoteSigner` - concrete implementations

## Relationships
- Used by `crate::keystore::backends::remote::RemoteEntry` to integrate into the `Keystore`
- Referenced by `crate::keystore::config::KeystoreConfig::remote()` for configuration
- All implementations use `blueprint_crypto::k256::K256Ecdsa` as the key type
- Feature-gated: `aws-signer`, `gcp-signer`, `ledger-browser`/`ledger-node`
