use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=../oauth-blueprint-lib");

    let metadata = serde_json::json!({
        "name": "oauth-blueprint",
        "description": "OAuth-protected document storage blueprint",
        "version": env!("CARGO_PKG_VERSION"),
        "master_revision": "Latest",
        "manager": { "Evm": "OauthBlueprintBSM" },
        "jobs": [
            {
                "name": "write_doc",
                "job_index": WRITE_DOC_JOB_ID,
                "inputs": ["string", "string", "string"],
                "outputs": ["tuple(bool,string,string)"],
                "required_results": 1,
                "description": "Persist a document for a tenant"
            },
            {
                "name": "admin_purge",
                "job_index": ADMIN_PURGE_JOB_ID,
                "inputs": ["string"],
                "outputs": ["tuple(bool,string)"],
                "required_results": 1,
                "description": "Purge all documents for a tenant"
            }
        ]
    });

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..");
    std::fs::write(
        root.join("blueprint_oauth.json"),
        serde_json::to_string_pretty(&metadata).unwrap(),
    )
    .unwrap();
}

const WRITE_DOC_JOB_ID: u8 = oauth_blueprint_lib::WRITE_DOC_JOB_ID;
const ADMIN_PURGE_JOB_ID: u8 = oauth_blueprint_lib::ADMIN_PURGE_JOB_ID;
