fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/pricing.proto");
    tonic_build::compile_protos("proto/pricing.proto")?;
    Ok(())
}
