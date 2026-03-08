# hypervisor

## Purpose
Cloud Hypervisor VM orchestration for running Blueprint services in isolated virtual machines. Manages the full VM lifecycle: process spawning, disk image creation (FAT32 service disk, cloud-init CIDATA, QCOW2 data disk), network lease allocation, VM configuration and boot, and graceful shutdown with nftables cleanup.

## Contents (one hop)
### Subdirectories
- [x] `assets/` - Cloud-init configuration templates (user-data, network-config, meta-data) embedded at compile-time and templated with runtime network/instance parameters for guest VM provisioning.
- [x] `net/` - Network infrastructure for VM sandboxes: IP address pool allocation via `NetworkManager`, nftables firewall rule orchestration for VM isolation/NAT, and TAP interface readiness polling.

### Files
- `mod.rs` - Core VM orchestration module.
  - `HypervisorInstance` manages the full lifecycle: spawns `cloud-hypervisor` process, connects via Unix socket API, creates FAT32 disk images (service binary + keystore + launcher script), generates cloud-init CIDATA from embedded templates, creates QCOW2 data disks via `qemu-img`, allocates network leases, configures VM (memory, payload, disks, serial, vsock, TAP networking), boots, sets up nftables rules, and performs graceful shutdown with fallback kill.
  - `ServiceVmConfig` holds VM ID and PTY toggle.
  - Helper types `CopiedEntry`, `Directory`, `CopiedFile`, `FileSource`, `FatFsConfig` and function `new_fat_fs()` for FAT32 image creation using `fatfs` crate.
- `images.rs` - Ubuntu cloud image management. `CloudImage::fetch()` downloads Ubuntu 24.04 server cloud image, vmlinuz, and initrd to a cache directory if missing, converts QCOW2 to raw via `qemu-img convert`, and resizes to 20GB (`ALLOCATED_IMAGE_SIZE`).

## Key APIs (no snippets)
- **Types**: `HypervisorInstance`, `ServiceVmConfig`, `CloudImage`
- **Functions**: `HypervisorInstance::new(ctx, limits, config, cache_dir, runtime_dir, service_name)`, `.prepare()` (disk setup + VM create), `.start()` (boot + TAP + nftables), `.shutdown()` (graceful + cleanup), `.status()`, `.client()` (socket API), `.pty()`, `CloudImage::fetch()`, `download_image_if_needed()`

## Relationships
- **Depends on**: `cloud-hypervisor-client` (socket-based API for VM lifecycle), `fatfs` (FAT32 image creation), `walkdir` (keystore traversal), `reqwest` (image download), `qemu-img` (external tool for disk conversion/creation), `crate::config::BlueprintManagerContext` (VM network config), `crate::rt::ResourceLimits`, `crate::sources` (BlueprintEnvVars, BlueprintArgs), hypervisor `net` submodule (NetworkManager, nftables, TAP)
- **Used by**: Blueprint manager runtime when `vm-sandbox` feature is enabled; service orchestration layer creates `HypervisorInstance` per managed blueprint service
