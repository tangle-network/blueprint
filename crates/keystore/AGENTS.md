# keystore

## Purpose
Flexible and secure keystore (`blueprint-keystore`). Manages cryptographic keys across multiple schemes (ECDSA/k256, Sr25519, Ed25519, BLS381, BN254), multiple protocols (Tangle, EigenLayer, Symbiotic, EVM), and multiple storage/signing backends (local filesystem, AWS KMS, GCP KMS, Ledger hardware wallets).

## Contents (one hop)
### Subdirectories
- [x] `src/` - Crate source: `keystore/` (core keystore logic with `backends/` for storage backends), `remote/` (AWS/GCP/Ledger remote signing), `storage/` (storage abstractions), plus `error.rs` and top-level re-exports

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-keystore`); core deps `blueprint-crypto`, `parking_lot`, `zeroize`, `blake3`; optional crypto primitives (`k256`, `schnorrkel`, `ed25519-zebra`, `tnt-bls`, `rust-bls-bn254`); optional EVM deps (Alloy signer stack); optional EigenLayer deps (`eigensdk`, ark curves); optional remote signers (`alloy-signer-aws`, `alloy-signer-gcp`, `alloy-signer-ledger`, `aws-config`, `aws-sdk-kms`, `gcloud-sdk`)
- `README.md` - Comprehensive feature documentation: crypto primitives, protocol features, remote signing, feature bundles, dependency matrix

## Key APIs (no snippets)
- `Keystore` (re-exported from `keystore` module) -- main keystore interface for key generation, storage, and retrieval
- `error` module -- keystore error types
- `storage` module -- storage backend abstractions
- `remote` module (gated by `cfg_remote!`) -- AWS KMS, GCP KMS, and Ledger signing adapters
- Re-exports `blueprint_crypto` as `crypto`

## Relationships
- Depends on `blueprint-crypto` for cryptographic primitives
- Used by nearly every crate that needs key management: `blueprint-manager`, `blueprint-runner`, `blueprint-eigenlayer-extra`, `blueprint-pricing-engine`, `blueprint-qos`, CLI
- Feature-gated protocol support: `tangle` (EVM), `eigenlayer` (EVM + BN254), `symbiotic` (EVM)
- Feature bundles: `eigenlayer-full`, `symbiotic-full`, `all-remote-signers`
