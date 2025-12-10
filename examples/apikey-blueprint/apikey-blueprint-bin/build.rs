use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=../apikey-blueprint-lib");

    let metadata = serde_json::json!({
        "name": "apikey-blueprint",
        "description": "API key issuance and resource blueprint",
        "version": env!("CARGO_PKG_VERSION"),
        "master_revision": "Latest",
        "manager": { "Evm": "ApikeyBlueprintBSM" },
        "jobs": [
            {
                "name": "write_resource",
                "job_index": WRITE_RESOURCE_JOB_ID,
                "inputs": ["string", "string", "address"],
                "outputs": ["tuple(bool,string,string)"],
                "required_results": 1
            },
            {
                "name": "purchase_api_key",
                "job_index": PURCHASE_API_KEY_JOB_ID,
                "inputs": ["string", "address"],
                "outputs": ["tuple(bool,string)"],
                "required_results": 1
            }
        ]
    });

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..");
    std::fs::write(
        root.join("blueprint_apikey.json"),
        serde_json::to_string_pretty(&metadata).unwrap(),
    )
    .unwrap();
}

const WRITE_RESOURCE_JOB_ID: u8 = apikey_blueprint_lib::WRITE_RESOURCE_JOB_ID;
const PURCHASE_API_KEY_JOB_ID: u8 = apikey_blueprint_lib::PURCHASE_API_KEY_JOB_ID;
