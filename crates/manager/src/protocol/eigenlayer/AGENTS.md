# eigenlayer

## Purpose
EigenLayer protocol integration for the blueprint manager. Provides EVM block polling, AVS registration management, and multi-AVS blueprint lifecycle orchestration.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations and re-exports: `EigenlayerProtocolClient`, `EigenlayerEventHandler`. Re-exports registration types from `blueprint-eigenlayer-extra` (`AvsRegistration`, `AvsRegistrationConfig`, `RegistrationStateManager`, `RegistrationStatus`).
- `client.rs` - `EigenlayerProtocolClient` struct. Polls EVM blocks via alloy `Provider`, tracks `last_block`, returns `ProtocolEvent::Eigenlayer` with block logs. Uses configurable `poll_interval` (default 12s). `next_event()` loops until a new block with logs is found.
- `event_handler.rs` - `EigenlayerEventHandler` struct managing multi-AVS lifecycle. `initialize()` starts background services (rewards monitoring hourly, slashing monitoring every 5min) and spawns blueprint processes for each active AVS registration. `handle_event()` ensures all registered AVS blueprints stay alive. `spawn_avs_blueprint()` builds source fetchers, configures runtime (native/hypervisor/container/TEE), passes EigenLayer contract addresses as CLI args, and starts the service process.

## Key APIs
- `EigenlayerProtocolClient::new(env, ctx)` -- connect to EVM RPC and begin polling
- `EigenlayerProtocolClient::next_event()` -- async poll for next block event
- `EigenlayerEventHandler::initialize()` -- start background services and spawn AVS blueprints
- `EigenlayerEventHandler::handle_event()` -- process an EigenLayer event, ensuring AVS processes are running

## Relationships
- Depends on `blueprint-eigenlayer-extra` for registration state and contract types
- Depends on `crate::sources` for `BlueprintSourceHandler`, `TestSourceFetcher`, `DynBlueprintSource`
- Depends on `crate::rt::service::Service` for process lifecycle (native, VM, container, TEE runtimes)
- Peer module to `crate::protocol::tangle` -- both implement protocol-specific event handling
