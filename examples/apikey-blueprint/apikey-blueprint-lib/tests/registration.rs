use apikey_blueprint_lib::registration_payload;
use blueprint_sdk::registration::write_registration_inputs;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use tempfile::tempdir;

#[tokio::test]
async fn writes_registration_payload_to_expected_location() -> Result<(), Box<dyn std::error::Error>>
{
    let temp = tempdir()?;
    let mut env = BlueprintEnvironment::default();
    env.registration_mode = true;
    env.registration_capture_only = true;
    env.data_dir = temp.path().to_path_buf();
    env.keystore_uri = temp.path().join("keystore").display().to_string();

    let payload = registration_payload();
    let output_path = write_registration_inputs(&env, payload.clone()).await?;
    let saved = tokio::fs::read(&output_path).await?;
    assert_eq!(saved, payload);

    Ok(())
}
