use anyhow::Result;
use codegen_schema::manifest::Manifest;
use codegen_spec::GrammarSpecGeneratorExtensions;
use codegen_utils::context::CodegenContext;
use solidity_schema::SolidityGrammarExtensions;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let grammar = Manifest::load_solidity(codegen)?;
        let output_dir = codegen
            .repo_root
            .join("crates/solidity/outputs/spec/generated");

        return grammar.generate_spec(codegen, &output_dir);
    });
}
