use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;
use infra_utils::cargo::CargoWorkspace;

fn main() -> Result<()> {
    OutputLanguage::Cargo.generate_stubs()?;
    let output_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo")?.join("src/runtime");
    OutputLanguage::Cargo.wit_bindgen(&output_dir)?;
    Ok(())
}
