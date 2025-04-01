fn main() {
    let contract_dirs: Vec<&str> = vec!["./contracts", "./dependencies/eigenlayer-middleware-0.5.4"];
    blueprint_build_utils::soldeer_install();
    blueprint_build_utils::soldeer_update();
    blueprint_build_utils::build_contracts(contract_dirs);
}
