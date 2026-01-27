fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use system PROTOC if available, otherwise fall back to bundled protoc
    if std::env::var("PROTOC").is_err() {
        // SAFETY: This is a build script, and we're setting PROTOC before any proto compilation
        unsafe {
            std::env::set_var("PROTOC", protobuf_src::protoc());
        }
    }

    println!("cargo:rerun-if-changed=proto/pricing.proto");
    tonic_build::compile_protos("proto/pricing.proto")?;
    Ok(())
}
