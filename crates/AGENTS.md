# crates

## Purpose
Workspace root for the Tangle Network Blueprint SDK. Contains 40+ crates organized in a modular architecture for building blockchain services ("blueprints") on Tangle Network, EigenLayer, and EVM networks. Follows a meta-crate pattern where major functionality is exposed through aggregator crates that re-export component crates behind feature flags.

## Contents (one hop)
### Subdirectories
- [x] `auth/` - HTTP/WebSocket authentication: three-tier token model (API keys, PASETO, legacy), challenge-response verification (ECDSA/Sr25519/BN254), mTLS CA, OAuth 2.0, authenticated Axum reverse proxy, RocksDB persistence
- [x] `benchmarking/` - Runtime measurement primitives for blueprint performance benchmarking
- [x] `blueprint-faas/` - FaaS provider integrations: `FaasExecutor` trait with AWS Lambda, GCP Cloud Functions, Azure Functions, DigitalOcean Functions, and custom HTTP implementations
- [x] `blueprint-profiling/` - Automated job profiling (avg/p95/p99 duration, memory, CPU) with JSON/gzip/base64 serialization for on-chain storage
- [x] `blueprint-remote-providers/` - Multi-cloud infrastructure provisioning (AWS, GCP, Azure, DigitalOcean, Vultr, Kubernetes, Docker) with SSH deployment, health monitoring, pricing, and secure communication
- [x] `build-utils/` - Build script utilities for Solidity contract compilation via Foundry's `forge` and Soldeer dependency management
- [x] `chain-setup/` - Meta-crate for chain environment setup; currently provides Anvil (local EVM testnet) via feature flag
- [x] `clients/` - Meta-crate unifying network-specific clients: `BlueprintServicesClient` trait + Tangle, EigenLayer, and EVM client implementations
- [x] `contexts/` - Context provider traits bridging `BlueprintEnvironment` to protocol-specific clients (Tangle, EigenLayer, EVM, Keystore)
- [x] `core/` - Foundation crate: `Job` trait, `JobCall`/`JobResult` types, extraction traits (`FromJobCall`), extension traits, tracing macros; `no_std` compatible
- [x] `crypto/` - Meta-crate for cryptographic schemes: k256 (secp256k1), Sr25519, Ed25519, BLS12-377/381, BN254, hashing/KDF primitives
- [x] `eigenlayer-extra/` - EigenLayer-specific extensions: service context, sidecar management, AVS integration utilities
- [x] `evm-extra/` - EVM-specific extensions: event producers/consumers, log filters, extractors for Alloy-based chains
- [x] `keystore/` - Multi-backend key storage: filesystem, in-memory, and remote (AWS KMS, GCP) with hardware wallet support (Ledger)
- [x] `macros/` - Procedural macros: `#[job]`, `#[derive(FromRef)]`, `#[derive(BlueprintContext)]` with compile-time test validation
- [x] `manager/` - Blueprint Manager: service lifecycle management, protocol abstraction (Tangle/EigenLayer), binary fetching, runtime management (Native/Hypervisor/Container)
- [x] `metrics/` - Meta-crate for metrics instrumentation; currently provides RPC call metrics via feature flag
- [x] `networking/` - P2P networking layer built on libp2p with protocol extensions for signature aggregation gossip, gossip primitives, and round-based MPC
- [x] `pricing-engine/` - gRPC pricing server: EIP-712 signed quotes for service deployment and job execution, PoW anti-abuse, TOML config, benchmark-based pricing
- [x] `producers-extra/` - Additional job producer implementations for event-driven architectures
- [x] `qos/` - Quality of Service: heartbeat, metrics (Prometheus/OpenTelemetry), logging, managed Docker observability stack (Grafana, Loki)
- [x] `router/` - Job routing dispatch: maps `JobId` to async handlers with context provisioning, middleware support, and type-safe state management
- [x] `runner/` - Job execution engine: `BlueprintRunner` builder pattern, event loop consuming JobCalls through Router, multi-producer support
- [x] `sdk/` - Meta-crate aggregating 25+ crates into unified public API with 19 feature flags for selective compilation
- [x] `std/` - Standard library abstractions for `no_std`/`std` compatibility across the workspace
- [x] `stores/` - Meta-crate for storage backends; currently provides local JSON-based database via feature flag
- [x] `tangle-aggregation-svc/` - HTTP microservice for BLS signature aggregation: collects operator signatures, count/stake-weighted thresholds, on-chain-ready results
- [x] `tangle-extra/` - Tangle-specific extensions: `TangleProducer`/`TangleConsumer`, aggregation workflows, lifecycle keepers, metadata extractors
- [x] `tee/` - Trusted Execution Environment support: attestation (TDX/Nitro/SEV-SNP/Azure/GCP), X25519 key exchange, ChaCha20-Poly1305 secret sealing, Tower middleware
- [x] `testing-utils/` - Test utilities: Anvil harness, core test helpers, EigenLayer test infrastructure
- [x] `webhooks/` - HTTP webhook trigger system for blueprint event notifications
- [x] `x402/` - x402 payment gateway integration for cross-chain settlement

### Files
- `README.md` - Workspace-level documentation for the crates directory.

## Key APIs (no snippets)
- **Core**: `Job` trait, `JobCall`, `JobResult`, `FromJobCall`/`FromJobCallParts` extraction traits
- **Router**: `Router::route(job_id, handler)`, `Router::with_context(ctx)`, middleware layers
- **Runner**: `BlueprintRunner::builder()`, `.router()`, `.producer()`, `.run()`
- **Clients**: `BlueprintServicesClient` trait, `TangleClient`, `EigenlayerClient`, `InstrumentedClient`
- **Crypto**: `KeyType` trait, `K256Ecdsa`, `SchnorrkelSr25519`, `ArkBlsBn254`, `AggregatableSignature`
- **SDK**: Unified re-exports via `blueprint_sdk::*` with 19 feature flags

## Relationships
- **Architecture layers** (bottom-up):
  1. `std` + `core` + `crypto` - Foundation primitives
  2. `keystore` + `clients` + `networking` - Infrastructure services
  3. `router` + `runner` + `contexts` - Execution framework
  4. `macros` + `sdk` - Developer-facing API
  5. `manager` + `remote-providers` - Deployment and operations
- **Data/control flow**: Job submissions flow through Producer -> Runner -> Router -> Job handler -> Consumer -> chain submission
- **Multi-network**: Tangle (primary), EigenLayer, and generic EVM supported through feature-gated client/context/extra crates

## Notes
- Project is in alpha stage; APIs are evolving
- Rust 2024 edition, workspace resolver v3
- Rustfmt CI pinned to `nightly-2026-02-24`
- Several crates require serial test execution (see CLAUDE.md)
