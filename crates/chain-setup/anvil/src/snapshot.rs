use std::fs;
use std::path::{Path, PathBuf};

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
    let default = Path::new(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_SNAPSHOT_RELATIVE);
    if default.exists() {
        Some(default)
    } else {
        None
    }
}
