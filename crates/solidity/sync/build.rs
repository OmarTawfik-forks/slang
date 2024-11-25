use anyhow::{Ok, Result};
use codegen_runtime_generator::RuntimeGenerator;
use codegen_spec::Spec;
use codegen_testing::TestingGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use solidity_language::SolidityDefinition;

mod _make_sure_codegen_is_sync_first_ {
    use codegen_runtime_sync as _;
}

fn main() {
    [
        || {
            let mut fs = generate_product("codegen_runtime_cargo_crate", "slang_solidity")?;
            render_built_ins("slang_solidity", &mut fs)?;
            Ok(())
        },
        || {
            generate_product("codegen_runtime_cargo_wasm", "solidity_cargo_wasm")?;
            Ok(())
        },
        || {
            generate_product("codegen_runtime_npm_package", "solidity_npm_package")?;
            Ok(())
        },
        || {
            generate_spec("solidity_spec")?;
            Ok(())
        },
        || {
            generate_tests(
                "solidity_language",
                "solidity_testing_snapshots",
                "solidity_cargo_tests",
            )?;
            Ok(())
        },
    ]
    .into_par_iter()
    .for_each(|callback| callback().unwrap());
}

fn generate_product(input_crate: &str, output_crate: &str) -> Result<CodegenFileSystem> {
    let language = SolidityDefinition::create();

    let input_dir = CargoWorkspace::locate_source_crate(input_crate)?.join("src/runtime");

    let output_dir = CargoWorkspace::locate_source_crate(output_crate)?.join("src/generated");

    let fs = RuntimeGenerator::generate_product(&language, &input_dir, &output_dir)?;

    Ok(fs)
}

fn render_built_ins(output_crate: &str, fs: &mut CodegenFileSystem) -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate(output_crate)?
        .join("src/generated/bindings/generated/built_ins");

    codegen_runtime_generator::render_built_ins(
        fs,
        &language,
        &output_dir,
        solidity_language::render_built_ins,
    )
}

fn generate_spec(crate_name: &str) -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate(crate_name)?.join("generated");

    Spec::generate(language, &output_dir)
}

fn generate_tests(input_crate: &str, snapshots_crate: &str, tests_crate: &str) -> Result<()> {
    let language = SolidityDefinition::create();

    let input_dir = CargoWorkspace::locate_source_crate(input_crate)?;
    let snapshots_dir = CargoWorkspace::locate_source_crate(snapshots_crate)?;
    let tests_dir = CargoWorkspace::locate_source_crate(tests_crate)?;

    language.generate_version_breaks(&input_dir.join("src"), &tests_dir.join("src/generated"))?;

    language.generate_bindings_assertions_tests(
        &snapshots_dir.join("bindings_assertions"),
        &tests_dir.join("src/bindings_assertions/generated"),
    )?;

    language.generate_bindings_output_tests(
        &snapshots_dir.join("bindings_output"),
        &tests_dir.join("src/bindings_output/generated"),
    )?;

    language.generate_cst_output_tests(
        &snapshots_dir.join("cst_output"),
        &tests_dir.join("src/cst_output/generated"),
    )?;

    Ok(())
}
