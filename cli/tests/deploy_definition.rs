//! Deployment manifests exercised through the public file-to-contract API.

use alloy_primitives::keccak256;
use alloy_sol_types::{SolCall, SolValue};
use blueprint_client_tangle::blueprint_metadata::{
    GpuPolicy, extract_job_profiles_blob, resolve_execution_profile_from_profiling_data,
};
use cargo_tangle::command::deploy::definition::{
    BlueprintDefinition, BlueprintDefinitionLoadResult, FetcherKind, GithubArtifactSpec,
    SourceKind, createBlueprintCall, decode_blueprint_definition, load_blueprint_definition,
};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

const ABI_SCHEMA: &str = r#"[
    {"name":"recipient","type":"address"},
    {"name":"values","type":"uint64[]"},
    {"name":"triple","type":"bool[3]"},
    {"name":"pair","type":"tuple","components":[
        {"name":"enabled","type":"bool"},{"name":"tag","type":"bytes32"}
    ]}
]"#;
// Version 2, four named fields; the list and fixed array each contain an
// unnamed element descriptor, and the tuple contains two named fields.
const TLV_HEX: &str = concat!(
    "020004",
    "0e0000000009726563697069656e74",
    "15000000010676616c756573080000000000",
    "140003000106747269706c65010000000000",
    "16000000020470616972",
    "010000000007656e61626c6564",
    "0f0000000003746167",
);

fn manifest() -> Value {
    let binary = json!({
        "name":"worker", "arch":"x86_64", "os":"linux", "sha256":"aa".repeat(32)
    });
    json!({
        "metadata_uri":"ipfs://deployment-metadata",
        "metadata_hash":format!("0x{}", "11".repeat(32)),
        "manager":"0x0000000000000000000000000000000000000001",
        "metadata":{
            "name":"GPU worker",
            "profiling_data":"existing-compressed-profiles",
            "execution_profile":{
                "confidentiality":"tee_preferred",
                "gpu":{"policy":"required","class":"nvidia-b200","min_count":2,"min_vram_gb":180}
            }
        },
        "jobs":[{"name":"infer","params_schema":ABI_SCHEMA,"result_schema":"0xdeadbeef"}],
        "registration_schema":ABI_SCHEMA,
        "request_schema":"00ff",
        "sources":[
            {
                "kind":"container", "registry":"ghcr.io", "image":"tangle-network/worker", "tag":"test",
                "testing":{"cargo_package":"worker","cargo_bin":"worker","base_path":"./worker"},
                "binaries":[binary.clone()]
            },
            {
                "kind":"native", "fetcher":"github", "entrypoint":"worker",
                "github":{"owner":"tangle-network","repo":"worker","tag":"test","binaries":[binary.clone()]},
                "binaries":[binary]
            },
            {
                "kind":"wasm", "runtime":"wasmtime", "fetcher":"ipfs",
                "artifact_uri":"ipfs://worker-module", "entrypoint":"main",
                "binaries":[{"name":"worker.wasm","arch":"wasm32","os":"unknown","sha256":"bb".repeat(32)}]
            }
        ]
    })
}

fn write_manifest(dir: &Path, extension: &str, value: &Value) -> PathBuf {
    let text = match extension {
        "json" => serde_json::to_string_pretty(value).unwrap(),
        "yaml" => serde_yaml::to_string(value).unwrap(),
        "toml" => toml::to_string(value).unwrap(),
        _ => unreachable!("unsupported fixture format"),
    };
    let path = dir.join(format!("definition.{extension}"));
    fs::write(&path, text).unwrap();
    path
}

fn round_trip(path: &Path) -> (BlueprintDefinitionLoadResult, BlueprintDefinition) {
    let loaded = load_blueprint_definition(path, None).unwrap();
    let decoded = decode_blueprint_definition(&loaded.definition.encoded_bytes()).unwrap();
    assert_eq!(decoded.abi_encode(), loaded.definition.encoded.as_ref());
    assert_eq!(decoded.metadataUri, loaded.definition.metadata_uri);
    assert_eq!(decoded.metadataHash, loaded.definition.metadata_hash);
    assert_eq!(decoded.manager, loaded.definition.manager);
    let call = createBlueprintCall::abi_decode(&loaded.definition.call_data_bytes()).unwrap();
    assert_eq!(
        call.definition.abi_encode(),
        loaded.definition.encoded.as_ref()
    );
    assert_eq!(call.abi_encode(), loaded.definition.call_data.as_ref());
    assert_eq!(
        &loaded.definition.call_data[..4],
        createBlueprintCall::SELECTOR
    );
    (loaded, decoded)
}

