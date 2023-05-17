use anyhow::Result;
use codegen_schema::manifest::Manifest;
use codegen_utils::context::CodegenContext;

pub trait SolidityGrammarExtensions {
    fn load_solidity(codegen: &mut CodegenContext) -> Result<Manifest>;
}

impl SolidityGrammarExtensions for Manifest {
    fn load_solidity(codegen: &mut CodegenContext) -> Result<Manifest> {
        let manifest_dir = codegen
            .repo_root
            .join("crates/solidity/inputs/schema/grammar");

        // Rebuild if input files are added/removed
        codegen.track_input_dir(&manifest_dir);

        let grammar = Manifest::load_from_dir(manifest_dir, codegen)?;
        return Ok(grammar);
    }
}
