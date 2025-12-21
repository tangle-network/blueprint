use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_SNAPSHOT_RELATIVE: &str = "snapshots/localtestnet-state.json";

/// Return the default snapshot path bundled with this crate.
#[must_use]
pub fn default_snapshot_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_SNAPSHOT_RELATIVE)
}

pub fn snapshot_state_json() -> Option<String> {
    snapshot_state_json_from_path(&default_snapshot_path())
}

pub fn snapshot_state_json_from_path(path: &Path) -> Option<String> {
    match fs::read_to_string(path) {
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
    default_snapshot_path().exists()
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
