//! `cargo-tangle dev down` — stop the local devnet started by `dev up`.

use std::fs;
use std::path::{Path, PathBuf};

use clap::Args;
use color_eyre::eyre::{Result, bail};

use crate::workspace::WORKSPACE_FILE;

const DEV_DIR: &str = ".tangle/dev";

#[derive(Args, Debug)]
pub struct DownArgs {
    /// Keep `.tangle.toml` in place; only stop processes and clear `.tangle/dev/`.
    #[arg(long)]
    pub keep_workspace: bool,
}

pub fn execute(args: DownArgs) -> Result<()> {
    let dev_dir = PathBuf::from(DEV_DIR);

    if !dev_dir.exists() && !Path::new(WORKSPACE_FILE).exists() {
        println!("No devnet found in this directory.");
        return Ok(());
    }

    let mut stopped = 0usize;
    for (label, file) in [("anvil", "anvil.pid"), ("manager", "manager.pid")] {
        let path = dev_dir.join(file);
        if let Some(pid) = read_pid(&path) {
            if pid_alive(pid) {
                if let Err(e) = kill_pid(pid) {
                    eprintln!("⚠ failed to stop {label} ({pid}): {e}");
                } else {
                    println!("✓ stopped {label} (PID {pid})");
                    stopped += 1;
                }
            }
        }
    }

    if dev_dir.exists() {
        fs::remove_dir_all(&dev_dir)?;
        println!("✓ removed {}", dev_dir.display());
        // If `.tangle/` was empty after removing dev/, drop it too.
        if let Some(parent) = dev_dir.parent() {
            if parent.as_os_str() != "" && parent.is_dir() {
                if let Ok(mut iter) = fs::read_dir(parent) {
                    if iter.next().is_none() {
                        let _ = fs::remove_dir(parent);
                    }
                }
            }
        }
    }

    if !args.keep_workspace && Path::new(WORKSPACE_FILE).exists() {
        // Only remove workspaces we can prove we created (active = "local").
        if workspace_is_dev() {
            fs::remove_file(WORKSPACE_FILE)?;
            println!("✓ removed {WORKSPACE_FILE}");
        } else {
            println!(
                "  leaving {WORKSPACE_FILE} (active network is not 'local'; use --keep-workspace to silence this)"
            );
        }
    }

    if stopped == 0 && dev_dir.exists() {
        bail!("no running anvil/manager found (already stopped?)");
    }

    println!("Devnet is down.");
    Ok(())
}

fn workspace_is_dev() -> bool {
    let Ok(s) = fs::read_to_string(WORKSPACE_FILE) else {
        return false;
    };
    // Crude but correct: we wrote `network = "local"` with a `[networks.local]` block and
    // a `127.0.0.1:8545` URL. Don't delete someone else's .tangle.toml.
    s.contains("network = \"local\"") && s.contains("http://127.0.0.1:")
}

fn read_pid(path: &Path) -> Option<u32> {
    fs::read_to_string(path).ok().and_then(|s| s.trim().parse().ok())
}

fn pid_alive(pid: u32) -> bool {
    use nix::sys::signal::kill;
    use nix::unistd::Pid;
    kill(Pid::from_raw(pid as i32), None).is_ok()
}

fn kill_pid(pid: u32) -> Result<()> {
    use nix::errno::Errno;
    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;
    match kill(Pid::from_raw(pid as i32), Signal::SIGTERM) {
        Ok(()) | Err(Errno::ESRCH) => Ok(()),
        Err(e) => Err(color_eyre::eyre::eyre!("SIGTERM to {pid} failed: {e}")),
    }
}
