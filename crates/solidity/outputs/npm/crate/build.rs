use infra_utils::{cargo::CargoWorkspace, commands::Command};

fn main() {
    // Reference here to make sure we recompile when that changes:
    use solidity_npm_build as _;

    execute_codegen_for_local_development();

    napi_build::setup();
}

fn execute_codegen_for_local_development() {
    let crate_dir = CargoWorkspace::locate_source_crate("solidity_npm_build");

    Command::new("cargo")
        .arg("run")
        .property("--bin", "solidity_npm_build")
        .current_dir(crate_dir)
        .run()
        .expect("Failed to run codegen.");
}
