fn main() {
    let contract_dirs: Vec<&str> = vec!["./contracts"];
    println!("cargo::rerun-if-changed=./contracts/src");

    blueprint_build_utils::soldeer_install();
    blueprint_build_utils::soldeer_update();
    blueprint_build_utils::build_contracts(contract_dirs);
}
