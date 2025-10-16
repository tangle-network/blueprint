fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use bundled protoc to ensure CI compatibility
    // SAFETY: This is a build script, and we're setting PROTOC before any proto compilation
    unsafe {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }

    println!("cargo:rerun-if-changed=proto/pricing.proto");
    tonic_build::compile_protos("proto/pricing.proto")?;
    Ok(())
}
