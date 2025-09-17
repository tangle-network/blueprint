# Module Consolidation Plan

## Current State: 23 Modules (CRITICAL)
- 19 top-level .rs files
- 4 subdirectories (deployment/, monitoring/, pricing/, providers/)

## Target State: 6 Logical Modules

### 1. **Core** (`src/core/`)
**Purpose:** Essential types and errors
**Consolidate:**
- `error.rs` → `core/error.rs`
- `resources.rs` → `core/resources.rs`
- `remote.rs` → `core/remote.rs`
- `test_utils.rs` → `core/test_utils.rs`

### 2. **Providers** (`src/providers/`) ✅ Already exists
**Purpose:** Cloud provider implementations
**Current:**
- `providers/aws/`
- `providers/azure/`
- `providers/gcp/`
- `providers/digitalocean/`
- `providers/vultr/`
- `providers/common/`

### 3. **Infrastructure** (`src/infra/`)
**Purpose:** Provisioning and deployment
**Consolidate:**
- `cloud_provisioner.rs` → `infra/provisioner.rs`
- `infrastructure.rs` → `infra/adapter.rs`
- `provisioning.rs` → `infra/mapper.rs`
- `auto_deployment.rs` → `infra/auto.rs`

### 4. **Integration** (`src/integration/`)
**Purpose:** Blueprint Manager integration
**Consolidate:**
- `auth_integration.rs` → `integration/auth.rs`
- `blueprint_extensions.rs` → `integration/extensions.rs`
- `heartbeat_integration.rs` → `integration/heartbeat.rs`
- `runtime_interface.rs` → `integration/runtime.rs`
- `service_classifier.rs` → `integration/classifier.rs`

### 5. **Network** (`src/network/`)
**Purpose:** Communication and security
**Consolidate:**
- `networking.rs` → `network/config.rs`
- `secure_bridge.rs` → `network/bridge.rs`
- `resilience.rs` → `network/resilience.rs`

### 6. **Observability** (`src/observability/`)
**Purpose:** Monitoring and testing
**Consolidate:**
- `observability.rs` → `observability/metrics.rs`
- `provider_api_tests.rs` → `observability/api_tests.rs`

## Benefits
- **76% reduction**: 23 → 6 modules
- **Logical grouping**: Clear separation of concerns
- **Easier navigation**: Intuitive module structure
- **Better testing**: Unit tests co-located with code