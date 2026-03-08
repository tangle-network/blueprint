# aws

## Purpose
AWS cloud provider implementation with EC2 instance provisioning, TEE (Trusted Execution Environment) support, and resource-to-instance-type mapping. Supports VM, EKS, and generic Kubernetes deployment targets.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Feature-gated module declaration; re-exports `AwsAdapter` and `AwsProvisioner`
- `adapter.rs` - `AwsAdapter` implementing `CloudProviderAdapter` trait; routes deployments to VM/EKS/K8s targets; uses SSH deployment for VMs
- `provisioner.rs` - `AwsProvisioner` managing EC2 lifecycle (create, terminate, status); handles AMI selection, security groups, key pairs
- `instance_mapper.rs` - `AwsInstanceMapper` mapping `ResourceSpec` to EC2 instance types with pricing API integration

## Key APIs (no snippets)
- `AwsAdapter::new()` - create adapter using AWS SDK credential chain
- `AwsAdapter::provision_instance(type, region, require_tee)` - provision an EC2 instance
- `AwsAdapter::deploy_blueprint_with_target(target, image, spec, env_vars)` - deploy to VM, EKS, or generic K8s
- `AwsProvisioner` - low-level EC2 operations (run_instances, terminate, describe)
- `AwsInstanceMapper` - map `ResourceSpec` to optimal instance type with cost estimation

## Relationships
- **Imports from**: `aws_sdk_ec2`, `aws_sdk_eks` (feature-gated), `providers/common` (`ProvisioningConfig`, `CloudProvisioner`), `deployment/ssh` (VM deployment), `shared` (K8s deployment helpers)
- **Used by**: `providers/mod.rs` re-exports; selected at runtime by provider resolution logic
- **Feature gates**: `aws` (core), `aws-eks` (EKS support), `kubernetes` (K8s deployment)
- **Env vars**: `AWS_KEY_PAIR_NAME`, `BLUEPRINT_REMOTE_TEE_REQUIRED`