#[test]
fn file_formats_preserve_contract_payload_sources_and_explicit_gpu_class() {
    let dir = tempfile::tempdir().unwrap();
    let mut payloads = Vec::new();
    for extension in ["json", "yaml", "toml"] {
        let path = write_manifest(dir.path(), extension, &manifest());
        let (loaded, decoded) = round_trip(&path);
        let profile =
            resolve_execution_profile_from_profiling_data(&decoded.metadata.profilingData)
                .unwrap()
                .unwrap();
        assert_eq!(profile.gpu.class.as_deref(), Some("nvidia-b200"));
        assert_eq!(profile.gpu.policy, GpuPolicy::Required);
        assert_eq!((profile.gpu.min_count, profile.gpu.min_vram_gb), (2, 180));
        assert_eq!(
            extract_job_profiles_blob(&decoded.metadata.profilingData).as_deref(),
            Some("existing-compressed-profiles")
        );
        let profiling: Value = serde_json::from_str(&decoded.metadata.profilingData).unwrap();
        assert_eq!(
            profiling["execution_profile"]["confidentiality"],
            "tee_preferred"
        );
        assert_eq!(
            loaded.summaries.iter().map(|s| s.kind).collect::<Vec<_>>(),
            [SourceKind::Container, SourceKind::Native, SourceKind::Wasm]
        );
        assert_eq!(loaded.summaries[1].fetcher, Some(FetcherKind::Github));
        assert_eq!(loaded.summaries[2].fetcher, Some(FetcherKind::Ipfs));
        assert_eq!(
            decoded.sources.iter().map(|s| s.kind).collect::<Vec<_>>(),
            [0, 2, 1]
        );
        let container = &decoded.sources[0];
        assert_eq!(container.container.registry, "ghcr.io");
        assert_eq!(container.container.image, "tangle-network/worker");
        assert_eq!(container.container.tag, "test");
        assert_eq!(container.testing.cargoPackage, "worker");
        assert_eq!(container.testing.cargoBin, "worker");
        assert_eq!(container.testing.basePath, "./worker");
        assert_eq!(
            (container.binaries[0].arch, container.binaries[0].os),
            (5, 1)
        );
        assert_eq!(container.binaries[0].sha256.as_slice(), &[0xaa; 32]);
        let native = &decoded.sources[1].native;
        assert_eq!(native.fetcher, 3);
        assert_eq!(native.entrypoint, "worker");
        let artifact: GithubArtifactSpec = serde_json::from_str(&native.artifactUri).unwrap();
        assert_eq!(artifact.owner, "tangle-network");
        assert_eq!(artifact.repo, "worker");
        assert_eq!(artifact.tag, "test");
        assert_eq!(artifact.binaries.len(), 1);
        assert_eq!(artifact.binaries[0].name, "worker");
        assert_eq!(artifact.binaries[0].sha256, "aa".repeat(32));
        assert!(artifact.binaries[0].blake3.is_none());
        let wasm = &decoded.sources[2].wasm;
        assert_eq!((wasm.runtime, wasm.fetcher), (1, 1));
        assert_eq!(wasm.artifactUri, "ipfs://worker-module");
        assert_eq!(wasm.entrypoint, "main");
        assert_eq!(
            (
                decoded.sources[2].binaries[0].arch,
                decoded.sources[2].binaries[0].os
            ),
            (0, 0)
        );
        payloads.push((loaded.definition.encoded, loaded.definition.call_data));
    }
    assert!(payloads.windows(2).all(|pair| pair[0] == pair[1]));
}

#[test]
fn abi_json_hex_json_and_raw_tlv_have_identical_wire_bytes() {
    let dir = tempfile::tempdir().unwrap();
    let expected = hex::decode(TLV_HEX).unwrap();
    let mut payloads = Vec::new();
    for schema in [
        ABI_SCHEMA.to_owned(),
        format!("0x{}", hex::encode(ABI_SCHEMA)),
        format!("0x{TLV_HEX}"),
    ] {
        let mut input = manifest();
        input["jobs"][0]["params_schema"] = json!(schema);
        input["registration_schema"] = input["jobs"][0]["params_schema"].clone();
        let path = write_manifest(dir.path(), "json", &input);
        let (loaded, decoded) = round_trip(&path);
        assert_eq!(decoded.jobs[0].paramsSchema.as_ref(), expected.as_slice());
        assert_eq!(decoded.registrationSchema.as_ref(), expected.as_slice());
        assert_eq!(
            decoded.jobs[0].resultSchema.as_ref(),
            [0xde, 0xad, 0xbe, 0xef]
        );
        assert_eq!(decoded.requestSchema.as_ref(), [0, 0xff]);
        payloads.push((loaded.definition.encoded, loaded.definition.call_data));
    }
    assert!(payloads.windows(2).all(|pair| pair[0] == pair[1]));
}

