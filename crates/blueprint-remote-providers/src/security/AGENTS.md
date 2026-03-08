# security

## Purpose
Secure credential storage, cloud authentication helpers, and a hardened HTTP client for remote provider API interactions. Replaces plaintext credential handling with AES-GCM encryption and provides domain-allowlisted HTTPS-only requests.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports: `EncryptedCloudCredentials`, `PlaintextCredentials`, `SecureCredentialManager`, `ApiAuthentication`, `SecureHttpClient`.
- `auth.rs` - Cloud authentication helpers. `gcp_access_token()` fetches from `GCP_ACCESS_TOKEN` env var or GCP metadata service. `azure_access_token()` fetches from `AZURE_ACCESS_TOKEN` env var, Azure IMDS, or falls back to Azure CLI. URL validation for metadata endpoints (host and scheme checks).
- `encrypted_credentials.rs` - AES-256-GCM encrypted credential storage. `EncryptedCloudCredentials` (encrypt/decrypt with 32-byte key, random nonce, non-sensitive metadata). `PlaintextCredentials` (fields for AWS, GCP, Azure, DigitalOcean, Vultr; implements `Zeroize`/`ZeroizeOnDrop`; provider-specific accessor methods). `SecureCredentialManager` with BLAKE3 key derivation from password+salt, store/retrieve/validate operations.
- `secure_http_client.rs` - `SecureHttpClient` enforcing HTTPS-only, domain allowlist (AWS, GCP, Azure, DO, K8s endpoints), certificate pinning stubs, header injection detection, request/response size limits, and request ID tagging. `ApiAuthentication` enum: Bearer, ApiKey, AwsSignatureV4, None. Convenience constructors for each cloud provider. Methods: `get()`, `post()`, `post_json()`, `delete()`.

## Key APIs
- `gcp_access_token()` / `azure_access_token()` - async cloud token acquisition with fallback chains
- `EncryptedCloudCredentials::encrypt_with_key()` / `decrypt()` - AES-GCM credential encryption
- `SecureCredentialManager::new(password, salt)` - key-derived credential manager
- `PlaintextCredentials::aws_credentials()` / `gcp_credentials()` / `azure_credentials()` - typed credential accessors
- `SecureHttpClient::authenticated_request(method, url, auth, body)` - hardened HTTP with auth and validation
- `ApiAuthentication::digitalocean()` / `aws()` / `google_cloud()` / `azure()` - auth constructors

## Relationships
- `auth` is used by `shared/security` for Azure NSG management authentication
- `encrypted_credentials` replaces plaintext credential passing in `infra/` provisioner and auto-deployment flows
- `SecureHttpClient` is intended to replace raw `reqwest` usage across provider adapters
- `PlaintextCredentials` maps to the same provider set as `core/remote::CloudProvider`
