use anyhow::{Ok, Result};
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use testlang_language::TestlangDefinition;

mod _make_sure_codegen_is_sync_first_ {
    use codegen_runtime_sync as _;
}

fn main() {
    [
        || {
            generate_product("codegen_runtime_cargo_crate", "slang_testlang")?;
            Ok(())
        },
        || {
            generate_product("codegen_runtime_cargo_wasm", "testlang_cargo_wasm")?;
            Ok(())
        },
        || {
            generate_product("codegen_runtime_npm_package", "testlang_npm_package")?;
            Ok(())
        },
    ]
    .into_par_iter()
    .for_each(|callback| callback().unwrap());
}

fn generate_product(input_crate: &str, output_crate: &str) -> Result<CodegenFileSystem> {
    let language = TestlangDefinition::create();

    let input_dir = CargoWorkspace::locate_source_crate(input_crate)?.join("src/runtime");

    let output_dir = CargoWorkspace::locate_source_crate(output_crate)?.join("src/generated");

    let fs = RuntimeGenerator::generate_product(&language, &input_dir, &output_dir)?;

    Ok(fs)
}