#[test]
fn relative_metadata_file_hash_is_canonical_and_payload_sensitive() {
    let dir = tempfile::tempdir().unwrap();
    let mut input = manifest();
    input.as_object_mut().unwrap().remove("metadata_hash");
    input["metadata_file"] = json!("payload.json");
    let path = write_manifest(dir.path(), "json", &input);
    let metadata = dir.path().join("payload.json");
    fs::write(&metadata, r#"{"z":"last","list":[{"z":4,"a":3},2],"integrity":{"signature":"first"},"a":{"z":2,"a":1}}"#).unwrap();
    let (first, _) = round_trip(&path);
    let canonical = br#"{"a":{"a":1,"z":2},"list":[{"a":3,"z":4},2],"z":"last"}"#;
    assert_eq!(first.definition.metadata_hash, keccak256(canonical));
    fs::write(&metadata, r#"{"a":{"a":1,"z":2},"integrity":{"signature":"second"},"list":[{"a":3,"z":4},2],"z":"last"}"#).unwrap();
    let (second, _) = round_trip(&path);
    assert_eq!(first.definition.encoded, second.definition.encoded);
    assert_eq!(first.definition.call_data, second.definition.call_data);
    fs::write(
        &metadata,
        r#"{"a":{"a":1,"z":2},"list":[{"a":3,"z":4},2],"z":"changed"}"#,
    )
    .unwrap();
    let (changed, _) = round_trip(&path);
    assert_ne!(
        first.definition.metadata_hash,
        changed.definition.metadata_hash
    );
    assert_ne!(first.definition.encoded, changed.definition.encoded);
}

#[test]
fn malformed_manifest_schema_hash_and_contract_bytes_are_rejected() {
    let dir = tempfile::tempdir().unwrap();
    for (pointer, value, diagnostic) in [
        (
            "/jobs/0/params_schema",
            json!("["),
            "failed to parse JSON schema",
        ),
        (
            "/jobs/0/params_schema",
            json!(r#"[{"name":"x","type":"uint24"}]"#),
            "unsupported uint size",
        ),
        (
            "/jobs/0/params_schema",
            json!("0xzz"),
            "failed to decode hex schema",
        ),
        (
            "/metadata_hash",
            json!("0x00"),
            "metadata_hash digest must be 32 bytes",
        ),
        (
            "/metadata_hash",
            json!("0xzz"),
            "invalid metadata_hash digest",
        ),
    ] {
        let mut input = manifest();
        *input.pointer_mut(pointer).unwrap() = value;
        let path = write_manifest(dir.path(), "json", &input);
        let error = load_blueprint_definition(&path, None).unwrap_err();
        assert!(
            format!("{error:#}").contains(diagnostic),
            "{pointer}: {error:#}"
        );
    }
    let mut input = manifest();
    input["metadata"]["execution_profile"]["gpu"]["unexpected"] = json!(true);
    let path = write_manifest(dir.path(), "json", &input);
    assert!(
        format!("{:#}", load_blueprint_definition(&path, None).unwrap_err())
            .contains("unknown field `unexpected`")
    );
    input.as_object_mut().unwrap().remove("metadata_hash");
    input["metadata_file"] = json!("payload.json");
    input["metadata"]["execution_profile"]["gpu"]
        .as_object_mut()
        .unwrap()
        .remove("unexpected");
    fs::write(dir.path().join("payload.json"), "[]").unwrap();
    let path = write_manifest(dir.path(), "json", &input);
    assert!(
        load_blueprint_definition(&path, None)
            .unwrap_err()
            .to_string()
            .contains("top-level JSON object")
    );
    let path = write_manifest(dir.path(), "json", &manifest());
    let (loaded, _) = round_trip(&path);
    assert!(decode_blueprint_definition(&loaded.definition.encoded[..8]).is_err());
    let mut call = loaded.definition.call_data.to_vec();
    call[0] ^= 0xff;
    assert!(createBlueprintCall::abi_decode(&call).is_err());
}
