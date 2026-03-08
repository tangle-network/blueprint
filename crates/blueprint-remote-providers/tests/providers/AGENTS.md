# providers

## Purpose
Cloud provider integration tests for AWS, GCP, Azure, and DigitalOcean. Covers AWS instance type mapping, spot instance eligibility, and real-time pricing API validation across providers.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations for `aws_integration` and `pricing_api_tests`.
- `aws_integration.rs` - AWS-specific tests behind the `aws` feature flag: real AWS provisioning (ignored, requires credentials), provisioner creation, instance type mapping (`AwsInstanceMapper::map()` from `ResourceSpec` to EC2 type), and spot instance eligibility checks.
- `pricing_api_tests.rs` - Network-dependent pricing API tests (all `#[ignore]`): queries to instances.vantage.sh for AWS pricing, Azure retail pricing API, `PricingFetcher` integration across AWS/Azure/GCP, and cheapest provider selection across all providers for given resource specs.

## Key APIs
- `AwsProvisioner` / `ProvisioningConfig` - AWS EC2 instance provisioning
- `AwsInstanceMapper::map()` - maps `ResourceSpec` to AWS instance type with spot eligibility
- `PricingFetcher::find_best_instance()` - finds cheapest instance across providers given CPU, memory, and price constraints
- `InstanceInfo` - pricing query result with name, vCPUs, memory, and hourly price

## Relationships
- Depends on `blueprint_remote_providers` for provider implementations and pricing module
- AWS integration tests require `feature = "aws"` and real AWS credentials
- Pricing tests require network access and are marked `#[ignore]` for CI
