//! Registration helpers for blueprints.
//!
//! These utilities make it easier to participate in the preregistration flow by
//! writing TLV payloads to the location expected by the runner.

use crate::runner::config::BlueprintEnvironment;
use std::path::PathBuf;
use tokio::fs;

/// Persist a blueprint-specific registration payload to the agreed upon path.
///
/// When [`BlueprintEnvironment::registration_mode`] is enabled, the runner will
/// poll the file produced by this helper and forward the bytes to the manager.
///
/// # Errors
///
/// Returns any IO error raised while creating parent directories or writing the file.
pub async fn write_registration_inputs(
    env: &BlueprintEnvironment,
    payload: impl AsRef<[u8]>,
) -> Result<PathBuf, std::io::Error> {
    let output_path = env.registration_output_path();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(&output_path, payload.as_ref()).await?;
    Ok(output_path)
}
