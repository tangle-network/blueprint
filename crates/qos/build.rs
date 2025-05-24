fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .type_attribute(".", "#[allow(clippy::pedantic, clippy::missing_errors_doc, clippy::derive_partial_eq_without_eq)]")
        .compile(&["proto/qos.proto"], &["proto"])?;
    Ok(())
}
