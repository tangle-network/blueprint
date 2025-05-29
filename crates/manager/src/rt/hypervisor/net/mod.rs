mod manager;
pub use manager::{Lease, NetworkManager};

use nix::ioctl_write_ptr;
use nix::libc::{self, IFNAMSIZ, c_char, c_short};
use std::os::fd::AsRawFd;
use std::{fs, io};

const TUN_PATH: &str = "/dev/net/tun";
const INTERFACE_PREFIX: &str = "tap-tngl-";
const IFF_TAP: c_short = 0x0002;
const IFF_NO_PI: c_short = 0x1000;

ioctl_write_ptr!(tun_set_iff, b'T', 202, libc::ifreq);

/// Creates a TAP interface named `tap-tngl-{vm_id}` for use with a VM.
///
/// This returns the generated interface name.
///
/// # Errors
///
/// Returns an error if opening `/dev/net/tun` or setting the interface fails.
#[allow(clippy::cast_possible_wrap)]
pub fn create_tap_interface(vm_id: u32) -> io::Result<String> {
    let name = format!("{INTERFACE_PREFIX}{vm_id}");
    assert!(name.len() < IFNAMSIZ);

    let mut ifr_name: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    for (i, b) in name.bytes().enumerate() {
        ifr_name[i] = b as _;
    }

    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(TUN_PATH)?;

    let mut req = libc::ifreq {
        ifr_name,
        ifr_ifru: unsafe { std::mem::zeroed() },
    };
    req.ifr_ifru.ifru_flags = IFF_TAP | IFF_NO_PI;

    unsafe {
        tun_set_iff(file.as_raw_fd(), &req)?;
    }

    Ok(name)
}
