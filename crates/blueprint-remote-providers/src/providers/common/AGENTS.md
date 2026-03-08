# common

## Purpose
Common provisioning abstractions shared by all cloud provider implementations. Defines the core traits and data structures for instance selection, provisioning configuration, and provisioned infrastructure results.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Single file (~125 lines) containing all shared types and the `CloudProvisioner` trait

## Key APIs (no snippets)
- `CloudProvisioner` trait - async trait with `provision_instance(spec, config)` and `terminate_instance(id)` methods
- `InstanceSelection` - instance type string, spot capability flag, optional estimated hourly cost
- `ProvisioningConfig` - name, region, ssh_key_name, ami_id, machine_image, custom_config HashMap
- `ProvisionedInfrastructure` - provider, instance_id, public/private IP, region, instance_type, metadata; has `is_ready()` health check, `get_endpoint()`, and `into_provisioned_instance()` conversion

## Relationships
- **Imports from**: `crate::core::remote::CloudProvider`, `crate::core::resources::ResourceSpec`
- **Used by**: All 5 provider implementations (AWS, Azure, GCP, DigitalOcean, Vultr) use these types for their provisioner and adapter implementations
