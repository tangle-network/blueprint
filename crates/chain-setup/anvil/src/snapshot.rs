use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SNAPSHOT_ENV: &str = "ANVIL_SNAPSHOT_PATH";
const DEFAULT_SNAPSHOT_RELATIVE: &str = "snapshots/localtestnet-state.json";

pub fn snapshot_state_json() -> Option<String> {
    let path = snapshot_path()?;
    match fs::read_to_string(&path) {
        Ok(data) => Some(data),
        Err(err) => {
            eprintln!(
                "warning: failed to read anvil snapshot {}: {err}",
                path.display()
            );
            None
        }
    }
}

pub fn snapshot_available() -> bool {
    snapshot_path().is_some()
}

#[cfg(test)]
mod tests {
    #[test]
    fn detects_snapshot() {
        assert!(
            super::snapshot_available(),
            "expected localtestnet snapshot to exist"
        );
    }
}

fn snapshot_path() -> Option<PathBuf> {
    if let Some(from_env) = env_snapshot_path() {
        return Some(from_env);
    }
    let default = Path::new(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_SNAPSHOT_RELATIVE);
    if default.exists() {
        Some(default)
    } else {
        None
    }
}

fn env_snapshot_path() -> Option<PathBuf> {
    let env_value = env::var_os(SNAPSHOT_ENV)?;
    let path = PathBuf::from(env_value);
    if path.exists() {
        Some(path)
    } else {
        eprintln!(
            "warning: ANVIL_SNAPSHOT_PATH={} does not exist",
            path.display()
        );
        None
    }
}
