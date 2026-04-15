//! `cargo-tangle dev status` — show whether the local devnet is up and where.

use std::fs;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;

use crate::workspace::{TangleWorkspace, WORKSPACE_FILE};

const DEV_DIR: &str = ".tangle/dev";

pub fn execute() -> Result<()> {
    let dev_dir = PathBuf::from(DEV_DIR);
    let ws_path = Path::new(WORKSPACE_FILE);

    if !dev_dir.exists() && !ws_path.exists() {
        println!("No devnet in this directory. Run `cargo-tangle dev up` to start one.");
        return Ok(());
    }

    if ws_path.exists() {
        match TangleWorkspace::load(ws_path) {
            Ok(ws) => {
                println!("Workspace:    {}", ws.source.display());
                println!("Active net:   {}", ws.active);
                if let Ok(net) = ws.active_network() {
                    println!("HTTP RPC:     {}", net.http_rpc_url);
                    println!("WS RPC:       {}", net.ws_rpc_url);
                    println!("Tangle:       {}", net.tangle_contract);
                }
                if let Some(p) = &ws.defaults.keystore_path {
                    println!("Keystore:     {}", p.display());
                }
            }
            Err(e) => eprintln!("⚠ {WORKSPACE_FILE} exists but failed to parse: {e}"),
        }
    }

    for (label, file) in [("anvil", "anvil.pid"), ("manager", "manager.pid")] {
        let path = dev_dir.join(file);
        match read_pid(&path) {
            Some(pid) if pid_alive(pid) => println!("{label:10}  running (PID {pid})"),
            Some(pid) => println!("{label:10}  stale PID {pid} (not running)"),
            None => println!("{label:10}  not started"),
        }
    }

    Ok(())
}

fn read_pid(path: &Path) -> Option<u32> {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
}

fn pid_alive(pid: u32) -> bool {
    use nix::sys::signal::kill;
    use nix::unistd::Pid;
    kill(Pid::from_raw(pid as i32), None).is_ok()
}
