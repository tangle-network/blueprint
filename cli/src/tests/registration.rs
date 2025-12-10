use crate::utils::find_registration_inputs;
use color_eyre::eyre::Result;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn locates_latest_registration_payload() -> Result<()> {
    let temp = tempdir()?;
    let base = temp.path();

    create_payload_dir(base, 42, "alpha", "first")?;
    thread::sleep(Duration::from_millis(10));
    let expected = create_payload_dir(base, 42, "beta", "second")?;
    create_payload_dir(base, 7, "other", "third")?;

    let discovered = find_registration_inputs(base, 42).expect("payload not found");
    assert_eq!(discovered, expected);

    Ok(())
}

fn create_payload_dir(
    base: &Path,
    blueprint_id: u64,
    suffix: &str,
    contents: &str,
) -> Result<std::path::PathBuf> {
    let dir = base.join(format!("blueprint-{blueprint_id}-{suffix}"));
    fs::create_dir_all(&dir)?;
    let file = dir.join("registration_inputs.bin");
    fs::write(&file, contents)?;
    Ok(file)
}
