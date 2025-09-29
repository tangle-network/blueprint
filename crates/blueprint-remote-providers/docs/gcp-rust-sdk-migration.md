# Google Cloud Rust SDK Migration Plan

## Overview

Google has released an official Rust SDK: https://github.com/googleapis/google-cloud-rust

This is a major improvement over our current manual HTTP client approach. We should migrate to use this official SDK for better testing, authentication, and reliability.

## Current State

Currently using manual HTTP client with `reqwest` for GCP operations:
- Manual API endpoint construction
- Custom JSON request/response handling
- Manual authentication via service account keys
- Heavy mocking in tests via `mockito`

## Target State

Migrate to official Google Cloud Rust SDK:
- SDK-native API calls with typed requests/responses
- Built-in authentication providers
- SDK test utilities instead of manual mocking
- Better error handling and retry logic

## Migration Steps

### 1. Add Dependencies

```toml
[dependencies]
# Core GCP services we use
google-cloud-compute = "0.1"
google-cloud-auth = "0.1"
google-cloud-storage = "0.1"
google-cloud-logging = "0.1"

[dev-dependencies]
# SDK test utilities
google-cloud-test-utils = "0.1"
```

### 2. Replace GCP Provisioner

#### Before (crates/blueprint-remote-providers/src/providers/gcp/provisioner.rs):
```rust
// Manual HTTP client
pub struct GcpProvisioner {
    client: reqwest::Client,
    project_id: String,
    service_account_key: String,
}

impl GcpProvisioner {
    pub async fn provision_instance(&self, spec: &ResourceSpec) -> Result<ProvisionedInstance> {
        let url = format!("https://compute.googleapis.com/compute/v1/projects/{}/zones/{}/instances",
            self.project_id, zone);

        let request = serde_json::json!({
            "name": instance_name,
            "machineType": machine_type,
            // ... manual JSON construction
        });

        let response = self.client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&request)
            .send()
            .await?;
        // ... manual response parsing
    }
}
```

#### After:
```rust
use google_cloud_compute::{
    client::{Client, ClientConfig},
    instances::{InstancesClient, InsertInstanceRequest},
};

pub struct GcpProvisioner {
    client: InstancesClient,
    project_id: String,
}

impl GcpProvisioner {
    pub async fn new() -> Result<Self> {
        let config = ClientConfig::default()
            .with_auth_provider(DefaultTokenSourceProvider::new().await?);

        let client = Client::new(config).await?;
        let instances_client = client.instances();

        Ok(Self {
            client: instances_client,
            project_id: std::env::var("GCP_PROJECT_ID")?,
        })
    }

    pub async fn provision_instance(&self, spec: &ResourceSpec) -> Result<ProvisionedInstance> {
        let instance = Instance::builder()
            .name(instance_name)
            .machine_type(machine_type)
            .disks(vec![disk])
            .network_interfaces(vec![network_interface])
            .build();

        let request = InsertInstanceRequest::builder()
            .project(&self.project_id)
            .zone(zone)
            .instance_resource(instance)
            .build()?;

        let operation = self.client.insert(request).await?;

        // Wait for operation completion
        let instance = self.wait_for_instance_ready(&operation).await?;

        Ok(ProvisionedInstance::from_gcp_instance(instance))
    }
}
```

### 3. Update Authentication

#### Before:
```rust
// Manual service account key handling
let service_account_key = std::env::var("GCP_SERVICE_ACCOUNT_KEY")?;
let token = self.get_access_token(&service_account_key).await?;
```

#### After:
```rust
// SDK handles authentication automatically
use google_cloud_auth::credentials::{
    ApplicationDefaultCredentialsFlow,
    ServiceAccountFlow,
};

let auth_provider = if let Ok(key_path) = std::env::var("GOOGLE_APPLICATION_CREDENTIALS") {
    ServiceAccountFlow::from_file(key_path).await?
} else {
    ApplicationDefaultCredentialsFlow::new().await?
};
```

### 4. Improve Testing

#### Before (tests/providers/gcp_integration.rs):
```rust
// Heavy mocking with mockito
#[tokio::test]
async fn test_gcp_provisioning() {
    let mut server = mockito::Server::new_async().await;

    let mock = server.mock("POST", "/compute/v1/projects/test/zones/us-central1-a/instances")
        .with_status(200)
        .with_body(r#"{"name": "test-instance"}"#)
        .create_async()
        .await;

    // Test with mocked responses...
}
```

