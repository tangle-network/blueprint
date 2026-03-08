# assets

## Purpose
Cloud-init configuration templates for bootstrapping Blueprint service VMs during cloud-hypervisor initialization. These static templates are embedded at compile-time via `include_str!()` and dynamically templated with runtime network parameters before being written to a FAT32 CIDATA disk image.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `meta-data` - Cloud-init instance metadata with a single `instance-id` field templated with `{{BLUEPRINT_INSTANCE_ID}}`.
  - **Key items**: `instance-id` field (required by cloud-init to detect first boot)
- `network-config` - Netplan v2 network configuration template for guest VM TAP interface setup.
  - **Key items**: `ens5` interface, `{{GUEST_IP_ADDRESS}}`, `{{TAP_IP_ADDRESS}}`, static IPv4 with /24 subnet, DNS 1.1.1.1/8.8.8.8
- `user-data` - Cloud-init user-data cloud-config script for OS provisioning and service launch.
  - **Key items**: disk_setup (`/dev/vdc` MBR), fs_setup (ext4 DATADISK), mounts (`/srv` read-only SERVICEDISK, `/mnt/data` DATADISK), `launch.service` systemd unit, Docker installation via `get.docker.com`, `modprobe virtiofs`

## Key APIs (no snippets)
- **Constants** (in parent `hypervisor/mod.rs`): `CLOUD_INIT_USER_DATA`, `CLOUD_INIT_NETWORK_CONFIG`, `CLOUD_INIT_META_DATA`
- **Functions**: `create_cloud_init_image()` performs template variable replacement and writes FAT32 CIDATA image

## Relationships
- **Depends on**: Nothing directly (static template files)
- **Used by**: `HypervisorInstance::prepare()` in parent `hypervisor/mod.rs` embeds these via `include_str!()`, templates them, and writes to FAT32 CIDATA disk
- **Data/control flow**:
  - Templates embedded at compile time as string constants
  - `create_cloud_init_image()` computes guest/TAP IPs from leased network subnet and performs string replacement
  - FAT32 image mounted by cloud-hypervisor as read-only second disk
  - Guest cloud-init processes files: applies network config, runs user-data commands, starts `launch.service`

## Notes
