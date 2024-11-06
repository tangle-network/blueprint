fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    blueprint_metadata::generate_json();
    blueprint_build_utils::build_contracts(vec![
        "./contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts",
        "./contracts/lib/eigenlayer-middleware",
        "./contracts/lib/forge-std",
        "./contracts",
    ]);
}
