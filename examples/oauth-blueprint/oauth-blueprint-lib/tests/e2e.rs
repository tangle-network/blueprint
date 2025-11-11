use blueprint_sdk::tangle::layers::TangleLayer;
use blueprint_sdk::testing::tempfile;
use blueprint_sdk::testing::utils::setup_log;
use blueprint_sdk::testing::utils::tangle::{InputValue, TangleTestHarness};
use oauth_blueprint_lib::{
    self as lib, CHECK_SCOPE_JOB_ID, LIST_DOCS_JOB_ID, READ_DOC_JOB_ID, WHOAMI_JOB_ID,
    WRITE_DOC_JOB_ID,
};

#[tokio::test]
async fn oauth_docs_flow_scoped_and_isolated() -> color_eyre::Result<()> {
    color_eyre::install()?;
    setup_log();

    let temp_dir = tempfile::TempDir::new()?;
    let harness = TangleTestHarness::setup(temp_dir).await?;

    // Setup service
    let (mut test_env, service_id, _blueprint_id) = harness.setup_services::<1>(false).await?;
    test_env.initialize().await?;

    // Add jobs
    test_env.add_job(lib::whoami.layer(TangleLayer)).await;
    test_env.add_job(lib::check_scope.layer(TangleLayer)).await;
    test_env.add_job(lib::write_doc.layer(TangleLayer)).await;
    test_env.add_job(lib::read_doc.layer(TangleLayer)).await;
    test_env.add_job(lib::list_docs.layer(TangleLayer)).await;

    test_env.start(()).await?;

    // Tenant A writes a doc
    let job_a_write = harness
        .submit_job(
            service_id,
            WRITE_DOC_JOB_ID,
            vec![
                InputValue::String("doc1".into()),
                InputValue::String("hello".into()),
            ],
        )
        .await?;
    let _res = harness
        .wait_for_job_execution(service_id, job_a_write)
        .await?;

    // Tenant A lists docs
    let job_a_list = harness
        .submit_job(service_id, LIST_DOCS_JOB_ID, vec![])
        .await?;
    let _ = harness
        .wait_for_job_execution(service_id, job_a_list)
        .await?;

    // Tenant A reads doc
    let job_a_read = harness
        .submit_job(
            service_id,
            READ_DOC_JOB_ID,
            vec![InputValue::String("doc1".into())],
        )
        .await?;
    let _ = harness
        .wait_for_job_execution(service_id, job_a_read)
        .await?;

    // Scope check example
    let job_scope = harness
        .submit_job(
            service_id,
            CHECK_SCOPE_JOB_ID,
            vec![InputValue::String("docs:read".into())],
        )
        .await?;
    let _ = harness
        .wait_for_job_execution(service_id, job_scope)
        .await?;

    // whoami works
    let job_who = harness
        .submit_job(service_id, WHOAMI_JOB_ID, vec![])
        .await?;
    let _ = harness.wait_for_job_execution(service_id, job_who).await?;

    Ok(())
}
