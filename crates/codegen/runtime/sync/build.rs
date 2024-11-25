use anyhow::Result;
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    [
        || generate_stubs("codegen_runtime_cargo_crate"),
        || generate_stubs("codegen_runtime_cargo_wasm"),
        || generate_stubs("codegen_runtime_npm_package"),
    ]
    .into_par_iter()
    .for_each(|callback| callback().unwrap());
}

fn generate_stubs(crate_name: &str) -> Result<()> {
    let source_dir = CargoWorkspace::locate_source_crate(crate_name)?;

    RuntimeGenerator::generate_stubs(&source_dir)?;

    Ok(())
}
