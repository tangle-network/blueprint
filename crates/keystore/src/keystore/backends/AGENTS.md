# backends

## Purpose
Modular keystore backend implementations. Defines the core `Backend` trait for local key management and protocol-specific extension traits for BN254, EVM, EigenLayer, and remote signing.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `Backend` trait (13 methods: `generate`, `insert`, `sign_with_local`, `list_local`, `first_local`, `get_public_key_local`, `contains_local`, `remove`, `get_secret`, `get_storage_backends`, etc.). `BackendConfig` enum (`Local(Box<dyn RawStorage>)`, `Remote(RemoteConfig)`).
- `bn254.rs` - `Bn254Backend` trait for BLS on BN254 curve. Methods: `bls_bn254_generate_new`, `bls_bn254_sign`, `expose_bls_bn254_secret`, `iter_bls_bn254`. Implemented for `Keystore`. Feature-gated: `bn254`.
- `evm.rs` - `EvmBackend` trait for Ethereum wallet operations. Methods: `create_wallet_from_private_key`, `create_wallet_from_string_seed`, `get_address`, `create_wallet_from_mnemonic`, `create_wallet_from_mnemonic_with_path`. Uses alloy signer types. Feature-gated: `evm`.
- `eigenlayer.rs` - `EigenlayerBackend` trait extending `Bn254Backend` with K256 ECDSA key management. Methods: `ecdsa_generate_new`, `ecdsa_sign`, `expose_ecdsa_secret`, `iter_ecdsa`. Feature-gated: `eigenlayer`.
- `remote.rs` - `RemoteBackend` trait extending `Backend` with async remote signing via `EcdsaRemoteSigner`. Methods: `sign_with_remote`, `list_remote`. `RemoteEntry` holds config and capabilities. Feature-gated: `aws-signer`/`gcp-signer`/`ledger-*`.

## Key APIs
- `Backend` trait -- core local keystore operations (generate, sign, list, remove)
- `BackendConfig` enum -- selects local vs remote storage
- `Bn254Backend` / `EvmBackend` / `EigenlayerBackend` / `RemoteBackend` -- protocol-specific extensions
- All traits implemented for `Keystore`

## Relationships
- Depends on `blueprint-crypto` key types (`KeyType`, `ArkBlsBn254`, `K256Ecdsa`), `crate::storage::RawStorage`
- Used by `Keystore` as its operational interface
- Remote backend re-exports from `crate::remote` (AWS KMS, GCP, Ledger)