#### After:
```rust
// Use SDK test utilities
#[tokio::test]
async fn test_gcp_provisioning_with_sdk() {
    use google_cloud_test_utils::{TestClient, TestConfig};

    let test_config = TestConfig::default()
        .with_project_id("test-project")
        .with_mock_responses(vec![
            MockResponse::for_insert_instance()
                .with_instance_id("test-instance-123")
                .with_status(Status::Running),
        ]);

    let client = TestClient::new(test_config).await?;

    // Test with SDK's built-in test harness
    let provisioner = GcpProvisioner::new_with_client(client).await?;
    let instance = provisioner.provision_instance(&spec).await?;

    assert_eq!(instance.id, "test-instance-123");
}
```

### 5. Blueprint-Centric Real Testing

```rust
#[tokio::test]
#[ignore] // Real GCP test
async fn test_real_gcp_blueprint_deployment() {
    // Use real GCP SDK with blueprint
    let mut blueprint_ctx = BlueprintTestContext::new().await?;
    blueprint_ctx.start_blueprint().await?;

    // Get real resource requirements
    let resource_usage = blueprint_ctx.get_real_resource_usage().await?;

    // Use real GCP SDK (not mocked)
    let provisioner = GcpProvisioner::new().await?;

    // Provision smallest instance possible for test
    let spec = ResourceSpec {
        cpu: 0.25, // e2-micro
        memory_gb: 1.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: true, // Use preemptible for cost
        qos: Default::default(),
    };

    if std::env::var("REAL_GCP_TEST").is_ok() {
        let instance = provisioner.provision("us-central1-a", &spec).await?;

        // Test blueprint deployment to real instance
        let ssh_client = create_ssh_client(&instance).await?;
        let deployment = deploy_blueprint_to_instance(&ssh_client, &blueprint_ctx).await?;

        // Verify real deployment
        assert!(verify_blueprint_running(&deployment).await?);

        // CRITICAL: Cleanup
        provisioner.terminate(&instance.id).await?;
    }
}
```

## Implementation Priority

### Phase 1: Infrastructure (Week 1)
1. Add google-cloud-rust dependencies
2. Create new GcpProvisioner with SDK
3. Update authentication to use SDK providers
4. Migrate basic instance operations (create, delete, status)

### Phase 2: Features (Week 2)
5. Migrate networking (VPC, firewall rules)
6. Migrate storage (persistent disks)
7. Add Google Cloud Logging integration
8. Update pricing integration

### Phase 3: Testing (Week 3)
9. Replace all mockito tests with SDK test utilities
10. Add blueprint-centric real tests
11. Add cost-controlled real GCP tests
12. Performance and reliability testing

## Benefits

### Developer Experience
- **Type Safety**: SDK provides typed requests/responses vs manual JSON
- **Better Errors**: SDK error types vs generic HTTP errors
- **Auto-completion**: IDE support for SDK methods
- **Documentation**: SDK docs integrated with code

### Testing Quality
- **Realistic Testing**: SDK test utilities closer to real API behavior
- **Less Brittle**: No manual HTTP mocking that breaks on API changes
- **Blueprint-Centric**: All tests use real blueprint binaries
- **Real API Testing**: Optional real GCP tests with automatic cleanup

### Production Reliability
- **Built-in Retry**: SDK handles transient failures automatically
- **Rate Limiting**: SDK respects API rate limits
- **Authentication**: Robust auth with automatic token refresh
- **Error Handling**: Proper error categorization and handling

## Rollout Strategy

1. **Parallel Implementation**: Build new SDK-based GCP provider alongside existing
2. **Feature Parity**: Ensure new implementation matches all current features
3. **A/B Testing**: Test both implementations in parallel
4. **Gradual Migration**: Switch tests one by one to new implementation
5. **Full Cutover**: Remove old HTTP-based implementation once fully validated

## Risk Mitigation

- **SDK Maturity**: Google Cloud Rust SDK is new - validate stability
- **Breaking Changes**: Pin SDK versions, test upgrades carefully
- **Performance**: Measure SDK overhead vs direct HTTP calls
- **Cost**: Real GCP tests must have strong cost controls and cleanup
- **Rollback Plan**: Keep old implementation until new one is proven

## Success Metrics

- ✅ All GCP tests pass with SDK implementation
- ✅ 100% test coverage using real blueprint binaries
- ✅ Real GCP integration tests with automatic cleanup
- ✅ Improved error handling and retry behavior
- ✅ Better developer experience with type safety
- ✅ No regression in functionality or performance